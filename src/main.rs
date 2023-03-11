use gtk::gdk::*;
use gtk::gio::ApplicationFlags;
use gtk::prelude::*;
use gtk::*;
use gtk_layer_shell::Edge;
use std::time::Duration;
use std::fs::read_to_string;
use serde::Deserialize;
use rusty_bar::cpu::Cpu;
use rusty_bar::text::{Attributes,Font,Padding,Color};




#[derive(Deserialize)]
enum Pos {
    Top,
    Buttom,
}
#[derive(Deserialize)]
enum WType {
    Workspaces,
    Clock,
    ActiveWindow,
    CPU,
    RAM,
    Wireless,
    Battry,
    Systray,
    Temps,
    Disk,
    Alsa,
    Script,
}
#[derive(Deserialize)]
struct Widget {
    wtype: WType,
    callback: Option<String>,
    cmd: Option<String>,
    format: Option<String>,
    nomarkup: Option<bool>,
    tooltip: Option<String>,
    notoolmarkup: Option<bool>,
    icon: Option<String>,
    replace_with_icons: Option<Vec<Vec<String>>>,
    animate: Option<Vec<AniStr>>,
    warning: Option<Vec<AniStr>>
}
#[derive(Deserialize)]
struct AniStr {
    frame:u8,
    format:String,
}
#[derive(Deserialize)]
struct Widgets {
    left:Option<Vec<Widget>>,
    center:Option<Vec<Widget>>,
    right:Option<Vec<Widget>>,
}
#[derive(Deserialize)]
struct RustyBar {
    pos:Option<Pos>,
    noicons:Option<bool>,
    backgrund:Option<String>,
    foregrund:Option<String>,
    iconcolor:Option<String>,
    widgets:Option<Widgets>,}

pub trait BarWidget {
    fn tick();

    fn new();
}


fn main(){
    let application = Application::new(None, ApplicationFlags::default());
    application.connect_activate(|app| {
	let window = ApplicationWindow::new(app);

	/*
	let conf: String = if read_to_string("~.config/rusty_bar/config.ron").is_ok(){
	    read_to_string("~.config/rusty_bar/config.ron").unwrap()
	} else {
	    println!("no config in .config/rusty_bar/");
	    read_to_string("/etc/rusty_bar/config.ron").expect("no fallback config in etc/rusty_bar")
		
	};
	let config: RustyBar = ron::from_str(conf.as_str()).expect("error in config");
	*/

    
	window.connect_screen_changed(set_visual);
	window.connect_draw(draw);

    // Initialize layer shell before the window has been fully initialized.
	gtk_layer_shell::init_for_window(&window);

    // Order above normal windows
    // Prior to 0.2.9, this was set to Bottom but it caused issues with tooltips being shown below
    // windows.
	gtk_layer_shell::set_layer(&window, gtk_layer_shell::Layer::Top);

    
    // Push other windows out of the way
    // Toggling this off may help some if they are in applications that have weird unicode text, which may mess with the bars scaling.
	gtk_layer_shell::auto_exclusive_zone_enable(&window);

	/*let istop = match config.pos.is_some() {
	    true => true,
	    false =>true /*match config.pos.unwrap() {
		Pos::Top => true,
		Pos::Buttom => false,
	    }*/
	};*/
    
	gtk_layer_shell::set_anchor(&window, Edge::Top, true);//istop);
	gtk_layer_shell::set_anchor(&window, Edge::Right, true);
	gtk_layer_shell::set_anchor(&window, Edge::Left, true);
	gtk_layer_shell::set_anchor(&window, Edge::Bottom, false);// !istop);
    // Allows for specifing the namespace of the layer.
    // The default is "gtk-layer-shell" to not break existing configs.
	let namespace = "gtk-layer-shell".to_string();

	gtk_layer_shell::set_namespace(&window, &namespace);

    // Initialize gdk::Display by default value, which is decided by the compositor.
	let display = Display::default().expect("thing");

    // Loads the monitor variable from config, default is 0. 
    // Gets the actual gdk::Monitor from configured number.
	let monitor = display.monitor(0).expect("thing");

    // Sets which monitor should be used for the bar.
	gtk_layer_shell::set_monitor(&window, &monitor);

    // For transparency to work.
	window.set_app_paintable(true);

    // Build all the widgets.
	build_widgets(&window/*,config*/);});

    application.run();
}
fn set_visual(window: &ApplicationWindow, screen: Option<&Screen>) {
    if let Some(screen) = screen {
        if let Some(ref visual) = screen.rgba_visual() {
            window.set_visual(Some(visual)); // Needed for transparency, not available in GTK 4+ so
                                             // F.
        }
    }
}
// Draws the window using a custom color and opacity.
fn draw(_: &ApplicationWindow, ctx: &cairo::Context) -> Inhibit {
    // Applys
    ctx.set_source_rgba(0.1, 0.1, 0.1, 0.5);
    ctx.set_operator(cairo::Operator::Screen);
    ctx.paint().expect("thing");
    Inhibit(false)
}
fn build_widgets(window: &ApplicationWindow/*,config:RustyBar*/) {
    // Create box widgets, which we'll be using to draw the content onto.
    let root = Box::new(Orientation::Horizontal, 0);
    let left = Box::new(Orientation::Horizontal, 0);
    let centered = Box::new(Orientation::Horizontal, 0);
    let right = Box::new(Orientation::Horizontal, 0);

    // 0.2.5: Root expands across the entire bar, previously "left" would do this but it isn't
    //   ideal when customizing, since borders would draw on the entire bar rather than just on the
    //   left portion of the bar.
    root.set_widget_name("root");

    // 0.2.5: Allow for customizing left, centered and right.
    left.set_widget_name("left");
    centered.set_widget_name("centered");
    right.set_widget_name("right");

    root.set_center_widget(Some(&centered));
    root.pack_end(&right, false, true, 0);
    root.add(&left);
    window.add(&root);
    /*
    if let container = config.widgets.unwrap(){
	if let wedgets = container.left.unwrap() {
	    for i in &wedgets{
		wedgit("left".to_string(),i)
	    }
	}
	if let wedgets = container.center.unwrap() {
	    for i in &wedgets{
		wedgit("center".to_string(),i)
	    }
	}
	if let wedgets = container.right.unwrap() {
	    for i in &wedgets{
		wedgit("right".to_string(), i)
	    }   
	}
    }else {

}*/

    wedgit("center".to_owned(), &Widget{
	wtype: WType::CPU,
	callback: None,
        cmd: None,
        format: None,
        nomarkup: None,
        tooltip: None,
        notoolmarkup: None,
        icon: None,
        replace_with_icons: None,
        animate: None,
        warning: None,

    },&centered);

    // Prepare and show all of the widgets.
    window.show_all();
}
fn wedgit(con:String,wed:&Widget,cont:&Box) {
    match wed.wtype {
	WType::CPU => {
            let icon = wed.icon.clone().unwrap_or(" ".to_string());
		let text = if wed.format.is_none(){
		    format!("<span foreground='#229922'>{icon}</span><span foreground='#bbbbbb'>load</span>")
		} else{
		    wed.format.clone().unwrap()
		};
            let label=Label::new(None);

            label.set_widget_name("CPU");
	    label.set_use_markup(true);
            label.set_markup(&text);
	    cont.add(&label);

            let mut cpu=Cpu::new(attr(), text).unwrap();
            let mut tick = move || {
            label.set_markup(&cpu.tick().unwrap()[0].text);
            glib::Continue(true)};
    
            tick();
            glib::timeout_add_local(Duration::from_millis(1000), tick);
	},
	WType::RAM => {},
	WType::Disk => {},
	WType::Alsa => {},
	WType::Clock => {},
	WType::Temps => {},
	WType::Battry => {},
	WType::Script => {},
	WType::Systray => {},
	WType::Wireless => {},
	WType::Workspaces => {},
	WType::ActiveWindow => {},	    
    }
}
    
fn attr() -> Attributes {
    Attributes{
	font: Font::new("Hack Nerd Font 11"),
	fg_color: Color::from_hex("#eeeeee"),
	bg_color: None,
	padding: Padding::new(8.0, 8.0, 0.0, 0.0),
    }
}

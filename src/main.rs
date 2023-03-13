use gtk::gdk::*;
use gtk::gio::ApplicationFlags;
use gtk::prelude::*;
use gtk::*;
use gtk::traits::SettingsExt;
use gtk_layer_shell::Edge;
use pango::FontDescription;
use rusty_bar::cpu::Cpu;
use rusty_bar::clock::Clock;
use rusty_bar::workspaces::Workspaces;
use serde::Deserialize;
use std::time::Duration;

#[derive(Deserialize)]
enum Pos {
    Top,
    Buttom,
}
#[derive(Deserialize)]
enum WType {
    Workspaces,
    CLOCK,
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
    warning: Option<Vec<AniStr>>,
}
#[derive(Deserialize)]
struct AniStr {
    frame: u8,
    format: String,
}
#[derive(Deserialize)]
struct Widgets {
    left: Option<Vec<Widget>>,
    center: Option<Vec<Widget>>,
    right: Option<Vec<Widget>>,
}
#[derive(Deserialize)]
struct RustyBar {
    pos: Option<Pos>,
    noicons: Option<bool>,
    backgrund: Option<String>,
    foregrund: Option<String>,
    iconcolor: Option<String>,
    widgets: Option<Widgets>,
}

fn main() {
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
        Settings::for_screen(&gtk::prelude::WidgetExt::screen(&window).expect("msg")).expect("fuck").set_gtk_font_name(Some("Hack Nerd Font"));
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

        gtk_layer_shell::set_anchor(&window, Edge::Top, true); //istop);
        gtk_layer_shell::set_anchor(&window, Edge::Right, true);
        gtk_layer_shell::set_anchor(&window, Edge::Left, true);
        gtk_layer_shell::set_anchor(&window, Edge::Bottom, false); // !istop);
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
        build_widgets(&window /*,config*/);
    });

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
    let layout=pangocairo::functions::create_layout(ctx);
    layout.set_font_description(Some(&FontDescription::from_string("Hack Nerd Font")));
    pangocairo::functions::show_layout(ctx, &layout);

    Inhibit(false)
}
fn build_widgets(window: &ApplicationWindow /*,config:RustyBar*/) {
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


    // this is temporary code while porting to wayland
    wedgit(&Widget {wtype: WType::CPU,callback: None,cmd: None,format: None,nomarkup: None,tooltip: None,notoolmarkup: None,icon: None,replace_with_icons: None,animate: None,warning: None,},&right,);
    wedgit(&Widget {wtype: WType::CLOCK,callback: None,cmd: None,format: None,nomarkup: None,tooltip: None,notoolmarkup: None,icon: None,replace_with_icons: None,animate: None,warning: None,},&centered,);
    wedgit(&Widget {wtype: WType::Workspaces,callback: None,cmd: None,format: None,nomarkup: None,tooltip: None,notoolmarkup: None,icon: None,replace_with_icons: None,animate: None,warning: None,},&left,);

    // Prepare and show all of the widgets.
    window.show_all();
}
fn wedgit(wed: &Widget, cont: &Box) {
    match wed.wtype {
        WType::CPU => {
            let icon = wed.icon.clone().unwrap_or(" ".to_string());
            let text = if wed.format.is_none() {
                format!("<span foreground='#229922'>{icon}</span><span foreground='#bbbbbb'>load%</span>")
            } else {
                wed.format.clone().unwrap()
            };
            let mut cpu = Cpu::new(text, cont).unwrap();
            let mut tick = move || {
                cpu.tick();
                glib::Continue(true)
            };
            tick();
            glib::timeout_add_local(Duration::from_millis(1000), tick);
        }
        WType::RAM => {}
        WType::Disk => {}
        WType::Alsa => {}
        WType::CLOCK => {
            let icon = wed.icon.clone().unwrap_or(" ".to_string());
            let text = if wed.format.is_none() {
                format!("<span foreground='#229922'>{icon}</span><span foreground='#bbbbbb'>%d/%m/%Y </span><span foreground='#229922'>󱨰 </span><span foreground='#bbbbbb'>%a </span><span foreground='#229922'>󱑎 </span><span foreground='#bbbbbb'>%H:%M </span>")
            } else {
                wed.format.clone().unwrap()
            };
            let clock = Clock::new(text, cont);
            let tick = move || {
                clock.tick();
                glib::Continue(true)
            };
            tick();
            glib::timeout_add_local(Duration::from_millis(10000), tick);
        }
        WType::Temps => {}
        WType::Battry => {}
        WType::Script => {}
        WType::Systray => {}
        WType::Wireless => {}
        WType::Workspaces => {
            let _workspaces = Workspaces::new(cont);
        }
        WType::ActiveWindow => {}
    }
}
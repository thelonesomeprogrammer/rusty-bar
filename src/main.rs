use gtk4::gdk::*;
use gtk4::traits::WidgetExt;
use gtk4::prelude::*;
use gtk4::*;
use gtk4_layer_shell::Edge;
use rusty_bar::cpu::Cpu;
use rusty_bar::clock::Clock;
use rusty_bar::workspaces::Workspaces;
use rusty_bar::battery::BatteryView;
use rusty_bar::active_window_title::ActiveWindowTitle;
use rusty_bar::volume::Volume;
use rusty_bar::disk_usage::DiskUsage;
use rusty_bar::wireless::Wireless;
use rusty_bar::ram::RAM;
use rusty_bar::temps::Temps;
use rusty_bar::command::Command;
use serde_derive::{Deserialize, Serialize};
use std::time::Duration;
use std::path::PathBuf;
use rusty_bar::AniStr;
use rusty_bar::Replacement;

#[derive(Deserialize,Serialize,Debug)]
enum Pos {
    Top,
    Buttom,
}
#[derive(Deserialize,Serialize,Debug)]
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
#[derive(Deserialize,Serialize,Debug)]
struct Widget {
    wtype: WType,
    callback: Option<String>,
    cmd: Option<String>,
    format: Option<String>,
    tooltip: Option<String>,
    icon: Option<String>,
    replace_with_icons: Option<Vec<Replacement>>,
    animate: Option<Vec<AniStr>>,
    warning: Option<Vec<AniStr>>,
}

#[derive(Deserialize,Serialize,Debug)]
struct Widgets {
    left: Option<Vec<Widget>>,
    center: Option<Vec<Widget>>,
    right: Option<Vec<Widget>>,
}
#[derive(Deserialize,Serialize,Debug)]
struct RustyBar {
    pos: Option<Pos>,
    backgrund: Option<String>,
    foregrund: Option<String>,
    iconcolor: Option<String>,
    widgets: Widgets,
}

fn main() {
    let app = Application::builder()
        .application_id("org.example.HelloWorld")
        .build();

    app.connect_activate(|app| {
        // We create the main window.
        let window = ApplicationWindow::builder()
        .application(app).title("Hello, World!").build();

        let config_dir = dirs::config_dir().unwrap().join(PathBuf::from("rusty_bar/rustyconf.ron"));

        let conf: String = if std::fs::read_to_string(config_dir.as_path()).is_ok(){
            std::fs::read_to_string(config_dir.as_path()).unwrap()

        } else if std::fs::read_to_string("/etc/rusty_bar/rustyconf.ron").is_ok(){
            println!("no config in .config/rusty_bar/");
            std::fs::read_to_string("/etc/rusty_bar/rustyconf.ron").expect("no fallback config in etc/rusty_bar")

        } else {
            let left = vec![
                Widget {wtype: WType::Workspaces,callback: None,cmd: None,format: None,tooltip: None,icon: None,replace_with_icons: None,animate: None,warning: None,},
                Widget {wtype: WType::ActiveWindow,callback: None,cmd: None,format: None,tooltip: None,icon: None,replace_with_icons: None,animate: None,warning: None,},
            ];


            let clock_format = "<span foreground='#229922'>   </span><span foreground='#bbbbbb'>%d/%m/%Y </span><span foreground='#229922'>  󱨰  </span><span foreground='#bbbbbb'>%a </span><span foreground='#229922'>  󱑎   </span><span foreground='#bbbbbb'>%H:%M </span>".to_string();
            let center = vec![
                Widget {wtype: WType::CLOCK,callback: None,cmd: None,format: Some(clock_format),tooltip: None,icon: None,replace_with_icons: None,animate: None,warning: None,}
            ];


	        let battery_ani = vec![
                AniStr{treash:95,format:"<span foreground='#229922'> 󰂄  </span><span foreground='#bbbbbb'>load full</span>".to_string(),condition:None},
				AniStr{treash:90,format:dlayout(" 󰂋  ".to_string(),"#229922".to_string(),"#bbbbbb".to_string()),condition:Some(true)},
				AniStr{treash:80,format:dlayout(" 󰂊  ".to_string(),"#229922".to_string(),"#bbbbbb".to_string()),condition:Some(true)},
				AniStr{treash:70,format:dlayout(" 󰢞  ".to_string(),"#229922".to_string(),"#bbbbbb".to_string()),condition:Some(true)},
			    AniStr{treash:60,format:dlayout(" 󰂉  ".to_string(),"#229922".to_string(),"#bbbbbb".to_string()),condition:Some(true)},
			    AniStr{treash:50,format:dlayout(" 󰢝  ".to_string(),"#229922".to_string(),"#bbbbbb".to_string()),condition:Some(true)},
			    AniStr{treash:40,format:dlayout(" 󰂈  ".to_string(),"#229922".to_string(),"#bbbbbb".to_string()),condition:Some(true)},
			    AniStr{treash:30,format:dlayout(" 󰂇  ".to_string(),"#229922".to_string(),"#bbbbbb".to_string()),condition:Some(true)},
			    AniStr{treash:20,format:dlayout(" 󰂆  ".to_string(),"#229922".to_string(),"#bbbbbb".to_string()),condition:Some(true)},
				AniStr{treash:10,format:dlayout(" 󰢜  ".to_string(),"#229922".to_string(),"#bbbbbb".to_string()),condition:Some(true)},
				AniStr{treash:5 ,format:dlayout(" 󰢟  ".to_string(),"#229922".to_string(),"#bbbbbb".to_string()),condition:Some(true)},
				AniStr{treash:90,format:dlayout(" 󰂂  ".to_string(),"#229922".to_string(),"#bbbbbb".to_string()),condition:Some(false)},
				AniStr{treash:80,format:dlayout(" 󰂁  ".to_string(),"#229922".to_string(),"#bbbbbb".to_string()),condition:Some(false)},
				AniStr{treash:70,format:dlayout(" 󰂀  ".to_string(),"#229922".to_string(),"#bbbbbb".to_string()),condition:Some(false)},
				AniStr{treash:60,format:dlayout(" 󰁿  ".to_string(),"#229922".to_string(),"#bbbbbb".to_string()),condition:Some(false)},
				AniStr{treash:50,format:dlayout(" 󰁾  ".to_string(),"#229922".to_string(),"#bbbbbb".to_string()),condition:Some(false)},
				AniStr{treash:40,format:dlayout(" 󰁽  ".to_string(),"#229922".to_string(),"#bbbbbb".to_string()),condition:Some(false)},
				AniStr{treash:30,format:dlayout(" 󰁼  ".to_string(),"#229922".to_string(),"#bbbbbb".to_string()),condition:Some(false)},
				AniStr{treash:20,format:dlayout(" 󰁻  ".to_string(),"#229922".to_string(),"#bbbbbb".to_string()),condition:Some(false)},
				AniStr{treash:10,format:dlayout(" 󰁺  ".to_string(),"#229922".to_string(),"#bbbbbb".to_string()),condition:Some(false)},
				AniStr{treash:5 ,format:dlayout(" 󰂎  ".to_string(),"#229922".to_string(),"#bbbbbb".to_string()),condition:Some(false)},
				AniStr{treash:0 ,format:"<span foreground='#229922'> 󱉞 </span><span foreground='#bbbbbb'>load empty</span>".to_string(),condition:None},
	        ];

            let wifi_ani = vec![
                AniStr{treash:0 ,format:wlayout(" 󰤮 ".to_string(),"#229922".to_string(),"#bbbbbb".to_string()),condition:Some(false)},
				AniStr{treash:90,format:wlayout(" 󰤨 ".to_string(),"#229922".to_string(),"#bbbbbb".to_string()),condition:Some(true)},
				AniStr{treash:60,format:wlayout(" 󰤥 ".to_string(),"#229922".to_string(),"#bbbbbb".to_string()),condition:Some(true)},
				AniStr{treash:40,format:wlayout(" 󰤢 ".to_string(),"#229922".to_string(),"#bbbbbb".to_string()),condition:Some(true)},
			    AniStr{treash:20,format:wlayout(" 󰤟 ".to_string(),"#229922".to_string(),"#bbbbbb".to_string()),condition:Some(true)},
                AniStr{treash:5 ,format:wlayout(" 󰤯 ".to_string(),"#229922".to_string(),"#bbbbbb".to_string()),condition:Some(true)},
	        ];

            let vol_ani = vec![
                AniStr{treash:0 ,format: layout(" 󰝟  ".to_string(),"#229922".to_string(),"#bbbbbb".to_string()),condition:Some(false)},
				AniStr{treash:80,format: layout("   ".to_string(),"#229922".to_string(),"#bbbbbb".to_string()),condition:Some(true)},
				AniStr{treash:50,format: layout(" 󰕾  ".to_string(),"#229922".to_string(),"#bbbbbb".to_string()),condition:Some(true)},
				AniStr{treash:20,format: layout(" 󰖀  ".to_string(),"#229922".to_string(),"#bbbbbb".to_string()),condition:Some(true)},
			    AniStr{treash:0 ,format: layout(" 󰕿  ".to_string(),"#229922".to_string(),"#bbbbbb".to_string()),condition:Some(true)},
	        ];
        
            let temp_ani = vec![
                AniStr{treash:100,format:layout("  ".to_string(),"#229922".to_string(),"#bbbbbb".to_string()),condition:None},
				AniStr{treash:80 ,format:layout("  ".to_string(),"#229922".to_string(),"#bbbbbb".to_string()),condition:None},
				AniStr{treash:60 ,format:layout("  ".to_string(),"#229922".to_string(),"#bbbbbb".to_string()),condition:None},
				AniStr{treash:40 ,format:layout("  ".to_string(),"#229922".to_string(),"#bbbbbb".to_string()),condition:None},
			    AniStr{treash:20 ,format:layout("  ".to_string(),"#229922".to_string(),"#bbbbbb".to_string()),condition:None},
	        ];

            let mut icon = get_icon(&WType::CPU);

            let redfull_ani = vec![
			    AniStr{treash:100,format:layout(icon.clone(),"#aa2222".to_string(),"#bbbbbb".to_string()),condition:None},
                AniStr{treash:90 ,format:layout(icon.clone(),"#aa4422".to_string(),"#bbbbbb".to_string()),condition:None},
				AniStr{treash:80 ,format:layout(icon.clone(),"#885522".to_string(),"#bbbbbb".to_string()),condition:None},
				AniStr{treash:70 ,format:layout(icon.clone(),"#666622".to_string(),"#bbbbbb".to_string()),condition:None},
				AniStr{treash:60 ,format:layout(icon.clone(),"#447722".to_string(),"#bbbbbb".to_string()),condition:None},
			    AniStr{treash:50 ,format:layout(icon.clone(),"#228822".to_string(),"#bbbbbb".to_string()),condition:None},
                AniStr{treash:0  ,format:layout(icon.clone(),"#229922".to_string(),"#bbbbbb".to_string()),condition:None},
	        ];

            icon = get_icon(&WType::Disk);
            let redfull_ani1 = vec![
			    AniStr{treash:100,format:layout(icon.clone(),"#aa2222".to_string(),"#bbbbbb".to_string()),condition:None},
                AniStr{treash:90 ,format:layout(icon.clone(),"#aa4422".to_string(),"#bbbbbb".to_string()),condition:None},
				AniStr{treash:80 ,format:layout(icon.clone(),"#885522".to_string(),"#bbbbbb".to_string()),condition:None},
				AniStr{treash:70 ,format:layout(icon.clone(),"#666622".to_string(),"#bbbbbb".to_string()),condition:None},
				AniStr{treash:60 ,format:layout(icon.clone(),"#447722".to_string(),"#bbbbbb".to_string()),condition:None},
			    AniStr{treash:50 ,format:layout(icon.clone(),"#228822".to_string(),"#bbbbbb".to_string()),condition:None},
                AniStr{treash:0  ,format:layout(icon.clone(),"#229922".to_string(),"#bbbbbb".to_string()),condition:None},
	        ];
            
            let redfull_ani2 = vec![
			    AniStr{treash:100,format:layout("   ".to_string(),"#aa2222".to_string(),"#bbbbbb".to_string()),condition:None},
                AniStr{treash:90 ,format:layout("   ".to_string(),"#aa4422".to_string(),"#bbbbbb".to_string()),condition:None},
				AniStr{treash:80 ,format:layout("   ".to_string(),"#885522".to_string(),"#bbbbbb".to_string()),condition:None},
				AniStr{treash:70 ,format:layout("   ".to_string(),"#666622".to_string(),"#bbbbbb".to_string()),condition:None},
				AniStr{treash:60 ,format:layout("   ".to_string(),"#447722".to_string(),"#bbbbbb".to_string()),condition:None},
			    AniStr{treash:50 ,format:layout("   ".to_string(),"#228822".to_string(),"#bbbbbb".to_string()),condition:None},
                AniStr{treash:0  ,format:layout("   ".to_string(),"#229922".to_string(),"#bbbbbb".to_string()),condition:None},
	        ];

            icon = get_icon(&WType::RAM);
            let redfull_ani3 = vec![
			    AniStr{treash:100,format:layout(icon.clone(),"#aa2222".to_string(),"#bbbbbb".to_string()),condition:None},
                AniStr{treash:90 ,format:layout(icon.clone(),"#aa4422".to_string(),"#bbbbbb".to_string()),condition:None},
				AniStr{treash:80 ,format:layout(icon.clone(),"#885522".to_string(),"#bbbbbb".to_string()),condition:None},
				AniStr{treash:70 ,format:layout(icon.clone(),"#666622".to_string(),"#bbbbbb".to_string()),condition:None},
				AniStr{treash:60 ,format:layout(icon.clone(),"#447722".to_string(),"#bbbbbb".to_string()),condition:None},
			    AniStr{treash:50 ,format:layout(icon.clone(),"#228822".to_string(),"#bbbbbb".to_string()),condition:None},
                AniStr{treash:0  ,format:layout(icon.clone(),"#229922".to_string(),"#bbbbbb".to_string()),condition:None},
	        ];


            let wifi_replacements = vec![
                Replacement{ from:"Wahlqvist_wifi".to_string(),to: "󰟑".to_string(),},
                Replacement{ from:"SCU".to_string(),to: "󰑴".to_string(),},
                Replacement{ from:"IOT_NET".to_string(),to: "󰘚".to_string(),},
            ];

	    
            let bat_format="<span foreground='#229922'>{icon}</span><span foreground='#bbbbbb'>load time</span>".to_string();
            let right = vec![
                Widget {wtype: WType::CPU,callback: None,cmd: None,format: None,tooltip: None,icon: None,replace_with_icons: None,animate: Some(redfull_ani),warning: None,}, 
                Widget {wtype: WType::Battry,callback: None,cmd: None,format: Some(bat_format),tooltip: None,icon: None,replace_with_icons: None,animate: Some(battery_ani),warning: None,}, 
                Widget {wtype: WType::Alsa,callback: None,cmd: None,format: None,tooltip: None,icon: None,replace_with_icons: None,animate: Some(vol_ani),warning: None,}, 
                Widget {wtype: WType::Disk,callback: None,cmd: Some("/".to_string()),format: None,tooltip: None,icon: None,replace_with_icons: None,animate: Some(redfull_ani1),warning: None,}, 
                Widget {wtype: WType::Disk,callback: None,cmd: Some("/home".to_string()),format: None,tooltip: None,icon: Some("   ".to_string()),replace_with_icons: None,animate: Some(redfull_ani2),warning: None,}, 
                Widget {wtype: WType::Wireless,callback: None,cmd: Some("wlp0s20f3".to_string()),format: None,tooltip: None,icon: None,replace_with_icons: Some(wifi_replacements),animate: Some(wifi_ani),warning: None,}, 
                Widget {wtype: WType::RAM,callback: None,cmd: None,format: None,tooltip: None,icon: None,replace_with_icons: None,animate: Some(redfull_ani3),warning: None,}, 
                Widget {wtype: WType::Temps,callback: None,cmd: None,format: None,tooltip: None,icon: None,replace_with_icons: None,animate: Some(temp_ani),warning: None,}, 
                Widget {wtype: WType::Script,callback: None,cmd: None,format: None,tooltip: None,icon: None,replace_with_icons: None,animate: None,warning: None,},
            ];
    
            let config = RustyBar{ 
                pos: Some(Pos::Top), 
                backgrund: Some("#22222222".to_string()), 
                foregrund: Some("#bbbbbb".to_string()), 
                iconcolor: Some("#229922".to_string()), 
                widgets: Widgets{ 
                    left: Some(left), 
                    center: Some(center), 
                    right: Some(right) 
                }
            };

            let my_config = ron::ser::PrettyConfig::new().struct_names(true).indentor("    ".to_owned());


            let out = ron::ser::to_string_pretty(&config,my_config).unwrap();
            if !dirs::config_dir().unwrap().join(PathBuf::from("rusty_bar/")).is_dir(){
                if std::fs::write("/etc/rusty_bar/rustyconf.ron", out.clone()).is_err(){
                    print!("faild to make new config")
                }
            }

            out
        };
        let config: RustyBar = ron::from_str(conf.as_str()).expect("error in config");
         

        // Initialize layer shell before the window has been fully initialized.
        gtk4_layer_shell::init_for_window(&window);

        // Order above normal windows
        gtk4_layer_shell::set_layer(&window, gtk4_layer_shell::Layer::Top);

        // Push other windows out of the way
        // Toggling this off may help some if they are in applications that have weird unicode text, which may mess with the bars scaling.
        gtk4_layer_shell::auto_exclusive_zone_enable(&window);

        let istop = match config.pos.is_some() {
            true => true,
            false => match config.pos.as_ref().unwrap() {
            Pos::Top => true,
            Pos::Buttom => false,
            }
        };

        gtk4_layer_shell::set_anchor(&window, Edge::Top, istop);
        gtk4_layer_shell::set_anchor(&window, Edge::Right, true);
        gtk4_layer_shell::set_anchor(&window, Edge::Left, true);
        gtk4_layer_shell::set_anchor(&window, Edge::Bottom, !istop);

        // Allows for specifing the namespace of the layer, the default is "gtk-layer-shell" to not break existing configs.
        let namespace = "gtk-layer-shell".to_string();

        gtk4_layer_shell::set_namespace(&window, &namespace);


        // Build all the widgets.
        build_widgets(&window ,config);
	window.show()
    });

    app.run();
}


fn build_widgets(window: &ApplicationWindow ,config:RustyBar) {
    // Create box widgets, which we'll be using to draw the content onto.
    let root = CenterBox::new();
    let left = Box::new(Orientation::Horizontal, 0);
    let centered = Box::new(Orientation::Horizontal, 0);
    let right = Box::new(Orientation::Horizontal, 0);

    root.set_widget_name("root");

    left.set_widget_name("left");
    centered.set_widget_name("centered");
    right.set_widget_name("right");

    
    root.set_center_widget(Some(&centered));
    root.set_end_widget(Some(&right));
    root.set_start_widget(Some(&left));

    window.set_child(Some(&root));
    


    let fcolor = config.foregrund.unwrap_or("#bbbbbb".to_string());
    let icolor=config.iconcolor.unwrap_or("#229922".to_string());
    
    if config.widgets.left.is_some() {
        let wedgets = config.widgets.left.as_ref().unwrap();
        for i in wedgets.iter(){
            wedgit(&i,&left,fcolor.clone(),icolor.clone());
        }
    }

    if config.widgets.center.is_some() {
        let wedgets = config.widgets.center.as_ref().unwrap();
        for i in wedgets.iter(){
            wedgit(&i,&centered,fcolor.clone(),icolor.clone());
        }
    }

    if config.widgets.right.is_some() {
        let wedgets = config.widgets.right.as_ref().unwrap();
        for i in wedgets.iter(){
            wedgit(&i,&right,fcolor.clone(),icolor.clone());
        }
    }
}
    
fn wedgit(wed: &Widget, cont: &Box,fcolor:String,icolor:String) {
    let icon = wed.icon.clone().unwrap_or(get_icon(&wed.wtype));

    let format = if wed.format.is_none(){
	    format!("<span foreground='{icolor}'>{icon}</span><span foreground='{fcolor}'>load</span>")
    } else {
        wed.format.clone().unwrap().replace("{icon}", &icon)
    }; 

    match wed.wtype {
        WType::CPU => {       
            let mut cpu = Cpu::new(format, cont,&wed.animate,&wed.replace_with_icons);
	        let mut tick = move || {
                cpu.tick();
                glib::Continue(true)
            };
            tick();
            glib::timeout_add_local(Duration::from_millis(1000), tick);
        }

        WType::RAM => {
	        let ram = RAM::new(format,cont,&wed.animate,&wed.replace_with_icons);
	        let tick = move || {
		        ram.tick();
		        glib::Continue(true)
	        };
	        tick();
	        glib::timeout_add_local(Duration::from_millis(100), tick);
	    }

        WType::Disk => {
	        let pos = wed.cmd.clone().unwrap_or("/".to_string());
	        let disk = DiskUsage::new(pos,format,cont,&wed.animate,&wed.replace_with_icons);
	        let tick = move || {
		        disk.tick();
		        glib::Continue(true)
	        };
	        tick();
	        glib::timeout_add_local(Duration::from_secs(2), tick);
	    }

        WType::Alsa => {
	        let volume = Volume::new(format,cont,&wed.animate,&wed.replace_with_icons);
	        let tick = move || {
		        volume.tick();
		        glib::Continue(true)
	        };
	        tick();
	        glib::timeout_add_local(Duration::from_millis(200), tick);
	    }

        WType::CLOCK => {
            let clock = Clock::new(format, cont,&wed.replace_with_icons);
            let tick = move || {
                clock.tick();
                glib::Continue(true)
            };
            tick();
            glib::timeout_add_local(Duration::from_millis(10000), tick);
        }

        WType::Temps => {
	        let sens = wed.cmd.clone().unwrap_or("a".to_string());
	        let temps = Temps::new(sens,format,cont,&wed.animate,&wed.replace_with_icons);
	        let tick = move || {
		        temps.tick();
		        glib::Continue(true)
	        };
	        glib::timeout_add_local(Duration::from_millis(100), tick);
	    }

        WType::Battry => {
	    let mut bat = BatteryView::new(format,cont,&wed.animate,&wed.replace_with_icons);
	    let mut tick = move || {
		bat.tick();
		glib::Continue(true)
	    };
	    tick();
	    glib::timeout_add_local(Duration::from_secs(10), tick);
	    }

        WType::Script => {
	        let cmd = wed.cmd.clone().unwrap_or("echo no cmd".to_string());
	        let scrip = Command::new(cont,cmd,format,&wed.replace_with_icons);
	        let tick = move || {
		        scrip.tick();
		        glib::Continue(true)
	        };
	        tick();
	        glib::timeout_add_local(Duration::from_secs(1), tick);
	    }

        WType::Systray => {}
        
        WType::Wireless => {
	        let interface = wed.cmd.clone().unwrap_or("wlan0".to_string());
	        let wire = Wireless::new(format,interface, cont,&wed.animate,&wed.replace_with_icons);
	        let tick = move || {
		        wire.tick();
		        glib::Continue(true)
	        };
	        tick();
	        glib::timeout_add_local(Duration::from_millis(1000), tick);
	    }

        WType::Workspaces => {
            let mut workspaces = Workspaces::new(format,cont,&wed.replace_with_icons);
	        let mut tick = move || {
                workspaces.tick();
                glib::Continue(true)
            };
            tick();
            glib::timeout_add_local(Duration::from_millis(1000), tick);
        }
        WType::ActiveWindow => {
	        let mut windows = ActiveWindowTitle::new(format,cont,&wed.replace_with_icons);
	        let mut tick = move || {
		        windows.tick();
		        glib::Continue(true)
            };
            tick();
            glib::timeout_add_local(Duration::from_millis(1000), tick);
        }
    }
}
fn get_icon(wedgit:&WType) -> String {
    match wedgit{
        WType::Workspaces => "".to_string(),
        WType::CLOCK => " 󱑎 ".to_string(),
        WType::ActiveWindow => "".to_string(),
        WType::CPU => "   ".to_string(),
        WType::RAM => " 󰍛 ".to_string(),
        WType::Wireless => "  ".to_string(),
        WType::Battry => " 󰁿 ".to_string(),
        WType::Systray => "".to_string(),
        WType::Temps => "  ".to_string(),
        WType::Disk => " 󰋊 ".to_string(),
        WType::Alsa => " 󰕾 ".to_string(),
        WType::Script => "  ".to_string(),
    } 
}


fn dlayout(icon:String,icolor:String,fcolor:String)->String{
   format!("<span foreground='{icolor}'>{icon}</span><span foreground='{fcolor}'>load time</span>") 
}

fn layout(icon:String,icolor:String,fcolor:String)->String{
    format!("<span foreground='{icolor}'>{icon}</span><span foreground='{fcolor}'>load</span>") 
 }

 fn wlayout(icon:String,icolor:String,fcolor:String)->String{
    format!("<span foreground='{icolor}'>{icon}</span><span foreground='{fcolor}'>ssid </span>") 
 }

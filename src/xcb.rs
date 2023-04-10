use rusty_bar::clock::Clock;
use rusty_bar::battery::{BatteryView,BatteryInfo};
use battery::State;
use rusty_bar::cpu::Cpu;
use rusty_bar::active_window_title::ActiveWindowTitle;
use rusty_bar::leftwm::{LeftWM,LeftWMAttributes};
use rusty_bar::disk_usage::{DiskUsage,DiskInfo};
use rusty_bar::text::{Font,Color,Attributes,Padding,Threshold};
use rusty_bar::wireless::{Wireless, WirelessInfoStruct};
use rusty_bar::sensors::{Sensors, SensorsInfo,TempUnit};
use rusty_bar::volume::{Volume, VolumeInfo};
use rusty_bar::widget::Cnx;
use rusty_bar::bar::Position;
use anyhow::Result;



fn template(icon:String,info:String) -> String{
    let c1="#00ee00";
    let c2="#eeeeee";
    format!("<span foreground=\"{c1}\">{icon}</span><span foreground=\"{c2}\">{info}</span>")
}



fn attr() -> Attributes {
    Attributes{
	font: Font::new("Hack Nerd Font 11"),
	fg_color: Color::from_hex("#eeeeee"),
	bg_color: None,
	padding: Padding::new(8.0, 8.0, 0.0, 0.0),
    }
}



fn main() -> Result<()> {
    let focused = Attributes {
        fg_color: Color::from_hex("#55ff55"),
	padding: Padding::new(8.0, 8.0, 0.0, 0.0),
	bg_color: Some(Color::from_hex("#222222")),
	 ..attr()
        };
    let visible = Attributes {
        fg_color: Color::green(),
        ..attr()
    };
    let busy = Attributes {
	fg_color: Color::from_hex("#119911"),
	padding: Padding::new(1.0, 1.0, 0.0, 0.0),
	..attr()
    };
    let empty = Attributes {
	fg_color: Color::from_hex("#bbbbbb"),
	padding: Padding::new(1.0, 1.0, 0.0, 0.0),
	..attr()
    };    
    let pager = LeftWM::new("eDP-1".to_string(),LeftWMAttributes {focused,visible,busy,empty});


    let battery_render = Box::new(|battery_info: BatteryInfo| {
    let percentage = battery_info.capacity as i32;
	let time = match battery_info.time.is_some() {
		true => battery_info.time.unwrap().value,
		false => 0.0,
	};
	let mut min = (time/60.0).round() as i32;
	let mut hour = 0;
	while min>60 {
	    min-=60;
	    hour+=1;
	}
	let icon = match battery_info.state {
	    State::Full=> "󰂄 ",
	    State::Charging=> match percentage {
		d if d > 90 => "󰂋 ",
		d if d > 80 => "󰂊 ",
		d if d > 70 => "󰢞 ",
		d if d > 60 => "󰂉 ",
		d if d > 50 => "󰢝 ",
		d if d > 40 => "󰂈 ",
		d if d > 30 => "󰂇 ",
		d if d > 20 => "󰂆 ",
		d if d > 10 => "󰢜 ",
		_ => "󰢟 ",
	    },
	    State::Discharging=> match percentage {
		d if d > 90 => "󰂂 ",
		d if d > 80 => "󰂁 ",
		d if d > 70 => "󰂀 ",
		d if d > 60 => "󰁿 ",
		d if d > 50 => "󰁾 ",
		d if d > 40 => "󰁽 ",
		d if d < 30 => "󰁼 ",
		d if d > 20 => "󰁻 ",
		d if d > 10 => "󰁺 ",
		_ => "󰂎 ",
	    },
	    State::Unknown=> "󰂑  ",
	    State::Empty=> "󱉞  ",
	    State::__Nonexhaustive=> "󰂃 ",
	};
	
    let default_text = if time > 60.0 {
		format!("{percentage:0.}% {hour}:{min:0>2}")
	} else {
		format!("{percentage:0.}% none")
	};
	template(String::from(icon),default_text)
   });

    let battery = BatteryView::new(attr(), Color::red(), Some(battery_render));

   
    let root_render=Box::new(|disk_info: DiskInfo| {
        let left = (disk_info.used.get_bytes()*100)/(disk_info.total.get_bytes());
        let disk_text = format!("{left}%");
        template(String::from(" "), disk_text)
    });
    let root = DiskUsage::new(attr(), String::from("/"), Some(root_render)); 


    let home_render=Box::new(|disk_info: DiskInfo| {
        let left = (disk_info.used.get_bytes()*100)/(disk_info.total.get_bytes());
        let disk_text = format!("{left}%");
        template(String::from(" "), disk_text)
    });
    let home = DiskUsage::new(attr(), String::from("/home"), Some(home_render)); 


    let cpu_render = Box::new(|load| {
	
        template(String::from(" "),format!("{:2}%",load))
    });
    let cpu = Cpu::new(attr(), Some(cpu_render))?;
    

    let mut cnx = Cnx::new(Position::Top);


    let wireless_render=Box::new(|wireless: WirelessInfoStruct| {
	let text = match wireless.ssid.as_str() {
	    "Wahlqvist_wifi" => "󰟑",
	    "SCU" => "󰑴",
	    "IOT_NET" => "󰘚",
	    _=> wireless.ssid.as_str(),
	};

	let icon = match wireless.signal {
	    d if d > 90 => "󰤨 ",
	    d if d > 60 => "󰤥 ",
	    d if d > 40 => "󰤢 ",
	    d if d > 20 => "󰤟 ",
	    d if d > 5  => "󰤯 ",
	    _ => "󰤮 ",
	};


	template(icon.to_string(), text.to_string())
    });

    let volume_render = Box::new(|info: VolumeInfo|{
	let text = if !info.if_mute {
	    format!("{}%",info.volume)
	} else {"mute".to_string()};

	let icon = match info.if_mute {
	    true => "󰝟 ",
	    false => match info.volume {
		d if d > 80 => " ",
		d if d > 50 => "󰕾 ",
		d if d > 20 => "󰖀 ",
		_ => "󰕿 ",}};

	template(icon.to_string(), text)
    });

    let sensors_render = Box::new(|info: SensorsInfo|{
	let temp = &info.temp.as_str()[..info.temp.len()-2].to_string();
	let tempnr: i32 = temp.parse().unwrap();
	    
	let text = match info.unit {
	    TempUnit::SI => format!("{}󰔄",tempnr),
	    TempUnit::Imperial => format!("{}󰔅",tempnr)
	};
	let icon = " ".to_string();
	template(icon, text)
    });
    
    let clock_format = format!("{} {} {}",
			       template(" ".to_string(),"%d/%m/%y".to_string()),
			       template("󱨰 ".to_string(),"%a".to_string()),
			       template("󱑎 ".to_string(),"%H:%M".to_string()),
    );



    
    cnx.add_widget(pager);
    cnx.add_widget(ActiveWindowTitle::new(attr()));
    cnx.add_widget(cpu);
    cnx.add_widget(root);
    cnx.add_widget(home);
    cnx.add_widget(Wireless::new(attr(),String::from("wlan0"),Some(Threshold::default()),Some(wireless_render)));
    cnx.add_widget(Sensors::new(attr(),vec!["Package id 0"],Some(sensors_render)));
    cnx.add_widget(Volume::new(attr(),Some(volume_render)));
    cnx.add_widget(battery);

    cnx.add_widget(Clock::new(attr(),Some(clock_format)));
    cnx.run()?;

    Ok(())
    }

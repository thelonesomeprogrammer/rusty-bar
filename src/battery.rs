use battery::{units::Time, Battery, Manager, State};
use gtk::prelude::ContainerExt;
use gtk::prelude::LabelExt;
use gtk::Label;
use gtk::*;
use crate::AniStr;
/// Represent Battery's operating status

/// Shows battery charge percentage
///
/// This widget shows the battery's current charge percentage.
///
/// When the battery has less than 10% charge remaining, the widget's text will
/// change to the specified `warning_color`.
///
/// Battery charge information is read from [`/sys/class/power_supply/BAT0/`].
///
/// [`/sys/class/power_supply/BAT0/`]: https://www.kernel.org/doc/Documentation/power/power_supply_class.txt
pub struct BatteryView {
    label: Label,
    format: String,
    animation: Vec<AniStr>,
}

/// Represent Battery information
#[derive(Clone, Debug, PartialEq)]
struct BatteryInfo {
    /// Battery Status
    pub state: State,
    /// Capacity in percentage
    pub capacity: f32,
    /// time to full and time to empty
    pub time: Option<Time>,
}


impl BatteryView {
    pub fn new<'a>(format:String, con: &Box, refanimation:&'a Option<Vec<AniStr>>) -> BatteryView {
        let label = Label::new(None);
	    con.add(&label);

	    BatteryView { label, format, animation: refanimation.as_ref().unwrap_or(&Vec::new()).to_vec() }
    }

    fn get_value(&mut self) -> Option<BatteryInfo> {
        let managero = Manager::new();
        if managero.is_err(){
            return None
        }
        let manager = managero.unwrap();
        let mut batter: Option<Battery> = None;
        let batteries = manager.batteries();
        if batteries.is_err(){
            return None;
        }

        for (_, maybe_battery) in manager.batteries().unwrap().enumerate() {
            batter = Some(maybe_battery.unwrap());
        };
        if batter.is_none(){
            return None
        }
        let mut bat = batter.unwrap();
        if bat.refresh().is_err(){
            return None;
        }
        let capacity: f32 = bat.state_of_charge().value * 100.0;
        let state: State = bat.state();
        let time: Option<Time> = if bat.time_to_full().is_some() {
            Some(bat.time_to_full().expect(""))
        } else if bat.time_to_empty().is_some() {
            Some(bat.time_to_empty().expect(""))
        } else {
            None
        };
        Some(BatteryInfo {
            capacity,
            state,
            time,
            })


    }

    pub fn tick(&mut self) {
        let battery_infoo = self.get_value();
        if battery_infoo.is_none(){
            return;
        }
        let battery_info = battery_infoo.unwrap();
	    let percentage = battery_info.capacity.round() as u8;
	
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
	let timetext = format!("{hour}:{min:0>2}");

	let is_cherching = match battery_info.state {
	    State::Unknown => false,
	    State::Charging => true,
	    State::Discharging => false,
	    State::Empty => false,
	    State::Full => false,
	    _ => false,
};

	    let format = if self.animation.len() != 0 {
	        crate::animate(percentage, self.animation.to_vec(), is_cherching)
	    } else {
	        self.format.clone()
	    };
	
	    let text = format.clone().as_str()
	        .replace("load",&format!("{percentage:0>2}%").as_str()).as_str()
	        .replace("time", &timetext).as_str()
	        .replace("Min", &format!("{:0>3}",(time/60.0).round() as i32)).as_str()
	        .replace("min", &format!("{min:0>2}")).as_str()
	        .replace("hour", &format!("hour"));
	    self.label.set_markup(&text)
    }
}

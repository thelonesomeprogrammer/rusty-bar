use anyhow::Result;
use battery::{units::Time, Battery, Manager, State};
use gtk::prelude::ContainerExt;
use gtk::prelude::LabelExt;
use gtk::prelude::WidgetExt;
use gtk::Label;
use gtk::*;
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
    bat: Battery,
    label: Label,
}

/// Represent Battery information
#[derive(Clone, Debug, PartialEq)]
pub struct BatteryInfo {
    /// Battery Status
    pub state: State,
    /// Capacity in percentage
    pub capacity: f32,
    /// time to full and time to empty
    pub time: Option<Time>,
}

impl BatteryView {
    pub fn new(con: &Box) -> BatteryView {
        let manager = Manager::new().expect("no bat");
        let mut batter: Option<Battery> = None;
        for (_, maybe_battery) in manager.batteries().unwrap().enumerate() {
            batter = Some(maybe_battery.expect("no bat"));
        }
        let bat = batter.expect("no bat");
        let label = Label::new(None);
        BatteryView { bat, label }
    }

    fn get_value(&mut self) -> Result<BatteryInfo> {
        self.bat.refresh()?;
        let capacity: f32 = self.bat.state_of_charge().value * 100.0;
        let state: State = self.bat.state();
        let time: Option<Time> = if self.bat.time_to_full().is_some() {
            Some(self.bat.time_to_full().expect(""))
        } else if self.bat.time_to_empty().is_some() {
            Some(self.bat.time_to_empty().expect(""))
        } else {
            None
        };

        Ok(BatteryInfo {
            capacity,
            state,
            time,
        })
    }

    fn tick(&mut self) {
        let battery_info = self.get_value().expect("msg");
        let default_text = format!(
            "({percentage}%)",
            percentage = battery_info.capacity.round() as i32,
        );
    }
}

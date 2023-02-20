use anyhow::Result;
use crate::text::{Attributes, Color, Text};
use crate::widget::{Widget, WidgetStream};
use std::str::FromStr;
use std::time::Duration;
use tokio::time;
use tokio_stream::wrappers::IntervalStream;
use tokio_stream::StreamExt;
use battery::{Manager,Battery,State,units::Time};

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
    update_interval: Duration,
    attr: Attributes,
    warning_color: Color,
    bat: Battery,
    render: Option<Box<dyn Fn(BatteryInfo) -> String>>,
}

/// Represent Battery information
#[derive(Clone, Debug, PartialEq)]
pub struct BatteryInfo {
    /// Battery Status
    pub state: State,
    /// Capacity in percentage
    pub capacity: f32,
    /// time to full and time to empty
    pub time:Option<Time>,
}

impl BatteryView {
    ///  Creates a new Battery widget.
    ///
    ///  Creates a new `Battery` widget, whose text will be displayed with the
    ///  given [`Attributes`]. The caller can provide use the `warning_color`
    ///  argument, to control the [`Color`] of the text once the battery has
    ///  less than 10% charge remaining.
    ///
    ///  The [`cnx::Cnx`] instance is borrowed during construction in order to get
    ///  access to handles of its event loop. However, it is not borrowed for
    ///  the lifetime of the widget. See the [`cnx::Cnx::add_widget`] for more
    ///  discussion about the lifetime of the borrow.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[macro_use]
    /// # extern crate cnx;
    /// #
    /// # use cnx::*;
    /// # use cnx::text::*;
    /// # use cnx::widgets::*;
    /// # use cnx_contrib::widgets::battery::*;
    /// # use anyhow::Result;
    /// #
    /// # fn run() -> Result<()> {
    /// let attr = Attributes {
    ///     font: Font::new("SourceCodePro 21"),
    ///     fg_color: Color::white(),
    ///     bg_color: None,
    ///     padding: Padding::new(8.0, 8.0, 0.0, 0.0),
    /// };
    ///
    /// let mut cnx = Cnx::new(Position::Top);
    /// cnx.add_widget(Battery::new(attr.clone(), Color::red(), None, None));
    /// # Ok(())
    /// # }
    /// # fn main() { run().unwrap(); }
    /// ```
    pub fn new(
        attr: Attributes,
        warning_color: Color,
        render: Option<Box<dyn Fn(BatteryInfo) -> String>>,
    ) -> BatteryView {
	let manager = Manager::new().expect("no bat");
	let mut batter: Option<Battery>=None;
	for (_,maybe_battery) in manager.batteries().unwrap().enumerate() {
            batter = Some(maybe_battery.expect("no bat"));}
	let bat = batter.expect("no bat");
        BatteryView {
            update_interval: Duration::from_secs(60),
            attr,
            warning_color,
            render,
	    bat,
        }
    }

    fn get_value(&mut self) -> Result<BatteryInfo> {
	self.bat.refresh()?;
        let capacity: f32 = self.bat.state_of_charge().value*100.0;
        let state: State = self.bat.state();
	let time:Option<Time> = if self.bat.time_to_full().is_some(){
	    Some(self.bat.time_to_full().expect("no time"))
	} else if self.bat.time_to_empty().is_some() {
	    Some(self.bat.time_to_empty().expect("no time"))
	} else {
	    None
	};
	
        Ok(BatteryInfo { capacity, state, time })
    }

    fn tick(&mut self) -> Result<Vec<Text>> {
        let battery_info = self.get_value()?;
        let default_text = format!("({percentage}%)", percentage = battery_info.capacity.round() as i32,);
        let text = self
            .render
            .as_ref()
            .map_or(default_text, |x| (x)(battery_info.clone()));

        // If we're discharging and have <=10% left, then render with a
        // special warning color.
        let mut attr = self.attr.clone();
        if battery_info.state == State::Discharging && battery_info.capacity <= 10.0 {
            attr.fg_color = self.warning_color.clone()
        }

        Ok(vec![Text {
            attr,
            text,
            stretch: false,
            markup: self.render.is_some(),
        }])
    }
}

impl Widget for BatteryView {
    fn into_stream(mut self: Box<Self>) -> Result<WidgetStream> {
        let interval = time::interval(self.update_interval);
        let stream = IntervalStream::new(interval).map(move |_| self.tick());

        Ok(Box::pin(stream))
    }
}

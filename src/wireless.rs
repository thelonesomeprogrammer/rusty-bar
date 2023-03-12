use anyhow::Result;
use iwlib::*;
use std::time::Duration;

/// Wireless widget to show wireless information for a particular ESSID
pub struct Wireless {
    interface: String,

}



pub struct WirelessInfoStruct{
    pub ssid: String,
    pub signal: u8,
}

impl Wireless {
    /// Creates a new [`Wireless`] widget.
    ///
    /// Arguments
    ///
    /// * `attr` - Represents `Attributes` which controls properties like
    /// `Font`, foreground and background color etc.
    ///
    /// * `interface` - String representing the name name of the network
    /// interface for your wireless hardware. In Linux systems, you can
    /// find that out using `iw dev` command.
    ///
    /// * `threshold` - Represents threshold values to determine if
    /// the wireless strength is low, normal or high.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[macro_use]
    /// # extern crate cnx;
    /// #
    /// # use cnx::*;
    /// # use cnx::text::*;
    /// # use cnx_contrib::widgets::wireless::*;
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
    /// cnx.add_widget(Wireless::new(attr, "wlp2s0".into(),  None));
    /// # Ok(())
    /// # }
    /// # fn main() { run().unwrap(); }
    /// ```
    pub fn new(interface: String, threshold: Option<Threshold>, render: Option<Box<dyn Fn(WirelessInfoStruct) -> String>>) -> Wireless {
        Wireless {
            interface,

        }
    }

    fn tick(&self) -> Vec<Text> {
        let wireless_info = get_wireless_info(self.interface.clone());

        let d_text = match wireless_info {
            Some(info) => match &self.threshold {
                Some(thold) => {
                    let color = if info.wi_quality <= thold.low.threshold {
                        &thold.low.color
                    } else if info.wi_quality <= thold.normal.threshold {
                        &thold.normal.color
                    } else {
                        &thold.high.color
                    };
                    format!("{} <span foreground=\"{}\">{}%</span>",
                        info.wi_essid,
                        color.to_hex(),
                        info.wi_quality,
                    )
                }
                None => format!("{} {}%", info.wi_essid, info.wi_quality),
            },
            None => "NA".to_owned(),
        };
        let wireless_info2 = get_wireless_info(self.interface.clone()).unwrap();
	
	let info = WirelessInfoStruct{
	    ssid: wireless_info2.wi_essid.to_string().clone(),
	    signal: wireless_info2.wi_quality.clone(),
	};
        let text = self.render.as_ref().map_or(d_text, |x| (x)(info));
	let markup = if self.threshold.is_some(){
	    true
	} else if self.render.is_some(){
	    true
	} else {
	    false
	};
        vec![Text {
            attr: self.attr.clone(),
            text,
            stretch: false,
            markup: markup,
        }]
    }
}
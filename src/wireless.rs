use iwlib::*;
use gtk::{Box,Label};
use gtk::prelude::{ContainerExt,LabelExt};

/// Wireless widget to show wireless information for a particular ESSID
pub struct Wireless {
    interface: String,
    label: Label,
    format: String,
}



impl Wireless {
    pub fn new(format:String,interface: String, con:&Box) -> Wireless {
	    let label = Label::new(None);
	    con.add(&label);
        Wireless { interface, label, format }
    }

    pub fn tick(&self){
        let wireless = get_wireless_info(self.interface.clone());
	    let text = if wireless.is_some(){
	        self.format.as_str().replace("load",&format!("{}%",wireless.as_ref().unwrap().wi_quality))
		    .as_str().replace("ssid", &format!("{}",wireless.as_ref().unwrap().wi_essid))
	    } else {
	        self.format.as_str().replace("load", "NA")
	    };
	    self.label.set_markup(&text);
    }
}

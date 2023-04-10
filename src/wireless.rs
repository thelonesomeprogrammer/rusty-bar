use iwlib::*;
use gtk::{Box,Label};
use gtk::prelude::{ContainerExt,LabelExt};
use crate::{AniStr,Replacement,replacements,animate};

/// Wireless widget to show wireless information for a particular ESSID
pub struct Wireless {
    interface: String,
    label: Label,
    format: String,
    animation: Vec<AniStr>,
    replacements: Vec<Replacement>,
}



impl Wireless {
    pub fn new<'a>(
        format:String,
        interface: String, 
        con:&Box, 
        refanimation:&'a Option<Vec<AniStr>>, 
        refreplacement:&'a Option<Vec<Replacement>>,
    ) -> Wireless {
	    let label = Label::new(None);
	    con.add(&label);
        Wireless { 
            interface, 
            label, 
            format, 
            animation: refanimation.as_ref().unwrap_or(&Vec::new()).to_vec(),
            replacements: refreplacement.as_ref().unwrap_or(&Vec::new()).to_vec(),
        }
    }

    pub fn tick(&self){
        let wireless = get_wireless_info(self.interface.clone());
	    let text = replacements(if wireless.is_some(){
            let percentage = wireless.as_ref().unwrap().wi_quality;
            let format = if self.animation.len() != 0 {
                animate(percentage, self.animation.to_vec(), true)
            } else {
                self.format.clone()
            };
	        format.as_str().replace("load",&format!("{}%",percentage))
		    .as_str().replace("ssid", &format!("{}",wireless.as_ref().unwrap().wi_essid))
	    } else {
            let format = if self.animation.len() != 0 {
                animate(0, self.animation.to_vec(), false)
            } else {
                self.format.clone()
            };
	        format.as_str().replace("load", "NA").as_str().replace("ssid", "")
	    },self.replacements.to_vec());
	    self.label.set_markup(&text);
    }
}

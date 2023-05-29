use gtk4::{Label,Box};
use gtk4::prelude::BoxExt;
use psutil::sensors::temperatures;
use crate::{AniStr,Replacement,replacements,animate};


pub struct Temps {
    label: Label,
    sens: String,
    format: String,
	animation: Vec<AniStr>,
	replacements: Vec<Replacement>,
}


impl Temps {
    pub fn new<'a>(
		sens:String,
		format:String,
		con:&Box, 
		refanimation:&'a Option<Vec<AniStr>>, 
		refreplacement:&'a Option<Vec<Replacement>>,
	) -> Self {
		let label = Label::new(None);
		con.append(&label);
		Temps{
			label,
			sens,
			format, 
			animation: refanimation.as_ref().unwrap_or(&Vec::new()).to_vec(),
			replacements: refreplacement.as_ref().unwrap_or(&Vec::new()).to_vec(),}
    }

    pub fn tick(&self){
		let temps = temperatures();
		for sens in temps.iter() {
	   		if sens.is_ok(){
				if sens.as_ref().unwrap().label().is_some(){
		    		if sens.as_ref().unwrap().label().unwrap().contains(self.sens.as_str()){
						let temp = sens.as_ref().unwrap().current().celsius().round() as u8;
						let format = if self.animation.len() != 0 {
							animate(temp, self.animation.to_vec(), true) 
						} else {
							self.format.clone()
						};
						let text = replacements(
							format.as_str().replace("load",&format!("{temp}󰔄")),
							self.replacements.to_vec());
						self.label.set_markup(&text)
		    		}
				}
	    	}
		}
    }
}

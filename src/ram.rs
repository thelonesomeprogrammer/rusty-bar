use gtk4::{Box,Label};
use gtk4::prelude::BoxExt;
use psutil::memory::virtual_memory;
use crate::{AniStr,Replacement,replacements,animate};



pub struct RAM {
    label: Label,
    format: String,
	animation: Vec<AniStr>,
	replacements: Vec<Replacement>,
}

impl RAM {
    pub fn new<'a>(
		format:String, 
		con:&Box, 
		refanimation:&'a Option<Vec<AniStr>>,
		refreplacement:&'a Option<Vec<Replacement>>,
	) -> Self {
		let label = Label::new(None);
		con.append(&label);
		RAM{
			label, 
			format, 
			animation: refanimation.as_ref().unwrap_or(&Vec::new()).to_vec(),
			replacements: refreplacement.as_ref().unwrap_or(&Vec::new()).to_vec(),
		}
    }

    pub fn tick(&self){
		let ram = virtual_memory();
		let format;
		let load = if ram.is_ok() {
			let percentage = ram.unwrap().percent().round() as u8;
			format = if self.animation.len() != 0 {
				animate(percentage, self.animation.to_vec(), true) 
			} else {
				self.format.clone()
			};
	    	format!("{percentage:0>2}%")
		} else {
			format = if self.animation.len() != 0 {
				animate(0, self.animation.to_vec(), false) 
			} else {
				self.format.clone()
			};
			format!("{}","NA")
		};



		let text = replacements(format.as_str().replace("load", &load),self.replacements.to_vec());
		self.label.set_markup(&text);
    }
}

use gtk::{Box,Label};
use gtk::prelude::{LabelExt,ContainerExt};
use psutil::memory::virtual_memory;
use crate::AniStr;



pub struct RAM {
    label: Label,
    format: String,
	animation: Vec<AniStr>,
}

impl RAM {
    pub fn new<'a>(format:String, con:&Box, refanimation:&'a Option<Vec<AniStr>>) -> Self {
		let label = Label::new(None);
		con.add(&label);
		RAM{label, format, animation: refanimation.as_ref().unwrap_or(&Vec::new()).to_vec()}
    }

    pub fn tick(&self){
		let ram = virtual_memory();
		let format;
		let load = if ram.is_ok() {
			let percentage = ram.unwrap().percent().round() as u8;
			format = if self.animation.len() != 0 {
				crate::animate(percentage, self.animation.to_vec(), true) 
			} else {
				self.format.clone()
			};
	    	format!("{percentage:0>2}%")
		} else {
			format = if self.animation.len() != 0 {
				crate::animate(0, self.animation.to_vec(), false) 
			} else {
				self.format.clone()
			};
			format!("{}","NA")
		};



		let text = format.as_str().replace("load", &load);
		self.label.set_markup(&text);
    }
}
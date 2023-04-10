use alsa::mixer::{SelemChannelId, SelemId};
use alsa::{self, Mixer};
use gtk::{Label, Button, Box};
use gtk::prelude::{LabelExt,ButtonExt,ContainerExt};
use crate::{AniStr,Replacement,replacements,animate};

/// Shows the current volume of the default ALSA output.
///
/// This widget shows the current volume of the default ALSA output, or '`M`' if
/// the output is muted.


pub struct Volume {
    label: Label,
    format: String,
	animation: Vec<AniStr>,
	replacements: Vec<Replacement>,
}

impl Volume {
    pub fn new<'a>(
		format: String,
		con:&Box, 
		refanimation:&'a Option<Vec<AniStr>>,
		refreplacement:&'a Option<Vec<Replacement>>,
	) -> Volume {
		let label = Label::new(None);
		let button = Button::new();
		button.set_relief(gtk::ReliefStyle::None);
		button.set_border_width(0);
		button.add(&label);
		button.connect_clicked(|_|{
	    	let mixer_res = Mixer::new("default", true);
			if mixer_res.is_err(){
				return;
			}
			let mixer = mixer_res.as_ref().unwrap();
	    	let channel = SelemChannelId::FrontLeft;
	    	let master_res = mixer.find_selem(&SelemId::new("Master", 0));
			if master_res.is_none(){
				return;
			}
			let master = master_res.unwrap();
	    	let state = master.get_playback_switch(channel);
			if state.is_err(){
				return;
			}
			let mute = state.unwrap() == 0;
	    	if !mute {
				if master.set_playback_switch(channel, 0).is_ok(){
					return;
				}
			} else {
				if master.set_playback_switch(channel, 1).is_ok(){
					return;
				}
	    	}
			print!("volume error could not toggle ")
		});
	con.add(&button);
        Volume { 
			label, 
			format, 
			animation: refanimation.as_ref().unwrap_or(&Vec::new()).to_vec(),
			replacements: refreplacement.as_ref().unwrap_or(&Vec::new()).to_vec(),
		}
    }

    pub fn tick(&self){
		let mixer_res = Mixer::new("default", true);
		if mixer_res.is_err(){
			return;
		}
		let mixer = mixer_res.as_ref().unwrap();
		let channel = SelemChannelId::FrontLeft;
		let master_res = mixer.find_selem(&SelemId::new("Master", 0));
		if master_res.is_none(){
			return;
		}
		let master = master_res.unwrap();
		let state = master.get_playback_switch(channel);
		if state.is_err(){
			return;
		}
		let mute = state.unwrap() == 0;
		let volume_res = master.get_playback_volume(channel);
		if volume_res.is_err(){
			return;
		}
		let (min, max) = master.get_playback_volume_range();
		let percentage = ((volume_res.unwrap() as f64 / (max as f64 - min as f64)) * 100.0).round() as u8;

		let format;
		let insert = if !mute {
			format = if self.animation.len() != 0 {
				animate(percentage, self.animation.to_vec(), true) 
			} else {
				self.format.clone()
			};
	    	format!("{}%",percentage)
		} else {
			format = if self.animation.len() != 0 {
				animate(1, self.animation.to_vec(), false) 
			} else {
				self.format.clone()
			};
			"mute".to_string()
		};
		let text = replacements(
			format.as_str().replace("load", &insert),
			self.replacements.to_vec());
	
		self.label.set_markup(&text);
    }
}

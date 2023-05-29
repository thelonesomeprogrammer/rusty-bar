use gtk4::prelude::WidgetExt;
use gtk4::*;
use gtk4::prelude::BoxExt;
use gtk4::Label;
use crate::{Replacement,replacements};
/// Shows the current time and date.
///
/// This widget shows the current time and date, in the form `%Y-%m-%d %a %I:%M
/// %p`, e.g. `2017-09-01 Fri 12:51 PM`.
pub struct Clock {
    format: String,
    label: Label,
    replacements: Vec<Replacement>,
}

impl Clock {
    // Creates a new Clock widget.
    pub fn new<'a>(format: String,  
        con:&Box,
        refreplacement:&'a Option<Vec<Replacement>>,) -> Clock {
        let label = Label::new(None);
        label.set_widget_name("Clock");
        label.set_markup(format.as_str());
        con.append(&label);
        Clock { 
            format, 
            label,
            replacements: refreplacement.as_ref().unwrap_or(&Vec::new()).to_vec(),
        }
    }

    pub fn tick(&self){
        let now = chrono::Local::now();
	    let format = self.format.as_str().replace("load", "%H:%M");
        let text = replacements(
            now.format(&format).to_string(),
            self.replacements.to_vec());
        self.label.set_markup(&text);
    }

}

use gtk::prelude::WidgetExt;
use gtk::prelude::LabelExt;
use gtk::*;
use gtk::prelude::ContainerExt;
use gtk::Label;

/// Shows the current time and date.
///
/// This widget shows the current time and date, in the form `%Y-%m-%d %a %I:%M
/// %p`, e.g. `2017-09-01 Fri 12:51 PM`.
pub struct Clock {
    format: String,
    label: Label,
}

impl Clock {
    // Creates a new Clock widget.
    pub fn new(format: String,  con:&Box) -> Self {
        let label = Label::new(None);
        label.set_widget_name("Clock");
        label.set_markup(format.as_str());
        con.add(&label);
        Self { format, label }
    }

    pub fn tick(&self){
        let now = chrono::Local::now();
	    let format = self.format.as_str().replace("load", "%H:%M");
        let text = now.format(&format).to_string();
        self.label.set_markup(&text.as_str());
    }

}

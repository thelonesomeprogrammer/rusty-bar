use anyhow::Result;
use std::time::Duration;
use tokio::time;
use gtk::prelude::WidgetExt;
use gtk::prelude::LabelExt;
use gtk::*;
use gtk::prelude::ContainerExt;
use gtk::Label;
use crate::text::{Attributes, Text};

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
    pub fn new(format: String,  contain:&Box) -> Self {
        let label = Label::new(None);
        label.set_widget_name("Clock");
        label.set_use_markup(true);
        label.set_markup(format.as_str());
        contain.add(&label);
        Self {format, label }
    }

    pub fn tick(&self){
        let now = chrono::Local::now();
        let clock_formating="%Y-%m-%d %a %I:%M %p";
        let text = now.format(&self.format).to_string();
        self.label.set_markup(&text.as_str());
    }

}

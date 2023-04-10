use anyhow::{Context, Result, anyhow};
use gtk::prelude::{LabelExt, ContainerExt};
use gtk::{Box, Label};
use std::process::Command;
use regex::Regex;
use pango::EllipsizeMode;


pub struct ActiveWindowTitle {
    label: Label,
    format: String
}

impl ActiveWindowTitle {
    /// Creates a new Active Window Title widget.
    pub fn new(format: String ,con: &Box) -> ActiveWindowTitle {
	let label = Label::new(None);
	label.set_ellipsize(EllipsizeMode::End);
	con.add(&label);
        ActiveWindowTitle { label, format }
    }

    pub fn tick(&mut self) {
	if get_focused().is_ok(){
	    let text = self.format.as_str().replace("load", &format!("{}",get_focused().unwrap()));
	    self.label.set_markup(&text);}
    }
}
fn get_focused() -> Result<String>{
	let re: Regex = Regex::new(
            r"title: (?P<name>[\S| |]+)",
	)
	    .map_err(|_| anyhow!("Failed to compile regex for parsing sensors output")).expect("work2");

	 
	let output = Command::new("hyprctl")
            .arg("activewindow")
            .output()
            .context("Failed to run `workspaces`")
            .expect("work1");

	let mut map = Vec::new();
	for cap in re.captures_iter(String::from_utf8(output.stdout).expect("work3").as_str()) {
	    map.push(cap.name("name").unwrap().as_str().replace("&","&amp;").to_string())
	};
	let id = map.pop();
	if id.is_some() {Ok(id.unwrap())
	}else {
	    Err(anyhow!("no fucused window fund"))
	}
}

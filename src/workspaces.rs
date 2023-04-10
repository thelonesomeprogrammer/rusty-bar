use anyhow::{anyhow, Context};
use gtk::prelude::ContainerExt;
use gtk::prelude::LabelExt;
use gtk::prelude::WidgetExt;
use gtk::traits::ButtonExt;
use gtk::Button;
use gtk::*;
use regex::Regex;
use std::process::Command;

struct LabeledButton {
    pub label: Label,
    pub button: Button
}

pub struct Workspaces {
    workspaces: Box,
    buttons: Vec<LabeledButton>,
    format: String,
    
}

struct ActiveWorkspace {
    pub id: u8,
    name: String,
    _monitor: String,
    _windows: u8,
    _lactive: String,
}

fn get_workspaces() -> Vec<ActiveWorkspace> {
    let output = Command::new("hyprctl")
        .arg("workspaces")
        .output()
        .context("Failed to run `workspaces`")
        .expect("work1");

    let re = Regex::new(
        // Note: we ignore + but capture -
        r"(?P<id>[1-9]+) \((?P<name>[\S])\) on monitor (?P<monitor>\S+):\n\t[a-z]+: (?P<windows>[0-9]+)(\n\t\S+\s\S+){2}\n\t\S+\s(?P<lactive>(\w+| |@|:|,|~|/|&|-|\*|\.|●)+)",
    )
    .map_err(|_| anyhow!("Failed to compile regex for parsing sensors output"));
	if re.is_err(){
		return Vec::new();
	}

    let mut map = Vec::new();
    for mat in re.unwrap().captures_iter(String::from_utf8(output.stdout).expect("work3").as_str()) {
        map.push(ActiveWorkspace {
            id:mat.name("id").unwrap().as_str().parse::<u8>().unwrap() as u8,
            name: mat.name("name").unwrap().as_str().to_string(),
            _monitor: mat.name("monitor").unwrap().as_str().to_string(),
            _windows:mat.name("windows").unwrap().as_str().parse::<u8>().unwrap() as u8,
            _lactive: mat.name("lactive").unwrap().as_str().to_string(),
        });
		// These .unwraps() are harmless. If we have a match, we have these results.
    }
    map
}

impl Workspaces {
    pub fn new(format:String,con: &Box) -> Self {
        let container = Box::new(Orientation::Horizontal, 0);
        let mut workspaces = get_workspaces();
		workspaces.sort_by(|a, b| a.id.cmp(&b.id));
		let mut buttons = Vec::new();
        for i in workspaces.iter() {
            let but = Button::new();
	    	let lab = Label::new(None);
            but.set_border_width(0);
            but.set_relief(ReliefStyle::None);
	    	let text = format.as_str().replace("load", "").as_str().replace("name",&format!("{}",i.name));
	    	lab.set_markup(&text);
	    	but.set_widget_name(&i.name);
	    	but.add(&lab);
            but.connect_clicked(|button| {
                if Command::new("hyprctl")
                    .args([
                        "dispatch",
                        "workspace",
                        &format!("name:{}", button.widget_name()),
                    ])
                    .output().is_err() {
						print!("hyprctl could not be reached");
					}
            });
            container.add(&but);
	    	buttons.push(LabeledButton{label: lab ,button: but});
        }
        con.add(&container);
        Workspaces { workspaces: container, buttons, format }
    }



    pub fn tick(&mut self){
	let mut workspaces = get_workspaces();
	
	workspaces.sort_by(|a, b| a.id.cmp(&b.id));
	
	match workspaces.len()as i8-self.workspaces.children().len()as i8 {
	    d if d < 0 =>{
		for i in 0..workspaces.len()-1{
		    let text = self.format.as_str().replace("load", "").as_str()
			.replace("name",&format!("{}",workspaces[i].name));
		    self.buttons[i].label.set_markup(&text);
		    self.buttons[i].button.set_widget_name(&workspaces[i].name)
		}
		for i in workspaces.len()..self.workspaces.children().len(){
		    self.buttons.pop();
		    self.workspaces.children()[i].hide();
		    self.workspaces.remove(&self.workspaces.children()[i]);
		}
	    }
,
	    d if d > 0 =>{
		for i in 0..self.buttons.len()-1{
		    let text = self.format.as_str().replace("load", "").as_str()
			.replace("name",&format!("{}",workspaces[i].name));
		    self.buttons[i].label.set_markup(&text);
		    self.buttons[i].button.set_widget_name(&workspaces[i].name)
		}
		for i in self.workspaces.children().len()..workspaces.len() {
		    let but = Button::new();
		    let lab = Label::new(None);
		    but.set_border_width(0);
		    but.set_relief(ReliefStyle::None);
		    let text = self.format.as_str().replace("load", "").as_str()
			.replace("name",&format!("{}",workspaces[i].name));
		    lab.set_markup(&text);
		    but.add(&lab);
		    but.connect_clicked(|button| {
				if Command::new("hyprctl")
			    	.args([
					"dispatch",
					"workspace",
					&format!("name:{}", button.widget_name()),
			    	])
			    	.output().is_err() {
						print!("hyprctl could not be reached");
					}
		    });
		    but.set_widget_name(&workspaces[i].name);
		    but.show_all();
		    self.workspaces.add(&but);
		    self.buttons.push(LabeledButton { label: lab, button: but })
		}},
	    d if d == 0 =>
		for i in 0..self.buttons.len(){
		    let text = self.format.as_str().replace("load", "").as_str()
			.replace("name",&format!("{}",workspaces[i].name));
		    self.buttons[i].label.set_markup(&text);
		    self.buttons[i].button.set_widget_name(&workspaces[i].name)
		},
	    _ =>print!("workspases comparasin failed"),
	}
    }
}

use std::process::Command as Process;
use gtk::{Label,Box};
use gtk::prelude::{LabelExt,ContainerExt};

pub struct Command {
    label: Label,
    command: String,
    format: String,
}

impl Command {
    pub fn new(con:&Box, command: String,format:String) -> Command {
	    let label = Label::new(None);
	    con.add(&label);
        Self { label, command, format }
    }

    pub fn tick(&self){
        let output = Process::new("sh")
            .arg("-c")
            .arg(self.command.clone())
            .output();

        if output.is_ok(){
	        let mut res =String::from_utf8(output.unwrap().stdout).unwrap_or_else(|_| "error\n".into());
	        res.pop();
	        let text = self.format.as_str().replace("load",res.as_str());
	        self.label.set_markup(&text);
        }
    }
}

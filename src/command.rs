use std::process::Command as Process;
use gtk4::{Label,Box};
use gtk4::prelude::BoxExt;
use crate::{Replacement,replacements};
pub struct Command {
    label: Label,
    command: String,
    format: String,
    replacements: Vec<Replacement>,
}

impl Command {
    pub fn new<'a>(
        con:&Box, 
        command: String,
        format:String,
        refreplacement:&'a Option<Vec<Replacement>>,
    ) -> Command {
	    let label = Label::new(None);
	    con.append(&label);
        Command { 
            label, 
            command, 
            format,
            replacements: refreplacement.as_ref().unwrap_or(&Vec::new()).to_vec(),
        }
    }

    pub fn tick(&self){
        let output = Process::new("sh")
            .arg("-c")
            .arg(self.command.clone())
            .output();

        if output.is_ok(){
	        let mut res =String::from_utf8(output.unwrap().stdout).unwrap_or_else(|_| "error\n".into());
	        res.pop();
	        let text = replacements(
                self.format.as_str().replace("load",res.as_str()),
                self.replacements.to_vec()
            );
	        self.label.set_markup(&text);
        }
    }
}

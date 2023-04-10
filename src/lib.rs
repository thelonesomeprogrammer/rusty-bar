pub mod cpu;
pub mod clock;
pub mod workspaces;
pub mod battery;
pub mod active_window_title;
pub mod volume;
pub mod disk_usage;
pub mod wireless;
pub mod ram;
pub mod temps;
pub mod command;


use serde::{Serialize,Deserialize};
#[derive(Deserialize,Serialize,Debug,Clone)]
pub struct AniStr {
    pub condition: Option<bool>,
    pub treash: u8,
    pub format: String,
}

pub fn animate(load:u8, ani:Vec<AniStr>, cond:bool) -> String{
    for i in ani.iter() {
		if i.treash <= load{
	    	if i.condition.is_some(){
				if i.condition.unwrap()==cond {
		    		return i.format.clone();
				}
	    	} else {
				return i.format.clone();
	    	}
		}
    }
    return "".to_string();
}

#[derive(Deserialize,Serialize,Debug,Clone)]
pub struct Replacement{
    pub from: String,
    pub to: String,
}

pub fn replacements(input:String, replacements:Vec<Replacement>) -> String{
	let mut string = input.clone();
    for i in replacements.iter() {
		string = string.as_str().replace(&i.from, &i.to);
    }
    return string;
}
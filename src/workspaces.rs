use anyhow::{anyhow, Context, Result};
use gtk::prelude::WidgetExt;
use gtk::prelude::LabelExt;
use gtk::*;
use gtk::Button;
use std::collections::HashMap;
use gtk::prelude::ContainerExt;
use std::process::Command;
use regex::Regex;


pub struct Workspaces {
    workspaces: Vec<ActiveWorkspace>,
    container: Box,
}

struct ActiveWorkspace{
    id:u8,
    name:String,
    monitor:String,
    windows:String,
    lactive:String,
    button:Button,
}

struct ParseWorkspace<'a> {
    id:&'a u8,
    name:&'a str,
    monitor:&'a str,
    windows:&'a u8,
    lactive:&'a str,
}


fn get_workspaces(_lifetime:&u8) -> Result<Vec<ParseWorkspace<'_>>> {
    let output = Command::new("hyprctl workspaces")
    .output()
    .context("Failed to run `workspaces`")?;

    let re: Regex = Regex::new(
        // Note: we ignore + but capture -
        r"(?P<id>[1-9]+) \((?P<name>[\S])\) on monitor (?P<monitor>\S+):\n\t[a-z]+: (?P<windows>[1-9]+)(\n\t\S+\s\S+){2}\n\t\S+\s(?P<lactive>(\w+| |@|:|,|~|\/|&|-|\.)+)",
    )
    .map_err(|_| anyhow!("Failed to compile regex for parsing sensors output"))?;

    let mut map = Vec::new();
    for mat in re.captures_iter(String::from_utf8(output.stdout)?.as_str()) {
        let id = mat.name("id").unwrap().as_str().parse::<u8>().unwrap() as u8;
	let windows = mat.name("windows").unwrap().as_str().parse::<u8>().unwrap() as u8;
        // These .unwraps() are harmless. If we have a match, we have these groups.
        map.push(
            ParseWorkspace {
                id: &id,
                name: mat.name("name").unwrap().as_str(),
		monitor: mat.name("monitor").unwrap().as_str(),
		windows: &windows,
		lactive: mat.name("lactive").unwrap().as_str(),
            }
        );
    }

    Ok(map)
}


impl Workspaces {
    pub fn new(con:&Box) -> Self{
	let vec= Vec::new();
        let workspaces = get_workspaces(&1).unwrap();
	for i in workspaces.iter() {
	    i
	}
	Workspaces { workspaces: (), container: () }
    }
}

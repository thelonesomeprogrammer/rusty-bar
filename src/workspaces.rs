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
    workspaces: Vec<Button>
}
#[derive(Debug, PartialEq)]
struct ActiveWorkspace<'a> {
    ID:&'a u8,
    name:&'a str,
}


fn get_workspaces(_lifetime:&str) -> Result<HashMap<&str, ActiveWorkspace<'_>>> {



    let re: Regex = Regex::new(
        // Note: we ignore + but capture -
        r"\n(?P<name>[\w ]+):\s+\+?(?P<temp>-?\d+\.\d+).(?P<units>[C|F])",
    )
    .map_err(|_| anyhow!("Failed to compile regex for parsing sensors output"))?;

    let mut map = HashMap::new();
    for mat in re.captures_iter(output) {
        let id = mat.name("temp").unwrap().as_str().parse::<u8>().unwrap() as u8;
        // These .unwraps() are harmless. If we have a match, we have these groups.
        map.insert(
            mat.name("name").unwrap().as_str(),
            ActiveWorkspace {
                ID: &id,
                name: mat.name("units").unwrap().as_str(),
            },
        );
    }

    Ok(map)
}


impl Workspaces {
    pub fn new(con:&Box) -> Self{
        
    }
}
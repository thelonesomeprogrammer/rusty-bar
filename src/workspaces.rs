use anyhow::{anyhow, Context, Result};
use gtk::prelude::ContainerExt;
use gtk::prelude::LabelExt;
use gtk::prelude::WidgetExt;
use gtk::traits::ButtonExt;
use gtk::Button;
use gtk::*;
use regex::Regex;
use std::process::Command;

pub struct Workspaces {
    workspaces: Box,
}

struct ActiveWorkspace {
    pub id: u8,
    name: String,
    monitor: String,
    windows: u8,
    lactive: String,
}

fn get_workspaces() -> Vec<ActiveWorkspace> {
    let output = Command::new("hyprctl")
        .arg("workworkspaces")
        .output()
        .context("Failed to run `workspaces`")
        .expect("work1");

    let re: Regex = Regex::new(
        // Note: we ignore + but capture -
        r"(?P<id>[1-9]+) \((?P<name>[\S])\) on monitor (?P<monitor>\S+):\n\t[a-z]+: (?P<windows>[1-9]+)(\n\t\S+\s\S+){2}\n\t\S+\s(?P<lactive>(\w+| |@|:|,|~|/|&|-|\.)+)",
    )
    .map_err(|_| anyhow!("Failed to compile regex for parsing sensors output")).expect("work2");

    let mut map = Vec::new();
    for mat in re.captures_iter(String::from_utf8(output.stdout).expect("work3").as_str()) {
        let id = mat.name("id").unwrap().as_str().parse::<u8>().unwrap() as u8;
        let windows = mat.name("windows").unwrap().as_str().parse::<u8>().unwrap() as u8;
        // These .unwraps() are harmless. If we have a match, we have these groups.
        map.push(ActiveWorkspace {
            id: id,
            name: mat.name("name").unwrap().as_str().to_string(),
            monitor: mat.name("monitor").unwrap().as_str().to_string(),
            windows: windows,
            lactive: mat.name("lactive").unwrap().as_str().to_string(),
        });
    }
    map
}

impl Workspaces {
    pub fn new(con: &Box) -> Self {
        let container = Box::new(Orientation::Horizontal, 0);
        let workspaces = get_workspaces();
        for i in workspaces.iter() {
            let but = Button::new();
            but.set_border_width(0);
            but.set_relief(ReliefStyle::None);
            but.set_label(&i.name);
            but.connect_clicked(|button| {
                Command::new("hyprctl")
                    .args([
                        "dispatch",
                        "workspace",
                        &format!("name:{}", button.label().unwrap()),
                    ])
                    .output()
                    .expect("work4");
            });
            container.add(&but);
        }
        con.add(&container);
        Workspaces {
            workspaces: container,
        }
    }
}

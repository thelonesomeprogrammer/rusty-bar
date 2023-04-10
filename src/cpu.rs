use anyhow::{anyhow, Result};
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use gtk::Label;
use gtk::prelude::LabelExt;
use gtk::*;
use gtk::prelude::ContainerExt;
use crate::AniStr;
/// Represents CPU widget used to show current CPU consumptiong
pub struct Cpu {
    cpu_data: CpuData,
    format: String,
    label: Label,
    animation: Vec<AniStr>,
}

impl Cpu {
    pub fn new<'a>(format:String, con:&Box, refanimation:&'a Option<Vec<AniStr>>) -> Cpu{
        let label = Label::new(None);
        con.add(&label);
        let data = CpuData::get_values();
        let cpu_data = if data.is_ok(){
            data.unwrap()
        } else {
            CpuData{
                user_time: 0, 
                nice_time: 0, 
                system_time: 0, 
                idle_time: 0, 
                total_time: 0, 
                iowait_time: 0 }
        };
        Cpu { cpu_data, format, label, animation: refanimation.as_ref().unwrap_or(&Vec::new()).to_vec() }
    }

    pub fn tick(&mut self) {
        let data = CpuData::get_values();
        let cpu_data = if data.is_ok(){
            data.unwrap()
        } else {
            CpuData{
                user_time: 0, 
                nice_time: 0, 
                system_time: 0, 
                idle_time: 0, 
                total_time: 0, 
                iowait_time: 0 }
        };

        // https://github.com/jaor/xmobar/blob/61d075d3c275366c3344d59c058d7dd0baf21ef2/src/Xmobar/Plugins/Monitors/Cpu.hs#L128
        let previous = &self.cpu_data;
        let current = cpu_data;
        let diff_total = (current.user_time - previous.user_time)
            + (current.nice_time - previous.nice_time)
            + (current.system_time - previous.system_time)
            + (current.idle_time - previous.idle_time)
            + (current.iowait_time - previous.iowait_time);
        let percentage = match diff_total {
            0 => 0.0,
            _ => (current.total_time - previous.total_time) as f64 / diff_total as f64,
        };
        self.cpu_data = current;


        let cpu_usage = (percentage * 100.0) as u8;
        let format = if self.animation.len() != 0 {
	        crate::animate(cpu_usage, self.animation.to_vec(), true) 
	    } else {
	        self.format.clone()
	    };
        let text = format.clone().replace("load", &format!("{}%",cpu_usage));
        self.label.set_markup(&text);
    }
}

struct CpuData {
    user_time: i64,
    nice_time: i64,
    system_time: i64,
    idle_time: i64,
    total_time: i64,
    iowait_time: i64,
}

impl CpuData {
    fn get_values() -> Result<CpuData> {
        // https://www.kernel.org/doc/Documentation/filesystems/proc.txt
        let file = File::open("/proc/stat")?;
        let mut cpu_line = String::new();
        let mut reader = BufReader::new(file);
        reader.read_line(&mut cpu_line)?;
        let val: Vec<&str> = cpu_line
            .split(' ')
            .filter(|item| item != &"cpu" && !item.is_empty())
            .collect();
        let mut cpu_data = CpuData {
            user_time: 0,
            nice_time: 0,
            system_time: 0,
            idle_time: 0,
            total_time: 0,
            iowait_time: 0,
        };
        match val[..] {
            [user, nice, system, idle, iowait, ..] => {
                let user_time = user.parse()?;
                let nice_time = nice.parse()?;
                let system_time = system.parse()?;
                let idle_time = idle.parse()?;
                let iowait_time = iowait.parse()?;
                cpu_data.user_time = user_time;
                cpu_data.nice_time = nice_time;
                cpu_data.system_time = system_time;
                cpu_data.idle_time = idle_time;
                cpu_data.iowait_time = iowait_time;
                cpu_data.total_time = user_time + nice_time + system_time;
            }
            _ => return Err(anyhow!("Missing data in /proc/stat")),
        }
        Ok(cpu_data)
    }
}

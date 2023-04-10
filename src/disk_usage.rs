use gtk::{Box,Label};
use gtk::prelude::{ContainerExt, LabelExt};
use std::path::Path;
use psutil::disk::disk_usage;
use crate::AniStr;

/// Disk usage widget to show current usage and remaining free space
/// in the mounted filesystem.
pub struct DiskUsage {
    label:Label,
    path:String,
    format:String,
    animation: Vec<AniStr>,
}


impl DiskUsage {
    pub fn new<'a>(path: String, format:String, con:&Box, refanimation:&'a Option<Vec<AniStr>>) -> Self {
	let label = Label::new(None);
	con.add(&label);
        Self{ label, path, format, animation: refanimation.as_ref().unwrap_or(&Vec::new()).to_vec() }
    }

    pub fn tick(&self){
	    let path = Path::new(self.path.as_str());
        let disk = disk_usage(path);
        if disk.is_ok(){
            let percentage = disk.unwrap().percent().round() as u8;
            let format = if self.animation.len() != 0 {
                crate::animate(percentage, self.animation.to_vec(), true) 
            } else {
                self.format.clone()
            };
	        let text = format.as_str().replace("load", &format!("{percentage}%",));
	        self.label.set_markup(&text)
        }
    }
}

use gtk4::{Box,Label};
use gtk4::prelude::BoxExt;
use std::path::Path;
use psutil::disk::disk_usage;
use crate::{AniStr,Replacement,replacements,animate};

/// Disk usage widget to show current usage and remaining free space
/// in the mounted filesystem.
pub struct DiskUsage {
    label:Label,
    path:String,
    format:String,
    animation: Vec<AniStr>,
    replacements: Vec<Replacement>,
}


impl DiskUsage {
    pub fn new<'a>(
        path: String, 
        format:String, 
        con:&Box, 
        refanimation:&'a Option<Vec<AniStr>>,
        refreplacement:&'a Option<Vec<Replacement>>,
    ) -> DiskUsage {
	let label = Label::new(None);
	con.append(&label);
        DiskUsage{ 
            label, 
            path, 
            format, 
            animation: refanimation.as_ref().unwrap_or(&Vec::new()).to_vec(), 
            replacements: refreplacement.as_ref().unwrap_or(&Vec::new()).to_vec(), 
        }
    }

    pub fn tick(&self){
	    let path = Path::new(self.path.as_str());
        let disk = disk_usage(path);
        if disk.is_ok(){
            let percentage = disk.unwrap().percent().round() as u8;
            let format = if self.animation.len() != 0 {
                animate(percentage, self.animation.to_vec(), true) 
            } else {
                self.format.clone()
            };
	        let text = replacements(format.as_str().replace("load", &format!("{percentage}%",)),self.replacements.to_vec());
	        self.label.set_markup(&text)
        }
    }
}

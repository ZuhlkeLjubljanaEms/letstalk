// OpenCV
extern crate libc;
use libc::c_float;
use std::io::process::Command;

struct IplImage;

mod opencv_highgui;


fn main() {

    let result = "Test".with_c_str(|name| unsafe {
    opencv_highgui::cvNamedWindow(name, 0) 
    });
    
    let image = "ZuhlkeLogo.gif".with_c_str(|name| unsafe {
    opencv_highgui::cvLoadImage(name, -1) 
    });
    
    "Test".with_c_str(|name| unsafe {
    opencv_highgui::cvShowImage(name, image) 
    });
    
    let _process = Command::new("sleep").arg("5").spawn();
    println!("Output: {}", image);
    
    

}
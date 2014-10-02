// OpenCV
extern crate libc;
use libc::c_float;
use std::io::process::Command;
use std::io;

struct IplImage;

mod opencv_highgui;


fn main() {

    let window_name = "TestGUI";

    let result = opencv_highgui::named_window(window_name, 0);
    
    let image = opencv_highgui::load_image("ZuhlkeLogo.gif", opencv_highgui::CV_LOAD_IMAGE_UNCHANGED);
    
    opencv_highgui::show_image(window_name, image);
    
    print!("Press <Enter> to exit");
    let mut reader = std::io::stdin();
    reader.read_line();
}
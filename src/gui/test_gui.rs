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
    
    //opencv_highgui::show_image(window_name, image);
    
    let camera = opencv_highgui::capture_from_cam(0);
    
    
    let mut camera_image = opencv_highgui::query_frame(camera);
    loop {
        camera_image = opencv_highgui::query_frame(camera);
        opencv_highgui::show_image(window_name, camera_image);
        
        let key = opencv_highgui::wait_key(20);
        if key > -1 {
            println!("Key {}", key);
            break;
        }
    }
}
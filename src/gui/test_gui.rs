#![feature(link_args)]

extern crate libc;
use libc::c_float;
use std::io::process::Command;

struct IplImage;

// OpenCV
#[link(name = "opencv_core")]
#[link(name = "opencv_highgui")]
#[link_args = "-L/Users/mjakopec/Development/3rdParty/opencv-2.4.9/Build/lib/Release"]


extern {
    fn cvNamedWindow(name: *const libc::c_char , flags: int ) -> int;   
    fn cvCbrt(value: c_float) -> c_float;
    
    //IplImage* cvLoadImage(const char* filename, int iscolor=CV_LOAD_IMAGE_COLOR )
    fn cvLoadImage(filename: *const libc::c_char , iscolor: int ) -> *const IplImage;
    
    //void cvShowImage(const char* name, const CvArr* image)
    fn cvShowImage(name: *const libc::c_char, image: *const IplImage);//CvArr );
}



fn main() {

    let result = "Test".with_c_str(|name| unsafe {
    cvNamedWindow(name, 0) 
    });
    
    let image = "ZuhlkeLogo.gif".with_c_str(|name| unsafe {
    cvLoadImage(name, 0) 
    });
    
    "Test".with_c_str(|name| unsafe {
    cvShowImage(name, image) 
    });
    
    //unsafe { cvNamedWindow(c_str::to_c_str("Test"), 0)};
    let x = unsafe { cvCbrt(3.0) };
    
    let _process = Command::new("sleep").arg("5").spawn();
    println!("Output: {}", x);
    
    

}
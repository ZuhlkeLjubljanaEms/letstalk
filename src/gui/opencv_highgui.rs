// OpenCV HighGUI C library

extern crate libc;
use libc::c_float;
use std::io::process::Command;

pub enum LoadImageColor {
    CV_LOAD_IMAGE_UNCHANGED  = -1, // 8bit, color or not
    CV_LOAD_IMAGE_GRAYSCALE  = 0, // 8bit, gray
    CV_LOAD_IMAGE_COLOR      = 1, // ?, color
    CV_LOAD_IMAGE_ANYDEPTH   = 2, // any depth, ?
    CV_LOAD_IMAGE_ANYCOLOR   = 4, // ?, any color
}

struct IplImage;

// The libraries used in the following extern block
#[link(name = "opencv_core")]
#[link(name = "opencv_highgui")]

extern {
    pub fn cvNamedWindow(name: *const libc::c_char , flags: int ) -> int;   
    
    //IplImage* cvLoadImage(const char* filename, int iscolor=CV_LOAD_IMAGE_COLOR )
    pub fn cvLoadImage(filename: *const libc::c_char , iscolor: int ) -> *const IplImage;
    
    //void cvShowImage(const char* name, const CvArr* image)
    pub fn cvShowImage(name: *const libc::c_char, image: *const IplImage); //CvArr );
}


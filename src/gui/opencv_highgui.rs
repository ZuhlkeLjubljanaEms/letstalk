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
struct CvCapture;

// The libraries used in the following extern block.
// These must be declared here otherwise there are linker errors!
#[link(name = "opencv_core")]
#[link(name = "opencv_highgui")]

extern {
    fn cvNamedWindow(name: *const libc::c_char , flags: int ) -> int;   
    
    //IplImage* cvLoadImage(const char* filename, int iscolor=CV_LOAD_IMAGE_COLOR )
    fn cvLoadImage(filename: *const libc::c_char , iscolor: int ) -> *const IplImage;
    
    //void cvShowImage(const char* name, const CvArr* image)
    fn cvShowImage(name: *const libc::c_char, image: *const IplImage); //CvArr );
    
    //CvCapture* cvCaptureFromCAM(int device) - in library as cvCreateCameraCapture()
    fn cvCreateCameraCapture(device: int) -> *const CvCapture;
    
    //IplImage* cvQueryFrame(CvCapture* capture)
    fn cvQueryFrame(capture:*const  CvCapture) -> *const IplImage;
}

// Adaptor functions
pub fn named_window(name: &str, flags: int) -> int {
    name.with_c_str(|cname| unsafe {
        cvNamedWindow(cname, 0) 
    })
}

pub fn load_image(filename: &str, color_type: LoadImageColor) -> *const IplImage {
    filename.with_c_str(|name| unsafe {
        cvLoadImage(name, color_type as int) 
    })
}

pub fn show_image(window_name: &str, image: *const IplImage) {
    window_name.with_c_str(|name| unsafe {
        cvShowImage(name, image) 
    });
}

pub fn capture_from_cam(device: int) -> *const CvCapture {
    unsafe { cvCreateCameraCapture(device) }
}
   
pub fn query_frame(capture: *const CvCapture) -> *const IplImage {
    unsafe { cvQueryFrame(capture) }
}
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

struct CvMat;
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
    
    //int cvWaitKey(int delay=0 )
    fn cvWaitKey(delay: i32) -> i32;
    
    //CvMat* cvEncodeImage(const char* ext, const CvArr* image, const int* params=0 )
    fn cvEncodeImage(ext: *const libc::c_char, image: *const IplImage, params: *const int ) -> *const CvMat;

    //IplImage* cvDecodeImage(const CvMat* buf, int iscolor=CV_LOAD_IMAGE_COLOR)
    fn cvDecodeImage(buf: *const CvMat, iscolor: int) -> *const IplImage;
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

pub fn wait_key(delay: i32) -> i32 {
    unsafe { cvWaitKey(delay) }
}

pub fn encode_image(ext: &str, image: *const IplImage, params: *const int ) -> *const CvMat {
    ext.with_c_str(|cext| unsafe { cvEncodeImage(cext, image, params ) })
}

pub fn decode_image(buf: *const CvMat, color_type: LoadImageColor) -> *const IplImage {
    unsafe { cvDecodeImage(buf, color_type as int) }
}
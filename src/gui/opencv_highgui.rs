// OpenCV C library adaptor
#![crate_type = "lib"]
#![crate_name = "opencv"]

extern crate libc;
use libc::c_float;
use std::io::process::Command;
use std::ptr;
use std::vec::raw;

pub enum LoadImageColor {
    CV_LOAD_IMAGE_UNCHANGED  = -1, // 8bit, color or not
    CV_LOAD_IMAGE_GRAYSCALE  = 0, // 8bit, gray
    CV_LOAD_IMAGE_COLOR      = 1, // ?, color
    CV_LOAD_IMAGE_ANYDEPTH   = 2, // any depth, ?
    CV_LOAD_IMAGE_ANYCOLOR   = 4, // ?, any color
}

pub struct CvMat;
pub struct IplImage;
pub struct CvCapture;
pub struct CvSize {
    pub width: i32,
    pub height: i32
}

// The libraries used in the following extern block.
// These must be declared here otherwise there are linker errors!
#[link(name = "opencv_core")]
#[link(name = "opencv_highgui")]

extern {
    fn cvNamedWindow(name: *const libc::c_char , flags: int ) -> int;   
    
    //IplImage* cvLoadImage(const char* filename, int iscolor=CV_LOAD_IMAGE_COLOR )
    fn cvLoadImage(filename: *const libc::c_char , iscolor: int ) -> *mut IplImage;
    
    //void cvShowImage(const char* name, const CvArr* image)
    fn cvShowImage(name: *const libc::c_char, image: *const IplImage); //CvArr );
    
    //CvCapture* cvCaptureFromCAM(int device) - in library as cvCreateCameraCapture()
    fn cvCreateCameraCapture(device: int) -> *const CvCapture;
    
    //IplImage* cvQueryFrame(CvCapture* capture)
    fn cvQueryFrame(capture:*const  CvCapture) -> *mut IplImage;
    
    //int cvWaitKey(int delay=0 )
    fn cvWaitKey(delay: i32) -> i32;
    
    //CvMat* cvEncodeImage(const char* ext, const CvArr* image, const int* params=0 )
    fn cvEncodeImage(ext: *const libc::c_char, image: *const IplImage, params: *const int ) -> *const CvMat;

    //IplImage* cvDecodeImage(const CvMat* buf, int iscolor=CV_LOAD_IMAGE_COLOR)
    fn cvDecodeImage(buf: *const CvMat, iscolor: int) -> *mut IplImage;
    
    //void cvGetRawData(const CvArr* arr, uchar** data, int* step=NULL, CvSize* roi_size=NULL )
    fn cvGetRawData(arr: *const IplImage, data: *mut*const u8, step: *mut int, roi_size: *mut CvSize );

    //CvMat* cvCreateMatHeader(int rows, int cols, int type)
    fn cvCreateMatHeader(rows: int, cols: int, datatype: int) -> *mut CvMat; 
    
    //void cvSetData(CvArr* arr, void* data, int step)
    fn cvSetData(arr: *mut CvMat, data: *mut u8, step: int);
}

// Adaptor functions
pub fn named_window(name: &str, flags: int) -> int {
    name.with_c_str(|cname| unsafe {
        cvNamedWindow(cname, 0) 
    })
}

pub fn load_image(filename: &str, color_type: LoadImageColor) -> *mut IplImage {
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
   
pub fn query_frame(capture: *const CvCapture) -> *mut IplImage {
    unsafe { cvQueryFrame(capture) }
}

pub fn wait_key(delay: i32) -> i32 {
    unsafe { cvWaitKey(delay) }
}

pub fn encode_image(ext: &str, image: *const IplImage, params: *const int ) -> *const CvMat {
    ext.with_c_str(|cext| unsafe { cvEncodeImage(cext, image, params ) })
}

pub fn decode_image(buf: *const CvMat, color_type: LoadImageColor) -> *mut IplImage {
    unsafe { cvDecodeImage(buf, color_type as int) }
}


pub struct Image {
    image: *mut IplImage
}

impl Image {
    pub fn new(imagein: *mut IplImage) -> Image {
        Image {image: imagein}
    }
    pub fn set_image(&mut self, image: *mut IplImage) {
        self.image = image;
    }
    pub fn get_image(&mut self) -> *const IplImage {
        self.image as *const IplImage
    }
    
    pub fn encoded_image(&self) -> Vec<u8> {
        let p: *const int = ptr::null();
        let mut data: *const u8 = std::ptr::null();
        let mut step = 0;
        let mut roi_size = CvSize{width: 0, height: 0};
        
        let encoded_image = encode_image(".jpeg", self.image as *const IplImage, &0 );
                
        unsafe {
            cvGetRawData(encoded_image as *const IplImage, &mut data, &mut step, &mut roi_size);
        }
        
        println!("step: {}, width: {}, height {}", step, roi_size.width, roi_size.height);
        
        let size: uint = roi_size.width as uint * roi_size.height as uint;
        unsafe { std::vec::raw::from_buf(data, size) }
    }
    
    pub fn decode_image(&mut self, encoded: &mut Vec<u8> ) {
    
        println!("--->decode vec length: {}", encoded.len());
        unsafe { 
            let mut cvmat = cvCreateMatHeader(1, encoded.len() as int, 0);
    
            cvSetData(cvmat, encoded.as_mut_ptr(), encoded.len() as int); 
        
    //void cvSetData(CvArr* arr, void* data, int step)
        
        self.image = decode_image(cvmat as *const CvMat, CV_LOAD_IMAGE_UNCHANGED);
        }
    }
}

pub struct Camera {
    camera: *const CvCapture
}

impl Camera {
    pub fn new() -> Camera {
        Camera {camera: capture_from_cam(0) }
    }
    
    pub fn grab_image(&self) -> Image {
        Image::new( query_frame(self.camera))
    }
}
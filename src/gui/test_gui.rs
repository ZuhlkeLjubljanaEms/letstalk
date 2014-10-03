// OpenCV
extern crate libc;
extern crate opencv;

//mod image;

fn main() {

    let window_name = "TestGUI";

    let result = opencv::named_window(window_name, 0);
    
    
    let mut image = opencv::Image::new(opencv::load_image("ZuhlkeLogo.gif", opencv::CV_LOAD_IMAGE_UNCHANGED));
    let loaded_image = image.get_image();
    
    image.encoded_image();
    
    
    let camera = opencv::Camera::new();
    
    loop {
        let mut camera_image = camera.grab_image();
        
        let mut encoded_image = camera_image.encoded_image();
        
        image.decode_image(&mut encoded_image);
        
        opencv::show_image(window_name, image.get_image());
        
        let key = opencv::wait_key(20);
        if key > -1 {
            println!("Key {}", key);
            break;
        }
    }
}
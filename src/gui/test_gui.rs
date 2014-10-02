// OpenCV
extern crate libc;
extern crate opencv;


fn main() {

    let window_name = "TestGUI";

    let result = opencv::named_window(window_name, 0);
    
    let image = opencv::load_image("ZuhlkeLogo.gif", opencv::CV_LOAD_IMAGE_UNCHANGED);
        
    let camera = opencv::capture_from_cam(0);
    
    
    let mut camera_image = opencv::query_frame(camera);
    loop {
        camera_image = opencv::query_frame(camera);
        
        let encoded_image = opencv::encode_image(".jpeg", camera_image, &0 );
        
        let decoded_image = opencv::decode_image(encoded_image, opencv::CV_LOAD_IMAGE_UNCHANGED);
        
        opencv::show_image(window_name, decoded_image);
        
        let key = opencv::wait_key(20);
        if key > -1 {
            println!("Key {}", key);
            break;
        }
    }
}
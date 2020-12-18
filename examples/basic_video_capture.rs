extern crate rcv_dev;

use opencv::{
    highgui,
    prelude::*,
};

use rcv_dev::image_source::*;
use rcv_dev::image_source::ImageSource;

fn main() -> opencv::Result<()> {

    let mut v_source = match VideoSource::new(0) {
        Ok(source) => {
            source
        },
        Err(e) => {
            panic!("Error: {}", e);
        }
    };

    let window = "video capture";
    highgui::named_window(window, 1)?;

    loop {
        let mut frame = v_source.get_image().unwrap();

        if frame.size()?.width > 0 {
            highgui::imshow(window, &mut frame)?;
        }
        
        let key = highgui::wait_key(10)?;
        if key > 0 && key != 255 {
            break;
        }
    }
    Ok(())
}
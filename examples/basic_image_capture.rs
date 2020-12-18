extern crate rcv_dev;
use std::{
    env
};

use opencv::{
    highgui,
    prelude::*,
};

use rcv_dev::image_source::*;
use rcv_dev::image_source::ImageSource;

fn main() -> opencv::Result<()> {

    let filename = env::args().skip(1).next().unwrap();

    let mut f_source = match FileSource::new(filename) {
        Ok(source) => {
            source
        },
        Err(e) => {
            panic!("Error: {}", e);
        }
    };

    let window = "image capture";
    highgui::named_window(window, 1)?;

    loop {
        let mut frame = f_source.get_image().unwrap();

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
extern crate clap;
use clap::{Arg, App};

extern crate opencv;
extern crate imsource;

use opencv::{
    highgui,
    prelude::*,
};

use imsource::image_source::*;
use imsource::image_source::ImageSource;

mod contour;
use contour::Contour;

fn main() -> opencv::Result<()> {

    let matches = App::new("Basic Contour")
                          .version("1.0")
                          .author("Josh Bosley <bosley117@gmail.com>")
                          .about("Basic Active Contour")
                          .arg(Arg::with_name("INPUT")
                               .short("i")
                               .help("Sets the input file to use")
                               .required(true)
                               .index(1))
                          .get_matches();

    let file_name = matches.value_of("INPUT").unwrap();
    println!("Using input file: {}", file_name);

    let mut f_source = match FileSource::new(file_name.to_string()) {
        Ok(source) => {
            source
        },
        Err(e) => {
            panic!("Error: {}", e);
        }
    };

    let window = "basic contour";

    highgui::named_window(window, 1)?;

    let mut contour = Contour::new(window);

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


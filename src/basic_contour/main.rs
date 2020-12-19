extern crate clap;
use clap::{Arg, App};

use std::sync::{ Arc, Mutex };

extern crate opencv;
extern crate imsource;

use opencv::{
    core,
    highgui,
    imgproc,
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
    let sobel_window = "sobel";

    highgui::named_window(window, 1)?;
    highgui::named_window(sobel_window, 1)?;

    let contour = Arc::new(Mutex::new(Contour::new()));

    let (mut alpha, mut beta, mut gamma) = contour.lock().unwrap().get_params();

    let mut sobel_delta = 0;

    let _ = highgui::create_trackbar("alpha", window, &mut alpha, 100, None);
    let _ = highgui::create_trackbar("beta",  window, &mut beta,  100, None);
    let _ = highgui::create_trackbar("gamma", window, &mut gamma, 100, None);

    let _ = highgui::create_trackbar("s delta", window, &mut sobel_delta, 100, None);

    highgui::set_mouse_callback(window, Some(Box::new({

        let arc_contour = Arc::clone(&contour);

        move |event, x, y, _flags| {
            {
                let mut arc_con = match arc_contour.lock() {
                    Ok(g)  => g,
                    Err(_) => return
                };
                
                if event == highgui::EVENT_LBUTTONDOWN {

                    println!("Clicked ({}, {})", x, y);
                    arc_con.add_point(x, y);
                }
            }
        }
    })))?;

    'execution : loop {
        let mut frame = f_source.get_image().unwrap();
        let mut gray  = core::Mat::default()?;

        imgproc::cvt_color(
            &frame,
            &mut gray,
            imgproc::COLOR_BGR2GRAY,
            0
        )?;

        let mut sobel_image = core::Mat::default().unwrap();
        gray.copy_to(&mut sobel_image).unwrap();

        imgproc::sobel(&gray, &mut sobel_image, 0, 1, 1, 3, 1.0, sobel_delta as f64, 0).unwrap();

        highgui::imshow(sobel_window, &sobel_image)?;

        match contour.lock() {
            Ok(mut contour_inner)  => {
                   contour_inner.update_alpha(alpha);
                   contour_inner.update_beta(beta);
                   contour_inner.update_gamma(gamma);
                   contour_inner.step(&mut sobel_image);
            },
            Err(_) => {}
        };

        if frame.size()?.width > 0 {
            highgui::imshow(window, &mut frame)?;
        }
        
        let key = highgui::wait_key(10)?;

        match key {

            // Escape key
            27 => { 
                println!("Key: 'Escape'");
                break 'execution;
            }

            // Space key
            32 => { 
                println!("Key: 'Space'");
                match contour.lock() {
                    Ok(mut contour_inner)  => {
                        contour_inner.start();
                    },
                    Err(_) => {}
                };
            }

            // 'r' key
            114 => {
                println!("Key: 'r'");
                match contour.lock() {
                    Ok(mut contour_inner)  => {
                        contour_inner.reset();
                    },
                    Err(_) => {}
                };
            }

            _ => {}
        }
    }
    Ok(())
}


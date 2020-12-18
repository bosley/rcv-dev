
use opencv::{
    core,
    prelude::*,
    videoio,
    imgcodecs,
};
use std::error::Error;
use derive_more::{ Display };

pub trait ImageSource {

    fn get_image(&mut self) -> Result<Mat, Box<dyn Error>>;
}

#[derive(Display, Debug, Clone)]
pub enum SourceError {
    BadSource,
    UnableToOpenSource
}

pub struct VideoSource {

    camera: videoio::VideoCapture
}

impl VideoSource {

    pub fn new(source: i32) -> Result<Self, SourceError> {

        let cam = match videoio::VideoCapture::new(source, videoio::CAP_ANY) {
            Ok(cam) => {
                cam
            },
            Err(_) => {
                return Err(SourceError::BadSource);
            }
        };

        let opened = match videoio::VideoCapture::is_opened(&cam) {
            Ok(v)  => v,
            Err(_) => return Err(SourceError::BadSource)
        };

        if !opened {
            return Err(SourceError::UnableToOpenSource);
        }

        Ok(VideoSource {
            camera: cam
        })
    }
}

impl ImageSource for VideoSource {

    fn get_image(&mut self) -> Result<Mat, Box<dyn Error>> {

        let mut frame = core::Mat::default()?;
        self.camera.read(&mut frame)?;

        Ok(frame)
    }
}

pub struct FileSource {

    file_source: String,
    original_image: Mat
}

impl FileSource {

    pub fn new(source: String) -> Result<Self, SourceError> {

        let filename = match core::find_file(&source, true, false) {
            Ok(file) => file,
            Err(_)   => return Err(SourceError::BadSource)
        };
        
        
        let original_image = match imgcodecs::imread(&filename, imgcodecs::IMREAD_COLOR) {
            Ok(image) => image,
            Err(_)    => return Err(SourceError::UnableToOpenSource)
        };

        Ok(Self{
            file_source:    filename.clone(),
            original_image: original_image.clone()
        })
    }

    pub fn get_source(self) -> String {
        return self.file_source.clone();
    }
}

impl ImageSource for FileSource {

    fn get_image(&mut self) -> Result<Mat, Box<dyn Error>> {
        Ok(self.original_image.clone())
    }
}
//Steganography data structure and methods.

extern crate image;

use log::{error, info, warn};
use std::path::PathBuf;
use image::GenericImageView;

use crate::settings::Settings;

// Struct of parameters for embedd file and
// for file to be embedded.
pub struct Steganography {
    pub settings: Settings,
    pub img_to_proc: bool,
    pub img_proc_running: bool,
    pub image_file: String,
    pub pic_coded: bool,
    pub pic_password: bool,
    pub pic_code_name_len: u8,
    pub pic_width: u32,
    pub pic_height: u32,
    pub pic_col_planes: u8,
    pub row: u16,
    pub col: u16,
    pub plane: u8,
    pub bit: u8,
    pub bytes_read: u32,
    pub bytes_written: u32,
    pub embedded_file_path: String,
    pub embedded_file_name: String,
    pub embedded_file_size: u32,
    pub to_embed_file_path: String,
    pub to_embed_file_size: u32,
    pub embed_capacity: u32,
}

// Initialise all struct variables.
// This method called at the start.
impl Steganography {
    pub fn init(settings: Settings) -> Self {
        info!("Initialising Steganography struct.");
        Steganography {
            settings,
            img_to_proc: false,
            img_proc_running: false,
            image_file: String::from(""),
            pic_coded: false,
            pic_password: false,
            pic_code_name_len: 0,
            pic_width: 0,
            pic_height: 0,
            pic_col_planes: 0,
            row: 0,
            col: 0,
            plane: 0,
            bit: 0,
            bytes_read: 0,
            bytes_written: 0,
            embedded_file_path: String::from(""),
            embedded_file_name: String::from(""),
            embedded_file_size: 0,
            to_embed_file_path: String::from(""),
            to_embed_file_size: 0,
            embed_capacity: 0,
        }
    }
}

// Initialise struct for image loaded properties.
impl Steganography {
    pub fn init_image_params(&mut self) {
        info!("Initialising load image file parameters.");
        self.image_file = String::from("");
        self.img_to_proc = false;
        self.pic_coded = false;
        self.pic_password = false;
        self.pic_code_name_len = 0;
        self.pic_width = 0;
        self.pic_height = 0;
        self.pic_col_planes = 0;
    }
}

// Initialise struct for reading and writing
// embedded files.
impl Steganography {
    pub fn init_embed_params(&mut self) {
        info!("Initialising embedded file parameters.");
        self.row = 0;
        self.col = 0;
        self.plane = 0;
        self.bit = 0;
        self.bytes_read = 0;
        self.bytes_written = 0;
        self.embedded_file_path = String::from("");
        self.embedded_file_name = String::from("");
        self.embedded_file_size = 0;
        self.to_embed_file_path = String::from("");
        self.to_embed_file_size = 0;
        self.embed_capacity = 0;
    }
}

// Method to load a brand new image for analysis.
impl Steganography {
    pub fn load_new_file(&mut self, in_file:String) {
        // Do image intialisatioins to clean up after any
        // successful or failed image loading.
        // That is, parameters for loaded and imbedded image.
        self.init_image_params();
        self.init_embed_params();

        // Several checks along the way so status
        // to keep progress along the way.
        let mut cont_ckh: bool = true;

        // Create path to image.
        let mut img_path = PathBuf::new();
        img_path.push("images");        
        img_path.push(in_file.clone());
        let img_path_string = img_path.to_string_lossy().into_owned();
        self.image_file = img_path_string;

        let img_result = image::open(&img_path);
        // Handle exceptions, specific like file not found, and generic.
        let img = match img_result {
            Ok(img) => {
                // Set flag to indicate we have an image to process.
                self.img_to_proc = true;
                img
            }
            Err(err) => {
                // Set flag indicating that there was an issue opening the file.
                // So we don't have to continue after this.
                cont_ckh = false;
                match err {
                    // File not found error.
                    image::ImageError::IoError(io_err) if io_err.kind() == std::io::ErrorKind::NotFound => {
                        warn!("Warning file not found: {}", in_file.clone());
                        // Return a placeholder image.
                        image::DynamicImage::new_rgb8(1, 1)
                    }
                    // Generic exception.
                    _ => {
                        error!("Error openning image file: {}", in_file.clone());
                        // Return a placeholder image.
                        image::DynamicImage::new_rgb8(1, 1)
                    }
                }
            }
        };

        // If we have an image file open, then read the parameters.
        // Need to check if 3 colour  planes as well.
        if cont_ckh == true {
            // Get image width and height
            (self.pic_width, self.pic_height) = img.dimensions();
            info!("Image loaded with width: {}, height: {}", self.pic_width, self.pic_height);

            // Need to check if color format is acceptable.
            // Need 3 color planes.
            let cols = img.color();
            match cols {
                image::ColorType::Rgb8 | image::ColorType::Rgba8 => {
                    // Store number of color planes
                    self.pic_col_planes = 3;
                    info!("Image loaded with colour planes: {}", self.pic_col_planes);
                }
                _ => {
                    // Unsupported image color type
                    info!("Image not a supported rgb colour type.");
                }
            }
        }

        // Calcaulate the available space for storage.
        // Basically how many bits get used when embeddng files
        // in an image.
        // Here capacity is in bytes.
        if cont_ckh == true {
            let img_bytes: u32 = self.pic_width * self.pic_height * self.pic_col_planes as u32;
            let _embed_bytes: f32 = img_bytes as f32 * self.settings.max_embed_ratio;
            self.embed_capacity = _embed_bytes as u32;

            info!("Approx embedding capacity (bytes): {}", self.embed_capacity);
        }

        // Check if the file is already image coded.
        self.check_for_code();
        if self.pic_coded == true {
            info!("Image file contains preamble code.");
        }
    }
}

// Method to check if image has been previousl encoded,
// that is, it contains the preable code.
impl Steganography {
    pub fn check_for_code(&mut self) {
        // First check if file is even large enough to hold a code.
        // Can do this by checking emdedding capacity.
        if self.embed_capacity < self.settings.min_capacity {
            warn!("Capacity less than min for coding (bytes): {}", self.embed_capacity);
            self.pic_coded = false;
            return
        }

        // File largge enough to hold preamble code.
        // Extract data from image and match with code.
        //<ToDo>
    
        self.pic_coded = true;
    }
}

// Steganography data structure and methods.
//
// Steganography in this application is embedding files in lossless images,
// specifically in PNG format images.
// Supported formats are rgb and rgb colour formats,although only
// the rgb colour bytes are used to encode data into.
//
// A pic coded image contains a particular 8 byte string embedded in the image.
// Here 'contains' implies embedded in the image colour bytes.
// The format of pic coded files is as follows:
//
// Pic coded signature : 8 bytes
// Password enabled : 1 byte, 'Y' or 'N'
// If password enabled : 30 byte hash of password.
// Number of files embedded : 2 digit integer, leading zeros.
// For each file section the following applies:
//
// File name length: 2 digit integer, leading zeros.
// File name : file name string in file name length bytes.
// File length in bytes : 8 digit integer, leading zeros.
// File contents : file bytes in file length bytes.

extern crate image;

use log::{error, info, warn};
use std::path::PathBuf;
use image::{DynamicImage, GenericImageView, Pixel};

use crate::settings::Settings;

// Struct of parameters for embedd file and
// for file to be embedded.
pub struct Steganography {
    pub settings: Settings,
    pub img_to_proc: bool,
    pub img_proc_running: bool,
    pub image_file: String,
    pub image: Option<DynamicImage>,
    pub pic_coded: bool,
    pub pic_password: bool,
    pub pic_code_name_len: u8,
    pub pic_width: u32,
    pub pic_height: u32,
    pub pic_col_planes: u8,
    pub row: u32,
    pub col: u32,
    pub plane: usize,
    pub bit: u8,
    pub bytes_read: u32,
    pub bytes_written: u32,
    pub code_bytes: Vec<u8>,
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
            image: None,
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
            code_bytes: Vec::with_capacity(0),
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
        self.image = None;
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
        let _img = match img_result {
            Ok(_img) => {
                // Set flag to indicate we have an image to process.
                self.img_to_proc = true;
                self.image = Some(_img.clone());
                _img
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
            if let Some(image) = &self.image {
                // Get image width and height
                (self.pic_width, self.pic_height) = image.dimensions();
                info!("Image loaded with width: {}, height: {}", self.pic_width, self.pic_height);

                // Need to check if color format is acceptable.
                // Need 3 color planes.
                let cols = image.color();
                match cols {
                    // Even though only writing to rgb planes for now,
                    // Need to keep track if there is a transparency layer.
                    image::ColorType::Rgb8 => {
                        // Store number of color planes
                        self.pic_col_planes = 3;
                        info!("Image loaded with colour planes: {}", self.pic_col_planes);
                    }
                    image::ColorType::Rgba8 => {
                        // Store number of color planes
                        self.pic_col_planes = 4;
                        info!("Image loaded with colour planes: {}", self.pic_col_planes);
                    }
                    _ => {
                        // Unsupported image color type
                        info!("Image not a supported rgb colour type.");
                    }
                }
            }
            else {
                error!("Image is of None type");
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

        // File large enough to hold preamble code.
        // Extract data from image and match with code.
        // Read number of bytes for the pic code.
        let bytes_to_read:u32 = self.settings.prog_code.len().try_into().unwrap();
        self.read_data_from_image(bytes_to_read);
        if self.bytes_read != bytes_to_read {
            error!("Expected bytes: {}, bytes read: {}", bytes_to_read, self.bytes_read);
            info!("Image file is not pic coded.");  
            self.pic_coded = false;
            return;
        }
        else {
            // Compare the byte array read with the pic coded array (string).
            let string_result = String::from_utf8((&*self.code_bytes).to_vec());
            match string_result {
                Ok(string) => {
                    // String read so need to see if it matches the code.
                    if string == self.settings.prog_code {
                        info!("Image is pic coded.");

                        // Image file is pic coded,
                        // So we can extract the data from it.
                        // <TODO>
                        // Need to check if there is a password.
                        // Need to prompt for a password if applicable
                        // Need to check how many files are embedded,
                        // Need to extract and save embedded files.
                    }
                    else {
                        info!("Image is not pic coded.");
                        self.pic_coded = false;
                        return;
                    }
                }
                Err(e) => {
                    warn!("Warning, error converting to string: {}", e);
                    self.pic_coded = false;
                    return;
                }
            }
        }
    }
}

// Method to read a certain number of bytes from image..
impl Steganography {
    pub fn read_data_from_image(&mut self, bytes_to_read:u32) {
        info!("Reading bytes from image: {}", bytes_to_read);

        // Initial loop counters.
        let mut bytes_read:u32 = 0;
        let mut row_cnt:u32 = self.row;
        let mut col_cnt:u32 = self.col;
        let mut col_plane:usize = self.plane;
        let mut _bits_read:u8 = self.bit;
        let mut _col_part:u8 = 0;
        let mut _code_data:u8 = 0;
        let mut _byte_bit:u8 = 0;
        let mut _mask:u8 = 0;

        // Initialise byte vector for read data.
        self.code_bytes = Vec::with_capacity(bytes_to_read as usize);

        // Initialise a colour bit mask.
        // This is so we can read an individual
        // bit in a pixel colour byte.
        _mask = 1 << _bits_read;

        // Loop while there are still bytes to read.
        while bytes_read < bytes_to_read {
            _code_data = 0;

            // Extract 1 byte of data from image.,
            // one bit at a time.
            for _idx in 1..9 {
                // Get the pixel colour for the pixel we are at.
                if let Some(image) = &self.image {
                    if self.pic_col_planes == 3 { 
                        _col_part = image.get_pixel(col_cnt, row_cnt).to_rgb()[col_plane];
                    } else {
                        _col_part = image.get_pixel(col_cnt, row_cnt).to_rgba()[col_plane];
                    }
                }

                // Update the code data bit with the bit from the pixel.
                _byte_bit = _col_part & _mask;
                _byte_bit = _byte_bit >> _bits_read;
                _code_data = _code_data << 1;
                _code_data = _code_data | _byte_bit;

                // Next time around we need to point to the next pixel in the row.
                col_cnt = col_cnt + 1;
                // Until we get to the end of the row.
                // Then more to the start of the next row.
                if col_cnt == self.pic_width {
                    col_cnt = 0;
                    row_cnt = row_cnt + 1;
                    // If we have reached the end of the image then go
                    // back to the top and go to the text bit.
                    if row_cnt == self.pic_height {
                        row_cnt = 0;
                        col_plane = col_plane + 1;
                        // If we have processed the last plane (colour)
                        // We go back to the next bit of the first plane,
                        if col_plane == 3 {
                            col_plane = 0;
                            _bits_read = _bits_read + 1;
                            _mask = _mask << 1;
                        }
                    }
                }
            }
            // Push the completed byte into the byte vector.
            self.code_bytes.push(_code_data);

            // Increment bytes read.
            bytes_read = bytes_read + 1;
        }

        // Save the state of the reading.
        self.row = row_cnt;
        self.col = col_cnt;
        self.plane = col_plane;
        self.bit = _bits_read;
        self.bytes_read = bytes_read;
    }
}

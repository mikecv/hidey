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
// If password enabled : 32 byte hash of password.
// Number of files embedded : 3 digit integer, leading zeros.
// For each file section the following applies:
//
// File name length: 3 digit integer, leading zeros.
// File name : file name string in file name length bytes.
// File length in bytes : 10 digit integer, leading zeros.
// File contents : file bytes in file length bytes.

pub mod image_read;
pub mod image_write;

extern crate image;
extern crate ring;

use log::{error, info, warn};

use std::fs::{self, File};
use std::io::{self, Read};
use std::path::Path;
use std::path::PathBuf;
use image::{DynamicImage, GenericImageView};
use ring::digest;

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
    pub user_permit: bool,
    pub pic_has_pw: bool,
    pub pic_code_name_len: u8,
    pub pic_width: u32,
    pub pic_height: u32,
    pub pic_col_planes: u8,
    pub row: u32,
    pub col: u32,
    pub plane: usize,
    pub bit: u8,
    pub bytes_read: u32,
    pub code_bytes: Vec<u8>,
    pub embedded_file_path: String,
    pub embedded_file_name: String,
    pub embedded_file_size: u32,
    pub to_embed_file_path: String,
    pub to_embed_file_size: u32,
    pub embed_capacity: u64,
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
            user_permit: false,
            pic_has_pw: false,
            pic_code_name_len: 0,
            pic_width: 0,
            pic_height: 0,
            pic_col_planes: 0,
            row: 0,
            col: 0,
            plane: 0,
            bit: 0,
            bytes_read: 0,
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
        self.user_permit = false;
        self.pic_has_pw = false;
        self.pic_code_name_len = 0;
        self.pic_width = 0;
        self.pic_height = 0;
        self.pic_col_planes = 0;
        self.embed_capacity = 0;
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
        self.embedded_file_path = String::from("");
        self.embedded_file_name = String::from("");
        self.embedded_file_size = 0;
        self.to_embed_file_path = String::from("");
        self.to_embed_file_size = 0;
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

        // Calculate the available space for storage.
        // Basically how many bits get used when embeddng files
        // in an image.
        // Here capacity is in bytes.
        if cont_ckh == true {
            let img_bytes: u32 = self.pic_width * self.pic_height * self.pic_col_planes as u32;
            let _embed_bytes: f32 = img_bytes as f32 * self.settings.max_embed_ratio;
            self.embed_capacity = _embed_bytes as u64;

            info!("Approx embedding capacity (bytes): {}", self.embed_capacity);
        }

        // Check if the file is already pic coded.
        self.check_for_code();
        if self.pic_coded == true {
            info!("Image file contains preamble code.");

            // Now that we know that the image is pic coded,
            // we can see if there is a password encoded in the image.
            // Password yes or no is in the next 1 byte.
            self.check_for_password();
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
                        self.pic_coded = true;
                        info!("Image is pic coded.");
                    }
                    else {
                        self.pic_coded = false;
                        info!("Image is not pic coded.");
                    }
                }
                _ => {
                    self.pic_coded = false;
                    info!("Image is not pic coded.");
                }
            }
        }
    }
}

// Method to check if image has a password.
impl Steganography {
    pub fn check_for_password(&mut self) {

        // Read number of bytes for whether or not there is a password.
        let bytes_to_read:u32 = 1;
        self.read_data_from_image(bytes_to_read);
        if self.bytes_read != bytes_to_read {
            error!("Expected bytes: {}, bytes read: {}", bytes_to_read, self.bytes_read);
            info!("Image does not include a password.");  
            self.user_permit = false;
            return;
        }
        else {
            // Check for Y(es) or N(o) re password.
            let string_result = String::from_utf8((&*self.code_bytes).to_vec());
            match string_result {
                Ok(string) => {
                    // String read so need to see if it Y or N.
                    if string == "Y" {
                        self.pic_has_pw = true;
                        info!("Image includes a password.");
                    }
                }
                _ => {
                    self.pic_has_pw = false;
                    info!("Image does not include a password.");
                }
            }
        }
    }
}

// Method to check user's password entry.
impl Steganography {
    pub fn check_valid_password(&mut self, password: String) {
        // Before checking the password we have to get the
        // hashed password stored in the image.
        // The password is a SHA-256 so always 32 bytes long.
        let bytes_to_read:u32 = self.settings.pw_hash_len.into();
        self.read_data_from_image(bytes_to_read);
        if self.bytes_read != bytes_to_read {
            error!("Expected bytes: {}, bytes read: {}", bytes_to_read, self.bytes_read);
            info!("Image password invalid length.");  
            self.user_permit = false;
            return;
        }
        else {
            // Check password against hash of user entry.
            let string_result = String::from_utf8((&*self.code_bytes).to_vec());
            match string_result {
                Ok(string) => {
                    // Check password against hash of user entry.
                    let hashed_entry =  digest::digest(&digest::SHA256, password.as_bytes());
                    let hashed_password = hashed_entry.as_ref();
                    if string == std::str::from_utf8(hashed_password).unwrap() {
                        self.user_permit = true;
                        info!("User entered password matches.");
                    }
                    else {
                        self.user_permit = false;
                        info!("User entered password does not match.");
                    }
                }
                _ => {
                    self.user_permit = false;
                    info!("User entered password does not match.");
                }
            }
        }
    }
}

// Method to embed one or more files into a loaded image.
impl Steganography {
    pub fn embed_files(&mut self, pw:bool, pw_str:&str, embed_files:&[&str]) -> io::Result<()> {
        // Don't need to initialise image parameters as we require
        // a loaded image to embed files into.
        if self.img_to_proc == true {
            // We have an image to embed into so all good.
            // It doesn't matter if the image is already pic coded as we
            // will just overwrite the previous embedding.
            // We should also alaready know the embedding width, height,
            // and embedding capacity of the image.

            // First check is to see if there is space for the file(s) requested.
            let mut bytes_to_embed = 0;
            for file in embed_files {
                // Need to get sum of file lengths to embed.
                let metadata = fs::metadata(file)?;
                let file_size = metadata.len();
                bytes_to_embed = bytes_to_embed + file_size;
                info!("File: {} Size: {} bytes", file, file_size);
            }
            // Need to compare bytes to embed with image capacity.
            // Ignoring size of file names as not likely to be significant.
            if bytes_to_embed > self.embed_capacity {
                // Exceeded embedding capacity so can't imbed.
                warn!("Exceeded image emdedding: {}", self.embed_capacity)
            }
            else {
                // Within the embedding capacity of the image, so proceed.
                info!("Total data to embed: {} bytes", bytes_to_embed);

                // First step is to write the pic code preamble to the file.
                self.embed_preamble();

                // Next to to embed password if required.
                self.embed_password(pw, pw_str);

                // Next need to embed the number of files we are embedding.
                let num_files:u16 = embed_files.len() as u16;
                self.embed_num_of_files(num_files);

                // Next need to embed files themselves, one at a time.
                for file in embed_files {
                    // Need to embed the file.
                    // This also means embeddng the name of the file,
                    // and the length of the file.
                    if let Err(err) = self.embed_file(file) {
                        eprintln!("Error: {}", err); 
                    }
                    else {
                        info!("Successfully embedded file: {}", file);
                    }
                }
            }
            Ok(())
        }
        else {
            info!("No files to process.");
            Ok(())
        }
    }
}

// Method to add the preable code to the image.
impl Steganography {
    pub fn embed_preamble(&mut self) {
        info!("Embedding preamble into image.");

        // Initialise embedding parameters.
        self.init_embed_params();

        // Send preamble as bytes vector for embedding.
        // All writes to the image is done in chunks.
        let preamble_string = self.settings.prog_code.clone();
        let preamble_bytes = preamble_string.as_bytes();
        for chunk in preamble_bytes.chunks(self.settings.byte_chunk.try_into().unwrap()) {
            let bytes_written:u32 = self.write_data_to_image(chunk);
            if bytes_written != chunk.len() as u32{
                error!("Incorrect number of bytes written: {}", bytes_written)
            }
        }
    }
}

// Method to embed password (if required) to the image.
impl Steganography {
    pub fn embed_password(&mut self, _pw:bool, _pw_str:&str) {
        info!("Embedding whether passworded or not.");

        // Initialise embedding parameters.
        self.init_embed_params();

        // Send pasword as applicable as bytes vector for embedding.
        // All writes to the image is done in chunks.
        if _pw == false {
            let have_pw_str = String::from("N");
            let have_pw_bytes = have_pw_str.as_bytes();
            for chunk in have_pw_bytes.chunks(self.settings.byte_chunk.try_into().unwrap()) {
                let bytes_written:u32 = self.write_data_to_image(chunk);
                if bytes_written != chunk.len() as u32{
                    error!("Incorrect number of bytes written: {}", bytes_written)
                }
            }
        }
        else {
            // We have a password to embed.
            // First we need to get the hash of the password to embed.
            info!("Embedding passworded.");
            // First the tag that there is a password.
            let have_pw_str = String::from("Y");
            let have_pw_bytes = have_pw_str.as_bytes();
            // Next we have the hashed password.
            let digest = digest::digest(&digest::SHA256, _pw_str.as_bytes());
            let hashed_password = digest.as_ref();
            let password_bytes = hashed_password;
            // Concatenate the two.
            let pw_bytes:Vec<u8> = [have_pw_bytes, password_bytes].concat();
            // Embed into image.
            for chunk in pw_bytes.chunks(self.settings.byte_chunk.try_into().unwrap()) {
                let bytes_written:u32 = self.write_data_to_image(chunk);
                if bytes_written != chunk.len() as u32{
                    error!("Incorrect number of bytes written: {}", bytes_written)
                }
            }
        }
    }
}

// Method to embed the number of files being embedded.
impl Steganography {
    pub fn embed_num_of_files(&mut self, num_files:u16) {
        info!("Embedding number of files: {}", num_files);

        // Initialise embedding parameters.
        self.init_embed_params();

        // Get the number of files as a string with leading 0s.
        let _num_files:String = format!("{:0>3}", num_files);
        let num_file_bytes = _num_files.as_bytes();

        // Embed into image.
        for chunk in num_file_bytes.chunks(self.settings.byte_chunk.try_into().unwrap()) {
            let bytes_written:u32 = self.write_data_to_image(chunk);
            if bytes_written != chunk.len() as u32{
                error!("Incorrect number of bytes written: {}", bytes_written)
            }
        }
    }
}

// Method to embed the contents of a file into the image.
impl Steganography {
    pub fn embed_file(&mut self, file_path:&str) -> io::Result<()> {
        info!("Embedding file: {}", file_path);

        // Initialise embedding parameters.
        self.init_embed_params();

        // Need to get the filename to give the file,
        // and the length of this filename, as both are embedded.
        // File name, and filename length.
        let _file_path = Path::new(file_path);
        // Extract file name.
        let _file_name = _file_path.file_name().unwrap();
        let _file_name_bytes = _file_name.as_encoded_bytes();
        // Determine filename length.
        // And format to 3 digits, with leading 0s.
        let _file_name_len = _file_name.len() as u8;
        let _file_name_len_str:String = format!("{:0>3}", _file_name_len);
        let _file_name_len_bytes = _file_name_len_str.as_bytes();
        // Determine file length in bytes.
        // And format to 10 digits, with leading 0s.
        let _metadata = fs::metadata(file_path)?;
        let _file_size = _metadata.len();
        let _file_size_str:String = format!("{:0>10}", _file_size);
        let _file_size_bytes = _file_size_str.as_bytes();

        // Concatenate file details for embedding.
        let file_detail_bytes:Vec<u8> = [_file_name_len_bytes, _file_name_bytes, _file_size_bytes].concat();
        // Embed into image.
        for chunk in file_detail_bytes.chunks(self.settings.byte_chunk.try_into().unwrap()) {
            let bytes_written:u32 = self.write_data_to_image(chunk);
            if bytes_written != chunk.len() as u32{
                error!("Incorrect number of bytes written: {}", bytes_written)
            }
        }

        // Now the file needs to be written to the image.
        // Will do this by reading chunks of data from the file at a time,
        // and writing the data to the image, until the file is done.

        // Open the file for reading.
        let mut file = File::open(file_path)?;

        // Define a buffer to use for the chunks of read data.
        let mut buffer = vec![0u8; self.settings.byte_chunk as usize];

        // Loop until there are no bytes in the file to write.
        loop {
            // Read a chunk of data from the file.
            let bytes_read = file.read(&mut buffer)?;

            // If no bytes were read, we've reached the end of the file.
            if bytes_read == 0 {
                break;
            }

            // Write the chunk of data to the image.
            let bytes_written = self.write_data_to_image(&buffer[..bytes_read]);

            // Check that the correct number of bytes were written.
            if bytes_written != bytes_read as u32 {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    format!("Incorrect number of bytes written: {}", bytes_written),
                ));
            }
        }
        // Return ok result.
        info!("File data written successfully.");
        Ok(())
    }
}

// Method to save image with name.
// Will overwrite the existing image if no file specified.
impl Steganography {
    pub fn save_image(&mut self, mut save_file:String) {

        // Check if file path string provided.
        // If not then overwrite the loaded image file instead.
        if save_file.len() == 0 {
            save_file = self.image_file.clone();
            info!("Overwritting original image.")
        }
        // Create path to image file .
        let mut img_path = PathBuf::new();
        // img_path.push("images");        
        img_path.push(save_file.clone());
        let img_path_string = img_path.to_string_lossy().into_owned();
        info!("Writing to image: {}", img_path_string);

        // Save the image with embedded data to file.
        if let Some(image) = &self.image {
            image.save(img_path_string).expect("Failed to save image");
        } else {
            panic!("Failed to save image file.");
        }
    }
}

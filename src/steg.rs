//Steganography data structure and methods.

use log::info;

use crate::settings::Settings;

pub struct Steganography {
    pub settings: Settings,
    pub img_to_proc: bool,
    pub img_proc_running: bool,
    pub image_file: String,
    pub pic_coded: bool,
    pub pic_password: bool,
    pub pic_code_name_len: u8,
}

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
        }
    }
}

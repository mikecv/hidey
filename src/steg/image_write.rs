// Image pixel write methods.

use crate::steg::Steganography;

use log::info;

use image::{GenericImageView, Pixel};
use crate::steg::image::GenericImage;

// Method to writw a certain number of bytes to am image.
// Data is written to the image a chunk at a time.
// Only dealing with rgb image files.
// Expectation is that rgba documents will be converted to
// rgb format before embedding data.
impl Steganography {
    pub fn write_data_to_image(&mut self, bytes:&[u8]) {

        // Initial loop counters.
        let mut bytes_written:u32 = 0;
        let mut row_cnt:u32 = self.row;
        let mut col_cnt:u32 = self.col;
        let mut col_plane:usize = self.plane;
        let mut bit_write:u8 = self.bit;
        let mut col_part:image::Rgb<u8>;
        let mut _mask:u8 = 0;
        let mut _col_mask:u8 = 0;
        let mut _mapped_bit:u8 = 0;

        // Intialise colour bit mask.
        _col_mask = 1 << bit_write;

        for byte in bytes {
            info!("Processed byte: {}", byte);

            // Mask for reading byte bits.
            // Start from MSB so in bit order in the image (assume 8 bit byte).
            _mask = 128;

            // Extract 1 byte of data from image.
            // one bit at a time.
            for _idx in 1..9 {
                // Get next bit for byte in the array.
                if (byte & _mask) == 0{
                    _mapped_bit = 0;
                }
                else {
                    _mapped_bit = 1;
                }
                _mapped_bit = _mapped_bit << bit_write;

                // Get the pixel colour for the pixel we are at.
                if let Some(mut img) = self.image.take() {
                    col_part = img.get_pixel(col_cnt, row_cnt).to_rgb();

                    // Modify the colour plane component that we are up to.
                    let r = col_part[0] & (! _col_mask);
                    let g = col_part[1] & (! _col_mask);
                    let b = col_part[2] & (! _col_mask);

                    // Update the pixel colour now that the colour component has been modified.
                    let modified_pixel = image::Rgba([r, g, b, 255]);
                    img.put_pixel(col_cnt, row_cnt, modified_pixel);
                    self.image = Some(img);
                }

                // Shift mask right (towards LSB).
                _mask = _mask >> 1;
    
                // Point to next column.
                col_cnt = col_cnt + 1;
                if col_cnt == self.pic_width {
                    col_cnt = 0;
                    row_cnt = row_cnt + 1;
                    // If we have reached the end of the image then go
                    // back to the top and go to the next bit.
                    if row_cnt == self.pic_height {
                        row_cnt = 0;
                        // Point to next colour plane.
                        // Take into account number of planes.
                        col_plane = col_plane + 1;
                        if col_plane == 3 {
                            col_plane = 0;
                            // Used all colour planes so move to next bit.
                            bit_write = bit_write + 1;
                            _col_mask = _col_mask << 1;
                        }     
                    }
                }


            }
            // Increment characters write counter.
            bytes_written = bytes_written + 1;
        }

        // Save the state of the reading.
        self.row = row_cnt;
        self.col = col_cnt;
        self.plane = col_plane;
        self.bit = bit_write;
        self.bytes_written = bytes_written;
    }
}

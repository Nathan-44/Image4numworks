use std::error::Error;
use std::fs;

use image::ImageReader;

use super::NumImageFormat;
use super::pixels_to_vec::pixels_to_vec;


pub fn file_to_num_image(file_path: &str, format: NumImageFormat) -> Result<Vec<u8>, Box<dyn Error>> {
    // Load image
    let image_raw = ImageReader::open(file_path)?;
    let image_decode = image_raw.decode()?;

    let form: u16 = format as u16;
    let width: u16 = image_decode.width() as u16;
    let height: u16 = image_decode.height() as u16;

    // Image to return
    let mut image_num: Vec<u8> = vec![];

    // Add image's format + images size
    image_num.extend(form.to_le_bytes());
    image_num.extend(width.to_le_bytes());
    image_num.extend(height.to_le_bytes());

    // Add pixels
    match format {
        NumImageFormat::Simple => pixels_to_vec(image_decode, &mut image_num),
    }

    Ok(image_num)
}

/// Save image's vec into a file
pub fn save_numworks_image(data: Vec<u8>, file_name: &str) -> Result<(), Box<dyn Error>> {
    fs::write(file_name, data)?;
    Ok(())
}

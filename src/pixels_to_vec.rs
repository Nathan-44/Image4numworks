use image::{DynamicImage, GenericImageView, Rgba};

use crate::common::Flags;

/// Transform an image into a vector of u8 represents rgb format + flags
/// Just a list of pîxels, line per line
pub fn pixels_to_vec(image: DynamicImage, vec: &mut Vec<u8>) {
    // Get image's size
    let width = image.width();
    let height = image.height();

    // Iter over each pixel
    for y in 0..height {
        for x in 0..width {
            let pix = image.get_pixel(x, y);

            // If the pixel is transparent, we just put Black and we put the flag
            if pix.0[3] == 0 {
                let color: u16 = 0;
                let flag = Flags::Transparent as u8;

                vec.extend(color.to_le_bytes());
                vec.extend(flag.to_le_bytes());
                continue;
            }
            let color = rgb888_to_rgb565(pix);

            let flag = Flags::Base as u8;

            // Add pixel and flag to the vec
            vec.extend(color.to_le_bytes());
            vec.extend(flag.to_le_bytes());
        }
    }
}

/// Numworks calculator support only RGB 565
fn rgb888_to_rgb565(color: Rgba<u8>) -> u16 {
    // Extract colors and shift bits
    let red = (color.0[0] >> 3) as u16;
    let green = (color.0[1] >> 2)  as u16;
    let blue = (color.0[2] >> 3)  as u16;

    // Return rgb 565
    ((red << 11) | (green << 5)) | blue
}

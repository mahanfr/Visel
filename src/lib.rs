
#[no_mangle]
pub extern "C" fn _pixels_to_png(pixels: *const u32, width: usize, hight: usize) {
}

pub fn pixels_to_png(pixels: Vec<u32>) {
    let mut is_alpha_separation = true;
    let mut is_rgb_merging = false;
    for pix in pixels {
        let alpha: u8 = (pix >> 24) as u8 & 0xFF;
        let blue: u8 = (pix >> 16) as u8 & 0xFF;
        let green: u8 = (pix >> 8) as u8 & 0xFF;
        let red:u8 = pix as u8 & 0xFF;
        if alpha < 0xFF {
            is_alpha_separation = false;
        }
        if (red & green & blue) == red {
            is_rgb_merging = true;
        }
    }
    if is_alpha_separation {
        println!("Needs Alpha Seperation!");
    }
    if is_rgb_merging {
        println!("Needs rgb Merging");
    }
}

#[cfg(test)]
mod tests {}

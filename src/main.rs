use std::error::Error;
use std::fmt::Write;
use std::fs::{self, File};

use log::info;

const IMAGE_WIDTH: u32 = 256;
const IMAGE_HEIGHT: u32 = 256;

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    // create a 256x256 image in ppm (lossless) format which goes from black to red for the first row (0->255)
    // then for every progressive row a green is added (0->255)
    let mut buf = String::new();
    writeln!(buf, "P3")?;
    writeln!(buf, "{} {}", IMAGE_WIDTH, IMAGE_HEIGHT)?;
    writeln!(buf, "255")?;

    for y in 0..IMAGE_HEIGHT {
        for x in 0..IMAGE_WIDTH {
            info!("scanlines remaining: {}", IMAGE_HEIGHT - y);
            let r = x;
            let g = y;
            let b = 0;

            writeln!(buf, "{} {} {}", r, g, b)?;
        }
    }

    // write the buffer to a ppm file
    fs::write("image.ppm", buf)?;

    Ok(())
}

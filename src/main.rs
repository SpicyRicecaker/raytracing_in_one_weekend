use std::fmt::Write;
use std::fs;

use log::info;

const IMAGE_WIDTH: u32 = 256;
// const IMAGE_HEIGHT: u32 = 256;

use raytracing_in_one_weekend::color::*;
use raytracing_in_one_weekend::vec::*;
use raytracing_in_one_weekend::vec3;
use raytracing_in_one_weekend::Error;

struct Camera {
    aspect_ratio: f64,
    image_width: u32,
    image_height: u32,
    viewport_width: f64,
    viewport_height: f64,
    eye: Point3,
    camera_direction: Vec3,
    focal_length: f64
}

impl Camera {
    // note, by default aspect ratio is width / height
    // so to get height from width given aspect ratio...
    fn new(image_width: u32, aspect_ratio: f64, viewport_width: f64) -> Self {
        let image_height = (image_width as f64 * (1. / aspect_ratio)) as u32;
        // we recalculate the aspect ratio here since we're rounding the
        // image_height to integer values in the above line
        let viewport_height = viewport_width as f64 * (1. / (image_width as f64 / image_height as f64));

        let eye = vec3![0., 0., 0.];
        // arbitrarily define the camera direction as moving in the -z direction
        let camera_direction = vec3![0., 0., -1.];
        // we also need to define the camera normal so that we can know where the viewport plane is. 
        let focal_length = 1.;

        Self {
            aspect_ratio,
            image_width,
            image_height,
            viewport_width,
            viewport_height,
            eye,
            camera_direction,
            focal_length
        }
    }

    fn cast(&self) {
        let viewpoint_center: Point3 = self.focal_length * self.camera_direction;
        let q = k

        // calculate v_u (the left to right viewport vector)
        let v_u = 

    }
}

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    // create a 256x256 image in ppm (lossless) format which goes from black to red for the first row (0->255)
    // then for every progressive row a green is added (0->255)

    // choose an aspect ratio for the image, decide height based on width.

    // let mut buf = String::new();
    // writeln!(buf, "P3")?;
    // writeln!(buf, "{} {}", IMAGE_WIDTH, IMAGE_HEIGHT)?;
    // writeln!(buf, "255")?;

    // for y in 0..IMAGE_HEIGHT {
    //     for x in 0..IMAGE_WIDTH {
    //         info!("scanlines remaining: {}", IMAGE_HEIGHT - y);
    //         let color = vec3![
    //             (x as f64) / (IMAGE_WIDTH as f64 - 1.),
    //             (y as f64) / (IMAGE_HEIGHT as f64 - 1.),
    //             0.
    //         ];
    //         write_color(&mut buf, color)?;
    //     }
    // }

    // write the buffer to a ppm file
    // fs::write("image.ppm", buf)?;

    Ok(())
}

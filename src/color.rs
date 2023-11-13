use super::*;

use crate::vec::Vec3;

use std::fmt::Write;

pub type Color = Vec3;

pub fn write_color(
    buf: &mut String,
    pixel_color: Vec3,
    samples_per_pixel: u32,
) -> Result<(), Box<dyn Error>> {
    let mut r = pixel_color.x;
    let mut g = pixel_color.y;
    let mut b = pixel_color.z;

    // divide r, g, b by samples, essentially averaging the value of the pixel
    // over all samples
    let scale = 1. / samples_per_pixel as f64;

    r *= scale;
    g *= scale;
    b *= scale;

    r = linear_to_gamma(r);
    g = linear_to_gamma(g);
    b = linear_to_gamma(b);

    writeln!(
        buf,
        "{} {} {}",
        (r.clamp(0., 0.999) * 256.) as u32,
        (g.clamp(0., 0.999) * 256.) as u32,
        (b.clamp(0., 0.999) * 256.) as u32
    )?;

    Ok(())
}

fn linear_to_gamma(linear_component: f64) -> f64 {
    linear_component.sqrt()
}
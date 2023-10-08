use super::*;

use crate::vec::Vec3;

use std::fmt::Write;

pub type Color = Vec3;

pub fn write_color(buf: &mut String, v: Vec3) -> Result<(), Box<dyn Error>> {
    writeln!(
        buf,
        "{} {} {}",
        (v.x * 255.).round() as u32,
        (v.y * 255.).round() as u32,
        (v.z * 255.).round() as u32
    )?;

    Ok(())
}

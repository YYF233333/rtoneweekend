use crate::vec3::*;
use std::io::{Result, Write};

pub fn write_color<W: Write>(
    out: &mut W,
    pixel_color: Color,
    samples_per_pixel: i32,
) -> Result<()> {
    let r = pixel_color.x();
    let g = pixel_color.y();
    let b = pixel_color.z();

    // Divide the color by the number of samples and gamma-correct for
    // gamma=2.0.
    let scale = 1.0 / samples_per_pixel as f32;
    let r = (scale * r).sqrt();
    let g = (scale * g).sqrt();
    let b = (scale * b).sqrt();

    // Write the translated [0,255] value of each color component.
    writeln!(
        out,
        "{} {} {}",
        (256. * r.clamp(0.0, 0.999)) as i32,
        (256. * g.clamp(0.0, 0.999)) as i32,
        (256. * b.clamp(0.0, 0.999)) as i32
    )?;
    Ok(())
}

use crate::common::vec3::Color;
use crate::utils::clamp;

pub fn format_color(pixel_color: &Color, samples_per_pixel: i32) -> String {
    let mut r = pixel_color.x;
    let mut g = pixel_color.y;
    let mut b = pixel_color.z;

    let scale = 1.0 / f64::from(samples_per_pixel);
    r = (scale * r).sqrt();
    g = (scale * g).sqrt();
    b = (scale * b).sqrt();

    return format!(
        "{} {} {}",
        (255.999 * clamp(r, 0.0, 1.0 - 1e-8)) as i32,
        (255.999 * clamp(g, 0.0, 1.0 - 1e-8)) as i32,
        (255.999 * clamp(b, 0.0, 1.0 - 1e-8)) as i32,
    )
}
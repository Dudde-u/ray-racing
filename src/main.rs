use glam::{f64, DVec3};
use itertools::Itertools;
use structs::Ray;
mod structs;
use std::{fs, io};
const MAX_VALUE: u8 = 255;
const ASPECT_RATIO: f64 = 16. / 9.;
const IMAGE_WIDTH: u32 = 400;
const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
const VIEWPORT_HEIGHT: f64 = 2.0;
const VIEWPORT_WIDTH: f64 = VIEWPORT_HEIGHT * (IMAGE_WIDTH as f64 / IMAGE_HEIGHT as f64);
const FOCAL_LENGTH: f64 = 1.0;
const CAMERA_CENTER: DVec3 = DVec3::ZERO;
const VIEWPORT_U: DVec3 = DVec3::new(VIEWPORT_WIDTH, 0., 0.);
const VIEWPORT_V: DVec3 = DVec3::new(0., -VIEWPORT_HEIGHT, 0.);
fn main() -> io::Result<()> {
    let pixel_delta_u: DVec3 = VIEWPORT_U / IMAGE_WIDTH as f64;
    let pixel_delta_v: DVec3 = VIEWPORT_V / IMAGE_HEIGHT as f64;
    let viewport_upper_left: DVec3 =
        CAMERA_CENTER - DVec3::new(0., 0., FOCAL_LENGTH) - VIEWPORT_U / 2. - VIEWPORT_V / 2.;
    let viewport_upper_left: DVec3 =
        CAMERA_CENTER - DVec3::new(0., 0., FOCAL_LENGTH) - VIEWPORT_U / 2. - VIEWPORT_V / 2.;
    let pixel00_loc: DVec3 = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);
    let mut pixels = String::new();
    for j in 0..IMAGE_HEIGHT {
        for i in 0..IMAGE_WIDTH {
            let pixel_center =
                pixel00_loc + (i as f64 * pixel_delta_u) + (j as f64 * pixel_delta_v);
            let ray_direction = pixel_center - CAMERA_CENTER;
            let ray = Ray {
                origin: CAMERA_CENTER,
                direction: ray_direction,
            };
            let pixel_color = ray.color_of_ray() * 255.;
            pixels += &format!(
                "{} {} {} \n",
                pixel_color.x.round() as u16,
                pixel_color.y.round() as u16,
                pixel_color.z.round() as u16
            );
        }
    }
    fs::write(
        "output1.ppm",
        format!(
            "P3
{IMAGE_WIDTH} {IMAGE_HEIGHT}
{MAX_VALUE}
{pixels}
"
        ),
    )?;

    Ok(())
}
pub fn has_hit_sphere(center: &DVec3, radius: f64, ray: &Ray) -> bool {
    let oc = *center - ray.origin;
    let a = ray.direction.dot(ray.direction);
    let b = (-2.0) * ray.direction.dot(oc);
    let c = oc.dot(oc) - radius * radius;
    let diskriminant: f64 = b * b - 4. * a * c;
    if diskriminant >= 0. {
        return true;
    }
    return false;
}

use crate::structs::*;
use glam::DVec3;
use rand::prelude::*;
use std::{fs, io};
pub struct Camera {
    image_width: u32,
    image_height: u32,
    max_value: u16,
    aspect_ratio: f64,
    center: DVec3,
    pixel_delta_u: DVec3,
    pixel_delta_v: DVec3,
    samples_per_pixel: u32,
    pixel00_loc: DVec3,
}
impl Camera {
    pub fn new(image_width: u32, aspect_ratio: f64, max_value: u16) -> Self {
        let focal_length: f64 = 1.;
        let viewport_height: f64 = 2.;
        let center: DVec3 = DVec3::ZERO;
        let image_height: u32 = (image_width as f64 / aspect_ratio) as u32;
        let viewport_width: f64 = viewport_height * (image_width as f64 / image_height as f64);
        let viewport_u: DVec3 = DVec3::new(viewport_width, 0., 0.);
        let viewport_v: DVec3 = DVec3::new(0., -viewport_height, 0.);
        let viewport_upper_left: DVec3 =
            center - DVec3::new(0., 0., focal_length) - viewport_u / 2. - viewport_v / 2.;
        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;
        Camera {
            image_width,
            image_height: (image_width as f64 / aspect_ratio) as u32,
            center: DVec3::ZERO,
            aspect_ratio,
            max_value,
            pixel_delta_u,
            pixel_delta_v,
            pixel00_loc: viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v),
            samples_per_pixel: 15,
        }
    }
    pub fn get_ray(&self, i: u32, j: u32) -> Ray {
        let pixel_center =
            self.pixel00_loc + (i as f64 * self.pixel_delta_u) + (j as f64 * self.pixel_delta_v);
        let t1 = thread_rng().gen::<f64>() - 0.5;
        let sample_square: DVec3 = (t1 * self.pixel_delta_u) + (t1 * self.pixel_delta_v);
        let pixel_sample = pixel_center + sample_square;
        let ray_direction = pixel_sample - self.center;
        Ray {
            origin: self.center,
            direction: ray_direction,
        }
    }
    pub fn render_picture<T>(&self, scene: T, version: &str) -> io::Result<()>
    where
        T: Hittable,
    {
        let mut pixels = String::new();
        for j in 0..self.image_height {
            for i in 0..self.image_width {
                let mut pixel_color = self.get_ray(i, j).color(&scene) * 255.;
                for _p in 1..self.samples_per_pixel {
                    pixel_color += self.get_ray(i, j).color(&scene);
                }
                pixels += &format!(
                    "{} {} {} \n",
                    pixel_color.x.round() as u16,
                    pixel_color.y.round() as u16,
                    pixel_color.z.round() as u16
                );
            }
        }
        fs::write(
            format!("images/output{}.ppm", version),
            format!(
                "P3
{} {}
{}
{pixels}
",
                self.image_width, self.image_height, self.max_value
            ),
        )?;

        Ok(())
    }
}

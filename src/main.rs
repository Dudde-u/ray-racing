use camera::*;
use glam::{f64, DVec3};
use structs::*;
mod camera;
mod structs;
mod utilites_for_rand;
use std::io;
const MAX_VALUE: u16 = 255;
const ASPECT_RATIO: f64 = 16. / 9.;
const IMAGE_WIDTH: u32 = 400;
const SCENE_NUMBER: &str = "4";

fn main() -> io::Result<()> {
    let camera: Camera = Camera::new(IMAGE_WIDTH, ASPECT_RATIO, MAX_VALUE);
    let mut scene = HittableList {
        elements: Vec::new(),
    };
    scene.add(Sphere {
        center: DVec3::new(0., 0., -0.8),
        radius: 0.5,
    });
    scene.add(Sphere {
        center: DVec3::new(0., -100.5, -10.),
        radius: 100.,
    });
    camera.render_picture(scene, SCENE_NUMBER)?;
    Ok(())
}
//used earlier
pub fn has_hit_sphere(center: &DVec3, radius: f64, ray: &Ray) -> bool {
    let oc = ray.origin - *center;
    let a = ray.direction.length_squared();
    let h_b = ray.direction.dot(oc);
    let c = oc.dot(oc) - radius * radius;
    let diskriminant: f64 = h_b * h_b - a * c;
    if diskriminant < 0. {
        return true;
    }
    return false;
}

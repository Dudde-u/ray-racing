use crate::has_hit_sphere;
use glam::DVec3;
use std::f64::INFINITY;
pub struct Ray {
    pub origin: DVec3,
    pub direction: DVec3,
}
impl Ray {
    pub fn at(&self, t: f64) -> DVec3 {
        self.origin + t * self.direction
    }
    //used for earlier parts of render
    pub fn color<T>(&self, world: &T) -> DVec3
    where
        T: Hittable,
    {
        if let Some(rec) = world.hit(self, 0., INFINITY) {
            return 0.5 * (rec.normal + DVec3::new(1., 1., 1.));
        }
        let unit_direction: DVec3 = self.direction.normalize();
        let a = 0.5 * (unit_direction.y + 1.);
        return (1. - a) * DVec3::new(1., 1., 1.) + a * DVec3::new(0.5, 0.7, 1.);
    }
    pub fn color_of_ray(&self) -> DVec3 {
        if has_hit_sphere(&DVec3::new(0., 0., -1.), 0.5, &self) {
            return DVec3::new(1., 0., 0.);
        }
        let unit_dir = &self.direction.normalize();
        let a = 0.5 * unit_dir.y + 1.;
        let res_color = (1. - a) * DVec3::new(1., 1., 1.) + a * DVec3::new(0.5, 0.7, 1.);
        return res_color;
    }
}
pub struct HitRecord {
    pub p: DVec3,
    pub normal: DVec3,
    pub t: f64,
    pub is_front_face: bool,
}
pub struct Sphere {
    pub center: DVec3,
    pub radius: f64,
}
pub trait Hittable {
    fn hit(&self, ray: &Ray, ray_tmin: f64, ray_tmx: f64) -> Option<HitRecord>;
}
impl HitRecord {
    pub fn with_face_normal(p: DVec3, outward_normal: DVec3, t: f64, ray: &Ray) -> Self {
        let (is_front_face, normal) = HitRecord::calc_face_normal(ray, &outward_normal);
        HitRecord {
            p,
            normal,
            t,
            is_front_face,
        }
    }
    pub fn calc_face_normal(ray: &Ray, outward_normal: &DVec3) -> (bool, DVec3) {
        let front_face = ray.direction.dot(*outward_normal) < 0.;
        let normal = if front_face {
            *outward_normal
        } else {
            -*outward_normal
        };
        (front_face, normal)
    }
}
impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let h = oc.dot(ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = h * h - a * c;
        if discriminant < 0. {
            return None;
        }
        let sqrtd = discriminant.sqrt();
        let mut root = (-h - sqrtd) / a;
        if root <= ray_tmin || ray_tmax <= root {
            root = (-h + sqrtd) / a;
            if root <= ray_tmin || ray_tmax <= root {
                return None;
            }
        }

        let t = root;
        let point = ray.at(t);
        let outward_normal = (point - self.center) / self.radius;
        let res_record = HitRecord::with_face_normal(point, outward_normal, t, ray);
        return Some(res_record);
    }
}
// very proud of this struct as i actually wrote some rusty code
// no way that i'm actually learning rust
pub struct HittableList {
    pub elements: Vec<Box<dyn Hittable>>,
}
impl HittableList {
    pub fn clear(&mut self) {
        self.elements = vec![]
    }
    pub fn add<T>(&mut self, element: T)
    where
        T: Hittable + 'static,
    {
        self.elements.push(Box::new(element));
    }
}
impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, mut ray_tmin: f64, mut ray_tmax: f64) -> Option<HitRecord> {
        let mut t = ray_tmax;
        let mut closest_hit: Option<HitRecord> = None;
        for element in &self.elements {
            if let Some(new_hit) = element.hit(ray, ray_tmin, t) {
                t = new_hit.t;
                closest_hit = Some(new_hit);
            }
        }
        return closest_hit;
    }
}

use crate::has_hit_sphere;
use glam::DVec3;
pub struct Ray {
    pub origin: DVec3,
    pub direction: DVec3,
}
impl Ray {
    pub fn at(&self, t: f64) -> DVec3 {
        self.origin + t * self.direction
    }
    //used for earlier parts of render
    pub fn color(&self) -> DVec3 {
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

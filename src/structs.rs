use glam::DVec3;
pub struct Ray {
    pub origin: DVec3,
    pub direction: DVec3,
}
impl Ray {
    fn at(&self, t: f64) -> DVec3 {
        self.origin + t * self.direction
    }
    fn color(&self) -> DVec3 {
        let unit_direction: DVec3 = self.direction.normalize();
        let a = 0.5 * (unit_direction.y + 1.);
        return (1. - a) * DVec3::new(1., 1., 1.) + a * DVec3::new(0.5, 0.7, 1.);
    }
}

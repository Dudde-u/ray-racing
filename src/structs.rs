use glam::DVec3;
pub struct Ray {
    pub origin: DVec3,
    pub direction: DVec3,
}
impl Ray {
    fn at(&self, t: f64) -> DVec3 {
        self.origin + t * self.direction
    }
}
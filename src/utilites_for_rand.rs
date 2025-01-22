use glam::DVec3;
use rand::{thread_rng, Rng};
pub fn random_checked_vector(normal: &DVec3) -> DVec3 {
    let mut rng = thread_rng();
    let mut random_vector: DVec3 = DVec3::new(
        rng.gen_range(-1.0..1.),
        rng.gen_range(-1.0..1.),
        rng.gen_range(-1.0..1.),
    );

    loop {
        if random_vector.length_squared() < 1. {
            break;
        }
        random_vector = DVec3::new(
            rng.gen_range(-1.0..1.),
            rng.gen_range(-1.0..1.),
            rng.gen_range(-1.0..1.),
        );
    }

    let normalized_vector = random_vector.normalize();

    if normalized_vector.dot(*normal) > 0. {
        return normalized_vector;
    } else {
        return -normalized_vector;
    }
}

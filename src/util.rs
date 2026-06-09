use rand::Rng;

pub fn random_range(min: f32, max: f32) -> f32 {
    rand::rng().random_range(min..max)
}

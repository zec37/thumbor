use crate::pb::Spec;
use image::ImageOutputFormat;

mod photon;
pub use photon::Photon;

// Engine trait: For more engine in the future
pub trait Engine {

    // Operating on engine per specs
    fn apply(&mut self, specs: &[Spec]);

    // Generate image from engine, notice that self is used, not a reference to self
    fn generate(self, format: ImageOutputFormat) -> Vec<u8>;

}

pub trait SpecTransform<T> {
    fn transform(&mut self, op: T);
}
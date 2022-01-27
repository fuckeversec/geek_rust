use crate::pb::Spec;
use image::ImageOutputFormat;
mod photon;
pub use photon::Photon;

// 图片处理engine
pub trait Engine {
    fn apply(&mut self, specs: &[Spec]);

    fn generate(self, format: ImageOutputFormat) -> Vec<u8>;
}

// 对各种spec进行具体处理
pub trait SpecTransform<T> {
    fn transform(&mut self, op: T);
}

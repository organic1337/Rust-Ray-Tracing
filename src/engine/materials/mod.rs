pub mod material;
pub mod lambertian;
pub mod metal;
pub mod dielectric;

pub use material::{ScatterResult, Material};
pub use lambertian::Lambertian;
pub use metal::Metal;
pub use dielectric::Dielectric;
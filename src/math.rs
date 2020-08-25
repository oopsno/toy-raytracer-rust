use rand::prelude::random as rand;

#[cfg(feature = "single-precision")]
pub type Float = f32;
#[cfg(feature = "single-precision")]
pub use std::f32 as floats;

#[cfg(not(feature = "single-precision"))]
pub type Float = f64;
#[cfg(not(feature = "single-precision"))]
pub use std::f64 as floats;

pub fn random() -> Float {
    rand::<Float>()
}

pub fn random_between(min: Float, max: Float) -> Float {
    min + (max - min) * rand::<Float>()
}

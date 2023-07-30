use fastrand;

#[cfg(feature = "single-precision")]
pub type Float = f32;

#[cfg(feature = "single-precision")]
pub use std::f32 as floats;

#[cfg(feature = "single-precision")]
pub fn random() -> Float {
    fastrand::f32()
}

#[cfg(not(feature = "single-precision"))]
pub type Float = f64;

#[cfg(not(feature = "single-precision"))]
pub use std::f64 as floats;

#[cfg(not(feature = "single-precision"))]
pub fn random() -> Float {
    fastrand::f64()
}

pub fn random_between(min: Float, max: Float) -> Float {
    min + (max - min) * random()
}

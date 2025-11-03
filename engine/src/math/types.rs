use nalgebra::{Point3, UnitQuaternion, Vector3};

pub type Vector = Vector3<f32>;
pub type Quaternion = UnitQuaternion<f32>;
pub type Point = Point3<f32>;

pub trait VectorExt {
	fn new(x: f32, y: f32, z: f32) -> Self;
	fn zero() -> Self;
	fn one() -> Self;
}
impl VectorExt for Vector {
	#[inline]
	fn new(x: f32, y: f32, z: f32) -> Self {
		Self::new(x, y, z)
	}

	#[inline]
	fn zero() -> Self {
		Self::new(0.0, 0.0, 0.0)
	}

	#[inline]
	fn one() -> Self {
		Self::new(1.0, 1.0, 1.0)
	}
}

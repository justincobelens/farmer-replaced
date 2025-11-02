use nalgebra::{Point3, UnitQuaternion, Vector3};

pub type Vector = Vector3<f32>;
pub type Quaternion = UnitQuaternion<f32>;
pub type Point = Point3<f32>;

pub const fn vector(x: f32, y: f32, z: f32) -> [f32; 3] {
	[x, y, z]
}

pub trait VectorExt {
	fn new(x: f32, y: f32, z: f32) -> Self;
	fn zero() -> Self;
	fn one() -> Self;
	fn x() -> Self;
	fn y() -> Self;
	fn z() -> Self;
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
	#[inline]
	fn x() -> Self {
		Self::new(1.0, 0.0, 0.0)
	}
	#[inline]
	fn y() -> Self {
		Self::new(0.0, 1.0, 0.0)
	}
	#[inline]
	fn z() -> Self {
		Self::new(0.0, 0.0, 1.0)
	}
}

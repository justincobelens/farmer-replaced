use super::{Quaternion, Vector};

#[derive(Clone, Debug)]
pub struct Transform {
	pub location: Vector,
	pub rotation: Quaternion,
	pub scale: Vector,
}

impl Default for Transform {
	fn default() -> Self {
		Self {
			location: Vector::new(0.0, 0.0, 0.0),
			rotation: Quaternion::identity(),
			scale: Vector::new(1.0, 1.0, 1.0),
		}
	}
}

impl Transform {
	pub fn inverse(&self) -> Transform {
		let inversed_scale = Vector::new(
			if self.scale.x != 0.0 { 1.0 / self.scale.x } else { 0.0 },
			if self.scale.y != 0.0 { 1.0 / self.scale.y } else { 0.0 },
			if self.scale.z != 0.0 { 1.0 / self.scale.z } else { 0.0 },
		);
		let inversed_rotation = self.rotation.inverse();
		let inversed_location = inversed_rotation.transform_vector(&(self.location.component_mul(&(-inversed_scale))));
		Transform {
			location: inversed_location,
			rotation: inversed_rotation,
			scale: inversed_scale,
		}
	}

	#[inline]
	pub fn translate(&mut self, delta: Vector) {
		self.location += delta;
	}

	#[inline]
	pub fn set_location(&mut self, location: Vector) {
		self.location = location;
	}
	#[inline]
	pub fn set_rotation(&mut self, rotation: Quaternion) {
		self.rotation = rotation;
	}
	#[inline]
	pub fn set_scale(&mut self, scale: Vector) {
		self.scale = scale;
	}
}

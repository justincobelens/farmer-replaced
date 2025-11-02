use super::{Quaternion, Vector};
use nalgebra::UnitQuaternion;

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
	#[inline]
	pub fn identity() -> Self {
		Self::default()
	}

	#[inline]
	pub fn from_transform(location: Vector, rotation: Quaternion, scale: Vector) -> Self {
		Self {
			location,
			rotation,
			scale,
		}
	}

	/// Compose `self` (parent/world) with `local` to produce a world transform.
	pub fn compose(&self, local: &Transform) -> Transform {
		// World S = parent.S * local.S
		let scale = Vector::new(
			self.scale.x * local.scale.x,
			self.scale.y * local.scale.y,
			self.scale.z * local.scale.z,
		);

		// World R = parent.R * local.R
		let rotation = self.rotation * local.rotation;

		// World T = parent.T + parent.R * (parent.S * local.T)
		let scaled_local_t = Vector::new(
			self.scale.x * local.location.x,
			self.scale.y * local.location.y,
			self.scale.z * local.location.z,
		);
		let rotated = self.rotation.transform_vector(&scaled_local_t);
		let location = self.location + rotated;

		Transform {
			location,
			rotation,
			scale,
		}
	}

	/// Inverse transform
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

	/// Add a yaw/pitch/roll in radians
	#[inline]
	pub fn rotate_euler(&mut self, yaw: f32, pitch: f32, roll: f32) {
		let rotation = UnitQuaternion::from_euler_angles(pitch, yaw, roll);
		self.rotation = self.rotation * rotation;
	}

	#[inline]
	pub fn set_yaw(&mut self, yaw: f32) {
		let rotation = UnitQuaternion::from_euler_angles(0.0, yaw, 0.0);
		self.rotation = rotation;
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

	/// Move toward `target` by up to `max_step` length.
	pub fn move_towards(&mut self, target: Vector, max_step: f32) {
		let delta = target - self.location;
		let dist2 = delta.dot(&delta);
		if dist2 <= max_step * max_step {
			self.location = target;
		} else if dist2 > 0.0 {
			let len = dist2.sqrt();
			self.location += delta * (max_step / len);
		}
	}
}

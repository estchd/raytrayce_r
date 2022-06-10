use std::fmt::{Display, Formatter};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use rand::{Rng, thread_rng};

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Vec3 {
	pub x: f64,
	pub y: f64,
	pub z: f64,
}

impl Display for Vec3 {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "{} {} {}", self.x, self.y, self.z)
	}
}

impl Vec3 {
	pub fn new() -> Self {
		Self {
			x: 0.0,
			y: 0.0,
			z: 0.0
		}
	}

	pub fn create(x: f64, y: f64, z: f64) -> Self{
		Self {
			x,
			y,
			z
		}
	}

	pub fn length_squared(&self) -> f64 {
		self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
	}

	pub fn length(&self) -> f64 {
		self.length_squared().sqrt()
	}

	pub fn dot(lhs: Vec3, rhs: Vec3) -> f64 {
		lhs.x * rhs.x + lhs.y * rhs.y + lhs.z * rhs.z
	}

	pub fn cross(lhs: Vec3, rhs: Vec3) -> Vec3 {
		Vec3 {
			x: lhs.y * rhs.z - lhs.z * rhs.y,
			y: lhs.z * rhs.x - lhs.x * rhs.z,
			z: lhs.x * rhs.y - lhs.y * rhs.x,
		}
	}

	pub fn normalize(&mut self) {
		*self /= self.length()
	}

	pub fn normalized(self) -> Vec3 {
		self / self.length()
	}

	pub fn random_0_1() -> Self {
		Self::random(0.0, 1.0)
	}

	pub fn random(min: f64, max: f64) -> Self {
		let mut rand = thread_rng();

		Self {
			x: rand.gen_range(min..max),
			y: rand.gen_range(min..max),
			z: rand.gen_range(min..max)
		}
	}

	pub fn random_in_unit_sphere() -> Self {
		let mut rand = thread_rng();
		let direction = Self::random_0_1().normalized();
		let t = rand.gen_range(-1.0..=1.0);

		direction * t
	}

	pub fn random_normalized() -> Self {
		Self::random_in_unit_sphere().normalized()
	}

	pub fn random_in_hemisphere(normal: Vec3) -> Self {
		let in_unit_sphere = Self::random_in_unit_sphere();
		if Self::dot(in_unit_sphere, normal) > 0.0 {
			in_unit_sphere
		}
		else {
			-in_unit_sphere
		}
	}

	pub fn random_in_unit_disk() -> Self {
		let mut rand = thread_rng();

		let angle: f64 = rand.gen_range(0.0..=360.0);
		let distance: f64 = rand.gen_range(0.0..=1.0);

		let x = angle.to_radians().cos();
		let y = angle.to_radians().sin();

		let vec = Vec3 {
			x,
			y,
			z: 0.0
		};

		vec * distance
	}

	pub fn near_zero(&self, threshold: f64) -> bool {
		self.x < threshold && self.y < threshold && self.z < threshold
	}
}

impl Default for Vec3 {
	fn default() -> Self {
		Self::new()
	}
}

impl Neg for Vec3 {
	type Output = Vec3;

	fn neg(self) -> Self::Output {
		Vec3 {
			x: self.x * -1.0,
			y: self.y * -1.0,
			z: self.z * -1.0
		}
	}
}

impl AddAssign<Vec3> for Vec3 {
	fn add_assign(&mut self, rhs: Vec3) {
		self.x += rhs.x;
		self.y += rhs.y;
		self.z += rhs.z;
	}
}

impl AddAssign<&Vec3> for Vec3 {
	fn add_assign(&mut self, rhs: &Vec3) {
		self.x += rhs.x;
		self.y += rhs.y;
		self.z += rhs.z;
	}
}

impl AddAssign<f64> for Vec3 {
	fn add_assign(&mut self, rhs: f64) {
		self.x += rhs;
		self.y += rhs;
		self.z += rhs;
	}
}

impl Add<Vec3> for Vec3 {
	type Output = Vec3;

	fn add(self, rhs: Vec3) -> Self::Output {
		Vec3 {
			x: self.x + rhs.x,
			y: self.y + rhs.y,
			z: self.z + rhs.z
		}
	}
}

impl Add<f64> for Vec3 {
	type Output = Vec3;

	fn add(self, rhs: f64) -> Self::Output {
		Vec3 {
			x: self.x + rhs,
			y: self.y + rhs,
			z: self.z + rhs
		}
	}
}

impl SubAssign<Vec3> for Vec3 {
	fn sub_assign(&mut self, rhs: Vec3) {
		self.x -= rhs.x;
		self.y -= rhs.y;
		self.z -= rhs.z;
	}
}

impl SubAssign<&Vec3> for Vec3 {
	fn sub_assign(&mut self, rhs: &Vec3) {
		self.x -= rhs.x;
		self.y -= rhs.y;
		self.z -= rhs.z;
	}
}

impl SubAssign<f64> for Vec3 {
	fn sub_assign(&mut self, rhs: f64) {
		self.x -= rhs;
		self.y -= rhs;
		self.z -= rhs;
	}
}

impl Sub<Vec3> for Vec3 {
	type Output = Vec3;

	fn sub(self, rhs: Vec3) -> Self::Output {
		Vec3 {
			x: self.x - rhs.x,
			y: self.y - rhs.y,
			z: self.z - rhs.z
		}
	}
}

impl Sub<f64> for Vec3 {
	type Output = Vec3;

	fn sub(self, rhs: f64) -> Self::Output {
		Vec3 {
			x: self.x - rhs,
			y: self.y - rhs,
			z: self.z - rhs
		}
	}
}

impl MulAssign<Vec3> for Vec3 {
	fn mul_assign(&mut self, rhs: Vec3) {
		self.x *= rhs.x;
		self.y *= rhs.y;
		self.z *= rhs.z;
	}
}

impl MulAssign<&Vec3> for Vec3 {
	fn mul_assign(&mut self, rhs: &Vec3) {
		self.x *= rhs.x;
		self.y *= rhs.y;
		self.z *= rhs.z;
	}
}

impl MulAssign<f64> for Vec3 {
	fn mul_assign(&mut self, rhs: f64) {
		self.x *= rhs;
		self.y *= rhs;
		self.z *= rhs;
	}
}

impl Mul<Vec3> for Vec3 {
	type Output = Vec3;

	fn mul(self, rhs: Vec3) -> Self::Output {
		Vec3 {
			x: self.x * rhs.x,
			y: self.y * rhs.y,
			z: self.z * rhs.z
		}
	}
}

impl Mul<f64> for Vec3 {
	type Output = Vec3;

	fn mul(self, rhs: f64) -> Self::Output {
		Vec3 {
			x: self.x * rhs,
			y: self.y * rhs,
			z: self.z * rhs
		}
	}
}

impl DivAssign<Vec3> for Vec3 {
	fn div_assign(&mut self, rhs: Vec3) {
		self.x /= rhs.x;
		self.y /= rhs.y;
		self.z /= rhs.z;
	}
}

impl DivAssign<&Vec3> for Vec3 {
	fn div_assign(&mut self, rhs: &Vec3) {
		self.x /= rhs.x;
		self.y /= rhs.y;
		self.z /= rhs.z;
	}
}

impl DivAssign<f64> for Vec3 {
	fn div_assign(&mut self, rhs: f64) {
		self.x /= rhs;
		self.y /= rhs;
		self.z /= rhs;
	}
}

impl Div<Vec3> for Vec3 {
	type Output = Vec3;

	fn div(self, rhs: Vec3) -> Self::Output {
		Vec3 {
			x: self.x / rhs.x,
			y: self.y / rhs.y,
			z: self.z / rhs.z
		}
	}
}

impl Div<f64> for Vec3 {
	type Output = Vec3;

	fn div(self, rhs: f64) -> Self::Output {
		Vec3 {
			x: self.x / rhs,
			y: self.y / rhs,
			z: self.z / rhs
		}
	}
}
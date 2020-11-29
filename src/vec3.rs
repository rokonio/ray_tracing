#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
	e: [f64; 3]
}

impl Vec3 {
	pub fn new(e0: f64, e1: f64, e2: f64) -> Vec3 {
		Vec3{ e: [e0, e1, e2] }
	}

	pub fn zeros() -> Vec3 {
		Vec3{ e: [0., 0., 0.] }
	}

	pub fn x(&self) -> f64 {self.e[0]}
	pub fn y(&self) -> f64 {self.e[1]}
	pub fn z(&self) -> f64 {self.e[2]}

	pub fn length_squared(&self) -> f64 {
		self.e[0]*self.e[0] + self.e[1]*self.e[1] + self.e[2]*self.e[2]
	}

	pub fn length(&self) -> f64 {
		f64::sqrt(self.length_squared())
	}
}

// Implement addition (Vec3 + Vec3)
impl std::ops::Add for Vec3 {
	type Output = Self;

	fn add(self, other: Self) -> Self {
		Self::new(
			self.e[0] + other.e[0],
			self.e[1] + other.e[1],
			self.e[2] + other.e[2])
	} 
}

// Implement Substraction (Vec3 - Vec3)
impl std::ops::Sub for Vec3 {
	type Output = Self;

	fn sub(self, other: Self) -> Self {
		Self::new(
			self.e[0] - other.e[0],
			self.e[1] - other.e[1],
			self.e[2] - other.e[2])
	} 
}

// Implement multiplication (Vec3 * Vec3)
impl std::ops::Mul for Vec3 {
	type Output = Self;

	fn mul(self, other: Self) -> Self {
		Self::new(
			self.e[0] * other.e[0],
			self.e[1] * other.e[1],
			self.e[2] * other.e[2])
	} 
}

// Implement multiplication (Vec3 * number)
impl std::ops::Mul<f64> for Vec3 {
	type Output = Self;

	fn mul(self, other: f64) -> Self {
		Self::new(
			other*self.e[0],
			other*self.e[1],
			other*self.e[2])
	}
}

// Implement multiplication (number * Vec3)
impl std::ops::Mul<Vec3> for f64 {
	type Output = Vec3;

	fn mul(self, other: Vec3) -> Vec3 {
		other * self
	}
}

// Implement division (Vec3 / number)
impl std::ops::Div<f64> for Vec3 {
	type Output = Self;

	fn div(self, other: f64) -> Self{
		(1./other) * self
	}
}

// Implement negation (-Vec3)
impl std::ops::Neg for Vec3 {
	type Output = Self;

	fn neg(self) -> Self {
		Vec3::new(
			-self.e[0],
			-self.e[1],
			-self.e[2])
	}
}

// Implement index (for immutable only)
impl std::ops::Index<usize> for Vec3 {
	type Output = f64;

	fn index(&self, index: usize) -> &f64 {
		&self.e[index]
	}
}

// Implement index (for mutable only)
impl std::ops::IndexMut<usize> for Vec3 {
	fn index_mut(&mut self, index: usize) -> &mut f64 {
		&mut self.e[index]
	}
}

// Implement addition assigment (Vec3 += Vec3)
impl std::ops::AddAssign for Vec3 {
	fn add_assign(&mut self, other: Self) {
		self.e[0] += other.e[0];
		self.e[1] += other.e[1];
		self.e[2] += other.e[2];
	}
}

// Implement multiplication assigment (Vec3 *= Vec3)
impl std::ops::MulAssign<f64> for Vec3 {
	fn mul_assign(&mut self, other: f64) {
		self.e[0] *= other;
		self.e[1] *= other;
		self.e[2] *= other;
	}
}

// Implement division assigment (Vec3 /= Vec3)
impl std::ops::DivAssign<f64> for Vec3 {
	fn div_assign(&mut self, other: f64) {
		*self *= 1./other;
	}
}

pub fn dot(u: &Vec3, v: &Vec3) -> f64 {
	u.e[0] * v.e[0] +
	u.e[1] * v.e[1] +
	u.e[2] * v.e[2]
}

pub fn cross(u: &Vec3, v: &Vec3) -> Vec3 {
	Vec3::new(
		u.e[1] * v.e[2] - u.e[2] * v.e[1],
		u.e[2] * v.e[0] - u.e[0] * v.e[2],
		u.e[0] * v.e[1] - u.e[1] * v.e[0])
}

pub fn unit_vector(v: &Vec3) -> Vec3 {
	*v / v.length()
}

impl std::fmt::Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.x(), self.y(), self.z())
    }
}

pub use Vec3 as Color;
pub use Vec3 as Point3;
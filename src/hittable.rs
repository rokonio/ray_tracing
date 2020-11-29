use super::vec3::{Point3, Vec3};
use super::ray::Ray;

pub struct HitRecord {
	pub p: Point3,
	pub normal: Vec3,
	pub t: f64,
}

pub trait Hittable {
	fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}
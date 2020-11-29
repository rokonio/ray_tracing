use super::vec3::*;
use super::hittable::{HitRecord, Hittable};
use super::ray::Ray;

struct Sphere {
	center: Point3,
	radius: f64,
}

impl Sphere {
	pub fn new(cen: Point3, r: f64) -> Sphere {
		Sphere {
			center: cen,
			radius: r,
		}
	}
}

impl Hittable for Sphere {
	fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
		let oc = r.origin() - self.center;
		let a = r.direction().length_squared();
		let half_b = dot(&oc, &r.direction());
		let c = oc.length_squared() - self.radius*self.radius;

		let discriminant = half_b*half_b - a*c;
		if discriminant < 0. {return false;}
		let sqrtd = f64::sqrt(discriminant);

		// Find the nearest root that lies in the acceptable range.
		let mut root = (-half_b - sqrtd) / a;
		if root  < t_min || t_max < root {
			root = (-half_b + sqrtd) / a;
			if root  < t_min || t_max < root {
				return false;
			}
		}

		rec.t = root;
		rec.p = r.at(rec.t);
		rec.normal = (rec.p - self.center) / self.radius;

		true
	}
	
}
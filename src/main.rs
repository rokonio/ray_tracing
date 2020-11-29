mod vec3;
mod color;
mod ray;

use vec3::*;
use color::*;
use ray::*;

fn hit_sphere(center: &Point3, radius: f64, r: &Ray) -> f64 {
	let oc = r.origin() - *center;
	let a = dot(&r.direction(), &r.direction());
	let b = 2. * dot(&oc, &r.direction());
	let c = dot(&oc, &oc) - radius*radius;
	let discriminant = b*b - 4.*a*c;
	if discriminant  < 0. {
		return -1.;
	} else {
		return (-b - f64::sqrt(discriminant)) / (a * 2.);
	}
}

fn ray_color(r: &Ray) -> Color {
	let t = hit_sphere(&Point3::new(0., 0., -1.), 0.5, &r);
	if t > 0. {
		let n = unit_vector(&(r.at(t) - Vec3::new(0., 0., -1.)));
		return 0.5*Color::new(n.x()+1., n.y()+1., n.z()+1.);
	}
	let unit_direction = unit_vector(&r.direction());
	let t = 0.5*(unit_direction.y() + 1.);
	(1. - t) * Color::new(1., 1., 1.) + t * Color::new(0.5, 0.7, 1.)
}

fn main() {

	// Image
	const ASPECT_RATIO: f64 = 16. / 9.;
	const IMG_WIDTH: u32 = 400;
	const IMG_HEIGHT: u32 = (IMG_WIDTH as f64 / ASPECT_RATIO) as u32;

	// Camera

	let viewport_height = 2.;
	let viewport_width = ASPECT_RATIO * viewport_height;
	let focal_length = 1.;

	let origin = Point3::new(0., 0., 0.);
	let horizontal = Vec3::new(viewport_width, 0., 0.);
	let vertical = Vec3::new(0., viewport_height, 0.);
	let lower_left_corner = origin - horizontal/2. - vertical/2. - Vec3::new(0., 0., focal_length);

	let mut out = String::with_capacity(
		(
			IMG_WIDTH*IMG_HEIGHT*
			((IMG_HEIGHT).to_string().len() + (IMG_WIDTH).to_string().len() + 3) as u32
		)
		as usize
	);

	// Render

	out.push_str(format!("P3\n{} {}\n255\n", IMG_WIDTH, IMG_HEIGHT).as_str());

	for j in (0..IMG_HEIGHT).rev() {
		eprintln!("Scanlines ramaining: {}", j);
		for i in 0..IMG_WIDTH {
			let u = i as f64 / (IMG_WIDTH-1) as f64;
			let v = j as f64 / (IMG_HEIGHT-1) as f64;

			let r = Ray::new(origin, lower_left_corner + u*horizontal + v*vertical - origin);
			let pixel_color = ray_color(&r);
			write_color(&mut out, pixel_color);
		}
	}

	println!("{}", out);

	eprintln!("Done.");

	// Note: After building command >target\debug\ray_tracing.exe > output.ppm
}

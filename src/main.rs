mod vec3;
mod color;
mod ray;

use vec3::*;
use color::*;
use ray::*;

fn ray_color(r: &Ray) -> Color {
	let unit_direction = unit_vector(&r.direction());
	let t = 0.5*(unit_direction.y() + 1.);
	(1. - t) * Color::new(1., 1., 1.) + t * Color::new(0.2, 0.5, 1.)
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
			let u = (i as f64) / ((IMG_HEIGHT-1) as f64);
			let v = (j as f64) / ((IMG_WIDTH-1) as f64);
			let r = Ray::new(origin, lower_left_corner + u*horizontal + v*vertical - origin);
			let pixel_color = ray_color(&r);
			write_color(&mut out, pixel_color);
		}
	}

	println!("{}", out);

	eprintln!("Done.");

	// Note: After building command >target\debug\ray_tracing.exe > output.ppm
}

mod vec3;
mod color;

use vec3::*;
use color::*;

use std::io::{self, Write};

fn main() {

	// Image

	const IMG_WIDTH: u32 = 256;
	const IMG_HEIGHT: u32 = 256;

	// Render

	println!("P3\n{} {}\n255", IMG_WIDTH, IMG_HEIGHT);

	for j in (0..IMG_HEIGHT).rev() {
		match io::stderr().write_all(
			format!("Scanlines ramaining: {}\n", j)
			.as_bytes()
		) {
			Err(_) => panic!("Can't write in the stderr..."),
			Ok(_) => (),
		}
		for i in 0..IMG_WIDTH {
			let pixel_color = Color::new(
				(i as f64) / ((IMG_HEIGHT-1) as f64),
				(j as f64) / ((IMG_WIDTH-1) as f64),
				0.25,
			);
			write_color(io::stdout(), pixel_color);
		}
	}

	// Note: After building command >target\debug\ray_tracing.exe > output.ppm
}
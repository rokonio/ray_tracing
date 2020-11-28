mod vec3;
mod color;

use vec3::*;
use color::*;

fn main() {

	// Image

	const IMG_WIDTH: u32 = 256;
	const IMG_HEIGHT: u32 = 256;

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
			let pixel_color = Color::new(
				(i as f64) / ((IMG_HEIGHT-1) as f64),
				(j as f64) / ((IMG_WIDTH-1) as f64),
				0.25,
			);
			write_color(&mut out, pixel_color);
		}
	}

	println!("{}", out);

	eprintln!("Done.")
	
	// Note: After building command >target\debug\ray_tracing.exe > output.ppm
}

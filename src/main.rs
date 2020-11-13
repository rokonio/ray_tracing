fn main() {

	// Image

	const IMG_WIDTH: u32 = 256;
	const IMG_HEIGHT: u32 = 256;

	// Render

	println!("P3\n{} {}\n255", IMG_WIDTH, IMG_HEIGHT);

	for j in (0..IMG_HEIGHT).rev() {
		for i in 0..IMG_WIDTH {
			let r = i as f64 / (IMG_WIDTH-1) as f64;
			let g = j as f64 / (IMG_HEIGHT-1) as f64;
			let b = 0.25;

			let ir = (255.999 * r) as i32;
			let ig = (255.999 * g) as i32;
			let ib = (255.999 * b) as i32;

			println!("{} {} {}", ir, ig, ib);
		}
	}

	// Note: After building command >target\debug\ray_tracing.exe > output.ppm
}
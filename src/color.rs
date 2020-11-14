use super::vec3::Color;

use std::io::Write;

pub fn write_color(mut out: std::io::Stdout, pixel_color: Color) {
	match out.write_all(format!(
		"{} {} {}\n",
		(255.999 * pixel_color.x()) as i32,
		(255.999 * pixel_color.y()) as i32,
		(255.999 * pixel_color.z()) as i32)
	.as_bytes()) {
		Err(_) => panic!("Can't write the color"),
		Ok(_) => (),
	};
}
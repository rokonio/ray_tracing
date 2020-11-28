use super::vec3::Color;

pub fn write_color(out: &mut String, pixel_color: Color) {
	out.push_str(format!(
		"{} {} {}\n",
		(255.999 * pixel_color.x()) as i32,
		(255.999 * pixel_color.y()) as i32,
		(255.999 * pixel_color.z()) as i32).as_str())
}
use image::{self, Luma};
use magick_rust::{magick_wand_genesis, DrawingWand, MagickWand};
use qrcode::{EcLevel, QrCode, Version};
use std::sync::Once;

use std::fs::File;
use std::io::{BufReader, Read, Result};

static START: Once = Once::new();

const INPUT: &str = "src/inc/test.pub";
const OUTPUT: &str = "test.png";
const TEXT: &str = "Scan this code to reset your password";

fn main() {
	match encode() {
		Ok(_) => {
			add_text().unwrap();
			println!("`{}`", decode());
		}
		Err(e) => println!("failed to encode with error: {}", e),
	}
}

fn encode() -> Result<()> {
	let f = File::open(INPUT)?;
	let mut reader = BufReader::new(f);
	let mut buffer = Vec::new();
	reader.read_to_end(&mut buffer)?;

	let code = QrCode::with_version(buffer, Version::Normal(18), EcLevel::L).unwrap();
	let image = code.render::<Luma<u8>>().build();

	image.save(OUTPUT).unwrap();

	Ok(())
}

fn add_text() -> Result<()> {
	START.call_once(|| {
		magick_wand_genesis();
	});
	let mut wand = MagickWand::new();
	let mut draw = DrawingWand::new();
	wand.read_image(OUTPUT).unwrap();

	let width = wand.get_image_width();
	let height = wand.get_image_height();
	wand.extend_image(width, height + 64, 0, 0).unwrap();
	draw.set_font_size(42.0);
	draw.set_font_weight(32);
	wand.annotate_image(&draw, 32.0, (height + 28) as f64, 0.0, TEXT)
		.unwrap();

	wand.write_image(OUTPUT).unwrap();
	Ok(())
}

fn decode() -> String {
	let img = image::open(OUTPUT).unwrap();
	let decoder = bardecoder::default_decoder();
	let mut results = decoder.decode(&img).into_iter();

	let mut out = String::new();

	while let Some(Ok(contents)) = results.next() {
		out = format!("{}{}", out, contents);
	}
	out
}

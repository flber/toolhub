use image;
use magick_rust::{magick_wand_genesis, DrawingWand, MagickWand};

use fast_qr::convert::image::ImageBuilder;
use fast_qr::qr::QRBuilder;

use std::fs::File;
use std::io::{BufReader, Read, Result};
use std::sync::Once;

static START: Once = Once::new();
const TEXT: &str = "Scan this code to reset your password";

pub fn encode(input: &str, output: &str) -> Result<()> {
	let f = File::open(input)?;
	let mut reader = BufReader::new(f);
	let mut buffer = Vec::new();
	reader.read_to_end(&mut buffer)?;

	let qrcode = QRBuilder::new(buffer)
		.build()
		.unwrap();
	
	let _img = ImageBuilder::default()
		.fit_width(780)
		.to_file(&qrcode, output);

	add_text(output, TEXT)?;

	Ok(())
}

fn add_text(file: &str, text: &str) -> Result<()> {
	START.call_once(|| {
		magick_wand_genesis();
	});
	let mut wand = MagickWand::new();
	let mut draw = DrawingWand::new();
	wand.read_image(file).unwrap();

	let width = wand.get_image_width();
	let height = wand.get_image_height();
	wand.extend_image(width, height + 64, 0, 0).unwrap();
	draw.set_font_size(42.0);
	draw.set_font_weight(32);
	wand.annotate_image(&draw, 32.0, (height + 28) as f64, 0.0, text)
		.unwrap();

	wand.write_image(file).unwrap();
	Ok(())
}

pub fn decode(input: &str) -> String {
	let img = image::open(input).unwrap();
	let decoder = bardecoder::default_decoder();
	let mut results = decoder.decode(&img).into_iter();

	let mut out = String::new();

	while let Some(Ok(contents)) = results.next() {
		out = format!("{}{}", out, contents);
	}
	out
}

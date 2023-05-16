use artano::{Annotation, Canvas, Position};
use image::{self, /*GenericImage, GenericImageView,*/ Luma};
use qrcode::{EcLevel, QrCode, Version};
use rusttype::Font;
use text_to_png::TextRenderer;

use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Result, Write};

const INPUT: &str = "/home/benh/.ssh/id_rsa.pub";
const OUTPUT: &str = "./test.png";
// const TEXT: &str = "./text.png";

fn main() {
	match encode() {
		Ok(_) => {
			// generate_text().unwrap();
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

// fn generate_text() -> Result<()> {
// 	let renderer = TextRenderer::default();
// 	let text_img = renderer
// 		.render_text_to_png_data("hello world", 64, "#000")
// 		.unwrap();
// 
// 	let output_file = File::create(TEXT)?;
// 	let mut writer = BufWriter::new(output_file);
// 	writer.write_all(&text_img.data)?;
// 
// 	Ok(())
// }

fn add_text() -> Result<()> {
	// let images = &[image::open(OUTPUT).unwrap(), image::open(TEXT).unwrap()];
	// let img_width_out: u32 = images.iter().map(|im| im.width()).max().unwrap_or(0);
	// let img_height_out: u32 = images.iter().map(|im| im.height()).sum();
	// let mut imgbuf = image::ImageBuffer::new(img_width_out, img_height_out);
	// let mut accumulated_height = 0;
	// for img in images {
	// 	imgbuf.copy_from(img, accumulated_height, 0);
	// 	accumulated_height += img.height();
	// }
	// imgbuf.save(OUTPUT).unwrap();
	let f = File::open(OUTPUT)?;
	let mut reader = BufReader::new(f);
	let mut buffer = Vec::new();
	reader.read_to_end(&mut buffer)?;

	let mut canvas = Canvas::read_from_buffer(&buffer).unwrap();

	let font_data = include_bytes!("inc/DejaVuSans.ttf");
	let font = Font::try_from_bytes(font_data as &[u8]).unwrap();

	canvas.add_annotation(
		&Annotation {
			text: String::from("hello world"),
			position: Position::Bottom,
		},
		&font,
		1.0,
	);

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

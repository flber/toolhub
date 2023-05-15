use image::{self, Luma};
use qrcode::{EcLevel, QrCode, Version};

use std::fs::File;
use std::io::{BufReader, Read, Result};

const INPUT: &str = "/home/benh/.ssh/id_rsa.pub";
const OUTPUT: &str = "./test.jpg";

fn main() -> Result<()> {
	let f = File::open(INPUT)?;
	let mut reader = BufReader::new(f);
	let mut buffer = Vec::new();
	reader.read_to_end(&mut buffer)?;

	let code = QrCode::with_version(buffer, Version::Normal(18), EcLevel::L).unwrap();
	let image = code.render::<Luma<u8>>().build();

	image.save(OUTPUT).unwrap();

	let img = image::open(OUTPUT).unwrap();
	let decoder = bardecoder::default_decoder();
	let results = decoder.decode(&img);
	println!("{}", results.len());
	for result in results {
		match result {
			Ok(contents) => println!("{}", contents),
			Err(e) => println!("failed with error: `{}`", e),
		}
	}

	Ok(())
}

#[warn(unused_imports)]
extern crate image;

use std::fs::File;
use std::path::Path;
use std::env;
use image::GenericImage;
mod feature;

fn main() {
	let args: Vec<_> = env::args().collect();
	if args.len() > 1 {
		let mut img = image::open(&Path::new(&args[1])).unwrap();


        feature::harris_corner(&mut img, 3);

		let mut fout = File::create(&Path::new("out.png")).unwrap();
		let _ = img.save(&mut fout, image::PNG);
	}
}

#[warn(unused_imports)]
extern crate image;


use std::fs::File;
use std::path::Path;
use std::env;
//use image::GenericImage;


use image :: {
	GenericImage,
	ImageBuffer,
	imageops
};

fn main() {

	let args: Vec<_> = env::args().collect();
	if args.len() > 1 {
		let mut img = image::open(&Path::new(&args[1])).unwrap();

		//get row  and col
		let (row, col) = img.dimensions();		
//		println!("(row, col) = ({:?}, {:?})", row, col);
		
		let subimg = imageops::crop(img, 0, 0, 100, 100);


//		let ref mut fout = File::create(&Path::new("out.png")).unwrap();
//		let _ = img.save(fout, image::PNG);

	}
}

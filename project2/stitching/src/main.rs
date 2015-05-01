#[warn(unused_imports)]
extern crate image;

use std::fs::File;
use std::path::Path;
use std::env;
use std::str::FromStr;
mod feature;
mod cylindrical_wrap;

fn main() {
	let args: Vec<_> = env::args().collect();
	if args.len() == 4 {
		let mut img = image::open(&Path::new(&args[1])).unwrap();
        let ref output_name = args[2];
        let threshold = f64::from_str(&args[3]).unwrap();
        println!("threshold is {}", threshold);
        let mut new_img = cylindrical_wrap::transform(&img, 1500.0);

        feature::harris_corner(&mut new_img, 5, threshold);
        

		let mut fout = File::create(&Path::new(output_name)).unwrap();
		let _ = new_img.save(&mut fout, image::PNG);
	}
    else {
        println!("Usage: stitching <input> <output> <threshold>");
    }
}

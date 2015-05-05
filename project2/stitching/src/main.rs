extern crate image;
use std::fs::File;
use std::path::Path;
use std::env;
use std::str::FromStr;
mod feature;
mod cylindrical_wrap;
mod util;

fn main() {
	let args: Vec<_> = env::args().collect();
	if args.len() == 5 {
		let img = image::open(&Path::new(&args[4])).unwrap();
        let ref output_name = args[3];
        let threshold = f64::from_str(&args[1]).unwrap();
        println!("threshold is {}", threshold);
        let focal = f64::from_str(&args[2]).unwrap();
		// by test, the input focal length needs 5 times of real focal length
		let mut new_img = cylindrical_wrap::transform(&img, focal * 5.0);
        let points = feature::harris_corner(&mut new_img, threshold);
        util::draw_red_point(&mut new_img, &points);

		let mut fout = File::create(&Path::new(output_name)).unwrap();
		let _ = new_img.save(&mut fout, image::PNG);
	}
    else {
        println!("Usage: stitching <threshold> <focal> <output> <input1> <input2> ...");
    }
}

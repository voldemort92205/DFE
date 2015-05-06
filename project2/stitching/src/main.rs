extern crate image;
use image::DynamicImage;
use std::fs::File;
use std::path::Path;
use std::env;
use std::str::FromStr;
mod feature;
mod cylindrical_wrap;
mod util;
mod def;
use def::{Feature, ImgWithFeature};

fn main() {
	let args: Vec<_> = env::args().collect();
	if args.len() >= 5 {
        // 解析 args
        let threshold = f64::from_str(&args[1]).unwrap();
        let focal = f64::from_str(&args[2]).unwrap();
        let ref output_name = args[3];
        let mut imgs: Vec<DynamicImage> = Vec::new();
        let mut img_features = Vec::new();
        for i in 4..args.len() {
            let mut img = image::open(&Path::new(&args[i])).unwrap();
            let points = feature::harris_corner(&img, threshold);
            let mut img_with_feature = ImgWithFeature::new(img, points);
            img_features.push(img_with_feature);
        }

		// by test, the input focal length needs 5 times of real focal length
		// let mut new_img = cylindrical_wrap::transform(&img, focal * 5.0);
        // util::draw_red_point(&mut new_img, &points);

		// let mut fout = File::create(&Path::new(output_name)).unwrap();
		// let _ = new_img.save(&mut fout, image::PNG);
	}
    else {
        println!("Usage: stitching <threshold> <focal> <output> <input1> <input2> ...");
    }
}

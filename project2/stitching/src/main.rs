extern crate image;
// use image::DynamicImage;
// use std::fs::File;
use std::path::Path;
use std::env;
use std::str::FromStr;
mod feature;
mod cylindrical_wrap;
mod util;
mod def;
mod matchf;
use def::{ImgWithFeature};

fn main() {
	let args: Vec<_> = env::args().collect();
	if args.len() >= 5 {
        // 解析 args
        let threshold = f64::from_str(&args[1]).unwrap();
        let focal = f64::from_str(&args[2]).unwrap();
        let ref output_name = args[3];
        let mut img_fs = Vec::new();
        for i in 4..args.len() {
            let img = image::open(&Path::new(&args[i])).unwrap();
            let mut new_img = cylindrical_wrap::transform(&img, focal * 5.0); // 懷疑是否需要transform
            println!("count harris_corner, {}", args[i]);
            let points = feature::harris_corner(&new_img, threshold);
            let mut img_with_feature = ImgWithFeature::new(new_img, points);

            // let mut fout = File::create(&Path::new(output_name)).unwrap();
            // let _ = new_img.save(&mut fout, image::PNG);

            matchf::count_descriptor(&mut img_with_feature);
            if i != 4 {
                matchf::match_feature(&mut img_fs[i - 5], &mut img_with_feature);
                matchf::ransac(&img_fs[i - 5]);
            }
            img_fs.push(img_with_feature);
        }
	}
    else {
        println!("Usage: stitching <threshold> <focal> <output> <input1> <input2> ...");
    }
}

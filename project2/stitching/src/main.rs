extern crate image;
use image::{DynamicImage, GenericImage};
use std::fs::File;
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
        let output_name = &args[3];
        let mut img_fs = Vec::new();
        let mut distances = Vec::new();
        for i in 4..args.len() {
            let img = image::open(&Path::new(&args[i])).unwrap();
            let new_img = cylindrical_wrap::transform(&img, focal * 5.0); // 懷疑是否需要transform
            println!("count harris_corner, {}", args[i]);
            let points = feature::harris_corner(&new_img, threshold);
            let mut img_with_feature = ImgWithFeature::new(new_img, points);
            matchf::count_descriptor(&mut img_with_feature);
            if i != 4 {
                matchf::match_feature(&mut img_fs[i - 5], &mut img_with_feature);
                distances.push(matchf::ransac(&img_fs[i - 5]));
            }
            img_fs.push(img_with_feature);
        }
        println!("distances {:?}", distances);

        let mut result = DynamicImage::new_rgb8(img_fs[0].img.width() * (args.len() - 4) as u32, img_fs[0].img.height() * 2);
        let mut basex = (result.width() - 50 - img_fs[0].img.width()) as i32;
        let mut basey = (result.height() * 1 / 4) as i32;

        for i in 0..img_fs.len() {
            result.copy_from(&img_fs[i].img, basex as u32, basey as u32);
            if i != img_fs.len() - 1 {
                match distances[i] {
                    (x, y) => {
                        basex -= x;
                        basey -= y;
                    }
                }
            }
        }

        let mut fout = File::create(&Path::new(output_name)).unwrap();
        let _ = result.save(&mut fout, image::PNG);
	}
    else {
        println!("Usage: stitching <threshold> <focal> <output> <input1> <input2> ...");
    }
}

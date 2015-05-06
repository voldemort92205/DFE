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

        // TODO: 沒空了，不然要再寫個blending函式
        let width = img_fs[0].img.width();
        let height = img_fs[0].img.height();
        let mut result = DynamicImage::new_rgb8(width * (args.len() - 4) as u32, height / 2 * 3);
        let mut basex = (result.width() - 50 - width) as i32;
        let mut basey = (result.height() * 1 / 6) as i32;
        let (mut last_centerx, mut last_centery) = (0, 0);
        fn dis((x1, y1): (u32, u32), (x2, y2): (u32, u32)) -> f64 {
            (((x1-x2).pow(2) + (y1-y2).pow(2)) as f64).sqrt()
        }
        for i in 0..img_fs.len() {
            let (centerx, centery) = (basex as u32 + width / 2, basey as u32 + height / 2);
            for col in 0..img_fs[i].img.width() {
                for row in 0..img_fs[i].img.height() {
                    let mut pix = img_fs[i].img.get_pixel(col, row);
                    let opix = result.get_pixel(basex as u32 + col, basey as u32 + row);
                    let tx = basex as u32 + col;
                    let ty =  basey as u32 + row;
                    let target = (tx, ty);
                    if i > 0 && tx >= last_centerx - width / 2 && ty > last_centery - height / 2 && ty < last_centery + height / 2 {
                        let oritio = dis((centerx, centery), target);
                        let ritio = dis((last_centerx, last_centery), target);
                        // let oritio = width as f64 / 2.0 - dis((last_centerx, last_centery), target) + 20.0;
                        // let ritio = width as f64 / 2.0 - dis((centerx, centery), target) + 20.0;
                        let mother = oritio + ritio;
                        let r = ((ritio * pix.data[0] as f64 + oritio * opix.data[0] as f64) / mother) as u8;
                        let g = ((ritio * pix.data[1] as f64 + oritio * opix.data[1] as f64) / mother) as u8;
                        let b = ((ritio * pix.data[2] as f64 + oritio * opix.data[2] as f64) / mother) as u8;
                        let mixed = [r, g, b , 255];
                        pix.data = mixed;
                    }
                    result.put_pixel(basex as u32 + col, basey as u32 + row, pix);
                }
            }

            if i != img_fs.len() - 1 {
                last_centerx = centerx;
                last_centery = centery;
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

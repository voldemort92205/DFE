extern crate image;
extern crate rand;

use image::{GenericImage, DynamicImage, Pixel};
use def::{ImgWithFeature, Feature};
use util::{rgb_luma};

// 判斷是屬於八方位的哪一個位置
// 方位由上開始，順時針由零排到七
fn classify(gradient: (f64, f64)) -> usize {
    fn dot(a: (f64, f64), b: (f64, f64)) -> i32 {
        let (ax, ay) = a;
        let (bx, by) = b;
        (ax * bx + ay * by) as i32
    }
    let orientation: [(f64, f64); 8] = [(0.0, 2f64.sqrt()), (1.0, 1.0), (2f64.sqrt(), 0.0), (1.0, -1.0),
                                        (0.0, -2f64.sqrt()), (-1.0, -1.0), (-2f64.sqrt(), 0.0), (-1.0, 1.0)];
    let mut ans = 0;
    let mut max = 0;
    for i in (0..8) {
        if max < dot(orientation[i], gradient) {
            max = dot(orientation[i], gradient);
            ans = i;
        }
    }
    ans
}

#[test]
fn test_classify() {
    println!("{}", classify((1f64, 1f64)));
    println!("{}", classify((3f64, -4f64)));
    println!("{}", classify((-1f64, 2f64)));
    assert!(classify((1f64, 1f64)) == 1);
    assert!(classify((3f64, -4f64)) == 3);
    assert!(classify((-1f64, 2f64)) == 7);
}

// TODO: make it private
pub fn count_descriptor(img_f: &mut ImgWithFeature) {
    fn gradient(img: &DynamicImage, col: u32, row: u32) -> (f64, f64) {
        (rgb_luma(img.get_pixel(col + 1, row).to_rgb()) as f64 - rgb_luma(img.get_pixel(col - 1, row).to_rgb()) as f64,
        rgb_luma(img.get_pixel(col, row + 1).to_rgb()) as f64 - rgb_luma(img.get_pixel(col, row - 1).to_rgb()) as f64)
    };
    fn a_descriptor(img: &DynamicImage, feature: &mut Feature) {
        fn quadrant(x: i32, y: i32) -> usize {
            if x >= 0 && y >= 0 {
                0
            } else if x >= 0 && y < 0 {
                1
            } else if x < 0 && y < 0 {
                2
            } else {
                3
            }
        }
        let (col, row) = (feature.x, feature.y);
        for i in -4..4 {
            for j in -4..4 {
                // TODO: gaussian blur
                let (ncol, nrow) = ((col as i32 + i) as u32, (row as i32 + j) as u32);
                let g = classify(gradient(img, ncol, nrow));
                let q = quadrant(i, j);
                // let ii = if i >= 0 {i+ 1} else {i};
                // let jj = if j >= 0 {j+ 1} else {j};
                // let gg = ((ii*ii) as f64 + (jj*jj) as f64 / -0.5).exp();
                feature.descriptor[q][g] += 1.0;
            }
        }
    }
    for i in 0..img_f.features.len() {
        a_descriptor(&img_f.img, &mut img_f.features[i]);
    }
}

pub fn match_feature(img_f: &mut ImgWithFeature, img_f2: &mut ImgWithFeature) {
    fn dis(descriptor1: [[f64; 8]; 4], descriptor2: [[f64; 8]; 4]) -> f64 {
        let mut ans = 0.0;
        for i in 0..4 {
            for j in 0..8 {
                ans += (descriptor1[i][j] as i32 - descriptor2[i][j] as i32).pow(2) as f64;
            }
        }
        ans.sqrt()
    }
    // 計算最大跟次大求比例
    // let mut out = 0;
    for i in 0..img_f.features.len() {
        let mut min = 9999999999.9;
        let mut smallest = (0, 0);
        for j in 0..img_f2.features.len() {
            let distance = dis(img_f.features[i].descriptor, img_f2.features[j].descriptor);
            if min > distance {
                min = distance;
                smallest = (img_f2.features[j].x, img_f2.features[j].y);
            }
        }
        let mut sec_min = 9999999999.9;
        for j in 0..img_f2.features.len() {
            if (img_f2.features[j].x, img_f2.features[j].y) != smallest && 
                sec_min > dis(img_f.features[i].descriptor, img_f2.features[j].descriptor)
            {
                sec_min = dis(img_f.features[i].descriptor, img_f2.features[j].descriptor);
            }
        }
        // println!("feature {}, min: {}, sec_min:{}", i, min, sec_min);
        if  min / sec_min < 0.85 {
            img_f.features[i].match_to = smallest;
        } else {
            img_f.features[i].can_match = false;
            // out += 1;
        }
    }
    // println!("out: {}", out);
    img_f.features = img_f.features.iter().filter(|f| f.can_match).map(|f| *f).collect::<Vec<Feature>>();
    // for i in 0..img_f.features.len() {
    //     if img_f.features[i].can_match {
    //         println!("({}, {}) match to {:?}", img_f.features[i].x, img_f.features[i].y, img_f.features[i].match_to);
    //     }
    // }
}

pub fn ransac(img_f: &ImgWithFeature) -> (i32, i32) {
    fn rand_point (img_f: &ImgWithFeature) -> Vec<usize> {
        let mut node = Vec::new();
        for _ in 0..3 {
            let r = rand::random::<usize>() % img_f.features.len();
            node.push(r);
        }
        node
    }
    fn dis((x1, y1): (f64, f64), (x2, y2): (f64, f64)) -> f64 {
        ((x1-x2).powf(2.0) + (y1-y2).powf(2.0)).sqrt()
    }
    let threshold = 20.0;
    let mut best = 0;
    let mut ans = (0, 0);
    for _ in (0..1000) {
        let mut sumx = 0.0; let mut sumy = 0.0;
        let rand_p = rand_point(&img_f);
        for i in rand_p {
            match img_f.features[i].match_to {
                (x, y) => {
                    sumx += x as f64 - img_f.features[i].x as f64; 
                    sumy += y as f64 - img_f.features[i].y as f64;
                }
            }
        }
        sumx /= 3.0; sumy /= 3.0;
        let mut count = 0;
        for feature in &img_f.features {
            match feature.match_to {
                (x, y) => {
                    if dis((feature.x as f64 + sumx, feature.y as f64 + sumy), (x as f64, y as f64)) < threshold {
                        count += 1;
                    }
                }
            }
        }
        if count > best {
            best = count;
            ans = (sumx as i32, sumy as i32);
        }
    }
    println!("best move is {:?}, {} in threshold", ans, best);
    ans
}

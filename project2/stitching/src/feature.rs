extern crate image;
use image::{GenericImage, DynamicImage, Pixel, Rgb};
use std::marker::Copy;

// K 是 det(M) - K * trace(M)平方  中的K，經驗值為0.04到0.06
const K: f64 = 0.14;

fn new_2d_vector<T: Copy>(init: T, x: u32, y: u32) -> Vec<Vec<T>> {
    let mut _2d_vec = Vec::new();
    for _ in 1..x {
        let mut _1d_vec = Vec::new();
        for _ in 1..y {
            _1d_vec.push(init);
        }
        _2d_vec.push(_1d_vec);
    }
    _2d_vec
}

// windows 限制到ˋ3*3, 5*5, 7*7
pub fn harris_corner(img: &mut DynamicImage, window_size: i32, threshold: f64) {
    let gray_img = make_gray(img);
    let window = gaussian(window_size);
    let border = window_size / 2;
    let mut harris_value = new_2d_vector::<f64>(0.0 as f64, img.width(), img.height());

    // 所以相片不能比window還小
    for col in (border + 1)..(img.width() as i32 - 2 - border) {
        for row in(border + 1)..(img.height() as i32 - 2 - border) {
            // 計算每點的u, v
            let mut a: f64 = 0.0;
            let mut b: f64 = 0.0;
            let mut c: f64 = 0.0;
            for i in (-border)..(border + 1) {
                for j in (-border)..(border + 1) {
                    let ix = gray_img[(col + i + 1) as usize][(row + j) as usize] - gray_img[(col + i - 1) as usize][(row + j) as usize];
                    let iy = gray_img[(col + i) as usize][(row + j + 1) as usize] - gray_img[(col + i) as usize][(row + j - 1) as usize];
                    a += window[(i + border) as usize][(j + border) as usize] * ix.pow(2) as f64;
                    b += window[(i + border) as usize][(j + border) as usize] * iy.pow(2) as f64;
                    c += window[(i + border) as usize][(j + border) as usize] * ix as f64 * iy as f64;
                }
            }
            harris_value[col as usize][row as usize] = estimate_f(a, b, c);
        }
    }
    let mut max: f64 = 0.0 as f64;
    // TODO: make col as i32
    for col in 1..(img.width() - 2) {
        for row in 1..(img.height() - 2) {
            let value = harris_value[col as usize][row as usize];
            if value > threshold {
                // local maximal
                let mut ok = true;
                for i in [-1, 0, 1].iter() {
                    for j in [-1, 0, 1].iter() {
                        if (*i != 0 || *j != 0) && harris_value[(col as i32 + *i) as usize][(row as i32+ *j) as usize] >= value {
                            ok = false;
                        }
                    }
                }
                if ok {
                    for i in [-2, -1, 0, 1, 2].iter() {
                        for j in [-2, -1, 0, 1, 2].iter() {
                            img.put_pixel((col as i32 + *i) as u32, (row as i32 + *j) as u32, image::Rgba {data: [255, 0, 0, 255]});
                        }
                    }
                    println!("harris_value: {}, in ({}, {})", value, col, row);
                }
            }
            if max < value {max = value;}
        }
    }
}

fn estimate_f(a: f64, b: f64, c: f64) -> f64 {
    let det = a*b - c*c;
    let trace = a + b;
    det - K * trace * trace
}

fn make_gray(img: &DynamicImage) -> Vec<Vec<i32>> {
    fn rgb_luma(rgb: Rgb<u8>) -> i32 {
        rgb.data[0] as i32 + rgb.data[1] as i32 + rgb.data[2] as i32
    }

    let mut gray_img = Vec::new();
    for col in 0..(img.width() - 1) {
        let mut a_col = Vec::new();
        for row in 0..(img.height() - 1) {
            a_col.push(rgb_luma(img.get_pixel(col, row).to_rgb()));
        }
        gray_img.push(a_col);
    }
    gray_img
}

fn gaussian(s: i32) -> Vec<Vec<f64>> {
    fn gaussian_f(x: i32, y: i32) -> f64 {
        (((x * x) as f64 + (y * y) as f64) / -0.5).exp()
    }
    let mut window = Vec::new();
    let mid = s / 2;
    for i in 0..s {
        let mut tmp = Vec::new();
        for j in 0..s {
            tmp.push(gaussian_f(i - mid, j - mid));
        }
        window.push(tmp);
    }
    window
}

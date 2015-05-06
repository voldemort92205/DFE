extern crate image;
use image::{GenericImage, DynamicImage, Pixel};
use util::{new_2d_vector, rgb_luma};
use def;

// K 是 det(M) - K * trace(M)平方  中的K，經驗值為0.04到0.06
const K: f64 = 0.2;
const WINDOW_SIZE: i32 = 5;

// windows 限制到ˋ3*3, 5*5, 7*7
pub fn harris_corner(img: &DynamicImage, threshold: f64) -> Vec<def::Feature> {
    let gray_img = make_gray(img);
    let window = gaussian(WINDOW_SIZE);
    let border = WINDOW_SIZE / 2;
    let mut harris_value = new_2d_vector::<f64>(0.0 as f64, img.width(), img.height());

    // 所以相片不能比window還小
    // +4 是為了之後的descriptor
    for col in (border + 1 + 4)..(img.width() as i32 - 2 - border - 4) {
        for row in(border + 1 + 4)..(img.height() as i32 - 2 - border - 4) {
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
    let mut ans = Vec::new();
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
                if ok && col + 5 < img.width() - 5 && col as i32 - 5 >= 0 &&
                        row + 5 < img.height() && row as i32 - 5 >= 0 {
                    // println!("harris_value: {}, in ({}, {})", value, col, row);
                    ans.push(def::Feature::new(col, row));
                }
            }
            if max < value {max = value;}
        }
    }
    ans
}

fn estimate_f(a: f64, b: f64, c: f64) -> f64 {
    let det = a*b - c*c;
    let trace = a + b;
    det - K * trace * trace
}

fn make_gray(img: &DynamicImage) -> Vec<Vec<i32>> {
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

extern crate image;
use image::{GenericImage, DynamicImage, Pixel, Rgb};

// K 是 det(M) - K * trace(M)平方  中的K，經驗值為0.04到0.06
const K: f32 = 0.05;

// windows 限制到ˋ3*3, 5*5, 7*7
pub fn harris_corner(img: &mut DynamicImage, window_size: u8) {
    make_gray(img);
}

fn make_gray(img: &DynamicImage) -> Vec<Vec<u32>> {
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

fn rgb_luma(rgb: Rgb<u8>) -> u32 {
    rgb.data[0] as u32 + rgb.data[1] as u32 + rgb.data[2] as u32
}

extern crate image;
use image::{GenericImage, DynamicImage, Rgba};

pub fn transform(img: &DynamicImage, f: f64) -> DynamicImage {
    let mut target = DynamicImage::new_rgba8(img.width(), img.height());
    // TODO: 從圖片中心來計算
    for col in 0..img.width() {
        for row in 0..img.height() {
            let x_shift = (img.width() / 2) as f64;
            let y_shift = (img.height() / 2) as f64;
            let x = col as f64 - x_shift;
            let y = row as f64 - y_shift;
            let x_p = ((f * (x / f).tan()) + x_shift) as u32;
            let y_p = (f * (y / f) / (x / f).cos() + y_shift) as u32;
            if x_p < img.width() && y_p < img.height()
                && x_p >= 0 && y_p >= 0
            {
                target.put_pixel(col, row, img.get_pixel(x_p, y_p));
            }
            else {
                target.put_pixel(col, row, Rgba { data: [0, 0, 0, 255] });
            }
        }
    }
    target
}

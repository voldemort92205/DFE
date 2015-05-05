extern crate image;
use image::{GenericImage, DynamicImage};

pub fn draw_red_point(img: &mut DynamicImage, points: &Vec<(u32, u32)>) {
    for point in points {
        match *point {
            (col, row) => {
                for i in [-2, -1, 0, 1, 2].iter() {
                    for j in [-2, -1, 0, 1, 2].iter() {
                        img.put_pixel((col as i32 + *i) as u32, (row as i32 + *j) as u32, image::Rgba {data: [255, 0, 0, 255]});
                    }
                }
            }
        }
    }
}

pub fn new_2d_vector<T: Copy>(init: T, x: u32, y: u32) -> Vec<Vec<T>> {
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

extern crate image;
use image::DynamicImage;

#[derive(Copy, Clone)]
pub struct Feature {
    pub x: u32,
    pub y: u32,
    pub descriptor: [[f64; 8]; 4],
    pub can_match: bool,
    pub match_to: (u32, u32),    // 皆由左對應右
}

impl Feature {
    pub fn new(_x: u32, _y: u32) -> Feature {
        Feature { x: _x, y: _y, can_match: true, descriptor: [[0.0; 8]; 4], match_to: (0, 0) }
    }
}

pub struct ImgWithFeature {
    pub img: DynamicImage,
    pub features: Vec<Feature>,
}

impl ImgWithFeature {
    pub fn new(img: DynamicImage, features: Vec<Feature>) -> ImgWithFeature {
        ImgWithFeature { img: img, features: features }
    }
}

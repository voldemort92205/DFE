extern crate image;
use image::DynamicImage;

pub struct Feature {
    pub x: u32,
    pub y: u32,
    pub descriptor: [[u32; 8]; 4],
    pub match_to: (u32, u32),    // 皆由左對應右
}

impl Feature {
    pub fn new(_x: u32, _y: u32) -> Feature {
        Feature { x: _x, y: _y, descriptor: [[0u32; 8]; 4], match_to: (0, 0) }
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

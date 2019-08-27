extern crate image;
use image::{RgbImage};
use image::imageops::{blur, rotate90};

pub trait ImageOperation {
    fn transform(&self, im:&RgbImage) -> RgbImage;
}

pub struct BlurTransformer{
    pub sigma:f32,
}

impl ImageOperation for BlurTransformer{
    fn transform(&self, im:&RgbImage) -> RgbImage{
        return blur(im, self.sigma);
    }
}

pub struct Rot90Transformer{
}

impl ImageOperation for Rot90Transformer{
    fn transform(&self, im:&RgbImage) -> RgbImage{
        return rotate90(im);
    }
}



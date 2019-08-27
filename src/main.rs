extern crate image;
mod transformers;
use image::{open, RgbImage};
use std::env;
pub use transformers::*;

struct Pipeline<'a>{
    tfs: Vec<&'a dyn ImageOperation>,
}

trait ImagePipeline{
    fn execute<'a>(&self, im:&'a RgbImage) -> RgbImage;
}

impl ImagePipeline for Pipeline<'_> {
    fn execute<'a>(&self, im:&'a RgbImage) -> RgbImage {
        let mut result = im.clone();
        for idx in 0..self.tfs.len() {
            result = self.tfs[idx].transform(&result);
        }
        return result
    }
}

trait PathPipeline{
    fn execute<'a>(&self, im:&'a String) -> RgbImage;
}

impl PathPipeline for Pipeline<'_> {
    fn execute<'a>(&self, im:&'a String) -> RgbImage {
        // TODO handle panic.
        let mut result = open(im).unwrap().to_rgb();
        for idx in 0..self.tfs.len() {
            result = self.tfs[idx].transform(&result);
        }
        return result
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let im : &RgbImage = &open(path).unwrap().to_rgb();
    let pipeline = Pipeline{
        tfs: vec![&BlurTransformer{sigma: 1.0},
                  &Rot90Transformer{}]
    };
    let mut out = ImagePipeline::execute(&pipeline, im);
    out.save("p.png").unwrap();
    println!("Hello, world!");
    out = PathPipeline::execute(&pipeline, path);
    out.save("q.png").unwrap();
    println!("Hello, world!");

}

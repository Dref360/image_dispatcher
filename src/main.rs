extern crate image;
extern crate rayon;
mod transformers;
use image::{open, RgbImage};
use std::env;
pub use transformers::*;
use rayon::prelude::*;
use std::fs;

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
    let folder = &args[1];
    let files : Vec<_> = fs::read_dir(folder).unwrap().map(|r| r.unwrap().path().into_os_string().into_string().unwrap())
                                              .collect();
    let pipeline = Pipeline{
        tfs: vec![&BlurTransformer{sigma: 1.0},
                  &Rot90Transformer{}]
    };
    let _outs : Vec<_> = files.into_par_iter().map(|p| PathPipeline::execute(&pipeline, &p)).collect();
    
    println!("Hello, world!");

}

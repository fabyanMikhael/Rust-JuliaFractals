#![allow(non_snake_case)]
use std::time::Instant;

use rayon::{iter::{IndexedParallelIterator, ParallelIterator}, slice::ParallelSliceMut};

fn main() {
    let mut output = "fractal.png".to_string();
    let mut imgx = 800;
    let mut imgy = 800;


    if let Some(arg) = std::env::args().skip(1).next(){
        output = arg;
    }
    if let Some(arg) = std::env::args().skip(2).next(){
        imgx = arg.parse::<u32>().unwrap();
    } 
    if let Some(arg) = std::env::args().skip(3).next(){
        imgy = arg.parse::<u32>().unwrap();
    } 

    let scalex = 3.0 / imgx as f32;
    let scaley = 3.0 / imgy as f32;

    let now = Instant::now();

    //creating empty buffer
    let mut imgbuf = image::RgbImage::new(imgx, imgy);

    // splitting the buffer into chunks of 3 (1 pixel because of RGB)
    (*imgbuf).par_chunks_mut(3).enumerate().for_each(|(idx,pixel)| {
        //doing the math....
        let x = idx % imgx as usize;
        let y = idx / imgx as usize;
        let r = (0.3 * x as f32) as u8;
        let b = (0.3 * y as f32) as u8;
        let cx = y as f32 * scalex - 1.5;
        let cy = x as f32 * scaley - 1.5;

        let c = num_complex::Complex::new(-0.4, 0.6);
        let mut z = num_complex::Complex::new(cx, cy);

        let mut i = 0;
        while i < 255 && z.norm() <= 2.0 {
            z = z * z + c;
            i += 1;
        }
        //setting the corresponding colors of the pixels on the chunked [u8;3]
        pixel[0] = r;
        pixel[1] = i as u8;
        pixel[2] = b;
    });

    imgbuf.save(output).unwrap();
    println!("done. ({:?})", now.elapsed());
}

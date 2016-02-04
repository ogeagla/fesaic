
extern crate num;
extern crate image;

use std::fs::File;
use std::path::Path;

use num::complex::Complex;
use image::{
    GenericImage,
    ImageBuffer,
    imageops
};
use image::ColorType;
// use image::buffer::{
//     ImageBuffer,
//     RgbaImage,
//     Pixel
// };
// use image::imageops;
// use image::{GenericImage, SubImage};
// use image::dynimage::DynamicImage;
//
// fn avg_rgb(buf: DynamicImage) -> Rgba<u8> {
//
// /*
// impl<P, Container> ImageBuffer<P, Container>
// where P: Pixel + 'static,
//       P::Subpixel: 'static,
//       Container: Deref<Target=[P::Subpixel]> {
// */
//
//     let mut accR = 0.0 as f32;
//     let mut accG = 0.0 as f32;
//     let mut accB = 0.0 as f32;
//     let mut accA = 0.0 as f32;
//     let mut counter = 0 as u32;
//
//     for (_, _, pixel) in buf.to_rgba().enumerate_pixels() {
//         // here accumulate the values of ARGB pixel vector
//         counter = counter + 1;
//     }
//
//     //convert counter to float first i think
//     accR = accR / counter as f32;
//     accG = accG / counter as f32;
//     accB = accB / counter as f32;
//     accA = accA / counter as f32;
//
//     //create Rgba instance, return it
// }
//
// fn crop_as_grid<I: GenericImage + 'static>(
//     buf: &mut I,
//     // buf: RgbaImage,
//     row: u32,
//     col: u32,
//     rows: u32,
//     cols: u32) -> SubImage<I> where I::Pixel: 'static,
//           <I::Pixel as Pixel>::Subpixel: 'static  {
//
//     let (dx, dy) = buf.dimensions();
//     let col_size = dx / cols;
//     let row_size = dy / rows;
//
//     //this shit is mostly wrong, but im doing this offline on a plane,
//     //need tests to validate...also check that it compiles
//     let start_w = row_size * row;
//     let end_w = start_w + 1;
//
//     let start_c = col_size * col;
//     let end_c = start_c + 1;
//
//     imageops::crop(buf, start_w, start_c, end_w, end_c);
// }

fn main() {

    //in reality we want to compute these from the loaded image
    let img_w = 500;
    let img_h = 500;

    let rows = 10;
    let cols = 10;

    let ref mut imgbuf: image::RgbaImage = image::ImageBuffer::new(img_w as u32, img_h as u32);

    for r in 0..rows {
        for c in 0..cols {

            let (dx, dy) = imgbuf.dimensions();
            let col_size = dx / cols;
            let row_size = dy / rows;

            //this shit is mostly wrong, but im doing this offline on a plane,
            //need tests to validate...also check that it compiles
            let start_w = row_size * r;
            let end_w = start_w + 1;

            let start_c = col_size * c;
            let end_c = start_c + 1;

            let subimg = imageops::crop(imgbuf, start_w, start_c, end_w, end_c);

            let mut accR = 0.0 as f32;
            let mut accG = 0.0 as f32;
            let mut accB = 0.0 as f32;
            let mut accA = 0.0 as f32;
            let mut counter = 0 as u32;

            for (_, _, pixel) in subimg.to_image().enumerate_pixels() {
              // here accumulate the values of ARGB pixel vector
              counter = counter + 1;
            }

            //convert counter to float first i think
            accR = accR / counter as f32;
            accG = accG / counter as f32;
            accB = accB / counter as f32;
            accA = accA / counter as f32;

            //create Rgba Pixel instance
            let pix = image::Rgba([accR, accG, accB, accA]);


            // let subimg = crop_as_grid(target_img_buf, r, c, rows, cols);
            // let avg_argb_of_sub = avg_rgb(DynamicImage::ImageRgba8(subimg.to_image()));
            //
            // //we now have avg argb of each SubImage
            // //TODO
            // /*
            //     - for a list of images, compute avg argb of each img
            //     - match the subimage argb avg with the closest avg argb of the images in the set
            // */
        }
    }


}

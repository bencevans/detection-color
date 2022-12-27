use clap::Parser;
use indicatif::ParallelProgressIterator;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use std::path::PathBuf;
mod coco;
use coco::{Coco, CocoIndex};

/// Calculate the mean pixels of 1px inset and outset boxes around each object in a COCO dataset
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the image_dir
    image_dir: PathBuf,

    /// Path to COCO annotations
    coco_path: PathBuf,
}

fn main() {
    let args = Args::parse();

    let mut coco = Coco::new();
    coco.load(args.coco_path.to_str().unwrap());

    let coco_index = CocoIndex::new(&coco);

    println!("Images: {}", coco_index.images.len());
    println!("Annotations: {}", coco_index.annotations.len());
    println!("Categories: {}", coco_index.categories.len());

    for (category_id, category) in &coco_index.categories {
        println!("Category: {} - {}", category_id, category.name);
    }

    let object_means = coco_index
        .annotations
        .par_iter()
        .progress_count(coco_index.annotations.len() as u64)
        .map(|(_annotation_id, annotation)| {
            let image_meta = coco_index.images.get(&annotation.image_id).unwrap();
            let image_path = args.image_dir.join(&image_meta.file_name);

            let imagebuf = image::open(image_path).unwrap();
            let imagebuf = imagebuf.to_rgb8();

            let x = annotation.bbox[0] as u32;
            let y = annotation.bbox[1] as u32;
            let w = annotation.bbox[2] as u32;
            let h = annotation.bbox[3] as u32;

            // Inset Box
            let inset = 1;
            let xs = x + inset;
            let ys = y + inset;
            let ws = w - inset * 2;
            let hs = h - inset * 2;
            let mut s_pixels = vec![];

            for i in xs..xs + ws {
                s_pixels.push(*imagebuf.get_pixel(i, ys));
                s_pixels.push(*imagebuf.get_pixel(i, ys + hs));
            }

            for j in ys..ys + hs {
                s_pixels.push(*imagebuf.get_pixel(xs, j));
                s_pixels.push(*imagebuf.get_pixel(xs + ws, j));
            }

            // Outset Box
            let outset = 1;
            let xl = x - outset;
            let yl = y - outset;
            let wl = w + outset * 2;
            let hl = h + outset * 2;
            let mut l_pixels = vec![];

            fn get_pixel(imagebuf: &image::RgbImage, x: u32, y: u32) -> Option<image::Rgb<u8>> {
                if x < imagebuf.width() && y < imagebuf.height() {
                    Some(*imagebuf.get_pixel(x, y))
                } else {
                    None
                }
            }

            for i in xl..xl + wl {
                if let Some(pixel) = get_pixel(&imagebuf, i, yl) {
                    l_pixels.push(pixel);
                }

                if let Some(pixel) = get_pixel(&imagebuf, i, yl + hl) {
                    l_pixels.push(pixel);
                }
            }

            for j in yl..yl + hl {
                if let Some(pixel) = get_pixel(&imagebuf, xl, j) {
                    l_pixels.push(pixel);
                }

                if let Some(pixel) = get_pixel(&imagebuf, xl + wl, j) {
                    l_pixels.push(pixel);
                }
            }

            let s_pixels = [s_pixels, l_pixels].concat();

            // println!("Sample pixels: {:?}", s_pixels);

            // Calculate the mean
            let mut r_sum = 0;
            let mut g_sum = 0;
            let mut b_sum = 0;
            for pixel in &s_pixels {
                r_sum += pixel[0] as u32;
                g_sum += pixel[1] as u32;
                b_sum += pixel[2] as u32;
            }
            let r_mean = r_sum / s_pixels.len() as u32;
            let g_mean = g_sum / s_pixels.len() as u32;
            let b_mean = b_sum / s_pixels.len() as u32;

            // println!("Mean: {} {} {}", r_mean, g_mean, b_mean);

            // imagebuf.save("test.png").unwrap();

            [r_mean, g_mean, b_mean]
        });

    let sum: [u32; 3] =
        object_means.reduce(|| [0, 0, 0], |a, b| [a[0] + b[0], a[1] + b[1], a[2] + b[2]]);

    let mean = [
        sum[0] / coco_index.annotations.len() as u32,
        sum[1] / coco_index.annotations.len() as u32,
        sum[2] / coco_index.annotations.len() as u32,
    ];

    println!("Mean: rgb({}, {}, {})", mean[0], mean[1], mean[2]);
    println!("Mean: #{:02x}{:02x}{:02x}", mean[0], mean[1], mean[2]);
}

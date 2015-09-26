extern crate image;

use std::fs::File;
use std::env;

use image::{GenericImage, Rgba};

const ROWS: u32 = 48;
const COLS: u32 = 32;

#[derive(Clone, Copy, Debug)]
pub struct Rect {
    pub left: u32,
    pub top: u32,
    pub right: u32,
    pub bottom: u32
}

impl Rect {
    pub fn new(left: u32, top: u32, right: u32, bottom: u32) -> Rect {
        Rect { left: left, top: top, right: right, bottom: bottom }
    }

    pub fn width(&self) -> u32 {
        self.right - self.left
    }

    pub fn height(&self) -> u32 {
        self.bottom - self.top
    }
}

// Euclidean distance between two RGB colors.
fn distance(a: Rgba<u8>, b: Rgba<u8>) -> f64 {
    let (r1, g1, b1) = (a[0] as f64, a[1] as f64, a[2] as f64);
    let (r2, g2, b2) = (b[0] as f64, b[1] as f64, b[2] as f64);
    ((r2 - r1).powi(2) + (g2 - g1).powi(2) + (b2 - b1).powi(2)).sqrt()
}

fn is_similar_color(a: Rgba<u8>, b: Rgba<u8>, threshold: f64) -> bool {
    distance(a, b) < threshold
}

fn fill<I: GenericImage<Pixel=Rgba<u8>>>(img: &mut I, rect: Rect, color: Rgba<u8>) {
    for y in rect.top..rect.bottom {
        for x in rect.left..rect.right {
            img.put_pixel(x, y, color);
        }
    }
}

fn max_index(array: &[u32]) -> usize {
    let mut i = 0;

    for (j, &el) in array.iter().enumerate() {
        if el > array[i] {
            i = j;
        }
    }

    i
}

fn main() {
    let colors = [
        Rgba([0xff, 0xbf, 0x02, 0xff]), // yellow
        Rgba([0xed, 0x1f, 0x02, 0xff]), // orange
        Rgba([0x78, 0x32, 0x42, 0xff]), // purple
        Rgba([0x7d, 0x81, 0x02, 0xff]), // green
    ];
    //let thresholds = [40.0, 50.0, 20.0, 40.0];
    let bounds = Rect::new(72, 89, 541, 708);

    let pathname = env::args().skip(1).next().expect("no input given");
    let thresholds: Vec<f64> = env::args().skip(2).take(4).map(|t| {
        t.parse().unwrap()
    }).collect();

    let mut img = image::open(pathname).ok().expect("input failed to open");

    let cell_width = (bounds.width() as f64) / (COLS as f64);
    let cell_height = (bounds.height() as f64) / (ROWS as f64);

    for y in 0..ROWS {
        for x in 0..COLS {
            let left = (bounds.left as f64) + cell_width * (x as f64);
            let top = (bounds.top as f64) + cell_height * (y as f64);
            let right = left + cell_width - 1.0;
            let bottom = top + cell_height - 1.0;

            let cell = Rect::new(left as u32, top as u32, right as u32, bottom as u32);

            let mut votes = [0u32; 4];

            for y in cell.top..cell.bottom {
                for x in cell.left..cell.right {
                    let p = img.get_pixel(x, y);

                    for (i, (&color, &threshold)) in colors.iter()
                            .zip(thresholds.iter())
                            .enumerate() {
                        if is_similar_color(p, color, threshold) {
                            votes[i as usize] += 1;
                        }
                    }
                }
            }

            if votes.iter().all(|&c| c == 0) {
                print!("? ");
            } else {
                let i = max_index(&votes);

                fill(&mut img, cell, colors[i]);

                print!("{} ", match i {
                    0 => "Y",
                    1 => "O",
                    2 => "P",
                    3 => "G",
                    _ => unreachable!(),
                });
            }
        }

        println!("");
    }

    let mut f = File::create("out.png").ok().expect("output failed to open");
    img.save(&mut f, image::PNG).ok().expect("save failed");
}

//| Out of context documantation
//| sierpinski triangle image generator

extern crate image;
extern crate rand;

use rand::Rng;
// use std::fs::File;
use std::path::Path;
// use image::ColorType;

/// points used to build triangle and plot in the canvas
pub struct Point {
    x: u32,
    y: u32,
}

const WIDTH: u32 = 1800;
const HEIGHT: u32 = 1600;

/// main program - this shows in documentation if function is public
fn main() {
    let mut img = image::ImageBuffer::from_fn(WIDTH, HEIGHT, |x, y| {
        if x == 0 && y == 0 {
            image::Luma([0u8])
        } else {
            image::Luma([255u8])
        }
    });
    // counter
    let mut cnt: u32 = 1_000_000;

    // fixed sized array, not a vector
    let pts: [Point; 3] = [
        Point { x: WIDTH / 2, y: 0 },
        Point { x: 0, y: HEIGHT },
        Point {
            x: WIDTH,
            y: HEIGHT,
        },
    ];

    let mut num: usize;

    // arbitrary point
    let mut p = Point { x: 350, y: 350 };

    let pixel = img[(0, 0)];

    while cnt > 0 {
        cnt -= 1;
        num = rand::thread_rng().gen_range(0, 3);
        p.x = (p.x + pts[num].x) / 2;
        p.y = (p.y + pts[num].y) / 2;
        img.put_pixel(p.x, p.y, pixel);
    }

    // let ref mut fout = File::create(&Path::new("tri.png")).unwrap();

    img.save(Path::new("test_tri.png")).unwrap();
}

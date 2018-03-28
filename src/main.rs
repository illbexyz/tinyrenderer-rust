extern crate image;
extern crate num;

mod model;
mod utils;

use std::fs::File;
use image::{Rgb, ImageBuffer, imageops};
use utils::{Point};

fn line(img: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, p1: &mut Point, p2: &mut Point, color: Rgb<u8>) {
    let steep = 
        if (p1.x - p2.x).abs() < (p1.y - p2.y).abs() {
            true
        } else {
            false
        };
    
    if steep {
        let p1_cp = p1.clone();
        p1.x = p1_cp.y;
        p1.y = p1_cp.x;
        let p2_cp = p2.clone();
        p2.x = p2_cp.y;
        p2.y = p2_cp.x;
    };

    if p1.x > p2.x {
        Point::swap(p1, p2);
    };

    let dx = p2.x - p1.x;
    let dy = p2.y - p1.y;
    let derror2 = dy.abs() * 2;
    let mut error2 = 0;
    let mut y = p1.y;

    for x in p1.x .. p2.x+1 {
        // println!("line: {}, {}", x, y);
        if steep {
            *(img.get_pixel_mut(y as u32, x as u32)) = color;
        } else {
            *(img.get_pixel_mut(x as u32, y as u32)) = color;
        }
        error2 += derror2;
        if error2 > dx {
            y += if p2.y > p1.y { 1 } else { -1 };
            error2 -= dx * 2;
        }
    }
}

// fn barycentric(points: Vec<Point>, p: Point) {
//     let v1 =
//         Vec3::new(
//             points[2].x - points[0].x,
//             points[1].x - points[0].x,
//             points[0].x - p.x
//         );
    
//     let v2 =
//         Vec3::new(
//             points[2].y - points[0].y,
//             points[1].y - points[0].y,
//             points[0].y - p.y
//         );
// }

// fn triangle() {

// }

fn main() {
    let width = 600;
    let height = 600;
    let mut img = image::ImageBuffer::new(width, height);
    
    let m = model::Model::from_file("obj/african_head.obj");
    let white = Rgb([255u8, 255u8, 255u8]);

    for face in m.faces {
        for j in 0..3 {
            let v0 = m.verts[(face[j] - 1) as usize];
            let v1 = m.verts[(face[(j+1) % 3] - 1) as usize];
            let mut p1 = Point::new(
                (((v0.x + 1.) * (width as f32) / 2.) as i32).min(width as i32 - 1),
                (((v0.y+1.) * (height as f32) / 2.) as i32).min(height as i32 - 1)
            );
            let mut p2 = Point::new(
                (((v1.x + 1.) * (width as f32) / 2.) as i32).min(width as i32 - 1),
                (((v1.y+1.) * (height as f32) / 2.) as i32).min(height as i32 - 1)
            );
            line(&mut img, &mut p1, &mut p2, white);
        }
        
    }

    let flipped_image = imageops::flip_vertical(&img);

    let ref mut fout = File::create("output.png").unwrap();
    image::ImageRgb8(flipped_image).save(fout, image::PNG).unwrap();
}

extern crate image;
extern crate rand;

mod model;
mod utils;

use std::fs::File;
use image::{Rgb, ImageBuffer, imageops};
use utils::{Point, Vec3f};
use rand::{Rng, thread_rng};

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

fn barycentric(points: &Vec<Point>, p: Point) -> Vec3f {
    let v1 =
        Vec3f::new(
            (points[2].x - points[0].x) as f32,
            (points[1].x - points[0].x) as f32,
            (points[0].x - p.x) as f32
        );
    
    let v2 =
        Vec3f::new(
            (points[2].y - points[0].y) as f32,
            (points[1].y - points[0].y) as f32,
            (points[0].y - p.y) as f32
        );
    
    let c = Vec3f::cross(&v1, &v2);

    if (c.z).abs() < 1.0 {
        Vec3f::new(-1.0, 1.0, 1.0)
    } else {
        Vec3f::new(1.0 - (c.x + c.y) / c.z, c.y / c.z, c.x / c.z)
    }
}

fn triangle(img: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, pts: &Vec<Point>, color: Rgb<u8>) {
    let mut bboxmax = Point::new(0, 0);
    let mut bboxmin = Point::new((img.width() - 1) as i32, (img.height() - 1) as i32);
    for p in pts {
        if p.x < bboxmin.x {
            bboxmin.x = p.x;
        }
        if p.y < bboxmin.y {
            bboxmin.y = p.y
        }
        if p.x > bboxmax.x {
            bboxmax.x = p.x;
        }
        if p.y > bboxmax.y {
            bboxmax.y = p.y
        }
    }
    // println!("{} - {}", bboxmin, bboxmax);
    for i in bboxmin.x .. bboxmax.x + 1 {
        for j in bboxmin.y .. bboxmax.y + 1 {
            let p = Point::new(i, j);
            let bary = barycentric(pts, p);
            if bary.x >= 0.0 && bary.y >= 0.0 && bary.z >= 0.0 {
                *(img.get_pixel_mut(i as u32, j as u32)) = color;
            }
        }
    }
}

fn main() {
    let width = 1200;
    let height = 1200;
    let mut img = image::ImageBuffer::new(width, height);
    let light_dir = Vec3f::new(0.0, 0.0, -1.0).normalize();
    
    let m = model::Model::from_file("obj/african_head.obj");
    // let white = Rgb([255u8, 255u8, 255u8]);
    // let mut rng = thread_rng();

    for face in m.faces {
        let mut world_coords = vec![];
        let mut screen_coords = vec![];
        for j in 0..3 {
            world_coords.push(m.verts[(face[j] - 1) as usize]);
            screen_coords.push(
                Point::new(
                    (((world_coords[j].x+1.0) * (width as f32)/2.0) as i32).min(width as i32 - 1),
                    (((world_coords[j].y+1.0) * (height as f32)/2.0) as i32).min(height as i32 - 1)
                )
            );
        };
        let normal = Vec3f::cross(
            &(world_coords[2] - world_coords[0]),
            &(world_coords[1] - world_coords[0])
        ).normalize();

        // println!("{:?}", normal);

        let light_intensity = normal * light_dir;
        let triangle_color = (light_intensity * 255.0) as u8;

        // println!("{}", triangle_color);

        if light_intensity > 0.0 {
            triangle(&mut img, &screen_coords, Rgb([triangle_color, triangle_color, triangle_color]));
        }
        // for j in 0..3 {
        //     let v0 = m.verts[(face[j] - 1) as usize];
        //     let v1 = m.verts[(face[(j+1) % 3] - 1) as usize];
        //     let mut p1 = Point::new(
        //         (((v0.x + 1.) * (width as f32) / 2.) as i32).min(width as i32 - 1),
        //         (((v0.y + 1.) * (height as f32) / 2.) as i32).min(height as i32 - 1)
        //     );
        //     let mut p2 = Point::new(
        //         (((v1.x + 1.) * (width as f32) / 2.) as i32).min(width as i32 - 1),
        //         (((v1.y + 1.) * (height as f32) / 2.) as i32).min(height as i32 - 1)
        //     );
        //     line(&mut img, &mut p1, &mut p2, white);
        // }
    }

    // triangle(&mut img, &vec![Point::new(50, 50), Point::new(70, 70), Point::new(70, 50)], white);

    let flipped_image = imageops::flip_vertical(&img);

    let ref mut fout = File::create("output.png").unwrap();
    image::ImageRgb8(flipped_image).save(fout, image::PNG).unwrap();
}

extern crate image;
extern crate rand;
extern crate num;

mod model;
mod utils;
mod vec;

use std::fs::File;
use image::{Rgb, ImageBuffer, imageops};
use utils::{Point};
use vec::Vec3;
use model::Model;

#[allow(dead_code)]
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

#[allow(dead_code)]
fn barycentric(points: &Vec<Point>, p: Point) -> Vec3<f64> {
    let v1 =
        Vec3::new(
            (points[2].x - points[0].x) as f64,
            (points[1].x - points[0].x) as f64,
            (points[0].x - p.x) as f64
        );
    
    let v2 =
        Vec3::new(
            (points[2].y - points[0].y) as f64,
            (points[1].y - points[0].y) as f64,
            (points[0].y - p.y) as f64
        );
    
    let c = Vec3::cross(&v1, &v2);

    if (c.z).abs() < 1.0 {
        Vec3::new(-1.0, 1.0, 1.0)
    } else {
        Vec3::new(1.0 - (c.x + c.y) / c.z, c.y / c.z, c.x / c.z)
    }
}

#[allow(dead_code)]
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

#[allow(dead_code)]
fn draw_lines(img: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, m: Model, color: Rgb<u8>) {
    let width = img.width();
    let height = img.height();
    for face in m.faces {
        for j in 0..3 {
            let v0 = m.verts[(face[j] - 1) as usize];
            let v1 = m.verts[(face[(j+1) % 3] - 1) as usize];
            let mut p1 = Point::new(
                (((v0.x + 1.) * (width as f64) / 2.) as i32).min(width as i32 - 1),
                (((v0.y + 1.) * (height as f64) / 2.) as i32).min(height as i32 - 1)
            );
            let mut p2 = Point::new(
                (((v1.x + 1.) * (width as f64) / 2.) as i32).min(width as i32 - 1),
                (((v1.y + 1.) * (height as f64) / 2.) as i32).min(height as i32 - 1)
            );
            line(img, &mut p1, &mut p2, color);
        }
    }
}

#[allow(dead_code)]
fn draw_triangles(img: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, m: Model, light_dir: Vec3<f64>) {
    let width = img.width();
    let height = img.height();
    for face in m.faces {
        let mut world_coords = vec![];
        let mut screen_coords = vec![];
        for j in 0..3 {
            world_coords.push(m.verts[(face[j] - 1) as usize]);
            screen_coords.push(
                Point::new(
                    (((world_coords[j].x+1.0) * (width as f64)/2.0) as i32).min(width as i32 - 1),
                    (((world_coords[j].y+1.0) * (height as f64)/2.0) as i32).min(height as i32 - 1)
                )
            );
        };
        let normal = Vec3::cross(
            &(world_coords[2] - world_coords[0]),
            &(world_coords[1] - world_coords[0])
        ).normalize();

        let light_intensity = normal * light_dir;
        let triangle_color = (light_intensity * 255.0) as u8;

        if light_intensity > 0.0 {
            triangle(img, &screen_coords, Rgb([triangle_color, triangle_color, triangle_color]));
        }
        
    }
}

fn main() {
    let width = 1200;
    let height = 1200;
    let mut img = image::ImageBuffer::new(width, height);
    let light_dir = Vec3::new(0.0, 0.0, -1.0).normalize();
    
    let m = model::Model::from_file("obj/african_head.obj");
    let white = Rgb([255u8, 255u8, 255u8]);
    // let mut rng = thread_rng();

    draw_lines(&mut img, m, white);
    // draw_triangles(&mut img, m, light_dir);

    let flipped_image = imageops::flip_vertical(&img);

    let ref mut fout = File::create("output.png").unwrap();
    image::ImageRgb8(flipped_image).save(fout, image::PNG).unwrap();
}

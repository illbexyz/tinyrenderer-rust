use std::io::{BufReader, BufRead};
use std::fs::File;
use utils::Vec3;

// impl<i32> fmt::Display for Vec3<i32> {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "({}, {}, {})", self.x, self.y, self.z)
//     }
// }

pub struct Model {
    pub verts: Vec<Vec3<f32>>,
    pub faces: Vec<Vec<i32>>
}

impl Model {
    fn new(verts: Vec<Vec3<f32>>, faces: Vec<Vec<i32>>) -> Model {
        Model { verts, faces }
    }

    pub fn from_file(filename: &str) -> Model {
        let file = File::open(filename).expect("File not found");
        let mut verts: Vec<Vec3<f32>> = vec![];
        let mut faces: Vec<Vec<i32>> = vec![];

        for line in BufReader::new(file).lines() {
            let l = line.unwrap();
            let line_tokens: Vec<&str> = l.split(' ').collect();
            if line_tokens[0] == "v" {
                verts.push(
                    Vec3::new(
                        line_tokens[1].parse().unwrap(),
                        line_tokens[2].parse().unwrap(),
                        line_tokens[3].parse().unwrap()
                    )
                )
            } else if line_tokens[0] == "f" {
                let face1_tokens: Vec<&str> = line_tokens[1].split('/').collect();
                let face2_tokens: Vec<&str> = line_tokens[2].split('/').collect();
                let face3_tokens: Vec<&str> = line_tokens[3].split('/').collect();

                faces.push(vec![
                    face1_tokens[0].parse().unwrap(),
                    face2_tokens[0].parse().unwrap(),
                    face3_tokens[0].parse().unwrap()
                ])
            }
        }

        Model::new(verts, faces)
    }
}
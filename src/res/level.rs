use std::{fs, vec};

use raylib::prelude;
use raylib::prelude::RaylibMesh;
use raylib::{math::Vector3, models::Model, RaylibHandle, RaylibThread};

#[derive(Debug)]
pub struct Wall {
    pub id: usize,
    pub xstart: f32,
    pub zstart: f32,
    pub xend: f32,
    pub zend: f32,
    pub portalid: i32,
}

#[derive(Debug)]
pub struct Sector {
    pub id: usize,         // Sector id
    pub firstwall: usize,  // Alternative way of acessing walls...?
    pub nwalls: usize,     // Same as above
    pub floor_height: f32, // Floor height
    pub ceil_height: f32,  // Ceiling Height (Really?)
    pub rot_wall_id: i32,  // Id of wall which has been selected to be rotated
    pub rot_wall_ang: f32, // Angle of said rotation of wall
}

pub struct Map {
    pub name: String,
    pub sectors: Vec<Sector>,
    pub walls: Vec<Wall>,
    pub wallmodels: Option<Vec<Model>>,
}

#[derive(PartialEq, PartialOrd)]
pub enum ReaderMode {
    SEARCHING,
    SECTORS,
    WALLS,
}

impl Map {
    pub fn new(fname: &str, rl: &RaylibHandle, thread: &RaylibThread) -> Self {
        let mut rm: ReaderMode = ReaderMode::SEARCHING;
        let mut map: Map = Map {
            name: fname.to_string(),
            sectors: vec![],
            walls: vec![],
            wallmodels: None,
        };
        let f = fs::read_to_string(fname.to_string()).unwrap();

        for (count, line) in f.lines().enumerate() {
            if !line.is_empty() {
                // comment skipping
                if line.chars().nth(0) == Some('/') && line.chars().nth(1) == Some('/') {
                    continue;
                }
                //println!("{}", line);
                //Check for headers
                if line.contains("[SECTORS]") {
                    rm = ReaderMode::SECTORS
                } else if line.contains("[WALLS]") {
                    rm = ReaderMode::WALLS
                } else {
                    // Not sector or wall headers, the line is describing a sector/wall
                    if rm == ReaderMode::SECTORS {
                        let mut chopped: Vec<&str> = line.split_ascii_whitespace().collect();
                        chopped.retain(|x| **x != *" "); //what
                                                         //println!("{:?}", chopped);
                        unsafe {
                            let s: Sector = Sector {
                                id: chopped.get_unchecked(0).to_string().parse().unwrap(),
                                firstwall: chopped.get_unchecked(1).to_string().parse().unwrap(),
                                nwalls: chopped.get_unchecked(2).to_string().parse().unwrap(),
                                floor_height: chopped.get_unchecked(3).to_string().parse().unwrap(),
                                ceil_height: chopped.get_unchecked(4).to_string().parse().unwrap(),
                                rot_wall_id: chopped.get_unchecked(5).to_string().parse().unwrap(),
                                rot_wall_ang: chopped.get_unchecked(6).to_string().parse().unwrap(),
                            };
                            println!("{:?}", s);
                            map.sectors.push(s);
                        }
                    } else if rm == ReaderMode::WALLS {
                        let mut chopped: Vec<&str> = line.split_ascii_whitespace().collect();
                        chopped.retain(|x| **x != *" "); //what
                        unsafe {
                            let w: Wall = Wall {
                                id: chopped.get_unchecked(0).to_string().parse().unwrap(),
                                xstart: chopped.get_unchecked(1).to_string().parse().unwrap(),
                                zstart: chopped.get_unchecked(2).to_string().parse().unwrap(),
                                xend: chopped.get_unchecked(3).to_string().parse().unwrap(),
                                zend: chopped.get_unchecked(4).to_string().parse().unwrap(),
                                portalid: chopped.get_unchecked(5).to_string().parse().unwrap(),
                            };
                            println!("{:?}", w);
                            map.walls.push(w);
                        }
                    }
                }
            }
        }

        for s in &map.sectors {
            for i in s.firstwall..s.nwalls + 1 {
                let w = map.walls.get(i).unwrap();
                //draw_wall_lines(d2, w, s);
                let cube_pos = Vector3 {
                    // Midpoint formula to find center of line.
                    x: (w.xstart + w.xend) / 2.0,
                    y: (s.floor_height + s.ceil_height) / 2.0,
                    z: (w.zstart + w.zend) / 2.0,
                };

                let cube_height = s.ceil_height - s.floor_height; // How tall the wall?
                let line_xz_slope = (w.zend - w.zstart) / (w.xend - w.xstart); //Slope formula
                let cube_angle = f32::atan(line_xz_slope).to_degrees(); //Converts to rads, then deg
                let line_len =
                    f32::sqrt((w.xend - w.xstart).powf(2.0) + (w.zend - w.zstart).powf(2.0));

                //println!("{}", cube_angle);

                let model = unsafe {
                    rl.load_model_from_mesh(
                        &thread,
                        prelude::Mesh::gen_mesh_cube(&thread, line_len, cube_height, 0.0)
                            .make_weak(),
                    )
                    .unwrap()
                };
            }
        }
        return map;
    }
}

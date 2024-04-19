use std::{
    fs::{self, File},
    vec,
};

use raylib::ffi::Vector3;

pub struct Wall {
    id: usize,
    xstart: f32,
    zstart: f32,
    xend: f32,
    zend: f32,
    portalid: usize,
}

pub struct Sector {
    id: usize,         // Sector id
    walls: Vec<Wall>,  // List of walls in sector
    firstwall: usize,  // Alternative way of acessing walls...?
    nwalls: usize,     // Same as above
    floor_height: f32, // Floor height
    ceil_height: f32,  // Ceiling Height (Really?)
    rot_wall_id: i32,  // Id of wall which has been selected to be rotated
    rot_wall_ang: f32, // Angle of said rotation of wall
}

pub struct Entity {
    pos: Vector3,
}

pub struct Map {
    name: String,
    sectors: Vec<Sector>,
    entities: Vec<Entity>,
}

#[derive(PartialEq, PartialOrd)]
pub enum ReaderMode {
    SEARCHING,
    SECTORS,
    WALLS,
}

impl Map {
    pub fn new(fname: &str) -> Self {
        let mut rm: ReaderMode = ReaderMode::SEARCHING;
        let f = fs::read_to_string(fname.to_string()).unwrap();
        for line in f.lines() {
            if !line.is_empty() {
                // comment skipping
                if line.chars().nth(0) == Some('/') && line.chars().nth(1) == Some('/') {
                    continue;
                }
                println!("{}", line);
                //Check for headers
                if line.contains("[SECTORS]") {
                    rm = ReaderMode::SECTORS
                } else if line.contains("[WALLS]") {
                    rm = ReaderMode::WALLS
                } else {
                    // Not sector or wall headers, the line is describing a sector/wall

                    if rm == ReaderMode::SECTORS {
                        let chopped: Vec<char> = line.split("").map(|x| x.).collect();

                        println!("{:?}", chopped);
                    }
                }
            }
        }
        return Map {
            name: fname.to_string(),
            sectors: vec![],
            entities: vec![],
        };
    }
}

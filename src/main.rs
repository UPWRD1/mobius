use std::{f32::consts::PI, ops::Mul};

use raylib::prelude::*;

pub mod res;

const WINDOW_WIDTH: i32 = 1280;
const WINDOW_HEIGHT: i32 = 720; // / 2;

const PLAYER_CAMERA_MIN_CLAMP: f32 = 89.0;
const PLAYER_CAMERA_MAX_CLAMP: f32 = -89.0;
const PLAYER_CAMERA_PANNING_DIV: f32 = 5.1;

const MOVE_FORWARD: KeyboardKey = KeyboardKey::KEY_W;
const MOVE_BACKWARD: KeyboardKey = KeyboardKey::KEY_S;
const MOVE_LEFT: KeyboardKey = KeyboardKey::KEY_A;
const MOVE_RIGHT: KeyboardKey = KeyboardKey::KEY_D;

fn main() {
    //unsafe {
    // exit(0);
    //}
    let (mut rl, thread) = raylib::init()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("Hello, world!")
        .build();

    let mut camera: Camera3D = Camera3D::perspective(
        Vector3::new(2.5, 1.0, 2.5),
        Vector3::new(5.0, 1.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
        60.0,
    );

    let mut m = res::level::Map::new("demo.map");

    m.gen_wallmods(&mut rl, &thread);

    rl.set_camera_mode(&camera, CameraMode::CAMERA_FIRST_PERSON);
    rl.set_target_fps(30);
    rl.disable_cursor();
    rl.hide_cursor();

    while !rl.window_should_close() {
        rl.update_camera(&mut camera);

        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::BLACK);
        let mut d2 = d.begin_mode3D(camera);

        d2.draw_plane(
            Vector3::new(0.0, 0.0, 0.0),
            Vector2::new(32.0, 32.0),
            Color::LIGHTGRAY,
        );

        for (i, model) in m.wallmodels.clone().into_iter().enumerate() {
            model
                .model
                .materials()
                .first()
                .unwrap()
                .to_owned()
                .set_material_texture(
                    raylib::consts::MaterialMapIndex::MATERIAL_MAP_ALBEDO,
                    model.tex,
                );
            d2.draw_model_ex(
                model.model,
                model.position,
                Vector3::up(),
                model.angle,
                Vector3::one(),
                model.color,
            );
            //dbg!(model);
        }
        d2.draw_fps(10, 10);
    }
}

fn draw_wall_lines(
    d2: &mut RaylibMode3D<'_, RaylibDrawHandle<'_>>,
    w: &res::level::Wall,
    s: &res::level::Sector,
) {
    d2.draw_line_3D(
        Vector3 {
            x: w.xstart,
            y: s.floor_height,
            z: w.zstart,
        },
        Vector3 {
            x: w.xend,
            y: s.ceil_height,
            z: w.zend,
        },
        raylib::prelude::Color::RED,
    );
}

pub enum MovementState {
    STANDING,
    RUNNING,
    JUMPING,
}

pub struct Player {
    cam: Camera3D,
    sens: f32,
    target_dist: f32,
    eyespos: f32,
    angle: Vector3,
    speed: f32,
    health: i32,
    isdead: bool,
    movementstate: MovementState,
}

impl Player {
    pub fn new(pos: Vector3, target: Vector3, sens: f32, speed: f32) -> Self {
        let v1 = pos;
        let v2 = target;
        let dx = v2.x - v1.x;
        let dy = v2.y - v1.y;
        let dz = v2.z - v1.z;

        Player {
            cam: Camera3D::perspective(pos, target, Vector3::up(), 90.0),
            sens,
            target_dist: f32::sqrt(dx * dx + dy * dy + dz * dz),
            eyespos: 0.0,
            angle: Vector3::new(dx.atan2(dz), dy.atan2(f32::sqrt(dx * dx + dz * dz)), 0.0),
            speed,
            health: 100,
            isdead: false,
            movementstate: MovementState::STANDING,
        }
    }

    pub fn update(&mut self, rl: RaylibHandle) {
        let old_pos = self.cam.position;

        let mouse_pos_delta = rl.get_mouse_position();

        let direction: [bool; 4] = [
            rl.is_key_down(MOVE_FORWARD),
            rl.is_key_down(MOVE_BACKWARD),
            rl.is_key_down(MOVE_RIGHT),
            rl.is_key_down(MOVE_LEFT),
        ];

        let ang_x = self.angle.x;
        let ang_y = self.angle.y;

        // Move Player
        self.cam.position.x += ((f32::sin(ang_x) * ((direction[1] as usize) as f32)
            - f32::sin(ang_x) * ((direction[0] as usize) as f32)
            - f32::cos(ang_x) * ((direction[3] as usize) as f32)
            + f32::cos(ang_x) * ((direction[2] as usize) as f32))
            * self.speed)
            * rl.get_frame_time();

        self.cam.position.y += ((f32::sin(ang_y) * ((direction[0] as usize) as f32)
            - f32::sin(ang_x) * ((direction[1] as usize) as f32))
            * self.speed)
            * rl.get_frame_time();

        self.cam.position.z += ((f32::cos(ang_x) * ((direction[1] as usize) as f32)
            - f32::cos(ang_x) * ((direction[0] as usize) as f32)
            - f32::sin(ang_x) * ((direction[3] as usize) as f32)
            + f32::sin(ang_x) * ((direction[2] as usize) as f32))
            * self.speed)
            * rl.get_frame_time();

        //Calculate Camera Orientation
        self.angle.x -= mouse_pos_delta.x * self.sens * rl.get_frame_time();
        self.angle.y -= mouse_pos_delta.y * self.sens * rl.get_frame_time();

        if self.angle.y > PLAYER_CAMERA_MIN_CLAMP * DEG2RAD as f32 {
            self.angle.y = PLAYER_CAMERA_MIN_CLAMP * DEG2RAD as f32;
        } else if self.angle.y < PLAYER_CAMERA_MAX_CLAMP * DEG2RAD as f32 {
            self.angle.y = PLAYER_CAMERA_MAX_CLAMP * DEG2RAD as f32;
        }

        let translation: Matrix =
            Matrix::translate(0.0, 0.0, self.target_dist / PLAYER_CAMERA_PANNING_DIV);
        let rotation: Matrix = Matrix::inverted(&Matrix::rotate_xyz(Vector3 {
            x: PI * 2.0 - self.angle.y,
            y: PI * 2.0 - self.angle.y,
            z: 0.0,
        }));
        let transform: Matrix = Matrix::mul(translation, rotation);

        self.cam.target.x = self.cam.position.x - transform.m12;
        self.cam.target.y = self.cam.position.y - transform.m13;
        self.cam.target.z = self.cam.position.z - transform.m14;

        self.cam.position.y = self.eyespos;
    }
}

pub fn make_bounding_box(position: Vector3, size: Vector3) -> BoundingBox {
    let bb: BoundingBox = BoundingBox {
        min: Vector3 {
            x: position.x - size.x / 2.0,
            y: position.y - size.y / 2.0,
            z: position.z - size.z / 2.0,
        },
        max: Vector3 {
            x: position.x + size.x / 2.0,
            y: position.y + size.y / 2.0,
            z: position.z + size.z / 2.0,
        },
    };
    bb
}

pub fn check_collision(entity_pos: Vector3, entity_size: Vector3, entity_id: i32) {
    let entity_box = make_bounding_box(entity_pos, entity_size);
}

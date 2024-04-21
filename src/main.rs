use std::{f32::consts::PI, ops::Mul};

use raylib::prelude::*;
use res::player::Player;

pub mod res;

const WINDOW_WIDTH: i32 = 1280;
const WINDOW_HEIGHT: i32 = 720; // / 2;

const PLAYER_CAMERA_MIN_CLAMP: f32 = 89.0;
const PLAYER_CAMERA_MAX_CLAMP: f32 = -89.0;
const PLAYER_MOUSE_SENS: f32 = 0.1;
const PLAYER_CAM_DIV: f32 = 5.1;

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

    rl.set_camera_mode(&camera, CameraMode::CAMERA_CUSTOM);
    rl.set_target_fps(30);
    rl.disable_cursor();
    rl.hide_cursor();

    let mut p = Player::new(camera, &rl);

    while !rl.window_should_close() {
        //rl.update_camera(&mut camera);
        let dt = rl.get_frame_time();
        p.update(&rl, dt);

        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::BLACK);
        d.draw_fps(10, 10);

        let mut d2 = d.begin_mode3D(camera);

        d2.draw_plane(
            Vector3::new(0.0, 0.0, 0.0),
            Vector2::new(32.0, 32.0),
            Color::LIGHTGRAY,
        );

        for model in m.wallmodels.clone().into_iter() {
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
    }
}

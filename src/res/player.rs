use std::{f32::consts::PI, ops::Mul};

use raylib::{
    camera::Camera3D,
    math::{Matrix, Vector2, Vector3},
    RaylibHandle,
};

use crate::{
    MOVE_BACKWARD, MOVE_FORWARD, MOVE_LEFT, MOVE_RIGHT, PLAYER_CAMERA_MAX_CLAMP,
    PLAYER_CAMERA_MIN_CLAMP, PLAYER_CAM_DIV, PLAYER_MOUSE_SENS,
};

pub struct Player {
    pub cam: Camera3D,
    pub move_speed: f32,
    pub vel: Vector3,
    pub prev_mouse_pos: Vector2,
    pub pitch: f32,
    pub yaw: f32,
    pub targdist: f32,
}

impl Player {
    pub fn new(c: Camera3D, rl: &RaylibHandle) -> Self {
        let v1 = c.position;
        let v2 = c.target;

        let dx = v2.x - v1.x;
        let dy = v2.y - v1.y;
        let dz = v2.z - v1.z;
        Player {
            cam: c,
            move_speed: 1.0,
            vel: Vector3::zero(),
            prev_mouse_pos: rl.get_mouse_position(),
            pitch: 0.0,
            yaw: 0.0,
            targdist: f32::sqrt(dx * dx + dy * dy + dz * dz),
        }
    }

    #[inline]
    pub fn update(&mut self, rl: &RaylibHandle, dt: f32) {
        let direction: [bool; 4] = [
            rl.is_key_down(MOVE_FORWARD),
            rl.is_key_down(MOVE_BACKWARD),
            rl.is_key_down(MOVE_RIGHT),
            rl.is_key_down(MOVE_LEFT),
        ];
        println!("{:?}", direction);
        self.cam.position.x = (f32::sin(self.pitch) * (direction[1] as usize as f32)
            - f32::sin(self.pitch) * (direction[0] as usize as f32)
            - f32::cos(self.pitch) * (direction[3] as usize as f32)
            + f32::cos(self.pitch) * (direction[2] as usize as f32) * self.move_speed)
            * dt;

        self.cam.position.z = (f32::cos(self.pitch) * (direction[1] as usize as f32)
            - f32::cos(self.pitch) * (direction[0] as usize as f32)
            + f32::sin(self.pitch) * (direction[3] as usize as f32)
            - f32::sin(self.pitch) * (direction[2] as usize as f32) * self.move_speed)
            * dt;

        //let dt = rl.get_frame_time();
        let current_mouse_pos = rl.get_mouse_position();
        let mouse_delta = (
            (current_mouse_pos.x - self.prev_mouse_pos.x) as f32,
            (current_mouse_pos.y - self.prev_mouse_pos.y) as f32,
        );
        self.prev_mouse_pos = current_mouse_pos;
        self.yaw += mouse_delta.0 * PLAYER_MOUSE_SENS * dt;
        self.pitch -= mouse_delta.1 * PLAYER_MOUSE_SENS * dt;
        //println!("{}", self.pitch);
        self.pitch = self
            .pitch
            .clamp(PLAYER_CAMERA_MIN_CLAMP, PLAYER_CAMERA_MAX_CLAMP);

        let translation = Matrix::translate(0.0, 0.0, self.targdist / PLAYER_CAM_DIV);

        let rotation = Matrix::rotate_xyz(Vector3 {
            x: PI * 2.0 - self.yaw,
            y: PI * 2.0 - self.pitch,
            z: 0.0,
        })
        .inverted();
        let transform = Matrix::mul(translation, rotation);

        self.cam.target.x = self.cam.position.x - transform.m12;
        self.cam.target.y = self.cam.position.y - transform.m13;
        self.cam.target.z = self.cam.position.z - transform.m14;
    }
}

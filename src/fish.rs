use raylib::prelude::*;
use crate::{math::*, render::*};

const ALIGNMENT_WEIGHT: f32 = 1.0;
const COHESION_WEIGHT: f32 = 0.5;
const SEPARATION_WEIGHT: f32 = 2.0;

const ALIGNMENT_FORCE: f32 = 11.0;
const COHESION_FORCE: f32 = 5.0;
const SEPARATION_FORCE: f32 = 5.0;
const STEERING_FORCE: f32 = 0.5;

const DRAG: f32 = 1.5;

#[derive(Clone)]
pub struct Fish {
    id: i32,
    pos: Vector2,
    dir: f32,
    min_speed: f32,
    max_speed: f32,
    color: Color,
    vision_radius: f32,
    separation_radius: f32,
    speed: f32,
}

impl Fish {
    pub fn new(id: i32, pos: Vector2, dir: f32, min_speed: f32, max_speed:f32, color: Color) -> Self {
        Self {
            id,
            pos,
            dir,
            min_speed,
            max_speed,
            color,
            vision_radius: 25.0,
            separation_radius: 12.0,
            speed: min_speed,
        }
    }

    pub fn update(&mut self, d: &RaylibDrawHandle, fishes: &Vec<Fish>) {
        let mut vision_fishes: Vec<&Fish> = vec![];
        let mut separation_fishes: Vec<&Fish> = vec![];

        for fish in fishes.iter() {
            let distance = fish.pos.distance_to(self.pos);
            if distance > self.vision_radius || fish.id == self.id {continue}

            vision_fishes.push(fish);

            if distance > self.separation_radius || fish.id == self.id {continue}
            separation_fishes.push(fish);
        }

        let separation_positions: Vec<Vector2> = separation_fishes.iter().map(|f| f.pos).collect();
        let mut separation_vec = Vector2::zero();
        if !separation_positions.is_empty() {
            separation_vec = away_from_points(self.pos, &separation_positions, d.get_screen_width(), d.get_screen_height()).normalized();
        }

        let fish_dirs: Vec<f32> = vision_fishes.iter().map(|f| f.dir).collect();
        let mut alignment_vec = Vector2::zero();
        if !fish_dirs.is_empty() {
            let alignment_dir = average_angle(&fish_dirs);
            alignment_vec = Vector2 {
                x: alignment_dir.cos(),
                y: alignment_dir.sin()
            }.normalized();
        }

        let fish_positions: Vec<Vector2> = vision_fishes.iter().map(|f| f.pos).collect();
        let mut cohesion_vec = Vector2::zero();
        if !fish_positions.is_empty() {
            cohesion_vec = (center_of_mass(&fish_positions) - self.pos).normalized();
        }

        let mut steering = (separation_vec * SEPARATION_WEIGHT * SEPARATION_FORCE +
            alignment_vec * ALIGNMENT_WEIGHT * ALIGNMENT_FORCE +
            cohesion_vec * COHESION_WEIGHT * COHESION_FORCE) * d.get_frame_time();
        if steering.length_sqr() > 0.0 {
            steering = steering.normalized() * STEERING_FORCE;
        } else {
            steering = Vector2 { x: self.dir.cos(), y: self.dir.sin() } * STEERING_FORCE;
        }

        // Limit the turn rate to prevent flickering
        let desired_dir = steering.y.atan2(steering.x);
        let mut angle_diff = desired_dir - self.dir;
        // Wrap angle_diff to [-PI, PI]
        while angle_diff > std::f32::consts::PI { angle_diff -= 2.0 * std::f32::consts::PI; }
        while angle_diff < -std::f32::consts::PI { angle_diff += 2.0 * std::f32::consts::PI; }
        let max_turn_per_sec = 2.0 * std::f32::consts::PI / 180.0 * 180.0; // 180 degrees per second (example)
        let max_turn = max_turn_per_sec * d.get_frame_time();
        let clamped_diff = angle_diff.clamp(-max_turn, max_turn);
        self.dir += clamped_diff;
        // Keep self.dir in [-PI, PI]
        if self.dir > std::f32::consts::PI { self.dir -= 2.0 * std::f32::consts::PI; }
        if self.dir < -std::f32::consts::PI { self.dir += 2.0 * std::f32::consts::PI; }

        // Update speed based on steering force and drag
        let force_mag = steering.length();
        self.speed += (force_mag - DRAG * self.speed) * d.get_frame_time();
        self.speed = self.speed.clamp(self.min_speed, self.max_speed);

        let dir_vec = Vector2 {
            x: self.dir.cos(),
            y: self.dir.sin()
        };

        //println!("dir: {:#?}", dir_vec * self.speed * d.get_frame_time());

        self.pos += dir_vec * self.speed * d.get_frame_time();
    }

    pub fn draw(&mut self, d: &mut RaylibDrawHandle) {
        draw_oriented_triangle(d, self.pos, self.dir, 10.0, self.color);
    }
}

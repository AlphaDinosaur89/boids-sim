use std::f32::consts::PI;

use rand::Rng;
use raylib::prelude::*;
use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};

use crate::fish::*;

mod fish;
mod math;
mod render;

fn generate_fishes(amount: i32, width: i32, height: i32) -> Vec<Fish> {
    if amount < 0 { return vec![]; }
    let mut fishes: Vec<Fish> = vec![];

    let mut rng = rand::rng();

    for i in 0..amount {
        let x: i32 = rng.random_range(0..=width);
        let y: i32 = rng.random_range(0..=height);
        fishes.push(Fish::new(
            i,
            Vector2 {
                x: x as f32,
                y: y as f32
            },
            rng.random_range(0.0..=PI * 2.0), // 360 in rad pi * 2
            50.0,
            200.0,
            Color::BEIGE
        ))
    };

    fishes
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(1280, 720)
        .title("raylib [rust] example - basic window")
        .msaa_4x()
        .vsync()
        .resizable()
        .build();

    let mut fishes: Vec<Fish> = generate_fishes(1000, rl.get_screen_width(), rl.get_screen_height());
    
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::NAVY);
        //d.draw_fps(10, 10);

        let the_fishes = fishes.clone();
        fishes.par_iter_mut().for_each(|fish| {
            fish.update(&d, &the_fishes);
        });

        for fish in &mut fishes {
            fish.draw(&mut d);
        }
    }
}

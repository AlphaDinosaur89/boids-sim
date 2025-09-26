use raylib::prelude::*;

pub fn draw_oriented_triangle(d: &mut RaylibDrawHandle, pos: Vector2, dir: f32, size: f32, color: Color) {
    let half_base = size * 0.5;
    let height = size;

    let forward = Vector2::new(dir.cos(), dir.sin());

    let right = Vector2::new(-forward.y, forward.x);

    let tip = pos + forward * height;
    let left = pos - forward * (height * 0.3) - right * half_base;
    let right_p = pos - forward * (height * 0.3) + right * half_base;

    d.draw_triangle(tip, left, right_p, color);
}

// Very slow, dont use
/*
pub fn draw_trail(d: &mut RaylibDrawHandle, trail: &[Vector2], pos: Vector2) {
    for i in (0..trail.len()).step_by(2) {
        if i + 1 > trail.len() - 1 {
            let point = trail[i];
            d.draw_line(
                point.x as i32,
                point.y as i32,
                pos.x as i32,
                pos.y as i32,
                Color::RED
            );
        } else {
            let point = trail[i];
            let next_point = trail[i+1];
            d.draw_line(
                point.x as i32,
                point.y as i32,
                next_point.x as i32,
                next_point.y as i32,
                Color::RED
            );
        }
    }
}
*/

use raylib::prelude::*;

pub fn away_from_points(pos: Vector2, points: &[Vector2], width: i32, height: i32) -> Vector2 {
    let mut result = Vector2::zero();
    for p in points {
        let dir = pos - *p;
        if dir.length_sqr() > 0.0 {
            result += dir.normalized() / dir.length_sqr(); // weighted repulsion
        }
    }

    // Wall repulsion
    let margin = 20.0;
    let wall_strength = 1.0;
    // Left wall
    if pos.x < margin {
        let dist = pos.x.max(1.0); // avoid div by zero
        result.x += wall_strength / (dist * dist);
    }
    // Right wall
    if pos.x > width as f32 - margin {
        let dist = (width as f32 - pos.x).max(1.0);
        result.x -= wall_strength / (dist * dist);
    }
    // Top wall
    if pos.y < margin {
        let dist = pos.y.max(1.0);
        result.y += wall_strength / (dist * dist);
    }
    // Bottom wall
    if pos.y > height as f32 - margin {
        let dist = (height as f32 - pos.y).max(1.0);
        result.y -= wall_strength / (dist * dist);
    }

    if result.length_sqr() > 0.0 {
        result.normalized()
    } else {
        Vector2::zero()
    }
}

pub fn average_angle(angles: &[f32]) -> f32 {
    let mut sum_x = 0.0;
    let mut sum_y = 0.0;

    for &a in angles {
        sum_x += a.cos();
        sum_y += a.sin();
    }

    sum_y.atan2(sum_x)
}

pub fn center_of_mass(positions: &[Vector2]) -> Vector2 {
    let mut average = Vector2::zero();

    for position in positions {
        average += *position;
    }
    average.x /= positions.len() as f32;
    average.y /= positions.len() as f32;

    return average;
}

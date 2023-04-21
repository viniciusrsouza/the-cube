#[allow(dead_code)]
pub fn linear(dt: f32) -> f32 {
    dt
}

pub fn ease_in(dt: f32) -> f32 {
    dt * dt
}

pub fn ease_out(dt: f32) -> f32 {
    1.0 - (1.0 - dt) * (1.0 - dt)
}

#[allow(dead_code)]
pub fn ease_in_out(dt: f32) -> f32 {
    if dt < 0.5 {
        ease_in(dt * 2.0) / 2.0
    } else {
        ease_out(dt * 2.0 - 1.0) / 2.0 + 0.5
    }
}

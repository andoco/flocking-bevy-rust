use bevy::prelude::*;

pub fn signed_angle(a: Vec2, b: Vec2) -> f32 {
    (a.x * b.y - a.y * b.x).atan2(a.x * b.x + a.y * b.y)
}

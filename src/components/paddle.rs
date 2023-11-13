use bevy::math::Vec2;
use bevy::prelude::{Color, Component};

use crate::components::wall::BOTTOM_WALL_Y;

pub const PADDLE_START_Y: f32 = BOTTOM_WALL_Y + 50.0;
pub const PADDLE_SIZE: Vec2 = Vec2::new(120.0, 20.0);
pub const PADDLE_COLOR: Color = Color::rgb(0.3, 0.3, 0.7);
pub const PADDLE_SPEED: f32 = 500.0;

#[derive(Component)]
pub(crate) struct Paddle;
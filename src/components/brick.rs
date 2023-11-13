use bevy::math::Vec2;
use bevy::prelude::{Color, Component};

pub const BRICK_SIZE: Vec2 = Vec2::new(100., 30.);
pub const BRICK_COLOR: Color = Color::rgb(0.7, 0.7, 0.3);
pub const GAP_BETWEEN_BRICKS: f32 = 5.;
pub const GAP_BETWEEN_PADDLE_AND_BRICKS: f32 = 270.;
pub const GAP_BETWEEN_TOP_OF_SCREEN_AND_BRICKS: f32 = 20.;
pub const GAP_BETWEEN_LEFT_OF_SCREEN_AND_BRICKS: f32 = 30.;

#[derive(Component)]
pub struct Brick;
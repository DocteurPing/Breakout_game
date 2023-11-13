use bevy::math::Vec2;
use bevy::prelude::{Bundle, Color, Component, SpriteBundle};

pub const LEFT_WALL_X: f32 = -450.;
pub const RIGHT_WALL_X: f32 = 450.;
pub const TOP_WALL_Y: f32 = 300.;
pub const BOTTOM_WALL_Y: f32 = -300.;
pub const WALL_THICKNESS: f32 = 20.;
pub const WALL_COLOR: Color = Color::rgb(0.3, 0.7, 0.3);
pub const WALL_BLOCK_WIDTH: f32 = RIGHT_WALL_X - LEFT_WALL_X;
pub const WALL_BLOCK_HEIGHT: f32 = TOP_WALL_Y - BOTTOM_WALL_Y;

#[derive(Component)]
pub struct Collider {
    pub(crate) size: Vec2,
}

#[derive(Bundle)]
pub struct WallBundle {
    pub(crate) sprite: SpriteBundle,
    pub(crate) collider: Collider,
}


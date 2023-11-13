use bevy::math::{Vec2, Vec3};
use bevy::prelude::{Color, Component, Deref, DerefMut};

pub const BALL_COLOR: Color = Color::rgb(0.7, 0.3, 0.3);
pub const BALL_STARTING_POSITION: Vec3 = Vec3::new(0., -50., 1.);
pub const BALL_SIZE: Vec2 = Vec2::new(20., 20.);
pub const BALL_SPEED: f32 = 500.;
pub const BALL_STARTING_DIRECTION: Vec2 = Vec2::new(0.5, -0.5);

#[derive(Component, Deref, DerefMut)]
pub struct Ball {
    pub size: Vec2,
    #[deref]
    pub speed: Vec2,
}
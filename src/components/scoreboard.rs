use bevy::prelude::{Color, Resource, Val};

pub const SCOREBOARD_FONT_SIZE: f32 = 50.0;
pub const SCOREBOARD_FONT_COLOR: Color = Color::rgb(0.7, 0.7, 0.7);
pub const SCOREBOARD_TEXT_PADDING: Val = Val::Px(5.0);
pub const SCORE_COLOR: Color = Color::rgb(0.7, 0.7, 0.7);

#[derive(Resource, Clone, Copy)]
pub struct Scoreboard {
    pub(crate) score: usize,
}
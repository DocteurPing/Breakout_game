use bevy::math::vec3;
use bevy::prelude::*;
use crate::components::paddle::Paddle;

mod components {
    pub mod paddle;
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: vec3(0., components::paddle::PADDLE_START_Y, 0.),
                ..default()
            },
            sprite: Sprite {
                color: components::paddle::PADDLE_COLOR,
                custom_size: Some(components::paddle::PADDLE_SIZE),
                ..default()
            },
            ..default()
        },
        Paddle,
    ));
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9))) // Set the background color
        .add_systems(Update, bevy::window::close_on_esc)
        .add_systems(Startup, setup)
        .run();
}

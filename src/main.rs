use bevy::math::vec3;
use bevy::prelude::*;
use crate::components::ball::*;
use crate::components::paddle::Paddle;
use crate::components::paddle::*;

mod components {
    pub mod paddle;
    pub mod ball;
}

fn setup(mut commands: Commands, asser_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: vec3(0., PADDLE_START_Y, 0.),
                ..default()
            },
            sprite: Sprite {
                color: PADDLE_COLOR,
                custom_size: Some(PADDLE_SIZE),
                ..default()
            },
            ..default()
        },
        Paddle,
    ));
    let ball_texture_handle = asser_server.load("textures/ball.png");
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: BALL_STARTING_POSITION,
                ..default()
            },
            sprite: Sprite {
                color: BALL_COLOR,
                custom_size: Some(BALL_SIZE),
                ..default()
            },
            texture: ball_texture_handle,
            ..default()
        },
        Ball,
        Velocity(BALL_SPEED * Vec2 { x: 0.0, y: 0.0 }),
    ));
}

fn update_paddle(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Paddle, &mut Transform)>,
) {
    for (_, mut transform) in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::Left) {
            transform.translation.x -= PADDLE_SPEED * 0.02;
        }
        if keyboard_input.pressed(KeyCode::Right) {
            transform.translation.x += PADDLE_SPEED * 0.02;
        }
    }
}

fn update_ball(mut query: Query<(&Ball, &mut Transform, &mut Velocity)>) {
    for (_, mut transform, velocity) in query.iter_mut() {
        transform.translation.x += velocity.x * 0.02;
        transform.translation.y += velocity.y * 0.02;
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1))) // Set the background color
        .add_systems(Update, bevy::window::close_on_esc)
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, (update_paddle, update_ball))
        .run();
}

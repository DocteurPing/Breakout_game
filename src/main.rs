use bevy::math::{vec2, vec3};
use bevy::prelude::*;

use crate::components::ball::*;
use crate::components::paddle::*;
use crate::components::wall::*;

mod components {
    pub mod paddle;
    pub mod ball;
    pub mod wall;
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

    let vertical_wall_size = vec2(WALL_THICKNESS, WALL_BLOCK_HEIGHT + WALL_THICKNESS);
    let horizontal_wall_size = vec2(WALL_BLOCK_WIDTH + WALL_THICKNESS, WALL_THICKNESS);
    spawn_walls(&mut commands, vertical_wall_size, vec3(LEFT_WALL_X, 0.0, 0.0));
    spawn_walls(&mut commands, vertical_wall_size, vec3(RIGHT_WALL_X, 0.0, 0.0));
    spawn_walls(&mut commands, horizontal_wall_size, vec3(0.0, BOTTOM_WALL_Y, 0.0));
    spawn_walls(&mut commands, horizontal_wall_size, vec3(0.0, TOP_WALL_Y, 0.0));
}

fn spawn_walls(commands: &mut Commands, size: Vec2, translation: Vec3) {
    commands.spawn(WallBundle {
        sprite: SpriteBundle {
            transform: Transform {
                translation,
                ..default()
            },
            sprite: Sprite {
                color: WALL_COLOR,
                custom_size: Some(size),
                ..default()
            },
            ..default()
        },
        collider: Collider {
            size,
        },
    });
}

fn update_paddle(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Paddle, &mut Transform)>,
) {
    for (_, mut transform) in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::Left) {
            if (transform.translation.x - PADDLE_SIZE.x / 2.) > LEFT_WALL_X + WALL_THICKNESS / 2.0 + 1.0 {
                transform.translation.x -= PADDLE_SPEED * 0.02;
            }
        }
        if keyboard_input.pressed(KeyCode::Right) {
            if (transform.translation.x + PADDLE_SIZE.x / 2.) < RIGHT_WALL_X - WALL_THICKNESS / 2.0 - 1.0 {
                transform.translation.x += PADDLE_SPEED * 0.02;
            }
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

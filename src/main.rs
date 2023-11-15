use bevy::math::{vec2, vec3};
use bevy::prelude::*;
use bevy::sprite::collide_aabb::{collide, Collision};

use crate::components::ball::*;
use crate::components::brick::*;
use crate::components::paddle::*;
use crate::components::scoreboard::*;
use crate::components::wall::*;

mod components {
    pub mod paddle;
    pub mod ball;
    pub mod wall;
    pub mod brick;
    pub mod scoreboard;
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
        Collider {
            size: PADDLE_SIZE,
        },
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
        Ball { size: BALL_SIZE, speed: BALL_SPEED * Vec2::ZERO },
    ));

    let vertical_wall_size = vec2(WALL_THICKNESS, WALL_BLOCK_HEIGHT + WALL_THICKNESS);
    let horizontal_wall_size = vec2(WALL_BLOCK_WIDTH + WALL_THICKNESS, WALL_THICKNESS);
    spawn_walls(&mut commands, vertical_wall_size, vec3(LEFT_WALL_X, 0.0, 0.0));
    spawn_walls(&mut commands, vertical_wall_size, vec3(RIGHT_WALL_X, 0.0, 0.0));
    spawn_walls(&mut commands, horizontal_wall_size, vec3(0.0, BOTTOM_WALL_Y, 0.0));
    spawn_walls(&mut commands, horizontal_wall_size, vec3(0.0, TOP_WALL_Y, 0.0));

    spawn_bricks(&mut commands);

    commands.spawn((TextBundle::from_sections([
        TextSection::new(
            "Score: ",
            TextStyle {
                font_size: SCOREBOARD_FONT_SIZE,
                color: SCOREBOARD_FONT_COLOR,
                ..default()
            },
        ),
        TextSection::from_style(TextStyle {
            font_size: SCOREBOARD_FONT_SIZE,
            color: SCORE_COLOR,
            ..default()
        }),
    ]).with_style(Style {
        position_type: PositionType::Absolute,
        top: SCOREBOARD_TEXT_PADDING,
        left: SCOREBOARD_TEXT_PADDING,
        ..default()
    }), ));
}

fn spawn_bricks(commands: &mut Commands) {
    {
        let offset_x = LEFT_WALL_X + GAP_BETWEEN_LEFT_OF_SCREEN_AND_BRICKS + BRICK_SIZE.x * 0.5;
        let offset_y = BOTTOM_WALL_Y + GAP_BETWEEN_PADDLE_AND_BRICKS + BRICK_SIZE.y * 0.5;

        let bricks_total_width = (RIGHT_WALL_X - LEFT_WALL_X) - 2. * GAP_BETWEEN_LEFT_OF_SCREEN_AND_BRICKS;
        let bricks_total_height = (TOP_WALL_Y - BOTTOM_WALL_Y)
            - GAP_BETWEEN_TOP_OF_SCREEN_AND_BRICKS
            - GAP_BETWEEN_PADDLE_AND_BRICKS;

        let rows = (bricks_total_height / (BRICK_SIZE.y + GAP_BETWEEN_BRICKS)).floor() as i32;
        let columns = (bricks_total_width / (BRICK_SIZE.x + GAP_BETWEEN_BRICKS)).floor() as i32;

        for row in 0..rows {
            for column in 0..columns {
                let brick_pos = vec2(
                    offset_x + column as f32 * (BRICK_SIZE.x + GAP_BETWEEN_BRICKS),
                    offset_y + row as f32 * (BRICK_SIZE.y + GAP_BETWEEN_BRICKS),
                );

                commands.spawn((
                    SpriteBundle {
                        transform: Transform {
                            translation: brick_pos.extend(0.0),
                            ..default()
                        },
                        sprite: Sprite {
                            color: BRICK_COLOR,
                            custom_size: Some(BRICK_SIZE),
                            ..default()
                        },
                        ..default()
                    },
                    Brick,
                    Collider { size: BRICK_SIZE },
                ));
            }
        }
    }
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
        if keyboard_input.pressed(KeyCode::Left) && (transform.translation.x - PADDLE_SIZE.x / 2.) > LEFT_WALL_X + WALL_THICKNESS / 2.0 + 1.0 {
            transform.translation.x -= PADDLE_SPEED * 0.02;
        }
        if keyboard_input.pressed(KeyCode::Right) && (transform.translation.x + PADDLE_SIZE.x / 2.) < RIGHT_WALL_X - WALL_THICKNESS / 2.0 - 1.0 {
            transform.translation.x += PADDLE_SPEED * 0.02;
        }
    }
}

fn check_ball_collisions(
    mut command: Commands,
    mut scoreboard: ResMut<Scoreboard>,
    mut ball_query: Query<(&Transform, &mut Ball)>,
    mut collider_query: Query<(Entity, &Transform, &Collider, Option<&Brick>)>, // Note the mutability for Brick
) {
    for (ball_transform, mut ball) in &mut ball_query {
        for (other_entity, transform, other, opt_brick) in &mut collider_query {
            let collision = collide(
                ball_transform.translation,
                ball.size,
                transform.translation,
                other.size,
            );

            let mut reflect_x = false;
            let mut reflect_y = false;
            if let Some(collision) = collision {
                match collision {
                    Collision::Left => reflect_x = ball.speed.x > 0.0,
                    Collision::Right => reflect_x = ball.speed.x < 0.0,
                    Collision::Top => reflect_y = ball.speed.y < 0.0,
                    Collision::Bottom => reflect_y = ball.speed.y > 0.0,
                    Collision::Inside => { /* do nothing */ }
                }

                if reflect_x {
                    ball.speed.x *= -1.;
                }
                if reflect_y {
                    ball.speed.y *= -1.;
                }

                if opt_brick.is_some() {
                    scoreboard.score += 1;
                    command.entity(other_entity).despawn();
                }
            }
        }
    }
}

fn update_ball(mut query: Query<(&mut Ball, &mut Transform)>) {
    for (ball, mut transform) in query.iter_mut() {
        transform.translation.x += ball.speed.x * 0.02;
        transform.translation.y += ball.speed.y * 0.02;
    }
}

fn update_scoreboard(scoreboard: Res<Scoreboard>, mut query: Query<&mut Text>) {
    for mut text in query.iter_mut() {
        text.sections[1].value = scoreboard.score.to_string();
    }
}

fn launch_game(
    mut commands: Commands,
    mut scoreboard: ResMut<Scoreboard>,
    keyboard_input: Res<Input<KeyCode>>,
    mut ball_query: Query<(&mut Ball, &mut Transform)>,
    mut bricks_query: Query<(Entity, Option<&Brick>)>,
) {
    for (mut ball, mut transform) in ball_query.iter_mut() {
        if keyboard_input.pressed(KeyCode::Space) {
            for (brick, option) in bricks_query.iter_mut() {
                if option.is_some() {
                    commands.entity(brick).despawn();
                }
            }
            spawn_bricks(&mut commands);
            scoreboard.score = 0;
            transform.translation = BALL_STARTING_POSITION;
            ball.speed = BALL_SPEED * BALL_STARTING_DIRECTION;
        }
    }
}


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1))) // Set the background color
        .insert_resource(Scoreboard { score: 0 })
        .add_systems(Update, (bevy::window::close_on_esc, update_scoreboard, launch_game))
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, (update_paddle, update_ball, check_ball_collisions.after(update_ball)))
        .run();
}

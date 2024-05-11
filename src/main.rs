use bevy::{prelude::*, render::camera::ScalingMode};
use collision::{CollisionPlugin, CollisionSet};
use constants::*;
use food::{EatSet, FoodPlugin};
use movement::{MovePlugin, SnakeMoveSet, SnakeMoveTimerTickSet};
use play_area::PlayAreaPlugin;

mod collision;
mod constants;
mod food;
mod movement;
mod play_area;

#[derive(Component)]
struct CurrentDirection(Direction2d);

#[derive(Component)]
pub struct NextDirection(Direction2d);

#[derive(Component)]
pub struct SnakeHead;

#[derive(Resource)]
pub struct IsDead(bool);

#[derive(Resource)]
pub struct SnakeBody(Vec<Entity>);

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            CollisionPlugin,
            FoodPlugin,
            MovePlugin,
            PlayAreaPlugin,
        ))
        .add_systems(Startup, (setup_camera, setup_clear_color, spawn_snake))
        .add_systems(Update, change_head_direction)
        .configure_sets(
            Update,
            (SnakeMoveTimerTickSet, CollisionSet, EatSet, SnakeMoveSet).chain(),
        )
        .insert_resource(IsDead(false))
        .run();
}

fn change_head_direction(
    mut q: Query<(&CurrentDirection, &mut NextDirection), With<SnakeHead>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    let desired = if input.just_pressed(KeyCode::KeyW) {
        Direction2d::Y
    } else if input.just_pressed(KeyCode::KeyA) {
        Direction2d::NEG_X
    } else if input.just_pressed(KeyCode::KeyS) {
        Direction2d::NEG_Y
    } else if input.just_pressed(KeyCode::KeyD) {
        Direction2d::X
    } else {
        return;
    };

    let (current, mut next) = q.single_mut();

    // Don't allow 180's
    if desired != -current.0 {
        next.0 = desired;
    }
}

fn setup_camera(mut cmd: Commands) {
    let mut camera = Camera2dBundle::default();
    camera.projection.scaling_mode = ScalingMode::FixedHorizontal(GRID_WIDTH);
    // Pan the camera so that (0,0) is in the bottom left, assuming a 16x9 aspect ratio
    camera.transform = Transform::from_xyz(GRID_WIDTH / 2. - 0.5, GRID_HEIGHT / 2. - 0.5, 0.);
    cmd.spawn(camera);
}

fn setup_clear_color(mut cmd: Commands) {
    cmd.insert_resource(ClearColor(Color::GRAY));
}

fn spawn_snake(mut cmd: Commands) {
    let sprite = Sprite {
        color: Color::WHITE,
        custom_size: Some(Vec2::ONE),
        ..default()
    };
    let transforms: Vec<_> = (0..SNAKE_LENGTH)
        .map(|i| Transform::from_xyz(GRID_WIDTH / 2. - i as f32, GRID_HEIGHT / 2., 0.))
        .collect();

    let mut body = vec![];

    body.push(
        cmd.spawn((
            SpriteBundle {
                sprite: sprite.clone(),
                transform: transforms[0],
                ..default()
            },
            SnakeHead,
            CurrentDirection(Direction2d::X),
            NextDirection(Direction2d::X),
        ))
        .id(),
    );
    for transform in transforms.into_iter().skip(1) {
        body.push(
            cmd.spawn(SpriteBundle {
                sprite: sprite.clone(),
                transform,
                ..default()
            })
            .id(),
        );
    }
    cmd.insert_resource(SnakeBody(body));
}

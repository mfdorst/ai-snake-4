use bevy::{prelude::*, render::camera::ScalingMode};

const GRID_WIDTH: f32 = 32.;
const GRID_HEIGHT: f32 = 18.;
const STEPS_PER_SECOND: f32 = 4.;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Direction(Direction2d);

#[derive(Resource)]
struct MoveTimer(Timer);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(
            Startup,
            (
                setup_camera,
                setup_play_area,
                setup_clear_color,
                spawn_player,
            ),
        )
        .add_systems(Update, (change_player_direction, move_player))
        .insert_resource(MoveTimer(Timer::from_seconds(
            1. / STEPS_PER_SECOND,
            TimerMode::Repeating,
        )))
        .run();
}

fn setup_camera(mut cmd: Commands) {
    let mut camera = Camera2dBundle::default();
    camera.projection.scaling_mode = ScalingMode::FixedHorizontal(GRID_WIDTH as f32);
    // Pan the camera so that (0,0) is in the bottom left, assuming a 16x9 aspect ratio
    camera.transform = Transform::from_xyz(GRID_WIDTH / 2. - 0.5, GRID_HEIGHT / 2. - 0.5, 0.);
    cmd.spawn(camera);
}

fn setup_clear_color(mut cmd: Commands) {
    cmd.insert_resource(ClearColor(Color::GRAY));
}

fn setup_play_area(mut cmd: Commands) {
    cmd.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::BLACK,
            custom_size: Some(Vec2::new(GRID_WIDTH, GRID_HEIGHT)),
            ..default()
        },
        // Move the play area so the bottom left corner is at (0,0)
        transform: Transform::from_xyz(GRID_WIDTH / 2. - 0.5, GRID_HEIGHT / 2. - 0.5, 0.),
        ..default()
    });
}

fn spawn_player(mut cmd: Commands) {
    cmd.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::new(1., 1.)),
                ..default()
            },
            ..default()
        },
        Player,
        Direction(Direction2d::X),
    ));
}

fn change_player_direction(
    input: Res<ButtonInput<KeyCode>>,
    mut q: Query<&mut Direction, With<Player>>,
) {
    let mut direction = q.single_mut();

    if input.just_pressed(KeyCode::KeyW) {
        direction.0 = Direction2d::Y;
    } else if input.just_pressed(KeyCode::KeyA) {
        direction.0 = Direction2d::NEG_X;
    } else if input.just_pressed(KeyCode::KeyS) {
        direction.0 = Direction2d::NEG_Y;
    } else if input.just_pressed(KeyCode::KeyD) {
        direction.0 = Direction2d::X;
    }
}

fn move_player(
    mut q: Query<(&mut Transform, &Direction), With<Player>>,
    mut timer: ResMut<MoveTimer>,
    time: Res<Time>,
) {
    timer.0.tick(time.delta());

    if timer.0.finished() {
        let (mut transform, direction) = q.single_mut();
        transform.translation += direction.0.extend(0.);
    }
}

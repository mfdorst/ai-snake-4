use bevy::{prelude::*, render::camera::ScalingMode};

const GRID_WIDTH: f32 = 32.;
const GRID_HEIGHT: f32 = 18.;

#[derive(Component)]
struct Player;

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
        .add_systems(Update, move_player)
        .run();
}

fn setup_camera(mut cmd: Commands) {
    let mut camera = Camera2dBundle::default();
    // Set the viewport to be 32 units wide
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
    ));
}

fn move_player(input: Res<ButtonInput<KeyCode>>, mut q: Query<&mut Transform, With<Player>>) {
    let mut xform = q.single_mut();
    if input.just_pressed(KeyCode::KeyW) {
        xform.translation.y += 1.;
    } else if input.just_pressed(KeyCode::KeyA) {
        xform.translation.x -= 1.;
    } else if input.just_pressed(KeyCode::KeyS) {
        xform.translation.y -= 1.;
    } else if input.just_pressed(KeyCode::KeyD) {
        xform.translation.x += 1.;
    }
}

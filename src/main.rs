use bevy::{
    prelude::*,
    window::{PrimaryWindow, WindowResized},
};

const GRID_HEIGHT: u8 = 18;
const GRID_WIDTH: u8 = 32;

#[derive(Debug, Component)]
struct PlayArea(f32, f32);

#[derive(Component)]
struct Cell;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup_camera, setup_play_area, setup_clear_color))
        .add_systems(Startup, spawn_grid_cells_in_corners.after(setup_play_area))
        .add_systems(Update, resize_play_area)
        .run();
}

fn setup_camera(mut cmd: Commands) {
    cmd.spawn(Camera2dBundle::default());
}

fn setup_clear_color(mut cmd: Commands) {
    cmd.insert_resource(ClearColor(Color::GRAY));
}

fn setup_play_area(mut cmd: Commands, q: Query<&Window, With<PrimaryWindow>>) {
    let res = &q.single().resolution;
    cmd.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::BLACK,
                custom_size: Some(Vec2::new(0., 0.)),
                ..default()
            },
            ..default()
        },
        get_play_area(res.width(), res.height()),
    ));
}

fn resize_play_area(
    mut q: Query<(&mut Sprite, &mut PlayArea)>,
    mut resize_reader: EventReader<WindowResized>,
) {
    let (mut sprite, mut play_area) = q.single_mut();
    for e in resize_reader.read() {
        *play_area = get_play_area(e.width, e.height);
        sprite.custom_size = Some(Vec2::new(play_area.0, play_area.1));
    }
}

fn get_play_area(window_width: f32, window_height: f32) -> PlayArea {
    if window_width < 16. * window_height / 9. {
        PlayArea(window_width, 9. * window_width / 16.)
    } else {
        PlayArea(16. * window_height / 9., window_height)
    }
}

fn spawn_cell(cmd: &mut Commands, play_area: &PlayArea, x: u8, y: u8) {
    let cell_width = play_area.0 / GRID_WIDTH as f32;
    let cell_height = play_area.1 / GRID_HEIGHT as f32;
    let x_pos = -play_area.0 / 2. + (x as f32 + 0.5) * cell_width;
    let y_pos = -play_area.1 / 2. + (y as f32 + 0.5) * cell_height;
    cmd.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::new(cell_width, cell_height)),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(x_pos, y_pos, 1.),
                ..default()
            },
            ..default()
        },
        Cell,
    ));
}

fn spawn_grid_cells_in_corners(mut cmd: Commands, play_area_q: Query<&PlayArea>) {
    let play_area = play_area_q.single();
    spawn_cell(&mut cmd, play_area, 0, 0);
    spawn_cell(&mut cmd, play_area, 31, 17);
}

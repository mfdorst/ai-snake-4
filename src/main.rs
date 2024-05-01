use bevy::{prelude::*, window::WindowResized};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup_camera, setup_play_area, setup_clear_color))
        .add_systems(Update, on_resize_system)
        .run();
}

#[derive(Component)]
struct PlayArea;

fn setup_camera(mut cmd: Commands) {
    cmd.spawn(Camera2dBundle::default());
}

fn setup_clear_color(mut cmd: Commands) {
    cmd.insert_resource(ClearColor(Color::GRAY));
}

fn setup_play_area(mut cmd: Commands) {
    cmd.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::BLACK,
                custom_size: Some(Vec2::new(0., 0.)),
                ..default()
            },
            ..default()
        },
        PlayArea,
    ));
}

fn on_resize_system(
    mut q: Query<&mut Sprite, With<PlayArea>>,
    mut resize_reader: EventReader<WindowResized>,
) {
    let mut sprite = q.single_mut();
    for e in resize_reader.read() {
        let (width, height) = if e.width < 16. * e.height / 9. {
            (e.width, 9. * e.width / 16.)
        } else {
            (16. * e.height / 9., e.height)
        };
        sprite.custom_size = Some(Vec2::new(width, height));
    }
}

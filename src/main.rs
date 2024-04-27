use bevy::{prelude::*, window::WindowResized};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // Add setup handlers that will be run once
        .add_systems(Startup, (setup_camera, setup_play_area, setup_clear_color))
        // Add the resize handler to be run on every update
        .add_systems(Update, on_resize_system)
        .run();
}

// Marker struct that tags the play area sprite
#[derive(Component)]
struct PlayArea;

// Spawns the camera
fn setup_camera(mut cmd: Commands) {
    cmd.spawn(Camera2dBundle::default());
}

// Sets the background to gray
fn setup_clear_color(mut cmd: Commands) {
    cmd.insert_resource(ClearColor(Color::GRAY));
}

// Spawns a black sprite to show the play area
fn setup_play_area(mut cmd: Commands) {
    cmd.spawn((
        // Spawn a sprite that will show as a black rectangle
        SpriteBundle {
            sprite: Sprite {
                color: Color::BLACK,
                custom_size: Some(Vec2::new(0., 0.)),
                ..default()
            },
            ..default()
        },
        // Tag it as the play area sprite
        PlayArea,
    ));
}

/// This system responds to the window being resized.
fn on_resize_system(
    // Query for the sprite with a PlayArea attached
    mut q: Query<&mut Sprite, With<PlayArea>>,
    // Get all window resize events that have occured since the last update
    mut resize_reader: EventReader<WindowResized>,
) {
    // Get the sprite with a PlayArea attached (there should only be one)
    let mut sprite = q.single_mut();
    // Loop through all resize events that have occurred
    for e in resize_reader.read() {
        // Figure out the correct width and height for the play area
        let (width, height) = if e.width < 16. * e.height / 9. {
            // Window is taller than 16x9
            (e.width, 9. * e.width / 16.)
        } else {
            // Window is shorter than 16x9
            (16. * e.height / 9., e.height)
        };
        // Set the sprite to those dimensions
        sprite.custom_size = Some(Vec2::new(width, height));
    }
}

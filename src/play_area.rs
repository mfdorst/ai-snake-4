use crate::constants::*;
use bevy::prelude::*;

pub struct PlayAreaPlugin;

impl Plugin for PlayAreaPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_play_area);
    }
}

fn setup_play_area(mut cmd: Commands) {
    cmd.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::BLACK,
            custom_size: Some(Vec2::new(GRID_WIDTH, GRID_HEIGHT)),
            ..default()
        },
        // Move the play area so the bottom left corner is at (0,0)
        transform: Transform::from_xyz(GRID_WIDTH / 2. - 0.5, GRID_HEIGHT / 2. - 0.5, -1.),
        ..default()
    });
}

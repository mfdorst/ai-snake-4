use bevy::prelude::*;
use camera::CameraPlugin;
use collision::{CollisionPlugin, CollisionSet};
use food::{EatSet, FoodPlugin};
use input::InputPlugin;
use snake::{SnakeMoveSet, SnakeMoveTimerTickSet, SnakePlugin};
use score::ScorePlugin;

mod camera;
mod collision;
mod constants;
mod food;
mod input;
mod score;
mod snake;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            CameraPlugin,
            CollisionPlugin,
            FoodPlugin,
            InputPlugin,
            ScorePlugin,
            SnakePlugin,
        ))
        .configure_sets(
            Update,
            (SnakeMoveTimerTickSet, CollisionSet, EatSet, SnakeMoveSet).chain(),
        )
        .run();
}

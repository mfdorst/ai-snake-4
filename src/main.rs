use autopilot::{AutopilotPlugin, AutopilotSet};
use bevy::prelude::*;
use camera::CameraPlugin;
use collision::{CollisionPlugin, CollisionSet};
use food::{EatSet, FoodPlugin};
use input::InputPlugin;
use pause::PausePlugin;
use score::ScorePlugin;
use snake::{SnakeMoveSet, SnakeMoveTimerTickSet, SnakePlugin};

mod autopilot;
mod camera;
mod collision;
mod constants;
mod food;
mod input;
mod pause;
mod score;
mod snake;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            AutopilotPlugin,
            CameraPlugin,
            CollisionPlugin,
            FoodPlugin,
            InputPlugin,
            PausePlugin,
            ScorePlugin,
            SnakePlugin,
        ))
        .configure_sets(
            Update,
            (
                SnakeMoveTimerTickSet,
                AutopilotSet,
                CollisionSet,
                EatSet,
                SnakeMoveSet,
            )
                .chain(),
        )
        .run();
}

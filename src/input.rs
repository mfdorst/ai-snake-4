use crate::snake::SnakeHead;
use bevy::prelude::*;

pub struct InputPlugin;

#[derive(Component)]
pub struct CurrentDirection(pub Direction2d);

#[derive(Component)]
pub struct NextDirection(pub Direction2d);

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, change_head_direction);
    }
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

use bevy::prelude::*;

pub struct InputPlugin;

#[derive(Resource)]
pub struct CurrentDirection(pub Direction2d);

#[derive(Resource)]
pub struct NextDirection(pub Direction2d);

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, change_head_direction)
            .insert_resource(CurrentDirection(Direction2d::X))
            .insert_resource(NextDirection(Direction2d::X));
    }
}

fn change_head_direction(
    current_direction: Res<CurrentDirection>,
    mut next_direction: ResMut<NextDirection>,
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

    // Don't allow 180's
    if desired != -current_direction.0 {
        next_direction.0 = desired;
    }
}

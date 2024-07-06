use bevy::prelude::*;

pub struct InputPlugin;

#[derive(Resource)]
pub struct CurrentDirection(pub Dir2);

#[derive(Resource)]
pub struct NextDirection(pub Dir2);

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, change_head_direction)
            .insert_resource(CurrentDirection(Dir2::X))
            .insert_resource(NextDirection(Dir2::X));
    }
}

fn change_head_direction(
    current_direction: Res<CurrentDirection>,
    mut next_direction: ResMut<NextDirection>,
    input: Res<ButtonInput<KeyCode>>,
) {
    let desired = if input.just_pressed(KeyCode::KeyW) {
        Dir2::Y
    } else if input.just_pressed(KeyCode::KeyA) {
        Dir2::NEG_X
    } else if input.just_pressed(KeyCode::KeyS) {
        Dir2::NEG_Y
    } else if input.just_pressed(KeyCode::KeyD) {
        Dir2::X
    } else {
        return;
    };

    // Don't allow 180's
    if desired != -current_direction.0 {
        next_direction.0 = desired;
    }
}

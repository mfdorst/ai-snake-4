use bevy::prelude::*;

pub struct PausePlugin;

#[derive(Component)]
struct PauseButtonText;

#[derive(Resource)]
pub struct IsPaused(pub bool);

impl Plugin for PausePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_pause_button)
            .add_systems(
                Update,
                (handle_pause_button_click, toggle_pause, update_pause_button),
            )
            .insert_resource(IsPaused(false));
    }
}

fn setup_pause_button(mut cmd: Commands) {
    cmd.spawn(ButtonBundle {
        style: Style {
            flex_basis: Val::Px(150.),
            flex_shrink: 0.,
            position_type: PositionType::Absolute,
            left: Val::Px(10.),
            bottom: Val::Px(52.),
            padding: UiRect::all(Val::Px(5.)),
            ..default()
        },
        background_color: Srgba::BLACK.with_alpha(0.8).into(),
        ..default()
    })
    .with_children(|parent| {
        parent
            .spawn(TextBundle::from_section(
                "Pause",
                TextStyle {
                    font_size: 30.,
                    color: Srgba::WHITE.into(),
                    ..default()
                },
            ))
            .insert(PauseButtonText);
    });
}

fn handle_pause_button_click(
    mut is_paused: ResMut<IsPaused>,
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<Button>)>,
) {
    for &interaction in &mut interaction_query {
        if interaction == Interaction::Pressed {
            is_paused.0 = !is_paused.0;
        }
    }
}

fn toggle_pause(mut is_paused: ResMut<IsPaused>, input: Res<ButtonInput<KeyCode>>) {
    if input.just_pressed(KeyCode::Space) {
        is_paused.0 = !is_paused.0;
    }
}

fn update_pause_button(
    is_paused: Res<IsPaused>,
    mut query: Query<&mut Text, With<PauseButtonText>>,
) {
    let mut text = query.single_mut();
    if is_paused.0 {
        text.sections[0].value = "Unpause".to_string();
    } else {
        text.sections[0].value = "Pause".to_string();
    }
}

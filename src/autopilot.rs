use crate::{
    food::Food,
    input::{CurrentDirection, NextDirection},
    snake::SnakeHead,
};
use bevy::prelude::*;

pub struct AutopilotPlugin;

impl Plugin for AutopilotPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_autopilot_button)
            .add_systems(Update, update_autopilot_button)
            .add_systems(Update, toggle_autopilot)
            .add_systems(Update, autopilot_snake)
            .add_systems(Update, handle_button_click)
            .insert_resource(Autopilot(false));
    }
}

#[derive(Resource)]
pub struct Autopilot(pub bool);

fn toggle_autopilot(mut autopilot: ResMut<Autopilot>, input: Res<ButtonInput<KeyCode>>) {
    if input.just_pressed(KeyCode::Space) {
        autopilot.0 = !autopilot.0;
    }
}

fn handle_button_click(
    mut autopilot: ResMut<Autopilot>,
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<Button>)>,
) {
    for interaction in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            autopilot.0 = !autopilot.0;
        }
    }
}

fn setup_autopilot_button(mut cmd: Commands) {
    cmd.spawn(ButtonBundle {
        style: Style {
            flex_basis: Val::Px(150.0),
            flex_shrink: 0.,
            position_type: PositionType::Absolute,
            left: Val::Px(10.0),
            bottom: Val::Px(10.0),
            border: UiRect::all(Val::Px(2.0)),
            ..default()
        },
        background_color: Color::NONE.into(),
        border_color: Color::rgb(1.0, 1.0, 1.0).into(),
        ..default()
    })
    .with_children(|parent| {
        parent.spawn(TextBundle::from_section(
            "Autopilot: Off",
            TextStyle {
                font_size: 40.0,
                color: Color::rgb(1.0, 1.0, 1.0),
                ..default()
            },
        ));
    })
    .insert(Name::new("Autopilot Button")); // Add this line to give the button a name
}

fn update_autopilot_button(
    autopilot: Res<Autopilot>,
    mut query: Query<(Entity, &mut Text), With<Parent>>,
    parent_query: Query<&Parent>,
    name_query: Query<&Name>,
) {
    for (entity, mut text) in query.iter_mut() {
        if let Ok(parent) = parent_query.get(entity) {
            if let Ok(name) = name_query.get(parent.get()) {
                if name.as_str() == "Autopilot Button" {
                    if autopilot.0 {
                        text.sections[0].value = "Autopilot: On".to_string();
                    } else {
                        text.sections[0].value = "Autopilot: Off".to_string();
                    }
                }
            }
        }
    }
}

fn autopilot_snake(
    autopilot: Res<Autopilot>,
    mut q: Query<(&mut NextDirection, &CurrentDirection), With<SnakeHead>>,
    food_q: Query<&Transform, With<Food>>,
    head_transform_q: Query<&Transform, With<SnakeHead>>,
) {
    if autopilot.0 {
        let (mut next_direction, current_direction) = q.single_mut();
        let head_transform = head_transform_q.single();
        let food_transform = food_q.single();

        let difference = food_transform.translation - head_transform.translation;

        if difference.x > 0. && current_direction.0 != Direction2d::NEG_X {
            next_direction.0 = Direction2d::X;
        } else if difference.x < 0. && current_direction.0 != Direction2d::X {
            next_direction.0 = Direction2d::NEG_X;
        } else if difference.y > 0. && current_direction.0 != Direction2d::NEG_Y {
            next_direction.0 = Direction2d::Y;
        } else if difference.y < 0. && current_direction.0 != Direction2d::Y {
            next_direction.0 = Direction2d::NEG_Y;
        }
    }
}

use crate::{
    input::CurrentDirection,
    snake::{NextDirection, SnakeHead},
};
use bevy::prelude::*;

pub struct AutopilotPlugin;

impl Plugin for AutopilotPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_autopilot_button)
            .add_systems(Update, update_autopilot_button)
            .add_systems(Update, toggle_autopilot)
            .add_systems(Update, autopilot_snake)
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

fn setup_autopilot_button(mut cmd: Commands) {
    cmd.spawn(ButtonBundle {
        style: Style {
            size: Size::new(Val::Px(150.0), Val::Px(65.0)),
            margin: UiRect::all(Val::Auto),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        background_color: Color::rgb(0.15, 0.15, 0.15).into(),
        ..default()
    })
    .with_children(|parent| {
        parent.spawn(TextBundle::from_section(
            "Toggle Autopilot",
            TextStyle {
                font_size: 40.0,
                color: Color::rgb(0.9, 0.9, 0.9),
                ..default()
            },
        ));
    });
}

fn update_autopilot_button(
    autopilot: Res<Autopilot>,
    mut query: Query<&mut BackgroundColor, With<Button>>,
) {
    let mut color = query.single_mut();
    if autopilot.0 {
        *color = Color::rgb(0.0, 1.0, 0.0).into(); // Green
    } else {
        *color = Color::rgb(0.15, 0.15, 0.15).into(); // Gray
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

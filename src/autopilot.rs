use crate::{
    constants::*,
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

#[derive(Component)]
struct AutopilotButtonText;

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
        parent
            .spawn(TextBundle::from_section(
                "Autopilot: Off",
                TextStyle {
                    font_size: 40.0,
                    color: Color::rgb(1.0, 1.0, 1.0),
                    ..default()
                },
            ))
            .insert(AutopilotButtonText);
    });
}

fn update_autopilot_button(
    autopilot: Res<Autopilot>,
    mut query: Query<&mut Text, With<AutopilotButtonText>>,
) {
    let mut text = query.single_mut();
    if autopilot.0 {
        text.sections[0].value = "Autopilot: On".to_string();
    } else {
        text.sections[0].value = "Autopilot: Off".to_string();
    }
}

fn autopilot_snake(
    autopilot: Res<Autopilot>,
    mut q: Query<(&mut NextDirection, &CurrentDirection), With<SnakeHead>>,
    food_q: Query<&Transform, With<Food>>,
    head_transform_q: Query<&Transform, With<SnakeHead>>,
    body_q: Query<&Transform, Without<Food>>,
) {
    if autopilot.0 {
        let (mut next_direction, _) = q.single_mut();
        let head_transform = head_transform_q.single();
        let food_transform = food_q.single();

        let start = head_transform.translation.xy().as_ivec2();
        let end = food_transform.translation.xy().as_ivec2();

        let body_positions: Vec<_> = body_q
            .iter()
            .map(|t| t.translation.xy().as_ivec2())
            .collect();

        let mut open_list = vec![start];
        let mut came_from = std::collections::HashMap::new();
        let mut g_score = std::collections::HashMap::new();
        let mut f_score = std::collections::HashMap::new();

        g_score.insert(start, 0);
        f_score.insert(start, distance(start, end));

        while let Some(mut current) = open_list.iter().min_by_key(|&pos| f_score[pos]).cloned() {
            open_list.retain(|&pos| pos != current);

            if current == end {
                let mut path = vec![current];
                while let Some(&previous) = came_from.get(&current) {
                    path.push(previous);
                    current = previous;
                }
                path.reverse();

                if let Some(next_pos) = path.get(1) {
                    let direction = (*next_pos - start).as_vec2().normalize_or_zero();
                    next_direction.0 = Direction2d::new_unchecked(direction);
                }

                return;
            }

            for direction in &[
                IVec2::new(-1, 0),
                IVec2::new(1, 0),
                IVec2::new(0, -1),
                IVec2::new(0, 1),
            ] {
                let neighbor = current + *direction;
                if neighbor.x < 0
                    || neighbor.x >= GRID_WIDTH as i32
                    || neighbor.y < 0
                    || neighbor.y >= GRID_HEIGHT as i32
                {
                    continue;
                }
                if body_positions.contains(&neighbor) {
                    continue;
                }

                let tentative_g_score = g_score[&current] + 1;
                if !g_score.contains_key(&neighbor) || tentative_g_score < g_score[&neighbor] {
                    came_from.insert(neighbor, current);
                    g_score.insert(neighbor, tentative_g_score);
                    f_score.insert(neighbor, tentative_g_score + distance(neighbor, end));
                    if !open_list.contains(&neighbor) {
                        open_list.push(neighbor);
                    }
                }
            }
        }
    }
}

fn distance(a: IVec2, b: IVec2) -> i32 {
    (a.x - b.x).abs() + (a.y - b.y).abs()
}

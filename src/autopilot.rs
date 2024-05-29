use crate::{
    constants::*,
    food::Food,
    input::{CurrentDirection, NextDirection},
    snake::SnakeHead,
};
use bevy::prelude::*;
use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
};

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
    for &interaction in &mut interaction_query {
        if interaction == Interaction::Pressed {
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

struct Node {
    pos: IVec2,
    f_score: i32,
}

impl PartialEq for Node {
    fn eq(&self, other: &Node) -> bool {
        self.f_score == other.f_score
    }
}

impl Eq for Node {}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Node) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Node) -> Ordering {
        other
            .f_score
            .cmp(&self.f_score)
            .then(other.pos.x.cmp(&self.pos.x))
            .then(other.pos.y.cmp(&self.pos.y))
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

        if let Some(&next_pos) = find_path(start, end, body_positions).get(1) {
            next_direction.0 =
                Direction2d::new_unchecked((next_pos - start).as_vec2().normalize_or_zero());
        }
    }
}

fn find_path(start: IVec2, end: IVec2, body_positions: Vec<IVec2>) -> Vec<IVec2> {
    let mut cells = HashMap::new();
    let mut open_list = BinaryHeap::new();

    cells.insert(start, (0, 0, None));
    open_list.push(Node {
        pos: start,
        f_score: 0,
    });

    while let Some(Node {
        pos: mut current, ..
    }) = open_list.pop()
    {
        if current == end {
            let mut path = vec![current];
            while let Some(&(_, _, Some(previous))) = cells.get(&current) {
                path.push(previous);
                current = previous;
            }
            path.reverse();
            return path;
        }

        let neighbors = [(-1, 0), (1, 0), (0, -1), (0, 1)]
            .into_iter()
            .map(|(x, y)| IVec2::new(x, y))
            .map(|direction| current + direction);

        for neighbor in neighbors {
            if body_positions.contains(&neighbor)
                || neighbor.x < 0
                || neighbor.y < 0
                || neighbor.x >= GRID_WIDTH as i32
                || neighbor.y >= GRID_HEIGHT as i32
            {
                continue;
            }

            let g_score = cells[&current].0 + 1;
            let h_score = manhattan_distance(neighbor, end);
            let f_score = g_score + h_score;

            if !cells.contains_key(&neighbor) || g_score < cells[&neighbor].0 {
                cells.insert(neighbor, (g_score, h_score, Some(current)));
                open_list.push(Node {
                    pos: neighbor,
                    f_score,
                });
            }
        }
    }
    vec![]
}

fn manhattan_distance(a: IVec2, b: IVec2) -> i32 {
    (a.x - b.x).abs() + (a.y - b.y).abs()
}

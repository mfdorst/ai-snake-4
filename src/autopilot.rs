use crate::{
    constants::*,
    food::Food,
    input::NextDirection,
    snake::{SnakeHead, SnakeMoveEvent},
};
use bevy::prelude::*;
use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
};

pub struct AutopilotPlugin;

impl Plugin for AutopilotPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_autopilot_button)
            .add_systems(
                Update,
                (
                    autopilot_snake.in_set(AutopilotSet),
                    handle_button_click,
                    toggle_autopilot,
                    update_autopilot_button,
                ),
            )
            .insert_resource(Autopilot(false));
    }
}

#[derive(Resource)]
pub struct Autopilot(pub bool);

#[derive(Component)]
struct AutopilotButtonText;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct AutopilotSet;

#[derive(Copy, Clone)]
struct Node {
    position: IVec2,
    previous: Option<IVec2>,
    f_score: i32,
    g_score: i32,
}

impl Eq for Node {}

impl Ord for Node {
    fn cmp(&self, other: &Node) -> Ordering {
        other
            .f_score
            .cmp(&self.f_score)
            .then(other.position.x.cmp(&self.position.x))
            .then(other.position.y.cmp(&self.position.y))
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Node) -> bool {
        self.position == other.position
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Node) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

const CARDINAL_DIRECTIONS: [Direction2d; 4] = [
    Direction2d::X,
    Direction2d::NEG_X,
    Direction2d::Y,
    Direction2d::NEG_Y,
];

fn autopilot_snake(
    autopilot: Res<Autopilot>,
    mut ev_move: EventReader<SnakeMoveEvent>,
    body_q: Query<&Transform, Without<Food>>,
    food_q: Query<&Transform, With<Food>>,
    mut head_q: Query<(&Transform, &mut NextDirection), With<SnakeHead>>,
) {
    if !autopilot.0 || ev_move.is_empty() {
        return;
    }
    ev_move.clear();

    let (head_transform, mut next_direction) = head_q.single_mut();
    let food_transform = food_q.single();

    let start = head_transform.translation.xy().as_ivec2();
    let end = food_transform.translation.xy().as_ivec2();

    let body_positions: Vec<_> = body_q
        .iter()
        .map(|t| t.translation.xy().as_ivec2())
        .collect();

    if let Some(&next_pos) = find_path(start, end, &body_positions).get(1) {
        next_direction.0 = Direction2d::new_unchecked((next_pos - start).as_vec2());
    } else {
        // No path found – survival mode

        let mut largest_area = 0;
        let mut best_direction = None;

        for direction in CARDINAL_DIRECTIONS {
            let next_head_pos = head_transform.translation + direction.extend(0.);
            let next_pos = next_head_pos.xy().as_ivec2();

            if is_valid_move(next_pos, &body_positions) {
                let area = flood_fill(next_pos, &body_positions);
                if area > largest_area {
                    largest_area = area;
                    best_direction = Some(direction);
                }
            }
        }

        if let Some(direction) = best_direction {
            next_direction.0 = direction;
        }
    }
}

fn find_path(start: IVec2, end: IVec2, body_positions: &[IVec2]) -> Vec<IVec2> {
    let mut cells = HashMap::new();
    let mut open_list = BinaryHeap::new();

    // Start with the end position
    let node = Node {
        position: end,
        previous: None,
        f_score: 0,
        g_score: 0,
    };
    cells.insert(end, node);
    open_list.push(node);

    while let Some(Node {
        position: mut current,
        ..
    }) = open_list.pop()
    {
        // Reached the start, reconstruct path
        if current == start {
            let mut path = vec![current];
            while let Some(&Node {
                previous: Some(previous),
                ..
            }) = cells.get(&current)
            {
                path.push(previous);
                current = previous;
            }
            return path;
        }

        let neighbors = CARDINAL_DIRECTIONS
            .iter()
            .map(|direction| current + direction.as_ivec2());

        for neighbor in neighbors {
            if body_positions.contains(&neighbor) && neighbor != start
                || neighbor.x < 0
                || neighbor.y < 0
                || neighbor.x >= GRID_WIDTH as i32
                || neighbor.y >= GRID_HEIGHT as i32
            {
                continue;
            }

            let g_score = cells[&current].g_score + 1;
            let h_score = manhattan_distance(neighbor, start);
            let f_score = g_score + h_score;

            if !cells.contains_key(&neighbor) || g_score < cells[&neighbor].g_score {
                let node = Node {
                    position: neighbor,
                    previous: Some(current),
                    f_score,
                    g_score,
                };
                cells.insert(neighbor, node);
                open_list.push(node);
            }
        }
    }
    vec![]
}

fn flood_fill(start: IVec2, body_positions: &[IVec2]) -> usize {
    let mut queue = vec![start];
    let mut visited = HashSet::new();
    let mut area = 0;

    while let Some(pos) = queue.pop() {
        if !visited.contains(&pos) && is_valid_move(pos, &body_positions) {
            visited.insert(pos);
            area += 1;

            for direction in CARDINAL_DIRECTIONS {
                queue.push(pos + direction.as_ivec2());
            }
        }
    }

    area
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

fn is_valid_move(pos: IVec2, body_positions: &[IVec2]) -> bool {
    pos.x >= 0
        && pos.x < GRID_WIDTH as i32
        && pos.y >= 0
        && pos.y < GRID_HEIGHT as i32
        && !body_positions.contains(&pos)
}

fn manhattan_distance(a: IVec2, b: IVec2) -> i32 {
    (a.x - b.x).abs() + (a.y - b.y).abs()
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

fn toggle_autopilot(mut autopilot: ResMut<Autopilot>, input: Res<ButtonInput<KeyCode>>) {
    if input.just_pressed(KeyCode::Space) {
        autopilot.0 = !autopilot.0;
    }
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

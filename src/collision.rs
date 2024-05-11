use crate::{
    constants::*,
    food::{EatEvent, Food},
    movement::SnakeMoveEvent,
    IsDead, NextDirection, SnakeBody, SnakeHead,
};
use bevy::prelude::*;

pub struct CollisionPlugin;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct CollisionSet;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                check_body_collision,
                check_food_collision,
                check_wall_collision,
            )
                .in_set(CollisionSet),
        );
    }
}

fn check_body_collision(
    mut ev_move: EventReader<SnakeMoveEvent>,
    mut is_dead: ResMut<IsDead>,
    next_direction_q: Query<&NextDirection>,
    transform_q: Query<&Transform>,
    body: Res<SnakeBody>,
) {
    for _ in ev_move.read() {
        let head_transform = transform_q.get(body.0[0]).unwrap();
        let next_direction = next_direction_q.single();
        let next_head_pos = head_transform.translation + next_direction.0.extend(0.);

        for &segment in body.0.iter().skip(1) {
            let body_transform = transform_q.get(segment).unwrap();
            if next_head_pos == body_transform.translation {
                is_dead.0 = true;
            }
        }
    }
}

fn check_food_collision(
    mut ev_eat: EventWriter<EatEvent>,
    mut ev_move: EventReader<SnakeMoveEvent>,
    food_q: Query<&Transform, With<Food>>,
    head_transform_q: Query<&Transform, With<SnakeHead>>,
    next_direction_q: Query<&NextDirection>,
) {
    for _ in ev_move.read() {
        let head_transform = head_transform_q.single();
        let next_direction = next_direction_q.single();
        let next_head_pos = head_transform.translation + next_direction.0.extend(0.);

        for food_transform in &food_q {
            if next_head_pos == food_transform.translation {
                ev_eat.send(EatEvent);
            }
        }
    }
}

fn check_wall_collision(
    mut ev_move: EventReader<SnakeMoveEvent>,
    mut is_dead: ResMut<IsDead>,
    q: Query<(&Transform, &NextDirection), With<SnakeHead>>,
) {
    for _ in ev_move.read() {
        let (transform, direction) = q.single();
        let t = transform.translation + direction.0.extend(0.);

        if t.x < 0. || t.x >= GRID_WIDTH || t.y < 0. || t.y >= GRID_HEIGHT {
            is_dead.0 = true;
        }
    }
}

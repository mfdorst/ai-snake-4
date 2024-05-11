use crate::{constants::*, CurrentDirection, IsDead, NextDirection, SnakeBody, SnakeHead};
use bevy::prelude::*;

pub struct MovePlugin;

#[derive(Event)]
pub struct MoveEvent;

#[derive(Resource)]
pub struct MoveTimer(pub Timer);

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct MoveTimerTickSet;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct MoveSet;

impl Plugin for MovePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, tick_move_timer.in_set(MoveTimerTickSet))
            .add_systems(Update, move_snake.in_set(MoveSet))
            .insert_resource(MoveTimer(Timer::from_seconds(
                1. / INITIAL_SPEED,
                TimerMode::Repeating,
            )))
            .add_event::<MoveEvent>();
    }
}

fn move_snake(
    mut direction_q: Query<(&mut CurrentDirection, &NextDirection), With<SnakeHead>>,
    mut transform_q: Query<&mut Transform>,
    mut ev_move: EventReader<MoveEvent>,
    body: Res<SnakeBody>,
    is_dead: Res<IsDead>,
) {
    if is_dead.0 {
        return;
    }
    for _ in ev_move.read() {
        let body_iter = body.0.iter().rev().zip(body.0.iter().rev().skip(1));
        for (&current, &next) in body_iter {
            let next_transform = *transform_q.get(next).unwrap();
            let mut current_transform = transform_q.get_mut(current).unwrap();
            *current_transform = next_transform;
        }

        let mut head_transform = transform_q.get_mut(body.0[0]).unwrap();
        let (mut current_direction, next_direction) = direction_q.single_mut();
        current_direction.0 = next_direction.0;
        head_transform.translation += current_direction.0.extend(0.);
    }
}

fn tick_move_timer(
    mut timer: ResMut<MoveTimer>,
    mut ev_move: EventWriter<MoveEvent>,
    is_dead: Res<IsDead>,
    time: Res<Time>,
) {
    if !is_dead.0 && timer.0.tick(time.delta()).just_finished() {
        ev_move.send(MoveEvent);
    }
}

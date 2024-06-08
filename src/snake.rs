use crate::{
    constants::*,
    input::{CurrentDirection, NextDirection},
};
use bevy::prelude::*;
use std::collections::VecDeque;

pub struct SnakePlugin;

#[derive(Resource)]
pub struct IsDead(pub bool);

#[derive(Resource)]
pub struct SnakeBody(pub VecDeque<Entity>);

#[derive(Resource)]
pub struct SnakeMoveTimer(pub Timer);

#[derive(Event)]
pub struct SnakeMoveEvent;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct SetupSnakeSet;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct SnakeMoveSet;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct SnakeMoveTimerTickSet;

impl Plugin for SnakePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_snake.in_set(SetupSnakeSet))
            .add_systems(
                Update,
                (
                    tick_move_timer.in_set(SnakeMoveTimerTickSet),
                    move_snake.in_set(SnakeMoveSet),
                ),
            )
            .insert_resource(IsDead(false))
            .insert_resource(SnakeMoveTimer(Timer::from_seconds(
                1. / INITIAL_SPEED,
                TimerMode::Repeating,
            )))
            .add_event::<SnakeMoveEvent>();
    }
}

fn move_snake(
    mut transform_q: Query<&mut Transform>,
    mut ev_move: EventReader<SnakeMoveEvent>,
    mut body: ResMut<SnakeBody>,
    is_dead: Res<IsDead>,
    mut current_direction: ResMut<CurrentDirection>,
    next_direction: Res<NextDirection>,
) {
    if is_dead.0 || ev_move.is_empty() {
        return;
    }
    ev_move.clear();

    current_direction.0 = next_direction.0;

    let prev_head_transform = transform_q.get_mut(body.0[0]).unwrap();
    let new_head_pos = prev_head_transform.translation + next_direction.0.extend(0.);

    let tail = body.0.pop_back().unwrap();
    body.0.push_front(tail);

    let mut new_head_transform = transform_q.get_mut(body.0[0]).unwrap();
    new_head_transform.translation = new_head_pos;
}

fn spawn_snake(mut cmd: Commands) {
    let sprite = Sprite {
        color: Color::WHITE,
        custom_size: Some(Vec2::ONE),
        ..default()
    };
    let transforms: Vec<_> = (0..SNAKE_LENGTH)
        .map(|i| Transform::from_xyz(GRID_WIDTH / 2. - i as f32, GRID_HEIGHT / 2., 0.))
        .collect();

    let mut body = VecDeque::new();

    for transform in transforms {
        body.push_back(
            cmd.spawn(SpriteBundle {
                sprite: sprite.clone(),
                transform,
                ..default()
            })
            .id(),
        );
    }
    cmd.insert_resource(CurrentDirection(Direction2d::X));
    cmd.insert_resource(NextDirection(Direction2d::X));
    cmd.insert_resource(SnakeBody(body));
}

fn tick_move_timer(
    mut timer: ResMut<SnakeMoveTimer>,
    mut ev_move: EventWriter<SnakeMoveEvent>,
    is_dead: Res<IsDead>,
    time: Res<Time>,
) {
    if !is_dead.0 && timer.0.tick(time.delta()).just_finished() {
        ev_move.send(SnakeMoveEvent);
    }
}

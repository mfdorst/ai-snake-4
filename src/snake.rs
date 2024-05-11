use crate::{
    constants::*,
    input::{CurrentDirection, NextDirection},
};
use bevy::prelude::*;

pub struct SnakePlugin;

#[derive(Component)]
pub struct SnakeHead;

#[derive(Resource)]
pub struct IsDead(pub bool);

#[derive(Resource)]
pub struct SnakeBody(pub Vec<Entity>);

#[derive(Resource)]
pub struct SnakeMoveTimer(pub Timer);

#[derive(Event)]
pub struct SnakeMoveEvent;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct SnakeMoveSet;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct SnakeMoveTimerTickSet;

impl Plugin for SnakePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_snake)
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
    mut direction_q: Query<(&mut CurrentDirection, &NextDirection), With<SnakeHead>>,
    mut transform_q: Query<&mut Transform>,
    mut ev_move: EventReader<SnakeMoveEvent>,
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

fn spawn_snake(mut cmd: Commands) {
    let sprite = Sprite {
        color: Color::WHITE,
        custom_size: Some(Vec2::ONE),
        ..default()
    };
    let transforms: Vec<_> = (0..SNAKE_LENGTH)
        .map(|i| Transform::from_xyz(GRID_WIDTH / 2. - i as f32, GRID_HEIGHT / 2., 0.))
        .collect();

    let mut body = vec![];

    body.push(
        cmd.spawn((
            SpriteBundle {
                sprite: sprite.clone(),
                transform: transforms[0],
                ..default()
            },
            SnakeHead,
            CurrentDirection(Direction2d::X),
            NextDirection(Direction2d::X),
        ))
        .id(),
    );
    for transform in transforms.into_iter().skip(1) {
        body.push(
            cmd.spawn(SpriteBundle {
                sprite: sprite.clone(),
                transform,
                ..default()
            })
            .id(),
        );
    }
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

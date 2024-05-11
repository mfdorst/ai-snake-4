use crate::constants::*;
use crate::play_area::PlayAreaPlugin;
use bevy::{prelude::*, render::camera::ScalingMode, utils::Duration};
use collision::{CollisionPlugin, CollisionSet};
use rand::Rng;

mod collision;
mod constants;
mod play_area;

#[derive(Component)]
struct CurrentDirection(Direction2d);

#[derive(Component)]
pub struct NextDirection(Direction2d);

#[derive(Component)]
pub struct SnakeHead;

#[derive(Event)]
pub struct EatEvent;

#[derive(Event)]
pub struct MoveEvent;

#[derive(Resource)]
pub struct IsDead(bool);

#[derive(Resource)]
struct MoveTimer(Timer);

#[derive(Resource)]
struct Speed(f32);

#[derive(Resource)]
pub struct SnakeBody(Vec<Entity>);

#[derive(Component)]
pub struct Food;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PlayAreaPlugin, CollisionPlugin))
        .add_systems(
            Startup,
            (setup_camera, setup_clear_color, setup_food, spawn_snake),
        )
        .add_systems(
            Update,
            (
                change_head_direction,
                tick_move_timer,
                (grow_snake, respawn_food, speed_up).before(move_snake),
                move_snake
                    .after(tick_move_timer)
                    .after(change_head_direction),
            ),
        )
        .configure_sets(
            Update,
            CollisionSet.after(tick_move_timer).before(move_snake),
        )
        .insert_resource(MoveTimer(Timer::from_seconds(
            1. / INITIAL_SPEED,
            TimerMode::Repeating,
        )))
        .insert_resource(IsDead(false))
        .insert_resource(Speed(INITIAL_SPEED))
        .add_event::<EatEvent>()
        .add_event::<MoveEvent>()
        .run();
}

fn change_head_direction(
    mut q: Query<(&CurrentDirection, &mut NextDirection), With<SnakeHead>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    let desired = if input.just_pressed(KeyCode::KeyW) {
        Direction2d::Y
    } else if input.just_pressed(KeyCode::KeyA) {
        Direction2d::NEG_X
    } else if input.just_pressed(KeyCode::KeyS) {
        Direction2d::NEG_Y
    } else if input.just_pressed(KeyCode::KeyD) {
        Direction2d::X
    } else {
        return;
    };

    let (current, mut next) = q.single_mut();

    // Don't allow 180's
    if desired != -current.0 {
        next.0 = desired;
    }
}

fn grow_snake(mut cmd: Commands, mut body: ResMut<SnakeBody>, mut ev_eat: EventReader<EatEvent>) {
    for _ in ev_eat.read() {
        let new_segment = cmd
            .spawn(SpriteBundle {
                sprite: Sprite {
                    color: Color::WHITE,
                    custom_size: Some(Vec2::ONE),
                    ..default()
                },
                // Make new segment invisible by spawning it behind the play area
                transform: Transform::from_xyz(0., 0., -2.),
                ..default()
            })
            .id();

        body.0.push(new_segment);
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

fn respawn_food(
    mut cmd: Commands,
    mut ev_eat: EventReader<EatEvent>,
    food_q: Query<Entity, With<Food>>,
) {
    for _ in ev_eat.read() {
        for food in &food_q {
            cmd.entity(food).despawn();
            spawn_food(&mut cmd);
        }
    }
}

fn setup_camera(mut cmd: Commands) {
    let mut camera = Camera2dBundle::default();
    camera.projection.scaling_mode = ScalingMode::FixedHorizontal(GRID_WIDTH);
    // Pan the camera so that (0,0) is in the bottom left, assuming a 16x9 aspect ratio
    camera.transform = Transform::from_xyz(GRID_WIDTH / 2. - 0.5, GRID_HEIGHT / 2. - 0.5, 0.);
    cmd.spawn(camera);
}

fn setup_clear_color(mut cmd: Commands) {
    cmd.insert_resource(ClearColor(Color::GRAY));
}

fn setup_food(mut cmd: Commands) {
    spawn_food(&mut cmd);
}

fn spawn_food(cmd: &mut Commands) {
    let mut rng = rand::thread_rng();
    let x = rng.gen_range(0..GRID_WIDTH as i32);
    let y = rng.gen_range(0..GRID_HEIGHT as i32);

    cmd.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::RED,
            custom_size: Some(Vec2::ONE),
            ..default()
        },
        transform: Transform::from_xyz(x as f32, y as f32, 0.),
        ..default()
    })
    .insert(Food);
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

fn speed_up(
    mut speed: ResMut<Speed>,
    mut timer: ResMut<MoveTimer>,
    mut ev_eat: EventReader<EatEvent>,
) {
    for _ in ev_eat.read() {
        speed.0 *= 1.05;
        timer.0.set_duration(Duration::from_secs_f32(1. / speed.0));
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

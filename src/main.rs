use bevy::{prelude::*, render::camera::ScalingMode};
use rand::Rng;

const GRID_HEIGHT: f32 = 36.;
const GRID_WIDTH: f32 = 64.;
const SNAKE_LENGTH: usize = 5;
const STEPS_PER_SECOND: f32 = 8.;

#[derive(Component)]
struct CurrentDirection(Direction2d);

#[derive(Component)]
struct NextDirection(Direction2d);

#[derive(Component)]
struct SnakeHead;

#[derive(Resource)]
struct MoveTimer(Timer);

#[derive(Resource)]
struct SnakeBody(Vec<Entity>);

#[derive(Component)]
struct Food;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(
            Startup,
            (
                setup_camera,
                setup_clear_color,
                setup_play_area,
                setup_food,
                spawn_snake,
            ),
        )
        .add_systems(
            Update,
            (
                change_head_direction,
                tick_move_timer,
                (
                    check_body_collision,
                    check_food_collision,
                    check_wall_collision,
                )
                    .after(tick_move_timer)
                    .before(move_snake),
                move_snake
                    .after(tick_move_timer)
                    .after(change_head_direction),
            ),
        )
        .insert_resource(MoveTimer(Timer::from_seconds(
            1. / STEPS_PER_SECOND,
            TimerMode::Repeating,
        )))
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

fn check_body_collision(
    mut timer: ResMut<MoveTimer>,
    next_direction_q: Query<&NextDirection>,
    transform_q: Query<&Transform>,
    body: Res<SnakeBody>,
) {
    if !timer.0.finished() {
        return;
    }
    let head_transform = transform_q.get(body.0[0]).unwrap();
    let next_direction = next_direction_q.single();
    let next_head_pos = head_transform.translation + next_direction.0.extend(0.);

    for &segment in body.0.iter().skip(1) {
        let body_transform = transform_q.get(segment).unwrap();
        if next_head_pos == body_transform.translation {
            timer.0.pause();
            timer.0.reset();
        }
    }
}

fn check_food_collision(
    mut cmd: Commands,
    mut body: ResMut<SnakeBody>,
    food_q: Query<(Entity, &Transform), With<Food>>,
    head_transform_q: Query<&Transform, With<SnakeHead>>,
    next_direction_q: Query<&NextDirection>,
    timer: Res<MoveTimer>,
) {
    if !timer.0.finished() {
        return;
    }
    let head_transform = head_transform_q.single();
    let next_direction = next_direction_q.single();
    let next_head_pos = head_transform.translation + next_direction.0.extend(0.);

    for (food_entity, food_transform) in &food_q {
        if next_head_pos == food_transform.translation {
            cmd.entity(food_entity).despawn();
            spawn_food(&mut cmd);

            let new_segment = cmd
                .spawn(SpriteBundle {
                    sprite: Sprite {
                        color: Color::WHITE,
                        custom_size: Some(Vec2::ONE),
                        ..default()
                    },
                    // Put it somewhere in the body. Position will be updated next frame.
                    transform: *head_transform,
                    ..default()
                })
                .id();

            body.0.push(new_segment);
        }
    }
}

fn check_wall_collision(
    mut timer: ResMut<MoveTimer>,
    q: Query<(&Transform, &NextDirection), With<SnakeHead>>,
) {
    if !timer.0.finished() {
        return;
    }
    let (transform, direction) = q.single();
    let t = transform.translation + direction.0.extend(0.);

    if t.x < 0. || t.x >= GRID_WIDTH || t.y < 0. || t.y >= GRID_HEIGHT {
        timer.0.pause();
        timer.0.reset();
    }
}

fn move_snake(
    mut direction_q: Query<(&mut CurrentDirection, &NextDirection), With<SnakeHead>>,
    mut transform_q: Query<&mut Transform>,
    body: Res<SnakeBody>,
    timer: Res<MoveTimer>,
) {
    if timer.0.finished() {
        let body_iter = body.0.iter().rev().zip(body.0.iter().rev().skip(1));
        for (&current, &next) in body_iter {
            let next_transform = transform_q.get(next).unwrap().clone();
            let mut current_transform = transform_q.get_mut(current).unwrap();
            *current_transform = next_transform;
        }

        let mut head_transform = transform_q.get_mut(body.0[0]).unwrap();
        let (mut current_direction, next_direction) = direction_q.single_mut();
        current_direction.0 = next_direction.0;
        head_transform.translation += current_direction.0.extend(0.);
    }
}

fn setup_camera(mut cmd: Commands) {
    let mut camera = Camera2dBundle::default();
    camera.projection.scaling_mode = ScalingMode::FixedHorizontal(GRID_WIDTH as f32);
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

fn setup_play_area(mut cmd: Commands) {
    cmd.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::BLACK,
            custom_size: Some(Vec2::new(GRID_WIDTH, GRID_HEIGHT)),
            ..default()
        },
        // Move the play area so the bottom left corner is at (0,0)
        transform: Transform::from_xyz(GRID_WIDTH / 2. - 0.5, GRID_HEIGHT / 2. - 0.5, -1.),
        ..default()
    });
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
        .into_iter()
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

fn tick_move_timer(mut timer: ResMut<MoveTimer>, time: Res<Time>) {
    timer.0.tick(time.delta());
}

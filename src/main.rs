use bevy::{prelude::*, render::camera::ScalingMode};

const GRID_WIDTH: f32 = 32.;
const GRID_HEIGHT: f32 = 18.;
const STEPS_PER_SECOND: f32 = 4.;
const SNAKE_LENGTH: usize = 5;

#[derive(Component)]
struct SnakeHead;

#[derive(Resource)]
struct SnakeBody(Vec<Entity>);

#[derive(Component)]
struct CurrentDirection(Direction2d);

#[derive(Component)]
struct NextDirection(Direction2d);

#[derive(Resource)]
struct MoveTimer(Timer);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(
            Startup,
            (
                setup_camera,
                setup_play_area,
                setup_clear_color,
                spawn_snake,
            ),
        )
        .add_systems(
            Update,
            (
                tick_move_timer,
                change_head_direction,
                check_wall_collision
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

fn setup_play_area(mut cmd: Commands) {
    cmd.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::BLACK,
            custom_size: Some(Vec2::new(GRID_WIDTH, GRID_HEIGHT)),
            ..default()
        },
        // Move the play area so the bottom left corner is at (0,0)
        transform: Transform::from_xyz(GRID_WIDTH / 2. - 0.5, GRID_HEIGHT / 2. - 0.5, 0.),
        ..default()
    });
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

fn change_head_direction(
    input: Res<ButtonInput<KeyCode>>,
    mut q: Query<(&CurrentDirection, &mut NextDirection), With<SnakeHead>>,
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

fn tick_move_timer(mut timer: ResMut<MoveTimer>, time: Res<Time>) {
    timer.0.tick(time.delta());
}

fn move_snake(
    mut transform_q: Query<&mut Transform>,
    timer: Res<MoveTimer>,
    mut direction_q: Query<(&mut CurrentDirection, &NextDirection), With<SnakeHead>>,
    body: Res<SnakeBody>,
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

fn check_wall_collision(
    q: Query<(&Transform, &NextDirection), With<SnakeHead>>,
    mut timer: ResMut<MoveTimer>,
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

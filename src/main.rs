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
struct Direction(Direction2d);

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
                change_head_direction,
                move_snake,
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
            Direction(Direction2d::X),
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
    mut q: Query<&mut Direction, With<SnakeHead>>,
) {
    let mut direction = q.single_mut();

    if input.just_pressed(KeyCode::KeyW) {
        direction.0 = Direction2d::Y;
    } else if input.just_pressed(KeyCode::KeyA) {
        direction.0 = Direction2d::NEG_X;
    } else if input.just_pressed(KeyCode::KeyS) {
        direction.0 = Direction2d::NEG_Y;
    } else if input.just_pressed(KeyCode::KeyD) {
        direction.0 = Direction2d::X;
    }
}

fn move_snake(
    mut transform_q: Query<&mut Transform>,
    mut timer: ResMut<MoveTimer>,
    direction_q: Query<&Direction, With<SnakeHead>>,
    body: Res<SnakeBody>,
    time: Res<Time>,
) {
    timer.0.tick(time.delta());

    if timer.0.finished() {
        // Iterate over body segments in pairs from tail to head so positions are not written
        // before they are read
        let body_iter = body.0.iter().rev().zip(body.0.iter().rev().skip(1));

        // Set the position of each segment to the position of the next segment
        for (&current, &next) in body_iter {
            let next_transform = transform_q.get(next).unwrap().clone();
            let mut current_transform = transform_q.get_mut(current).unwrap();
            *current_transform = next_transform;
        }

        // Move the head in the direciton of movement
        let mut head_transform = transform_q.get_mut(body.0[0]).unwrap();
        let head_direction = direction_q.get(body.0[0]).unwrap();
        head_transform.translation += head_direction.0.extend(0.);
    }
}

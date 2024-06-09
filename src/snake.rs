use crate::{
    constants::*,
    input::{CurrentDirection, NextDirection},
};
use bevy::{
    prelude::*,
    render::{
        mesh::{Indices, PrimitiveTopology},
        render_asset::RenderAssetUsages,
    },
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use std::{
    collections::VecDeque,
    f32::consts::{FRAC_PI_2, PI},
};

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

#[derive(Resource)]
struct CornerMesh(Handle<Mesh>);

#[derive(Resource)]
pub struct StraightMesh(pub Handle<Mesh>);

#[derive(Resource)]
pub struct SnakeMaterial(pub Handle<ColorMaterial>);

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
    mut mesh_q: Query<&mut Mesh2dHandle>,
    mut transform_q: Query<&mut Transform>,
    mut ev_move: EventReader<SnakeMoveEvent>,
    mut body: ResMut<SnakeBody>,
    corner_mesh: Res<CornerMesh>,
    mut current_direction: ResMut<CurrentDirection>,
    is_dead: Res<IsDead>,
    next_direction: Res<NextDirection>,
    straight_mesh: Res<StraightMesh>,
) {
    if is_dead.0 || ev_move.is_empty() {
        return;
    }
    ev_move.clear();

    // Make the old tail the new head
    body.0.rotate_right(1);

    let new_head = body.0[0];
    let old_head = body.0[1];

    *transform_q.get_mut(new_head).unwrap() = {
        let translation =
            transform_q.get(old_head).unwrap().translation + next_direction.0.extend(0.);
        let rotation = if next_direction.0.x == 0.0 {
            FRAC_PI_2
        } else {
            0.0
        };
        Transform {
            translation,
            rotation: Quat::from_rotation_z(rotation),
            ..default()
        }
    };
    *mesh_q.get_mut(new_head).unwrap() = straight_mesh.0.clone().into();

    // If the direction changed, make the old head into a corner
    if current_direction.0 != next_direction.0 {
        *mesh_q.get_mut(old_head).unwrap() = corner_mesh.0.clone().into();
        transform_q.get_mut(old_head).unwrap().rotation =
            get_corner_rotation(current_direction.0, next_direction.0);
    }
    current_direction.0 = next_direction.0;
}

fn spawn_snake(
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let straight_mesh = Mesh::from(Rectangle::new(1.0, 0.8));
    let straight_mesh_handle = meshes.add(straight_mesh);

    let corner_mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    )
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vec![
            [-0.5, 0.4, 0.],
            [-0.4, 0.4, 0.],
            [0.4, 0.4, 0.],
            [-0.5, -0.4, 0.],
            [-0.4, -0.4, 0.],
            [0.4, -0.4, 0.],
            [-0.4, -0.5, 0.],
            [0.4, -0.5, 0.],
        ],
    )
    .with_inserted_indices(Indices::U32(vec![
        0, 1, 3, 3, 1, 4, 4, 1, 2, 2, 5, 4, 4, 5, 6, 6, 5, 7,
    ]));
    let corner_mesh_handle = meshes.add(corner_mesh);

    let material = materials.add(ColorMaterial::default());

    let transforms: Vec<_> = (0..SNAKE_LENGTH)
        .map(|i| Transform::from_xyz(GRID_WIDTH / 2. - i as f32, GRID_HEIGHT / 2., 0.))
        .collect();

    let mut body = VecDeque::new();

    for transform in transforms {
        body.push_back(
            cmd.spawn(MaterialMesh2dBundle {
                mesh: straight_mesh_handle.clone().into(),
                material: material.clone(),
                transform,
                ..default()
            })
            .id(),
        );
    }
    cmd.insert_resource(CurrentDirection(Direction2d::X));
    cmd.insert_resource(NextDirection(Direction2d::X));
    cmd.insert_resource(SnakeBody(body));
    cmd.insert_resource(StraightMesh(straight_mesh_handle));
    cmd.insert_resource(CornerMesh(corner_mesh_handle));
    cmd.insert_resource(SnakeMaterial(material));
}

fn get_corner_rotation(direction_in: Direction2d, direction_out: Direction2d) -> Quat {
    let up = Direction2d::Y;
    let down = Direction2d::NEG_Y;
    let left = Direction2d::NEG_X;
    let right = Direction2d::X;

    let directions = (direction_in, direction_out);

    let rotation = if directions == (up, left) || directions == (right, down) {
        0.
    } else if directions == (down, left) || directions == (right, up) {
        3. * FRAC_PI_2
    } else if directions == (down, right) || directions == (left, up) {
        PI
    } else if directions == (up, right) || directions == (left, down) {
        FRAC_PI_2
    } else {
        panic!("Invalid turn: {direction_in:?} -> {direction_out:?}");
    };
    Quat::from_rotation_z(rotation)
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

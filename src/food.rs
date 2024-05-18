use crate::{
    constants::*,
    snake::{SetupSnakeSet, SnakeBody, SnakeMoveTimer},
};
use bevy::{prelude::*, utils::Duration};
use rand::Rng;

pub struct FoodPlugin;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct EatSet;

#[derive(Component)]
pub struct Food;

#[derive(Resource)]
struct Speed(f32);

#[derive(Event)]
pub struct EatEvent;

impl Plugin for FoodPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_food.after(SetupSnakeSet))
            .add_systems(
                Update,
                (grow_snake, respawn_food, speed_up_snake).in_set(EatSet),
            )
            .insert_resource(Speed(INITIAL_SPEED))
            .add_event::<EatEvent>();
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

fn respawn_food(
    mut cmd: Commands,
    mut ev_eat: EventReader<EatEvent>,
    food_q: Query<Entity, With<Food>>,
    transform_q: Query<&Transform, Without<Food>>,
) {
    if !ev_eat.is_empty() {
        ev_eat.clear();
        let food = food_q.single();
        cmd.entity(food).despawn();
        spawn_food(&mut cmd, transform_q);
    }
}

fn setup_food(mut cmd: Commands, transform_q: Query<&Transform, Without<Food>>) {
    spawn_food(&mut cmd, transform_q);
}

fn spawn_food(cmd: &mut Commands, transform_q: Query<&Transform, Without<Food>>) {
    let mut rng = rand::thread_rng();

    let transform = 'outer: loop {
        let x = rng.gen_range(0..GRID_WIDTH as i32);
        let y = rng.gen_range(0..GRID_HEIGHT as i32);
        let food_pos = Vec3::new(x as f32, y as f32, 0.);

        for segment_transform in &transform_q {
            if segment_transform.translation == food_pos {
                continue 'outer;
            }
        }
        break Transform::from_translation(food_pos);
    };

    cmd.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::RED,
            custom_size: Some(Vec2::ONE),
            ..default()
        },
        transform,
        ..default()
    })
    .insert(Food);
}

fn speed_up_snake(
    mut speed: ResMut<Speed>,
    mut timer: ResMut<SnakeMoveTimer>,
    mut ev_eat: EventReader<EatEvent>,
) {
    for _ in ev_eat.read() {
        speed.0 *= 1.05;
        timer.0.set_duration(Duration::from_secs_f32(1. / speed.0));
    }
}

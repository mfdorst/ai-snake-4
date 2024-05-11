use crate::{constants::*, MoveTimer};
use bevy::{prelude::*, utils::Duration};
use rand::Rng;

use crate::SnakeBody;

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
        app.add_systems(Startup, setup_food)
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
) {
    for _ in ev_eat.read() {
        for food in &food_q {
            cmd.entity(food).despawn();
            spawn_food(&mut cmd);
        }
    }
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

fn speed_up_snake(
    mut speed: ResMut<Speed>,
    mut timer: ResMut<MoveTimer>,
    mut ev_eat: EventReader<EatEvent>,
) {
    for _ in ev_eat.read() {
        speed.0 *= 1.05;
        timer.0.set_duration(Duration::from_secs_f32(1. / speed.0));
    }
}

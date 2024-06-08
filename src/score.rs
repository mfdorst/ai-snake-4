use crate::{constants::*, food::EatEvent};
use bevy::prelude::*;

#[derive(Component, Default)]
struct Score(u32);

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_score_ui)
            .add_systems(Update, update_score_ui);
    }
}

fn setup_score_ui(mut cmd: Commands) {
    let text_pos = Vec3::new(4.5, GRID_HEIGHT - 2., 0.);
    let text_size = 0.025;

    let text_style = TextStyle {
        font_size: 80.,
        ..default()
    };
    cmd.spawn((
        Text2dBundle {
            text: Text::from_sections([
                TextSection::new("Score: ", text_style.clone()),
                TextSection::new("0", text_style),
            ]),
            transform: Transform {
                translation: text_pos,
                scale: Vec3::splat(text_size),
                ..default()
            },
            ..default()
        },
        Score(0),
    ));
}

fn update_score_ui(mut ev_eat: EventReader<EatEvent>, mut q: Query<(&mut Text, &mut Score)>) {
    if ev_eat.is_empty() {
        return;
    }
    ev_eat.clear();

    for (mut text, mut score) in &mut q {
        score.0 += 1;
        text.sections[1].value = score.0.to_string();
    }
}

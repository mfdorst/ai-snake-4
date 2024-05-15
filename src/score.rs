use crate::food::EatEvent;
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
    let text_style = TextStyle {
        font_size: 40.0,
        ..default()
    };
    cmd.spawn((
        TextBundle::from_sections([
            TextSection::new("Score: ", text_style.clone()),
            TextSection::new("0", text_style),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        }),
        Score(0),
    ));
}

fn update_score_ui(mut ev_eat: EventReader<EatEvent>, mut q: Query<(&mut Text, &mut Score)>) {
    for _ in ev_eat.read() {
        for (mut text, mut score) in &mut q {
            score.0 += 1;
            text.sections[1].value = score.0.to_string();
        }
    }
}

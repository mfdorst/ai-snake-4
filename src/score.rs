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
        font_size: 40.,
        ..default()
    };

    cmd.spawn(NodeBundle {
        style: Style {
            position_type: PositionType::Absolute,
            left: Val::Px(10.),
            top: Val::Px(10.),
            padding: UiRect::all(Val::Px(5.)),
            ..default()
        },
        background_color: Srgba::BLACK.with_alpha(0.8).into(),
        border_radius: BorderRadius::all(Val::Px(10.)),
        ..default()
    })
    .with_children(|parent| {
        parent
            .spawn(TextBundle::from_sections([
                TextSection::new("Score: ", text_style.clone()),
                TextSection::new("0", text_style),
            ]))
            .insert(Score(0));
    });
}

fn update_score_ui(mut ev_eat: EventReader<EatEvent>, mut q: Query<(&mut Text, &mut Score)>) {
    if ev_eat.is_empty() {
        return;
    }
    ev_eat.clear();

    let (mut text, mut score) = q.single_mut();
    score.0 += 1;
    text.sections[1].value = score.0.to_string();
}

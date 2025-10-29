// module containing debug plugin which displays debug information like FPS, frame time, etc

use bevy::{diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin}, prelude::*, text::FontSmoothing};

use crate::constants::{DISTANCE_SCALE, TIME_SCALE, TO_1_DAY, TO_1_MONTH};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FrameTimeDiagnosticsPlugin::default());
        app.add_systems(Startup, create_text);
        app.add_systems(Update, update_fps_text);
    }
}

#[derive(Component)]
struct FpsText;

fn create_text(
    mut cmds: Commands,
    assets: Res<AssetServer>,
    windows: Query<&Window>,
) {
    let window = windows.single().unwrap();
    let window_width = window.resolution.width();

    let font = TextFont {
        font: assets.load("fonts/Monocraft.ttf"),
        font_size: 23.0,
        font_smoothing: FontSmoothing::None,
        ..Default::default()
    };

    cmds.spawn((
        Node {
            flex_direction: FlexDirection::Column,
            padding: UiRect::axes(Val::Px(10.0), Val::Px(5.0)),
            row_gap: Val::Px(4.0),
            ..Default::default()
        },
        BackgroundColor(Color::BLACK),
    ))
    .with_children(|builder| {
        spawn_text_with_comp(builder, &font, "FPS: ", FpsText);

        // time scale
        let time_span = if TIME_SCALE == TO_1_DAY {
            "1 day"
        } else if TIME_SCALE == TO_1_MONTH {
            "1 month"
        } else {
            "1 year"
        };
        spawn_text(builder, &font, format!("1 sec = {time_span}"));

        // distance scale
        let px_to_m = (DISTANCE_SCALE / window_width as f64).round();
        spawn_text(builder, &font, format!("1px = {}m", group_digits(px_to_m as _, ' ')));
    });
}

fn update_fps_text(
    diagnostics: Res<DiagnosticsStore>,
    query: Query<&mut TextSpan, With<FpsText>>,
) {
    for mut text in query {
        if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) && let Some(value) = fps.average() {
            text.0 = format!("{}", value.round());
        }
    }
}

// helper functions
fn spawn_text(
    builder: &mut ChildSpawnerCommands,
    font: &TextFont,
    text: impl Into<String>,
) {
    builder.spawn((
        Text::new(text),
        font.clone(),
    ));
}

fn spawn_text_with_comp(
    builder: &mut ChildSpawnerCommands,
    font: &TextFont,
    label: &str,
    comp: impl Component,
) {
    builder
        .spawn((
            Text::new(label),
            font.clone(),
        ))
        .with_child((
            TextSpan::default(),
            font.clone(),
            comp,
        ));
}

fn group_digits(number: u64, separator: char) -> String {
    let mut grouped = String::new();
    let number_str = number.to_string();
    for (i, ch) in number_str.char_indices() {
        if i % 3 == 0 && i != 0 {
            grouped.push(separator);
        }
        grouped.push(ch);
    }
    grouped
}

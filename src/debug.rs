// module containing debug plugin which displays debug information like FPS, frame time, etc

use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
    text::FontSmoothing,
};

use crate::{
    camera::{DEFAULT_DISTANCE, PointCamera},
    constants::{TIME_SCALE, TO_1_DAY, TO_1_MONTH},
};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FrameTimeDiagnosticsPlugin::default())
            .add_systems(Startup, create_text)
            .add_systems(Update, (update_fps_text, update_camera_stats));
    }
}

#[derive(Component)]
struct FpsText;

#[derive(Component)]
struct PitchText;

#[derive(Component)]
struct YawText;

#[derive(Component)]
struct ZoomText;

fn create_text(mut cmds: Commands, assets: Res<AssetServer>) {
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

        // camera stats
        spawn_text_with_comp(builder, &font, "Pitch: ", PitchText);
        spawn_text_with_comp(builder, &font, "Yaw: ", YawText);
        spawn_text_with_comp(builder, &font, "Zoom: ", ZoomText);
    });
}

fn update_fps_text(
    diagnostics: Res<DiagnosticsStore>,
    query: Single<&mut TextSpan, With<FpsText>>,
) {
    let mut text = query.into_inner();

    if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS)
        && let Some(value) = fps.average()
    {
        text.0 = format!("{}", value.round());
    }
}

fn update_camera_stats(
    mut texts: ParamSet<(
        Single<&mut TextSpan, With<PitchText>>,
        Single<&mut TextSpan, With<YawText>>,
        Single<&mut TextSpan, With<ZoomText>>,
    )>,
    camera: Single<&PointCamera>,
) {
    // pitch
    {
        let mut text = texts.p0().into_inner();
        text.0 = format!("{:.2}°", camera.pitch.to_degrees());
    }

    // yaw
    {
        let mut text = texts.p1().into_inner();
        text.0 = format!("{:.2}°", camera.yaw.to_degrees());
    }

    // zoom
    {
        let mut text = texts.p2().into_inner();
        let zoom = DEFAULT_DISTANCE / camera.distance;
        text.0 = format!("{zoom:.2}x");
    }
}

// helper functions
fn spawn_text(builder: &mut ChildSpawnerCommands, font: &TextFont, text: impl Into<String>) {
    builder.spawn((Text::new(text), font.clone()));
}

fn spawn_text_with_comp(
    builder: &mut ChildSpawnerCommands,
    font: &TextFont,
    label: &str,
    comp: impl Component,
) {
    builder.spawn((Text::new(label), font.clone())).with_child((
        TextSpan::default(),
        font.clone(),
        comp,
    ));
}

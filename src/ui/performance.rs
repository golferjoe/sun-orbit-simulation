// module containing debug plugin which displays debug information like FPS, frame time, etc

use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
    text::FontSmoothing,
};

use crate::{
    camera::{DEFAULT_DISTANCE, PointCamera},
    ui::egui::Gui,
};

#[derive(Component)]
pub struct PerformanceText;

#[derive(Component)]
pub struct FpsText;

#[derive(Component)]
pub struct PitchText;

#[derive(Component)]
pub struct YawText;

#[derive(Component)]
pub struct ZoomText;

pub fn create_text(mut cmds: Commands, assets: Res<AssetServer>) {
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
        spawn_text_with_comp(builder, &font, "Pitch: ", PitchText);
        spawn_text_with_comp(builder, &font, "Yaw: ", YawText);
        spawn_text_with_comp(builder, &font, "Zoom: ", ZoomText);
    });
}

pub fn update_fps_text(
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

pub fn update_camera_stats(
    settings: Res<Gui>,
    all_text: Query<&mut Node, With<PerformanceText>>,
    mut texts: ParamSet<(
        Single<&mut TextSpan, With<PitchText>>,
        Single<&mut TextSpan, With<YawText>>,
        Single<&mut TextSpan, With<ZoomText>>,
    )>,
    camera: Single<&PointCamera>,
) {
    for mut node in all_text {
        node.display = if settings.show_performance {
            Display::Block
        } else {
            Display::None
        };
    }

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

fn spawn_text_with_comp(
    builder: &mut ChildSpawnerCommands,
    font: &TextFont,
    label: &str,
    comp: impl Component,
) {
    builder
        .spawn((Text::new(label), font.clone(), PerformanceText))
        .with_child((TextSpan::default(), font.clone(), comp));
}

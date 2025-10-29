// module containing debug plugin which displays debug information like FPS, frame time, etc

use bevy::{diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin}, prelude::*};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FrameTimeDiagnosticsPlugin::default());
        app.add_systems(Startup, setup);
        app.add_systems(Update, update_fps);
    }
}

#[derive(Component)]
struct FpsText;

fn setup(mut cmds: Commands) {
    cmds.spawn((
        Text::new("FPS: 0"),
        TextShadow::default(),
        Node {
            position_type: PositionType::Absolute,
            top: px(5),
            left: px(5),
            ..Default::default()
        },
        FpsText,
    ));
}

fn update_fps(
    diagnostics: Res<DiagnosticsStore>,
    query: Query<&mut Text, With<FpsText>>,
) {
    for mut text in query {
        if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) && let Some(value) = fps.average() {
            text.0 = format!("FPS: {}", value.round());
        }
    }
}

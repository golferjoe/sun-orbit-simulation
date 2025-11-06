use bevy::{
    app::{App, Plugin, Startup, Update},
    diagnostic::FrameTimeDiagnosticsPlugin,
    ecs::{
        schedule::{IntoScheduleConfigs, common_conditions::run_once},
        system::{ResMut, Single},
    },
    prelude::Result,
};
use bevy_egui::{
    EguiContextSettings, EguiContexts, EguiPlugin, EguiPrimaryContextPass,
    egui::{FontId, TextStyle},
};

use crate::ui::{egui::Gui, performance};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Gui::default())
            .add_plugins((EguiPlugin::default(), FrameTimeDiagnosticsPlugin::default()))
            .add_systems(
                EguiPrimaryContextPass,
                ((setup_fonts, setup_ui_scale).run_if(run_once), draw_gui),
            )
            .add_systems(Startup, performance::create_text)
            .add_systems(
                Update,
                (
                    performance::update_fps_text,
                    performance::update_camera_stats,
                ),
            );
    }
}

fn setup_ui_scale(mut settings: Single<&mut EguiContextSettings>) {
    settings.scale_factor = 1.25;
}

fn setup_fonts(mut contexts: EguiContexts) -> Result {
    let ctx = contexts.ctx_mut()?;

    ctx.all_styles_mut(move |style| {
        let font = FontId::new(16.0, bevy_egui::egui::FontFamily::Monospace);
        style.text_styles = [
            (TextStyle::Heading, font.clone()),
            (TextStyle::Button, font.clone()),
            (TextStyle::Body, font.clone()),
            (TextStyle::Small, font.clone()),
        ]
        .into();
    });

    Ok(())
}

fn draw_gui(mut contexts: EguiContexts, mut gui: ResMut<Gui>) -> Result {
    gui.draw(&mut contexts)
}

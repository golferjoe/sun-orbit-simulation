use bevy::ecs::resource::Resource;
use bevy_egui::{EguiContexts, egui};

#[derive(Resource)]
pub struct Gui {
    pub open: bool,
    pub show_performance: bool,
    pub time_scale: TimeScale,
    pub show_orbits: bool,
}

// unfortunately i had to embed it to make it work
impl Gui {
    pub fn draw(&mut self, contexts: &mut EguiContexts) -> bevy::prelude::Result {
        egui::Window::new("Debug")
            .collapsible(false)
            .resizable(false)
            .open(&mut self.open)
            .show(contexts.ctx_mut()?, |ui| {
                ui.set_width(300.0);
                ui.checkbox(&mut self.show_performance, "Show performance stats");
                ui.checkbox(&mut self.show_orbits, "Show orbits");

                egui::ComboBox::from_label("Time scale")
                    .selected_text(format!("1s = {:?}", self.time_scale))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.time_scale, TimeScale::Day, "Day");
                        ui.selectable_value(&mut self.time_scale, TimeScale::Month, "Month");
                        ui.selectable_value(&mut self.time_scale, TimeScale::Year, "Year");
                    });
            });
        Ok(())
    }
}

impl Default for Gui {
    fn default() -> Self {
        Self {
            open: true,
            show_performance: false,
            time_scale: TimeScale::Day,
            show_orbits: true,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum TimeScale {
    Day,
    Month,
    Year,
}

impl TimeScale {
    pub fn to_seconds(&self) -> f64 {
        match self {
            Self::Day => 86_400.0,
            Self::Month => 2.6298e6,
            Self::Year => 3.15576e7,
        }
    }
}

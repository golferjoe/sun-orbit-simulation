mod constants;
mod debug;
mod sun;
mod earth;

use bevy::{prelude::*, window::WindowResolution};

use crate::{debug::DebugPlugin, earth::EarthPlugin, sun::SunPlugin};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "My First Window".to_string(),
                resolution: WindowResolution::new(1280, 720),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugins((DebugPlugin, SunPlugin, EarthPlugin))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut cmds: Commands) {
    cmds.spawn(Camera2d);
}

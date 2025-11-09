use bevy::prelude::*;

use crate::constants::SUN_RADIUS;

pub struct SunPlugin;

impl Plugin for SunPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AmbientLight {
            color: Color::linear_rgb(0.05, 0.05, 0.05),
            brightness: 1.0,
            ..Default::default()
        })
        .add_systems(Startup, setup);
    }
}

fn setup(
    asset_server: Res<AssetServer>,
    mut cmds: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let texture = asset_server.load("textures/sun.jpg");

    let material = materials.add(StandardMaterial {
        emissive: LinearRgba::rgb(0.9, 0.4, 0.0),
        base_color_texture: Some(texture),
        unlit: true,
        ..Default::default()
    });

    let mesh = meshes.add(Sphere::new(SUN_RADIUS).mesh().ico(8).unwrap());

    // create sun sphere
    cmds.spawn((Mesh3d(mesh), MeshMaterial3d(material), Transform::default()));

    // create light coming from it
    cmds.spawn((
        PointLight {
            intensity: 100_000_000.0,
            range: 10_000.0,
            radius: SUN_RADIUS,
            shadows_enabled: false,
            color: Color::linear_rgb(1.0, 0.98, 0.9),
            ..Default::default()
        },
        Transform::from_translation(Vec3::ZERO),
    ));
}

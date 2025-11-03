use bevy::prelude::*;

use crate::constants::SUN_RADIUS;

pub struct SunPlugin;

impl Plugin for SunPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn setup(
    mut cmds: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let material = materials.add(StandardMaterial {
        emissive: LinearRgba::rgb(0.9, 0.4, 0.0),
        ..Default::default()
    });

    let mesh = meshes.add(Sphere::new(SUN_RADIUS).mesh().ico(8).unwrap());

    // create sun sphere
    cmds.spawn((Mesh3d(mesh), MeshMaterial3d(material), Transform::default()));

    // create light coming from it
    cmds.spawn((
        PointLight {
            intensity: 10000000.0,
            range: 10000.0,
            radius: SUN_RADIUS,
            shadows_enabled: true,
            ..Default::default()
        },
        Transform::default(),
    ));
}

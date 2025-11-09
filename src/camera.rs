use std::f32::consts::{FRAC_PI_2, PI};

use bevy::{
    core_pipeline::{Skybox, tonemapping::Tonemapping},
    input::mouse::{AccumulatedMouseMotion, AccumulatedMouseScroll},
    pbr::ScreenSpaceAmbientOcclusion,
    post_process::bloom::Bloom,
    prelude::*,
    render::render_resource::{TextureViewDescriptor, TextureViewDimension},
};
use bevy_egui::input::egui_wants_any_pointer_input;

const PITCH_LIMIT: f32 = FRAC_PI_2 - 0.01;
const MIN_DISTANCE: f32 = 10.0;
const MAX_DISTANCE: f32 = 1000.0;

const YAW_SENSITIVITY: f32 = 0.005;
const PITCH_SENSITIVITY: f32 = 0.003;
const ZOOM_SENSITIVITY: f32 = 2.0;

pub const DEFAULT_DISTANCE: f32 = 50.0;
// both values below are in degrees
const DEFAULT_PITCH: f32 = 35.0;
const DEFAULT_YAW: f32 = -90.0;

const SKYBOX_PATH: &str = "stars_skybox.png";

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera).add_systems(
            Update,
            (
                camera_movement.run_if(not(egui_wants_any_pointer_input)),
                setup_skybox,
            ),
        );
    }
}

#[derive(Component)]
pub struct PointCamera {
    pub distance: f32, // or radius or zoom, however you call it
    pub pitch: f32,
    pub yaw: f32,
}

impl PointCamera {
    pub fn new() -> Self {
        Self {
            distance: DEFAULT_DISTANCE,
            pitch: DEFAULT_PITCH.to_radians() as f32,
            yaw: DEFAULT_YAW.to_radians() as f32,
        }
    }
}

#[derive(Resource)]
struct Cubemap {
    is_loaded: bool,
    image_handle: Handle<Image>,
}

fn setup_camera(mut cmds: Commands, assets: Res<AssetServer>) {
    let skybox_handle = assets.load(SKYBOX_PATH);

    cmds.spawn((
        Camera3d::default(),
        // Camera {
        //     clear_color: ClearColorConfig::Custom(Color::WHITE),
        //     ..Default::default()
        // },
        Transform::default().looking_at(Vec3::ZERO, Vec3::Y),
        Tonemapping::TonyMcMapface,
        Bloom::NATURAL,
        Msaa::Off,
        ScreenSpaceAmbientOcclusion::default(),
        PointCamera::new(),
        Skybox {
            image: skybox_handle.clone(),
            brightness: 500.0,
            ..Default::default()
        },
    ));

    cmds.insert_resource(Cubemap {
        is_loaded: false,
        image_handle: skybox_handle,
    });
}

// i hate this i hate this i hate this i hate this
fn setup_skybox(
    assets: Res<AssetServer>,
    mut images: ResMut<Assets<Image>>,
    mut cubemap: ResMut<Cubemap>,
    skyboxes: Query<&mut Skybox>,
) {
    if !cubemap.is_loaded && assets.load_state(&cubemap.image_handle).is_loaded() {
        info!("Updating skybox' texture view descriptor");
        let image = images.get_mut(&cubemap.image_handle).unwrap();
        if image.texture_descriptor.array_layer_count() == 1 {
            image.reinterpret_stacked_2d_as_array(image.height() / image.width());
            image.texture_view_descriptor = Some(TextureViewDescriptor {
                dimension: Some(TextureViewDimension::Cube),
                ..Default::default()
            });
        }

        for mut skybox in skyboxes {
            skybox.image = cubemap.image_handle.clone();
        }

        cubemap.is_loaded = true;
    }
}

fn camera_movement(
    query: Single<(&mut Transform, &mut PointCamera)>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mouse_motion: Res<AccumulatedMouseMotion>,
    mouse_scroll: Res<AccumulatedMouseScroll>,
) {
    let (mut transform, mut camera) = query.into_inner();
    let delta = mouse_motion.delta;

    let delta_yaw = delta.x * YAW_SENSITIVITY;
    let delta_pitch = delta.y * PITCH_SENSITIVITY;

    if mouse_buttons.pressed(MouseButton::Left) {
        camera.yaw += delta_yaw;
        camera.yaw = camera.yaw % (2.0 * PI); // wrap yaw so that it doesnt overflow if we keep spinning

        camera.pitch += delta_pitch;
        camera.pitch = camera.pitch.clamp(-PITCH_LIMIT, PITCH_LIMIT);
    }

    camera.distance -= mouse_scroll.delta.y * ZOOM_SENSITIVITY;
    camera.distance = camera.distance.clamp(MIN_DISTANCE, MAX_DISTANCE);

    transform.translation.x = camera.distance * camera.pitch.cos() * camera.yaw.cos();
    transform.translation.y = camera.distance * camera.pitch.sin();
    transform.translation.z = camera.distance * camera.pitch.cos() * camera.yaw.sin();

    transform.look_at(Vec3::ZERO, Vec3::Y);
}

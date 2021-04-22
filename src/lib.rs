use bevy_mod_picking::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn main() {
    let mut app = App::build();
    app.add_plugins(DefaultPlugins);
    app.add_plugin(PickingPlugin);
    app.add_plugin(DebugCursorPickingPlugin);

    #[cfg(target_arch = "wasm32")]
    app.add_plugin(bevy_webgl2::WebGL2Plugin);
    app.add_startup_system(setup.system()).run();

    app.add_plugin(LogPlugin::default())
        .add_system(hello_wasm_system.system())
        .run();
}

fn hello_wasm_system() {
    info!("hello wasm");
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // add entities to the world
    // plane
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..Default::default()
    });
    // cube
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_translation(Vec3::new(0.0, 0.5, 0.0)),
        ..Default::default()
    });
    // light
    commands.spawn_bundle(LightBundle {
        transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
        ..Default::default()
    });
    // camera
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_translation(Vec3::new(-2.0, 2.5, 5.0))
            .looking_at(Vec3::default(), Vec3::Y),
        ..Default::default()
        .insert_bundle(PickingCameraBundle::default());
}

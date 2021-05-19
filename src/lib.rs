use bevy::prelude::*;
use bevy_mod_picking::*;
use wasm_bindgen::prelude::*;

mod pieces;
use pieces::PiecesPlugin;

mod board;
use board::BoardPlugin;

#[wasm_bindgen]
pub fn main() {
    let mut app = App::build();
    app.insert_resource(Msaa { samples: 4 });
    app.insert_resource(WindowDescriptor {
        title: "RustGame!".to_string(),
        width: 1000.0,
        height: 1000.0,
        ..Default::default()
    });
    app.add_plugins(DefaultPlugins);
    app.add_plugin(PickingPlugin);
    app.add_plugin(DebugCursorPickingPlugin);
    app.add_plugin(InteractablePickingPlugin);
    app.add_plugin(HighlightablePickingPlugin);
    app.add_plugin(BoardPlugin);
    app.add_plugin(PiecesPlugin);

    #[cfg(target_arch = "wasm32")]
    app.add_plugin(bevy_webgl2::WebGL2Plugin);

    app.add_startup_system(setup.system()).run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 8.0 })),
        material: materials.add(Color::rgb(1.0, 0.9, 0.9).into()),
        transform: Transform::from_translation(Vec3::new(4., 0., 4.)),
        ..Default::default()
    });

    commands.spawn_bundle(LightBundle {
        transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
        ..Default::default()
    });

    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_translation(Vec3::new(-30.0, 16.0, 4.0))
                .looking_at(Vec3::default(), Vec3::Y),
            ..Default::default()
        })
        .insert_bundle(PickingCameraBundle::default());
}

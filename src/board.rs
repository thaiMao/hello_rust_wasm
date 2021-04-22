use bevy::prelude::*;
use bevy_mod_picking::*;

pub fn create_board(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
) {
  let mesh = meshes.add(Mesh::from(shape::Plane { size: 1.0 }));
  let white_material = materials.add(Color::rgb(1.0, 0.9, 0.9).into());
  let black_material = materials.add(Color::rgb(0.0, 0.1, 0.1).into());

  for i in 0..8 {
    for j in 0..8 {
      commands
        .spawn_bundle(PbrBundle {
          mesh: mesh.clone(),
          material: if (i + j + 1) % 2 == 0 {
            white_material.clone()
          } else {
            black_material.clone()
          },
          transform: Transform::from_translation(Vec3::new(i as f32, 0., j as f32)),
          ..Default::default()
        })
fn color_squares(
  selected_square: Res<SelectedSquare>,
  mut materials: ResMut<Assets<StandardMaterial>>,
  query: Query<(Entity, &Square, &Handle<StandardMaterial>, &Hover)>,
) {
  for (entity, square, material_handle, hover) in query.iter() {
    let material = materials.get_mut(material_handle).unwrap();

    material.base_color = if hover.hovered() == true {
      Color::rgb(0.8, 0.3, 0.3)
    } else if Some(entity) == selected_square.entity {
      Color::rgb(0.9, 0.1, 0.1)
    } else if square.is_white() {
      Color::rgb(1.0, 0.9, 0.9)
    } else {
      Color::rgb(0.0, 0.1, 0.1)
    }
  }
}
fn select_square(
  query: Query<(Entity, &Square, &Handle<StandardMaterial>, &Hover)>,
  mouse_button_inputs: Res<Input<MouseButton>>,
  mut selected_square: ResMut<SelectedSquare>,
) {
  if !mouse_button_inputs.just_pressed(MouseButton::Left) {
    return;
  }

  for (entity, _square, _material_handle, hover) in query.iter() {
    if hover.hovered() == true {
      selected_square.entity = Some(entity);
    }
  }
    }
  }
}

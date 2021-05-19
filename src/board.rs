use crate::pieces::*;
use bevy::prelude::*;
use bevy_mod_picking::*;
use wasm_bindgen::prelude::*;

#[derive(Default)]
struct SelectedSquare {
  entity: Option<Entity>,
}

#[derive(Default)]
struct SelectedPiece {
  entity: Option<Entity>,
}

#[wasm_bindgen]
extern "C" {
  // Use `js_namespace` here to bind `console.log(..)` instead of just
  // `log(..)`
  #[wasm_bindgen(js_namespace = console)]
  fn log(s: &str);

  #[wasm_bindgen(js_namespace = console, js_name = log)]
  fn log_position(x: u8, y: u8);

}

pub struct Square {
  pub x: u8,
  pub y: u8,
}

impl Square {
  fn is_white(&self) -> bool {
    (self.x + self.y + 1) % 2 == 0
  }
}

fn create_board(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
) {
  let mesh = meshes.add(Mesh::from(shape::Plane { size: 1.0 }));

  for i in 0..8 {
    for j in 0..8 {
      commands
        .spawn_bundle(PbrBundle {
          mesh: mesh.clone(),
          material: if (i + j + 1) % 2 == 0 {
            materials.add(Color::rgb(1.0, 0.9, 0.9).into())
          } else {
            materials.add(Color::rgb(0.0, 0.1, 0.1).into())
          },
          transform: Transform::from_translation(Vec3::new(i as f32, 0., j as f32)),
          ..Default::default()
        })
        .insert_bundle(PickableBundle::default())
        .insert(Square { x: i, y: j });
    }
  }
}

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
  square_query: Query<(Entity, &Square, &Handle<StandardMaterial>, &Hover)>,
  mut piece_query: Query<(Entity, &mut Piece)>,
  mouse_button_inputs: Res<Input<MouseButton>>,
  mut selected_square: ResMut<SelectedSquare>,
  mut selected_piece: ResMut<SelectedPiece>,
) {
  if !mouse_button_inputs.just_pressed(MouseButton::Left) {
    return;
  }

  for (entity, _square, _material_handle, hover) in square_query.iter() {
    if hover.hovered() == true {
      log("hovered");
      selected_square.entity = Some(entity);
    } else {
      log("not hovered");
    }
  }

  if let Some(square_entity) = selected_square.entity {
    if let Ok((_sq_entity, square, _a, _b)) = square_query.get(square_entity) {
      if let Some(selected_piece_entity) = selected_piece.entity {
        log("selected_piece.entity exists");
        if let Ok((_piece_entity, mut piece)) = piece_query.get_mut(selected_piece_entity) {
          log("query for selected_piece_entity return Ok result");
          log("square position is...");
          log_position(square.x, square.y);
          log("attempt to assign");
          piece.x = square.x;
          piece.y = square.y;
        }
        // selected_square.entity = None;
        // selected_piece.entity = None;
      } else {
        log("selected_piece.entity does not exist");
        if let Some(selected_square_entity) = selected_square.entity {
          if let Ok((_square_entity, sq, _a, _b)) = square_query.get(selected_square_entity) {
            for (piece_entity, piece) in piece_query.iter_mut() {
              log("square position is...");
              log_position(sq.x, sq.y);
              if piece.x == sq.x && piece.y == sq.y {
                selected_piece.entity = Some(piece_entity);
                log("assigned");
                break;
              }
            }
          }
        }
      }
    }
  }
}

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
  fn build(&self, app: &mut AppBuilder) {
    app
      .init_resource::<SelectedSquare>()
      .init_resource::<SelectedPiece>()
      .add_startup_system(create_board.system())
      .add_system(color_squares.system())
      .add_system(select_square.system());
  }
}

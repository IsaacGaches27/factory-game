use bevy::asset::AssetContainer;
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use crate::terrain::Terrain;
use bevy::window::PrimaryWindow;
use crate::container::ItemContainer;
use crate::conveyor::{ConveyorLogic, TailConveyor};
use crate::processor::Processor;
use crate::producer::Producer;

fn place_conveyors(
    buttons: Res<Input<MouseButton>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mut tilemap_query: Query<(&mut TileStorage,Entity),Without<Terrain>>,
    terrain: Query<(&TileStorage),With<Terrain>>,
    tiles: Query<&TileTextureIndex>,
    mut commands: Commands,
    mut tails: Query<&TailConveyor>,
) {
    if buttons.pressed(MouseButton::Left) || buttons.pressed(MouseButton::Right) {
        let (camera, camera_transform) = camera_query.single();
        let window = window_query.single();

        if let Some(world_position) = window.cursor_position()
            .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
            .map(|ray| ray.origin.truncate())
        {
            let (mut tilemap,tilemap_entity) = tilemap_query.single_mut();
            let tile_pos = TilePos{x: ((world_position.x + 7.5)/15.0) as u32,y: ((world_position.y + 7.5)/15.0) as u32};

            if tilemap.get(&tile_pos).is_some(){
                return;
            }

            let mut adj_entity = None;
            let mut direction = (0,0);

            'outer: for x in 0..3{
                for y in 0..3{
                    if (x+y) % 2 == 0 {continue;}
                    if let Some(adj) = tilemap.get(&TilePos{x:tile_pos.x + x - 1,y:tile_pos.y + y - 1}){
                        if tails.contains(adj){
                            commands.entity(adj).remove::<TailConveyor>();
                            adj_entity = Some(adj);
                            direction = (x,y);
                            break 'outer;
                        }
                    }
                }
            }

            let index = match direction{
                (0,1) => 2,
                (2,1) => 3,
                (1,0) => 0,
                (1,2) => 1,
                _ => 0,
            };

            let tile_entity = if buttons.pressed(MouseButton::Right) {
                if tiles.get(terrain.single().get(&tile_pos).unwrap()).unwrap().0 == 7{
                    commands
                        .spawn(TileBundle {
                            position: tile_pos,
                            tilemap_id: TilemapId(tilemap_entity),
                            texture_index: TileTextureIndex(16),
                            ..Default::default()
                        })
                        .insert((ItemContainer::new(10), Producer{ timer: 0 }, ConveyorLogic{ incoming: adj_entity, timer: 0 }, TailConveyor()))
                        .id()
                }
                else{
                    commands
                        .spawn(TileBundle {
                            position: tile_pos,
                            tilemap_id: TilemapId(tilemap_entity),
                            texture_index: TileTextureIndex(17),
                            ..Default::default()
                        })
                        .insert((ConveyorLogic{ incoming: adj_entity, timer: 0 },ItemContainer::new(10),TailConveyor(),Processor{
                            inputs: if let Some(entity) = adj_entity { vec![entity] } else { vec![] },
                            required_input_quantities: vec![1],
                            output_item_id: 0,
                        }))
                        .id()
                }
            }
            else{
                commands
                    .spawn(TileBundle {
                        position: tile_pos,
                        tilemap_id: TilemapId(tilemap_entity),
                        texture_index: TileTextureIndex(index),
                        ..Default::default()
                    })
                    .insert((ConveyorLogic{ incoming: adj_entity, timer: 0 },ItemContainer::new(1),TailConveyor()))
                    .id()
            };

            tilemap.set(&tile_pos,tile_entity);
        }
    }
}

pub struct BuildPlugin;

impl Plugin for BuildPlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(Update,place_conveyors);
    }
}
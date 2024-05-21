use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use crate::world::Terrain;
use bevy::window::PrimaryWindow;
use crate::container::ItemContainer;
use crate::conveyor::ConveyorLogic;

fn place_conveyors(
    buttons: Res<Input<MouseButton>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mut tilemap_query: Query<(&mut TileStorage,Entity),Without<Terrain>>,
    mut commands: Commands,
) {
    if buttons.just_pressed(MouseButton::Left) {
        let (camera, camera_transform) = camera_query.single();
        let window = window_query.single();

        if let Some(world_position) = window.cursor_position()
            .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
            .map(|ray| ray.origin.truncate())
        {
            let (mut tilemap,tilemap_entity) = tilemap_query.single_mut();
            let tile_pos = TilePos{x: ((world_position.x + 7.5)/15.0) as u32,y: ((world_position.y + 7.5)/15.0) as u32};

            let mut conveyor_logic = ConveyorLogic::default();

            for x in 0..3{
                for y in 0..3{
                    if let Some(adj) = tilemap.get(&TilePos{x:tile_pos.x + x - 1,y:tile_pos.y + y - 1}){
                        if x == 1 && y == 1 {return;}
                        conveyor_logic.incoming = Some(adj);
                    }
                }
            }

            let tile_entity = commands
                .spawn(TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    ..Default::default()
                })
                .insert((conveyor_logic,ItemContainer::default()))
                .id();

            tilemap.set(&tile_pos,tile_entity);
        }
    }
}

pub struct BuildPlugin();

impl Plugin for BuildPlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(Update,place_conveyors);
    }
}
use bevy_ecs_tilemap::prelude::*;
use bevy::prelude::*;
use simdnoise::NoiseBuilder;
use crate::container::ItemContainer;
use crate::conveyor::TailConveyor;

fn spawn_tilemap(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let size = 1000;

    let map_size = TilemapSize { x: size, y: size };

    let mut tile_storage = TileStorage::empty(map_size);
    let tilemap_entity = commands.spawn_empty().id();

    let noise_1 = NoiseBuilder::fbm_2d(size as usize, size as usize).with_seed(1337).with_freq(0.1).generate_scaled(0.0, 1.0);
    let noise_2 = NoiseBuilder::fbm_2d(size as usize, size as usize).with_seed(0).with_freq(0.05).generate_scaled(0.0, 1.0);
    let noise_3 = NoiseBuilder::fbm_2d(size as usize, size as usize).with_seed(2092).with_freq(0.02).generate_scaled(0.0, 1.0);

    for x in 0..size{
        for y in 0..size{
            let tile_index = if ((x as f32-200.)*(x as f32-200.) + (y as f32-700.)*(y as f32-700.)).sqrt() < noise_3[(x*size+y) as usize] * 400.{ // stone biome
                if noise_1[(x*size+y) as usize] < 0.1 {7} // iron
                else if noise_2[(x*size+y) as usize] < 0.05 {5} // coal
                else if noise_1[(x*size+y) as usize] < 0.8  {2} // stone
                else if noise_2[(x*size+y) as usize] < 0.2 {0} // grass
                else {1} //dirt
            }
            else if ((x as f32-800.)*(x as f32-800.) + (y as f32-200.)*(y as f32-200.)).sqrt() < noise_3[(x*size+y) as usize] * 400.{ // desert biome
                if noise_1[(x*size+y) as usize] < 0.02 {7} // iron
                else if noise_2[(x*size+y) as usize] < 0.1 {5} // coal
                else if noise_1[(x*size+y) as usize] < 0.1  {2} // stone
                else {3} //sand
            }
            else{
                if noise_1[(x*size+y) as usize] < 0.05 {7} // iron
                else if noise_2[(x*size+y) as usize] < 0.08 {5} // coal
                else if noise_1[(x*size+y) as usize] < 0.1 {2} // stone
                else if noise_2[(x*size+y) as usize] < 0.7 {0} // grass
                else {1} //dirt
            };

            let tile_pos = TilePos {x, y};
            let tile_entity = commands
                .spawn(TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    texture_index: TileTextureIndex(tile_index),
                    ..Default::default()
                })
                .id();

            tile_storage.set(&tile_pos, tile_entity);
        }
    }

    let tile_size = TilemapTileSize { x: 15.0, y: 15.0 };
    let grid_size = tile_size.into();
    let map_type = TilemapType::default();

    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size,
        map_type,
        size: map_size,
        storage: tile_storage,
        texture: TilemapTexture::Single(asset_server.load("terrain.png")),
        tile_size,
        ..Default::default()
    }).insert(Terrain);

    let tile_storage = TileStorage::empty(map_size);
    let tilemap_entity = commands.spawn_empty().id();

    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size,
        map_type,
        size: map_size,
        storage: tile_storage,
        texture: TilemapTexture::Single(asset_server.load("buildings.png")),
        tile_size: TilemapTileSize { x: 15.0, y: 15.0 },
        transform: Transform::from_xyz(0.,0.,1.0),
        ..Default::default()
    });
}

#[derive(Component)]
pub struct Terrain;

pub struct WorldPlugin;

impl Plugin for WorldPlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(Startup,spawn_tilemap);
    }
}
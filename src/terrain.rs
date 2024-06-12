use bevy_ecs_tilemap::prelude::*;
use bevy::prelude::*;
use simdnoise::NoiseBuilder;
use crate::container::ItemContainer;
use crate::conveyor::TailConveyor;
use rand::{Rng, thread_rng};

fn spawn_tilemap(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let size = 1000;

    let map_size = TilemapSize { x: size, y: size };

    let mut tile_storage = TileStorage::empty(map_size);
    let tilemap_entity = commands.spawn_empty().id();

    let seed = thread_rng().gen_range(-100000..100000);

    let noise_1 = NoiseBuilder::fbm_2d_offset(seed as f32,size as usize,seed as f32, size as usize).with_seed(seed*-2).with_freq(0.1).generate_scaled(0.0, 1.0);
    let noise_2 = NoiseBuilder::fbm_2d_offset(seed as f32,size as usize,seed as f32, size as usize).with_seed(seed*-1).with_freq(0.06).generate_scaled(0.0, 1.0);
    let noise_3 = NoiseBuilder::fbm_2d_offset(seed as f32,size as usize,seed as f32, size as usize).with_seed(seed*4).with_freq(0.02).generate_scaled(0.0, 1.0);
    let noise_4 = NoiseBuilder::fbm_2d_offset(seed as f32,size as usize,seed as f32, size as usize).with_seed(seed*3).with_freq(0.002).with_octaves(2).generate_scaled(0.0, 1.0);
    let noise_5 = NoiseBuilder::fbm_2d_offset(seed as f32,size as usize,seed as f32, size as usize).with_seed(seed).with_freq(0.08).generate_scaled(0.0, 1.0);
    let noise_6 = NoiseBuilder::fbm_2d_offset(seed as f32,size as usize,seed as f32, size as usize).with_seed(seed*2).with_freq(0.1).generate_scaled(0.0, 1.0);

    for x in 0..size{
        for y in 0..size{
            let tile_index =
                if noise_4[(x*size+y) as usize] < 0.515 && noise_4[(x*size+y) as usize] > 0.485 {11}//water
                else if ((x as f32-100.)*(x as f32-100.) + (y as f32-800.)*(y as f32-800.)).sqrt() < noise_3[(x*size+y) as usize] * 500.{ // stone biome
                    if noise_1[(x*size+y) as usize] < 0.1 {7} // iron
                    else if noise_6[(x*size+y) as usize] * noise_2[(x*size+y) as usize] < 0.02 {9} // tungsten
                    else if noise_5[(x*size+y) as usize] < 0.07 {6} // copper
                    else if noise_2[(x*size+y) as usize] < 0.05 {5} // coal
                    else if noise_1[(x*size+y) as usize] < 0.8  {2} // stone
                    else if noise_2[(x*size+y) as usize] < 0.2 {0} // grass
                    else {1} //dirt
                }
                else if ((x as f32-900.)*(x as f32-900.) + (y as f32-200.)*(y as f32-200.)).sqrt() < noise_3[(x*size+y) as usize] * 500.{ // desert biome
                    if noise_2[(x*size+y) as usize] * noise_1[(x*size+y) as usize] < 0.01 {13} // adurite
                    else if noise_6[(x*size+y) as usize] * noise_3[(x*size+y) as usize] < 0.01 {8} // gold
                    else if noise_1[(x*size+y) as usize] < 0.02 {7} // iron
                    else if noise_5[(x*size+y) as usize] < 0.08 {6} // copper
                    else if noise_2[(x*size+y) as usize] < 0.1 {5} // coal
                    else if noise_1[(x*size+y) as usize] < 0.1  {2} // stone
                    else {3} //sand
                }
                else{ //grass
                    if noise_1[(x*size+y) as usize] < 0.05 {7} // iron
                    else if noise_5[(x*size+y) as usize] < 0.05 {6} // copper
                    else if noise_2[(x*size+y) as usize] < 0.08 {5} // coal
                    else if noise_1[(x*size+y) as usize] < 0.1 {2} // stone
                    else if noise_3[(x*size+y) as usize] < 0.1 {4} // clay
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
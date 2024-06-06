use bevy_ecs_tilemap::prelude::*;
use bevy::prelude::*;
use simdnoise::NoiseBuilder;
use crate::container::ItemContainer;
use crate::conveyor::TailConveyor;

fn spawn_tilemap(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let map_size = TilemapSize { x: 100, y: 100 };

    let mut tile_storage = TileStorage::empty(map_size);
    let tilemap_entity = commands.spawn_empty().id();

    let noise = NoiseBuilder::fbm_2d(100, 100).with_freq(0.1).generate_scaled(0.0, 1.0);

    for x in 0..100{
        for y in 0..100{
            let tile_index =
                if noise[(x*100+y) as usize] > 0.5 {2}
                else if noise[(x*100+y) as usize] > 0.2 {0}
                else {3};

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
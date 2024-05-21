use bevy_ecs_tilemap::prelude::*;
use bevy::prelude::*;

fn spawn_tilemap(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let map_size = TilemapSize { x: 32, y: 32 };

    let mut tile_storage = TileStorage::empty(map_size);
    let tilemap_entity = commands.spawn_empty().id();

    fill_tilemap(
        TileTextureIndex(0),
        map_size,
        TilemapId(tilemap_entity),
        &mut commands,
        &mut tile_storage,
    );

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
    }).insert(Terrain());

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
pub struct Terrain();

pub struct WorldPlugin();

impl Plugin for WorldPlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(Startup,spawn_tilemap);
    }
}
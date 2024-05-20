mod conveyor;
mod item;
mod container;

use bevy::prelude::*;
use crate::container::ItemContainer;
use crate::conveyor::{ConveyorLogic, ConveyorPlugin, TailConveyor};
use crate::item::Item;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(ConveyorPlugin())
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    let item = commands.spawn((
        SpriteBundle {
            texture: asset_server.load("grass.png"),
            transform: Transform::from_xyz(0., 0., 10.).with_scale(Vec3::splat(0.4)),
            ..default()
        }
    )).insert(Item::default()).id();

    let mut item_container = ItemContainer::default();
    item_container.add_item(item);
    item_container.set_block(true);

    let conveyor = commands.spawn((
        SpriteBundle {
            texture: asset_server.load("conveyor.png"),
            transform: Transform::from_xyz(0., 0., 0.),
            ..default()
        },
        ConveyorLogic { incoming: None,timer: 0 },
        item_container
    )).id();

    let item = commands.spawn((
        SpriteBundle {
            texture: asset_server.load("grass.png"),
            transform: Transform::from_xyz(15., 0., 10.).with_scale(Vec3::splat(0.4)),
            ..default()
        }
    )).insert(Item::default()).id();

    let mut item_container = ItemContainer::default();
    item_container.add_item(item);

    let conveyor = commands.spawn((
        SpriteBundle {
            texture: asset_server.load("conveyor.png"),
            transform: Transform::from_xyz(15., 0., 0.),
            ..default()
        },
        ConveyorLogic { incoming: Some(conveyor),timer: 0 },
        item_container,
    )).id();

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("conveyor.png"),
            transform: Transform::from_xyz(30., 0., 0.),
            ..default()
        },
        ConveyorLogic { incoming: Some(conveyor),timer: 0 },
        ItemContainer::default(),
        TailConveyor()
    ));
}
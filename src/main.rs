mod conveyor;
mod item;
mod container;

use bevy::prelude::*;
use crate::container::ItemContainer;
use crate::conveyor::{ConveyorLogic, ConveyorPlugin, TailConveyor};
use crate::item::Item;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ConveyorPlugin())
        .add_systems(Startup, setup)
        .run();
}

#[derive(Component)]
enum Direction {
    Up,
    Down,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    let item = commands.spawn((
        SpriteBundle {
            texture: asset_server.load("grass.png"),
            transform: Transform::from_xyz(50., 0., 0.).with_scale(Vec3::splat(0.5)),
            ..default()
        },
        Direction::Up,
    )).insert(Item::default()).id();

    let mut item_container = ItemContainer::default();
    item_container.add_item(item);
    item_container.set_block(true);

    let conveyor = commands.spawn((
        SpriteBundle {
            texture: asset_server.load("grass.png"),
            transform: Transform::from_xyz(100., 0., 0.),
            ..default()
        },
        Direction::Up,
    )).insert((ConveyorLogic { incoming: None,timer: 60 },item_container)).id();

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("grass.png"),
            transform: Transform::from_xyz(200., 0., 0.),
            ..default()
        },
        Direction::Up,
    )).insert((ConveyorLogic { incoming: Some(conveyor),timer: 0 },ItemContainer::default(),TailConveyor()));
}
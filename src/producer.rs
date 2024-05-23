use std::process::Command;
use bevy::prelude::*;
use bevy::prelude::KeyCode::Up;
use bevy_ecs_tilemap::prelude::TilePos;
use crate::container::ItemContainer;
use crate::conveyor::ConveyorLogic;
use crate::item::Item;

#[derive(Component)]
pub struct Producer{

}

fn update_producers(
    mut commands: Commands,
    mut producers: Query<(&mut ItemContainer, &mut Producer)>,
    asset_server: Res<AssetServer>,
){
    for (mut container,mut producer) in &mut producers{
        let item = commands.spawn((
            SpriteBundle{
                texture: asset_server.load("items.png"),
                transform: Transform::from_xyz(0.,0.,10.),
                ..default()
            }
        )).insert(Item::default()).id();

        container.add_item(item);
    }
}

pub struct ProducerPlugin;

impl Plugin for ProducerPlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(Update,update_producers);
    }
}
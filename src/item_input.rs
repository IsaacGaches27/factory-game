use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::TilePos;
use crate::container::ItemContainer;
use crate::conveyor::{ConveyorLogic, TailConveyor};
use crate::item::Item;

#[derive(Component)]
pub struct ItemInput {
    pub incoming: Option<Entity>,
    pub allowed_item_id: usize,
}

fn update_restricted_inputs(
    mut restricted_inputs: Query<(&mut ItemInput, &mut ItemContainer),Without<Item>>,
    mut containers: Query<(&mut ItemContainer),Without<ItemInput>>,
    //mut items: Query<&mut Transform,With<Item>>,
){
    for (input,mut container) in restricted_inputs.iter_mut(){
        let incoming = if let Some(entity) = input.incoming {entity} else{ continue };

        if !container.full(){
            let mut in_container = containers.get_mut(incoming).unwrap();

             if let Some(item) = in_container.take(){
                 container.add_item(item);
             }
        }
    }
}

pub struct UpdateItemInputs;

impl Plugin for UpdateItemInputs{
    fn build(&self, app: &mut App) {
        app.add_systems(Update,update_restricted_inputs);
    }
}
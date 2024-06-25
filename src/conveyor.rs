use bevy::prelude::*;
use bevy_ecs_tilemap::tiles::TilePos;
use crate::container::ItemContainer;
use crate::item::Item;

#[derive(Component,Default)]
pub struct ConveyorLogic{
    pub incoming: Option<Entity>,
    pub timer: u8,
}

#[derive(Component)]
pub struct TailConveyor();

pub fn update_conveyors(
    tail_conveyors: Query<Entity,With<TailConveyor>>,
    mut conveyors: Query<(&mut ConveyorLogic, &mut ItemContainer, &TilePos),Without<Item>>,
    mut items: Query<&mut Transform,With<Item>>,
){
    tail_conveyors.iter().for_each(|conveyor|{
        let mut next = update_conveyor(conveyor,&mut conveyors,&mut items);
        while let Some(current) = next {
            next = update_conveyor(current,&mut conveyors,&mut items)
        }
    });
}

fn update_conveyor(
    current: Entity,
    conveyors: &mut Query<(&mut ConveyorLogic, &mut ItemContainer, &TilePos),Without<Item>>,
    items: &mut Query<&mut Transform,With<Item>>,
) -> Option<Entity>{
    let (mut conveyor,mut container,_) = conveyors.get_mut(current).unwrap();

    if conveyor.timer <= 20 { conveyor.timer += 1; }
    container.set_block(conveyor.timer < 20);

    let incoming = conveyor.incoming?;

    if !container.full() && !container.blocked(){
        let (_,mut in_container,_) = conveyors.get_mut(incoming).unwrap();

        if let Some(item) = in_container.take(){
            let mut item_pos = items.get_mut(item).unwrap();

            let (mut conveyor,mut container,position) = conveyors.get_mut(current).unwrap();

            item_pos.translation = Vec3::new(position.x as f32 * 15.,position.y as f32 * 15.,item_pos.translation.z);
            conveyor.timer = 0;
            container.add_item(item);
            container.set_block(true);
        }
    }

    Some(incoming)
}

pub struct ConveyorPlugin;

impl Plugin for ConveyorPlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(Update,update_conveyors);
    }
}
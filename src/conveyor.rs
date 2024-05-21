use bevy::prelude::*;
use crate::container::ItemContainer;
use crate::item::Item;

#[derive(Component,Default)]
pub struct ConveyorLogic{
    pub incoming: Option<Entity>,
    pub timer: u16,
}

#[derive(Component)]
pub struct TailConveyor();

fn update_conveyors(
    tail_conveyors: Query<Entity,With<TailConveyor>>,
    mut conveyors: Query<(&mut ConveyorLogic, &mut ItemContainer, &Transform),Without<Item>>,
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
    conveyors: &mut Query<(&mut ConveyorLogic, &mut ItemContainer, &Transform),Without<Item>>,
    items: &mut Query<&mut Transform,With<Item>>,
) -> Option<Entity>{
    let (mut conveyor,mut container,_) = conveyors.get_mut(current).unwrap();

    conveyor.timer = conveyor.timer.saturating_sub(1);
    container.set_block(conveyor.timer > 0);

    let incoming = conveyor.incoming?;

    if container.empty(){
        let (_,mut in_container,_) = conveyors.get_mut(incoming).unwrap();

        if let Some(item) = in_container.take(){
            let mut item_pos = items.get_mut(item).unwrap();

            let (mut conveyor,mut container,position) = conveyors.get_mut(current).unwrap();

            item_pos.translation = Vec3::new(position.translation.x,position.translation.y,item_pos.translation.z);
            conveyor.timer = 60;
            container.add_item(item);
            container.set_block(true);
        }
    }

    Some(incoming)
}

pub struct ConveyorPlugin();

impl Plugin for ConveyorPlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(Update,update_conveyors);
    }
}
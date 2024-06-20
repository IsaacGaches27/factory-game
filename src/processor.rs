use bevy::prelude::*;
use crate::container::ItemContainer;
use crate::item::Item;

#[derive(Component)]
pub struct Processor{
    pub inputs: Vec<Entity>,
    pub required_input_quantities: Vec<usize>,
    pub output_item_id: usize,
}

fn update_processors(
    mut processors: Query<&mut Processor>,
    mut item_inputs: Query<&mut ItemContainer>,
    mut items: Query<&mut Item>,
){
    for mut processor in &mut processors{
        let mut can_produce = true;
        processor.inputs.iter().zip(processor.required_input_quantities.iter()).for_each(|(input_entity,quantity)|{
            let input = item_inputs.get(*input_entity).unwrap();
            if input.num_items() < *quantity{
                can_produce = false;
            }
        });

        
    }
}


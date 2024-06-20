use bevy::prelude::*;

#[derive(Component)]
pub struct RestrictedItemInput{
    pub incoming: Option<Entity>,
    pub allowed_item_id: usize,
}


use bevy::prelude::*;

#[derive(Component)]
pub struct ItemStorage{
    pub item_id: usize,
    pub items: Vec<Entity>,
}
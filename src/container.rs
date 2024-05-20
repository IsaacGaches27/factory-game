use bevy::prelude::*;
use crate::item::Item;

#[derive(Component,Default)]
pub struct ItemContainer{
    item: Option<Entity>,
    blocked: bool,
}

impl ItemContainer{
    pub fn take(&mut self) -> Option<Entity>{
        if self.blocked{
            None
        }
        else{
            self.item.take()
        }
    }
    pub fn empty(&self) -> bool{
        self.item.is_none()
    }
    pub fn add_item(&mut self, item: Entity){
        self.item = Some(item);
    }
    pub fn set_block(&mut self, block: bool){
        self.blocked = block;
    }
}
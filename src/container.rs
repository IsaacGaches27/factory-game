use bevy::prelude::*;

#[derive(Component)]
pub struct ItemContainer{
    items: Vec<Entity>,
    blocked: bool,
    capacity: usize,
}

impl ItemContainer{
    pub fn new(capacity: usize)-> Self{
        Self{
            capacity,
            blocked: false,
            items: Vec::with_capacity(capacity)
        }
    }
    pub fn take(&mut self) -> Option<Entity>{
        if self.blocked{
            None
        }
        else{
            self.items.pop()
        }
    }
    pub fn blocked(&self) -> bool{
        self.blocked
    }
    pub fn empty(&self) -> bool{
        self.items.is_empty()
    }
    pub fn full(&self) -> bool{
        self.items.len() >= self.capacity
    }
    pub fn add_item(&mut self, item: Entity){
        if !self.blocked{
            self.items.push(item);
        }
    }
    pub fn set_block(&mut self, block: bool){
        self.blocked = block;
    }
    pub fn num_items(&self) -> usize{
        self.items.len()
    }
}
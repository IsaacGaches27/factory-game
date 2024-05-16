pub struct Conveyor{
    pub next: Option<usize>,
    pub items: [usize;4],
    pub num_items: usize,
}

impl Conveyor{
    pub fn add_item(&mut self, item: usize){
        self.items[self.num_items] = item;
        self.num_items+=1;
    }
    pub fn remove_first_item(&mut self) -> usize{
        let item = self.items[0];
        if self.num_items > 0 {
            for i in 1..=self.num_items {
                self.items[i - 1] = self.items[i];
            }
            self.num_items -= 1;
        }
        item
    }
}
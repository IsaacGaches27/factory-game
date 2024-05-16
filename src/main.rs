use crate::conveyor::Conveyor;
use crate::item::Item;

mod conveyor;
mod item;


fn main() {
    let mut items = Vec::new();

    let item_a = Item{ id: 0 };
    let item_b = Item{ id: 1 };
    let item_c = Item{ id: 2 };

    items.push(&item_a);
    items.push(&item_b);
    items.push(&item_c);


    let mut conveyors = Vec::new();

    let conveyor_a = Conveyor{ next: None, items: [0;4], num_items: 0 };
    let conveyor_b = Conveyor{ next: Some(0), items: [0;4], num_items: 0 };
    let mut conveyor_c = Conveyor{ next: Some(1), items: [0;4], num_items: 0 };

    conveyor_c.add_item(1);

    conveyors.push(conveyor_a);
    conveyors.push(conveyor_b);
    conveyors.push(conveyor_c);

    for _ in 0..3{
        for i in 0..conveyors.len(){
            let mut conveyor = &mut conveyors[i];
            println!("{:?}",conveyor.items);
            if conveyor.num_items > 0 {
                match conveyor.next {
                    Some(next_id) => {
                        let item = conveyor.remove_first_item();
                        let mut next = &mut conveyors[next_id];
                        next.add_item(item);
                    }
                    None => {}
                }
            }
        }
    }
}


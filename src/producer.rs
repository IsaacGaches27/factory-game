
use bevy::prelude::*;
use crate::container::ItemContainer;
use crate::item::Item;

#[derive(Component)]
pub struct Producer{
    pub timer: u16,
}

fn update_producers(
    mut commands: Commands,
    mut producers: Query<(&mut ItemContainer, &mut Producer)>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlas>>,
){
    for (mut container,mut producer) in &mut producers{
        if producer.timer > 120{
            if !container.full() {
                producer.timer = 0;

                let texture = asset_server.load("items.png");
                let layout = TextureAtlas::from_grid(texture, Vec2::new(15., 15.), 5, 2, None, Some(Vec2::new(2., 2.)));
                let texture_atlas = texture_atlas_layouts.add(layout);

                let item = commands.spawn((
                    SpriteSheetBundle {
                        texture_atlas,
                        sprite: TextureAtlasSprite {
                            index: 0,
                            ..default()
                        },
                        transform: Transform::from_xyz(0., 0., 10.),
                        ..default()
                    }
                )).insert(Item::default()).id();

                container.add_item(item);
            }
        }
        else{
            producer.timer += 1;
        }
    }
}

pub struct ProducerPlugin;

impl Plugin for ProducerPlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(Update,update_producers.after(crate::conveyor::update_conveyors));
    }
}
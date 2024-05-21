mod conveyor;
mod container;
mod item;
mod build;
mod world;

use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use crate::build::BuildPlugin;
use crate::conveyor::ConveyorPlugin;
use crate::world::WorldPlugin;

fn startup(
    mut commands: Commands,
) {
    commands.spawn(Camera2dBundle::default());
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin{
            primary_window: Some(Window {
                title: String::from(
                    "Basic Example",
                ),
                ..Default::default()
            }),
            ..default()
        }).set(ImagePlugin::default_nearest()))
        .add_plugins(BuildPlugin())
        .add_plugins(WorldPlugin())
        .add_plugins(ConveyorPlugin())
        .add_plugins(TilemapPlugin)
        .add_systems(Startup, startup)
        .run();
}
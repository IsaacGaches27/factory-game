mod conveyor;
mod container;
mod item;
mod build;
mod terrain;
mod producer;
mod camera;
mod processor;
mod item_input;

use bevy::prelude::*;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::diagnostic::LogDiagnosticsPlugin;
use bevy_ecs_tilemap::prelude::*;
use crate::build::BuildPlugin;
use crate::camera::CameraPlugin;
use crate::conveyor::ConveyorPlugin;
use crate::processor::ProcessorPlugin;
use crate::producer::ProducerPlugin;
use crate::terrain::WorldPlugin;


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
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(LogDiagnosticsPlugin::default())
        .add_plugins(BuildPlugin)
        .add_plugins(ProcessorPlugin)
        .add_plugins(WorldPlugin)
        .add_plugins(ConveyorPlugin)
        .add_plugins(TilemapPlugin)
        .add_plugins(ProducerPlugin)
        .add_plugins(CameraPlugin)
        .run();
}
use bevy::prelude::*;
fn camera_controller(keys: Res<Input<KeyCode>>,mut camera_query: Query<&mut Transform,With<Camera>>){
    let mut camera = camera_query.single_mut();
    if keys.pressed(KeyCode::W) {
        camera.translation = Vec3::new(camera.translation.x,camera.translation.y + 5.,0.);
    }
    if keys.pressed(KeyCode::A) {
        camera.translation = Vec3::new(camera.translation.x - 5.,camera.translation.y,0.);
    }
    if keys.pressed(KeyCode::S) {
        camera.translation = Vec3::new(camera.translation.x,camera.translation.y - 5.,0.);
    }
    if keys.pressed(KeyCode::D) {
        camera.translation = Vec3::new(camera.translation.x + 5.,camera.translation.y,0.);
    }
}
fn add_camera(
    mut commands: Commands,
) {
    commands.spawn(Camera2dBundle::default());
}
pub struct CameraPlugin;

impl Plugin for CameraPlugin{
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup,add_camera)
            .add_systems(Update,camera_controller);
    }
}
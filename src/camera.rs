use bevy::prelude::*;
fn camera_controller(keys: Res<Input<KeyCode>>,mut camera_query: Query<(&mut Transform,&mut OrthographicProjection)>){
    let (mut transform,mut camera) = camera_query.single_mut();
    if keys.pressed(KeyCode::W) {
        transform.translation = Vec3::new(transform.translation.x,transform.translation.y + 10.,0.);
    }
    if keys.pressed(KeyCode::A) {
        transform.translation = Vec3::new(transform.translation.x - 10.,transform.translation.y,0.);
    }
    if keys.pressed(KeyCode::S) {
        transform.translation = Vec3::new(transform.translation.x,transform.translation.y - 10.,0.);
    }
    if keys.pressed(KeyCode::D) {
        transform.translation = Vec3::new(transform.translation.x + 10.,transform.translation.y,0.);
    }

    if keys.pressed(KeyCode::Equals) {
        camera.scale -= 0.05;
    }
    if keys.pressed(KeyCode::Minus) {
        camera.scale += 0.05;
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
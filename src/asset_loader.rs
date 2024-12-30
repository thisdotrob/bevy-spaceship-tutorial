use bevy::prelude::*;

#[derive(Resource, Debug, Default)]
pub struct SceneAssets {
    pub asteroid: Handle<Scene>,
    pub spaceship: Handle<Scene>,
    pub astronaut_frog: Handle<Scene>,
    pub astronaut_mouse: Handle<Scene>,
    pub astronaut_bird: Handle<Scene>,
}

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SceneAssets>()
            .add_systems(Startup, load_assets);
    }
}

fn load_assets(mut scene_assets: ResMut<SceneAssets>, asset_server: Res<AssetServer>) {
    *scene_assets = SceneAssets {
        asteroid: asset_server.load("Enemy Flying.glb#Scene0"),
        spaceship: asset_server.load("Spaceship.glb#Scene0"),
        astronaut_frog: asset_server.load("Astronaut-0D54W8yfrA.glb#Scene0"),
        astronaut_mouse: asset_server.load("Astronaut-OgeSH89Nmx.glb#Scene0"),
        astronaut_bird: asset_server.load("Astronaut.glb#Scene0"),
    }
}

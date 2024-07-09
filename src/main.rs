use bevy::{gltf::Gltf, log, prelude::*};
use bevy_asset_loader::prelude::*;
use bevy_gltf_components::ComponentsFromGltfPlugin;
use bevy_registry_export::*;
fn main() {
    App::new()
        .register_type::<Block>()
        .add_plugins((
            DefaultPlugins,
            ExportRegistryPlugin {
                save_path: "model/registry.json".into(),
                ..Default::default()
            },
            ComponentsFromGltfPlugin::default(),
        ))
        .add_systems(Startup, startup)
        .run();
}
#[derive(Component, Reflect)]
#[reflect(Component)]
struct Block;

fn startup(mut commands: Commands, assets: Res<AssetServer>, gltf_assets: Res<Assets<Gltf>>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(SceneBundle {
        scene: assets.load("model/untitled.glb"),
        ..Default::default()
    });

    // commands.spawn(SceneBundle {
    //     scene: assets
    //         .load(AssetLabel::Scene(0).from_asset("models/FlightHelmet/FlightHelmet.gltf")),
    //     ..default()
    // });
    let gltf: Handle<Gltf> = assets.load("model/untitled.glb");
    log::info!("{:?}", gltf)
}

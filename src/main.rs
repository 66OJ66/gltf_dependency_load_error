mod assets;

use self::assets::*;
use bevy::gltf::Gltf;
use bevy::prelude::*;
use bevy::utils::HashMap;

#[derive(Resource)]
pub struct ItemMap(pub HashMap<u64, Item>);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(AssetPlugin::processed_dev()))
        .init_asset::<Item>()
        .register_asset_loader(SerialisedItemAssetLoader)
        .add_systems(Update, load_asset)
        .run();
}

#[allow(clippy::too_many_arguments)]
fn load_asset(
    mut commands: Commands,
    // Resources
    asset_server: Res<AssetServer>,
    // Assets
    item_assets: Res<Assets<Item>>,
    gltf_assets: Res<Assets<Gltf>>,
    // Local
    mut item_handle: Local<Handle<Item>>,
    mut gltf_handle: Local<Handle<Gltf>>,
    mut item_loaded: Local<bool>,
    mut gltf_loaded: Local<bool>,
) {
    *item_handle = asset_server.load("items/cauldron.item.ron");
    *gltf_handle = asset_server.load("gltf/cauldron.gltf");

    if !*item_loaded {
        if let Some(loaded_item) = item_assets.get(&*item_handle) {
            info!("{} asset loaded", loaded_item.name);
            commands.insert_resource(ItemMap(HashMap::from([(
                loaded_item.id,
                loaded_item.clone(),
            )])));
            *item_loaded = true;
        }
    }

    if !*gltf_loaded && gltf_assets.get(&*gltf_handle).is_some() {
        info!("Gltf loaded");
        *gltf_loaded = true;
    }
}

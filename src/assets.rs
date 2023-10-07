use bevy::asset::io::Reader;
use bevy::asset::*;
use bevy::gltf::Gltf;
use bevy::prelude::*;
use bevy::reflect::TypePath;
use serde::{Deserialize, Serialize};
use thiserror::Error;

const ITEM_FILE_EXTENSION: &str = "item.ron";

//****************************************************************************
// ASSETS
//****************************************************************************

#[derive(Asset, Serialize, Deserialize, TypePath)]
pub struct SerialisedItem {
    id: u64,
    name: String,
    weight: u64,
    model_path: String,
}

#[derive(Asset, Clone, TypePath)]
pub struct Item {
    pub id: u64,
    pub name: String,
    pub weight: u64,
    pub model: Handle<Gltf>,
}

//****************************************************************************
// ERRORS
//****************************************************************************

#[derive(Error, Debug)]
pub enum LoaderError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    RonSpannedError(#[from] ron::error::SpannedError),
    #[error(transparent)]
    LoadDirectError(#[from] LoadDirectError),
}

//****************************************************************************
// ASSET LOADERS
//****************************************************************************

#[derive(Default)]
pub struct SerialisedItemAssetLoader;

impl AssetLoader for SerialisedItemAssetLoader {
    type Asset = Item;
    type Settings = ();
    type Error = LoaderError;

    fn load<'a>(
        &'a self,
        reader: &'a mut Reader,
        _settings: &'a Self::Settings,
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<Item, Self::Error>> {
        Box::pin(async move {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;
            let ron: SerialisedItem = ron::de::from_bytes(&bytes)?;

            Ok(Item {
                id: ron.id,
                name: ron.name,
                weight: ron.weight,
                model: load_context.load(ron.model_path),
            })
        })
    }

    fn extensions(&self) -> &[&str] {
        &[ITEM_FILE_EXTENSION]
    }
}

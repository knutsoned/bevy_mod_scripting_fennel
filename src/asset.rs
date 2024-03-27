use bevy::asset::io::Reader;
use bevy::asset::{ AssetLoader, AsyncReadExt, BoxedFuture, LoadContext };
use bevy::prelude::*;

use bevy_mod_scripting::core::asset::CodeAsset;

use anyhow::Error;

#[derive(Asset, TypePath, Debug)]
/// A Fennel code file in bytes
pub struct FennelFile {
    pub bytes: Vec<u8>,
}

impl CodeAsset for FennelFile {
    fn bytes(&self) -> &[u8] {
        self.bytes.as_slice()
    }
}

#[derive(Default)]
pub struct FennelLoader;

impl AssetLoader for FennelLoader {
    type Asset = FennelFile;
    type Settings = ();
    type Error = Error;

    fn load<'a>(
        &'a self,
        reader: &'a mut Reader,
        _settings: &'a Self::Settings,
        _load_context: &'a mut LoadContext
    ) -> BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;
            //info!("Loaded script: {:?}", bytes);
            Ok(FennelFile {
                bytes,
            })
        })
    }

    fn extensions(&self) -> &[&str] {
        &["fnl"]
    }
}

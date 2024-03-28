use bevy::asset::io::Reader;
use bevy::asset::{ AssetLoader, AsyncReadExt, BoxedFuture, LoadContext };
use bevy::prelude::*;

use bevy_mod_scripting_lua::assets::LuaFile;

use anyhow::Error;

#[derive(Default)]
pub struct FennelLoader;

impl AssetLoader for FennelLoader {
    type Asset = LuaFile;
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
            info!("Loaded Fennel script: {:?}", bytes);

            // compile the file to hand off to ScriptHost
            let code = String::from_utf8(bytes)?;
            let src = format!("return require(\"scripts/fennel\").eval([[ {} ]])", code);

            Ok(LuaFile {
                bytes: src.as_bytes().into(),
            })
        })
    }

    fn extensions(&self) -> &[&str] {
        &["fnl"]
    }
}

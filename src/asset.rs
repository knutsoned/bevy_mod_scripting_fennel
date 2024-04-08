use std::sync::{ Mutex, OnceLock };

use bevy::asset::io::Reader;
use bevy::asset::{ AssetLoader, AsyncReadExt, BoxedFuture, LoadContext };
use bevy::log::info;

use bevy_mod_scripting_lua::assets::LuaFile;

use mlua::Lua;

use crate::fennel::FENNEL;

// add a static Lua context as in https://users.rust-lang.org/t/static-lua-reference/60941
// with the Fennel compiler preloaded
fn compiler() -> &'static Mutex<Lua> {
    static COMPILER: OnceLock<Mutex<Lua>> = OnceLock::new();
    COMPILER.get_or_init(|| {
        let temp_fn_name = "__FENNEL_COMPILER";
        let short_name = "fennel".to_owned();
        let full_name = short_name.clone() + ".lua";
        let lua = Lua::new();

        // adapted from https://github.com/LaserWitch/lw_bevy_lua_demo/blob/main/src/lua/host.rs
        let runstr = format!(
            "
--Out of significant paranoia, ensure the required table structure exists already
--  and then make local references to it for brevity
if not _G.package then
    _G.package = {{}}
end
local packs = _G.package
if not packs.preload then
    packs.preload = {{}}
end
if not packs.loaded then
    packs.loaded = {{}}
end

local tf = {temp_fn_name}
local function loader()
    print(\"lua running loader for bevy asset: {full_name}\")
    return tf(\"{short_name}\")
end

print(\"lua creating loader for bevy asset: {full_name}\")
packs.preload[\"{short_name}\"] = loader
print(\"...created loader for bevy asset: {full_name}\")
            "
        );

        // need to use a closure to be able to return a lua context at the end
        let loader = || {
            // load the source for the Fennel compiler
            let chunk = lua.load(FENNEL);
            let wrapped = chunk.into_function().expect("error loading Fennel compiler source");

            // try to store it in the Lua globals table
            lua.globals()
                .set(temp_fn_name, wrapped)
                .expect("bad globals table setting Fennel compiler package fn");

            // now run the custom loader code
            let new_chunk = lua.load(&runstr);
            new_chunk.exec().expect("error executing custom loader for Fennel compiler");
        };
        loader();

        // return a Lua context with Fennel already loaded
        lua.into()
    })
}

#[derive(Default)]
pub struct FennelLoader;

impl AssetLoader for FennelLoader {
    type Asset = LuaFile;
    type Settings = ();
    type Error = anyhow::Error;

    fn load<'a>(
        &'a self,
        reader: &'a mut Reader,
        _settings: &'a Self::Settings,
        load_context: &'a mut LoadContext
    ) -> BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;
            let code = String::from_utf8(bytes)?;
            let lua = compiler().lock().expect("bad lua state");
            let path = load_context.path().to_str().expect("bad file path");
            // errors will be sent up the stack when the Lua plugin processes the code
            // original way:
            // let src = format!("return require(\"fennel\").eval([[ {} ]])", code);
            // info!("Loaded Fennel script: {:?}", src);

            // new way:
            // use the static Lua context with Fennel already loaded:
            let script_key = format!("__FENNEL_SRC_{}", path);

            // TODO: error handling so we don't just go into poison mode when there's a compile error
            lua.globals().set(script_key.clone(), code)?;

            // - send the [u8] with the source code in the globals table to the fennel compiler
            let cmd = format!("require(\"fennel\").compileString(_G[\"{script_key}\"])");

            // - transpile string to string
            let lua_src = lua.load(cmd).eval::<String>().expect("error compiling Fennel to Lua");
            info!("lua src: {}", lua_src);

            // - hand off to the "regular" Lua mod system
            Ok(LuaFile {
                bytes: lua_src.as_bytes().into(),
            })
        })
    }

    fn extensions(&self) -> &[&str] {
        &["fnl"]
    }
}

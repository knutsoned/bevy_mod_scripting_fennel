use bevy::prelude::*;
use bevy_mod_scripting::{ core::event::ScriptLoaded, prelude::* };
use bevy_mod_scripting_fennel::prelude::*;

// set up Lua
fn main() -> std::io::Result<()> {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .add_plugins(ScriptingPlugin)
        .add_script_host::<LuaScriptHost<()>>(PostUpdate)
        .add_script_handler::<LuaScriptHost<()>, 0, 0>(PostUpdate)
        .init_asset_loader::<FennelLoader>()
        .add_systems(Startup, load_our_script)
        .add_systems(PreUpdate, check_loaded);
    app.run();

    Ok(())
}

// script loading
fn load_our_script(server: Res<AssetServer>, mut commands: Commands) {
    info!("loading script");
    let path = "scripts/basic_test.fnl"; // just use "scripts/" to load entire dir
    let handle = server.load::<LuaFile>(path);
    commands.spawn(()).insert(ScriptCollection::<LuaFile> {
        scripts: vec![Script::<LuaFile>::new(path.to_owned(), handle)],
    });
}

// check for ScriptLoaded event
fn check_loaded(
    mut reader: EventReader<ScriptLoaded>,
    mut writer: PriorityEventWriter<LuaEvent<()>>
) {
    for load_event in reader.read() {
        info!("loaded script: {:?}", load_event);
        writer.send(
            LuaEvent {
                hook_name: "hello".to_owned(),
                args: (),
                recipients: Recipients::ScriptID(load_event.sid),
            },
            0
        );
    }
}

use bevy::{ app::AppExit, prelude::* };
use bevy_mod_scripting::prelude::*;
use bevy_mod_scripting_fennel::asset::FennelLoader;

// set up Lua
fn main() -> std::io::Result<()> {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .add_plugins(ScriptingPlugin)
        .add_script_host::<LuaScriptHost<()>>(PostUpdate)
        .init_asset_loader::<FennelLoader>() // attach Fennel asset loader
        .add_systems(Startup, load_our_script)
        .add_systems(Update, run_our_script);
    app.run();

    Ok(())
}

// load script
fn load_our_script(server: Res<AssetServer>, mut commands: Commands) {
    let path = "scripts/basic_test.fnl";
    let handle = server.load::<LuaFile>(path);

    commands.spawn(()).insert(ScriptCollection::<LuaFile> {
        scripts: vec![Script::<LuaFile>::new(path.to_string(), handle)],
    });
}

fn run_our_script(world: &mut World) {
    // make a unit entity to be the script target
    let entity = world.spawn(()).id();

    // run script
    world.resource_scope(|world, mut host: Mut<LuaScriptHost<()>>| {
        host.run_one_shot(
            r#"
                hello()
            "#.as_bytes(),
            "script.lua",
            entity,
            world,
            LuaEvent {
                hook_name: "once".to_owned(),
                args: (),
                recipients: Recipients::All,
            }
        ).expect("Something went wrong in the script!");

        world.send_event(AppExit);
    });
}

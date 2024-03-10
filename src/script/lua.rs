use bevy::prelude::*;
use bevy_mod_scripting::prelude::*;
use bevy_script_api::lua::RegisterForeignLuaType;

//We need to be state aware so we don't try to start setting up scripts before all are loaded.
use crate::preload::GameState;

pub struct LuaPlugin;

impl Plugin for LuaPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ScriptingPlugin)
            .add_script_host::<FennelScriptHost<()>>(PostUpdate)
            .add_api_provider::<FennelScriptHost<()>>(Box::new(OurAPI))
            .add_api_provider::<FennelScriptHost<()>>(Box::new(LuaBevyAPIProvider))
            .add_script_handler::<FennelScriptHost<()>, 0, 0>(PostUpdate)
            .add_systems(OnEnter(GameState::Playing), load_startup_scripts)
            .add_systems(Update, do_update)
            .register_foreign_lua_type::<Entity>();
        //            .update_documentation::<bevy_mod_scripting_lua::LuaScriptHost<()>>();
    }
}
fn load_startup_scripts(
    server: Res<AssetServer>,
    script_assets: ResMut<Assets<LuaFennel>>,
    mut commands: Commands
) {
    info!("load_startup_scripts");
    let mut scripts = Vec::new();
    for (id, _) in script_assets.iter() {
        let mut h = server.get_handle(id);
        h.make_strong(&script_assets);
        let path = server.get_handle_path(id).expect("msg");
        let n = path.path().to_str().unwrap().to_string();
        let s = Script::<LuaFennel>::new(n, h);

        info!("{:#?}", s);
        scripts.push(s);
    }
    commands.spawn(ScriptCollection::<LuaFennel> { scripts });
}
//I wanted to call script files directly but screw it, I'm using the event system that's in there
fn do_update(mut w: PriorityEventWriter<LuaEvent<()>>) {
    w.send(
        LuaEvent {
            hook_name: "on_level".to_string(),
            args: (),
            recipients: Recipients::All,
        },
        0
    );
}

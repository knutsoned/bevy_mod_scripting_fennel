use bevy::prelude::*;

mod preload;
use crate::preload::*;

mod script;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        //.add_systems(Startup, setup)
        // LoadScreenPlugin loads defaults: .add_plugins(DefaultPlugins)
        .add_plugins(LoadScreenPlugin)
        .run()
}

//fn setup(mut commands: Commands) {}

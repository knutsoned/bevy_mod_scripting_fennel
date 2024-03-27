use bevy::prelude::*;

pub mod asset;

pub mod prelude {
    pub use crate::asset::{ FennelFile, FennelLoader };
}

pub struct FennelPlugin;
impl Plugin for FennelPlugin {
    fn build(&self, app: &mut App) {
        /*
        app
            .add_state::<GameState>()
            .add_systems(OnEnter(GameState::Loading), start_loading)
            .add_systems(Update, while_loading.run_if(in_state(GameState::Loading)));
        */
    }
}

use super::main;
use super::options;
use bevy::prelude::*;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        main::setup_menu(app);
        options::setup_menu(app);
    }
}

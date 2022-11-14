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

pub fn remove_menu_screen<T: Component>(mut commands: Commands, q_menu: Query<Entity, &T>) {
    if let Some(menu_entity) = q_menu.iter().next() {
        commands.entity(menu_entity).despawn_recursive();
    }
}

use super::super::game::cell;
use super::super::game::gravity;
use super::super::game::grid;
use super::super::game::input;
use super::camera;
use super::config;
use super::icon;
use bevy::prelude::*;
use bevy::time::FixedTimestep;

pub fn start() {
    App::new()
        .insert_resource(WindowDescriptor {
            width: config::WINDOW_WIDTH,
            height: config::WINDOW_HEIGHT,
            title: config::GAME_NAME.to_string(),
            resizable: false,
            present_mode: bevy::window::PresentMode::AutoVsync,
            ..default()
        })
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins)
        .add_startup_system(icon::setup)
        .add_startup_system(camera::setup)
        .add_startup_system(grid::setup)
        .add_system(
            cell::clear
                .before(grid::draw)
                .with_run_criteria(FixedTimestep::steps_per_second(15.)),
        )
        .add_system(
            grid::refresh
                .before(grid::draw)
                .with_run_criteria(FixedTimestep::steps_per_second(15.)),
        )
        .add_system(grid::draw.with_run_criteria(FixedTimestep::steps_per_second(15.)))
        .add_system(gravity::apply.with_run_criteria(FixedTimestep::steps_per_second(1.)))
        .add_system(input::handle)
        .run();
}

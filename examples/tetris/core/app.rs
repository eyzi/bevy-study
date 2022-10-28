use super::camera;
use super::config;
use super::grid;
use super::tetris;
use bevy::prelude::*;
use bevy::time::*;
use bevy::window::WindowId;
use bevy::winit::WinitWindows;
use winit::window::Icon;

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
        .add_startup_system(set_window_icon)
        .add_startup_system(camera::setup)
        .add_startup_system(grid::setup)
        .add_startup_system(tetris::setup)
        .add_system(grid::draw.with_run_criteria(FixedTimestep::steps_per_second(30.)))
        .add_system(grid::apply_gravity.with_run_criteria(FixedTimestep::steps_per_second(1.)))
        .run();
}

fn set_window_icon(windows: NonSend<WinitWindows>) {
    let primary = windows.get_window(WindowId::primary()).unwrap();

    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open("assets/common/eyzi-logo.png")
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };

    let icon = Icon::from_rgba(icon_rgba, icon_width, icon_height).unwrap();
    primary.set_window_icon(Some(icon));
}

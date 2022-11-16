#![windows_subsystem = "windows"]

mod core;
mod fader;
mod menu;
mod scene;
mod splashscreen;

fn main() {
    core::app::start();
}

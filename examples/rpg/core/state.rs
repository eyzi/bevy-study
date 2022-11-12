#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum GameState {
    Startup,
    Splashscreen,
    MainMenu,
    OptionsMenu,
    Playing,
    Paused,
}

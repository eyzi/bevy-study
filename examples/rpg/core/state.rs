#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum GameState {
    Startup,
    MainMenu,
    Playing,
    Paused,
}

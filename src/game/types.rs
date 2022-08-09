// Basically a .h file
// lmfao
// Contains our generic types so they dont take up space

pub struct StatusNoGame {}
pub struct StatusInGame {}
pub struct StatusGameHost {}

pub enum GameStatus {
    StatusNoGame,
    StatusInGame,
    StatusGameHost
}
// Struct that has our game data
// Only real struct lmao
pub struct GameData {
    // the ping average
    pub average_ping: f32,

    // the current ping
    pub curr_ping: u16,

    // a field for calculations, contains all pings 
    // way more mem efficient that a vector
    pub _all_ping: u32,

    // the number of measurments
    pub measurments: u32,

    // the status of the game in custom structs
    pub status: GameStatus
}

impl GameData {
    pub fn new() -> GameData {
        GameData {average_ping: 0.0, curr_ping: 0, _all_ping: 0, measurments: 0, status: GameStatus::StatusNoGame{}}
    }
}
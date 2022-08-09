// Basically a .h file
// lmfao
// Contains our generic types so they dont take up space

pub struct StatusNoGame {}
pub struct StatusInGame {}
pub struct StatusGameHost {}


// Struct that has our game data
// Only real struct lmao
pub struct GameData {
    pub average_ping: u32,
    pub curr_ping: u32,
    pub measurments: u32,
    pub status: Option<StatusNoGame, StatusInGame, StatusGameHost>
}

pub impl GameData {
    pub fn new() -> GameData {
        GameData {0, 0, 0, StatusNoGame}
    }
}
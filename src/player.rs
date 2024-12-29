use crate:: util::*;

pub struct Player {
    pub pos: Pos,
}


impl Player
{
    pub fn new() -> Player { 
        return Player {
            pos: Pos::new(32.0, 32.0, 0),
        }
    }
}
pub struct Pos {
    pub x: f32,
    pub z: f32,
    pub y: i32,
}

impl Pos 
{
    pub fn new(x: f32, z: f32, y:i32) -> Pos
    {
        Pos
        {
            x: x,
            y: y,
            z: z,
        }
    }

    pub fn to_tpos(&self) -> TPos
    {
        return TPos::new(self.x as i32, self.z as i32, self.y as i32)
    }
}

pub struct TPos {
    pub x: i32,
    pub z: i32,
    pub y: i32,
}

impl TPos 
{
    pub fn new(x: i32, z: i32, y:i32) -> TPos
    {
        TPos
        {
            x: x,
            y: y,
            z: z,
        }
    }
}
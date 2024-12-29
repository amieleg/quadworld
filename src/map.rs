use macroquad::prelude::*;
//use rand::Rng;
use crate::player::*;
use crate::util::*;

pub const SCREEN_WIDTH: i32 = 32;

pub const SIZE_X: usize = 64;
pub const SIZE_Z: usize = 64;
pub const SIZE_Y: usize = 3;

pub struct Map {
    pub m: Vec<Vec<Vec<f32>>>
}

impl Map
{
    pub fn new() -> Map
    {
        return Map
        {
            m: vec![vec![vec![0.0 as f32;SIZE_X];SIZE_Z];SIZE_Y]
        }
    }

    pub fn num_to_coords(i: i32) -> TPos
    {
        let y = i / (SIZE_X as i32 * SIZE_Z as i32);
        let z = (i / (SIZE_X as i32)) % SIZE_Z as i32;
        let x = i % SIZE_X as i32;
        return TPos::new(x,z,y);
    }

    pub fn gen_fib(&mut self)
    {
        let mut first = 1;
        let mut second = 1;

        while second < (SIZE_X as i32 * SIZE_Z as i32 * SIZE_Y as i32)
        {
            let coord = Self::num_to_coords(second);
            self.m[coord.y as usize][coord.z as usize][coord.x as usize] = 1.0/(coord.y as f32 + 1.0);
            let b = first + second;
            first = second;
            second = b;
        }
    }

    
    pub fn gen_fizzbuzz(&mut self)
    {
        let mut i = 0;
        while i < (SIZE_X as i32 * SIZE_Z as i32 * SIZE_Y as i32)
        {
            if i % 7 == 0 && i % 5 == 0
            {

            }
            else if i % 5 == 0 || i % 7 == 0
            {
                let coord = Self::num_to_coords(i);
                self.m[coord.y as usize][coord.z as usize][coord.x as usize] = 1.0/(coord.y as f32 + 1.0);
            }
            i = i + 1;
        }
    }

    pub fn gen_stair(&mut self)
    {
        let mut i = 31;
        for y in 0..SIZE_Y
        {
            for z in 32..SIZE_X
            {
                self.m[y][z][i] = 1.0;
            }
            i = i + 1;
        }

        for z in 0..SIZE_Z
        {
            for x in 0..SIZE_X
            {
                self.m[0][x][z] = 0.5;
            }
        }
    }

    pub fn gen_block_in_air(&mut self)
    {
        self.m[1][40][40] = 1.0;
    }

    pub fn get(&self, tpos: TPos) -> f32
    {
        return self.m[tpos.y as usize][tpos.z as usize][tpos.x as usize]
    }

    pub fn highest_empty_above(&self, tpos: TPos) -> TPos
    {
        if tpos.z < 0 || tpos.z >= SIZE_Z as i32 || tpos.x < 0 || tpos.x >= SIZE_Z as i32
        {
            return TPos::new(0, 0, SIZE_Y as i32-1);
        }

        if self.m[tpos.y as usize][tpos.z as usize][tpos.x as usize] != 0.0
        {
            return tpos;
        }

        for i in tpos.y..SIZE_Y as i32
        {
            if self.m[i as usize][tpos.z as usize][tpos.x as usize] != 0.0
            {
                return TPos::new(tpos.x, tpos.z, i-1);
            }
        }
        return TPos::new(tpos.x, tpos.z, SIZE_Y as i32-1);
    }

    pub fn first_full_below(&self, tpos: TPos) -> TPos
    {
        let mut i = tpos.y;
        while i >= 0
        {
            if self.m[i as usize][tpos.z as usize][tpos.x as usize] != 0.0
            {
                return TPos::new(tpos.x, tpos.z, i);
            }
            i = i - 1;
        }
        return TPos::new(tpos.x, tpos.z, 0);
    }

    pub fn get_tile(&self, x: i32, z: i32, pl: &Player) -> (f32, i32) // tile, height
    {
        let campos = self.highest_empty_above(pl.pos.to_tpos());
        draw_text(&format!("{}\n", campos.y), 10.0, 30.0, 20.0, WHITE);
        let drawntile = self.first_full_below(TPos::new(x,z,campos.y));
        let offset = drawntile.y - pl.pos.y;
        return (self.get(drawntile), offset);
    }
}
use macroquad::prelude::*;

mod map;
mod player;
mod util;

use map::*;
use player::*;
use util::*;

pub const SCREEN_WIDTH: i32 = 32;
pub const SPEED: f32 = 10.0/60.0;

#[macroquad::main("MyGame")]
async fn main() {
    let mut map = Map::new();
    map.gen_fib();
    let mut player = Player::new();

    println!("{}", 0.0 == 0.0);

    loop {
        /*clear_background(RED);

        draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);

        draw_text("Hello, Macroquad!", 20.0, 20.0, 30.0, DARKGRAY);*/

        clear_background(BLACK);

        draw_map(&map, &player);

        draw_text(&format!("XZY: {},{},{}",player.pos.x,player.pos.z,player.pos.y), 10.0, 20.0, 20.0, WHITE);
        draw_circle(screen_width()/2.0, screen_height()/2.0,5.0, RED);
        

        if is_key_down(KeyCode::Right) {
            player.pos.x += SPEED;
        }

        if is_key_down(KeyCode::Left) {
            player.pos.x -= SPEED;
        }

        if is_key_down(KeyCode::Up) {
            player.pos.z -= SPEED;
        }

        if is_key_down(KeyCode::Down) {
            player.pos.z += SPEED;
        }

        if is_key_pressed(KeyCode::A) {
            if player.pos.y < SIZE_Y as i32 -1
            {
                player.pos.y += 1;
            }
        }

        if is_key_pressed(KeyCode::Z) {
            if player.pos.y >= 1
            {
                player.pos.y -= 1;
            }
        }

        if is_key_pressed(KeyCode::Q) {
            break;
        }
        
        next_frame().await
    }
}

fn draw_map(m: &Map, p: &Player)
{
    let tile_size = screen_width() / SCREEN_WIDTH as f32;

    let pos_x_i = p.pos.x as i32;
    let pos_x_diff = (p.pos.x - pos_x_i as f32) * tile_size;
    let pos_z_i = p.pos.z as i32;
    let pos_z_diff = (p.pos.z - pos_z_i as f32) * tile_size;
    let half_screen = SCREEN_WIDTH/2;

    let mut tile_x = -tile_size;
    let mut tile_z = -tile_size + (screen_height()/2.0) - ((half_screen as f32) * tile_size);
    
    for z in pos_z_i-half_screen-1..pos_z_i+half_screen+1 as i32
    {
        for x in pos_x_i-half_screen-1..pos_x_i+half_screen+1 as i32
        {
            if x >= 0 && x < map::SIZE_X as i32 && z >= 0 && z < map::SIZE_Z as i32
            {
                let (tile, offset) = m.get_tile(x,z, p);
                let col = tile;
                draw_rectangle(tile_x - pos_x_diff, tile_z - pos_z_diff /*- (offset as f32 * (tile_size / 2.0))*/, tile_size, tile_size, Color{r: col, g: col, b: col, a: 1.0/*/(offset as f32 + 1.0)*/});
            }
            tile_x += tile_size;
        }
        tile_z += tile_size;
        tile_x = -tile_size;
    }
}
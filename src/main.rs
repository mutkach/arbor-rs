pub mod player;
pub mod flail;

use flail::Flail;
use macroquad::prelude::{SKYBLUE, clear_background, draw_text, draw_circle, draw_line, next_frame, DARKGRAY, BLUE };
use macroquad::input::{is_key_down, KeyCode};
use macroquad::time::{get_frame_time};
use player::Player;
use glam::{Vec2};

#[macroquad::main("BasicShapes")]
async fn main() {
    let start_position : Vec2 = Vec2::new(0.0,0.0);
    let mut player: Player  = Player::new(start_position);
    let mut flail: Flail = Flail::new(start_position);
    //let mut timer : Timer = Timer::new();

    loop {
        clear_background(SKYBLUE);
        //timer.update();
        let delta = get_frame_time() as f64;

        flail.update_position(player.position, delta);

        if is_key_down(KeyCode::A) {
            player.move_player( Vec2::new(0.0, -1.0), delta);
            println!("{}", delta);
        }
        else if is_key_down(KeyCode::D) {
            player.move_player( Vec2::new(0.0,  1.0), delta);
            println!("{}", delta);
        }
        else if is_key_down(KeyCode::W) {
            player.move_player( Vec2::new(1.0,  0.0), delta);
            println!("{}", delta);
        }
        else if is_key_down(KeyCode::S) {
            player.move_player(Vec2::new(-1.0,  0.0), delta);
            println!("{}", delta);
        }

        //draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        //draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        //draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);
        draw_line(player.position.x, player.position.y, flail.position.x, flail.position.y, 2.0, BLUE);
        draw_circle(player.position.x, player.position.y, 15.0, DARKGRAY);
        draw_circle(flail.position.x, flail.position.y, 5.0, DARKGRAY);
        draw_text("IT WORKS!", 20.0, 20.0, 30.0, DARKGRAY);
        next_frame().await;
    }
}

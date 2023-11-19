pub mod flail;
pub mod physics;
pub mod player;

use flail::Flail;

use macroquad::input::{is_key_down, KeyCode};
use macroquad::miniquad::window::order_quit;
use macroquad::prelude::{
    clear_background, draw_circle, draw_line, draw_text, next_frame, BLUE, DARKGRAY, SKYBLUE, WHITE,
};
use macroquad::time::get_frame_time;
use nalgebra::vector;
use physics::Physics;
use player::Player;
use rapier2d::prelude::{Real, Vector};

#[macroquad::main("BasicShapes")]
async fn main() {
    let width = macroquad::window::screen_width();
    let height = macroquad::window::screen_height();
    let start_position: Vector<Real> = vector![width / 2.0, height / 2.0];
    let mut physics = Physics::new();
    let mut player: Player = Player::new(start_position);
    let mut flail: Flail = Flail::new(
        start_position,
        &mut physics.rigid_body_set,
        &mut physics.collider_set,
    );

    loop {
        physics.step();
        clear_background(SKYBLUE);
        let delta = get_frame_time() as f64;

        flail.update_position(player.position, &mut physics.rigid_body_set);

        if is_key_down(KeyCode::W) {
            player.move_player(vector![0.0, -1.0], delta);
            println!("{}", delta);
        } else if is_key_down(KeyCode::S) {
            player.move_player(vector![0.0, 1.0], delta);
            println!("{}", delta);
        } else if is_key_down(KeyCode::D) {
            player.move_player(vector![1.0, 0.0], delta);
            println!("{}", delta);
        } else if is_key_down(KeyCode::A) {
            player.move_player(vector![-1.0, 0.0], delta);
            println!("{}", delta);
        } else if is_key_down(KeyCode::Escape) {
            order_quit();
        }

        //draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        //draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        //draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);
        draw_line(
            player.position.x,
            player.position.y,
            flail.position.x,
            flail.position.y,
            2.0,
            BLUE,
        );
        draw_circle(player.position.x, player.position.y, 15.0, WHITE);
        draw_circle(flail.position.x, flail.position.y, 15.0, DARKGRAY);
        draw_text("IT WORKS!", 20.0, 20.0, 30.0, DARKGRAY);
        next_frame().await;
    }
}

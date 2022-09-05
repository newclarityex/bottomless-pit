mod ui_elements;
mod game;

use raylib::prelude::*;
use game::Game;

fn main() {
    let screen_width: u16 = 800;
    let screen_height: u16 = 450;


    let(mut rl, thread) = raylib::init()
        .transparent()
        .size(screen_width as i32, screen_height as i32)
        .title("cheesed to meet u")
        .build();
    
    rl.set_target_fps(30);
    rl.set_exit_key(None);

    let mut game = Game::new();

    while !rl.window_should_close() {
        game.update(&rl);

        let mut d_handle = rl.begin_drawing(&thread);
        game.draw(d_handle);

        if game.close_check() {
            break
        }
    }
}

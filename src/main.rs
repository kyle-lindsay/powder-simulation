use macroquad::prelude::*;

mod game;
mod grid;
mod input;
mod render;

fn window_conf() -> Conf {
    Conf {
        window_title: "powder-sim".to_owned(),
        window_width: 800,
        window_height: 800,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game = game::Game::new();

    loop {
        game.update();
        game.draw();
        next_frame().await;
    }
}
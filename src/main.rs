use macroquad::prelude::*;

const WIDTH : u8 = 16;
const HEIGHT : u8 = 16;

#[macroquad::main("powder-sim")]
async fn main() {
    let window_width = screen_width();
    let window_height = screen_height();

    loop {
        clear_background(BLUE);
        // 1) update your state
        // 2) draw the frame
        next_frame().await;
    }
}
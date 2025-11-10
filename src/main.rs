use macroquad::prelude::*;

mod assets;
mod utils;

#[macroquad::main("space friend")]
async fn main() {
    loop {
        next_frame().await
    }
}

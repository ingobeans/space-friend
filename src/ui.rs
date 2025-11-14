use crate::assets::Assets;
use crate::utils::*;
use macroquad::miniquad::window::screen_size;
use macroquad::prelude::*;
pub fn draw_tooltip(assets: &Assets) {
    let (actual_screen_width, actual_screen_height) = screen_size();
    let scale_factor = (actual_screen_width / SCREEN_WIDTH)
        .min(actual_screen_height / SCREEN_HEIGHT)
        .floor()
        .max(1.0);
    let x = (actual_screen_width - assets.tooltip.width() * scale_factor) / 2.0;
    let y = actual_screen_height - assets.tooltip.height() * scale_factor - 4.0 * scale_factor;
    draw_texture_ex(
        &assets.tooltip,
        x,
        y,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(
                assets.tooltip.width() * scale_factor,
                assets.tooltip.height() * scale_factor,
            )),
            ..Default::default()
        },
    );
}

use crate::assets::Assets;
use crate::player::Player;
use crate::utils::*;
use macroquad::miniquad::window::screen_size;
use macroquad::prelude::*;

pub const PLAYER_HEALTH_COLOR: Color = Color::from_hex(0x87d1ef);

pub fn draw_ui(assets: &Assets, show_tooltip: bool, player: &Player) {
    let (actual_screen_width, actual_screen_height) = screen_size();
    let scale_factor = (actual_screen_width / SCREEN_WIDTH)
        .min(actual_screen_height / SCREEN_HEIGHT)
        .floor()
        .max(1.0);

    let x = 10.0 * scale_factor;
    let y = 10.0 * scale_factor;
    draw_rectangle(
        x + 8.0 * scale_factor,
        y + 2.0 * scale_factor,
        170.0 * scale_factor * player.health / 100.0,
        20.0 * scale_factor,
        BLACK,
    );
    draw_rectangle(
        x + 8.0 * scale_factor,
        y + 2.0 * scale_factor,
        170.0 * scale_factor * player.health / 100.0,
        20.0 * scale_factor,
        PLAYER_HEALTH_COLOR,
    );
    draw_texture_ex(
        &assets.healthbar,
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

    if show_tooltip {
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
}

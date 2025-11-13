use crate::{assets::Assets, player::Player};
use macroquad::prelude::*;

pub struct EnemyType {
    pub draw_fn: &'static dyn Fn(&Assets, &Enemy),
    pub speed: f32,
}

pub const GREENO: EnemyType = EnemyType {
    draw_fn: &|assets, enemy| {
        draw_texture_ex(
            assets.enemies.animations[0].get_at_time((enemy.animation_time * 1000.0) as u32),
            enemy.pos.x.floor(),
            enemy.pos.y.floor(),
            WHITE,
            DrawTextureParams {
                flip_x: enemy.moving_left,
                ..Default::default()
            },
        );
    },
    speed: 20.0,
};

pub struct Enemy {
    pub ty: &'static EnemyType,
    pub pos: Vec2,
    pub health: f32,
    pub animation_time: f32,
    pub moving_left: bool,
}
impl Enemy {
    pub fn update(&mut self, delta_time: f32, player: &mut Player) {
        let delta = player.pos - self.pos;
        if delta.length() > 0.0 {
            self.pos += delta.normalize() * delta_time * self.ty.speed;
            self.animation_time += delta_time;
            self.moving_left = delta.x < 0.0;
        }
    }
    pub fn draw(&self, assets: &Assets) {
        (self.ty.draw_fn)(assets, &self);
    }
}

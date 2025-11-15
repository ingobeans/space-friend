use std::collections::VecDeque;

use crate::{
    assets::{Assets, World},
    player::{Player, update_physicsbody},
};
use macroquad::prelude::*;

pub struct EnemyType {
    pub animation_id: usize,
    pub melee_attack: Option<f32>,
    pub speed: f32,
    pub health: f32,
    pub pathfind: bool,
}

pub const GREENO: EnemyType = EnemyType {
    animation_id: 0,
    health: 20.0,
    speed: 25.0,
    pathfind: false,
    melee_attack: Some(15.0),
};
pub const DOG: EnemyType = EnemyType {
    animation_id: 2,
    health: 9.0,
    speed: 80.0,
    pathfind: false,
    melee_attack: Some(10.0),
};
pub static ENEMIES: &[EnemyType] = &[GREENO, DOG];

pub struct Enemy {
    pub ty: &'static EnemyType,
    pub pos: Vec2,
    pub health: f32,
    pub animation_time: f32,
    pub moving_left: bool,
    pub path: Option<VecDeque<(i16, i16)>>,
    pub time_til_pathfind: f32,
    pub velocity: Vec2,
    pub attacking: bool,
    pub emerging: bool,
    pub just_finished_attack: bool,
}
impl Enemy {
    pub fn new(ty: &'static EnemyType, pos: Vec2) -> Self {
        Self {
            ty,
            pos,
            health: ty.health,
            animation_time: 0.0,
            moving_left: false,
            path: None,
            time_til_pathfind: 0.0,
            attacking: false,
            emerging: true,
            just_finished_attack: false,
            velocity: Vec2::ZERO,
        }
    }
    pub fn update(&mut self, delta_time: f32, player: &mut Player, world: &World) {
        self.animation_time += delta_time;
        if self.emerging && self.animation_time < HOLE_TIME {
            return;
        } else if self.emerging {
            self.emerging = false;
        }
        if self.attacking {
            return;
        }
        let delta = player.pos - self.pos;
        let mut target = player.pos + 8.0;
        if delta.length() > 0.0 {
            self.time_til_pathfind -= delta_time;

            if self.ty.pathfind && (self.path.is_none() || self.time_til_pathfind <= 0.0) {
                self.time_til_pathfind = 2.0;
                self.path = world
                    .pathfind(self.pos, player.pos + 8.0)
                    .map(|f| f.0.into());
            }
            if let Some(path) = &mut self.path
                && let Some((x, y)) = path.get(1)
            {
                let next = vec2(*x as f32 * 16.0, *y as f32 * 16.0);
                if next.distance(self.pos) < 4.0 {
                    path.pop_front();
                }
                target = next;
            }
        }
        let distance = target.distance_squared(self.pos);

        if self.just_finished_attack {
            self.just_finished_attack = false;
            if distance < 250.0
                && let Some(damage) = self.ty.melee_attack
            {
                player.health -= damage;
            }
        } else if distance < 144.0 && self.ty.melee_attack.is_some() {
            self.animation_time = 0.0;
            self.attacking = true;
        } else if distance > 0.0 {
            self.moving_left = (target - self.pos).x > 0.0;
            self.velocity = (target - self.pos).normalize() * self.ty.speed;
            self.pos = update_physicsbody(self.pos, &mut self.velocity, delta_time, &world);
        }
    }
    pub fn draw(&mut self, assets: &Assets) {
        if self.emerging && self.animation_time < HOLE_TIME {
            let max_hole_diameter = 20.0;
            let diameter = (self.animation_time / HOLE_EMERGE_TIME * max_hole_diameter)
                .min(max_hole_diameter)
                .floor();
            draw_ellipse(
                self.pos.x.floor(),
                self.pos.y.floor() + 8.0,
                diameter,
                diameter / 2.0,
                0.0,
                BLACK,
            );
            if self.animation_time > HOLE_EMERGE_TIME {
                let amt = (self.animation_time - HOLE_EMERGE_TIME) / (HOLE_TIME - HOLE_EMERGE_TIME);
                let amt = (amt - 1.0).powi(5) + 1.0;
                let pos = self.pos.floor() + vec2(0.0, 13.0 - amt * 13.0);
                draw_texture_ex(
                    assets.enemies.animations[self.ty.animation_id]
                        .get_at_time((self.animation_time * 1000.0) as u32),
                    pos.x.floor() - 16.0,
                    pos.y.floor() - 16.0,
                    WHITE,
                    DrawTextureParams {
                        flip_x: self.moving_left,
                        ..Default::default()
                    },
                );
            }
            return;
        }
        if self.attacking {
            if self.animation_time * 1000.0
                >= assets.enemies.animations[self.ty.animation_id + 1].total_length as f32
            {
                self.attacking = false;
                self.just_finished_attack = true;
            }
        }
        let animation_id = if self.attacking {
            self.ty.animation_id + 1
        } else {
            self.ty.animation_id
        };
        draw_texture_ex(
            assets.enemies.animations[animation_id]
                .get_at_time((self.animation_time * 1000.0) as u32),
            self.pos.x.floor() - 16.0,
            self.pos.y.floor() - 16.0,
            WHITE,
            DrawTextureParams {
                flip_x: self.moving_left,
                ..Default::default()
            },
        );
        let width = 25.0;
        let height = 4.0;
        let pos = self.pos.floor() - 16.0 + vec2(0.0, -4.0) + (32.0 - width) / 2.0;
        draw_rectangle(pos.x - 1.0, pos.y - 1.0, width + 2.0, height + 2.0, BLACK);
        draw_rectangle(
            pos.x,
            pos.y,
            self.health / self.ty.health * width,
            height,
            HEALTHBAR_COLOR,
        );
    }
}
pub const HEALTHBAR_COLOR: Color = Color::from_hex(0x39741f);
const HOLE_EMERGE_TIME: f32 = 0.7;
const HOLE_TIME: f32 = 1.8;

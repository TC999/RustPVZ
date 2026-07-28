// [TRANSLATION_NOTE]: Projectile.h -> Rust struct

use crate::const_enums::*;
use super::game_object::GameObject;

#[derive(Clone)]
pub struct ProjectileDefinition {
    pub m_projectile_type: ProjectileType,
    pub m_image_row: i32,
    pub m_damage: i32,
}

#[derive(Clone)]
pub struct Projectile {
    pub base: GameObject,
    pub m_frame: i32,
    pub m_num_frames: i32,
    pub m_anim_counter: i32,
    pub m_pos_x: f32,
    pub m_pos_y: f32,
    pub m_pos_z: f32,
    pub m_vel_x: f32,
    pub m_vel_y: f32,
    pub m_vel_z: f32,
    pub m_acc_z: f32,
    pub m_shadow_y: f32,
    pub m_dead: bool,
    pub m_anim_ticks_per_frame: i32,
    pub m_motion_type: i32,
    pub m_projectile_type: ProjectileType,
    pub m_projectile_age: i32,
    pub m_click_backoff_counter: i32,
    pub m_rotation: f32,
    pub m_rotation_speed: f32,
    pub m_on_high_ground: bool,
    pub m_damage_range_flags: i32,
    pub m_hit_torchwood_grid_x: i32,
    pub m_attachment_id: AttachmentID,
    pub m_cob_target_x: f32,
    pub m_cob_target_row: i32,
    pub m_target_zombie_id: ZombieID,
    pub m_last_portal_x: i32,
}

impl Projectile {
    pub fn new() -> Self {
        Projectile {
            base: GameObject::new(),
            m_frame: 0,
            m_num_frames: 0,
            m_anim_counter: 0,
            m_pos_x: 0.0,
            m_pos_y: 0.0,
            m_pos_z: 0.0,
            m_vel_x: 0.0,
            m_vel_y: 0.0,
            m_vel_z: 0.0,
            m_acc_z: 0.0,
            m_shadow_y: 0.0,
            m_dead: false,
            m_anim_ticks_per_frame: 0,
            m_motion_type: 0,
            m_projectile_type: ProjectileType::PROJECTILE_PEA,
            m_projectile_age: 0,
            m_click_backoff_counter: 0,
            m_rotation: 0.0,
            m_rotation_speed: 0.0,
            m_on_high_ground: false,
            m_damage_range_flags: 0,
            m_hit_torchwood_grid_x: 0,
            m_attachment_id: AttachmentID::ATTACHMENTID_NULL,
            m_cob_target_x: 0.0,
            m_cob_target_row: 0,
            m_target_zombie_id: ZombieID::ZOMBIEID_NULL,
            m_last_portal_x: 0,
        }
    }

    pub unsafe fn ProjectileInitialize(&mut self, theX: i32, theY: i32, theRenderOrder: i32, theRow: i32, theProjectileType: ProjectileType) {
        self.m_pos_x = theX as f32;
        self.m_pos_y = theY as f32;
        self.base.m_render_order = theRenderOrder;
        self.base.m_row = theRow;
        self.m_projectile_type = theProjectileType;
        self.m_dead = false;
        self.m_projectile_age = 0;
        // Set velocity based on projectile type
        match theProjectileType {
            ProjectileType::PROJECTILE_PEA => { self.m_vel_x = 4.0; }
            ProjectileType::PROJECTILE_SNOWPEA => { self.m_vel_x = 4.0; }
            ProjectileType::PROJECTILE_CABBAGE => { self.m_vel_x = 3.0; self.m_vel_z = -5.0; self.m_acc_z = 0.2; }
            ProjectileType::PROJECTILE_MELON => { self.m_vel_x = 2.8; self.m_vel_z = -4.5; self.m_acc_z = 0.18; }
            _ => { self.m_vel_x = 3.0; }
        }
    }

    pub unsafe fn Update(&mut self) {
        if self.m_dead { return; }
        self.m_projectile_age += 1;
        self.m_pos_x += self.m_vel_x;
        self.m_pos_y += self.m_vel_y;
        self.m_pos_z += self.m_vel_z;
        self.m_vel_z += self.m_acc_z;

        // Rotation for some projectile types
        if self.m_rotation_speed != 0.0 {
            self.m_rotation += self.m_rotation_speed;
        }

        // Check if projectile is offscreen
        if self.m_pos_x > 900.0 || self.m_pos_x < -100.0 || self.m_pos_y > 700.0 {
            self.m_dead = true;
        }

        // Animation
        self.m_anim_counter += 1;
        if self.m_anim_counter >= 6 {
            self.m_anim_counter = 0;
            self.m_frame += 1;
            if self.m_frame >= self.m_num_frames {
                self.m_frame = 0;
            }
        }
    }

    pub unsafe fn Draw(&self, _g: &mut crate::sexy_app_framework::graphics::graphics::Graphics) {
        if self.m_dead { return; }
        // TODO: Draw projectile sprite based on projectile_type and frame
    }
}

impl Default for Projectile {
    fn default() -> Self {
        Self::new()
    }
}

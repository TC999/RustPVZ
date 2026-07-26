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
}

impl Default for Projectile {
    fn default() -> Self {
        Self::new()
    }
}

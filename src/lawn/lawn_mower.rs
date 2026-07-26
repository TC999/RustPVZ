// [TRANSLATION_NOTE]: LawnMower.h -> Rust struct

use crate::const_enums::*;
use super::game_object::GameObject;

pub struct LawnMower {
    pub base: GameObject,
    pub m_mower_type: LawnMowerType,
    pub m_pos_x: f32,
    pub m_pos_y: f32,
    pub m_row: i32,
    pub m_mower_state: MowerState,
    pub m_ground_y: i32,
    pub m_anim_counter: i32,
    pub m_frame: i32,
    pub m_rolling_in_counter: i32,
    pub m_vel_x: f32,
    pub m_squish_counter: i32,
    pub m_driving_count: bool,
    pub m_dead: bool,
    pub m_lawn_mower_age: i32,
    pub m_particle_id: ParticleID,
    pub m_attachment_id: AttachmentID,
}

impl LawnMower {
    pub fn new() -> Self {
        LawnMower {
            base: GameObject::new(),
            m_mower_type: LawnMowerType::LAWNMOWER_NORMAL,
            m_pos_x: 0.0,
            m_pos_y: 0.0,
            m_row: 0,
            m_mower_state: MowerState::MOWER_READY,
            m_ground_y: 0,
            m_anim_counter: 0,
            m_frame: 0,
            m_rolling_in_counter: 0,
            m_vel_x: 0.0,
            m_squish_counter: 0,
            m_driving_count: false,
            m_dead: false,
            m_lawn_mower_age: 0,
            m_particle_id: ParticleID::PARTICLEID_NULL,
            m_attachment_id: AttachmentID::ATTACHMENTID_NULL,
        }
    }
}

impl Default for LawnMower {
    fn default() -> Self {
        Self::new()
    }
}

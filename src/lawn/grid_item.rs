// [TRANSLATION_NOTE]: GridItem.h -> Rust struct

use crate::const_enums::*;
use super::game_object::GameObject;

pub struct GridItem {
    pub base: GameObject,
    pub m_grid_x: i32,
    pub m_grid_y: i32,
    pub m_grid_item_type: GridItemType,
    pub m_pos_x: f32,
    pub m_pos_y: f32,
    pub m_anim_counter: i32,
    pub m_frame: i32,
    pub m_dead: bool,
    pub m_reanim_id: ReanimationID,
    pub m_particle_id: ParticleID,
    pub m_coin_id: CoinID,
    pub m_swing_x: f32,
    pub m_swing_y: f32,
    pub m_door_dir: i32,
    pub m_door_moving: bool,
    pub m_tiny_uran_mov_x: i32,
    pub m_tiny_uran_mov_y: i32,
    pub m_tiny_uran_counter: i32,
    pub m_crater_counter: i32,
}

impl GridItem {
    pub fn new() -> Self {
        GridItem {
            base: GameObject::new(),
            m_grid_x: 0,
            m_grid_y: 0,
            m_grid_item_type: GridItemType::GRIDITEM_NONE,
            m_pos_x: 0.0,
            m_pos_y: 0.0,
            m_anim_counter: 0,
            m_frame: 0,
            m_dead: false,
            m_reanim_id: ReanimationID::REANIMATIONID_NULL,
            m_particle_id: ParticleID::PARTICLEID_NULL,
            m_coin_id: CoinID::COINID_NULL,
            m_swing_x: 0.0,
            m_swing_y: 0.0,
            m_door_dir: 0,
            m_door_moving: false,
            m_tiny_uran_mov_x: 0,
            m_tiny_uran_mov_y: 0,
            m_tiny_uran_counter: 0,
            m_crater_counter: 0,
        }
    }
}

impl Default for GridItem {
    fn default() -> Self {
        Self::new()
    }
}

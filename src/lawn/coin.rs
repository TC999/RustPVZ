// [TRANSLATION_NOTE]: Coin.h -> Rust struct

use crate::const_enums::*;
use super::game_object::GameObject;

#[derive(Clone)]
pub struct PottedPlant {
    pub m_potted_plant_index: i32,
    pub m_seed_type: SeedType,
    pub m_draw_variation: DrawVariation,
    pub m_age: i32,
    pub m_need: i32,
    pub m_watered: bool,
    pub m_last_watered_time: i32,
    pub m_last_fertilized_time: i32,
    pub m_last_bug_sprayed_time: i32,
    pub m_last_phonograph_time: i32,
    pub m_last_chocolate_time: i32,
    pub m_fertilizer_count: i32,
    pub m_bug_spray_count: i32,
    pub m_phonograph_count: i32,
    pub m_chocolate_count: i32,
    pub m_plant_health: i32,
    pub m_facing_left: bool,
    pub m_which_flower: i32,
    pub m_times_fertilized: i32,
    pub m_last_needy_time: i32,
    pub m_twice_watered: bool,
}

impl PottedPlant {
    pub fn new() -> Self {
        PottedPlant {
            m_potted_plant_index: 0,
            m_seed_type: SeedType::SEED_MARIGOLD,
            m_draw_variation: DrawVariation::VARIATION_NORMAL,
            m_age: 0,
            m_need: 0,
            m_watered: false,
            m_last_watered_time: -1,
            m_last_fertilized_time: -1,
            m_last_bug_sprayed_time: -1,
            m_last_phonograph_time: -1,
            m_last_chocolate_time: -1,
            m_fertilizer_count: 0,
            m_bug_spray_count: 0,
            m_phonograph_count: 0,
            m_chocolate_count: 0,
            m_plant_health: 0,
            m_facing_left: false,
            m_which_flower: 0,
            m_times_fertilized: 0,
            m_last_needy_time: 0,
            m_twice_watered: false,
        }
    }
}

#[derive(Clone)]
pub struct Coin {
    pub base: GameObject,
    pub m_pos_x: f32,
    pub m_pos_y: f32,
    pub m_vel_x: f32,
    pub m_vel_y: f32,
    pub m_scale: f32,
    pub m_dead: bool,
    pub m_fade_count: i32,
    pub m_collect_x: f32,
    pub m_collect_y: f32,
    pub m_ground_y: i32,
    pub m_coin_age: i32,
    pub m_is_being_collected: bool,
    pub m_disappear_counter: i32,
    pub m_type: CoinType,
    pub m_coin_motion: CoinMotion,
    pub m_attachment_id: AttachmentID,
    pub m_collection_distance: f32,
    pub m_usable_seed_type: SeedType,
    pub m_potted_plant_spec: PottedPlant,
    pub m_needs_bouncy_arrow: bool,
    pub m_has_bouncy_arrow: bool,
    pub m_hit_ground: bool,
    pub m_times_dropped: i32,
}

impl Coin {
    pub fn new() -> Self {
        Coin {
            base: GameObject::new(),
            m_pos_x: 0.0,
            m_pos_y: 0.0,
            m_vel_x: 0.0,
            m_vel_y: 0.0,
            m_scale: 1.0,
            m_dead: false,
            m_fade_count: 0,
            m_collect_x: 0.0,
            m_collect_y: 0.0,
            m_ground_y: 0,
            m_coin_age: 0,
            m_is_being_collected: false,
            m_disappear_counter: 0,
            m_type: CoinType::COIN_NONE,
            m_coin_motion: CoinMotion::COIN_MOTION_FROM_SKY,
            m_attachment_id: AttachmentID::ATTACHMENTID_NULL,
            m_collection_distance: 0.0,
            m_usable_seed_type: SeedType::SEED_NONE,
            m_potted_plant_spec: PottedPlant::new(),
            m_needs_bouncy_arrow: false,
            m_has_bouncy_arrow: false,
            m_hit_ground: false,
            m_times_dropped: 0,
        }
    }
}

impl Default for Coin {
    fn default() -> Self {
        Self::new()
    }
}

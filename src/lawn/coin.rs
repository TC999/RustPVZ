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

    /// C++ Coin::CoinInitialize (Coin.cpp:50)
    pub unsafe fn CoinInitialize(&mut self, theX: i32, theY: i32, theCoinType: CoinType, theCoinMotion: CoinMotion) {
        self.m_pos_x = theX as f32;
        self.m_pos_y = theY as f32;
        self.m_ground_y = theY;
        self.base.m_x = theX;
        self.base.m_y = theY;
        self.m_type = theCoinType;
        self.m_coin_motion = theCoinMotion;
        self.m_dead = false;
        self.m_coin_age = 0;
        self.base.m_visible = true;
        self.m_hit_ground = false;
        self.m_times_dropped = 0;
        self.m_disappear_counter = 0;
        self.m_is_being_collected = false;

        // 初始化速度
        match theCoinMotion {
            CoinMotion::COIN_MOTION_FROM_SKY => {
                self.m_vel_x = crate::sexy_app_framework::common::rand_float(2.0) - 1.0;
                self.m_vel_y = -3.0;
                self.m_scale = 1.0;
            }
            CoinMotion::COIN_MOTION_FROM_PLANT => {
                self.m_vel_x = crate::sexy_app_framework::common::rand_float(2.0) + 1.0;
                self.m_vel_y = -4.0 - crate::sexy_app_framework::common::rand_float(3.0);
                self.m_scale = 0.75;
            }
            CoinMotion::COIN_MOTION_COIN => {
                self.m_vel_x = crate::sexy_app_framework::common::rand_float(4.0) - 2.0;
                self.m_vel_y = -7.0 - crate::sexy_app_framework::common::rand_float(3.0);
            }
            _ => {}
        }
    }

    /// C++ Coin::Update (Coin.cpp:735)
    pub unsafe fn Update(&mut self) {
        if self.m_dead {
            return;
        }
        self.m_coin_age += 1;

        // C++: UpdateFall — 下落/弹跳
        if !self.m_is_being_collected && self.m_coin_motion != CoinMotion::COIN_MOTION_FROM_PRESENT {
            self.m_vel_y += 0.3; // gravity
            self.m_pos_y += self.m_vel_y;
            self.m_pos_x += self.m_vel_x;

            // 空气阻力（水平减速）
            self.m_vel_x *= 0.95;

            // 弹跳
            if self.m_pos_y >= self.m_ground_y as f32 {
                self.m_pos_y = self.m_ground_y as f32;
                if self.m_vel_y > 0.5 {
                    self.m_vel_y = -(self.m_vel_y * 0.5); // bounce
                    self.m_times_dropped += 1;
                } else if self.m_vel_y > 0.0 {
                    self.m_vel_y = 0.0;
                    self.m_hit_ground = true;
                }
                if self.m_times_dropped >= 4 {
                    self.m_hit_ground = true;
                }
            }
        }

        // C++: UpdateCollected — 收集动画
        if self.m_is_being_collected {
            let dx = self.m_collect_x - self.m_pos_x;
            let dy = self.m_collect_y - self.m_pos_y;
            self.m_pos_x += dx * 0.1;
            self.m_pos_y += dy * 0.1;
            self.m_fade_count += 1;
            if self.m_fade_count > 50 {
                self.m_dead = true;
            }
        }

        // 自动消失
        if self.m_disappear_counter > 0 {
            self.m_disappear_counter -= 1;
            if self.m_disappear_counter == 0 {
                self.m_dead = true;
            }
        }

        self.base.m_x = self.m_pos_x as i32;
        self.base.m_y = self.m_pos_y as i32;
    }

    /// C++ Coin::Collect (Coin.cpp:1050)
    pub unsafe fn Collect(&mut self, theCollectX: f32, theCollectY: f32) {
        self.m_is_being_collected = true;
        self.m_collect_x = theCollectX;
        self.m_collect_y = theCollectY;
        self.m_fade_count = 0;
    }

    /// C++ Coin::Die (Coin.cpp:1411)
    pub unsafe fn Die(&mut self) {
        self.m_dead = true;
        // [TODO]: Remove attachment if any
    }

    pub unsafe fn Draw(&self, _g: &mut crate::sexy_app_framework::graphics::graphics::Graphics) {
        if self.m_dead { return; }
        // [TODO]: Draw coin sprite (IMAGE_SUN, IMAGE_SILVER_COIN, etc.)
    }

    pub unsafe fn MouseDown(&mut self, _x: i32, _y: i32, _click_count: i32) {
        if self.m_dead { return; }
        // C++: if mouse hits coin, start collection
        self.m_is_being_collected = true;
    }

    pub unsafe fn GetCoinValue(theCoinType: CoinType) -> i32 {
        match theCoinType {
            CoinType::COIN_SUN => 25,
            CoinType::COIN_LARGESUN => 50,
            CoinType::COIN_SILVER => 10,
            CoinType::COIN_GOLD => 100,
            CoinType::COIN_DIAMOND => 1000,
            CoinType::COIN_FINAL_SEED_PACKET => 100,
            _ => 0,
        }
    }
}

impl Default for Coin {
    fn default() -> Self {
        Self::new()
    }
}

// [TRANSLATION_NOTE]: Zombie.h -> Rust 模块
// C++ Zombie 类翻译为 Rust struct + impl

use crate::const_enums::*;
use super::game_object::GameObject;
use crate::sexy_app_framework::graphics::graphics::Graphics;
use crate::sexy_app_framework::common::*;
use crate::sexy_tod_lib::tod_foley::FoleyType;
use crate::sexy_tod_lib::reanimator::ReanimLoopType;
use crate::sexy_tod_lib::tod_common::rand_range_float;

pub const MAX_ZOMBIE_FOLLOWERS: i32 = 4;
pub const NUM_BOBSLED_FOLLOWERS: i32 = 3;
pub const NUM_BACKUP_DANCERS: i32 = 4;
pub const NUM_BOSS_BUNGEES: i32 = 3;

pub const ZOMBIE_START_RANDOM_OFFSET: i32 = 40;
pub const BUNGEE_ZOMBIE_HEIGHT: i32 = 3000;
pub const RENDER_GROUP_SHIELD: i32 = 1;
pub const RENDER_GROUP_ARMS: i32 = 2;
pub const RENDER_GROUP_OVER_SHIELD: i32 = 3;
pub const RENDER_GROUP_BOSS_BACK_LEG: i32 = 4;
pub const RENDER_GROUP_BOSS_FRONT_LEG: i32 = 5;
pub const RENDER_GROUP_BOSS_BACK_ARM: i32 = 6;
pub const RENDER_GROUP_BOSS_FIREBALL_ADDITIVE: i32 = 7;
pub const RENDER_GROUP_BOSS_FIREBALL_TOP: i32 = 8;
pub const ZOMBIE_LIMP_SPEED_FACTOR: i32 = 2;
pub const POGO_BOUNCE_TIME: i32 = 80;
pub const DOLPHIN_JUMP_TIME: i32 = 120;
pub const JACK_IN_THE_BOX_ZOMBIE_RADIUS: i32 = 115;
pub const JACK_IN_THE_BOX_PLANT_RADIUS: i32 = 90;
pub const BOBSLED_CRASH_TIME: i32 = 150;
pub const ZOMBIE_BACKUP_DANCER_RISE_HEIGHT: i32 = -200;
pub const BOSS_FLASH_HEALTH_FRACTION: i32 = 10;
pub const TICKS_BETWEEN_EATS: i32 = 4;
pub const DAMAGE_PER_EAT: i32 = TICKS_BETWEEN_EATS;
pub const THROWN_ZOMBIE_GRAVITY: f32 = 0.05;
pub const CHILLED_SPEED_FACTOR: f32 = 0.4;
pub const CLIP_HEIGHT_LIMIT: f32 = -100.0;
pub const CLIP_HEIGHT_OFF: f32 = -200.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum ZombieAttackType {
    ATTACKTYPE_CHEW,
    ATTACKTYPE_DRIVE_OVER,
    ATTACKTYPE_VAULT,
    ATTACKTYPE_LADDER,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum ZombieParts {
    PART_BODY,
    PART_HEAD,
    PART_HEAD_EATING,
    PART_TONGUE,
    PART_ARM,
    PART_HAIR,
    PART_HEAD_YUCKY,
    PART_ARM_PICKAXE,
    PART_ARM_POLEVAULT,
    PART_ARM_LEASH,
    PART_ARM_FLAG,
    PART_POGO,
    PART_DIGGER,
}

#[derive(Clone)]
pub struct ZombieDrawPosition {
    pub m_head_x: i32,
    pub m_head_y: i32,
    pub m_arm_y: i32,
    pub m_body_y: f32,
    pub m_image_offset_x: f32,
    pub m_image_offset_y: f32,
    pub m_clip_height: f32,
}

impl ZombieDrawPosition {
    pub fn new() -> Self {
        ZombieDrawPosition {
            m_head_x: 0, m_head_y: 0, m_arm_y: 0,
            m_body_y: 0.0, m_image_offset_x: 0.0, m_image_offset_y: 0.0,
            m_clip_height: 0.0,
        }
    }
}

// Wave constants
pub const ZOMBIE_WAVE_DEBUG: i32 = -1;
pub const ZOMBIE_WAVE_CUTSCENE: i32 = -2;
pub const ZOMBIE_WAVE_UI: i32 = -3;
pub const ZOMBIE_WAVE_WINNER: i32 = -4;
pub const HIGH_GROUND_HEIGHT: i32 = 60;

#[derive(Clone)]
pub struct Zombie {
    pub base: GameObject,
    pub m_zombie_type: ZombieType,
    pub m_zombie_phase: ZombiePhase,
    pub m_pos_x: f32,
    pub m_pos_y: f32,
    pub m_vel_x: f32,
    pub m_anim_counter: i32,
    pub m_groan_counter: i32,
    pub m_anim_ticks_per_frame: i32,
    pub m_anim_frames: i32,
    pub m_frame: i32,
    pub m_prev_frame: i32,
    pub m_variant: bool,
    pub m_is_eating: bool,
    pub m_just_got_shot_counter: i32,
    pub m_shield_just_got_shot_counter: i32,
    pub m_shield_recoil_counter: i32,
    pub m_zombie_age: i32,
    pub m_zombie_height: ZombieHeight,
    pub m_phase_counter: i32,
    pub m_from_wave: i32,
    pub m_dropped_loot: bool,
    pub m_zombie_fade: i32,
    pub m_flat_tires: bool,
    pub m_use_ladder_col: i32,
    pub m_target_col: i32,
    pub m_altitude: f32,
    pub m_hit_umbrella: bool,
    pub m_zombie_rect: crate::sexy_app_framework::misc::rect::Rect,
    pub m_zombie_attack_rect: crate::sexy_app_framework::misc::rect::Rect,
    pub m_chilled_counter: i32,
    pub m_buttered_counter: i32,
    pub m_ice_trap_counter: i32,
    pub m_mind_controlled: bool,
    pub m_blowing_away: bool,
    pub m_has_head: bool,
    pub m_has_arm: bool,
    pub m_has_object: bool,
    pub m_in_pool: bool,
    pub m_on_high_ground: bool,
    pub m_yucky_face: bool,
    pub m_yucky_face_counter: i32,
    pub m_helm_type: i32,
    pub m_body_health: i32,
    pub m_body_max_health: i32,
    pub m_helm_health: i32,
    pub m_helm_max_health: i32,
    pub m_shield_type: ShieldType,
    pub m_shield_health: i32,
    pub m_shield_max_health: i32,
    pub m_flying_health: i32,
    pub m_flying_max_health: i32,
    pub m_dead: bool,
    pub m_related_zombie_id: ZombieID,
    pub m_follower_zombie_id: [ZombieID; MAX_ZOMBIE_FOLLOWERS as usize],
    pub m_playing_song: bool,
    pub m_particle_offset_x: i32,
    pub m_particle_offset_y: i32,
    pub m_attachment_id: AttachmentID,
    pub m_summon_counter: i32,
    pub m_body_reanim_id: ReanimationID,
    pub m_scale_zombie: f32,
    pub m_vel_z: f32,
    pub m_original_anim_rate: f32,
    pub m_target_plant_id: PlantID,
    pub m_boss_mode: i32,
    pub m_target_row: i32,
    pub m_boss_bungee_counter: i32,
    pub m_boss_stomp_counter: i32,
    pub m_boss_head_counter: i32,
    pub m_boss_fire_ball_reanim_id: ReanimationID,
    pub m_special_head_reanim_id: ReanimationID,
    pub m_fireball_row: i32,
    pub m_is_fire_ball: bool,
    pub m_mowered_reanim_id: ReanimationID,
    pub m_zombatar_head_reanim_id: ReanimationID,
    pub m_last_portal_x: i32,
}

impl Zombie {
    pub fn new() -> Self {
        Zombie {
            base: GameObject::new(),
            m_zombie_type: ZombieType::ZOMBIE_NORMAL,
            m_zombie_phase: ZombiePhase::PHASE_ZOMBIE_NORMAL,
            m_pos_x: 0.0,
            m_pos_y: 0.0,
            m_vel_x: 0.0,
            m_anim_counter: 0,
            m_groan_counter: 0,
            m_anim_ticks_per_frame: 0,
            m_anim_frames: 0,
            m_frame: 0,
            m_prev_frame: 0,
            m_variant: false,
            m_is_eating: false,
            m_just_got_shot_counter: 0,
            m_shield_just_got_shot_counter: 0,
            m_shield_recoil_counter: 0,
            m_zombie_age: 0,
            m_zombie_height: ZombieHeight::HEIGHT_ZOMBIE_NORMAL,
            m_phase_counter: 0,
            m_from_wave: 0,
            m_dropped_loot: false,
            m_zombie_fade: 0,
            m_flat_tires: false,
            m_use_ladder_col: 0,
            m_target_col: 0,
            m_altitude: 0.0,
            m_hit_umbrella: false,
            m_zombie_rect: crate::sexy_app_framework::misc::rect::Rect::new(0, 0, 0, 0),
            m_zombie_attack_rect: crate::sexy_app_framework::misc::rect::Rect::new(0, 0, 0, 0),
            m_chilled_counter: 0,
            m_buttered_counter: 0,
            m_ice_trap_counter: 0,
            m_mind_controlled: false,
            m_blowing_away: false,
            m_has_head: true,
            m_has_arm: true,
            m_has_object: true,
            m_in_pool: false,
            m_on_high_ground: false,
            m_yucky_face: false,
            m_yucky_face_counter: 0,
            m_helm_type: 0,
            m_body_health: 0,
            m_body_max_health: 0,
            m_helm_health: 0,
            m_helm_max_health: 0,
            m_shield_type: ShieldType::SHIELDTYPE_NONE,
            m_shield_health: 0,
            m_shield_max_health: 0,
            m_flying_health: 0,
            m_flying_max_health: 0,
            m_dead: false,
            m_related_zombie_id: ZombieID::ZOMBIEID_NULL,
            m_follower_zombie_id: [ZombieID::ZOMBIEID_NULL; MAX_ZOMBIE_FOLLOWERS as usize],
            m_playing_song: false,
            m_particle_offset_x: 0,
            m_particle_offset_y: 0,
            m_attachment_id: AttachmentID::ATTACHMENTID_NULL,
            m_summon_counter: 0,
            m_body_reanim_id: ReanimationID::REANIMATIONID_NULL,
            m_scale_zombie: 1.0,
            m_vel_z: 0.0,
            m_original_anim_rate: 1.0,
            m_target_plant_id: PlantID::PLANTID_NULL,
            m_boss_mode: 0,
            m_target_row: 0,
            m_boss_bungee_counter: 0,
            m_boss_stomp_counter: 0,
            m_boss_head_counter: 0,
            m_boss_fire_ball_reanim_id: ReanimationID::REANIMATIONID_NULL,
            m_special_head_reanim_id: ReanimationID::REANIMATIONID_NULL,
            m_fireball_row: 0,
            m_is_fire_ball: false,
            m_mowered_reanim_id: ReanimationID::REANIMATIONID_NULL,
            m_zombatar_head_reanim_id: ReanimationID::REANIMATIONID_NULL,
            m_last_portal_x: 0,
        }
    }
}

impl Default for Zombie {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// ★ ZombieDefinition — 僵尸类型定义数据 (from Zombie.h:423)
// C++ class ZombieDefinition 的 1:1 翻译
// =========================================================================
#[derive(Clone, Copy)]
pub struct ZombieDefinition {
    pub mZombieType: ZombieType,
    pub mReanimationType: ReanimationType,
    pub mZombieValue: i32,
    pub mStartingLevel: i32,
    pub mFirstAllowedWave: i32,
    pub mPickWeight: i32,
    pub mZombieName: &'static str,
}

// gZombieDefs array — 34 entries matching C++ gZombieDefs
pub static mut G_ZOMBIE_DEFS: [ZombieDefinition; 33] = [
    // ZOMBIE_NORMAL (0)
    ZombieDefinition { mZombieType: ZombieType::ZOMBIE_NORMAL, mReanimationType: ReanimationType::REANIM_ZOMBIE, mZombieValue: 1, mStartingLevel: 1, mFirstAllowedWave: 1, mPickWeight: 4000, mZombieName: "ZOMBIE" },
    // ZOMBIE_FLAG (1)
    ZombieDefinition { mZombieType: ZombieType::ZOMBIE_FLAG, mReanimationType: ReanimationType::REANIM_ZOMBIE, mZombieValue: 1, mStartingLevel: 1, mFirstAllowedWave: 1, mPickWeight: 0, mZombieName: "FLAG_ZOMBIE" },
    // ZOMBIE_TRAFFIC_CONE (2)
    ZombieDefinition { mZombieType: ZombieType::ZOMBIE_TRAFFIC_CONE, mReanimationType: ReanimationType::REANIM_ZOMBIE, mZombieValue: 2, mStartingLevel: 3, mFirstAllowedWave: 1, mPickWeight: 4000, mZombieName: "CONEHEAD_ZOMBIE" },
    // ZOMBIE_POLEVAULTER (3)
    ZombieDefinition { mZombieType: ZombieType::ZOMBIE_POLEVAULTER, mReanimationType: ReanimationType::REANIM_POLEVAULTER, mZombieValue: 2, mStartingLevel: 6, mFirstAllowedWave: 5, mPickWeight: 2000, mZombieName: "POLE_VAULTING_ZOMBIE" },
    // ZOMBIE_PAIL (4)
    ZombieDefinition { mZombieType: ZombieType::ZOMBIE_PAIL, mReanimationType: ReanimationType::REANIM_ZOMBIE, mZombieValue: 4, mStartingLevel: 8, mFirstAllowedWave: 1, mPickWeight: 3000, mZombieName: "BUCKETHEAD_ZOMBIE" },
    // ZOMBIE_NEWSPAPER (5)
    ZombieDefinition { mZombieType: ZombieType::ZOMBIE_NEWSPAPER, mReanimationType: ReanimationType::REANIM_ZOMBIE_NEWSPAPER, mZombieValue: 2, mStartingLevel: 11, mFirstAllowedWave: 1, mPickWeight: 1000, mZombieName: "NEWSPAPER_ZOMBIE" },
    // ZOMBIE_DOOR (6)
    ZombieDefinition { mZombieType: ZombieType::ZOMBIE_DOOR, mReanimationType: ReanimationType::REANIM_ZOMBIE, mZombieValue: 4, mStartingLevel: 13, mFirstAllowedWave: 5, mPickWeight: 3500, mZombieName: "SCREEN_DOOR_ZOMBIE" },
    // ZOMBIE_FOOTBALL (7)
    ZombieDefinition { mZombieType: ZombieType::ZOMBIE_FOOTBALL, mReanimationType: ReanimationType::REANIM_ZOMBIE_FOOTBALL, mZombieValue: 7, mStartingLevel: 16, mFirstAllowedWave: 5, mPickWeight: 2000, mZombieName: "FOOTBALL_ZOMBIE" },
    // ZOMBIE_DANCER (8)
    ZombieDefinition { mZombieType: ZombieType::ZOMBIE_DANCER, mReanimationType: ReanimationType::REANIM_DANCER, mZombieValue: 5, mStartingLevel: 18, mFirstAllowedWave: 5, mPickWeight: 1000, mZombieName: "DANCING_ZOMBIE" },
    // ZOMBIE_BACKUP_DANCER (9)
    ZombieDefinition { mZombieType: ZombieType::ZOMBIE_BACKUP_DANCER, mReanimationType: ReanimationType::REANIM_BACKUP_DANCER, mZombieValue: 1, mStartingLevel: 18, mFirstAllowedWave: 1, mPickWeight: 0, mZombieName: "BACKUP_DANCER" },
    // ZOMBIE_DUCKY_TUBE (10)
    ZombieDefinition { mZombieType: ZombieType::ZOMBIE_DUCKY_TUBE, mReanimationType: ReanimationType::REANIM_ZOMBIE, mZombieValue: 1, mStartingLevel: 21, mFirstAllowedWave: 5, mPickWeight: 0, mZombieName: "DUCKY_TUBE_ZOMBIE" },
    // ZOMBIE_SNORKEL (11)
    ZombieDefinition { mZombieType: ZombieType::ZOMBIE_SNORKEL, mReanimationType: ReanimationType::REANIM_SNORKEL, mZombieValue: 3, mStartingLevel: 23, mFirstAllowedWave: 10, mPickWeight: 2000, mZombieName: "SNORKEL_ZOMBIE" },
    // ZOMBIE_ZAMBONI (12)
    ZombieDefinition { mZombieType: ZombieType::ZOMBIE_ZAMBONI, mReanimationType: ReanimationType::REANIM_ZOMBIE_ZAMBONI, mZombieValue: 7, mStartingLevel: 26, mFirstAllowedWave: 10, mPickWeight: 2000, mZombieName: "ZOMBONI" },
    // ZOMBIE_BOBSLED (13)
    ZombieDefinition { mZombieType: ZombieType::ZOMBIE_BOBSLED, mReanimationType: ReanimationType::REANIM_BOBSLED, mZombieValue: 3, mStartingLevel: 26, mFirstAllowedWave: 10, mPickWeight: 2000, mZombieName: "ZOMBIE_BOBSLED_TEAM" },
    // ZOMBIE_DOLPHIN_RIDER (14)
    ZombieDefinition { mZombieType: ZombieType::ZOMBIE_DOLPHIN_RIDER, mReanimationType: ReanimationType::REANIM_ZOMBIE_DOLPHINRIDER, mZombieValue: 3, mStartingLevel: 28, mFirstAllowedWave: 10, mPickWeight: 1500, mZombieName: "DOLPHIN_RIDER_ZOMBIE" },
    // ZOMBIE_JACK_IN_THE_BOX (15)
    ZombieDefinition { mZombieType: ZombieType::ZOMBIE_JACK_IN_THE_BOX, mReanimationType: ReanimationType::REANIM_JACKINTHEBOX, mZombieValue: 3, mStartingLevel: 31, mFirstAllowedWave: 10, mPickWeight: 1000, mZombieName: "JACK_IN_THE_BOX_ZOMBIE" },
    // ZOMBIE_BALLOON (16)
    ZombieDefinition { mZombieType: ZombieType::ZOMBIE_BALLOON, mReanimationType: ReanimationType::REANIM_BALLOON, mZombieValue: 2, mStartingLevel: 33, mFirstAllowedWave: 10, mPickWeight: 2000, mZombieName: "BALLOON_ZOMBIE" },
    // ZOMBIE_DIGGER (17)
    ZombieDefinition { mZombieType: ZombieType::ZOMBIE_DIGGER, mReanimationType: ReanimationType::REANIM_DIGGER, mZombieValue: 4, mStartingLevel: 36, mFirstAllowedWave: 10, mPickWeight: 1000, mZombieName: "DIGGER_ZOMBIE" },
    // ZOMBIE_POGO (18)
    ZombieDefinition { mZombieType: ZombieType::ZOMBIE_POGO, mReanimationType: ReanimationType::REANIM_POGO, mZombieValue: 4, mStartingLevel: 38, mFirstAllowedWave: 10, mPickWeight: 1000, mZombieName: "POGO_ZOMBIE" },
    // ZOMBIE_YETI (19)
    ZombieDefinition { mZombieType: ZombieType::ZOMBIE_YETI, mReanimationType: ReanimationType::REANIM_YETI, mZombieValue: 4, mStartingLevel: 40, mFirstAllowedWave: 1, mPickWeight: 1, mZombieName: "ZOMBIE_YETI" },
    // ZOMBIE_BUNGEE (20)
    ZombieDefinition { mZombieType: ZombieType::ZOMBIE_BUNGEE, mReanimationType: ReanimationType::REANIM_BUNGEE, mZombieValue: 3, mStartingLevel: 41, mFirstAllowedWave: 10, mPickWeight: 1000, mZombieName: "BUNGEE_ZOMBIE" },
    // ZOMBIE_LADDER (21)
    ZombieDefinition { mZombieType: ZombieType::ZOMBIE_LADDER, mReanimationType: ReanimationType::REANIM_LADDER, mZombieValue: 4, mStartingLevel: 43, mFirstAllowedWave: 10, mPickWeight: 1000, mZombieName: "LADDER_ZOMBIE" },
    // ZOMBIE_CATAPULT (22)
    ZombieDefinition { mZombieType: ZombieType::ZOMBIE_CATAPULT, mReanimationType: ReanimationType::REANIM_CATAPULT, mZombieValue: 5, mStartingLevel: 46, mFirstAllowedWave: 10, mPickWeight: 1500, mZombieName: "CATAPULT_ZOMBIE" },
    // ZOMBIE_GARGANTUAR (23)
    ZombieDefinition { mZombieType: ZombieType::ZOMBIE_GARGANTUAR, mReanimationType: ReanimationType::REANIM_GARGANTUAR, mZombieValue: 10, mStartingLevel: 48, mFirstAllowedWave: 15, mPickWeight: 1500, mZombieName: "GARGANTUAR" },
    // ZOMBIE_IMP (24)
    ZombieDefinition { mZombieType: ZombieType::ZOMBIE_IMP, mReanimationType: ReanimationType::REANIM_IMP, mZombieValue: 10, mStartingLevel: 48, mFirstAllowedWave: 1, mPickWeight: 0, mZombieName: "IMP" },
    // ZOMBIE_BOSS (25)
    ZombieDefinition { mZombieType: ZombieType::ZOMBIE_BOSS, mReanimationType: ReanimationType::REANIM_BOSS, mZombieValue: 10, mStartingLevel: 50, mFirstAllowedWave: 1, mPickWeight: 0, mZombieName: "BOSS" },
    // ZOMBIE_PEA_HEAD (26)
    ZombieDefinition { mZombieType: ZombieType::ZOMBIE_PEA_HEAD, mReanimationType: ReanimationType::REANIM_ZOMBIE, mZombieValue: 1, mStartingLevel: 99, mFirstAllowedWave: 1, mPickWeight: 4000, mZombieName: "ZOMBIE" },
    // ZOMBIE_WALLNUT_HEAD (27)
    ZombieDefinition { mZombieType: ZombieType::ZOMBIE_WALLNUT_HEAD, mReanimationType: ReanimationType::REANIM_ZOMBIE, mZombieValue: 4, mStartingLevel: 99, mFirstAllowedWave: 1, mPickWeight: 3000, mZombieName: "ZOMBIE" },
    // ZOMBIE_JALAPENO_HEAD (28)
    ZombieDefinition { mZombieType: ZombieType::ZOMBIE_JALAPENO_HEAD, mReanimationType: ReanimationType::REANIM_ZOMBIE, mZombieValue: 3, mStartingLevel: 99, mFirstAllowedWave: 10, mPickWeight: 1000, mZombieName: "ZOMBIE" },
    // ZOMBIE_GATLING_HEAD (29)
    ZombieDefinition { mZombieType: ZombieType::ZOMBIE_GATLING_HEAD, mReanimationType: ReanimationType::REANIM_ZOMBIE, mZombieValue: 3, mStartingLevel: 99, mFirstAllowedWave: 10, mPickWeight: 2000, mZombieName: "ZOMBIE" },
    // ZOMBIE_SQUASH_HEAD (30)
    ZombieDefinition { mZombieType: ZombieType::ZOMBIE_SQUASH_HEAD, mReanimationType: ReanimationType::REANIM_ZOMBIE, mZombieValue: 3, mStartingLevel: 99, mFirstAllowedWave: 10, mPickWeight: 2000, mZombieName: "ZOMBIE" },
    // ZOMBIE_TALLNUT_HEAD (31)
    ZombieDefinition { mZombieType: ZombieType::ZOMBIE_TALLNUT_HEAD, mReanimationType: ReanimationType::REANIM_ZOMBIE, mZombieValue: 4, mStartingLevel: 99, mFirstAllowedWave: 10, mPickWeight: 2000, mZombieName: "ZOMBIE" },
    // ZOMBIE_REDEYE_GARGANTUAR (32)
    ZombieDefinition { mZombieType: ZombieType::ZOMBIE_REDEYE_GARGANTUAR, mReanimationType: ReanimationType::REANIM_GARGANTUAR, mZombieValue: 10, mStartingLevel: 48, mFirstAllowedWave: 15, mPickWeight: 6000, mZombieName: "REDEYED_GARGANTUAR" },
];

pub fn GetZombieDefinition(theZombieType: ZombieType) -> &'static ZombieDefinition {
    unsafe { &G_ZOMBIE_DEFS[theZombieType as usize] }
}

// =========================================================================
// ★ Zombie 游戏逻辑核心方法
// =========================================================================

impl Zombie {
    /// 获取所属 Board 的可变引用
    unsafe fn board(&self) -> &'static mut super::board::Board {
        let ptr = self.base.m_board as *mut super::board::Board;
        debug_assert!(!ptr.is_null());
        &mut *ptr
    }

    /// 获取所属 LawnApp 的可变引用
    unsafe fn app(&self) -> &'static mut crate::lawn_app::LawnApp {
        &mut *(self.base.m_app as *mut crate::lawn_app::LawnApp)
    }

    /// C++ Zombie::ZombieInitialize (Zombie.cpp:115)
    /// 僵尸初始化 — 设置所有字段，加载动画等
    pub unsafe fn ZombieInitialize(&mut self, theRow: i32, theType: ZombieType, theVariant: bool, _theParentZombie: *mut Zombie, theFromWave: i32) {
        self.m_from_wave = theFromWave;
        self.base.m_row = theRow as i32;
        self.m_pos_x = 780.0 + crate::sexy_app_framework::common::rand_int() as f32 % ZOMBIE_START_RANDOM_OFFSET as f32;
        // [TODO]: mPosY = GetPosYBasedOnRow(theRow)
        self.m_vel_x = 0.0;
        self.m_vel_z = 0.0;
        self.base.m_width = 120;
        self.base.m_height = 120;
        self.m_frame = 0;
        self.m_prev_frame = 0;
        self.m_zombie_type = theType;
        self.m_variant = theVariant;
        self.m_is_eating = false;
        self.m_just_got_shot_counter = 0;
        self.m_shield_just_got_shot_counter = 0;
        self.m_shield_recoil_counter = 0;
        self.m_chilled_counter = 0;
        self.m_ice_trap_counter = 0;
        self.m_buttered_counter = 0;
        self.m_mind_controlled = false;
        self.m_blowing_away = false;
        self.m_has_head = true;
        self.m_has_arm = true;
        self.m_has_object = false;
        self.m_in_pool = false;
        self.m_on_high_ground = false;
        self.m_helm_type = 0; // HELMTYPE_NONE
        self.m_shield_type = ShieldType::SHIELDTYPE_NONE;
        self.m_yucky_face = false;
        self.m_yucky_face_counter = 0;
        self.m_anim_counter = 0;
        self.m_groan_counter = crate::sexy_tod_lib::tod_common::rand_range_int(300, 400);
        self.m_anim_ticks_per_frame = 12;
        self.m_anim_frames = 12;
        self.m_zombie_age = 0;
        self.m_target_col = -1;
        self.m_zombie_phase = ZombiePhase::PHASE_ZOMBIE_NORMAL;
        self.m_zombie_height = ZombieHeight::HEIGHT_ZOMBIE_NORMAL;
        self.m_phase_counter = 0;
        self.m_hit_umbrella = false;
        self.m_dropped_loot = false;
        self.m_related_zombie_id = ZombieID::ZOMBIEID_NULL;
        // [TODO]: zombie rect/attack rect based on type
        self.m_playing_song = false;
        self.m_zombie_fade = -1;
        self.m_flat_tires = false;
        self.m_scale_zombie = 1.0;
        self.m_use_ladder_col = -1;
        self.m_shield_health = 0;
        self.m_helm_health = 0;
        self.m_altitude = 0.0;
        self.m_flying_health = 0;
        self.m_original_anim_rate = 0.0;
        self.m_attachment_id = AttachmentID::ATTACHMENTID_NULL;
        self.m_summon_counter = 0;
        self.m_boss_stomp_counter = -1;
        self.m_boss_bungee_counter = -1;
        self.m_boss_head_counter = -1;
        self.m_body_reanim_id = ReanimationID::REANIMATIONID_NULL;
        self.m_target_plant_id = PlantID::PLANTID_NULL;
        self.m_boss_mode = 0;
        self.m_boss_fire_ball_reanim_id = ReanimationID::REANIMATIONID_NULL;
        self.m_special_head_reanim_id = ReanimationID::REANIMATIONID_NULL;
        self.m_target_row = -1;
        self.m_fireball_row = -1;
        self.m_is_fire_ball = false;
        self.m_mowered_reanim_id = ReanimationID::REANIMATIONID_NULL;
        self.m_zombatar_head_reanim_id = ReanimationID::REANIMATIONID_NULL;
        self.m_last_portal_x = -1;
        for i in 0..MAX_ZOMBIE_FOLLOWERS as usize {
            self.m_follower_zombie_id[i] = ZombieID::ZOMBIEID_NULL;
        }
        self.m_body_health = 270;
        // [TODO]: LoadReanim based on zombie type definition
    }

    pub unsafe fn IsDeadOrDying(&self) -> bool {
        self.m_dead
            || self.m_zombie_phase == ZombiePhase::PHASE_ZOMBIE_DYING
            || self.m_zombie_phase == ZombiePhase::PHASE_ZOMBIE_BURNED
            || self.m_zombie_phase == ZombiePhase::PHASE_ZOMBIE_MOWERED
    }

    pub unsafe fn IsOnBoard(&self) -> bool {
        !self.base.m_board.is_null()
    }

    /// C++ Zombie::Update() — 主更新循环 (lines 4270-4361)
    pub unsafe fn Update(&mut self) {
        // TOD_ASSERT(!mDead)
        self.m_zombie_age += 1;
        let mut do_update = false;

        let board = self.board();
        if (*self.app()).mGameScene as i32 == GameScenes::SCENE_LEVEL_INTRO as i32
            && self.m_zombie_type == ZombieType::ZOMBIE_BOSS
        {
            do_update = true;
        } else if self.IsOnBoard() && !board.mCutScene.is_null()
            && (*board.mCutScene).ShouldRunUpsellBoard()
        {
            do_update = true;
        } else if (*self.app()).mGameScene as i32 == GameScenes::SCENE_PLAYING as i32
            || !self.IsOnBoard()
            || self.m_from_wave == crate::lawn::zombie::ZOMBIE_WAVE_WINNER
        {
            do_update = true;
        }

        if do_update {
            if self.m_zombie_phase == ZombiePhase::PHASE_ZOMBIE_BURNED {
                self.UpdateBurn();
            } else if self.m_zombie_phase == ZombiePhase::PHASE_ZOMBIE_MOWERED {
                self.UpdateMowered();
            } else if self.m_zombie_phase == ZombiePhase::PHASE_ZOMBIE_DYING {
                self.UpdateDeath();
                self.UpdateZombieWalking();
            } else {
                if self.m_phase_counter > 0 && !self.IsImmobilizied() {
                    self.m_phase_counter -= 1;
                }

                if (*self.app()).mGameScene as i32 == GameScenes::SCENE_ZOMBIES_WON as i32 {
                    if !board.mCutScene.is_null() {
                        // if board->mCutScene->ShowZombieWalking()
                        self.UpdateZombieChimney();
                        self.UpdateZombieWalking();
                    }
                } else if self.IsOnBoard() {
                    self.UpdatePlaying();
                }

                // Zombie-type-specific updates
                if self.m_zombie_type == ZombieType::ZOMBIE_BUNGEE {
                    self.UpdateZombieBungee();
                }
                if self.m_zombie_type == ZombieType::ZOMBIE_POGO {
                    self.UpdateZombiePogo();
                }

                self.Animate();
            }

            if self.m_just_got_shot_counter > 0 {
                self.m_just_got_shot_counter -= 1;
            }
            if self.m_shield_just_got_shot_counter > 0 {
                self.m_shield_just_got_shot_counter -= 1;
            }
            if self.m_shield_recoil_counter > 0 {
                self.m_shield_recoil_counter -= 1;
            }
            if self.m_zombie_fade > 0 {
                self.m_zombie_fade -= 1;
                if self.m_zombie_fade == 0 {
                    self.DieNoLoot();
                }
            }

            self.base.m_x = self.m_pos_x as i32;
            self.base.m_y = self.m_pos_y as i32;

            // AttachmentUpdateAndMove(mAttachmentID, mPosX, mPosY);
            self.UpdateReanim();
        }
    }

    /// C++ Zombie::Draw() — 绘制 (Zombie.cpp:6264)
    pub unsafe fn Draw(&self, _g: &mut Graphics) {
        if self.m_zombie_height == ZombieHeight::HEIGHT_GETTING_BUNGEE_DROPPED {
            return;
        }

        // [TODO]: ZombieDrawPosition aDrawPos = GetDrawPos()
        let _aDrawPos = ZombieDrawPosition::new();
        let _board = self.board();

        if (*self.app()).mGameScene as i32 == GameScenes::SCENE_ZOMBIES_WON as i32 {
            // [TODO]: if !SetupDrawZombieWon(g) { return; }
        }

        // 冰陷阱后层
        if self.m_ice_trap_counter > 0 {
            // [TODO]: DrawIceTrap(g, aDrawPos, false);
        }

        // 主僵尸绘制（除隐形模式外）
        if (*self.app()).mGameMode as i32 != GameMode::GAMEMODE_CHALLENGE_INVISIGHOUL as i32
            || self.m_from_wave == ZOMBIE_WAVE_UI
        {
            if self.m_body_reanim_id != ReanimationID::REANIMATIONID_NULL {
                // [TODO]: DrawReanim(g, aDrawPos, RENDER_GROUP_NORMAL);
            } else {
                // [TODO]: DrawZombieWithParts(g, aDrawPos) — sprite-based fallback
            }
        }

        // 冰陷阱前层 + 黄油
        if self.m_ice_trap_counter > 0 {
            // [TODO]: DrawIceTrap(g, aDrawPos, true);
        }
        if self.m_buttered_counter > 0 {
            // [TODO]: DrawButter(g, aDrawPos);
        }

        // 附着物（粒子效果等）
        if self.m_attachment_id != AttachmentID::ATTACHMENTID_NULL {
            // [TODO]: AttachmentDraw(mAttachmentID, &particleGraphics, false);
        }

        // g->ClearClipRect();
    }

    // === Sub-update methods (stubs, to be filled in) ===

    /// C++ Zombie::UpdatePlaying (Zombie.cpp:4543)
    pub unsafe fn UpdatePlaying(&mut self) {
        self.m_groan_counter -= 1;
        let board = self.board();
        let a_zombies_count = board.mZombies.m_size;
        if self.m_groan_counter == 0
            && rand_int() % (a_zombies_count.max(1) as i32) == 0
            && self.m_has_head
            && self.m_zombie_type != ZombieType::ZOMBIE_BOSS
        // [TODO]: && !board.HasLevelAwardDropped()
        {
            let a_pitch = if (*self.app()).IsLittleTroubleLevel() {
                rand_float(10.0) + 40.0
            } else { 0.0 };

            if self.m_zombie_type == ZombieType::ZOMBIE_GARGANTUAR {
                self.app().PlayFoley(FoleyType::FOLEY_LOW_GROAN);
            } else if self.m_variant {
                self.app().PlayFoleyPitch(FoleyType::FOLEY_BRAINS, a_pitch);
            } else if (*self.app()).m_sukhbir_mode {
                self.app().PlayFoleyPitch(FoleyType::FOLEY_SUKHBIR, a_pitch);
            } else {
                self.app().PlayFoleyPitch(FoleyType::FOLEY_GROAN, a_pitch);
            }
            self.m_groan_counter = rand_int() % 1000 + 500;
        }

        // 冰/冻/黄油递减
        if self.m_ice_trap_counter > 0 {
            self.m_ice_trap_counter -= 1;
            if self.m_ice_trap_counter == 0 {
                // [TODO]: RemoveIceTrap(); AddAttachedParticle(...)
            }
        }
        if self.m_chilled_counter > 0 {
            self.m_chilled_counter -= 1;
            if self.m_chilled_counter == 0 {
                // [TODO]: UpdateAnimSpeed()
            }
        }
        if self.m_buttered_counter > 0 {
            self.m_buttered_counter -= 1;
            if self.m_buttered_counter == 0 {
                // [TODO]: RemoveButter()
            }
        }

        // 从墓碑升起
        if self.m_zombie_phase == ZombiePhase::PHASE_RISING_FROM_GRAVE {
            self.UpdateZombieRiseFromGrave();
            return;
        }

        // 位置/动作更新
        if !self.IsImmobilizied() {
            self.UpdateActions();
            self.UpdateZombiePosition();
            // [TODO]: CheckIfPreyCaught()
            // [TODO]: CheckForPool()
            // [TODO]: CheckForHighGround()
            // [TODO]: CheckForBoardEdge()
        }

        // Boss 特殊更新
        if self.m_zombie_type == ZombieType::ZOMBIE_BOSS {
            // [TODO]: UpdateBoss()
        }

        // 缓慢死亡逻辑
        if !self.IsDeadOrDying() && self.m_from_wave != ZOMBIE_WAVE_WINNER {
            let mut is_dying = !self.m_has_head;
            if self.m_zombie_type == ZombieType::ZOMBIE_ZAMBONI
                || self.m_zombie_type == ZombieType::ZOMBIE_CATAPULT
            {
                if self.m_body_health < 200 {
                    is_dying = true;
                }
            }

            if is_dying {
                let mut a_damage = 1;
                if self.m_zombie_type == ZombieType::ZOMBIE_YETI {
                    a_damage = 10;
                }
                if self.m_body_max_health >= 500 {
                    a_damage = 3;
                }

                if rand_int() % 5 == 0 {
                    // [TODO]: TakeDamage(a_damage, 9U)
                }
            }
        }
    }

    pub unsafe fn UpdateActions(&mut self) {
        // C++ Zombie::UpdateActions (Zombie.cpp:4395)
        // 高度/位置相关更新
        if self.m_zombie_height == ZombieHeight::HEIGHT_UP_LADDER {
            // [TODO]: UpdateClimbingLadder()
        }
        if self.m_zombie_height == ZombieHeight::HEIGHT_OUT_OF_POOL
            || self.m_zombie_height == ZombieHeight::HEIGHT_IN_TO_POOL
            || self.m_in_pool
        {
            self.UpdateZombiePool();
        }
        if self.m_zombie_height == ZombieHeight::HEIGHT_UP_TO_HIGH_GROUND
            || self.m_zombie_height == ZombieHeight::HEIGHT_DOWN_OFF_HIGH_GROUND
        {
            self.UpdateZombieHighGround();
        }
        if self.m_zombie_height == ZombieHeight::HEIGHT_FALLING {
            self.UpdateZombieFalling();
        }
        if self.m_zombie_height == ZombieHeight::HEIGHT_IN_TO_CHIMNEY {
            self.UpdateZombieChimney();
        }

        // 僵尸类型特定更新
        match self.m_zombie_type {
            ZombieType::ZOMBIE_POLEVAULTER => { self.UpdateZombiePolevaulter(); }
            ZombieType::ZOMBIE_CATAPULT => { self.UpdateZombieCatapult(); }
            ZombieType::ZOMBIE_DOLPHIN_RIDER => { self.UpdateZombieDolphinRider(); }
            ZombieType::ZOMBIE_SNORKEL => { self.UpdateZombieSnorkel(); }
            ZombieType::ZOMBIE_BALLOON => { self.UpdateZombieFlyer(); }
            ZombieType::ZOMBIE_NEWSPAPER => { self.UpdateZombieNewspaper(); }
            ZombieType::ZOMBIE_DIGGER => { self.UpdateZombieDigger(); }
            ZombieType::ZOMBIE_JACK_IN_THE_BOX => { self.UpdateZombieJackInTheBox(); }
            ZombieType::ZOMBIE_GARGANTUAR | ZombieType::ZOMBIE_REDEYE_GARGANTUAR => { self.UpdateZombieGargantuar(); }
            ZombieType::ZOMBIE_BOBSLED => { self.UpdateZombieBobsled(); }
            ZombieType::ZOMBIE_ZAMBONI => { self.UpdateZamboni(); }
            ZombieType::ZOMBIE_LADDER => { self.UpdateClimbingLadder(); }
            ZombieType::ZOMBIE_YETI => { self.UpdateYeti(); }
            ZombieType::ZOMBIE_DANCER => { self.UpdateZombieDancer(); }
            ZombieType::ZOMBIE_BUNGEE => { self.UpdateZombieBungee(); }
            ZombieType::ZOMBIE_POGO => { self.UpdateZombiePogo(); }
            _ => {}
        }

        // 地面僵尸行走（非特殊类型）
        if self.m_zombie_height == ZombieHeight::HEIGHT_ZOMBIE_NORMAL
            && self.m_zombie_type != ZombieType::ZOMBIE_BUNGEE
            && self.m_zombie_type != ZombieType::ZOMBIE_POGO
            && self.m_zombie_type != ZombieType::ZOMBIE_CATAPULT
        {
            self.UpdateZombieWalking();
        }
    }

    pub unsafe fn UpdateZombieWalking(&mut self) {
        // C++ Zombie::UpdateZombieWalking (Zombie.cpp:4071)
        // C++: if (ZombieNotWalking()) return; → mIsEating || IsImmobilizied()
        if self.m_is_eating || self.IsImmobilizied() {
            return;
        }

        let app = self.app();
        let a_body_reanim = app.ReanimationTryToGet(self.m_body_reanim_id);
        if !a_body_reanim.is_null() {
            let mut a_speed: f32;
            // C++: 特殊速度计算
            let is_pogo_bouncing = self.m_zombie_phase as i32 >= ZombiePhase::PHASE_POGO_BOUNCING as i32
                && self.m_zombie_phase as i32 <= ZombiePhase::PHASE_POGO_FORWARD_BOUNCE_7 as i32;
            let is_moving_chilled = self.m_chilled_counter > 0;
            let chilled_speed_factor: f32 = 0.5;

            if is_pogo_bouncing
                || self.m_zombie_phase == ZombiePhase::PHASE_BALLOON_FLYING
                || self.m_zombie_phase == ZombiePhase::PHASE_DOLPHIN_RIDING
                || self.m_zombie_phase == ZombiePhase::PHASE_SNORKEL_WALKING_IN_POOL
                || self.m_zombie_type == ZombieType::ZOMBIE_CATAPULT
            {
                a_speed = self.m_vel_x;
                if is_moving_chilled {
                    a_speed *= chilled_speed_factor;
                }
            } else if self.m_zombie_type == ZombieType::ZOMBIE_ZAMBONI
                || self.m_zombie_phase == ZombiePhase::PHASE_DIGGER_TUNNELING
                || self.m_zombie_phase == ZombiePhase::PHASE_DOLPHIN_IN_JUMP
                || self.m_zombie_phase == ZombiePhase::PHASE_POLEVAULTER_IN_VAULT
                || self.m_zombie_phase == ZombiePhase::PHASE_SNORKEL_INTO_POOL
            {
                a_speed = self.m_vel_x;
            } else {
                // C++: 默认速度 = 动画地面轨道速度 * 缩放
                // [TODO]: aBodyReanim->GetTrackVelocity("_ground") * mScaleZombie
                a_speed = self.m_vel_x;
                if is_moving_chilled {
                    a_speed *= CHILLED_SPEED_FACTOR;
                }
            }

            // C++: 行走方向
            let is_walking_backwards = self.m_mind_controlled;
            if is_walking_backwards || self.m_zombie_phase == ZombiePhase::PHASE_DANCER_DANCING_IN {
                self.m_pos_x += a_speed;
            } else {
                self.m_pos_x -= a_speed;
            }

            // C++: 橄榄球僵尸粒子效果
            if self.m_zombie_type == ZombieType::ZOMBIE_FOOTBALL {
                // [TODO]: ShouldTriggerTimedEvent 粒子效果
            }
            // C++: 撑杆跳僵尸粒子效果
            if self.m_zombie_phase == ZombiePhase::PHASE_POLEVAULTER_PRE_VAULT {
                // [TODO]: ShouldTriggerTimedEvent 粒子效果
            }
        } else {
            // C++: 无重动画时的行走逻辑（基于帧）
            let mut do_walk = false;
            if self.m_zombie_phase == ZombiePhase::PHASE_POLEVAULTER_IN_VAULT
                || self.m_zombie_phase == ZombiePhase::PHASE_DIGGER_TUNNELING
                || self.m_zombie_type == ZombieType::ZOMBIE_DANCER
                || self.m_zombie_type == ZombieType::ZOMBIE_BACKUP_DANCER
                || self.m_zombie_type == ZombieType::ZOMBIE_BOBSLED
                || self.m_zombie_type == ZombieType::ZOMBIE_POGO
                || self.m_zombie_type == ZombieType::ZOMBIE_DOLPHIN_RIDER
                || self.m_zombie_type == ZombieType::ZOMBIE_BALLOON
            {
                do_walk = true;
            } else if self.m_zombie_type == ZombieType::ZOMBIE_SNORKEL && self.m_in_pool {
                do_walk = true;
            } else if self.m_frame >= 0 && self.m_frame <= 2 {
                do_walk = true;
            } else if self.m_frame >= 6 && self.m_frame <= 8 {
                do_walk = true;
            }

            if do_walk {
                let mut a_speed = self.m_vel_x;
                if self.m_chilled_counter > 0 {
                    a_speed *= 0.5; // CHILLED_SPEED_FACTOR
                }
                if self.m_mind_controlled {
                    self.m_pos_x += a_speed;
                } else {
                    self.m_pos_x -= a_speed;
                }
            }
        }
    }

    pub unsafe fn UpdateZombiePosition(&mut self) {
        // C++ Zombie::UpdateZombiePosition (Zombie.cpp:4217)
        // 某些类型不更新位置
        if self.m_zombie_type == ZombieType::ZOMBIE_BUNGEE
            || self.m_zombie_type == ZombieType::ZOMBIE_BOSS
            || self.m_zombie_phase == ZombiePhase::PHASE_RISING_FROM_GRAVE
            || self.m_zombie_height == ZombieHeight::HEIGHT_ZOMBIQUARIUM
        {
            return;
        }

        // C++: 行走 + 压路（冰车/投石车）
        self.UpdateZombieWalking();
        // [TODO]: CheckForZombieStep()

        // C++: 被吹走
        if self.m_blowing_away {
            self.m_pos_x += 10.0;
            if self.m_pos_x > 850.0 {
                // [TODO]: DieWithLoot()
                return;
            }
        }

        // C++: Y 轴对齐到行位置
        if self.m_zombie_height == ZombieHeight::HEIGHT_ZOMBIE_NORMAL {
            let a_desired_y = 80.0 + self.base.m_row as f32 * 100.0; // [TODO]: GetPosYBasedOnRow
            if self.m_pos_y < a_desired_y {
                self.m_pos_y += (a_desired_y - self.m_pos_y).min(1.0);
            } else if self.m_pos_y > a_desired_y {
                self.m_pos_y -= (self.m_pos_y - a_desired_y).min(1.0);
            }
        }

        // 更新基类坐标
        self.base.m_x = self.m_pos_x as i32;
        self.base.m_y = self.m_pos_y as i32;
    }

    /// C++ Zombie::Animate (Zombie.cpp:4898) — 帧动画更新
    pub unsafe fn Animate(&mut self) {
        self.m_prev_frame = self.m_frame;
        // C++: 某些阶段不更新帧
        if self.m_zombie_phase == ZombiePhase::PHASE_JACK_IN_THE_BOX_POPPING
            || self.m_zombie_phase == ZombiePhase::PHASE_NEWSPAPER_MADDENING
            || self.m_zombie_phase == ZombiePhase::PHASE_DIGGER_RISING
            || self.m_zombie_phase == ZombiePhase::PHASE_DIGGER_TUNNELING_PAUSE_WITHOUT_AXE
            || self.m_zombie_phase == ZombiePhase::PHASE_DIGGER_RISE_WITHOUT_AXE
            || self.m_zombie_phase == ZombiePhase::PHASE_DIGGER_STUNNED
            || self.IsImmobilizied()
        {
            return;
        }

        self.m_anim_counter += 1;

        if self.m_yucky_face {
            self.UpdateYuckyFace();
        }

        if self.m_is_eating && self.m_has_head {
            let mut a_frame_length = 6;
            if self.m_chilled_counter > 0 {
                a_frame_length = 12;
            }
            if self.m_anim_counter >= self.m_anim_frames * a_frame_length {
                self.m_anim_counter = a_frame_length;
            }
            self.m_frame = self.m_anim_counter / a_frame_length;

            let a_body_reanim = self.app().ReanimationTryToGet(self.m_body_reanim_id) as *mut crate::sexy_tod_lib::reanimator::Reanimation;
            if !a_body_reanim.is_null() {
                // C++: 基于僵尸类型设置啃食动画时间事件参数
                let (a_left_hand_time, a_right_hand_time) = match self.m_zombie_type {
                    ZombieType::ZOMBIE_POLEVAULTER => (0.38, 0.8),
                    ZombieType::ZOMBIE_NEWSPAPER | ZombieType::ZOMBIE_LADDER => (0.42, 0.42),
                    ZombieType::ZOMBIE_JACK_IN_THE_BOX => (0.53, 0.53),
                    ZombieType::ZOMBIE_BOBSLED => (0.33, 0.83),
                    ZombieType::ZOMBIE_IMP => (0.33, 0.79),
                    _ => (0.14, 0.68),
                };
                // [TODO]: ShouldTriggerTimedEvent 方法尚未在 Reanimation 上实现
                // if (*a_body_reanim).ShouldTriggerTimedEvent(a_left_hand_time)
                //     || (*a_body_reanim).ShouldTriggerTimedEvent(a_right_hand_time)
                {
                    self.AnimateChewSound();
                    self.AnimateChewEffect();
                }
            } else {
                if self.m_anim_counter == 4 * a_frame_length {
                    self.AnimateChewSound();
                }
                if self.m_anim_counter == 7 * a_frame_length && !self.m_mind_controlled {
                    self.AnimateChewEffect();
                }
            }
        } else {
            if self.m_anim_counter >= self.m_anim_frames * self.m_anim_ticks_per_frame {
                self.m_anim_counter = 0;
            }
            self.m_frame = self.m_anim_counter / self.m_anim_ticks_per_frame;
        }
    }

    // =========================================================================
    // ★ 僵尸啃食/攻击系统 (C++ 保真翻译)
    // =========================================================================

    /// C++ Zombie::StartEating (Zombie.cpp:6685) — 开始啃食植物
    pub unsafe fn StartEating(&mut self) {
        if self.m_is_eating { return; }
        self.m_is_eating = true;

        // C++: 挖掘僵尸不停止挖掘
        if self.m_zombie_phase == ZombiePhase::PHASE_DIGGER_TUNNELING {
            return;
        }

        // C++: 播放啃食动画
        if self.m_zombie_phase == ZombiePhase::PHASE_LADDER_CARRYING {
            // [TODO]: PlayZombieReanim("anim_laddereat", REANIM_LOOP, 20, 0.0f)
        } else if self.m_zombie_phase == ZombiePhase::PHASE_NEWSPAPER_MAD {
            // [TODO]: PlayZombieReanim("anim_eat_nopaper", REANIM_LOOP, 20, 0.0f)
        } else {
            // [TODO]: PlayZombieReanim("anim_eat", REANIM_LOOP, 20, 0.0f)
            // C++: 门板僵尸隐藏门板手臂
            if self.m_shield_type == ShieldType::SHIELDTYPE_DOOR {
                // [TODO]: ShowDoorArms(false)
            }
        }
    }

    /// C++ Zombie::StartWalkAnim (Zombie.cpp:6717) — 恢复行走动画
    pub unsafe fn StartWalkAnim(&mut self, the_blend_time: i32) {
        self.PickRandomSpeed();
        // [TODO]: 调用 PlayZombieReanim 根据僵尸类型播放行走动画
        // C++ 原逻辑: 根据僵尸类型和阶段选择 "anim_walk" / "anim_run" / "anim_dig" 等
    }

    /// C++ Zombie::UpdateBurn (Zombie.cpp:4261) — 燃烧死亡计时器
    pub unsafe fn UpdateBurn(&mut self) {
        self.m_phase_counter -= 1;
        if self.m_phase_counter == 0 {
            self.DieWithLoot();
        }
    }

    /// C++ Zombie::UpdateDeath (Zombie.cpp:9038) — 死亡动画帧
    pub unsafe fn UpdateDeath(&mut self) {
        let app = self.app();
        let a_body_reanim = app.ReanimationTryToGet(self.m_body_reanim_id);
        if a_body_reanim.is_null() {
            self.DieNoLoot();
            return;
        }

        // C++: 坠落中更新
        if self.m_zombie_height == ZombieHeight::HEIGHT_FALLING {
            self.UpdateZombieFalling();
        }

        // C++: 巨人倒地地震
        if self.m_zombie_type == ZombieType::ZOMBIE_GARGANTUAR
            || self.m_zombie_type == ZombieType::ZOMBIE_REDEYE_GARGANTUAR
        {
            // [TODO]: ShouldTriggerTimedEvent → 震动
        }

        // C++: 倒地音效
        if !self.m_in_pool {
            // [TODO]: 根据僵尸类型在特定动画帧播放倒地音效
        }

        // C++: Boss 特殊死亡效果
        if self.m_zombie_type == ZombieType::ZOMBIE_BOSS {
            // [TODO]: 爆炸粒子效果
        }

        // C++: 动画循环结束 → 掉落并真正死亡
        // [TODO]: aBodyReanim->mLoopCount > 0 → DropLoot()
    }

    /// C++ Zombie::UpdateMowered (Zombie.cpp:9255) — 割草机碾压
    pub unsafe fn UpdateMowered(&mut self) {
        let app = self.app();
        let a_mowered_reanim = app.ReanimationTryToGet(self.m_mowered_reanim_id);

        // C++: 如果被碾压动画存在则播放
        if a_mowered_reanim.is_null() {
            // 没有碾压动画 → 直接死亡
            self.m_phase_counter -= 1;
            if self.m_phase_counter <= 0 {
                self.DieNoLoot();
            }
        } else {
            // [TODO]: 碾压动画循环结束 → DieNoLoot()
        }
    }
    pub unsafe fn UpdateZombiePool(&mut self) {
        // C++ Zombie::UpdateZombiePool (Zombie.cpp:3230)
        if self.m_zombie_height == ZombieHeight::HEIGHT_OUT_OF_POOL {
            self.m_altitude += 1.0;
            if self.m_zombie_type == ZombieType::ZOMBIE_SNORKEL {
                self.m_altitude += 1.0;
            }
            if self.m_altitude >= 0.0 {
                self.m_altitude = 0.0;
                self.m_zombie_height = ZombieHeight::HEIGHT_ZOMBIE_NORMAL;
                self.m_in_pool = false;
            }
        } else if self.m_zombie_height == ZombieHeight::HEIGHT_IN_TO_POOL {
            self.m_altitude -= 1.0;
            let a_depth = -40.0 * self.m_scale_zombie;
            if self.m_altitude <= a_depth {
                self.m_altitude = a_depth;
                self.m_zombie_height = ZombieHeight::HEIGHT_ZOMBIE_NORMAL;
                self.StartWalkAnim(0);
            }
        } else if self.m_zombie_height == ZombieHeight::HEIGHT_DRAGGED_UNDER {
            self.m_altitude -= 1.0;
        }
    }

    pub unsafe fn UpdateZombieHighGround(&mut self) {
        // C++ Zombie::UpdateZombieHighGround (Zombie.cpp:3264)
        if self.m_zombie_type == ZombieType::ZOMBIE_POGO { return; }

        if self.m_zombie_height == ZombieHeight::HEIGHT_UP_TO_HIGH_GROUND {
            self.m_altitude += 1.0;
            if self.m_altitude >= HIGH_GROUND_HEIGHT as f32 {
                self.m_altitude = HIGH_GROUND_HEIGHT as f32;
                self.m_zombie_height = ZombieHeight::HEIGHT_ZOMBIE_NORMAL;
            }
        } else if self.m_zombie_height == ZombieHeight::HEIGHT_DOWN_OFF_HIGH_GROUND {
            self.m_altitude -= 1.0;
            if self.m_altitude <= 0.0 {
                self.m_altitude = 0.0;
                self.m_zombie_height = ZombieHeight::HEIGHT_ZOMBIE_NORMAL;
                self.m_on_high_ground = false;
            }
        }
    }

    pub unsafe fn UpdateZombieFalling(&mut self) {
        // C++ Zombie::UpdateZombieFalling (Zombie.cpp:3290)
        self.m_altitude -= 1.0;
        if self.m_zombie_phase == ZombiePhase::PHASE_POLEVAULTER_PRE_VAULT {
            self.m_altitude -= 1.0;
        }
        let mut a_ground_height = 0.0;
        if self.m_on_high_ground {
            a_ground_height = HIGH_GROUND_HEIGHT as f32;
        }
        if self.m_altitude <= a_ground_height {
            self.m_altitude = a_ground_height;
            self.m_zombie_height = ZombieHeight::HEIGHT_ZOMBIE_NORMAL;
        }
    }

    pub unsafe fn UpdateZombieChimney(&mut self) {
        // C++ Zombie::UpdateZombieChimney (Zombie.cpp:9596)
        let board = self.board();
        if board.mBackground as i32 == BackgroundType::BACKGROUND_5_ROOF as i32
            || board.mBackground as i32 == BackgroundType::BACKGROUND_6_BOSS as i32
        {
            // C++: mAltitude = TodAnimateCurve(4000, 5000, mCutScene->mCutsceneTime, 200, 0, CURVE_EASE_IN)
            // [TODO]: 使用正确的 cutscene time
            self.m_altitude = crate::sexy_tod_lib::tod_common::tod_animate_curve_float(
                4000, 5000,
                if board.mCutScene.is_null() { 0 } else { (*board.mCutScene).mCutsceneTime },
                200.0, 0.0, TodCurves::CURVE_EASE_IN
            );
        }
    }

    pub unsafe fn UpdateZombieBungee(&mut self) {
        // C++ Zombie::UpdateZombieBungee (Zombie.cpp:1307)
        if self.IsDeadOrDying() || self.IsImmobilizied() { return; }

        if self.m_zombie_phase == ZombiePhase::PHASE_BUNGEE_DIVING
            || self.m_zombie_phase == ZombiePhase::PHASE_BUNGEE_DIVING_SCREAMING
        {
            let a_old_altitude = self.m_altitude;
            self.m_altitude -= 8.0;
            // [TODO]: BungeeLanding()
        } else if self.m_zombie_phase == ZombiePhase::PHASE_BUNGEE_AT_BOTTOM {
            if self.m_phase_counter <= 0 {
                // [TODO]: BungeeStealTarget()
                self.m_zombie_phase = ZombiePhase::PHASE_BUNGEE_GRABBING;
            }
        } else if self.m_zombie_phase == ZombiePhase::PHASE_BUNGEE_GRABBING {
            // [TODO]: 检测动画循环 → BungeeLiftTarget() → PHASE_BUNGEE_RISING
            self.m_zombie_phase = ZombiePhase::PHASE_BUNGEE_RISING;
        } else if self.m_zombie_phase == ZombiePhase::PHASE_BUNGEE_HIT_OUCHY {
            if self.m_phase_counter <= 0 {
                // [TODO]: DieWithLoot()
            }
        } else if self.m_zombie_phase == ZombiePhase::PHASE_BUNGEE_RISING {
            self.m_altitude += 8.0;
            if self.m_altitude >= 600.0 {
                // [TODO]: DieNoLoot()
            }
        } else if self.m_zombie_phase == ZombiePhase::PHASE_BUNGEE_CUTSCENE {
            // [TODO]: 振荡动画
        }

        self.base.m_x = self.m_pos_x as i32;
        self.base.m_y = self.m_pos_y as i32;
    }

    pub unsafe fn UpdateZombiePogo(&mut self) {
        // C++ Zombie::UpdateZombiePogo (Zombie.cpp:1401)
        // [TODO]: Pogo 僵尸弹跳逻辑 — 检测植物、弹跳越过
        self.UpdateZombieWalking();
    }

    pub unsafe fn UpdateReanim(&mut self) { self.UpdateReanimFull(); }
    pub unsafe fn UpdateYuckyFace(&mut self) {
        self.m_yucky_face_counter -= 1;
        if self.m_yucky_face_counter <= 0 { self.m_yucky_face = false; }
    }
    pub unsafe fn UpdateAnimSpeed(&mut self) {
        // C++: if (!IsOnBoard()) return;
        if !self.IsOnBoard() {
            return;
        }

        // C++: Reanimation* aBodyReanim = mApp->ReanimationTryToGet(mBodyReanimID);
        // C++: if (aBodyReanim == nullptr) return;
        // [TODO]: Reanimation 获取（mBodyReanimID）

        // C++: if (IsImmobilizied() || (mYuckyFace && mYuckyFaceCounter < 170)) { ApplyAnimRate(0.0f); return; }
        if self.IsImmobilizied() || (self.m_yucky_face && self.m_yucky_face_counter < 170) {
            self.ApplyAnimRate(0.0);
            return;
        }

        // C++: 潜泳进食/返回状态、死亡 → 原始动画速率
        if self.m_zombie_phase == ZombiePhase::PHASE_SNORKEL_UP_TO_EAT
            || self.m_zombie_phase == ZombiePhase::PHASE_SNORKEL_DOWN_FROM_EAT
            || self.IsDeadOrDying()
        {
            self.ApplyAnimRate(self.m_original_anim_rate);
            return;
        }

        // C++: 进食状态：特定僵尸类型 20，其余 36
        if self.m_is_eating {
            if self.m_zombie_type == ZombieType::ZOMBIE_POLEVAULTER
                || self.m_zombie_type == ZombieType::ZOMBIE_BALLOON
                || self.m_zombie_type == ZombieType::ZOMBIE_IMP
                || self.m_zombie_type == ZombieType::ZOMBIE_DIGGER
                || self.m_zombie_type == ZombieType::ZOMBIE_JACK_IN_THE_BOX
                || self.m_zombie_type == ZombieType::ZOMBIE_SNORKEL
                || self.m_zombie_type == ZombieType::ZOMBIE_YETI
            {
                self.ApplyAnimRate(20.0);
            } else {
                self.ApplyAnimRate(36.0);
            }
        } else {
            // C++: 非行走/特殊状态 → 原始动画速率
            if self.ZombieNotWalking()
                || self.IsBobsledTeamWithSled()
                || self.m_zombie_type == ZombieType::ZOMBIE_CATAPULT
                || self.m_zombie_phase == ZombiePhase::PHASE_DOLPHIN_RIDING
                || self.m_zombie_phase == ZombiePhase::PHASE_SNORKEL_WALKING_IN_POOL
            {
                self.ApplyAnimRate(self.m_original_anim_rate);
            } else {
                // C++: else if (aBodyReanim->TrackExists("_ground")) — 根据 _ground 轨道
                // 帧位移计算动画速率: aAnimRate = mVelX * aOneOverSpeed * 47.0f / mScaleZombie
                // [TODO]: 依赖 Reanimation 定义/帧数据（mFrameStart/mFrameCount/mTransforms），
                // 待 Reanimation 系统翻译后实现
                // self.ApplyAnimRate(self.m_vel_x * a_one_over_speed * 47.0 / self.m_scale_zombie);
            }
        }
    }

    // =========================================================================
    // ★ 僵尸伤害系统 (C++ 保真翻译)
    // =========================================================================

    /// C++ Zombie::TakeDamage (Zombie.cpp:7940) — 主伤害入口
    pub unsafe fn TakeDamage(&mut self, the_damage: i32, the_damage_flags: u32) {
        if self.m_zombie_phase == ZombiePhase::PHASE_JACK_IN_THE_BOX_POPPING || self.IsDeadOrDying() {
            return;
        }
        let mut a_damage_remaining = the_damage;
        if self.IsFlying() {
            a_damage_remaining = self.TakeFlyingDamage(a_damage_remaining, the_damage_flags);
        }
        if a_damage_remaining > 0
            && self.m_shield_type != ShieldType::SHIELDTYPE_NONE
            && (the_damage_flags & (1 << DamageFlags::DAMAGE_BYPASSES_SHIELD as i32)) == 0
        {
            a_damage_remaining = self.TakeShieldDamage(a_damage_remaining, the_damage_flags);
            if (the_damage_flags & (1 << DamageFlags::DAMAGE_HITS_SHIELD_AND_BODY as i32)) != 0 {
                a_damage_remaining = the_damage;
            }
        }
        if a_damage_remaining > 0 && self.m_helm_type != HelmType::HELMTYPE_NONE as i32 {
            a_damage_remaining = self.TakeHelmDamage(a_damage_remaining, the_damage_flags);
        }
        if a_damage_remaining > 0 {
            self.TakeBodyDamage(a_damage_remaining, the_damage_flags);
        }
    }

    pub unsafe fn TakeBodyDamage(&mut self, the_damage: i32, the_damage_flags: u32) {
        // C++: if (!TestBit(flags, DAMAGE_DOESNT_CAUSE_FLASH)) mJustGotShotCounter = 25;
        if (the_damage_flags & (1 << DamageFlags::DAMAGE_DOESNT_CAUSE_FLASH as i32)) == 0 {
            self.m_just_got_shot_counter = 25;
        }

        // C++: if (TestBit(flags, DAMAGE_FREEZE)) ApplyChill(false);
        if (the_damage_flags & (1 << DamageFlags::DAMAGE_FREEZE as i32)) != 0 {
            self.ApplyChill(false);
        }

        // C++: int aBodyHealthOrigin = mBodyHealth;
        let a_body_health_origin = self.m_body_health;
        // C++: int aDamageIndexBeforeDamage = GetBodyDamageIndex();
        let a_damage_index_before_damage = self.GetBodyDamageIndex();
        self.m_body_health -= the_damage;
        let a_damage_index_after_damage = self.GetBodyDamageIndex();

        if self.m_zombie_type == ZombieType::ZOMBIE_ZAMBONI {
            // C++: Reanimation* aBodyReanim = mApp->ReanimationGet(mBodyReanimID);
            // C++: if (!TestBit(flags, DAMAGE_DOESNT_CAUSE_FLASH)) mApp->PlayFoley(FOLEY_SHIELD_HIT);
            if (the_damage_flags & (1 << DamageFlags::DAMAGE_DOESNT_CAUSE_FLASH as i32)) == 0 {
                // [TODO]: mApp->PlayFoley(FOLEY_SHIELD_HIT)
            }
            if (the_damage_flags & (1 << DamageFlags::DAMAGE_SPIKE as i32)) != 0 {
                // [TODO]: SetImageOverride("Zombie_zamboni_1/2", DAMAGE2)
                self.ZamboniDeath(the_damage_flags);
            } else if self.m_body_health <= 0 {
                self.ZamboniDeath(the_damage_flags);
            } else if a_damage_index_before_damage != a_damage_index_after_damage {
                if a_damage_index_after_damage == 1 {
                    // [TODO]: SetImageOverride("Zombie_zamboni_1/2", DAMAGE1)
                } else if a_damage_index_after_damage == 2 {
                    // [TODO]: SetImageOverride(...DAMAGE2); AddAttachedParticle(27, 72, PARTICLE_ZAMBONI_SMOKE)
                }
            }
        } else if self.m_zombie_type == ZombieType::ZOMBIE_CATAPULT {
            if (the_damage_flags & (1 << DamageFlags::DAMAGE_SPIKE as i32)) != 0 || self.m_body_health <= 0 {
                // [TODO]: SetImageOverride("Zombie_catapult_siding", DAMAGE)
                self.CatapultDeath(the_damage_flags);
            } else if a_damage_index_before_damage != a_damage_index_after_damage {
                if a_damage_index_after_damage == 1 {
                    // [TODO]: SetImageOverride("Zombie_catapult_siding", DAMAGE)
                } else if a_damage_index_after_damage == 2 {
                    // [TODO]: AddAttachedParticle(47, 77, PARTICLE_ZAMBONI_SMOKE)
                }
            }
        } else if self.m_zombie_type == ZombieType::ZOMBIE_GARGANTUAR
            || self.m_zombie_type == ZombieType::ZOMBIE_REDEYE_GARGANTUAR
        {
            if a_damage_index_before_damage != a_damage_index_after_damage {
                if a_damage_index_after_damage == 1 {
                    // [TODO]: SetImageOverride 伽刚特尔身体/手臂损伤贴图
                } else if a_damage_index_after_damage == 2 {
                    // [TODO]: SetImageOverride 伽刚特尔身体/腿/手臂/头损伤贴图（红眼特殊头）
                }
            }
        } else if self.m_zombie_type == ZombieType::ZOMBIE_BOSS {
            if (the_damage_flags & (1 << DamageFlags::DAMAGE_DOESNT_CAUSE_FLASH as i32)) == 0 {
                // [TODO]: mApp->PlayFoley(FOLEY_SHIELD_HIT)
            }
            if a_damage_index_before_damage != a_damage_index_after_damage {
                if a_damage_index_after_damage == 1 {
                    // [TODO]: SetImageOverride Boss 损伤贴图 1
                } else if a_damage_index_after_damage == 2 {
                    // [TODO]: SetImageOverride Boss 损伤贴图 2; ApplyBossSmokeParticles(true)
                }
            }
            // C++: BOSS_FLASH_HEALTH_FRACTION 血量阈值触发爆炸粒子
            if a_body_health_origin >= self.m_body_max_health / 5 && self.m_body_health < self.m_body_max_health / 5 {
                // [TODO]: AddTodParticle(770, 260, PARTICLE_BOSS_EXPLOSION); PlayFoley(FOLEY_BOSS_EXPLOSION_SMALL); ApplyBossSmokeParticles(true)
            }
            // C++: if (mBodyHealth <= 0) mBodyHealth = 1; (Boss 不会因常规伤害死亡)
            if self.m_body_health <= 0 {
                self.m_body_health = 1;
            }
        } else {
            // C++: UpdateDamageStates(theDamageFlags);
            self.UpdateDamageStates(the_damage_flags);
        }

        // C++: if (mBodyHealth <= 0) { mBodyHealth = 0; PlayDeathAnim(theDamageFlags); DropLoot(); }
        if self.m_body_health <= 0 {
            self.m_body_health = 0;
            self.PlayDeathAnim(the_damage_flags);
            self.DropLoot();
        }
    }

    /// C++ Zombie::TakeFlyingDamage (Zombie.cpp:7773) — 飞行僵尸伤害（气球/飞行器）
    pub unsafe fn TakeFlyingDamage(&mut self, the_damage: i32, the_damage_flags: u32) -> i32 {
        // C++: if (!TestBit(flags, DAMAGE_DOESNT_CAUSE_FLASH)) mJustGotShotCounter = 25;
        if (the_damage_flags & (1 << DamageFlags::DAMAGE_DOESNT_CAUSE_FLASH as i32)) == 0 {
            self.m_just_got_shot_counter = 25;
        }

        // C++: int aDamageActual = std::min(mFlyingHealth, theDamage);
        let a_damage_actual = self.m_flying_health.min(the_damage);
        // C++: int aDamageRemaining = theDamage - aDamageActual;
        let a_damage_remaining = the_damage - a_damage_actual;
        self.m_flying_health -= a_damage_actual;
        // C++: if (mFlyingHealth == 0) LandFlyer(theDamageFlags);
        if self.m_flying_health == 0 {
            self.LandFlyer(the_damage_flags);
        }

        a_damage_remaining
    }

    /// C++ Zombie::TakeShieldDamage (Zombie.cpp:7570)
    pub unsafe fn TakeShieldDamage(&mut self, the_damage: i32, the_damage_flags: u32) -> i32 {
        // C++: if (!TestBit(flags, DAMAGE_DOESNT_CAUSE_FLASH)) { mShieldJustGotShotCounter = 25; mJustGotShotCounter = std::max(mJustGotShotCounter, 0); }
        if (the_damage_flags & (1 << DamageFlags::DAMAGE_DOESNT_CAUSE_FLASH as i32)) == 0 {
            self.m_shield_just_got_shot_counter = 25;
            self.m_just_got_shot_counter = self.m_just_got_shot_counter.max(0);
        }

        // C++: if (!TestBit(flags, DAMAGE_DOESNT_CAUSE_FLASH) && !TestBit(flags, DAMAGE_HITS_SHIELD_AND_BODY)) { mShieldRecoilCounter = 12; ... }
        if (the_damage_flags & (1 << DamageFlags::DAMAGE_DOESNT_CAUSE_FLASH as i32)) == 0
            && (the_damage_flags & (1 << DamageFlags::DAMAGE_HITS_SHIELD_AND_BODY as i32)) == 0
        {
            self.m_shield_recoil_counter = 12;
            if self.m_shield_type == ShieldType::SHIELDTYPE_DOOR
                || self.m_shield_type == ShieldType::SHIELDTYPE_LADDER
            {
                // [TODO]: mApp->PlayFoley(FOLEY_SHIELD_HIT)
            }
        }

        // C++: int aDamageIndexBeforeDamage = GetShieldDamageIndex();
        let a_damage_index_before_damage = self.GetShieldDamageIndex();
        // C++: int aDamageActual = std::min(mShieldHealth, theDamage);
        let a_damage_actual = self.m_shield_health.min(the_damage);
        // C++: int aDamageRemaining = theDamage - aDamageActual;
        let a_damage_remaining = the_damage - a_damage_actual;
        self.m_shield_health -= a_damage_actual;
        // C++: if (mShieldHealth == 0) { DropShield(theDamageFlags); return aDamageRemaining; }
        if self.m_shield_health == 0 {
            self.DropShield(the_damage_flags);
            return a_damage_remaining;
        }

        // C++: int aDamageIndexAfterDamage = GetShieldDamageIndex();
        let a_damage_index_after_damage = self.GetShieldDamageIndex();
        if a_damage_index_after_damage != a_damage_index_before_damage {
            if self.m_shield_type == ShieldType::SHIELDTYPE_DOOR && a_damage_index_after_damage == 1 {
                // [TODO]: SetImageOverride("anim_screendoor", IMAGE_REANIM_ZOMBIE_SCREENDOOR2)
            } else if self.m_shield_type == ShieldType::SHIELDTYPE_DOOR && a_damage_index_after_damage == 2 {
                // [TODO]: SetImageOverride("anim_screendoor", IMAGE_REANIM_ZOMBIE_SCREENDOOR3)
            } else if self.m_shield_type == ShieldType::SHIELDTYPE_NEWSPAPER && a_damage_index_after_damage == 1 {
                // [TODO]: SetImageOverride("Zombie_paper_paper", IMAGE_REANIM_ZOMBIE_PAPER_PAPER2)
            } else if self.m_shield_type == ShieldType::SHIELDTYPE_NEWSPAPER && a_damage_index_after_damage == 2 {
                // [TODO]: SetImageOverride("Zombie_paper_paper", IMAGE_REANIM_ZOMBIE_PAPER_PAPER3)
            } else if self.m_shield_type == ShieldType::SHIELDTYPE_LADDER && a_damage_index_after_damage == 1 {
                // [TODO]: SetImageOverride("Zombie_ladder_1", IMAGE_REANIM_ZOMBIE_LADDER_1_DAMAGE1)
            } else if self.m_shield_type == ShieldType::SHIELDTYPE_LADDER && a_damage_index_after_damage == 2 {
                // [TODO]: SetImageOverride("Zombie_ladder_1", IMAGE_REANIM_ZOMBIE_LADDER_1_DAMAGE2)
            }
        }

        a_damage_remaining
    }

    pub unsafe fn TakeHelmDamage(&mut self, the_damage: i32, the_damage_flags: u32) -> i32 {
        // C++: if (!TestBit(flags, DAMAGE_DOESNT_CAUSE_FLASH)) mJustGotShotCounter = 25;
        if (the_damage_flags & (1 << DamageFlags::DAMAGE_DOESNT_CAUSE_FLASH as i32)) == 0 {
            self.m_just_got_shot_counter = 25;
        }

        // C++: int aDamageIndexBeforeDamage = GetHelmDamageIndex();
        let a_damage_index_before_damage = self.GetHelmDamageIndex();
        // C++: int aDamageActual = std::min(mHelmHealth, theDamage);
        let a_damage_actual = self.m_helm_health.min(the_damage);
        // C++: int aDamageRemaining = theDamage - aDamageActual;
        let a_damage_remaining = the_damage - a_damage_actual;
        self.m_helm_health -= a_damage_actual;
        // C++: if (TestBit(flags, DAMAGE_FREEZE)) ApplyChill(false);
        if (the_damage_flags & (1 << DamageFlags::DAMAGE_FREEZE as i32)) != 0 {
            self.ApplyChill(false);
        }
        // C++: if (mHelmHealth == 0) { DropHelm(theDamageFlags); return aDamageRemaining; }
        if self.m_helm_health == 0 {
            self.DropHelm(the_damage_flags);
            return a_damage_remaining;
        }

        // C++: int aDamageIndexAfterDamage = GetHelmDamageIndex();
        let a_damage_index_after_damage = self.GetHelmDamageIndex();
        if a_damage_index_before_damage != a_damage_index_after_damage {
            // C++: 各种头盔类型的损伤贴图切换（SetImageOverride）
            // [TODO]: TRAFFIC_CONE / PAIL / DIGGER / FOOTBALL / WALLNUT / TALLNUT 的损伤贴图覆盖
            let _ = self.m_helm_type;
            let _ = a_damage_index_after_damage;
        }

        a_damage_remaining
    }


    /// C++ Zombie::EffectedByDamage (Zombie.cpp:8051) — 判断僵尸是否受该伤害范围影响
    pub unsafe fn EffectedByDamage(&self, the_damage_range_flags: u32) -> bool {
        // C++: if (!TestBit(flags, DAMAGES_DYING) && IsDeadOrDying()) return false;
        if (the_damage_range_flags & (1 << DamageRangeFlags::DAMAGES_DYING as i32)) == 0
            && self.IsDeadOrDying()
        {
            return false;
        }

        // C++: 心智控制相关
        if (the_damage_range_flags & (1 << DamageRangeFlags::DAMAGES_ONLY_MINDCONTROLLED as i32)) != 0 {
            if !self.m_mind_controlled {
                return false;
            }
        } else if self.m_mind_controlled {
            return false;
        }

        // C++: 蹦极僵尸只有在停留/抓取时才受攻击
        if self.m_zombie_type == ZombieType::ZOMBIE_BUNGEE
            && self.m_zombie_phase != ZombiePhase::PHASE_BUNGEE_AT_BOTTOM
            && self.m_zombie_phase != ZombiePhase::PHASE_BUNGEE_GRABBING
        {
            return false;
        }

        // C++: 被空投过程中不受攻击
        if self.m_zombie_height == ZombieHeight::HEIGHT_GETTING_BUNGEE_DROPPED {
            return false;
        }

        // C++: Boss 只有在低头状态下受攻击
        if self.m_zombie_type == ZombieType::ZOMBIE_BOSS {
            if self.m_zombie_phase != ZombiePhase::PHASE_BOSS_HEAD_IDLE_BEFORE_SPIT
                && self.m_zombie_phase != ZombiePhase::PHASE_BOSS_HEAD_IDLE_AFTER_SPIT
                && self.m_zombie_phase != ZombiePhase::PHASE_BOSS_HEAD_SPIT
            {
                return false;
            }
            // [TODO]: Reanimation mAnimTime 检查（PHASE_BOSS_HEAD_ENTER/LEAVE 阶段）
        }

        // C++: 存在雪橇时，只有领头僵尸受攻击
        if self.m_zombie_type == ZombieType::ZOMBIE_BOBSLED && self.GetBobsledPosition() > 0 {
            return false;
        }

        // C++: 特殊阶段（撑杆跳/小鬼投掷/挖地/海豚/潜泳/气球/出土/舞者）— 仅受 DAMAGES_OFF_GROUND
        if self.m_zombie_phase == ZombiePhase::PHASE_POLEVAULTER_IN_VAULT
            || self.m_zombie_phase == ZombiePhase::PHASE_IMP_GETTING_THROWN
            || self.m_zombie_phase == ZombiePhase::PHASE_DIGGER_RISING
            || self.m_zombie_phase == ZombiePhase::PHASE_DIGGER_TUNNELING_PAUSE_WITHOUT_AXE
            || self.m_zombie_phase == ZombiePhase::PHASE_DIGGER_RISE_WITHOUT_AXE
            || self.m_zombie_phase == ZombiePhase::PHASE_DOLPHIN_INTO_POOL
            || self.m_zombie_phase == ZombiePhase::PHASE_DOLPHIN_IN_JUMP
            || self.m_zombie_phase == ZombiePhase::PHASE_SNORKEL_INTO_POOL
            || self.m_zombie_phase == ZombiePhase::PHASE_BALLOON_POPPING
            || self.m_zombie_phase == ZombiePhase::PHASE_RISING_FROM_GRAVE
            || self.m_zombie_phase == ZombiePhase::PHASE_BOBSLED_CRASHING
            || self.m_zombie_phase == ZombiePhase::PHASE_DANCER_RISING
        {
            return (the_damage_range_flags & (1 << DamageRangeFlags::DAMAGES_OFF_GROUND as i32)) != 0;
        }

        // C++: 除雪橇小队外，场外僵尸不受攻击
        if self.m_zombie_type != ZombieType::ZOMBIE_BOBSLED
            && self.GetZombieRect().m_x > 800 // C++: WIDE_BOARD_WIDTH
        {
            return false;
        }

        let submerged = self.m_zombie_type == ZombieType::ZOMBIE_SNORKEL && self.m_in_pool && !self.m_is_eating;
        if (the_damage_range_flags & (1 << DamageRangeFlags::DAMAGES_SUBMERGED as i32)) != 0 && submerged {
            return true;
        }

        let underground = self.m_zombie_phase == ZombiePhase::PHASE_DIGGER_TUNNELING;
        if (the_damage_range_flags & (1 << DamageRangeFlags::DAMAGES_UNDERGROUND as i32)) != 0 && underground {
            return true;
        }

        if (the_damage_range_flags & (1 << DamageRangeFlags::DAMAGES_FLYING as i32)) != 0 && self.IsFlying() {
            return true;
        }

        // C++: return TestBit(flags, DAMAGES_GROUND) && !IsFlying() && !submerged && !underground;
        (the_damage_range_flags & (1 << DamageRangeFlags::DAMAGES_GROUND as i32)) != 0
            && !self.IsFlying()
            && !submerged
            && !underground
    }

    /// C++ Zombie::RemoveColdEffects (Zombie.cpp:8614)
    pub unsafe fn RemoveColdEffects(&mut self) {
        if self.m_ice_trap_counter > 0 {
            self.RemoveIceTrap();
        }

        if self.m_chilled_counter > 0 {
            self.m_chilled_counter = 0;
            self.UpdateAnimSpeed();
        }
    }
    /// C++ Zombie::UpdateDamageStates (Zombie.cpp:3871) — 普通僵尸伤害状态（断臂/断头）
    pub unsafe fn UpdateDamageStates(&mut self, the_damage_flags: u32) {
        // C++: if (!CanLoseBodyParts()) return;
        if !self.CanLoseBodyParts() {
            return;
        }

        // C++: if (mHasArm && mBodyHealth < 2 * mBodyMaxHealth / 3 && mBodyHealth > 0) DropArm(theDamageFlags);
        if self.m_has_arm && self.m_body_health < 2 * self.m_body_max_health / 3 && self.m_body_health > 0 {
            self.DropArm(the_damage_flags);
        }

        // C++: if (mHasHead && mBodyHealth < mBodyMaxHealth / 3)
        if self.m_has_head && self.m_body_health < self.m_body_max_health / 3 {
            self.DropHead(the_damage_flags);
            self.DropLoot();
            self.StopZombieSound();

            // C++: if (mBoard->HasLevelAwardDropped()) PlayDeathAnim(theDamageFlags);
            // [TRANSLATION_NOTE]: 游戏内 mBoard 恒非空；加 null 保护以容忍测试/未挂载场景
            let the_board = self.base.m_board as *mut crate::lawn::board::Board;
            if !the_board.is_null() && (*the_board).mLevelAwardSpawned {
                self.PlayDeathAnim(the_damage_flags);
            }

            // C++: if (mZombiePhase == PHASE_SNORKEL_WALKING_IN_POOL) DieNoLoot();
            if self.m_zombie_phase == ZombiePhase::PHASE_SNORKEL_WALKING_IN_POOL {
                self.DieNoLoot();
            }
        }
    }

    /// C++ Zombie::ZamboniDeath (Zombie.cpp:6469) — 冰车僵尸死亡
    pub unsafe fn ZamboniDeath(&mut self, the_damage_flags: u32) {
        if (the_damage_flags & (1 << DamageFlags::DAMAGE_SPIKE as i32)) != 0 {
            // C++: mFlatTires = true; mApp->PlayFoley(FOLEY_TIRE_POP);
            self.m_flat_tires = true;
            // [TODO]: mApp->PlayFoley(FOLEY_TIRE_POP)
            self.m_zombie_phase = ZombiePhase::PHASE_ZOMBIE_DYING;
            // [TODO]: mApp->AddTodParticle(mPosX + 29, mPosY + 114, mRenderOrder + 1, PARTICLE_ZAMBONI_TIRE)
            self.m_vel_x = 0.0;

            // C++: if (Rand(4) == 0 && mPosX < 600.0f)
            if crate::sexy_app_framework::common::rand_int() % 4 == 0 && self.m_pos_x < 600.0 {
                // [TODO]: PlayZombieReanim("anim_wheelie2", REANIM_PLAY_ONCE_AND_HOLD, 10, 10.0f)
                self.m_phase_counter = 280;
            } else {
                // [TODO]: Reanimation 获取 + AddTodParticle(PARTICLE_ZAMBONI_SMOKE) + AttachParticleToTrack
                self.m_phase_counter = 280;
                // [TODO]: PlayZombieReanim("anim_wheelie1", REANIM_PLAY_ONCE_AND_HOLD, 10, 12.0f)
            }
        } else {
            // C++: mApp->AddTodParticle(mPosX + 80, mPosY + 60, mRenderOrder + 1, PARTICLE_ZAMBONI_EXPLOSION);
            // [TODO]: AddTodParticle(PARTICLE_ZAMBONI_EXPLOSION)
            self.DieWithLoot();
            // [TODO]: mApp->PlayFoley(FOLEY_EXPLOSION)
        }
    }

    /// C++ Zombie::CatapultDeath (Zombie.cpp:6505) — 投石车僵尸死亡
    pub unsafe fn CatapultDeath(&mut self, the_damage_flags: u32) {
        if (the_damage_flags & (1 << DamageFlags::DAMAGE_SPIKE as i32)) != 0 {
            // [TODO]: mApp->PlayFoley(FOLEY_TIRE_POP)
            self.m_zombie_phase = ZombiePhase::PHASE_ZOMBIE_DYING;
            // [TODO]: mApp->AddTodParticle(mPosX + 29, mPosY + 114, mRenderOrder + 1, PARTICLE_ZAMBONI_TIRE)
            self.m_vel_x = 0.0;

            // [TODO]: AddAttachedParticle(47, 77, PARTICLE_ZAMBONI_SMOKE)
            self.m_phase_counter = 280;
            // [TODO]: PlayZombieReanim("anim_bounce", REANIM_PLAY_ONCE_AND_HOLD, 10, 12.0f)
        } else {
            // C++: mApp->AddTodParticle(mPosX + 80, mPosY + 60, mRenderOrder + 1, PARTICLE_CATAPULT_EXPLOSION);
            // [TODO]: AddTodParticle(PARTICLE_CATAPULT_EXPLOSION)
            self.DieWithLoot();
            // [TODO]: mApp->PlayFoley(FOLEY_EXPLOSION)
        }
    }

    pub fn GetBodyDamageIndex(&self) -> i32 {
        if self.m_body_max_health > 0 {
            let ratio = self.m_body_health as f32 / self.m_body_max_health as f32;
            if ratio < 0.333 { 2 } else if ratio < 0.667 { 1 } else { 0 }
        } else { 0 }
    }
    pub unsafe fn DieNoLoot(&mut self) {
        self.m_dead = true;
    }

    /// C++ Zombie::DieWithLoot (Zombie.cpp:7291) — 标准死亡+掉落
    pub unsafe fn DieWithLoot(&mut self) {
        self.DieNoLoot();
        // [TODO]: DropLoot() — 掉落阳光/金币/钻石
    }

    /// C++ Zombie::ApplyChill (Zombie.cpp:7489) — 施加冰冻/减速
    pub unsafe fn ApplyChill(&mut self, the_is_ice_trap: bool) {
        if !self.CanBeChilled() { return; }
        if self.m_chilled_counter == 0 {
            self.app().PlayFoley(crate::sexy_tod_lib::tod_foley::FoleyType::FOLEY_FROZEN);
        }
        let a_chill_time = if the_is_ice_trap { 2000 } else { 1000 };
        self.m_chilled_counter = a_chill_time.max(self.m_chilled_counter);
        self.UpdateAnimSpeed();
    }

    /// C++ Zombie::UpdateZombieRiseFromGrave (Zombie.cpp:3011)
    pub unsafe fn UpdateZombieRiseFromGrave(&mut self) {
        // [TODO]: 从墓碑升起的动画逻辑
        // 检测动画是否播放完毕 → 切换到正常行走状态
        self.m_zombie_phase = ZombiePhase::PHASE_ZOMBIE_NORMAL;
    }
    pub unsafe fn IsImmobilizied(&self) -> bool {
        self.m_chilled_counter > 0 || self.m_buttered_counter > 0
    }

    pub unsafe fn IsFlying(&self) -> bool {
        self.m_zombie_phase == ZombiePhase::PHASE_BALLOON_FLYING
    }

    pub unsafe fn HasShadow(&self) -> bool {
        self.m_zombie_type == ZombieType::ZOMBIE_BOSS
            || self.m_zombie_type == ZombieType::ZOMBIE_GARGANTUAR
            || self.m_zombie_type == ZombieType::ZOMBIE_CATAPULT
    }
    pub unsafe fn EnableMustache(&mut self, _enable: bool) {}
    pub unsafe fn EnableFuture(&mut self, _enable: bool) {}
    pub unsafe fn EnableDance(&mut self) {}

    // =========================================================================
    // ★ 新增缺失函数 (1:1 翻译自 Zombie.cpp)
    // =========================================================================

    /// C++ Zombie::DropLoot (Zombie.cpp:7248)
    pub unsafe fn DropLoot(&mut self) {
        if !self.IsOnBoard() { return; }
        if self.m_zombie_type == ZombieType::ZOMBIE_YETI {
            (*self.board()).mKilledYeti = true;
        }
        if self.m_dropped_loot || !(*self.board()).CanDropLoot() {
            return;
        }
        self.m_dropped_loot = true;
        let a_zombie_value = GetZombieDefinition(self.m_zombie_type).mZombieValue;
        if (*self.app()).IsLittleTroubleLevel() && rand_int() % 4 != 0 { return; }
        if (*self.app()).mGameMode == GameMode::GAMEMODE_CHALLENGE_ZOMBIQUARIUM
            || (*self.app()).IsIZombieLevel()
        { return; }

        let a_zombie_rect = self.GetZombieRect();
        let a_center_x = a_zombie_rect.m_x + a_zombie_rect.m_width / 2;
        let a_center_y = a_zombie_rect.m_y + a_zombie_rect.m_height / 4;
        if self.m_zombie_type == ZombieType::ZOMBIE_YETI {
            (*self.app()).PlayFoley(FoleyType::FOLEY_SPAWN_SUN);
            (*self.board()).AddCoin(a_center_x - 20, a_center_y, CoinType::COIN_DIAMOND, CoinMotion::COIN_MOTION_COIN);
            (*self.board()).AddCoin(a_center_x - 30, a_center_y, CoinType::COIN_DIAMOND, CoinMotion::COIN_MOTION_COIN);
            (*self.board()).AddCoin(a_center_x - 40, a_center_y, CoinType::COIN_DIAMOND, CoinMotion::COIN_MOTION_COIN);
            (*self.board()).AddCoin(a_center_x - 50, a_center_y, CoinType::COIN_DIAMOND, CoinMotion::COIN_MOTION_COIN);
        } else {
            // [TODO]: (*self.board()).DropLootPiece(a_center_x, a_center_y, a_zombie_value);
        }
    }

    /// C++ Zombie::CanBeChilled (Zombie.cpp:8002)
    pub unsafe fn CanBeChilled(&self) -> bool {
        if self.m_zombie_type == ZombieType::ZOMBIE_ZAMBONI || self.IsBobsledTeamWithSled() { return false; }
        if self.IsDeadOrDying() { return false; }
        if self.m_zombie_phase == ZombiePhase::PHASE_DIGGER_TUNNELING
            || self.m_zombie_phase == ZombiePhase::PHASE_DIGGER_RISING
            || self.m_zombie_phase == ZombiePhase::PHASE_DIGGER_TUNNELING_PAUSE_WITHOUT_AXE
            || self.m_zombie_phase == ZombiePhase::PHASE_DIGGER_RISE_WITHOUT_AXE
            || self.m_zombie_phase == ZombiePhase::PHASE_RISING_FROM_GRAVE
            || self.m_zombie_phase == ZombiePhase::PHASE_DANCER_RISING
        { return false; }
        if self.m_mind_controlled { return false; }
        return self.m_zombie_type != ZombieType::ZOMBIE_BOSS
            || self.m_zombie_phase == ZombiePhase::PHASE_BOSS_HEAD_IDLE_BEFORE_SPIT
            || self.m_zombie_phase == ZombiePhase::PHASE_BOSS_HEAD_IDLE_AFTER_SPIT
            || self.m_zombie_phase == ZombiePhase::PHASE_BOSS_HEAD_SPIT;
    }

    /// C++ Zombie::CanBeFrozen (Zombie.cpp:8028)
    pub unsafe fn CanBeFrozen(&self) -> bool {
        if !self.CanBeChilled() { return false; }
        if self.m_zombie_phase == ZombiePhase::PHASE_POLEVAULTER_IN_VAULT
            || self.m_zombie_phase == ZombiePhase::PHASE_DOLPHIN_INTO_POOL
            || self.m_zombie_phase == ZombiePhase::PHASE_DOLPHIN_IN_JUMP
            || self.m_zombie_phase == ZombiePhase::PHASE_SNORKEL_INTO_POOL
            || self.IsFlying()
            || self.m_zombie_phase == ZombiePhase::PHASE_IMP_GETTING_THROWN
            || self.m_zombie_phase == ZombiePhase::PHASE_IMP_LANDING
            || self.m_zombie_phase == ZombiePhase::PHASE_BOBSLED_CRASHING
            || self.m_zombie_phase == ZombiePhase::PHASE_JACK_IN_THE_BOX_POPPING
            || self.m_zombie_phase == ZombiePhase::PHASE_SQUASH_RISING
            || self.m_zombie_phase == ZombiePhase::PHASE_SQUASH_FALLING
            || self.m_zombie_phase == ZombiePhase::PHASE_SQUASH_DONE_FALLING
            || self.IsBouncingPogo()
        { return false; }
        return self.m_zombie_type != ZombieType::ZOMBIE_BUNGEE
            || self.m_zombie_phase == ZombiePhase::PHASE_BUNGEE_AT_BOTTOM;
    }

    /// C++ Zombie::ApplyChill — 补全 CanBeChilled 检查
    pub unsafe fn ApplyChillFull(&mut self, the_is_ice_trap: bool) -> bool {
        if !self.CanBeChilled() { return false; }
        if self.m_chilled_counter == 0 {
            (*self.app()).PlayFoley(FoleyType::FOLEY_FROZEN);
        }
        let a_chill_time = if the_is_ice_trap { 2000 } else { 1000 };
        self.m_chilled_counter = a_chill_time.max(self.m_chilled_counter);
        self.UpdateAnimSpeed();
        true
    }

    /// C++ Zombie::ApplyButter (Zombie.cpp:8484)
    pub unsafe fn ApplyButter(&mut self) {
        if !self.m_has_head || !self.CanBeFrozen() { return; }
        if self.m_zombie_type == ZombieType::ZOMBIE_ZAMBONI
            || self.m_zombie_type == ZombieType::ZOMBIE_BOSS
            || self.IsTanglekelpTarget()
            || self.IsBobsledTeamWithSled()
            || self.IsFlying()
        { return; }
        self.m_buttered_counter = 400;
        // [TODO]: ZombieTryToGet
        // let a_zombie = (*self.board()).ZombieTryToGet(self.m_related_zombie_id);
        // if !a_zombie.is_null() { ... }
        self.m_related_zombie_id = ZombieID::ZOMBIEID_NULL;
        if self.m_zombie_type == ZombieType::ZOMBIE_POGO {
            self.m_altitude = 0.0;
            if self.m_on_high_ground { self.m_altitude += HIGH_GROUND_HEIGHT as f32; }
        } else if self.m_zombie_type == ZombieType::ZOMBIE_BALLOON {
            self.BalloonPropellerHatSpin(false);
        }
        self.UpdateAnimSpeed();
        self.StopZombieSound();
    }

    /// C++ Zombie::RemoveButter (Zombie.cpp:8453)
    pub unsafe fn RemoveButter(&mut self) {
        self.m_buttered_counter = 0;
        self.UpdateAnimSpeed();
    }

    /// C++ Zombie::HitIceTrap (Zombie.cpp:8354)
    pub unsafe fn HitIceTrap(&mut self) {
        if !self.CanBeChilled() { return; }
        self.m_ice_trap_counter = 2000;
        if self.m_zombie_type == ZombieType::ZOMBIE_POGO {
            self.m_altitude = 0.0;
        }
        self.UpdateAnimSpeed();
        (*self.app()).PlayFoley(FoleyType::FOLEY_FROZEN);
    }

    /// C++ Zombie::RemoveIceTrap (Zombie.cpp:8499)
    pub unsafe fn RemoveIceTrap(&mut self) {
        self.m_ice_trap_counter = 0;
        self.UpdateAnimSpeed();
    }

    /// C++ Zombie::BobsledDie (Zombie.cpp:7297)
    pub unsafe fn BobsledDie(&mut self) {
        if !self.IsBobsledTeamWithSled() || !self.IsOnBoard() { return; }
        // [TODO]: 需要 DataArray 的 ZombieGet 方法
        // C++ 原逻辑: 遍历 followerZombieID 数组, 对每个僵尸调用 DieNoLoot()
    }

    /// C++ Zombie::IsBobsledTeamWithSled
    pub unsafe fn IsBobsledTeamWithSled(&self) -> bool {
        if self.m_zombie_type != ZombieType::ZOMBIE_BOBSLED { return false; }
        if self.m_zombie_phase == ZombiePhase::PHASE_BOBSLED_CRASHING { return false; }
        if self.m_zombie_phase == ZombiePhase::PHASE_ZOMBIE_BURNED { return false; }
        // [TODO]: 需要检查 leader 存活状态
        true
    }

    /// C++ Zombie::GetBobsledPosition
    pub unsafe fn GetBobsledPosition(&self) -> i32 {
        if self.m_related_zombie_id == ZombieID::ZOMBIEID_NULL { return 0; }
        // [TODO]: 需要 DataArray 访问方法返回位置索引
        0
    }

    /// C++ Zombie::DragUnder (Zombie.cpp:3041)
    pub unsafe fn DragUnder(&mut self) {
        self.m_zombie_height = ZombieHeight::HEIGHT_DRAGGED_UNDER;
        self.m_zombie_phase = ZombiePhase::PHASE_ZOMBIE_DYING;
        if self.m_zombie_type == ZombieType::ZOMBIE_DOLPHIN_RIDER {
            self.m_zombie_phase = ZombiePhase::PHASE_DOLPHIN_WALKING_IN_POOL;
        }
        self.m_phase_counter = 200;
        self.m_has_head = false;
        self.m_has_arm = false;
        self.m_has_object = false;
        self.m_attachment_id = AttachmentID::ATTACHMENTID_NULL;
    }

    /// C++ Zombie::MowDown (Zombie.cpp:8525)
    pub unsafe fn MowDown(&mut self) {
        if self.m_dead || self.m_zombie_phase == ZombiePhase::PHASE_ZOMBIE_MOWERED
            || self.m_zombie_type == ZombieType::ZOMBIE_BOSS
        { return; }

        if self.m_zombie_type == ZombieType::ZOMBIE_CATAPULT {
            (*self.app()).AddTodParticle(self.m_pos_x + 80.0, self.m_pos_y + 60.0, self.base.m_render_order + 1, ParticleEffect::PARTICLE_CATAPULT_EXPLOSION as i32);
            (*self.app()).PlayFoley(FoleyType::FOLEY_EXPLOSION);
            self.DieWithLoot();
            return;
        }
        if self.m_zombie_type == ZombieType::ZOMBIE_ZAMBONI {
            (*self.app()).AddTodParticle(self.m_pos_x + 80.0, self.m_pos_y + 60.0, self.base.m_render_order + 1, ParticleEffect::PARTICLE_ZAMBONI_EXPLOSION as i32);
            (*self.app()).PlayFoley(FoleyType::FOLEY_EXPLOSION);
            self.DieWithLoot();
            return;
        }

        let dying_phases = self.m_zombie_phase == ZombiePhase::PHASE_ZOMBIE_DYING
            || self.m_zombie_phase == ZombiePhase::PHASE_POLEVAULTER_IN_VAULT
            || self.m_zombie_phase == ZombiePhase::PHASE_RISING_FROM_GRAVE
            || self.m_zombie_phase == ZombiePhase::PHASE_DANCER_RISING
            || self.m_zombie_phase == ZombiePhase::PHASE_SNORKEL_INTO_POOL
            || self.m_zombie_phase == ZombiePhase::PHASE_ZOMBIE_BURNED;

        if dying_phases
            || self.m_zombie_type == ZombieType::ZOMBIE_GARGANTUAR
            || self.m_zombie_type == ZombieType::ZOMBIE_REDEYE_GARGANTUAR
            || self.m_zombie_type == ZombieType::ZOMBIE_BUNGEE
            || self.m_zombie_type == ZombieType::ZOMBIE_DIGGER
            || self.m_zombie_type == ZombieType::ZOMBIE_IMP
            || self.m_zombie_type == ZombieType::ZOMBIE_YETI
            || self.m_zombie_type == ZombieType::ZOMBIE_DOLPHIN_RIDER
            || self.IsBobsledTeamWithSled()
            || self.IsFlying()
            || self.m_in_pool
        {
            let a_puff_reanim = (*self.app()).AddReanimation(
                self.m_pos_x - 73.0, self.m_pos_y - 56.0,
                self.base.m_render_order + 2, ReanimationType::REANIM_PUFF
            ) as *mut crate::sexy_tod_lib::reanimator::Reanimation;
            if !a_puff_reanim.is_null() {
                (*a_puff_reanim).set_frames_for_layer("anim_puff");
            }
            (*self.app()).AddTodParticle(
                self.m_pos_x + 110.0, self.m_pos_y + 0.0,
                self.base.m_render_order + 1, ParticleEffect::PARTICLE_MOWER_CLOUD as i32
            );
            if (*self.board()).mPlantRow[self.base.m_row as usize] != PlantRowType::PLANTROW_POOL as i32 {
                self.DropHead(0);
                self.DropArm(0);
                self.DropHelm(0);
                self.DropShield(0);
            }
            self.DieWithLoot();
            return;
        }

        if self.m_ice_trap_counter > 0 { self.RemoveIceTrap(); }
        self.m_buttered_counter = self.m_buttered_counter.min(0);

        self.DropShield(0);
        self.DropHelm(0);
        match self.m_zombie_type {
            ZombieType::ZOMBIE_FLAG => { self.DropFlag(); }
            ZombieType::ZOMBIE_POLEVAULTER => { self.DropPole(); }
            ZombieType::ZOMBIE_NEWSPAPER | ZombieType::ZOMBIE_BALLOON => { self.DropHead(0); }
            ZombieType::ZOMBIE_POGO => { self.DropHead(0); self.m_altitude = 0.0; }
            _ => {}
        }

        let a_mowered_reanim = (*self.app()).AddReanimation(
            0.0, 0.0, self.base.m_render_order, ReanimationType::REANIM_LAWN_MOWERED_ZOMBIE
        ) as *mut crate::sexy_tod_lib::reanimator::Reanimation;
        if !a_mowered_reanim.is_null() {
            (*a_mowered_reanim).m_anim_rate = 8.0;
            (*a_mowered_reanim).m_is_attachment = false;
            (*a_mowered_reanim).m_loop_type = ReanimLoopType::REANIM_PLAY_ONCE_AND_HOLD;
        }
        self.m_mowered_reanim_id = (*self.app()).ReanimationGetID(a_mowered_reanim as *mut std::ffi::c_void);
        self.m_zombie_phase = ZombiePhase::PHASE_ZOMBIE_MOWERED;
        self.DropLoot();
    }

    /// C++ Zombie::IsBouncingPogo
    pub unsafe fn IsBouncingPogo(&self) -> bool {
        self.m_zombie_type == ZombieType::ZOMBIE_POGO
            && self.m_zombie_phase as i32 >= ZombiePhase::PHASE_POGO_BOUNCING as i32
            && self.m_zombie_phase as i32 <= ZombiePhase::PHASE_POGO_FORWARD_BOUNCE_7 as i32
    }

    /// C++ Zombie::IsTanglekelpTarget
    pub unsafe fn IsTanglekelpTarget(&self) -> bool {
        false // [TODO]: 遍历植物检测 SEED_TANGLEKELP 的 mTargetZombieID
    }

    /// C++ Zombie::BalloonPropellerHatSpin
    pub unsafe fn BalloonPropellerHatSpin(&mut self, _the_spinning: bool) {
        // [TODO]: 设置气球僵尸螺旋桨动画速率
    }

    /// C++ Zombie::StopZombieSound
    pub unsafe fn StopZombieSound(&mut self) {
        // [TODO]: 停止僵尸声音
    }

    /// C++ Zombie::GetHelmDamageIndex
    pub unsafe fn GetHelmDamageIndex(&self) -> i32 {
        if self.m_helm_max_health > 0 && self.m_helm_type != HelmType::HELMTYPE_NONE as i32 {
            let ratio = self.m_helm_health as f32 / self.m_helm_max_health as f32;
            if ratio < 0.333 { 2 } else if ratio < 0.667 { 1 } else { 0 }
        } else { 0 }
    }

    /// C++ Zombie::GetShieldDamageIndex
    pub unsafe fn GetShieldDamageIndex(&self) -> i32 {
        if self.m_shield_max_health > 0 && self.m_shield_type != ShieldType::SHIELDTYPE_NONE {
            let ratio = self.m_shield_health as f32 / self.m_shield_max_health as f32;
            if ratio < 0.333 { 2 } else if ratio < 0.667 { 1 } else { 0 }
        } else { 0 }
    }

    /// C++ Zombie::TakeHelmDamage — 修复后的完整实现
    pub unsafe fn TakeHelmDamageFull(&mut self, the_damage: i32, _flags: u32) -> i32 {
        self.m_helm_health -= the_damage;
        if self.m_helm_health <= 0 {
            let a_remaining = -self.m_helm_health;
            self.m_helm_health = 0;
            // [TODO]: DropHelm(theFlags)
            a_remaining
        } else { 0 }
    }

    /// C++ Zombie::DropHead (Zombie.cpp:3471)
    pub unsafe fn DropHead(&mut self, _damage_flags: u32) {
        if !self.m_has_head { return; }
        self.m_has_head = false;
        // [TODO]: 头部掉落粒子效果和动画隐藏
    }

    /// C++ Zombie::DropArm (Zombie.cpp:3845)
    pub unsafe fn DropArm(&mut self, _damage_flags: u32) {
        if !self.m_has_arm { return; }
        self.m_has_arm = false;
        // [TODO]: 手臂掉落效果
    }

    /// C++ Zombie::DropHelm (Zombie.cpp:7636)
    pub unsafe fn DropHelm(&mut self, _damage_flags: u32) {
        if self.m_helm_type == HelmType::HELMTYPE_NONE as i32 { return; }
        self.m_helm_type = HelmType::HELMTYPE_NONE as i32;
        // [TODO]: 头盔掉落效果
    }

    /// C++ Zombie::DropShield (Zombie.cpp:7509)
    pub unsafe fn DropShield(&mut self, _damage_flags: u32) {
        if self.m_shield_type == ShieldType::SHIELDTYPE_NONE { return; }
        self.m_shield_type = ShieldType::SHIELDTYPE_NONE;
        // [TODO]: 盾牌掉落效果
    }

    /// C++ Zombie::DropFlag (Zombie.cpp:3338)
    pub unsafe fn DropFlag(&mut self) {
        // [TODO]: 旗子掉落
    }

    /// C++ Zombie::DropPole (Zombie.cpp:3439)
    pub unsafe fn DropPole(&mut self) {
        // [TODO]: 撑杆掉落
    }

    /// C++ Zombie::GetZombieRect (Zombie.cpp:4159)
    pub unsafe fn GetZombieRect(&self) -> crate::sexy_app_framework::misc::rect::Rect {
        // [TODO]: 根据僵尸类型和状态返回合适的碰撞矩形
        self.m_zombie_rect
    }

    /// C++ Zombie::GetZombieAttackRect (Zombie.cpp:4194)
    pub unsafe fn GetZombieAttackRect(&self) -> crate::sexy_app_framework::misc::rect::Rect {
        // [TODO]: 根据僵尸类型和状态返回合适的攻击矩形
        self.m_zombie_attack_rect
    }

    /// C++ Zombie::CanLoseBodyParts
    pub unsafe fn CanLoseBodyParts(&self) -> bool {
        !self.m_in_pool
            && self.m_zombie_type != ZombieType::ZOMBIE_ZAMBONI
            && self.m_zombie_type != ZombieType::ZOMBIE_BUNGEE
            && self.m_zombie_type != ZombieType::ZOMBIE_BOSS
            && self.m_zombie_type != ZombieType::ZOMBIE_DANCER
            && self.m_zombie_type != ZombieType::ZOMBIE_BACKUP_DANCER
            && self.m_zombie_type != ZombieType::ZOMBIE_CATAPULT
            && !self.IsBobsledTeamWithSled()
    }

    // =========================================================================
    // ★ 类型特定更新方法 (桩实现，需对接 Reanimation 系统后完善)
    // =========================================================================

    pub unsafe fn UpdateZombiePolevaulter(&mut self) {
        if self.m_zombie_phase == ZombiePhase::PHASE_POLEVAULTER_PRE_VAULT && self.m_has_head
            && self.m_zombie_height == ZombieHeight::HEIGHT_ZOMBIE_NORMAL
        {
            let a_plant = self.FindPlantTarget(ZombieAttackType::ATTACKTYPE_VAULT);
            if !a_plant.is_null() {
                // [TODO]: 需要正确的 Plant 字段名和 GetLadderAt 返回值类型
                // if (*self.board()).GetLadderAt((*a_plant).m_plant_col, self.base.m_row).is_null() == false {
                //     ...
                //     return;
                // }
                self.m_zombie_phase = ZombiePhase::PHASE_POLEVAULTER_IN_VAULT;
                // [TODO]: PlayZombieReanim("anim_jump", REANIM_PLAY_ONCE_AND_HOLD, 20, 24.0f);
                self.m_has_object = false;
            }
        } else if self.m_zombie_phase == ZombiePhase::PHASE_POLEVAULTER_IN_VAULT {
            // [TODO]: 检测动画完成 → 切换为 PHASE_POLEVAULTER_POST_VAULT
            self.m_zombie_phase = ZombiePhase::PHASE_POLEVAULTER_POST_VAULT;
            self.StartWalkAnim(0);
        }
    }

    pub unsafe fn UpdateZombieCatapult(&mut self) {
        // [TODO]: 投石车找目标并发射
    }

    pub unsafe fn UpdateZombieFlyer(&mut self) {
        if self.m_zombie_phase == ZombiePhase::PHASE_BALLOON_FLYING && !self.m_dead {
            if !self.m_has_head || self.m_flying_health <= 0 {
                self.m_zombie_phase = ZombiePhase::PHASE_BALLOON_POPPING;
                self.m_phase_counter = 100;
                // [TODO]: PlayZombieReanim("anim_pop", ...);
            }
        }
    }

    pub unsafe fn UpdateZombieNewspaper(&mut self) {
        if self.m_zombie_phase == ZombiePhase::PHASE_NEWSPAPER_MADDENING {
            // [TODO]: 需要正确的 Reanimation 指针类型和 mLoopCount 字段
            // let a_body_reanim = (*self.app()).ReanimationTryToGet(self.m_body_reanim_id) as *mut Reanimation;
            // if !a_body_reanim.is_null() && (*a_body_reanim).m_loop_count > 0 {
                self.m_zombie_phase = ZombiePhase::PHASE_NEWSPAPER_MAD;
                // [TODO]: CountZombiesOnScreen
                // if (*self.board()).CountZombiesOnScreen() <= 10 && self.m_has_head {
                    (*self.app()).PlayFoley(FoleyType::FOLEY_NEWSPAPER_RARRGH);
                // }
                self.StartWalkAnim(20);
            // }
        }
    }

    pub unsafe fn UpdateZombieDolphinRider(&mut self) {
        if self.IsTanglekelpTarget() { return; }
        let a_backwards = self.m_mind_controlled;
        if self.m_zombie_phase == ZombiePhase::PHASE_DOLPHIN_WALKING && !a_backwards {
            if self.base.m_x > 700 && self.base.m_x <= 720 {
                self.m_zombie_phase = ZombiePhase::PHASE_DOLPHIN_INTO_POOL;
                // [TODO]: PlayZombieReanim("anim_jumpinpool", ...);
            }
        } else if self.m_zombie_phase == ZombiePhase::PHASE_DOLPHIN_INTO_POOL {
            // [TODO]: 检测动画结束 → PHASE_DOLPHIN_RIDING
        } else if self.m_zombie_phase == ZombiePhase::PHASE_DOLPHIN_RIDING {
            if self.base.m_x <= 10 {
                self.m_altitude = -40.0;
                self.m_zombie_height = ZombieHeight::HEIGHT_OUT_OF_POOL;
                self.m_zombie_phase = ZombiePhase::PHASE_DOLPHIN_WALKING;
                // [TODO]: PoolSplash, PlayZombieReanim, PickRandomSpeed
                return;
            }
            if self.m_has_head && !self.IsTanglekelpTarget() {
                let a_plant = self.FindPlantTarget(ZombieAttackType::ATTACKTYPE_VAULT);
                if !a_plant.is_null() {
                    // [TODO]: 跳躍动画
                }
            }
        }
    }

    pub unsafe fn UpdateZombieSnorkel(&mut self) {
        let a_backwards = self.m_mind_controlled;
        if self.m_zombie_phase == ZombiePhase::PHASE_SNORKEL_WALKING && !a_backwards {
            if self.base.m_x > 700 && self.base.m_x <= 720 {
                self.m_vel_x = 0.2;
                self.m_zombie_phase = ZombiePhase::PHASE_SNORKEL_INTO_POOL;
                // [TODO]: PlayZombieReanim("anim_jumpinpool", ...);
            }
        } else if self.m_zombie_phase == ZombiePhase::PHASE_SNORKEL_INTO_POOL {
            // [TODO]: 检测动画结束 → PHASE_SNORKEL_WALKING_IN_POOL
        }
    }

    pub unsafe fn UpdateZombieJackInTheBox(&mut self) {
        if self.m_zombie_phase == ZombiePhase::PHASE_JACK_IN_THE_BOX_RUNNING {
            self.m_phase_counter -= 1;
            if self.m_phase_counter <= 0 {
                self.m_zombie_phase = ZombiePhase::PHASE_JACK_IN_THE_BOX_POPPING;
                // [TODO]: 爆炸效果
            }
        }
    }

    pub unsafe fn UpdateZombieGargantuar(&mut self) {
        // [TODO]: 巨人扔小鬼逻辑
    }

    pub unsafe fn UpdateZombieBobsled(&mut self) {
        if self.m_zombie_phase == ZombiePhase::PHASE_BOBSLED_SLIDING {
            self.m_phase_counter -= 1;
            if self.m_phase_counter <= 0 /* [TODO]: || (*self.board()).StageHasIce() == 0 */ {
                self.BobsledCrash();
            }
        }
    }

    pub unsafe fn BobsledCrash(&mut self) {
        if self.m_zombie_phase != ZombiePhase::PHASE_BOBSLED_SLIDING { return; }
        self.m_zombie_phase = ZombiePhase::PHASE_BOBSLED_CRASHING;
        self.m_phase_counter = BOBSLED_CRASH_TIME;
        // [TODO]: 播放碰撞动画 + DataArray 访问
        // for i in 0..NUM_BOBSLED_FOLLOWERS as usize {
        //     let a_zombie = ... 需要 DataArray 访问方式
        // }
    }

    pub unsafe fn UpdateZombieDigger(&mut self) {
        // [TODO]: 挖掘僵尸逻辑
    }

    pub unsafe fn UpdateZombieDancer(&mut self) {
        // [TODO]: 舞者僵尸逻辑
    }

    pub unsafe fn UpdateZamboni(&mut self) {
        // [TODO]: 冰车僵尸逻辑
    }

    pub unsafe fn UpdateClimbingLadder(&mut self) {
        // [TODO]: 梯子僵尸逻辑
    }

    pub unsafe fn UpdateYeti(&mut self) {
        // [TODO]: 雪人僵尸逻辑
    }

    /// C++ Zombie::ApplyBurn (Zombie.cpp:8628)
    pub unsafe fn ApplyBurn(&mut self) {
        if self.m_dead || self.m_zombie_phase == ZombiePhase::PHASE_ZOMBIE_BURNED { return; }
        if self.m_body_health >= 1800 || self.m_zombie_type == ZombieType::ZOMBIE_BOSS {
            self.TakeDamage(1800, 18);
            return;
        }
        if self.m_ice_trap_counter > 0 { self.RemoveIceTrap(); }
        self.m_buttered_counter = self.m_buttered_counter.min(0);
        // [TODO]: 完整烧死逻辑
        self.DieWithLoot();
    }

    /// C++ Zombie::PlayDeathAnim (Zombie.cpp:8869)
    pub unsafe fn PlayDeathAnim(&mut self, the_damage_flags: u32) {
        // C++: if (mZombiePhase == PHASE_ZOMBIE_DYING || PHASE_ZOMBIE_BURNED || PHASE_ZOMBIE_MOWERED) return;
        if self.m_zombie_phase == ZombiePhase::PHASE_ZOMBIE_DYING
            || self.m_zombie_phase == ZombiePhase::PHASE_ZOMBIE_BURNED
            || self.m_zombie_phase == ZombiePhase::PHASE_ZOMBIE_MOWERED
        {
            return;
        }

        // [TODO]: Reanimation 死亡动画轨道检查（aBodyReanim->TrackExists("anim_death")，
        // 无轨道 / 海豚骑手 / 潜泳状态时 DieNoLoot + return）

        // C++: if (TestBit(flags, DAMAGE_DOESNT_LEAVE_BODY)) { 非 Boss/伽刚特尔 → DieNoLoot + return }
        if (the_damage_flags & (1 << DamageFlags::DAMAGE_DOESNT_LEAVE_BODY as i32)) != 0 {
            if self.m_zombie_type != ZombieType::ZOMBIE_BOSS
                && self.m_zombie_type != ZombieType::ZOMBIE_GARGANTUAR
                && self.m_zombie_type != ZombieType::ZOMBIE_REDEYE_GARGANTUAR
            {
                self.DieNoLoot();
                return;
            }
        }

        // C++: mZombiePhase = PHASE_ZOMBIE_DYING; + 按类型播放死亡动画
        self.m_zombie_phase = ZombiePhase::PHASE_ZOMBIE_DYING;
        // [TODO]: 按僵尸类型播放死亡动画（anim_death / anim_die 等）
    }

    pub unsafe fn GetDrawPos(&self, the_draw_pos: &mut ZombieDrawPosition) {
        the_draw_pos.m_image_offset_x = self.m_pos_x - self.base.m_x as f32;
        the_draw_pos.m_image_offset_y = self.m_pos_y - self.base.m_y as f32;

        // C++: 头部位置基于帧
        if self.m_is_eating {
            the_draw_pos.m_head_x = 47;
            the_draw_pos.m_head_y = 4;
        } else {
            let (hx, hy) = match self.m_frame {
                0 => (50, 2),  1 => (49, 1),  2 => (49, 2),  3 => (48, 4),
                4 => (48, 5),  5 => (48, 4),  6 => (48, 2),  7 => (49, 1),
                8 => (49, 2),  9 => (50, 4), 10 => (50, 5),
                _ => (50, 4),
            };
            the_draw_pos.m_head_x = hx;
            the_draw_pos.m_head_y = hy;
        }
        the_draw_pos.m_arm_y = the_draw_pos.m_head_y / 2;

        // C++: 僵尸类型偏移
        match self.m_zombie_type {
            ZombieType::ZOMBIE_FOOTBALL => the_draw_pos.m_image_offset_y -= 16.0,
            ZombieType::ZOMBIE_YETI => the_draw_pos.m_image_offset_y -= 20.0,
            ZombieType::ZOMBIE_CATAPULT => { the_draw_pos.m_image_offset_x -= 25.0; the_draw_pos.m_image_offset_y -= 18.0; }
            ZombieType::ZOMBIE_POGO => the_draw_pos.m_image_offset_y += 16.0,
            ZombieType::ZOMBIE_BALLOON => the_draw_pos.m_image_offset_y += 17.0,
            ZombieType::ZOMBIE_POLEVAULTER => { the_draw_pos.m_image_offset_x -= 6.0; the_draw_pos.m_image_offset_y -= 11.0; }
            ZombieType::ZOMBIE_ZAMBONI => { the_draw_pos.m_image_offset_x += 68.0; the_draw_pos.m_image_offset_y -= 23.0; }
            ZombieType::ZOMBIE_GARGANTUAR | ZombieType::ZOMBIE_REDEYE_GARGANTUAR => the_draw_pos.m_image_offset_y -= 8.0,
            ZombieType::ZOMBIE_BOBSLED => the_draw_pos.m_image_offset_y -= 12.0,
            _ => {}
        }

        // C++: 高度和裁剪逻辑
        if self.m_zombie_phase == ZombiePhase::PHASE_RISING_FROM_GRAVE {
            the_draw_pos.m_body_y = -self.m_altitude;
            if self.m_in_pool {
                the_draw_pos.m_clip_height = the_draw_pos.m_body_y;
            } else {
                let a_height_limit = self.m_phase_counter.min(40) as f32;
                the_draw_pos.m_clip_height = the_draw_pos.m_body_y + a_height_limit;
            }
            if self.m_on_high_ground { the_draw_pos.m_body_y -= HIGH_GROUND_HEIGHT as f32; }
            return;
        }

        if self.m_in_pool {
            the_draw_pos.m_body_y = -self.m_altitude;
            the_draw_pos.m_clip_height = -self.m_altitude - 7.0;
            the_draw_pos.m_clip_height += 10.0 - 10.0 * self.m_scale_zombie;
            if self.m_is_eating { the_draw_pos.m_clip_height += 7.0; }
        } else if self.m_zombie_phase == ZombiePhase::PHASE_DANCER_RISING {
            the_draw_pos.m_body_y = -self.m_altitude;
            the_draw_pos.m_clip_height = -self.m_altitude;
            if self.m_on_high_ground { the_draw_pos.m_body_y -= HIGH_GROUND_HEIGHT as f32; }
        } else if self.m_zombie_phase == ZombiePhase::PHASE_DIGGER_RISING
            || self.m_zombie_phase == ZombiePhase::PHASE_DIGGER_RISE_WITHOUT_AXE
        {
            the_draw_pos.m_body_y = -self.m_altitude;
            if self.m_phase_counter > 20 {
                the_draw_pos.m_clip_height = -self.m_altitude;
            } else {
                the_draw_pos.m_clip_height = CLIP_HEIGHT_OFF;
            }
        } else if self.m_zombie_type == ZombieType::ZOMBIE_BUNGEE {
            the_draw_pos.m_body_y = -self.m_altitude;
            the_draw_pos.m_image_offset_x -= 18.0;
            if self.m_on_high_ground { the_draw_pos.m_body_y -= HIGH_GROUND_HEIGHT as f32; }
            the_draw_pos.m_clip_height = CLIP_HEIGHT_OFF;
        } else {
            the_draw_pos.m_body_y = -self.m_altitude;
            the_draw_pos.m_clip_height = CLIP_HEIGHT_OFF;
        }
    }

    /// C++ Zombie::DrawReanim (Zombie.cpp:5626)
    pub unsafe fn DrawReanim(&self, g: *mut Graphics, the_draw_pos: &ZombieDrawPosition, _the_base_render_group: i32) {
        let a_body_reanim = (*self.app()).ReanimationTryToGet(self.m_body_reanim_id) as *mut crate::sexy_tod_lib::reanimator::Reanimation;
        if a_body_reanim.is_null() { return; }

        let an_offset_x = the_draw_pos.m_image_offset_x + 15.0;
        let mut an_offset_y = the_draw_pos.m_image_offset_y + the_draw_pos.m_body_y - 28.0 + 20.0;

        let mut an_opposite = self.m_mind_controlled;
        if self.m_zombie_type == ZombieType::ZOMBIE_DANCER || self.m_zombie_type == ZombieType::ZOMBIE_BACKUP_DANCER {
            an_opposite = false;
            if self.m_zombie_phase == ZombiePhase::PHASE_DANCER_DANCING_IN && !self.m_is_eating {
                an_opposite = true;
            }
            if self.m_mind_controlled { an_opposite = !an_opposite; }
        }
        if an_opposite {
            // [TODO]: 翻转绘制
        }

        (*a_body_reanim).reanimation_set_scale(self.m_scale_zombie);
        (*a_body_reanim).reanimation_set_position(
            an_offset_x + 30.0 - self.m_scale_zombie * 30.0,
            an_offset_y + 120.0 - self.m_scale_zombie * 120.0,
        );
        (*a_body_reanim).reanimation_draw(&mut *g);
    }

    /// C++ Zombie::DrawIceTrap (Zombie.cpp:6170)
    pub unsafe fn DrawIceTrap(&self, g: *mut Graphics, the_draw_pos: &ZombieDrawPosition, _the_front: bool) {
        if self.m_in_pool || self.m_zombie_type == ZombieType::ZOMBIE_BOSS { return; }
        // [TODO]: 绘制冰陷阱
    }

    /// C++ Zombie::DrawButter (Zombie.cpp:6213)
    pub unsafe fn DrawButter(&self, g: *mut Graphics, the_draw_pos: &ZombieDrawPosition) {
        // [TODO]: 绘制黄油效果
    }

    /// C++ Zombie::DrawShadow (Zombie.cpp:9334)
    pub unsafe fn DrawShadow(&self, g: *mut Graphics) {
        // [TODO]: 绘制阴影
    }

    /// C++ Zombie::UpdateReanim (Zombie.cpp:5250)
    pub unsafe fn UpdateReanimFull(&mut self) {
        // [TODO]: 完整的 Reanimation 更新 — 需要正确的指针类型转换和方法名
        // let a_body_reanim = (*self.app()).ReanimationTryToGet(self.m_body_reanim_id) as *mut Reanimation;
        // if a_body_reanim.is_null() || (*a_body_reanim).m_dead { return; }
        // GetDrawPos, 设置矩阵/位置/缩放, 更新动画
    }

    /// C++ Zombie::FindPlantTarget (Zombie.cpp:6396)
    pub unsafe fn FindPlantTarget(&self, _the_attack_type: ZombieAttackType) -> *mut crate::lawn::plant::Plant {
        // [TODO]: 遍历植物检测碰撞
        std::ptr::null_mut()
    }

    /// C++ Zombie::IsStandingOnSpikeweed
    pub unsafe fn IsStandingOnSpikeweed(&self) -> *mut crate::lawn::plant::Plant {
        // [TODO]: 检测是否站在地刺上
        std::ptr::null_mut()
    }

    /// C++ Zombie::CanTargetPlant (Zombie.cpp:6317)
    pub unsafe fn CanTargetPlant(&self, _the_plant: *mut crate::lawn::plant::Plant, _the_attack_type: ZombieAttackType) -> bool {
        // [TODO]: 完整碰撞检测
        false
    }

    /// C++ Zombie::CheckForPool
    pub unsafe fn CheckForPool(&mut self) {
        // [TODO]: 检测僵尸是否进入泳池
    }

    /// C++ Zombie::CheckForHighGround
    pub unsafe fn CheckForHighGround(&mut self) {
        // [TODO]: 检测僵尸是否进入高台
    }

    /// C++ Zombie::CheckForBoardEdge
    pub unsafe fn CheckForBoardEdge(&mut self) {
        // [TODO]: 检测是否走到屏幕边缘
    }

    /// C++ Zombie::CheckIfPreyCaught
    pub unsafe fn CheckIfPreyCaught(&mut self) {
        // [TODO]: 检测僵尸是否抓到目标
    }

    /// C++ Zombie::CheckForZombieStep
    pub unsafe fn CheckForZombieStep(&mut self) {
        // [TODO]: 脚步声音效
    }

    /// C++ Zombie::PoolSplash
    pub unsafe fn PoolSplash(&mut self, _in_to_pool_sound: bool) {
        // [TODO]: 水花效果
    }

    /// C++ Zombie::PlayZombieReanim
    pub unsafe fn PlayZombieReanim(&mut self, _track_name: &str, _loop_type: ReanimLoopType, _blend_time: i32, _anim_rate: f32) {
        // [TODO]: 播放僵尸动画
    }

    /// C++ Zombie::ShowDoorArms
    pub unsafe fn ShowDoorArms(&mut self, _show: bool) {
        // [TODO]: 显示/隐藏门板僵尸手臂
    }

    /// C++ Zombie::LandFlyer
    pub unsafe fn LandFlyer(&mut self, _damage_flags: u32) {
        // [TODO]: 飞行僵尸降落
    }

    /// C++ Zombie::PogoBreak
    pub unsafe fn PogoBreak(&mut self, _damage_flags: u32) {
        // [TODO]: 弹跳杆断裂
    }

    /// C++ Zombie::SetupWaterTrack
    pub unsafe fn SetupWaterTrack(&mut self, _track_name: &str) {
        // [TODO]: 设置水花轨道
    }

    /// C++ Zombie::GetPosYBasedOnRow
    pub unsafe fn GetPosYBasedOnRow(&self, the_row: i32) -> f32 {
        // [TODO]: 基于行的 Y 坐标
        80.0 + the_row as f32 * 100.0
    }

    /// C++ Zombie::SetRow
    pub unsafe fn SetRow(&mut self, the_row: i32) {
        self.base.m_row = the_row;
        // RENDER_LAYER_ZOMBIE = 14 (typical value), offset 4
        self.base.m_render_order = (14 << 16) | ((the_row & 0xFF) << 8) | 4;
    }

    /// C++ Zombie::BungeeDropZombie (Zombie.cpp:1119)
    pub unsafe fn BungeeDropZombie(&mut self, theDroppedZombie: *mut Zombie, theGridX: i32, theGridY: i32) {
        // C++: mTargetCol = theGridX;
        self.m_target_col = theGridX;
        // C++: SetRow(theGridY);
        self.SetRow(theGridY);
        // C++: mPosX = mBoard->GridToPixelX(mTargetCol, mRow);
        let theBoard = self.base.m_board as *mut crate::lawn::board::Board;
        self.m_pos_x = (*theBoard).GridToPixelX(self.m_target_col, self.base.m_row) as f32;
        // C++: mPosY = GetPosYBasedOnRow(mRow);
        self.m_pos_y = self.GetPosYBasedOnRow(self.base.m_row);
        // C++: PlayZombieReanim("anim_raise", ReanimLoopType::REANIM_PLAY_ONCE_AND_HOLD, 0, 36.0f);
        // [TODO]: PlayZombieReanim 动画系统尚未实现
        // C++: mRelatedZombieID = mBoard->ZombieGetID(theDroppedZombie);
        let theDroppedID = (*theBoard).mZombies.data_array_get_id(theDroppedZombie);
        self.m_related_zombie_id = std::mem::transmute::<u32, ZombieID>(theDroppedID);

        // C++: theDroppedZombie->mPosX = mPosX - 15.0f;
        (*theDroppedZombie).m_pos_x = self.m_pos_x - 15.0f32;
        // C++: theDroppedZombie->SetRow(theGridY);
        (*theDroppedZombie).SetRow(theGridY);
        // C++: theDroppedZombie->mPosY = GetPosYBasedOnRow(theGridY);
        (*theDroppedZombie).m_pos_y = (*theDroppedZombie).GetPosYBasedOnRow(theGridY);
        // C++: theDroppedZombie->mZombieHeight = ZombieHeight::HEIGHT_GETTING_BUNGEE_DROPPED;
        (*theDroppedZombie).m_zombie_height = ZombieHeight::HEIGHT_GETTING_BUNGEE_DROPPED;
        // C++: theDroppedZombie->PlayZombieReanim("anim_idle", ReanimLoopType::REANIM_LOOP, 0, 0.0f);
        // [TODO]: PlayZombieReanim 动画系统尚未实现
        // C++: theDroppedZombie->mRenderOrder = mRenderOrder + 1;
        (*theDroppedZombie).base.m_render_order = self.base.m_render_order + 1;
    }

    /// C++ Zombie::OverrideParticleColor
    pub unsafe fn OverrideParticleColor(&self, _a_particle: *mut crate::sexy_tod_lib::tod_particle::TodParticleSystem) {
        // [TODO]
    }

    /// C++ Zombie::OverrideParticleScale
    pub unsafe fn OverrideParticleScale(&self, _a_particle: *mut crate::sexy_tod_lib::tod_particle::TodParticleSystem) {
        // [TODO]
    }

    /// C++ Zombie::AddAttachedParticle
    pub unsafe fn AddAttachedParticle(&mut self, _pos_x: i32, _pos_y: i32, _effect: ParticleEffect) -> *mut crate::sexy_tod_lib::tod_particle::TodParticleSystem {
        std::ptr::null_mut()
    }

    /// C++ Zombie::AddAttachedReanim
    pub unsafe fn AddAttachedReanim(&mut self, _pos_x: i32, _pos_y: i32, _reanim_type: ReanimationType) -> *mut crate::sexy_tod_lib::reanimator::Reanimation {
        std::ptr::null_mut()
    }

    /// C++ Zombie::LoadReanim
    pub unsafe fn LoadReanim(&mut self, _reanimation_type: ReanimationType) -> *mut crate::sexy_tod_lib::reanimator::Reanimation {
        std::ptr::null_mut()
    }

    /// C++ Zombie::AttachShield
    pub unsafe fn AttachShield(&mut self) {
        // [TODO]: 附加盾牌
    }

    /// C++ Zombie::DetachShield
    pub unsafe fn DetachShield(&mut self) {
        // [TODO]: 分离盾牌
    }

    /// C++ Zombie::GetTrackPosition
    pub unsafe fn GetTrackPosition(&self, _track_name: &str, _pos_x: &mut f32, _pos_y: &mut f32) {
        // [TODO]
    }

    /// C++ Zombie::ReanimShowTrack
    pub unsafe fn ReanimShowTrack(&mut self, _track_name: &str, _render_group: i32) {
        // [TODO]
    }

    /// C++ Zombie::ReanimShowPrefix
    pub unsafe fn ReanimShowPrefix(&mut self, _track_prefix: &str, _render_group: i32) {
        // [TODO]
    }

    /// C++ Zombie::ReanimIgnoreClipRect
    pub unsafe fn ReanimIgnoreClipRect(&mut self, _track_name: &str, _ignore_clip_rect: bool) {
        // [TODO]
    }

    /// C++ Zombie::ReanimReenableClipping
    pub unsafe fn ReanimReenableClipping(&mut self) {
        // [TODO]
    }

    /// C++ Zombie::SetAnimRate
    pub unsafe fn SetAnimRate(&mut self, _anim_rate: f32) {
        // [TODO]
    }

    /// C++ Zombie::ApplyAnimRate
    pub unsafe fn ApplyAnimRate(&mut self, _anim_rate: f32) {
        // [TODO]
    }

    /// C++ Zombie::StartZombieSound
    pub unsafe fn StartZombieSound(&mut self) {
        // [TODO]
    }

    /// C++ Zombie::AnimateChewSound
    pub unsafe fn AnimateChewSound(&mut self) {
        // [TODO]: 播放咀嚼音效
    }

    /// C++ Zombie::AnimateChewEffect
    pub unsafe fn AnimateChewEffect(&mut self) {
        // [TODO]: 播放咀嚼粒子效果
    }

    /// C++ Zombie::IsFireResistant
    pub unsafe fn IsFireResistant(&self) -> bool {
        self.m_zombie_type == ZombieType::ZOMBIE_ZAMBONI
            || self.m_zombie_type == ZombieType::ZOMBIE_BOSS
            || self.m_zombie_type == ZombieType::ZOMBIE_CATAPULT
    }

    /// C++ Zombie::ZombieNotWalking
    pub unsafe fn ZombieNotWalking(&self) -> bool {
        self.m_is_eating
            || self.IsImmobilizied()
            || self.m_zombie_phase == ZombiePhase::PHASE_ZOMBIE_DYING
            || self.m_zombie_phase == ZombiePhase::PHASE_ZOMBIE_BURNED
            || self.m_zombie_phase == ZombiePhase::PHASE_ZOMBIE_MOWERED
    }

    /// C++ Zombie::NeedsMoreBackupDancers
    pub unsafe fn NeedsMoreBackupDancers(&self) -> bool {
        let mut a_count = 0;
        for i in 0..NUM_BACKUP_DANCERS as usize {
            if self.m_follower_zombie_id[i] != ZombieID::ZOMBIEID_NULL { a_count += 1; }
        }
        a_count < NUM_BACKUP_DANCERS
    }

    /// C++ Zombie::SummonBackupDancer
    pub unsafe fn SummonBackupDancer(&mut self, _the_row: i32, _the_pos_x: i32) -> ZombieID {
        ZombieID::ZOMBIEID_NULL
    }

    /// C++ Zombie::SummonBackupDancers
    pub unsafe fn SummonBackupDancers(&mut self) {
        // [TODO]: 召唤伴舞
    }

    /// C++ Zombie::IsZombotany
    pub unsafe fn IsZombotany(zombie_type: ZombieType) -> bool {
        zombie_type == ZombieType::ZOMBIE_PEA_HEAD
            || zombie_type == ZombieType::ZOMBIE_WALLNUT_HEAD
            || zombie_type == ZombieType::ZOMBIE_TALLNUT_HEAD
            || zombie_type == ZombieType::ZOMBIE_JALAPENO_HEAD
            || zombie_type == ZombieType::ZOMBIE_GATLING_HEAD
            || zombie_type == ZombieType::ZOMBIE_SQUASH_HEAD
    }

    /// C++ Zombie::ZombieTypeCanGoInPool
    pub unsafe fn ZombieTypeCanGoInPool(zombie_type: ZombieType) -> bool {
        zombie_type == ZombieType::ZOMBIE_SNORKEL
            || zombie_type == ZombieType::ZOMBIE_DOLPHIN_RIDER
            || zombie_type == ZombieType::ZOMBIE_DUCKY_TUBE
            || zombie_type == ZombieType::ZOMBIE_ZAMBONI
    }

    /// C++ Zombie::ZombieTypeCanGoOnHighGround
    pub unsafe fn ZombieTypeCanGoOnHighGround(zombie_type: ZombieType) -> bool {
        zombie_type != ZombieType::ZOMBIE_POGO
    }

    /// C++ Zombie::PreloadZombieResources
    pub unsafe fn PreloadZombieResources(_zombie_type: ZombieType) {
        // [TODO]
    }

    /// C++ Zombie::PickRandomSpeed (Zombie.cpp:1137)
    pub unsafe fn PickRandomSpeed(&mut self) {
        if self.m_zombie_phase == ZombiePhase::PHASE_SNORKEL_WALKING_IN_POOL {
            self.m_vel_x = 0.3;
        } else if self.m_zombie_phase == ZombiePhase::PHASE_DIGGER_WALKING {
            if (*self.app()).IsIZombieLevel() { self.m_vel_x = 0.23; }
            else { self.m_vel_x = 0.12; }
        } else if self.m_zombie_type == ZombieType::ZOMBIE_IMP && (*self.app()).IsIZombieLevel() {
            self.m_vel_x = 0.9;
        } else if self.m_zombie_phase == ZombiePhase::PHASE_YETI_RUNNING {
            self.m_vel_x = 0.8;
        } else if self.m_zombie_type == ZombieType::ZOMBIE_YETI {
            self.m_vel_x = 0.4;
        } else if self.m_zombie_type == ZombieType::ZOMBIE_DANCER
            || self.m_zombie_type == ZombieType::ZOMBIE_BACKUP_DANCER
            || self.m_zombie_type == ZombieType::ZOMBIE_POGO
            || self.m_zombie_type == ZombieType::ZOMBIE_FLAG
        {
            self.m_vel_x = 0.45;
        } else if self.m_zombie_phase == ZombiePhase::PHASE_DIGGER_TUNNELING
            || self.m_zombie_phase == ZombiePhase::PHASE_POLEVAULTER_PRE_VAULT
            || self.m_zombie_type == ZombieType::ZOMBIE_FOOTBALL
            || self.m_zombie_type == ZombieType::ZOMBIE_SNORKEL
            || self.m_zombie_type == ZombieType::ZOMBIE_JACK_IN_THE_BOX
        {
            self.m_vel_x = rand_range_float(0.66, 0.68);
        } else if self.m_zombie_phase == ZombiePhase::PHASE_LADDER_CARRYING
            || self.m_zombie_type == ZombieType::ZOMBIE_SQUASH_HEAD
        {
            self.m_vel_x = rand_range_float(0.79, 0.81);
        } else if self.m_zombie_phase == ZombiePhase::PHASE_NEWSPAPER_MAD
            || self.m_zombie_phase == ZombiePhase::PHASE_DOLPHIN_WALKING
            || self.m_zombie_phase == ZombiePhase::PHASE_DOLPHIN_WALKING_WITHOUT_DOLPHIN
        {
            self.m_vel_x = rand_range_float(0.89, 0.91);
        } else {
            self.m_vel_x = rand_range_float(0.23, 0.37);
            if self.m_vel_x < 0.3 { self.m_anim_ticks_per_frame = 12; }
            else { self.m_anim_ticks_per_frame = 15; }
        }
        self.UpdateAnimSpeed();
    }
}

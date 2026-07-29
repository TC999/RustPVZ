// [TRANSLATION_NOTE]: Zombie.h -> Rust 模块
// C++ Zombie 类翻译为 Rust struct + impl

use crate::const_enums::*;
use super::game_object::GameObject;
use crate::sexy_app_framework::graphics::graphics::Graphics;
use crate::sexy_app_framework::common::*;
use crate::sexy_tod_lib::tod_foley::FoleyType;

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
            // [TODO]: UpdateZombieRiseFromGrave()
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
        // TODO: Zombie action dispatch based on type and state
        // Handles: walking, eating, pole-vaulting, dolphin-riding, etc.
    }

    pub unsafe fn UpdateZombieWalking(&mut self) {
        // TODO: Movement logic
    }

    pub unsafe fn UpdateZombiePosition(&mut self) {
        // TODO: Position update from velocity
    }

    pub unsafe fn Animate(&mut self) {
        // TODO: Frame animation counter update
        self.m_anim_counter += 1;
        if self.m_anim_counter >= self.m_anim_ticks_per_frame {
            self.m_anim_counter = 0;
            self.m_prev_frame = self.m_frame;
            self.m_frame += 1;
        }
    }

    pub unsafe fn UpdateBurn(&mut self) {}
    pub unsafe fn UpdateDeath(&mut self) {}
    pub unsafe fn UpdateMowered(&mut self) {}
    pub unsafe fn UpdateZombieBungee(&mut self) {}
    pub unsafe fn UpdateZombiePogo(&mut self) {}
    pub unsafe fn UpdateZombieChimney(&mut self) {}
    pub unsafe fn UpdateReanim(&mut self) {}
    pub unsafe fn UpdateYuckyFace(&mut self) {}
    pub unsafe fn UpdateZombiePool(&mut self) {}
    pub unsafe fn UpdateZombieHighGround(&mut self) {}
    pub unsafe fn UpdateZombieFalling(&mut self) {}
    pub unsafe fn UpdateAnimSpeed(&mut self) {}
    pub unsafe fn DieNoLoot(&mut self) {
        self.m_dead = true;
    }
    pub unsafe fn IsImmobilizied(&self) -> bool {
        self.m_chilled_counter > 0 || self.m_buttered_counter > 0
    }
    pub unsafe fn HasShadow(&self) -> bool {
        self.m_zombie_type == ZombieType::ZOMBIE_BOSS
            || self.m_zombie_type == ZombieType::ZOMBIE_GARGANTUAR
            || self.m_zombie_type == ZombieType::ZOMBIE_CATAPULT
    }
    pub unsafe fn EnableMustache(&mut self, _enable: bool) {}
    pub unsafe fn EnableFuture(&mut self, _enable: bool) {}
    pub unsafe fn EnableDance(&mut self) {}
}

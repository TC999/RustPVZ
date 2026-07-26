// [TRANSLATION_NOTE]: Zombie.h -> Rust 模块
// C++ Zombie 类翻译为 Rust struct + impl

use crate::const_enums::*;
use super::game_object::GameObject;

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

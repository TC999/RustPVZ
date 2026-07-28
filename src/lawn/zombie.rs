// [TRANSLATION_NOTE]: Zombie.h -> Rust 模块
// C++ Zombie 类翻译为 Rust struct + impl

use crate::const_enums::*;
use super::game_object::GameObject;
use crate::sexy_app_framework::graphics::graphics::Graphics;

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
// =========================================================================
#[derive(Clone, Copy)]
pub struct ZombieDefinition {
    pub mZombieType: ZombieType,
    pub mReanimationType: ReanimationType,
    pub mZombieValue: i32,
    pub mStartingLevel: i32,
    pub mFirstAllowedWave: i32,
    pub mPickWeight: i32,
}

pub const NUM_ZOMBIE_TYPES: i32 = 34;

// gZombieDefs array (stub - will be populated properly in future)
pub static mut G_ZOMBIE_DEFS: [ZombieDefinition; 34] = [ZombieDefinition {
    mZombieType: ZombieType::ZOMBIE_NORMAL,
    mReanimationType: ReanimationType::REANIM_NONE,
    mZombieValue: 0,
    mStartingLevel: 0,
    mFirstAllowedWave: 0,
    mPickWeight: 1,
}; 34];

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

    /// C++ Zombie::Draw() — 绘制 (lines 6264-6315)
    pub unsafe fn Draw(&self, g: &mut Graphics) {
        if self.m_zombie_height == ZombieHeight::HEIGHT_GETTING_BUNGEE_DROPPED {
            return;
        }

        // ZombieDrawPosition aDrawPos;
        // GetDrawPos(aDrawPos);
        let board = self.board();

        if (*self.app()).mGameScene as i32 == GameScenes::SCENE_ZOMBIES_WON as i32 {
            // if !SetupDrawZombieWon(g) { return; }
        }

        if self.m_ice_trap_counter > 0 {
            // DrawIceTrap(g, aDrawPos, false);
        }
        if (*self.app()).mGameMode as i32 != GameMode::GAMEMODE_CHALLENGE_INVISIGHOUL as i32
            || self.m_from_wave == crate::lawn::zombie::ZOMBIE_WAVE_UI
        {
            if self.m_body_reanim_id != ReanimationID::REANIMATIONID_NULL {
                // DrawReanim(g, aDrawPos, RENDER_GROUP_NORMAL);
            }
        }
        if self.m_ice_trap_counter > 0 {
            // DrawIceTrap(g, aDrawPos, true);
        }
        if self.m_buttered_counter > 0 {
            // DrawButter(g, aDrawPos);
        }

        // AttachmentDraw
        // g->ClearClipRect();
    }

    // === Sub-update methods (stubs, to be filled in) ===

    pub unsafe fn UpdatePlaying(&mut self) {
        // TODO: Full zombie playing update - movement, eating, etc.
        self.UpdateActions();
        self.UpdateZombiePosition();
        self.UpdateYuckyFace();
        self.UpdateBurn();
        self.UpdateDeath();
        self.UpdateMowered();
        self.UpdateZombiePool();
        self.UpdateZombieHighGround();
        self.UpdateZombieFalling();
        self.UpdateAnimSpeed();
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

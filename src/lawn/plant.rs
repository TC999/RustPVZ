// [TRANSLATION_NOTE]: Plant.h -> Rust 模块
// C++ Plant 类翻译为 Rust struct + impl 块

use crate::const_enums::*;
use crate::sexy_app_framework::misc::rect::Rect;
use crate::sexy_app_framework::graphics::graphics::{Graphics, Image};
use crate::sexy_tod_lib::tod_foley::FoleyType;
use super::game_object::GameObject;

pub const MAX_MAGNET_ITEMS: i32 = 5;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PlantSubClass {
    SUBCLASS_NORMAL = 0,
    SUBCLASS_SHOOTER = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PlantWeapon {
    WEAPON_PRIMARY,
    WEAPON_SECONDARY,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PlantOnBungeeState {
    NOT_ON_BUNGEE,
    GETTING_GRABBED_BY_BUNGEE,
    RISING_WITH_BUNGEE,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PlantState {
    STATE_NOTREADY,
    STATE_READY,
    STATE_DOINGSPECIAL,
    STATE_SQUASH_LOOK,
    STATE_SQUASH_PRE_LAUNCH,
    STATE_SQUASH_RISING,
    STATE_SQUASH_FALLING,
    STATE_SQUASH_DONE_FALLING,
    STATE_GRAVEBUSTER_LANDING,
    STATE_GRAVEBUSTER_EATING,
    STATE_CHOMPER_BITING,
    STATE_CHOMPER_BITING_GOT_ONE,
    STATE_CHOMPER_BITING_MISSED,
    STATE_CHOMPER_DIGESTING,
    STATE_CHOMPER_SWALLOWING,
    STATE_POTATO_RISING,
    STATE_POTATO_ARMED,
    STATE_POTATO_MASHED,
    STATE_SPIKEWEED_ATTACKING,
    STATE_SPIKEWEED_ATTACKING_2,
    STATE_SCAREDYSHROOM_LOWERING,
    STATE_SCAREDYSHROOM_SCARED,
    STATE_SCAREDYSHROOM_RAISING,
    STATE_SUNSHROOM_SMALL,
    STATE_SUNSHROOM_GROWING,
    STATE_SUNSHROOM_BIG,
    STATE_MAGNETSHROOM_SUCKING,
    STATE_MAGNETSHROOM_CHARGING,
    STATE_BOWLING_UP,
    STATE_BOWLING_DOWN,
    STATE_CACTUS_LOW,
    STATE_CACTUS_RISING,
    STATE_CACTUS_HIGH,
    STATE_CACTUS_LOWERING,
    STATE_TANGLEKELP_GRABBING,
    STATE_COBCANNON_ARMING,
    STATE_COBCANNON_LOADING,
    STATE_COBCANNON_READY,
    STATE_COBCANNON_FIRING,
    STATE_KERNELPULT_BUTTER,
    STATE_UMBRELLA_TRIGGERED,
    STATE_UMBRELLA_REFLECTING,
    STATE_IMITATER_MORPHING,
    STATE_ZEN_GARDEN_WATERED,
    STATE_ZEN_GARDEN_NEEDY,
    STATE_ZEN_GARDEN_HAPPY,
    STATE_MARIGOLD_ENDING,
    STATE_FLOWERPOT_INVULNERABLE,
    STATE_LILYPAD_INVULNERABLE,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PLANT_LAYER {
    PLANT_LAYER_BELOW = -1,
    PLANT_LAYER_MAIN,
    PLANT_LAYER_REANIM,
    PLANT_LAYER_REANIM_HEAD,
    PLANT_LAYER_REANIM_BLINK,
    PLANT_LAYER_ON_TOP,
    NUM_PLANT_LAYERS,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PLANT_ORDER {
    PLANT_ORDER_LILYPAD,
    PLANT_ORDER_NORMAL,
    PLANT_ORDER_PUMPKIN,
    PLANT_ORDER_FLYER,
    PLANT_ORDER_CHERRYBOMB,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum MagnetItemType {
    MAGNET_ITEM_NONE,
    MAGNET_ITEM_PAIL_1,
    MAGNET_ITEM_PAIL_2,
    MAGNET_ITEM_PAIL_3,
    MAGNET_ITEM_FOOTBALL_HELMET_1,
    MAGNET_ITEM_FOOTBALL_HELMET_2,
    MAGNET_ITEM_FOOTBALL_HELMET_3,
    MAGNET_ITEM_DOOR_1,
    MAGNET_ITEM_DOOR_2,
    MAGNET_ITEM_DOOR_3,
    MAGNET_ITEM_POGO_1,
    MAGNET_ITEM_POGO_2,
    MAGNET_ITEM_POGO_3,
    MAGNET_ITEM_JACK_IN_THE_BOX,
    MAGNET_ITEM_LADDER_1,
    MAGNET_ITEM_LADDER_2,
    MAGNET_ITEM_LADDER_3,
    MAGNET_ITEM_LADDER_PLACED,
    MAGNET_ITEM_SILVER_COIN,
    MAGNET_ITEM_GOLD_COIN,
    MAGNET_ITEM_DIAMOND,
    MAGNET_ITEM_PICK_AXE,
}

#[derive(Clone)]
pub struct MagnetItem {
    pub m_pos_x: f32,
    pub m_pos_y: f32,
    pub m_dest_offset_x: f32,
    pub m_dest_offset_y: f32,
    pub m_item_type: MagnetItemType,
}

impl MagnetItem {
    pub fn new() -> Self {
        MagnetItem {
            m_pos_x: 0.0,
            m_pos_y: 0.0,
            m_dest_offset_x: 0.0,
            m_dest_offset_y: 0.0,
            m_item_type: MagnetItemType::MAGNET_ITEM_NONE,
        }
    }
}

impl Default for MagnetItem {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone)]
pub struct Plant {
    pub base: GameObject,
    pub m_seed_type: SeedType,
    pub m_plant_col: i32,
    pub m_anim_counter: i32,
    pub m_frame: i32,
    pub m_frame_length: i32,
    pub m_num_frames: i32,
    pub m_state: PlantState,
    pub m_plant_health: i32,
    pub m_plant_max_health: i32,
    pub m_subclass: i32,
    pub m_disappear_countdown: i32,
    pub m_do_special_countdown: i32,
    pub m_state_countdown: i32,
    pub m_launch_counter: i32,
    pub m_launch_rate: i32,
    pub m_plant_rect: Rect,
    pub m_plant_attack_rect: Rect,
    pub m_target_x: i32,
    pub m_target_y: i32,
    pub m_start_row: i32,
    pub m_particle_id: ParticleID,
    pub m_shooting_counter: i32,
    pub m_body_reanim_id: ReanimationID,
    pub m_head_reanim_id: ReanimationID,
    pub m_head_reanim_id2: ReanimationID,
    pub m_head_reanim_id3: ReanimationID,
    pub m_blink_reanim_id: ReanimationID,
    pub m_light_reanim_id: ReanimationID,
    pub m_sleeping_reanim_id: ReanimationID,
    pub m_blink_countdown: i32,
    pub m_recently_eaten_countdown: i32,
    pub m_eaten_flash_countdown: i32,
    pub m_beghouled_flash_countdown: i32,
    pub m_shake_offset_x: f32,
    pub m_shake_offset_y: f32,
    pub m_magnet_items: [MagnetItem; MAX_MAGNET_ITEMS as usize],
    pub m_target_zombie_id: ZombieID,
    pub m_wake_up_counter: i32,
    pub m_on_bungee_state: PlantOnBungeeState,
    pub m_imitater_type: i32,
    pub m_potted_plant_index: i32,
    pub m_anim_ping: bool,
    pub m_dead: bool,
    pub m_squished: bool,
    pub m_is_asleep: bool,
    pub m_is_on_board: bool,
    pub m_highlighted: bool,
}

impl Plant {
    pub fn new() -> Self {
        Plant {
            base: GameObject::new(),
            m_seed_type: SeedType::SEED_PEASHOOTER,
            m_plant_col: 0,
            m_anim_counter: 0,
            m_frame: 0,
            m_frame_length: 0,
            m_num_frames: 0,
            m_state: PlantState::STATE_NOTREADY,
            m_plant_health: 0,
            m_plant_max_health: 0,
            m_subclass: 0,
            m_disappear_countdown: 0,
            m_do_special_countdown: 0,
            m_state_countdown: 0,
            m_launch_counter: 0,
            m_launch_rate: 0,
            m_plant_rect: Rect::new(0, 0, 0, 0),
            m_plant_attack_rect: Rect::new(0, 0, 0, 0),
            m_target_x: 0,
            m_target_y: 0,
            m_start_row: 0,
            m_particle_id: ParticleID::PARTICLEID_NULL,
            m_shooting_counter: 0,
            m_body_reanim_id: ReanimationID::REANIMATIONID_NULL,
            m_head_reanim_id: ReanimationID::REANIMATIONID_NULL,
            m_head_reanim_id2: ReanimationID::REANIMATIONID_NULL,
            m_head_reanim_id3: ReanimationID::REANIMATIONID_NULL,
            m_blink_reanim_id: ReanimationID::REANIMATIONID_NULL,
            m_light_reanim_id: ReanimationID::REANIMATIONID_NULL,
            m_sleeping_reanim_id: ReanimationID::REANIMATIONID_NULL,
            m_blink_countdown: 0,
            m_recently_eaten_countdown: 0,
            m_eaten_flash_countdown: 0,
            m_beghouled_flash_countdown: 0,
            m_shake_offset_x: 0.0,
            m_shake_offset_y: 0.0,
            m_magnet_items: [MagnetItem::new(), MagnetItem::new(), MagnetItem::new(), MagnetItem::new(), MagnetItem::new()],
            m_target_zombie_id: ZombieID::ZOMBIEID_NULL,
            m_wake_up_counter: 0,
            m_on_bungee_state: PlantOnBungeeState::NOT_ON_BUNGEE,
            m_imitater_type: SeedType::SEED_NONE as i32,
            m_potted_plant_index: 0,
            m_anim_ping: false,
            m_dead: false,
            m_squished: false,
            m_is_asleep: false,
            m_is_on_board: false,
            m_highlighted: false,
        }
    }

    // Static helpers
    pub fn is_nocturnal(seed_type: SeedType) -> bool {
        matches!(seed_type, 
            SeedType::SEED_PUFFSHROOM | SeedType::SEED_SEASHROOM |
            SeedType::SEED_SUNSHROOM | SeedType::SEED_FUMESHROOM |
            SeedType::SEED_HYPNOSHROOM | SeedType::SEED_DOOMSHROOM |
            SeedType::SEED_ICESHROOM | SeedType::SEED_MAGNETSHROOM |
            SeedType::SEED_SCAREDYSHROOM | SeedType::SEED_GLOOMSHROOM
        )
    }

    pub fn is_fungus(seed_type: SeedType) -> bool {
        matches!(seed_type,
            SeedType::SEED_PUFFSHROOM | SeedType::SEED_SUNSHROOM |
            SeedType::SEED_FUMESHROOM | SeedType::SEED_HYPNOSHROOM |
            SeedType::SEED_SCAREDYSHROOM | SeedType::SEED_ICESHROOM |
            SeedType::SEED_DOOMSHROOM | SeedType::SEED_GLOOMSHROOM |
            SeedType::SEED_SEASHROOM | SeedType::SEED_MAGNETSHROOM
        )
    }

    pub fn is_aquatic(seed_type: SeedType) -> bool {
        matches!(seed_type,
            SeedType::SEED_LILYPAD | SeedType::SEED_TANGLEKELP |
            SeedType::SEED_SEASHROOM | SeedType::SEED_CATTAIL
        )
    }

    pub fn is_flying(seed_type: SeedType) -> bool {
        seed_type == SeedType::SEED_INSTANT_COFFEE
    }

    pub fn is_upgrade(seed_type: SeedType) -> bool {
        matches!(seed_type,
            SeedType::SEED_GATLINGPEA | SeedType::SEED_TWINSUNFLOWER |
            SeedType::SEED_GLOOMSHROOM | SeedType::SEED_CATTAIL |
            SeedType::SEED_WINTERMELON | SeedType::SEED_GOLD_MAGNET |
            SeedType::SEED_SPIKEROCK | SeedType::SEED_COBCANNON
        )
    }

    pub fn not_on_ground(&self) -> bool {
        self.m_state == PlantState::STATE_SQUASH_RISING 
            || self.m_state == PlantState::STATE_SQUASH_FALLING 
            || self.m_state == PlantState::STATE_SQUASH_DONE_FALLING
            || self.m_on_bungee_state != PlantOnBungeeState::NOT_ON_BUNGEE
    }

    pub fn makes_sun(&self) -> bool {
        self.m_seed_type == SeedType::SEED_SUNFLOWER 
            || self.m_seed_type == SeedType::SEED_TWINSUNFLOWER 
            || self.m_seed_type == SeedType::SEED_SUNSHROOM 
            || self.m_seed_type == SeedType::SEED_MARIGOLD
    }
}

// =========================================================================
// ★ 静态工具函数 (from Plant.cpp)
// =========================================================================
impl Plant {
    /// C++ Plant::GetCost (Plant.cpp:4973)
    pub unsafe fn GetCost(theSeedType: SeedType, theImitaterType: SeedType) -> i32 {
        let app = &mut *crate::lawn_app::G_LAWN_APP;
        let mode = app.mGameMode;
        if mode == GameMode::GAMEMODE_CHALLENGE_BEGHOULED || mode == GameMode::GAMEMODE_CHALLENGE_BEGHOULED_TWIST {
            match theSeedType {
                SeedType::SEED_REPEATER => return 1000,
                SeedType::SEED_FUMESHROOM => return 500,
                SeedType::SEED_TALLNUT => return 250,
                SeedType::SEED_BEGHOULED_BUTTON_SHUFFLE => return 100,
                SeedType::SEED_BEGHOULED_BUTTON_CRATER => return 200,
                _ => {}
            }
        }

        match theSeedType {
            SeedType::SEED_SLOT_MACHINE_SUN => 0,
            SeedType::SEED_SLOT_MACHINE_DIAMOND => 0,
            SeedType::SEED_ZOMBIQUARIUM_SNORKLE => 100,
            SeedType::SEED_ZOMBIQUARIUM_TROPHY => 1000,
            SeedType::SEED_ZOMBIE_NORMAL => 50,
            SeedType::SEED_ZOMBIE_TRAFFIC_CONE => 75,
            SeedType::SEED_ZOMBIE_POLEVAULTER => 75,
            SeedType::SEED_ZOMBIE_PAIL => 125,
            SeedType::SEED_ZOMBIE_LADDER => 150,
            SeedType::SEED_ZOMBIE_DIGGER => 125,
            SeedType::SEED_ZOMBIE_BUNGEE => 125,
            SeedType::SEED_ZOMBIE_FOOTBALL => 175,
            SeedType::SEED_ZOMBIE_BALLOON => 150,
            SeedType::SEED_ZOMBIE_SCREEN_DOOR => 100,
            SeedType::SEED_ZOMBONI => 175,
            SeedType::SEED_ZOMBIE_POGO => 200,
            SeedType::SEED_ZOMBIE_DANCER => 350,
            SeedType::SEED_ZOMBIE_GARGANTUAR => 300,
            SeedType::SEED_ZOMBIE_IMP => 50,
            _ => {
                if theSeedType == SeedType::SEED_IMITATER && theImitaterType != SeedType::SEED_NONE {
                    GetPlantDefinition(theImitaterType).mSeedCost
                } else {
                    GetPlantDefinition(theSeedType).mSeedCost
                }
            }
        }
    }

    /// C++ Plant::GetNameString (Plant.cpp:5037)
    pub unsafe fn GetNameString(theSeedType: SeedType, theImitaterType: SeedType) -> String {
        let aPlantDef = GetPlantDefinition(theSeedType);
        let aName = format!("[{}]", aPlantDef.mPlantName);
        // NOTE: TodStringTranslate 暂未翻译为 Rust, 返回原始字符串
        let aTranslatedName = aName;

        if theSeedType == SeedType::SEED_IMITATER && theImitaterType != SeedType::SEED_NONE {
            let aImitaterDef = GetPlantDefinition(theImitaterType);
            let aImitaterName = format!("[{}]", aImitaterDef.mPlantName);
            let aTranslatedImitaterName = aImitaterName;
            return format!("{} {}", aTranslatedName, aTranslatedImitaterName);
        }

        aTranslatedName
    }

    /// C++ Plant::GetToolTip (Plant.cpp:5054)
    pub unsafe fn GetToolTip(theSeedType: SeedType) -> String {
        let aPlantDef = GetPlantDefinition(theSeedType);
        format!("[{}_TOOLTIP]", aPlantDef.mPlantName)
    }

    /// C++ Plant::GetRefreshTime (Plant.cpp:5061)
    pub unsafe fn GetRefreshTime(theSeedType: SeedType, theImitaterType: SeedType) -> i32 {
        if crate::lawn::challenge::Challenge::IsZombieSeedType(theSeedType) {
            return 0;
        }

        if theSeedType == SeedType::SEED_IMITATER && theImitaterType != SeedType::SEED_NONE {
            GetPlantDefinition(theImitaterType).mRefreshTime
        } else {
            GetPlantDefinition(theSeedType).mRefreshTime
        }
    }
}

impl Default for Plant {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// ★ PlantDefinition — 植物类型定义 (from Plant.h:324)
// C++ struct PlantDefinition 的 1:1 翻译
// 注意: mPlantImage 是 Image** 类型，但所有条目均为 nullptr
// =========================================================================
#[derive(Clone, Copy)]
pub struct PlantDefinition {
    pub mSeedType: SeedType,
    pub mPlantImage: *mut *mut Image,
    pub mReanimationType: ReanimationType,
    pub mPacketIndex: i32,
    pub mSeedCost: i32,
    pub mRefreshTime: i32,
    pub mSubClass: PlantSubClass,
    pub mLaunchRate: i32,
    pub mPlantName: &'static str,
}

pub static mut G_PLANT_DEFS: [PlantDefinition; 53] = [
    // SEED_PEASHOOTER (0)
    PlantDefinition { mSeedType: SeedType::SEED_PEASHOOTER, mPlantImage: std::ptr::null_mut(), mReanimationType: ReanimationType::REANIM_PEASHOOTER, mPacketIndex: 0, mSeedCost: 100, mRefreshTime: 750, mSubClass: PlantSubClass::SUBCLASS_SHOOTER, mLaunchRate: 150, mPlantName: "PEASHOOTER" },
    // SEED_SUNFLOWER (1)
    PlantDefinition { mSeedType: SeedType::SEED_SUNFLOWER, mPlantImage: std::ptr::null_mut(), mReanimationType: ReanimationType::REANIM_SUNFLOWER, mPacketIndex: 1, mSeedCost: 50, mRefreshTime: 750, mSubClass: PlantSubClass::SUBCLASS_NORMAL, mLaunchRate: 2500, mPlantName: "SUNFLOWER" },
    // SEED_CHERRYBOMB (2)
    PlantDefinition { mSeedType: SeedType::SEED_CHERRYBOMB, mPlantImage: std::ptr::null_mut(), mReanimationType: ReanimationType::REANIM_CHERRYBOMB, mPacketIndex: 3, mSeedCost: 150, mRefreshTime: 5000, mSubClass: PlantSubClass::SUBCLASS_NORMAL, mLaunchRate: 0, mPlantName: "CHERRY_BOMB" },
    // SEED_WALLNUT (3)
    PlantDefinition { mSeedType: SeedType::SEED_WALLNUT, mPlantImage: std::ptr::null_mut(), mReanimationType: ReanimationType::REANIM_WALLNUT, mPacketIndex: 2, mSeedCost: 50, mRefreshTime: 3000, mSubClass: PlantSubClass::SUBCLASS_NORMAL, mLaunchRate: 0, mPlantName: "WALL_NUT" },
    // SEED_POTATOMINE (4)
    PlantDefinition { mSeedType: SeedType::SEED_POTATOMINE, mPlantImage: std::ptr::null_mut(), mReanimationType: ReanimationType::REANIM_POTATOMINE, mPacketIndex: 37, mSeedCost: 25, mRefreshTime: 3000, mSubClass: PlantSubClass::SUBCLASS_NORMAL, mLaunchRate: 0, mPlantName: "POTATO_MINE" },
    // SEED_SNOWPEA (5)
    PlantDefinition { mSeedType: SeedType::SEED_SNOWPEA, mPlantImage: std::ptr::null_mut(), mReanimationType: ReanimationType::REANIM_SNOWPEA, mPacketIndex: 4, mSeedCost: 175, mRefreshTime: 750, mSubClass: PlantSubClass::SUBCLASS_SHOOTER, mLaunchRate: 150, mPlantName: "SNOW_PEA" },
    // SEED_CHOMPER (6)
    PlantDefinition { mSeedType: SeedType::SEED_CHOMPER, mPlantImage: std::ptr::null_mut(), mReanimationType: ReanimationType::REANIM_CHOMPER, mPacketIndex: 31, mSeedCost: 150, mRefreshTime: 750, mSubClass: PlantSubClass::SUBCLASS_NORMAL, mLaunchRate: 0, mPlantName: "CHOMPER" },
    // SEED_REPEATER (7)
    PlantDefinition { mSeedType: SeedType::SEED_REPEATER, mPlantImage: std::ptr::null_mut(), mReanimationType: ReanimationType::REANIM_REPEATER, mPacketIndex: 5, mSeedCost: 200, mRefreshTime: 750, mSubClass: PlantSubClass::SUBCLASS_SHOOTER, mLaunchRate: 150, mPlantName: "REPEATER" },
    // SEED_PUFFSHROOM (8)
    PlantDefinition { mSeedType: SeedType::SEED_PUFFSHROOM, mPlantImage: std::ptr::null_mut(), mReanimationType: ReanimationType::REANIM_PUFFSHROOM, mPacketIndex: 6, mSeedCost: 0, mRefreshTime: 750, mSubClass: PlantSubClass::SUBCLASS_SHOOTER, mLaunchRate: 150, mPlantName: "PUFF_SHROOM" },
    // SEED_SUNSHROOM (9)
    PlantDefinition { mSeedType: SeedType::SEED_SUNSHROOM, mPlantImage: std::ptr::null_mut(), mReanimationType: ReanimationType::REANIM_SUNSHROOM, mPacketIndex: 7, mSeedCost: 25, mRefreshTime: 750, mSubClass: PlantSubClass::SUBCLASS_NORMAL, mLaunchRate: 2500, mPlantName: "SUN_SHROOM" },
    // SEED_FUMESHROOM (10)
    PlantDefinition { mSeedType: SeedType::SEED_FUMESHROOM, mPlantImage: std::ptr::null_mut(), mReanimationType: ReanimationType::REANIM_FUMESHROOM, mPacketIndex: 9, mSeedCost: 75, mRefreshTime: 750, mSubClass: PlantSubClass::SUBCLASS_SHOOTER, mLaunchRate: 150, mPlantName: "FUME_SHROOM" },
    // SEED_GRAVEBUSTER (11)
    PlantDefinition { mSeedType: SeedType::SEED_GRAVEBUSTER, mPlantImage: std::ptr::null_mut(), mReanimationType: ReanimationType::REANIM_GRAVE_BUSTER, mPacketIndex: 40, mSeedCost: 75, mRefreshTime: 750, mSubClass: PlantSubClass::SUBCLASS_NORMAL, mLaunchRate: 0, mPlantName: "GRAVE_BUSTER" },
    // SEED_HYPNOSHROOM (12)
    PlantDefinition { mSeedType: SeedType::SEED_HYPNOSHROOM, mPlantImage: std::ptr::null_mut(), mReanimationType: ReanimationType::REANIM_HYPNOSHROOM, mPacketIndex: 10, mSeedCost: 75, mRefreshTime: 3000, mSubClass: PlantSubClass::SUBCLASS_NORMAL, mLaunchRate: 0, mPlantName: "HYPNO_SHROOM" },
    // SEED_SCAREDYSHROOM (13)
    PlantDefinition { mSeedType: SeedType::SEED_SCAREDYSHROOM, mPlantImage: std::ptr::null_mut(), mReanimationType: ReanimationType::REANIM_SCRAREYSHROOM, mPacketIndex: 33, mSeedCost: 25, mRefreshTime: 750, mSubClass: PlantSubClass::SUBCLASS_SHOOTER, mLaunchRate: 150, mPlantName: "SCAREDY_SHROOM" },
    // SEED_ICESHROOM (14)
    PlantDefinition { mSeedType: SeedType::SEED_ICESHROOM, mPlantImage: std::ptr::null_mut(), mReanimationType: ReanimationType::REANIM_ICESHROOM, mPacketIndex: 36, mSeedCost: 75, mRefreshTime: 5000, mSubClass: PlantSubClass::SUBCLASS_NORMAL, mLaunchRate: 0, mPlantName: "ICE_SHROOM" },
    // SEED_DOOMSHROOM (15)
    PlantDefinition { mSeedType: SeedType::SEED_DOOMSHROOM, mPlantImage: std::ptr::null_mut(), mReanimationType: ReanimationType::REANIM_DOOMSHROOM, mPacketIndex: 20, mSeedCost: 125, mRefreshTime: 5000, mSubClass: PlantSubClass::SUBCLASS_NORMAL, mLaunchRate: 0, mPlantName: "DOOM_SHROOM" },
    // SEED_LILYPAD (16)
    PlantDefinition { mSeedType: SeedType::SEED_LILYPAD, mPlantImage: std::ptr::null_mut(), mReanimationType: ReanimationType::REANIM_LILYPAD, mPacketIndex: 19, mSeedCost: 25, mRefreshTime: 750, mSubClass: PlantSubClass::SUBCLASS_NORMAL, mLaunchRate: 0, mPlantName: "LILY_PAD" },
    // SEED_SQUASH (17)
    PlantDefinition { mSeedType: SeedType::SEED_SQUASH, mPlantImage: std::ptr::null_mut(), mReanimationType: ReanimationType::REANIM_SQUASH, mPacketIndex: 21, mSeedCost: 50, mRefreshTime: 3000, mSubClass: PlantSubClass::SUBCLASS_NORMAL, mLaunchRate: 0, mPlantName: "SQUASH" },
    // SEED_THREEPEATER (18)
    PlantDefinition { mSeedType: SeedType::SEED_THREEPEATER, mPlantImage: std::ptr::null_mut(), mReanimationType: ReanimationType::REANIM_THREEPEATER, mPacketIndex: 12, mSeedCost: 325, mRefreshTime: 750, mSubClass: PlantSubClass::SUBCLASS_SHOOTER, mLaunchRate: 150, mPlantName: "THREEPEATER" },
    // SEED_TANGLEKELP (19)
    PlantDefinition { mSeedType: SeedType::SEED_TANGLEKELP, mPlantImage: std::ptr::null_mut(), mReanimationType: ReanimationType::REANIM_TANGLEKELP, mPacketIndex: 17, mSeedCost: 25, mRefreshTime: 3000, mSubClass: PlantSubClass::SUBCLASS_NORMAL, mLaunchRate: 0, mPlantName: "TANGLE_KELP" },
    // SEED_JALAPENO (20)
    PlantDefinition { mSeedType: SeedType::SEED_JALAPENO, mPlantImage: std::ptr::null_mut(), mReanimationType: ReanimationType::REANIM_JALAPENO, mPacketIndex: 11, mSeedCost: 125, mRefreshTime: 5000, mSubClass: PlantSubClass::SUBCLASS_NORMAL, mLaunchRate: 0, mPlantName: "JALAPENO" },
    // SEED_SPIKEWEED (21)
    PlantDefinition { mSeedType: SeedType::SEED_SPIKEWEED, mPlantImage: std::ptr::null_mut(), mReanimationType: ReanimationType::REANIM_SPIKEWEED, mPacketIndex: 22, mSeedCost: 100, mRefreshTime: 750, mSubClass: PlantSubClass::SUBCLASS_NORMAL, mLaunchRate: 0, mPlantName: "SPIKEWEED" },
    // SEED_TORCHWOOD (22)
    PlantDefinition { mSeedType: SeedType::SEED_TORCHWOOD, mPlantImage: std::ptr::null_mut(), mReanimationType: ReanimationType::REANIM_TORCHWOOD, mPacketIndex: 29, mSeedCost: 175, mRefreshTime: 750, mSubClass: PlantSubClass::SUBCLASS_NORMAL, mLaunchRate: 0, mPlantName: "TORCHWOOD" },
    // SEED_TALLNUT (23)
    PlantDefinition { mSeedType: SeedType::SEED_TALLNUT, mPlantImage: std::ptr::null_mut(), mReanimationType: ReanimationType::REANIM_TALLNUT, mPacketIndex: 28, mSeedCost: 125, mRefreshTime: 3000, mSubClass: PlantSubClass::SUBCLASS_NORMAL, mLaunchRate: 0, mPlantName: "TALL_NUT" },
    // SEED_SEASHROOM (24)
    PlantDefinition { mSeedType: SeedType::SEED_SEASHROOM, mPlantImage: std::ptr::null_mut(), mReanimationType: ReanimationType::REANIM_SEASHROOM, mPacketIndex: 39, mSeedCost: 0, mRefreshTime: 3000, mSubClass: PlantSubClass::SUBCLASS_SHOOTER, mLaunchRate: 150, mPlantName: "SEA_SHROOM" },
    // SEED_PLANTERN (25)
    PlantDefinition { mSeedType: SeedType::SEED_PLANTERN, mPlantImage: std::ptr::null_mut(), mReanimationType: ReanimationType::REANIM_PLANTERN, mPacketIndex: 38, mSeedCost: 25, mRefreshTime: 3000, mSubClass: PlantSubClass::SUBCLASS_NORMAL, mLaunchRate: 2500, mPlantName: "PLANTERN" },
    // SEED_CACTUS (26)
    PlantDefinition { mSeedType: SeedType::SEED_CACTUS, mPlantImage: std::ptr::null_mut(), mReanimationType: ReanimationType::REANIM_CACTUS, mPacketIndex: 15, mSeedCost: 125, mRefreshTime: 750, mSubClass: PlantSubClass::SUBCLASS_SHOOTER, mLaunchRate: 150, mPlantName: "CACTUS" },
    // SEED_BLOVER (27)
    PlantDefinition { mSeedType: SeedType::SEED_BLOVER, mPlantImage: std::ptr::null_mut(), mReanimationType: ReanimationType::REANIM_BLOVER, mPacketIndex: 18, mSeedCost: 100, mRefreshTime: 750, mSubClass: PlantSubClass::SUBCLASS_NORMAL, mLaunchRate: 0, mPlantName: "BLOVER" },
    // SEED_SPLITPEA (28)
    PlantDefinition { mSeedType: SeedType::SEED_SPLITPEA, mPlantImage: std::ptr::null_mut(), mReanimationType: ReanimationType::REANIM_SPLITPEA, mPacketIndex: 32, mSeedCost: 125, mRefreshTime: 750, mSubClass: PlantSubClass::SUBCLASS_SHOOTER, mLaunchRate: 150, mPlantName: "SPLIT_PEA" },
    // SEED_STARFRUIT (29)
    PlantDefinition { mSeedType: SeedType::SEED_STARFRUIT, mPlantImage: std::ptr::null_mut(), mReanimationType: ReanimationType::REANIM_STARFRUIT, mPacketIndex: 30, mSeedCost: 125, mRefreshTime: 750, mSubClass: PlantSubClass::SUBCLASS_SHOOTER, mLaunchRate: 150, mPlantName: "STARFRUIT" },
    // SEED_PUMPKINSHELL (30)
    PlantDefinition { mSeedType: SeedType::SEED_PUMPKINSHELL, mPlantImage: std::ptr::null_mut(), mReanimationType: ReanimationType::REANIM_PUMPKIN, mPacketIndex: 25, mSeedCost: 125, mRefreshTime: 3000, mSubClass: PlantSubClass::SUBCLASS_NORMAL, mLaunchRate: 0, mPlantName: "PUMPKIN" },
    // SEED_MAGNETSHROOM (31)
    PlantDefinition { mSeedType: SeedType::SEED_MAGNETSHROOM, mPlantImage: std::ptr::null_mut(), mReanimationType: ReanimationType::REANIM_MAGNETSHROOM, mPacketIndex: 35, mSeedCost: 100, mRefreshTime: 750, mSubClass: PlantSubClass::SUBCLASS_NORMAL, mLaunchRate: 0, mPlantName: "MAGNET_SHROOM" },
    // SEED_CABBAGEPULT (32)
    PlantDefinition { mSeedType: SeedType::SEED_CABBAGEPULT, mPlantImage: std::ptr::null_mut(), mReanimationType: ReanimationType::REANIM_CABBAGEPULT, mPacketIndex: 13, mSeedCost: 100, mRefreshTime: 750, mSubClass: PlantSubClass::SUBCLASS_SHOOTER, mLaunchRate: 300, mPlantName: "CABBAGE_PULT" },
    // SEED_FLOWERPOT (33)
    PlantDefinition { mSeedType: SeedType::SEED_FLOWERPOT, mPlantImage: std::ptr::null_mut(), mReanimationType: ReanimationType::REANIM_FLOWER_POT, mPacketIndex: 33, mSeedCost: 25, mRefreshTime: 750, mSubClass: PlantSubClass::SUBCLASS_NORMAL, mLaunchRate: 0, mPlantName: "FLOWER_POT" },
    // SEED_KERNELPULT (34)
    PlantDefinition { mSeedType: SeedType::SEED_KERNELPULT, mPlantImage: std::ptr::null_mut(), mReanimationType: ReanimationType::REANIM_KERNELPULT, mPacketIndex: 13, mSeedCost: 100, mRefreshTime: 750, mSubClass: PlantSubClass::SUBCLASS_SHOOTER, mLaunchRate: 300, mPlantName: "KERNEL_PULT" },
    // SEED_INSTANT_COFFEE (35)
    PlantDefinition { mSeedType: SeedType::SEED_INSTANT_COFFEE, mPlantImage: std::ptr::null_mut(), mReanimationType: ReanimationType::REANIM_COFFEEBEAN, mPacketIndex: 33, mSeedCost: 75, mRefreshTime: 750, mSubClass: PlantSubClass::SUBCLASS_NORMAL, mLaunchRate: 0, mPlantName: "COFFEE_BEAN" },
    // SEED_GARLIC (36)
    PlantDefinition { mSeedType: SeedType::SEED_GARLIC, mPlantImage: std::ptr::null_mut(), mReanimationType: ReanimationType::REANIM_GARLIC, mPacketIndex: 8, mSeedCost: 50, mRefreshTime: 750, mSubClass: PlantSubClass::SUBCLASS_NORMAL, mLaunchRate: 0, mPlantName: "GARLIC" },
    // SEED_UMBRELLA (37)
    PlantDefinition { mSeedType: SeedType::SEED_UMBRELLA, mPlantImage: std::ptr::null_mut(), mReanimationType: ReanimationType::REANIM_UMBRELLALEAF, mPacketIndex: 23, mSeedCost: 100, mRefreshTime: 750, mSubClass: PlantSubClass::SUBCLASS_NORMAL, mLaunchRate: 0, mPlantName: "UMBRELLA_LEAF" },
    // SEED_MARIGOLD (38)
    PlantDefinition { mSeedType: SeedType::SEED_MARIGOLD, mPlantImage: std::ptr::null_mut(), mReanimationType: ReanimationType::REANIM_MARIGOLD, mPacketIndex: 24, mSeedCost: 50, mRefreshTime: 3000, mSubClass: PlantSubClass::SUBCLASS_NORMAL, mLaunchRate: 2500, mPlantName: "MARIGOLD" },
    // SEED_MELONPULT (39)
    PlantDefinition { mSeedType: SeedType::SEED_MELONPULT, mPlantImage: std::ptr::null_mut(), mReanimationType: ReanimationType::REANIM_MELONPULT, mPacketIndex: 14, mSeedCost: 300, mRefreshTime: 750, mSubClass: PlantSubClass::SUBCLASS_SHOOTER, mLaunchRate: 300, mPlantName: "MELON_PULT" },
    // SEED_GATLINGPEA (40)
    PlantDefinition { mSeedType: SeedType::SEED_GATLINGPEA, mPlantImage: std::ptr::null_mut(), mReanimationType: ReanimationType::REANIM_GATLINGPEA, mPacketIndex: 5, mSeedCost: 250, mRefreshTime: 5000, mSubClass: PlantSubClass::SUBCLASS_SHOOTER, mLaunchRate: 150, mPlantName: "GATLING_PEA" },
    // SEED_TWINSUNFLOWER (41)
    PlantDefinition { mSeedType: SeedType::SEED_TWINSUNFLOWER, mPlantImage: std::ptr::null_mut(), mReanimationType: ReanimationType::REANIM_TWIN_SUNFLOWER, mPacketIndex: 1, mSeedCost: 150, mRefreshTime: 5000, mSubClass: PlantSubClass::SUBCLASS_NORMAL, mLaunchRate: 2500, mPlantName: "TWIN_SUNFLOWER" },
    // SEED_GLOOMSHROOM (42)
    PlantDefinition { mSeedType: SeedType::SEED_GLOOMSHROOM, mPlantImage: std::ptr::null_mut(), mReanimationType: ReanimationType::REANIM_GLOOMSHROOM, mPacketIndex: 27, mSeedCost: 150, mRefreshTime: 5000, mSubClass: PlantSubClass::SUBCLASS_SHOOTER, mLaunchRate: 200, mPlantName: "GLOOM_SHROOM" },
    // SEED_CATTAIL (43)
    PlantDefinition { mSeedType: SeedType::SEED_CATTAIL, mPlantImage: std::ptr::null_mut(), mReanimationType: ReanimationType::REANIM_CATTAIL, mPacketIndex: 27, mSeedCost: 225, mRefreshTime: 5000, mSubClass: PlantSubClass::SUBCLASS_SHOOTER, mLaunchRate: 150, mPlantName: "CATTAIL" },
    // SEED_WINTERMELON (44)
    PlantDefinition { mSeedType: SeedType::SEED_WINTERMELON, mPlantImage: std::ptr::null_mut(), mReanimationType: ReanimationType::REANIM_WINTER_MELON, mPacketIndex: 27, mSeedCost: 200, mRefreshTime: 5000, mSubClass: PlantSubClass::SUBCLASS_SHOOTER, mLaunchRate: 300, mPlantName: "WINTER_MELON" },
    // SEED_GOLD_MAGNET (45)
    PlantDefinition { mSeedType: SeedType::SEED_GOLD_MAGNET, mPlantImage: std::ptr::null_mut(), mReanimationType: ReanimationType::REANIM_GOLD_MAGNET, mPacketIndex: 27, mSeedCost: 50, mRefreshTime: 5000, mSubClass: PlantSubClass::SUBCLASS_NORMAL, mLaunchRate: 0, mPlantName: "GOLD_MAGNET" },
    // SEED_SPIKEROCK (46)
    PlantDefinition { mSeedType: SeedType::SEED_SPIKEROCK, mPlantImage: std::ptr::null_mut(), mReanimationType: ReanimationType::REANIM_SPIKEROCK, mPacketIndex: 27, mSeedCost: 125, mRefreshTime: 5000, mSubClass: PlantSubClass::SUBCLASS_NORMAL, mLaunchRate: 0, mPlantName: "SPIKEROCK" },
    // SEED_COBCANNON (47)
    PlantDefinition { mSeedType: SeedType::SEED_COBCANNON, mPlantImage: std::ptr::null_mut(), mReanimationType: ReanimationType::REANIM_COBCANNON, mPacketIndex: 16, mSeedCost: 500, mRefreshTime: 5000, mSubClass: PlantSubClass::SUBCLASS_NORMAL, mLaunchRate: 600, mPlantName: "COB_CANNON" },
    // SEED_IMITATER (48)
    PlantDefinition { mSeedType: SeedType::SEED_IMITATER, mPlantImage: std::ptr::null_mut(), mReanimationType: ReanimationType::REANIM_IMITATER, mPacketIndex: 33, mSeedCost: 0, mRefreshTime: 750, mSubClass: PlantSubClass::SUBCLASS_NORMAL, mLaunchRate: 0, mPlantName: "IMITATER" },
    // SEED_EXPLODE_O_NUT (49)
    PlantDefinition { mSeedType: SeedType::SEED_EXPLODE_O_NUT, mPlantImage: std::ptr::null_mut(), mReanimationType: ReanimationType::REANIM_WALLNUT, mPacketIndex: 2, mSeedCost: 0, mRefreshTime: 3000, mSubClass: PlantSubClass::SUBCLASS_NORMAL, mLaunchRate: 0, mPlantName: "EXPLODE_O_NUT" },
    // SEED_GIANT_WALLNUT (50)
    PlantDefinition { mSeedType: SeedType::SEED_GIANT_WALLNUT, mPlantImage: std::ptr::null_mut(), mReanimationType: ReanimationType::REANIM_WALLNUT, mPacketIndex: 2, mSeedCost: 0, mRefreshTime: 3000, mSubClass: PlantSubClass::SUBCLASS_NORMAL, mLaunchRate: 0, mPlantName: "GIANT_WALLNUT" },
    // SEED_SPROUT (51)
    PlantDefinition { mSeedType: SeedType::SEED_SPROUT, mPlantImage: std::ptr::null_mut(), mReanimationType: ReanimationType::REANIM_ZENGARDEN_SPROUT, mPacketIndex: 33, mSeedCost: 0, mRefreshTime: 3000, mSubClass: PlantSubClass::SUBCLASS_NORMAL, mLaunchRate: 0, mPlantName: "SPROUT" },
    // SEED_LEFTPEATER (52)
    PlantDefinition { mSeedType: SeedType::SEED_LEFTPEATER, mPlantImage: std::ptr::null_mut(), mReanimationType: ReanimationType::REANIM_REPEATER, mPacketIndex: 5, mSeedCost: 200, mRefreshTime: 750, mSubClass: PlantSubClass::SUBCLASS_SHOOTER, mLaunchRate: 150, mPlantName: "REPEATER" },
];

pub fn GetPlantDefinition(theSeedType: SeedType) -> &'static PlantDefinition {
    unsafe { &G_PLANT_DEFS[theSeedType as usize] }
}

// =========================================================================
// ★ Plant 游戏逻辑核心方法
// =========================================================================

impl Plant {
    unsafe fn board(&self) -> &'static mut super::board::Board {
        &mut *(self.base.m_board as *mut super::board::Board)
    }

    unsafe fn app(&self) -> &'static mut crate::lawn_app::LawnApp {
        &mut *(self.base.m_app as *mut crate::lawn_app::LawnApp)
    }

    /// C++ Plant::Update() — 主更新 (from Plant.cpp line 2853)
    /// C++ Plant::DoSpecial (Plant.cpp:4290) — 特殊植物激活
    pub unsafe fn DoSpecial(&mut self) {
        // [TODO]: 完整 DoSpecial（樱桃/辣椒/毁灭菇/冰菇/土豆雷/三叶草等 20+ 分支）
    }

    /// C++ Plant::RemoveEffects (Plant.cpp:2303) — 移除植物粒子与动画
    pub unsafe fn RemoveEffects(&mut self) {
        // C++: mApp->RemoveParticle(mParticleID);
        // C++: mApp->RemoveReanimation(mBodyReanimID/mHeadReanimID/mHeadReanimID2/mHeadReanimID3/mLightReanimID/mBlinkReanimID/mSleepingReanimID);
        // [TODO]: 粒子/Reanimation 移除（待 EffectSystem 完整翻译）
    }

    /// C++ Plant::Squish (Plant.cpp:2315) — 植物被压扁
    pub unsafe fn Squish(&mut self) {
        if self.NotOnGround() {
            return;
        }

        // C++: 未入睡的爆炸/特殊植物被压时直接触发
        if !self.m_is_asleep {
            if self.m_seed_type == SeedType::SEED_CHERRYBOMB
                || self.m_seed_type == SeedType::SEED_JALAPENO
                || self.m_seed_type == SeedType::SEED_DOOMSHROOM
                || self.m_seed_type == SeedType::SEED_ICESHROOM
            {
                self.DoSpecial();
                return;
            } else if self.m_seed_type == SeedType::SEED_POTATOMINE
                && self.m_state != PlantState::STATE_NOTREADY
            {
                self.DoSpecial();
                return;
            }
        }

        // C++: 准备就绪的倭瓜被压时不处理
        if self.m_seed_type == SeedType::SEED_SQUASH && self.m_state != PlantState::STATE_NOTREADY {
            return;
        }

        // C++: mRenderOrder = MakeRenderOrder(RENDER_LAYER_GRAVE_STONE, mRow, 8);
        self.base.m_render_order = super::board::Board::MakeRenderOrder(
            RenderLayer::RENDER_LAYER_GRAVE_STONE,
            self.base.m_row,
            8,
        );
        self.m_squished = true;
        self.m_disappear_countdown = 500;
        self.app().PlayFoley(crate::sexy_tod_lib::tod_foley::FoleyType::FOLEY_SQUISH);
        self.RemoveEffects();

        // C++: GridItem* aLadder = mBoard->GetLadderAt(mPlantCol, mRow); if (aLadder) aLadder->GridItemDie();
        let the_board = self.board();
        let a_ladder = the_board.GetLadderAt(self.m_plant_col, self.base.m_row);
        if !a_ladder.is_null() {
            (*a_ladder).GridItemDie();
        }

        // C++: if (mApp->IsIZombieLevel()) mBoard->mChallenge->IZombiePlantDropRemainingSun(this);
        if self.app().IsIZombieLevel() {
            // [TODO]: mChallenge->IZombiePlantDropRemainingSun(this)
        }
    }

    /// C++ Plant::KillAllPlantsNearDoom (Plant.cpp:4277) — 毁灭菇清除同格植物
    /// C++ Plant::SetSleeping — 设置睡眠状态
    pub unsafe fn SetSleeping(&mut self, the_is_sleeping: bool) {
        self.m_is_asleep = the_is_sleeping;
    }

    /// C++ Plant::AddAttachedParticle — 附加粒子
    pub unsafe fn AddAttachedParticle(&mut self, _pos_x: i32, _pos_y: i32, _render_order: i32, _effect: ParticleEffect) -> *mut std::ffi::c_void {
        // [TODO]: 粒子系统（mParticleID 创建）
        std::ptr::null_mut()
    }
    pub unsafe fn KillAllPlantsNearDoom(&mut self) {
        let mut a_plant: *mut Plant = std::ptr::null_mut();
        let the_board = self.board();
        while the_board.IteratePlants(&mut a_plant) {
            if (*a_plant).base.m_row == self.base.m_row && (*a_plant).m_plant_col == self.m_plant_col {
                (*a_plant).Die();
            }
        }
    }
    /// C++ Plant::GetDamageRangeFlags (Plant.cpp:603) — 植物伤害范围标志
    pub fn GetDamageRangeFlags(&self, the_plant_weapon: PlantWeapon) -> u32 {
        // C++: switch (mSeedType) — 位标志对应 DamageRangeFlags 枚举
        match self.m_seed_type {
            SeedType::SEED_CACTUS => {
                // C++: return thePlantWeapon == WEAPON_SECONDARY ? 1 : 2;
                if the_plant_weapon == PlantWeapon::WEAPON_SECONDARY {
                    1 << DamageRangeFlags::DAMAGES_GROUND as i32
                } else {
                    1 << DamageRangeFlags::DAMAGES_FLYING as i32
                }
            }
            SeedType::SEED_CHERRYBOMB | SeedType::SEED_JALAPENO | SeedType::SEED_COBCANNON
            | SeedType::SEED_DOOMSHROOM => {
                // C++: return 127; (除 MINDCONTROLLED 外的全部位)
                (1 << DamageRangeFlags::DAMAGES_GROUND as i32)
                    | (1 << DamageRangeFlags::DAMAGES_FLYING as i32)
                    | (1 << DamageRangeFlags::DAMAGES_SUBMERGED as i32)
                    | (1 << DamageRangeFlags::DAMAGES_DOG as i32)
                    | (1 << DamageRangeFlags::DAMAGES_OFF_GROUND as i32)
                    | (1 << DamageRangeFlags::DAMAGES_DYING as i32)
                    | (1 << DamageRangeFlags::DAMAGES_UNDERGROUND as i32)
            }
            SeedType::SEED_MELONPULT | SeedType::SEED_CABBAGEPULT | SeedType::SEED_KERNELPULT
            | SeedType::SEED_WINTERMELON | SeedType::SEED_SQUASH => {
                // C++: return 13; (GROUND|SUBMERGED|DOG)
                (1 << DamageRangeFlags::DAMAGES_GROUND as i32)
                    | (1 << DamageRangeFlags::DAMAGES_SUBMERGED as i32)
                    | (1 << DamageRangeFlags::DAMAGES_DOG as i32)
            }
            SeedType::SEED_POTATOMINE => {
                // C++: return 77; (GROUND|SUBMERGED|DOG|UNDERGROUND)
                (1 << DamageRangeFlags::DAMAGES_GROUND as i32)
                    | (1 << DamageRangeFlags::DAMAGES_SUBMERGED as i32)
                    | (1 << DamageRangeFlags::DAMAGES_DOG as i32)
                    | (1 << DamageRangeFlags::DAMAGES_UNDERGROUND as i32)
            }
            SeedType::SEED_PUFFSHROOM | SeedType::SEED_SEASHROOM | SeedType::SEED_FUMESHROOM
            | SeedType::SEED_GLOOMSHROOM | SeedType::SEED_CHOMPER => {
                // C++: return 9; (GROUND|DOG)
                (1 << DamageRangeFlags::DAMAGES_GROUND as i32)
                    | (1 << DamageRangeFlags::DAMAGES_DOG as i32)
            }
            SeedType::SEED_CATTAIL => {
                // C++: return 11; (GROUND|FLYING|DOG)
                (1 << DamageRangeFlags::DAMAGES_GROUND as i32)
                    | (1 << DamageRangeFlags::DAMAGES_FLYING as i32)
                    | (1 << DamageRangeFlags::DAMAGES_DOG as i32)
            }
            SeedType::SEED_TANGLEKELP => {
                // C++: return 5; (GROUND|SUBMERGED)
                (1 << DamageRangeFlags::DAMAGES_GROUND as i32)
                    | (1 << DamageRangeFlags::DAMAGES_SUBMERGED as i32)
            }
            SeedType::SEED_GIANT_WALLNUT => {
                // C++: return 17; (GROUND|OFF_GROUND)
                (1 << DamageRangeFlags::DAMAGES_GROUND as i32)
                    | (1 << DamageRangeFlags::DAMAGES_OFF_GROUND as i32)
            }
            _ => {
                // C++: default: return 1; (GROUND)
                1 << DamageRangeFlags::DAMAGES_GROUND as i32
            }
        }
    }

    /// C++ Plant::PlayBodyReanim (Plant.cpp:1129) — 播放主体动画
    pub unsafe fn PlayBodyReanim(&mut self, _the_track_name: &str, _the_loop_type: crate::sexy_tod_lib::reanimator::ReanimLoopType, _the_blend_time: i32, _the_anim_rate: f32) {
        // [TODO]: Reanimation 播放（StartBlend/mAnimRate/mLoopType/SetFramesForLayer）
    }

    /// C++ Plant::UpdatePotato (Plant.cpp:1143) — 土豆雷更新
    pub unsafe fn UpdatePotato(&mut self) {
        if self.m_state == PlantState::STATE_NOTREADY {
            if self.m_state_countdown == 0 {
                // [TODO]: AddTodParticle(PARTICLE_POTATO_MINE_RISE); PlayBodyReanim("anim_rise", ...)
                self.m_state = PlantState::STATE_POTATO_RISING;
                // [TODO]: mApp->PlayFoley(FOLEY_DIRT_RISE)
            }
        } else if self.m_state == PlantState::STATE_POTATO_RISING {
            // C++: if (aBodyReanim->mLoopCount > 0)
            // [TODO]: Reanimation mLoopCount 检查
            {
                // C++: float aRate = RandRangeFloat(12.0f, 15.0f);
                // [TODO]: PlayBodyReanim("anim_armed", ...); 发光动画创建
                self.m_state = PlantState::STATE_POTATO_ARMED;
                self.m_blink_countdown = 400 + crate::sexy_app_framework::common::rand_int() % 4000;
            }
        } else if self.m_state == PlantState::STATE_POTATO_ARMED {
            if !self.FindTargetZombie(self.base.m_row, PlantWeapon::WEAPON_PRIMARY).is_null() {
                self.DoSpecial();
            } else {
                // [TODO]: 发光动画帧数随僵尸距离变化（TodAnimateCurve）
            }
        }
    }

    /// C++ Plant::UpdateSunShroom (Plant.cpp:1073) — 阳光菇更新
    pub unsafe fn UpdateSunShroom(&mut self) {
        if self.m_state == PlantState::STATE_SUNSHROOM_SMALL {
            if self.m_state_countdown == 0 {
                // [TODO]: PlayBodyReanim("anim_grow", ...)
                self.m_state = PlantState::STATE_SUNSHROOM_GROWING;
                // [TODO]: mApp->PlayFoley(FOLEY_PLANTGROW)
            }

            self.UpdateProductionPlant();
        } else if self.m_state == PlantState::STATE_SUNSHROOM_GROWING {
            // C++: if (aBodyReanim->mLoopCount > 0)
            // [TODO]: Reanimation mLoopCount 检查
            {
                // [TODO]: PlayBodyReanim("anim_bigidle", ...)
                self.m_state = PlantState::STATE_SUNSHROOM_BIG;
            }
        } else {
            self.UpdateProductionPlant();
        }
    }

    /// C++ Plant::UpdateGraveBuster (Plant.cpp:1101) — 墓碑吞噬者更新
    pub unsafe fn UpdateGraveBuster(&mut self) {
        if self.m_state == PlantState::STATE_GRAVEBUSTER_LANDING {
            // C++: if (mApp->ReanimationGet(mBodyReanimID)->mLoopCount > 0)
            // [TODO]: Reanimation mLoopCount 检查
            {
                // [TODO]: PlayBodyReanim("anim_idle", ...)
                self.m_state_countdown = 400;
                self.m_state = PlantState::STATE_GRAVEBUSTER_EATING;
                // [TODO]: AddAttachedParticle(mX + 40, mY + 40, PARTICLE_GRAVE_BUSTER)
            }
        } else if self.m_state == PlantState::STATE_GRAVEBUSTER_EATING && self.m_state_countdown == 0 {
            // C++: GridItem* aGraveStone = mBoard->GetGraveStoneAt(mPlantCol, mRow);
            let the_board = self.board();
            let a_grave_stone = the_board.GetGraveStoneAt(self.m_plant_col, self.base.m_row);
            if !a_grave_stone.is_null() {
                (*a_grave_stone).GridItemDie();
                the_board.mGravesCleared += 1;
            }

            // [TODO]: AddTodParticle(PARTICLE_GRAVE_BUSTER_DIE)
            self.Die();
            // [TODO]: mBoard->DropLootPiece(mX + 40, mY, 12)
        }
    }

    /// C++ Plant::FindTargetZombie (Plant.cpp:4769) — 寻找攻击目标
    pub unsafe fn FindTargetZombie(&self, the_row: i32, the_plant_weapon: PlantWeapon) -> *mut super::zombie::Zombie {
        let a_damage_range_flags = self.GetDamageRangeFlags(the_plant_weapon);
        let mut a_attack_rect = self.GetPlantAttackRect(the_plant_weapon);
        let mut a_highest_weight = 0;
        let mut a_best_zombie: *mut super::zombie::Zombie = std::ptr::null_mut();

        let the_board = self.board();
        let mut a_zombie: *mut super::zombie::Zombie = std::ptr::null_mut();
        while the_board.IterateZombies(&mut a_zombie) {
            let mut a_row_deviation = (*a_zombie).base.m_row - the_row;
            if (*a_zombie).m_zombie_type == ZombieType::ZOMBIE_BOSS {
                a_row_deviation = 0;
            }

            if !(*a_zombie).m_has_head || (*a_zombie).IsTangleKelpTarget() {
                if self.m_seed_type == SeedType::SEED_POTATOMINE
                    || self.m_seed_type == SeedType::SEED_CHOMPER
                    || self.m_seed_type == SeedType::SEED_TANGLEKELP
                {
                    continue;
                }
            }

            // [TODO]: PORTAL_COMBAT 模式下 needPortalCheck（PEASHOOTER/CACTUS/REPEATER）
            let need_portal_check = false;

            if self.m_seed_type != SeedType::SEED_CATTAIL {
                if self.m_seed_type == SeedType::SEED_GLOOMSHROOM {
                    if a_row_deviation < -1 || a_row_deviation > 1 {
                        continue;
                    }
                } else if need_portal_check {
                    // [TODO]: mBoard->mChallenge->CanTargetZombieWithPortals(this, aZombie)
                } else if a_row_deviation != 0 {
                    continue;
                }
            }

            if (*a_zombie).EffectedByDamage(a_damage_range_flags) {
                let mut a_extra_range = 0;

                if self.m_seed_type == SeedType::SEED_CHOMPER {
                    if (*a_zombie).m_zombie_phase == ZombiePhase::PHASE_DIGGER_WALKING {
                        a_attack_rect.m_x += 20;
                        a_attack_rect.m_width -= 20;
                    }

                    if (*a_zombie).m_zombie_phase == ZombiePhase::PHASE_POGO_BOUNCING
                        || ((*a_zombie).m_zombie_type == ZombieType::ZOMBIE_BUNGEE
                            && (*a_zombie).m_target_col == self.m_plant_col)
                    {
                        continue;
                    }

                    if (*a_zombie).m_is_eating || self.m_state == PlantState::STATE_CHOMPER_BITING {
                        a_extra_range = 60;
                    }
                }

                if self.m_seed_type == SeedType::SEED_POTATOMINE {
                    if ((*a_zombie).m_zombie_type == ZombieType::ZOMBIE_POGO && (*a_zombie).m_has_object)
                        || (*a_zombie).m_zombie_phase == ZombiePhase::PHASE_POLEVAULTER_IN_VAULT
                        || (*a_zombie).m_zombie_phase == ZombiePhase::PHASE_POLEVAULTER_PRE_VAULT
                    {
                        continue;
                    }

                    if (*a_zombie).m_zombie_type == ZombieType::ZOMBIE_POLEVAULTER {
                        a_attack_rect.m_x += 40;
                        a_attack_rect.m_width -= 40;
                    }

                    if (*a_zombie).m_zombie_type == ZombieType::ZOMBIE_BUNGEE
                        && (*a_zombie).m_target_col != self.m_plant_col
                    {
                        continue;
                    }

                    if (*a_zombie).m_is_eating {
                        a_extra_range = 30;
                    }
                }

                if (self.m_seed_type == SeedType::SEED_EXPLODE_O_NUT
                    && (*a_zombie).m_zombie_phase == ZombiePhase::PHASE_POLEVAULTER_IN_VAULT)
                    || (self.m_seed_type == SeedType::SEED_TANGLEKELP && !(*a_zombie).m_in_pool)
                {
                    continue;
                }

                let a_zombie_rect = (*a_zombie).GetZombieRect();
                if !need_portal_check && self.GetRectOverlapRect(a_attack_rect, a_zombie_rect) < -a_extra_range {
                    continue;
                }

                let mut a_weight = -a_zombie_rect.m_x;
                if self.m_seed_type == SeedType::SEED_CATTAIL {
                    a_weight = -crate::sexy_tod_lib::tod_common::distance_2d(
                        self.base.m_x as f32 + 40.0,
                        self.base.m_y as f32 + 40.0,
                        (a_zombie_rect.m_x + a_zombie_rect.m_width / 2) as f32,
                        (a_zombie_rect.m_y + a_zombie_rect.m_height / 2) as f32,
                    ) as i32;
                    if (*a_zombie).IsFlying() {
                        a_weight += 10000;
                    }
                }

                if a_best_zombie.is_null() || a_weight > a_highest_weight {
                    a_highest_weight = a_weight;
                    a_best_zombie = a_zombie;
                }
            }
        }

        a_best_zombie
    }

    /// C++ Plant::BurnRow (Plant.cpp:4226) — 烧毁整行僵尸
    pub unsafe fn BurnRow(&mut self, the_row: i32) {
        let a_damage_range_flags = self.GetDamageRangeFlags(PlantWeapon::WEAPON_PRIMARY);

        let the_board = self.board();
        let mut a_zombie: *mut super::zombie::Zombie = std::ptr::null_mut();
        while the_board.IterateZombies(&mut a_zombie) {
            // C++: if ((aZombie->mZombieType == ZOMBIE_BOSS || aZombie->mRow == theRow) && aZombie->EffectedByDamage(...))
            if ((*a_zombie).m_zombie_type == ZombieType::ZOMBIE_BOSS || (*a_zombie).base.m_row == the_row)
                && (*a_zombie).EffectedByDamage(a_damage_range_flags)
            {
                (*a_zombie).RemoveColdEffects();
                (*a_zombie).ApplyBurn();
            }
        }

        // C++: 烧毁该行梯子
        let mut a_grid_item: *mut crate::lawn::grid_item::GridItem = std::ptr::null_mut();
        while the_board.IterateGridItems(&mut a_grid_item) {
            if (*a_grid_item).mGridY == the_row && (*a_grid_item).mGridItemType == GridItemType::GRIDITEM_LADDER {
                (*a_grid_item).GridItemDie();
            }
        }

        // C++: Boss 冰球摧毁（若冰球在该行）
        let a_boss_zombie = the_board.GetBossZombie();
        if !a_boss_zombie.is_null() {
            // [TODO]: aBossZombie->mFireballRow == theRow 时 BossDestroyIceballInRow()
        }
    }

    /// C++ Plant::IceZombies (Plant.cpp:4204) — 冰冻全场僵尸
    pub unsafe fn IceZombies(&mut self) {
        let the_board = self.board();
        let mut a_zombie: *mut super::zombie::Zombie = std::ptr::null_mut();
        while the_board.IterateZombies(&mut a_zombie) {
            (*a_zombie).HitIceTrap();
        }

        // C++: mBoard->mIceTrapCounter = 300;
        the_board.mIceTrapCounter = 300;
        // [TODO]: 池面闪光粒子恢复（mPoolSparklyParticleID）

        // C++: Boss 火焰球摧毁
        let a_boss_zombie = the_board.GetBossZombie();
        if !a_boss_zombie.is_null() {
            // [TODO]: aBossZombie->BossDestroyFireball()
        }
    }
    pub unsafe fn Update(&mut self) {
        let mut do_update = false;
        let board = self.board();
        let app = self.app();

        if self.base.m_visible && app.mGameScene as i32 == GameScenes::SCENE_LEVEL_INTRO as i32
            && app.is_wallnut_bowling_level()
        {
            do_update = true;
        } else if self.base.m_visible
            && app.mGameMode as i32 == GameMode::GAMEMODE_CHALLENGE_ZEN_GARDEN as i32
        {
            do_update = true;
        } else if self.base.m_visible && !board.mCutScene.is_null()
            && (*board.mCutScene).ShouldRunUpsellBoard()
        {
            do_update = true;
        } else if !self.base.m_visible
            || app.mGameScene as i32 == GameScenes::SCENE_PLAYING as i32
        {
            do_update = true;
        }

        if do_update {
            self.UpdateAbilities();
            self.Animate();

            if self.m_plant_health < 0 {
                self.Die();
            }

            self.UpdateReanim();
        }
    }

    /// C++ Plant::BeginDraw — 绘制前准备
    pub unsafe fn BeginDraw(&self, g: &mut Graphics) -> bool {
        if !self.base.m_visible || self.m_dead {
            return false;
        }
        self.base.begin_draw(g)
    }

    pub unsafe fn EndDraw(&self, g: &mut Graphics) {
        self.base.end_draw(g);
    }

    /// C++ Plant::Draw (Plant.cpp:3937)
    pub unsafe fn Draw(&self, _g: &mut Graphics) {
        let _aOffsetX = 0.0f32;
        let mut aOffsetY = 0.0f32; // [TODO]: PlantDrawHeightOffset(mBoard, this, mSeedType, mPlantCol, mRow)
        if Plant::is_flying(self.m_seed_type) && self.m_squished {
            aOffsetY += 30.0;
        }

        let _aImageIndex = self.m_frame;

        if self.m_squished {
            if self.m_seed_type == SeedType::SEED_FLOWERPOT {
                aOffsetY -= 15.0;
            }
            if self.m_seed_type == SeedType::SEED_INSTANT_COFFEE {
                aOffsetY -= 20.0;
            }

            // [TODO]: g.SetScale(1.0, 0.25, 0, 0)
            // [TODO]: DrawSeedType(g, mSeedType, mImitaterType, VARIATION_NORMAL, aOffsetX, 60 + aOffsetY)
            // [TODO]: g.SetScale(1.0, 1.0, 0, 0)
        } else {
            // 南瓜壳后层绘制
            let _aDrawPumpkinBack = false;
            // [TODO]: GetPumpkinAt / GetTopPlantAt logic

            // [TODO]: DrawShadow(g, aOffsetX, aOffsetY)

            // 飞行植物浮动
            if Plant::is_flying(self.m_seed_type) && self.IsOnBoard() {
                // [TODO]: sin-wave floating animation
            }

            // 主绘制分支
            if self.IsOnBoard() && self.app().IsIZombieLevel() {
                // [TODO]: mBoard->mChallenge->IZombieDrawPlant(g, this)
            } else if self.m_body_reanim_id != ReanimationID::REANIMATIONID_NULL {
                // [TODO]: Reanimation* aBodyReanim = mApp->ReanimationTryToGet(mBodyReanimID)
                // [TODO]: aBodyReanim->Draw(g)
            } else {
                // 基于精灵表的绘制
                // [TODO]: g.SetColorizeImages / SetColor for upgradable/highlight
                // [TODO]: TodDrawImageCelF(g, aPlantImage, aOffsetX, aOffsetY, aImageIndex, 0)
            }

            // 磁铁物品绘制
            if self.m_seed_type == SeedType::SEED_MAGNETSHROOM {
                // [TODO]: DrawMagnetItems(g)
            }
        }
    }

    /// C++ Plant::DrawShadow (Plant.cpp:3808)
    pub unsafe fn DrawShadow(&self, _g: &mut Graphics, _theOffsetX: f32, _theOffsetY: f32) {
        // [TODO]: Draw shadow based on seed type
        // TodDrawImageCelF(g, IMAGE_PLANTSHADOW, ...)
    }

    pub unsafe fn DrawMagnetItems(&self, _g: &mut Graphics) {
        // [TODO]: Draw magnet items above plant
    }

    /// C++ Plant::Fire (Plant.cpp:4475)
    pub unsafe fn Fire(&mut self, _theTargetZombie: *mut super::zombie::Zombie, theRow: i32, thePlantWeapon: PlantWeapon) {
        // 烟雾蘑菇 / 忧郁蘑菇 — 范围伤害，不发射弹丸
        if self.m_seed_type == SeedType::SEED_FUMESHROOM {
            // [TODO]: DoRowAreaDamage(20, 2U);
            self.app().PlayFoley(FoleyType::FOLEY_FUME);
            return;
        }
        if self.m_seed_type == SeedType::SEED_GLOOMSHROOM {
            // [TODO]: DoRowAreaDamage(20, 2U);
            return;
        }
        // 星星果 — 特殊弹射
        if self.m_seed_type == SeedType::SEED_STARFRUIT {
            // [TODO]: StarFruitFire();
            return;
        }

        // 确定弹丸类型
        let aProjectileType = match self.m_seed_type {
            SeedType::SEED_PEASHOOTER | SeedType::SEED_REPEATER | SeedType::SEED_THREEPEATER
            | SeedType::SEED_SPLITPEA | SeedType::SEED_GATLINGPEA | SeedType::SEED_LEFTPEATER
                => ProjectileType::PROJECTILE_PEA,
            SeedType::SEED_SNOWPEA => ProjectileType::PROJECTILE_SNOWPEA,
            SeedType::SEED_PUFFSHROOM | SeedType::SEED_SCAREDYSHROOM | SeedType::SEED_SEASHROOM
                => ProjectileType::PROJECTILE_PUFF,
            SeedType::SEED_CACTUS | SeedType::SEED_CATTAIL
                => ProjectileType::PROJECTILE_SPIKE,
            SeedType::SEED_CABBAGEPULT => ProjectileType::PROJECTILE_CABBAGE,
            SeedType::SEED_KERNELPULT => ProjectileType::PROJECTILE_KERNEL,
            SeedType::SEED_MELONPULT => ProjectileType::PROJECTILE_MELON,
            SeedType::SEED_WINTERMELON => ProjectileType::PROJECTILE_WINTERMELON,
            SeedType::SEED_COBCANNON => ProjectileType::PROJECTILE_COB,
            _ => { return; } // 不是射击植物
        };

        // 玉米投手黄油弹
        let mut aProjectileType = aProjectileType;
        if self.m_seed_type == SeedType::SEED_KERNELPULT && thePlantWeapon == PlantWeapon::WEAPON_SECONDARY {
            aProjectileType = ProjectileType::PROJECTILE_BUTTER;
        }

        // 播放音效
        self.app().PlayFoley(FoleyType::FOLEY_THROW);
        if self.m_seed_type == SeedType::SEED_SNOWPEA || self.m_seed_type == SeedType::SEED_WINTERMELON {
            self.app().PlayFoley(FoleyType::FOLEY_SNOW_PEA_SPARKLES);
        } else if self.m_seed_type == SeedType::SEED_PUFFSHROOM
            || self.m_seed_type == SeedType::SEED_SCAREDYSHROOM
            || self.m_seed_type == SeedType::SEED_SEASHROOM
        {
            self.app().PlayFoley(FoleyType::FOLEY_PUFF);
        }

        // 计算弹丸发射原点
        let (aOriginX, aOriginY) = self.calc_projectile_origin(thePlantWeapon);

        // 花盆偏移
        let board = self.board();
        // [TODO]: board.GetFlowerPotAt(m_plant_col, m_row)
        // if flowerPot { aOriginY -= 5 }

        // 粒子效果
        let aRenderPos = super::board::Board::MakeRenderOrder(RenderLayer::RENDER_LAYER_LAWN_MOWER, theRow, 1);
        if self.m_seed_type == SeedType::SEED_SNOWPEA {
            self.app().AddTodParticle((aOriginX + 8) as f32, (aOriginY + 13) as f32, aRenderPos, 0);
        } else if self.m_seed_type == SeedType::SEED_PUFFSHROOM {
            self.app().AddTodParticle((aOriginX + 18) as f32, (aOriginY + 13) as f32, aRenderPos, 0);
        } else if self.m_seed_type == SeedType::SEED_SCAREDYSHROOM {
            self.app().AddTodParticle((aOriginX + 27) as f32, (aOriginY + 13) as f32, aRenderPos, 0);
        }

        // 创建弹丸
        let aProjectile = board.AddProjectile(aOriginX, aOriginY, aRenderPos, theRow, aProjectileType);
        if !aProjectile.is_null() {
            if self.m_seed_type == SeedType::SEED_CATTAIL {
                // [TODO]: (*aProjectile).m_target_zombie_id = board.DataArrayGetID(theTargetZombie)
            }
        }

        // [TODO]: 树桩/火炬效果
    }

    /// 辅助方法：计算弹丸发射原点 (对应 C++ Fire 中的 origin 计算)
    unsafe fn calc_projectile_origin(&self, thePlantWeapon: PlantWeapon) -> (i32, i32) {
        match self.m_seed_type {
            SeedType::SEED_PUFFSHROOM => (self.base.m_x + 40, self.base.m_y + 40),
            SeedType::SEED_SEASHROOM => (self.base.m_x + 45, self.base.m_y + 63),
            SeedType::SEED_CABBAGEPULT => (self.base.m_x + 5, self.base.m_y - 12),
            SeedType::SEED_MELONPULT | SeedType::SEED_WINTERMELON => (self.base.m_x + 25, self.base.m_y - 46),
            SeedType::SEED_CATTAIL => (self.base.m_x + 20, self.base.m_y - 3),
            SeedType::SEED_KERNELPULT if thePlantWeapon == PlantWeapon::WEAPON_PRIMARY => (self.base.m_x + 19, self.base.m_y - 37),
            SeedType::SEED_KERNELPULT => (self.base.m_x + 12, self.base.m_y - 56),
            SeedType::SEED_LEFTPEATER => (self.base.m_x - 57, self.base.m_y - 33), // 向左射
            SeedType::SEED_GATLINGPEA => (self.base.m_x + 34, self.base.m_y - 33),
            SeedType::SEED_SPLITPEA if thePlantWeapon == PlantWeapon::WEAPON_SECONDARY => (self.base.m_x - 64, self.base.m_y - 33),
            SeedType::SEED_SPLITPEA => (self.base.m_x + 24, self.base.m_y - 33),
            SeedType::SEED_THREEPEATER => (self.base.m_x + 45, self.base.m_y + 10),
            SeedType::SEED_SCAREDYSHROOM => (self.base.m_x + 29, self.base.m_y + 21),
            SeedType::SEED_CACTUS if thePlantWeapon == PlantWeapon::WEAPON_PRIMARY => (self.base.m_x + 93, self.base.m_y - 50),
            SeedType::SEED_CACTUS => (self.base.m_x + 70, self.base.m_y + 23),
            SeedType::SEED_COBCANNON => (self.base.m_x - 44, self.base.m_y - 184),
            // PEASHOOTER / SNOWPEA / REPEATER / SPLITPEA primary
            _ => (self.base.m_x + 24, self.base.m_y - 33), // default pea head offset
        }
    }

    /// C++ Plant::Die (Plant.cpp:4930)
    pub unsafe fn Die(&mut self) {
        // C++: if (IsOnBoard() && mSeedType == SEED_TANGLEKELP) { ... aZombie->DieWithLoot(); }
        if self.IsOnBoard() && self.m_seed_type == SeedType::SEED_TANGLEKELP {
            let _board = self.board();
            // [TODO]: board->ZombieTryToGet(mTargetZombieID)
            // if aZombie { aZombie->DieWithLoot(); }
        }

        self.m_dead = true;
        // [TODO]: RemoveEffects() — remove particles & reanimations

        if !Plant::is_flying(self.m_seed_type) && self.IsOnBoard() {
            // [TODO]: GridItem* aLadder = board->GetLadderAt(mPlantCol, mRow);
            // if aLadder { aLadder->GridItemDie(); }
        }

        if self.IsOnBoard() {
            // [TODO]: Plant* aTopPlant = board->GetTopPlantAt(mPlantCol, mRow, TOPPLANT_BUNGEE_ORDER);
            // [TODO]: Plant* aFlowerPot = board->GetFlowerPotAt(mPlantCol, mRow);
            // if aFlowerPot && aTopPlant == aFlowerPot { ... }
        }
    }

    /// C++ Plant::UpdateAbilities (Plant.cpp:2509) — 主要能力分发
    /// C++ Plant::DrawSeedType — 绘制植物种子类型
    /// C++ Plant::GetImage (Plant.cpp:3802)
    /// [TRANSLATION_NOTE]: C++ 使用 Resources.h 的 IMAGE_PLANTS 贴图集数组；
    /// Rust 侧按植物类型从 ResourceManager 加载独立图像并缓存。
    pub unsafe fn GetImage(the_seed_type: SeedType) -> *mut Image {
        static mut G_PLANT_IMAGE_CACHE: Option<std::collections::HashMap<i32, *mut Image>> = None;
        let a_key = the_seed_type as i32;
        unsafe {
            let a_cache = G_PLANT_IMAGE_CACHE.get_or_insert_with(std::collections::HashMap::new);
            if let Some(&a_cached) = a_cache.get(&a_key) {
                return a_cached;
            }

            // C++: 图像 id（IMAGE_PLANTS 贴图集 + cel）
            let a_id = match the_seed_type {
                SeedType::SEED_PEASHOOTER => "IMAGE_PLANTS",
                SeedType::SEED_SUNFLOWER => "IMAGE_PLANTS",
                SeedType::SEED_WALLNUT => "IMAGE_PLANTS",
                SeedType::SEED_POTATOMINE => "IMAGE_PLANTS",
                _ => "IMAGE_PLANTS",
            };

            let a_base = crate::sexy_app_framework::sexy_app_base::g_sexy_app_ptr();
            if a_base.is_null() || (*a_base).m_resource_manager.is_null() {
                return std::ptr::null_mut();
            }
            let a_img = (*(*a_base).m_resource_manager).GetImage(a_id);
            a_cache.insert(a_key, a_img);
            a_img
        }
    }

    /// C++ Plant::DrawSeedType (Plant.cpp:4100) — 绘制植物种子类型
    pub fn DrawSeedType(g: &mut Graphics, the_seed_type: SeedType, the_imitater_type: SeedType, the_draw_variation: crate::const_enums::DrawVariation, the_pos_x: f32, the_pos_y: f32) {
        // C++: 模仿者变体
        let mut a_seed_type = the_seed_type;
        let mut a_draw_variation = the_draw_variation;
        if the_seed_type == SeedType::SEED_IMITATER && the_imitater_type != SeedType::SEED_NONE {
            a_seed_type = the_imitater_type;
            a_draw_variation = crate::const_enums::DrawVariation::VARIATION_IMITATER;
            if the_imitater_type == SeedType::SEED_HYPNOSHROOM
                || the_imitater_type == SeedType::SEED_SQUASH
                || the_imitater_type == SeedType::SEED_POTATOMINE
                || the_imitater_type == SeedType::SEED_GARLIC
                || the_imitater_type == SeedType::SEED_LILYPAD
            {
                a_draw_variation = crate::const_enums::DrawVariation::VARIATION_IMITATER_LESS;
            }
        } else if the_draw_variation == crate::const_enums::DrawVariation::VARIATION_NORMAL && the_seed_type == SeedType::SEED_TANGLEKELP {
            a_draw_variation = crate::const_enums::DrawVariation::VARIATION_AQUARIUM;
        }

        // C++: BIG_TIME 模式放大核桃/向日葵/万寿菊
        let a_game_mode = unsafe { (*crate::lawn_app::G_LAWN_APP).mGameMode };
        if a_game_mode == GameMode::GAMEMODE_CHALLENGE_BIG_TIME
            && (a_seed_type == SeedType::SEED_WALLNUT || a_seed_type == SeedType::SEED_SUNFLOWER || a_seed_type == SeedType::SEED_MARIGOLD)
        {
            // [TODO]: 缩放
        }

        // C++: GetImage + cel 计算 + DrawImageCel
        let a_image = unsafe { Self::GetImage(a_seed_type) };
        if a_image.is_null() {
            return;
        }
        // [TODO]: 变体 switch（各植物 cel/偏移）→ DrawImageCel
        let _ = (g, the_pos_x, the_pos_y, a_draw_variation);
    }

    pub unsafe fn UpdateAbilities(&mut self) {
        if self.m_do_special_countdown > 0 {
            self.m_do_special_countdown -= 1;
            if self.m_do_special_countdown == 0 {
                // [TODO]: DoSpecial()
            }
        }

        // 射击植物
        if self.m_launch_rate > 0 {
            self.UpdateShooter();
        }

        // 产阳光植物
        if self.m_seed_type == SeedType::SEED_SUNFLOWER
            || self.m_seed_type == SeedType::SEED_TWINSUNFLOWER
            || self.m_seed_type == SeedType::SEED_SUNSHROOM
            || self.m_seed_type == SeedType::SEED_MARIGOLD
        {
            self.UpdateProductionPlant();
        }

        // 各植物类型特定更新
        match self.m_seed_type {
            SeedType::SEED_CHOMPER => { self.UpdateChomper(); }
            SeedType::SEED_SCAREDYSHROOM => { /* [TODO]: UpdateScaredyShroom() */ }
            SeedType::SEED_SUNSHROOM => { /* [TODO]: UpdateSunShroom() */ }
            SeedType::SEED_TORCHWOOD => { self.UpdateTorchwood(); }
            SeedType::SEED_SPIKEWEED | SeedType::SEED_SPIKEROCK => { /* [TODO]: UpdateSpikeweed() */ }
            SeedType::SEED_POTATOMINE => { /* [TODO]: UpdatePotato() */ }
            SeedType::SEED_SQUASH => { /* [TODO]: UpdateSquash() */ }
            SeedType::SEED_GRAVEBUSTER => { /* [TODO]: UpdateGraveBuster() */ }
            SeedType::SEED_MAGNETSHROOM => { /* [TODO]: UpdateMagnetShroom() */ }
            SeedType::SEED_DOOMSHROOM => { /* [TODO]: UpdateDoomShroom() */ }
            SeedType::SEED_ICESHROOM => { /* [TODO]: UpdateIceShroom() */ }
            SeedType::SEED_BLOVER => { /* [TODO]: UpdateBlover() */ }
            SeedType::SEED_CACTUS => { /* [TODO]: UpdateCactus() */ }
            SeedType::SEED_TANGLEKELP => { /* [TODO]: UpdateTanglekelp() */ }
            SeedType::SEED_COBCANNON => { /* [TODO]: UpdateCobCannon() */ }
            SeedType::SEED_GOLD_MAGNET => { /* [TODO]: UpdateGoldMagnetShroom() */ }
            SeedType::SEED_IMITATER => { /* [TODO]: UpdateImitater() */ }
            _ => {}
        }
    }

    // =========================================================================
    // ★ 植物特定更新方法 (C++ 保真翻译)
    // =========================================================================

    /// C++ Plant::UpdateTorchwood (Plant.cpp:1374)
    /// 火炬树桩 — 检测经过的豌豆并升级为火球
    pub unsafe fn UpdateTorchwood(&mut self) {
        let board = self.board();
        let a_attack_rect = self.GetPlantAttackRect(PlantWeapon::WEAPON_PRIMARY);

        let mut a_projectile: *mut super::projectile::Projectile = std::ptr::null_mut();
        while board.IterateProjectiles(&mut a_projectile) {
            if (*a_projectile).base.m_row == self.base.m_row
                && ((*a_projectile).m_projectile_type == ProjectileType::PROJECTILE_PEA
                    || (*a_projectile).m_projectile_type == ProjectileType::PROJECTILE_SNOWPEA)
            {
                let a_projectile_rect = self.GetProjectileRect(a_projectile);
                if self.GetRectOverlapRect(a_attack_rect, a_projectile_rect) >= 10 {
                    if (*a_projectile).m_projectile_type == ProjectileType::PROJECTILE_PEA {
                        // aProjectile->ConvertToFireball(mPlantCol);
                        // [TODO]: 转换为火球
                        (*a_projectile).m_hit_torchwood_grid_x = self.m_plant_col;
                    } else if (*a_projectile).m_projectile_type == ProjectileType::PROJECTILE_SNOWPEA {
                        // aProjectile->ConvertToPea(mPlantCol);
                        // [TODO]: 冰豆经过树桩变普通豌豆
                        (*a_projectile).m_projectile_type = ProjectileType::PROJECTILE_PEA;
                    }
                }
            }
        }
    }

    /// C++ Plant::UpdateChomper (Plant.cpp:1747)
    /// 大嘴花 — 咬住并消化僵尸
    pub unsafe fn UpdateChomper(&mut self) {
        let app = self.app();

        if self.m_state == PlantState::STATE_READY {
            // C++: 检测前方是否有僵尸
            // [TODO]: FindTargetZombie(mRow, WEAPON_PRIMARY)
            // 如果找到僵尸 → 切换到 biting 状态
            self.m_state = PlantState::STATE_CHOMPER_BITING;
            self.m_state_countdown = 70;
        } else if self.m_state == PlantState::STATE_CHOMPER_BITING {
            if self.m_state_countdown == 0 {
                app.PlayFoley(crate::sexy_tod_lib::tod_foley::FoleyType::FOLEY_BIGCHOMP);
                // C++: 检测僵尸 + 判定吞没或咬伤
                // [TODO]: FindTargetZombie 判断逻辑
                // if gargantuar/boss → doBite (只造成伤害)
                // if pogo/pole-vaulting → doMiss
                // else → aZombie->DieWithLoot(); state = STATE_CHOMPER_BITING_GOT_ONE
                self.m_state = PlantState::STATE_CHOMPER_BITING_MISSED;
            }
        } else if self.m_state == PlantState::STATE_CHOMPER_BITING_GOT_ONE {
            // C++: 动画循环结束 → 进入消化状态
            self.m_state = PlantState::STATE_CHOMPER_DIGESTING;
            self.m_state_countdown = 4000;
        } else if self.m_state == PlantState::STATE_CHOMPER_DIGESTING {
            if self.m_state_countdown == 0 {
                // C++: 吞咽动画 → 回到 ready
                self.m_state = PlantState::STATE_CHOMPER_SWALLOWING;
            }
        } else if self.m_state == PlantState::STATE_CHOMPER_BITING_MISSED
            || self.m_state == PlantState::STATE_CHOMPER_SWALLOWING
        {
            // C++: 回到 ready 状态
            self.m_state = PlantState::STATE_READY;
        }
    }

    /// 获取植物攻击矩形 (C++ Plant::GetPlantAttackRect)
    /// C++ Plant::GetPlantRect (Plant.cpp:5139)
    pub fn GetPlantRect(&self) -> crate::sexy_app_framework::misc::rect::Rect {
        if self.m_seed_type == SeedType::SEED_TALLNUT {
            crate::sexy_app_framework::misc::rect::Rect::new(self.base.m_x + 10, self.base.m_y, self.base.m_width, self.base.m_height)
        } else if self.m_seed_type == SeedType::SEED_PUMPKINSHELL {
            crate::sexy_app_framework::misc::rect::Rect::new(self.base.m_x, self.base.m_y, self.base.m_width - 20, self.base.m_height)
        } else if self.m_seed_type == SeedType::SEED_COBCANNON {
            crate::sexy_app_framework::misc::rect::Rect::new(self.base.m_x, self.base.m_y, 140, 80)
        } else {
            crate::sexy_app_framework::misc::rect::Rect::new(self.base.m_x + 10, self.base.m_y, self.base.m_width - 20, self.base.m_height)
        }
    }
    pub unsafe fn GetPlantAttackRect(&self, _weapon: PlantWeapon) -> crate::sexy_app_framework::misc::rect::Rect {
        // [TODO]: 根据不同植物类型返回攻击范围矩形
        crate::sexy_app_framework::misc::rect::Rect {
            m_x: self.base.m_x - 20,
            m_y: self.base.m_y - 20,
            m_width: self.base.m_width + 40,
            m_height: self.base.m_height + 40,
        }
    }

    /// 获取投射物矩形 (C++ Projectile::GetProjectileRect)
    pub unsafe fn GetProjectileRect(&self, projectile: *mut super::projectile::Projectile) -> crate::sexy_app_framework::misc::rect::Rect {
        crate::sexy_app_framework::misc::rect::Rect {
            m_x: (*projectile).m_pos_x as i32 - 10,
            m_y: ((*projectile).m_pos_y + (*projectile).m_pos_z) as i32 - 10,
            m_width: 20,
            m_height: 20,
        }
    }

    /// 矩形重叠检测 (C++ GetRectOverlap)
    pub unsafe fn GetRectOverlapRect(&self, r1: crate::sexy_app_framework::misc::rect::Rect, r2: crate::sexy_app_framework::misc::rect::Rect) -> i32 {
        let overlap_x = (r1.m_x + r1.m_width).min(r2.m_x + r2.m_width) - r1.m_x.max(r2.m_x);
        let overlap_y = (r1.m_y + r1.m_height).min(r2.m_y + r2.m_height) - r1.m_y.max(r2.m_y);
        if overlap_x <= 0 || overlap_y <= 0 { -1 } else { overlap_x * overlap_y }
    }

    /// C++ Plant::UpdateShooter (Plant.cpp:942)
    pub unsafe fn UpdateShooter(&mut self) {
        self.m_launch_counter -= 1;
        if self.m_launch_counter <= 0 {
            self.m_launch_counter = self.m_launch_rate - crate::sexy_app_framework::common::rand_int() % 15;

            match self.m_seed_type {
                SeedType::SEED_THREEPEATER => {
                    // [TODO]: LaunchThreepeater()
                }
                SeedType::SEED_STARFRUIT => {
                    // [TODO]: LaunchStarFruit()
                }
                SeedType::SEED_SPLITPEA => {
                    // [TODO]: FindTargetAndFire(mRow, WEAPON_PRIMARY)
                    // [TODO]: FindTargetAndFire(mRow, WEAPON_SECONDARY)
                }
                SeedType::SEED_CACTUS => {
                    // [TODO]: Fire based on STATE_CACTUS_HIGH/LOW
                }
                _ => {
                    // [TODO]: FindTargetAndFire(mRow, WEAPON_PRIMARY)
                }
            }
        }

        // 香蒲/双发射手/裂荚射手二次射击
        if self.m_launch_counter == 50 && self.m_seed_type == SeedType::SEED_CATTAIL {
            // [TODO]: FindTargetAndFire(mRow, WEAPON_PRIMARY)
        }
        if self.m_launch_counter == 25 {
            if self.m_seed_type == SeedType::SEED_REPEATER || self.m_seed_type == SeedType::SEED_LEFTPEATER {
                // [TODO]: FindTargetAndFire(mRow, WEAPON_PRIMARY)
            } else if self.m_seed_type == SeedType::SEED_SPLITPEA {
                // [TODO]: FindTargetAndFire(mRow, WEAPON_SECONDARY)
            }
        }
    }

    /// C++ Plant::UpdateProductionPlant (Plant.cpp:1001)
    pub unsafe fn UpdateProductionPlant(&mut self) {
        // [TODO]: Decrement mLaunchCounter, produce sun when ready
    }

    /// C++ Plant::FindTargetAndFire (Plant.cpp:730)
    pub unsafe fn FindTargetAndFire(&mut self, _theRow: i32, _thePlantWeapon: PlantWeapon) -> bool {
        // [TODO]: FindTargetZombie(theRow, thePlantWeapon)
        // if zombie found → Fire(zombie, row, weapon)
        false
    }

    pub unsafe fn Animate(&mut self) {
        // TODO: Frame and reanimation animation update
    }

    pub unsafe fn UpdateReanim(&mut self) {
        // TODO: Reanimation update
    }

    pub unsafe fn NotOnGround(&self) -> bool {
        if self.m_seed_type == SeedType::SEED_SQUASH {
            if self.m_state == PlantState::STATE_SQUASH_RISING
                || self.m_state == PlantState::STATE_SQUASH_FALLING
                || self.m_state == PlantState::STATE_SQUASH_DONE_FALLING
            {
                return true;
            }
        }
        self.m_squished || self.m_on_bungee_state == PlantOnBungeeState::RISING_WITH_BUNGEE || self.m_dead
    }

    pub fn IsOnBoard(&self) -> bool {
        !self.base.m_board.is_null()
    }
}

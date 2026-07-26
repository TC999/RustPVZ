// [TRANSLATION_NOTE]: Plant.h -> Rust 模块
// C++ Plant 类翻译为 Rust struct + impl 块

use crate::const_enums::*;
use crate::sexy_app_framework::misc::rect::Rect;
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
            SeedType::SEED_PUFFSHROOM | SeedType::SEED_SUNSHROOM | 
            SeedType::SEED_FUMESHROOM | SeedType::SEED_GRAVEBUSTER |
            SeedType::SEED_HYPNOSHROOM | SeedType::SEED_SCAREDYSHROOM |
            SeedType::SEED_ICESHROOM | SeedType::SEED_DOOMSHROOM |
            SeedType::SEED_GLOOMSHROOM | SeedType::SEED_SEASHROOM |
            SeedType::SEED_MAGNETSHROOM
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
        matches!(seed_type, SeedType::SEED_LILYPAD | SeedType::SEED_TANGLEKELP | SeedType::SEED_CATTAIL)
    }

    pub fn is_flying(seed_type: SeedType) -> bool {
        matches!(seed_type, SeedType::SEED_CATTAIL | SeedType::SEED_STARFRUIT | SeedType::SEED_UMBRELLA)
    }

    pub fn is_upgrade(seed_type: SeedType) -> bool {
        matches!(seed_type,
            SeedType::SEED_GATLINGPEA | SeedType::SEED_TWINSUNFLOWER |
            SeedType::SEED_GLOOMSHROOM | SeedType::SEED_CATTAIL |
            SeedType::SEED_WINTERMELON | SeedType::SEED_GOLD_MAGNET |
            SeedType::SEED_SPIKEROCK | SeedType::SEED_COBCANNON |
            SeedType::SEED_IMITATER
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

impl Default for Plant {
    fn default() -> Self {
        Self::new()
    }
}

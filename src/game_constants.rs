// [TRANSLATION_NOTE]: GameConstants.h -> Rust 常量
// C++ constexpr 映射为 Rust const

pub const PI: f64 = 3.141592653589793;

// Board dimensions
pub const BOARD_WIDTH: i32 = 800;
pub const BOARD_HEIGHT: i32 = 600;
pub const WIDE_BOARD_WIDTH: i32 = 800;
pub const BOARD_OFFSET: i32 = 220;
pub const BOARD_EDGE: i32 = -100;
pub const BOARD_IMAGE_WIDTH_OFFSET: i32 = 1180;
pub const BOARD_ICE_START: i32 = 800;
pub const LAWN_XMIN: i32 = 40;
pub const LAWN_YMIN: i32 = 80;
pub const HIGH_GROUND_HEIGHT: i32 = 30;

// Seed bank
pub const SEEDBANK_MAX: i32 = 10;
pub const SEED_BANK_OFFSET_X: i32 = 0;
pub const SEED_BANK_OFFSET_X_END: i32 = 10;
pub const SEED_CHOOSER_OFFSET_Y: i32 = 516;
pub const SEED_PACKET_WIDTH: i32 = 50;
pub const SEED_PACKET_HEIGHT: i32 = 70;
pub const IMITATER_DIALOG_WIDTH: i32 = 500;
pub const IMITATER_DIALOG_HEIGHT: i32 = 600;

// Levels
pub const ADVENTURE_AREAS: i32 = 5;
pub const LEVELS_PER_AREA: i32 = 10;
pub const NUM_LEVELS: i32 = ADVENTURE_AREAS * LEVELS_PER_AREA;
pub const FINAL_LEVEL: i32 = NUM_LEVELS;
pub const FLAG_RAISE_TIME: i32 = 100;
pub const LAST_STAND_FLAGS: i32 = 5;
pub const ZOMBIE_COUNTDOWN_FIRST_WAVE: i32 = 1800;
pub const ZOMBIE_COUNTDOWN: i32 = 2500;
pub const ZOMBIE_COUNTDOWN_RANGE: i32 = 600;
pub const ZOMBIE_COUNTDOWN_BEFORE_FLAG: i32 = 4500;
pub const ZOMBIE_COUNTDOWN_BEFORE_REPICK: i32 = 5499;
pub const ZOMBIE_COUNTDOWN_MIN: i32 = 400;
pub const FOG_BLOW_RETURN_TIME: i32 = 2000;
pub const SUN_COUNTDOWN: i32 = 425;
pub const SUN_COUNTDOWN_RANGE: i32 = 275;
pub const SUN_COUNTDOWN_MAX: i32 = 950;
pub const SURVIVAL_NORMAL_FLAGS: i32 = 5;
pub const SURVIVAL_HARD_FLAGS: i32 = 10;

// Store screen layout
pub const STORESCREEN_ITEMOFFSET_1_X: i32 = 422;
pub const STORESCREEN_ITEMOFFSET_1_Y: i32 = 206;
pub const STORESCREEN_ITEMOFFSET_2_X: i32 = 372;
pub const STORESCREEN_ITEMOFFSET_2_Y: i32 = 310;
pub const STORESCREEN_ITEMSIZE: i32 = 74;
pub const STORESCREEN_COINBANK_X: i32 = 650;
pub const STORESCREEN_COINBANK_Y: i32 = 559;
pub const STORESCREEN_PAGESTRING_X: i32 = 470;
pub const STORESCREEN_PAGESTRING_Y: i32 = 500;

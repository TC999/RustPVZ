// [TRANSLATION_NOTE]: LawnApp.h -> Rust 模块
// 游戏应用主类，继承自 SexyApp

use std::collections::LinkedList;
use crate::const_enums::*;
use crate::lawn::board::Board;

pub type ButtonList = LinkedList<*mut std::ffi::c_void>;
pub type ImageList = LinkedList<*mut std::ffi::c_void>;

#[derive(Clone)]
pub struct LevelStats {
    pub m_unused_lawn_mowers: i32,
}

impl LevelStats {
    pub fn new() -> Self {
        LevelStats { m_unused_lawn_mowers: 0 }
    }
    pub fn reset(&mut self) {
        self.m_unused_lawn_mowers = 0;
    }
}

pub struct LawnApp {
    pub m_board: Option<Box<Board>>,
    pub m_title_screen: *mut std::ffi::c_void,
    pub m_game_selector: *mut std::ffi::c_void,
    pub m_seed_chooser_screen: *mut std::ffi::c_void,
    pub m_award_screen: *mut std::ffi::c_void,
    pub m_credit_screen: *mut std::ffi::c_void,
    pub m_challenge_screen: *mut std::ffi::c_void,
    pub m_sound_system: *mut std::ffi::c_void,
    pub m_control_button_list: ButtonList,
    pub m_created_image_list: ImageList,
    pub m_refer_id: String,
    pub m_register_link: String,
    pub m_mod: String,
    pub m_register_resources_loaded: bool,
    pub m_tod_cheat_keys: bool,
    pub mGameMode: GameMode,
    pub mGameScene: GameScenes,
    pub m_loading_zombies_thread_completed: bool,
    pub m_first_time_game_selector: bool,
    pub m_games_played: i32,
    pub m_max_executions: i32,
    pub m_max_plays: i32,
    pub m_max_time: i32,
    pub m_easy_planting_cheat: bool,
    pub m_pool_effect: *mut std::ffi::c_void,
    pub m_zen_garden: *mut std::ffi::c_void,
    pub m_effect_system: *mut std::ffi::c_void,
    pub m_reanimator_cache: *mut std::ffi::c_void,
    pub m_profile_mgr: *mut std::ffi::c_void,
    pub m_player_info: *mut std::ffi::c_void,
    pub m_last_level_stats: Option<Box<LevelStats>>,
    pub m_close_request: bool,
    pub m_app_counter: u32,
    pub m_music: *mut std::ffi::c_void,
    pub m_crazy_dave_reanim_id: ReanimationID,
    pub m_crazy_dave_state: CrazyDaveState,
    pub m_crazy_dave_blink_counter: i32,
    pub m_crazy_dave_blink_reanim_id: ReanimationID,
    pub m_crazy_dave_message_index: i32,
    pub m_crazy_dave_message_text: String,
    pub m_app_rand_seed: i32,
    pub m_session_id: isize,
    pub m_play_time_active_session: i32,
    pub m_play_time_inactive_session: i32,
    pub mBoardResult: BoardResult,
    pub mSawYeti: bool,
    pub mAppRandSeed: i32,
    pub mPlayerInfo: *mut std::ffi::c_void,
    pub m_konami_check: *mut std::ffi::c_void,
    pub m_mustache_check: *mut std::ffi::c_void,
    pub m_moustache_check: *mut std::ffi::c_void,
    pub m_super_mower_check: *mut std::ffi::c_void,
    pub m_super_mower_check2: *mut std::ffi::c_void,
    pub m_future_check: *mut std::ffi::c_void,
    pub m_pinata_check: *mut std::ffi::c_void,
    pub m_dance_check: *mut std::ffi::c_void,
    pub m_daisy_check: *mut std::ffi::c_void,
    pub m_sukhbir_check: *mut std::ffi::c_void,
    pub m_mustache_mode: bool,
    pub m_super_mower_mode: bool,
    pub m_future_mode: bool,
    pub m_pinata_mode: bool,
    pub m_dance_mode: bool,
    pub m_daisy_mode: bool,
    pub m_sukhbir_mode: bool,
    pub m_trial_type: TrialType,
    pub m_debug_trial_locked: bool,
    pub m_mute_sounds_for_cutscene: bool,
}

impl LawnApp {
    pub fn new() -> Self {
        LawnApp {
            m_board: None,
            m_title_screen: std::ptr::null_mut(),
            m_game_selector: std::ptr::null_mut(),
            m_seed_chooser_screen: std::ptr::null_mut(),
            m_award_screen: std::ptr::null_mut(),
            m_credit_screen: std::ptr::null_mut(),
            m_challenge_screen: std::ptr::null_mut(),
            m_sound_system: std::ptr::null_mut(),
            m_control_button_list: LinkedList::new(),
            m_created_image_list: LinkedList::new(),
            m_refer_id: String::new(),
            m_register_link: String::new(),
            m_mod: String::new(),
            m_register_resources_loaded: false,
            m_tod_cheat_keys: false,
            mGameMode: GameMode::GAMEMODE_ADVENTURE,
            mGameScene: GameScenes::SCENE_LOADING,
            m_loading_zombies_thread_completed: false,
            m_first_time_game_selector: true,
            m_games_played: 0,
            m_max_executions: 0,
            m_max_plays: 0,
            m_max_time: 0,
            m_easy_planting_cheat: false,
            m_pool_effect: std::ptr::null_mut(),
            m_zen_garden: std::ptr::null_mut(),
            m_effect_system: std::ptr::null_mut(),
            m_reanimator_cache: std::ptr::null_mut(),
            m_profile_mgr: std::ptr::null_mut(),
            m_player_info: std::ptr::null_mut(),
            m_last_level_stats: None,
            m_close_request: false,
            m_app_counter: 0,
            m_music: std::ptr::null_mut(),
            m_crazy_dave_reanim_id: ReanimationID::REANIMATIONID_NULL,
            m_crazy_dave_state: CrazyDaveState::CRAZY_DAVE_OFF,
            m_crazy_dave_blink_counter: 0,
            m_crazy_dave_blink_reanim_id: ReanimationID::REANIMATIONID_NULL,
            m_crazy_dave_message_index: 0,
            m_crazy_dave_message_text: String::new(),
            m_app_rand_seed: 0,
            m_session_id: 0,
            m_play_time_active_session: 0,
            m_play_time_inactive_session: 0,
            mBoardResult: BoardResult::BOARDRESULT_NONE,
            mSawYeti: false,
            mAppRandSeed: 0,
            mPlayerInfo: std::ptr::null_mut(),
            m_konami_check: std::ptr::null_mut(),
            m_mustache_check: std::ptr::null_mut(),
            m_moustache_check: std::ptr::null_mut(),
            m_super_mower_check: std::ptr::null_mut(),
            m_super_mower_check2: std::ptr::null_mut(),
            m_future_check: std::ptr::null_mut(),
            m_pinata_check: std::ptr::null_mut(),
            m_dance_check: std::ptr::null_mut(),
            m_daisy_check: std::ptr::null_mut(),
            m_sukhbir_check: std::ptr::null_mut(),
            m_mustache_mode: false,
            m_super_mower_mode: false,
            m_future_mode: false,
            m_pinata_mode: false,
            m_dance_mode: false,
            m_daisy_mode: false,
            m_sukhbir_mode: false,
            m_trial_type: TrialType::TRIALTYPE_NONE,
            m_debug_trial_locked: false,
            m_mute_sounds_for_cutscene: false,
        }
    }

    pub fn is_adventure_mode(&self) -> bool {
        self.mGameMode == GameMode::GAMEMODE_ADVENTURE
    }

    pub fn is_survival_mode(&self) -> bool {
        let mode = self.mGameMode as i32;
        mode >= GameMode::GAMEMODE_SURVIVAL_NORMAL_STAGE_1 as i32 
            && mode <= GameMode::GAMEMODE_SURVIVAL_ENDLESS_STAGE_5 as i32
    }

    pub fn is_art_challenge(&self) -> bool {
        let mode = self.mGameMode as i32;
        mode >= GameMode::GAMEMODE_CHALLENGE_WAR_AND_PEAS as i32 
            && mode <= GameMode::GAMEMODE_CHALLENGE_RAINING_SEEDS_2 as i32
    }

    pub fn is_challenge_without_seed_bank(&self) -> bool {
        self.mGameMode == GameMode::GAMEMODE_CHALLENGE_SLOT_MACHINE
            || self.mGameMode == GameMode::GAMEMODE_CHALLENGE_BEGHOULED
            || self.mGameMode == GameMode::GAMEMODE_CHALLENGE_INVISIGHOUL
            || self.mGameMode == GameMode::GAMEMODE_CHALLENGE_LITTLE_TROUBLE
            || self.mGameMode == GameMode::GAMEMODE_CHALLENGE_PORTAL_COMBAT
            || self.mGameMode == GameMode::GAMEMODE_CHALLENGE_COLUMN
            || self.mGameMode == GameMode::GAMEMODE_CHALLENGE_POGO_PARTY
            || self.mGameMode == GameMode::GAMEMODE_CHALLENGE_WALLNUT_BOWLING_2
            || self.mGameMode == GameMode::GAMEMODE_CHALLENGE_ZOMBIES_ON_THE_ROCKS
    }

    pub fn is_shovel_level(&self) -> bool {
        (self.mGameMode as i32) >= GameMode::GAMEMODE_CHALLENGE_WALLNUT_BOWLING as i32
            && (self.mGameMode as i32) <= GameMode::GAMEMODE_CHALLENGE_LITTLE_TROUBLE as i32
    }

    pub fn is_wallnut_bowling_level(&self) -> bool {
        self.mGameMode == GameMode::GAMEMODE_CHALLENGE_WALLNUT_BOWLING
            || self.mGameMode == GameMode::GAMEMODE_CHALLENGE_WALLNUT_BOWLING_2
    }

    pub fn is_night(&self) -> bool {
        false // TODO: check background type
    }

    pub fn can_show_almanac(&self) -> bool {
        self.mGameScene == GameScenes::SCENE_PLAYING
            || self.mGameScene == GameScenes::SCENE_CHALLENGE
    }

    pub fn end_level(&self) {
        // Placeholder
    }

    pub fn IsFirstTimeAdventureMode(&self) -> bool {
        self.mGameScene == GameScenes::SCENE_LEVEL_INTRO
            && self.mGameMode == GameMode::GAMEMODE_ADVENTURE
    }

    pub fn IsScaryPotterLevel(&self) -> bool {
        let mode = self.mGameMode as i32;
        mode >= GameMode::GAMEMODE_SCARY_POTTER_1 as i32
            && mode <= GameMode::GAMEMODE_SCARY_POTTER_ENDLESS as i32
    }

    pub fn IsEndlessScaryPotter(&self, theGameMode: GameMode) -> bool {
        theGameMode as i32 == GameMode::GAMEMODE_SCARY_POTTER_ENDLESS as i32
    }

    pub fn IsStormyNightLevel(&self) -> bool {
        self.mGameMode as i32 == GameMode::GAMEMODE_CHALLENGE_STORMY_NIGHT as i32
    }

    pub fn IsFinalBossLevel(&self) -> bool {
        self.mGameMode as i32 == GameMode::GAMEMODE_CHALLENGE_FINAL_BOSS as i32
    }

    pub fn IsWhackAZombieLevel(&self) -> bool {
        self.mGameMode as i32 == GameMode::GAMEMODE_CHALLENGE_WHACK_A_ZOMBIE as i32
    }

    pub fn IsSlotMachineLevel(&self) -> bool {
        self.mGameMode as i32 == GameMode::GAMEMODE_CHALLENGE_SLOT_MACHINE as i32
    }

    pub fn IsIZombieLevel(&self) -> bool {
        let mode = self.mGameMode as i32;
        mode >= GameMode::GAMEMODE_CHALLENGE_PUZZLE_I_ZOMBIE_1 as i32
            && mode <= GameMode::GAMEMODE_CHALLENGE_PUZZLE_I_ZOMBIE_ENDLESS as i32
    }
}

// C++ gLawnApp 全局指针的 Rust 映射
pub static mut G_LAWN_APP: *mut LawnApp = std::ptr::null_mut();



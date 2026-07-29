// [TRANSLATION_NOTE]: LawnApp.h -> Rust 模块
// 游戏应用主类，继承自 SexyApp

use std::collections::LinkedList;
use crate::const_enums::*;
use crate::sexy_tod_lib::tod_foley::FoleyType;
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

    // =========================================================================
    // ★ 核心接口 (被 Plant / Zombie / Board 大量调用)
    // =========================================================================

    /// C++ LawnApp::PlayFoley (LawnApp.cpp:〜line 1200)
    pub unsafe fn PlayFoley(&mut self, _theFoleyType: FoleyType) {
        // [TODO]: mSoundSystem->PlayFoley(theFoleyType)
    }

    /// C++ LawnApp::PlayFoleyPitch
    pub unsafe fn PlayFoleyPitch(&mut self, _theFoleyType: FoleyType, _thePitch: f32) {
        // [TODO]
    }

    /// C++ LawnApp::PlaySample
    pub unsafe fn PlaySample(&mut self, _theSoundNum: isize) {
        // [TODO]: SexyApp::PlaySample
    }

    /// C++ LawnApp::AddReanimation (LawnApp.cpp:〜line 1400)
    pub unsafe fn AddReanimation(
        &mut self,
        _theX: f32,
        _theY: f32,
        _theRenderOrder: i32,
        _theReanimationType: ReanimationType,
    ) -> *mut std::ffi::c_void {
        // [TODO]: mEffectSystem->mReanimationHolder->AllocReanimation(...)
        std::ptr::null_mut()
    }

    /// C++ LawnApp::ReanimationGetID (inline)
    pub unsafe fn ReanimationGetID(&self, _theReanimation: *mut std::ffi::c_void) -> ReanimationID {
        // [TODO]
        ReanimationID::REANIMATIONID_NULL
    }

    /// C++ LawnApp::ReanimationGet (inline)
    pub unsafe fn ReanimationGet(&self, _theReanimationID: ReanimationID) -> *mut std::ffi::c_void {
        // [TODO]: mEffectSystem->mReanimationHolder->GetReanimation(theReanimationID)
        std::ptr::null_mut()
    }

    /// C++ LawnApp::ReanimationTryToGet (inline)
    pub unsafe fn ReanimationTryToGet(&self, _theReanimationID: ReanimationID) -> *mut std::ffi::c_void {
        std::ptr::null_mut()
    }

    /// C++ LawnApp::RemoveReanimation
    pub unsafe fn RemoveReanimation(&mut self, _theReanimationID: ReanimationID) {
        // [TODO]: mEffectSystem->mReanimationHolder->RemoveReanimation(theReanimationID)
    }

    /// C++ LawnApp::AddTodParticle
    pub unsafe fn AddTodParticle(
        &mut self,
        _theX: f32,
        _theY: f32,
        _theRenderOrder: i32,
        _theEffect: i32,  // [TODO]: ParticleEffect 枚举尚未翻译
    ) -> *mut std::ffi::c_void {
        // [TODO]: mEffectSystem->mParticleHolder->AllocParticle(...)
        std::ptr::null_mut()
    }

    /// C++ LawnApp::ParticleTryToGet (inline)
    pub unsafe fn ParticleTryToGet(&self, _theParticleID: ParticleSystemID) -> *mut std::ffi::c_void {
        std::ptr::null_mut()
    }

    /// C++ LawnApp::ParticleGet (inline)
    pub unsafe fn ParticleGet(&self, _theParticleID: ParticleSystemID) -> *mut std::ffi::c_void {
        std::ptr::null_mut()
    }

    /// C++ LawnApp::ParticleGetID (inline)
    pub unsafe fn ParticleGetID(&self, _theParticle: *mut std::ffi::c_void) -> ParticleSystemID {
        ParticleSystemID::PARTICLESYSTEMID_NULL
    }

    /// C++ LawnApp::RemoveParticle
    pub unsafe fn RemoveParticle(&mut self, _theParticleID: ParticleSystemID) {
        // [TODO]: mEffectSystem->mParticleHolder->RemoveParticle(theParticleID)
    }

    /// C++ LawnApp::KillBoard
    pub unsafe fn KillBoard(&mut self) {
        // [TODO]: delete mBoard; mBoard = nullptr
        // ChallengeScreen/StoreScreen cleanup
    }

    /// C++ LawnApp::MakeNewBoard
    pub unsafe fn MakeNewBoard(&mut self) {
        // [TODO]: mBoard = new Board(); mBoard->Init()
    }

    /// C++ LawnApp::StartPlaying
    pub unsafe fn StartPlaying(&mut self) {
        // [TODO]: mBoard->StartLevel()
    }

    /// C++ LawnApp::PreNewGame
    pub unsafe fn PreNewGame(&mut self, _theGameMode: GameMode, _theLookForSavedGame: bool) {
        // [TODO]: cleanup old board, init new
    }

    /// C++ LawnApp::NewGame
    pub unsafe fn NewGame(&mut self) {
        // [TODO]
    }

    /// C++ LawnApp::ShowGameSelector
    pub unsafe fn ShowGameSelector(&mut self) {
        // [TODO]
    }

    /// C++ LawnApp::KillGameSelector
    pub unsafe fn KillGameSelector(&mut self) {
        // [TODO]
    }

    /// C++ LawnApp::ShowAwardScreen
    pub unsafe fn ShowAwardScreen(&mut self, _theAwardType: AwardType, _theShowAchievements: bool) {
        // [TODO]
    }

    /// C++ LawnApp::KillAwardScreen
    pub unsafe fn KillAwardScreen(&mut self) {
        // [TODO]
    }

    /// C++ LawnApp::IsContinuousChallenge
    pub unsafe fn IsContinuousChallenge(&self) -> bool {
        // [TODO]: check if continuous mode
        false
    }

    /// C++ LawnApp::HasFinishedAdventure (inline)
    pub unsafe fn HasFinishedAdventure(&self) -> bool {
        // [TODO]: check registry flags
        false
    }

    /// C++ LawnApp::GetSeedsAvailable
    pub unsafe fn GetSeedsAvailable(&self) -> i32 {
        // [TODO]: count available seeds from player info
        0
    }

    /// C++ LawnApp::HasSeedType
    pub unsafe fn HasSeedType(&self, _theSeedType: SeedType) -> bool {
        // [TODO]: check player info
        false
    }

    /// C++ LawnApp::WriteCurrentUserConfig (inline)
    pub unsafe fn WriteCurrentUserConfig(&self) -> bool {
        // [TODO]: write registry / save file
        true
    }

    /// C++ LawnApp::CheckForGameEnd
    /// C++ LawnApp::CheckForGameEnd (LawnApp.cpp:1515)
    pub unsafe fn CheckForGameEnd(&mut self) {
        let level_complete = self.m_board.as_ref().map_or(false, |b| b.mLevelComplete);
        if !level_complete {
            return;
        }

        let aLevel = self.m_board.as_ref().map(|b| b.mLevel).unwrap_or(0);
        let is_first_time = self.IsFirstTimeAdventureMode();

        // [TODO]: bool aUnlockedNewChallenge = UpdatePlayerProfileForFinishingLevel()

        if self.is_adventure_mode() {
            self.KillBoard();

            if is_first_time && aLevel < 50 {
                self.ShowAwardScreen(AwardType::AWARD_FORLEVEL, true);
            } else if aLevel == 50 /* FINAL_LEVEL */ {
                self.ShowAwardScreen(AwardType::AWARD_CREDITS_ZOMBIENOTE, true);
            } else if aLevel == 9 || aLevel == 19 || aLevel == 29 || aLevel == 39 || aLevel == 49 {
                self.ShowAwardScreen(AwardType::AWARD_FORLEVEL, true);
            } else {
                self.PreNewGame(self.mGameMode, false);
            }
        } else {
            self.KillBoard();
            self.ShowGameSelector();
        }
    }

    /// C++ LawnApp::WriteToRegistry
    pub unsafe fn WriteToRegistry(&mut self) {
        // [TODO]: save game state
    }

    /// C++ LawnApp::ReadFromRegistry
    pub unsafe fn ReadFromRegistry(&mut self) {
        // [TODO]: load game state
    }

    /// C++ LawnApp::PreloadForUser
    pub unsafe fn PreloadForUser(&mut self) {
        // [TODO]: preload zombie/plant resources for current level
    }

    /// C++ LawnApp::CanSpawnYetis (inline)
    pub unsafe fn CanSpawnYetis(&self) -> bool {
        // [TODO]: check adventure progress
        false
    }

    /// C++ LawnApp::IsMiniBossLevel (inline)
    pub unsafe fn IsMiniBossLevel(&self) -> bool {
        // [TODO]: C++ uses GAMEMODE_CHALLENGE_MINIBOSS which may map differently
        false
    }

    /// C++ LawnApp::IsLittleTroubleLevel (inline)
    pub unsafe fn IsLittleTroubleLevel(&self) -> bool {
        self.mGameMode as i32 == GameMode::GAMEMODE_CHALLENGE_LITTLE_TROUBLE as i32
    }

    /// C++ LawnApp::IsBungeeBlitzLevel (inline)
    pub unsafe fn IsBungeeBlitzLevel(&self) -> bool {
        self.mGameMode as i32 == GameMode::GAMEMODE_CHALLENGE_BUNGEE_BLITZ as i32
    }

    /// C++ LawnApp::CanShowStore (inline)
    pub unsafe fn CanShowStore(&self) -> bool {
        self.mGameScene == GameScenes::SCENE_PLAYING
    }

    /// C++ LawnApp::GetNumTrophies
    pub unsafe fn GetNumTrophies(&self, _thePage: ChallengePage) -> i32 {
        // [TODO]: count from player info
        0
    }

    /// C++ LawnApp::Pluralize (static)
    pub fn Pluralize(theCount: i32, theSingular: &str, thePlural: &str) -> String {
        if theCount == 1 {
            format!("{} {}", theCount, theSingular)
        } else {
            format!("{} {}", theCount, thePlural)
        }
    }

    // =========================================================================
    // ★ CrazyDave 系统
    // =========================================================================
    pub unsafe fn CrazyDaveEnter(&mut self) { /* TODO */ }
    pub unsafe fn UpdateCrazyDave(&mut self) { /* TODO */ }
    pub unsafe fn CrazyDaveTalkIndex(&mut self, _theMessageIndex: i32) { /* TODO */ }
    pub unsafe fn CrazyDaveTalkMessage(&mut self, _theMessage: &str) { /* TODO */ }
    pub unsafe fn CrazyDaveLeave(&mut self) { /* TODO */ }
    pub unsafe fn CrazyDaveDie(&mut self) { /* TODO */ }
    pub unsafe fn CrazyDaveStopTalking(&mut self) { /* TODO */ }
    pub unsafe fn DrawCrazyDave(&self, _g: *mut std::ffi::c_void) { /* TODO */ }
    pub unsafe fn GetCrazyDaveText(&self, _theMessageIndex: i32) -> String {
        String::new()
    }
}

// C++ gLawnApp 全局指针的 Rust 映射
pub static mut G_LAWN_APP: *mut LawnApp = std::ptr::null_mut();

// =========================================================================
// ★ 全局辅助函数 (from LawnApp.cpp)
// =========================================================================

/// C++ LawnGetCloseRequest — 检查关闭请求
pub unsafe fn LawnGetCloseRequest() -> bool {
    if G_LAWN_APP.is_null() {
        return false;
    }
    (*G_LAWN_APP).m_close_request
}

/// C++ LawnHasUsedCheatKeys — 检查是否使用过作弊键
pub unsafe fn LawnHasUsedCheatKeys() -> bool {
    if G_LAWN_APP.is_null() {
        return false;
    }
    // [TODO]: check (*G_LAWN_APP).mPlayerInfo->mHasUsedCheatKeys
    false
}

/// C++ LawnGetCurrentLevelName — 获取当前关卡名称
pub unsafe fn LawnGetCurrentLevelName() -> String {
    if G_LAWN_APP.is_null() {
        return "Before App".to_string();
    }
    match (*G_LAWN_APP).mGameScene {
        GameScenes::SCENE_LOADING => "Game Loading".to_string(),
        GameScenes::SCENE_MENU => "Game Selector".to_string(),
        GameScenes::SCENE_AWARD => "Award Screen".to_string(),
        GameScenes::SCENE_CHALLENGE => "Challenge Screen".to_string(),
        _ => format!("{:?}", (*G_LAWN_APP).mGameMode)
    }
}
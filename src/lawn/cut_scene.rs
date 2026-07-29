// [TRANSLATION_NOTE]: Cutscene.h -> Rust stub
// board.rs 引用此类型，保持基本结构可用

#![allow(non_snake_case, dead_code)]

use crate::const_enums::*;

// CutScene time constants (from CutScene.cpp static consts)
const TimeSodTutorial: i32 = 2000;
const TimeSodRegular: i32 = 2000;
const TimeGraveStones: i32 = 1000;
const TimeCrazyDave: i32 = 800;
const TimeFog: i32 = 50;
const TimeBoss: i32 = 1830;
const TimeLawnMower: i32 = 250;
const TimeReadySetPlant: i32 = 1830;
const TimeIntroEnd: i32 = 6000;
const TimeSeedChoserSlideOnEnd: i32 = 4250;
const LostTimeEnd: i32 = 11000;

pub struct CutScene {
    pub mApp: *mut crate::lawn_app::LawnApp,
    pub mBoard: *mut crate::lawn::board::Board,
    pub mCutsceneTime: i32,
    pub mSodTime: i32,
    pub mGraveStoneTime: i32,
    pub mReadySetPlantTime: i32,
    pub mFogTime: i32,
    pub mBossTime: i32,
    pub mCrazyDaveTime: i32,
    pub mLawnMowerTime: i32,
    pub mCrazyDaveDialogStart: i32,
    pub mSeedChoosing: bool,
    pub mPreloaded: bool,
    pub mPlacedZombies: bool,
    pub mPlacedLawnItems: bool,
}

impl CutScene {
    pub fn new() -> Self {
        CutScene {
            mApp: std::ptr::null_mut(),
            mBoard: std::ptr::null_mut(),
            mCutsceneTime: 0,
            mSodTime: 0,
            mGraveStoneTime: 0,
            mReadySetPlantTime: 0,
            mFogTime: 0,
            mBossTime: 0,
            mCrazyDaveTime: 0,
            mLawnMowerTime: 0,
            mCrazyDaveDialogStart: 0,
            mSeedChoosing: false,
            mPreloaded: false,
            mPlacedZombies: false,
            mPlacedLawnItems: false,
        }
    }

    pub unsafe fn StartLevelIntro(&mut self) {
        let board = self.board();
        let app = self.app();

        // Calculate intro timings based on background type and game mode
        if (*app).IsFirstTimeAdventureMode() && board.mLevel == 1 {
            self.mSodTime = TimeSodTutorial as i32;
        } else {
            self.mSodTime = TimeSodRegular as i32;
        }
        self.mGraveStoneTime = if board.StageHasGraveStones() { TimeGraveStones as i32 } else { 0 };
        self.mCrazyDaveTime = TimeCrazyDave as i32;
        self.mFogTime = if board.StageHasFog() { TimeFog as i32 } else { 0 };
        self.mBossTime = if board.IsMiniBossLevel() { TimeBoss as i32 } else { 0 };
        self.mLawnMowerTime = if (*app).IsFirstTimeAdventureMode() && board.mLevel <= 2 { TimeLawnMower as i32 } else { 0 };
        self.mReadySetPlantTime = TimeReadySetPlant as i32;
        self.mCutsceneTime = 0;

        if !board.mMenuButton.is_null() {
            (*board.mMenuButton).mBtnNoDraw = true;
        }
        board.mShowShovel = false;

        // LawnMower placement
        if (*app).IsFirstTimeAdventureMode() && board.mLevel == 1 {
            // PlaceGraveStones is handled elsewhere
        }

        // Preload effects
        // ReanimatorEnsureDefinitionLoaded(REANIM_LOADINGBAR, true);
        // app->mMusic->StartGame();
    }

    pub unsafe fn CancelIntro(&mut self) {
        self.mCutsceneTime = 99999; // Force immediate end
        self.mSeedChoosing = false;
    }

    /// C++ CutScene::Update() (from CutScene.cpp:1413)
    pub unsafe fn Update(&mut self) {
        let board = self.board();
        let app = self.app();

        if board.mPaused {
            return;
        }

        // Zombies won scene
        if (*app).mGameScene as i32 == GameScenes::SCENE_ZOMBIES_WON as i32 {
            self.mCutsceneTime += 10;
            // UpdateZombiesWon();
            return;
        }

        if (*app).mGameScene as i32 != GameScenes::SCENE_LEVEL_INTRO as i32
            || board.mBoardUpdateCounter <= 1
        {
            return;
        }

        // Preloading
        if !self.mPreloaded {
            // PreloadResources();
        }
        if !self.mPlacedZombies {
            // PlaceStreetZombies();
        }
        if self.IsNonScrollingCutscene() || !board.ChooseSeedsOnCurrentLevel() {
            // PlaceLawnItems();
        }

        // Check if cutscene should pause for seed choosing or Crazy Dave
        let mut aCutsceneTimeStop = false;
        if self.mSeedChoosing {
            aCutsceneTimeStop = true;
        }

        if !aCutsceneTimeStop {
            self.mCutsceneTime += 10;
            // Start seed chooser at the right time
            // if mCutsceneTime == TimeSeedChoserSlideOnEnd + mCrazyDaveTime && mBoard->ChooseSeedsOnCurrentLevel()
            //     StartSeedChooser();
        }

        // Check if cutscene is over
        let aTimeStart = TimeIntroEnd as i32 + self.mLawnMowerTime + self.mSodTime
            + self.mGraveStoneTime + self.mCrazyDaveTime + self.mFogTime
            + self.mBossTime + self.mReadySetPlantTime;
        if self.mCutsceneTime >= aTimeStart {
            // board->RemoveCutsceneZombies();
        if !board.mMenuButton.is_null() {
            (*board.mMenuButton).mBtnNoDraw = false;
        }
        board.mShowShovel = true;
            // app->StartPlaying();
            return;
        }

        // AnimateBoard();
    }

    pub unsafe fn IsCutSceneOver(&self) -> bool {
        // TOD_ASSERT(mApp->mGameScene == SCENE_ZOMBIES_WON);
        self.mCutsceneTime >= LostTimeEnd as i32
    }

    pub unsafe fn IsAfterSeedChooser(&self) -> bool {
        let board = self.board();
        self.mSeedChoosing
            || (!board.ChooseSeedsOnCurrentLevel())
            || self.mCutsceneTime >= TimeSeedChoserSlideOnEnd as i32 + self.mCrazyDaveTime
    }

    pub unsafe fn IsSurvivalRepick(&self) -> bool {
        let board = self.board();
        (*board.mApp).is_survival_mode() && !board.IsFinalSurvivalStage()
    }

    pub unsafe fn ShouldRunUpsellBoard(&self) -> bool {
        (*self.mApp).mGameMode as i32 == GameMode::GAMEMODE_UPSELL as i32
    }

    pub unsafe fn ZombieWonClick(&mut self) {
        if self.IsCutSceneOver() || (*self.mApp).m_tod_cheat_keys {
            // mApp->EndLevel();
        }
    }

    pub unsafe fn MouseDown(&mut self, _x: i32, _y: i32) {
        // Advance Crazy Dave dialog or handle cutscene click
    }

    pub unsafe fn IsNonScrollingCutscene(&self) -> bool {
        false
    }

    /// C++ CutScene::ShowZombieWalking — 僵尸胜利后是否在行走
    pub unsafe fn ShowZombieWalking(&self) -> bool {
        // [TODO]: Return true while zombies walk into house after losing
        true
    }

    /// C++ CutScene::StartZombiesWon — 开始僵尸胜利过场
    pub unsafe fn StartZombiesWon(&mut self) {
        // [TODO]: Start level-lost cutscene animation
        // Set up zombie walk-to-house, fade out, etc.
    }

    // Helper
    unsafe fn board(&self) -> &'static mut crate::lawn::board::Board {
        &mut *(self.mBoard)
    }
    unsafe fn app(&self) -> &'static mut crate::lawn_app::LawnApp {
        &mut *(self.mApp)
    }
}

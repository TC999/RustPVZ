// [TRANSLATION_NOTE]: Board.cpp -> Rust 模块
// 使用裸指针 + unsafe 模拟 C++ 跨结构体引用，保持 1:1 逻辑

use std::ptr;
use std::cmp;
use crate::game_constants::*;
use crate::const_enums::*;
use crate::lawn_app::LawnApp;
use crate::lawn::plant::Plant;
use crate::lawn::plant::{PlantState, PlantOnBungeeState};
use crate::lawn::zombie::Zombie;
use crate::lawn::zombie::ZOMBIE_START_RANDOM_OFFSET;
use crate::lawn::projectile::Projectile;
use crate::lawn::coin::Coin;
use crate::lawn::lawn_mower::LawnMower;
use crate::lawn::grid_item::GridItem;
use crate::sexy_app_framework::graphics::graphics::Graphics;
use crate::lawn::cursor_object::{CursorObject, CursorPreview, GameButton, ToolTipWidget};
use crate::lawn::message_widget::MessageWidget;
use crate::lawn::seed_packet::{SeedBank, SeedPacket};
use crate::lawn::cut_scene::CutScene;
use crate::lawn::challenge::Challenge;
use crate::sexy_app_framework::misc::mtrand::MTRand;
use crate::sexy_app_framework::common;
use crate::sexy_tod_lib::data_array::DataArray;
use crate::sexy_tod_lib::tod_common::{TodSmoothArray, clamp_int};
use crate::sexy_app_framework::misc::rect::Rect;
use crate::lawn::board_consts::*;

pub static mut gShownMoreSunTutorial: bool = false;

pub fn BoardInitForPlayer() {
    unsafe { gShownMoreSunTutorial = false; }
}

fn get_saved_game_name(the_game_mode: i32, the_player_id: i32) -> String {
    format!("save_{:04}_{:04}.dat", the_game_mode, the_player_id)
}

fn aGameMode_check_early_return(mode: i32) -> bool {
    mode == GameMode::GAMEMODE_CHALLENGE_ICE as i32
        || mode == GameMode::GAMEMODE_CHALLENGE_ZEN_GARDEN as i32
        || mode == GameMode::GAMEMODE_TREE_OF_WISDOM as i32
        || mode == GameMode::GAMEMODE_UPSELL as i32
        || mode == GameMode::GAMEMODE_INTRO as i32
        || mode == GameMode::GAMEMODE_CHALLENGE_FINAL_BOSS as i32
}

// Board struct - all fields public, matching C++ layout
pub struct Board {
    pub mApp: *mut LawnApp,
    pub mZombies: DataArray<Zombie>,
    pub mPlants: DataArray<Plant>,
    pub mProjectiles: DataArray<Projectile>,
    pub mCoins: DataArray<Coin>,
    pub mLawnMowers: DataArray<LawnMower>,
    pub mGridItems: DataArray<GridItem>,
    pub mCursorObject: *mut CursorObject,
    pub mCursorPreview: *mut CursorPreview,
    pub mAdvice: *mut MessageWidget,
    pub mSeedBank: *mut SeedBank,
    pub mMenuButton: *mut GameButton,
    pub mStoreButton: *mut GameButton,
    pub mIgnoreMouseUp: bool,
    pub mToolTip: *mut ToolTipWidget,
    pub mCutScene: *mut CutScene,
    pub mChallenge: *mut Challenge,
    pub mPaused: bool,
    pub mGridSquareType: [[GridSquareType; 6]; 9],
    pub mGridCelLook: [[i32; 6]; 9],
    pub mGridCelFog: [[i32; 7]; 9],
    pub mEnableGraveStones: bool,
    pub mSpecialGraveStoneX: i32,
    pub mSpecialGraveStoneY: i32,
    pub mFogOffset: f32,
    pub mFogBlownCountDown: i32,
    pub mPlantRow: [i32; 6],
    pub mWaveRowGotLawnMowered: [i32; 6],
    pub mBonusLawnMowersRemaining: i32,
    pub mIceMinX: [i32; 6],
    pub mIceTimer: [i32; 6],
    pub mIceParticleID: [u32; 6],
    pub mRowPickingArray: [TodSmoothArray; 6],
    pub mZombiesInWave: [[i32; 50]; 100],
    pub mZombieAllowed: [bool; 100],
    pub mSunCountDown: i32,
    pub mNumSunsFallen: i32,
    pub mShakeCounter: i32,
    pub mShakeAmountX: i32,
    pub mShakeAmountY: i32,
    pub mBackground: i32,
    pub mLevel: i32,
    pub mSodPosition: i32,
    pub mPrevMouseX: i32,
    pub mPrevMouseY: i32,
    pub mSunMoney: i32,
    pub mNumWaves: i32,
    pub mMainCounter: u32,
    pub mEffectCounter: u32,
    pub mBoardUpdateCounter: u32,
    pub mDrawCount: u32,
    pub mRiseFromGraveCounter: i32,
    pub mOutOfMoneyCounter: i32,
    pub mCurrentWave: i32,
    pub mTotalSpawnedWaves: i32,
    pub mTutorialState: i32,
    pub mTutorialParticleID: u32,
    pub mTutorialTimer: i32,
    pub mLastBungeeWave: i32,
    pub mZombieHealthToNextWave: i32,
    pub mZombieHealthWaveStart: i32,
    pub mZombieCountDown: i32,
    pub mZombieCountDownStart: i32,
    pub mHugeWaveCountDown: i32,
    pub mHelpDisplayed: [bool; 100],
    pub mHelpIndex: i32,
    pub mFinalBossKilled: bool,
    pub mShowShovel: bool,
    pub mCoinBankFadeCount: i32,
    pub mDebugTextMode: i32,
    pub mLevelComplete: bool,
    pub mBoardFadeOutCounter: i32,
    pub mNextSurvivalStageCounter: i32,
    pub mScoreNextMowerCounter: i32,
    pub mLevelAwardSpawned: bool,
    pub mProgressMeterWidth: i32,
    pub mFlagRaiseCounter: i32,
    pub mIceTrapCounter: i32,
    pub mBoardRandSeed: i32,
    pub mPoolSparklyParticleID: u32,
    pub mFwooshID: [[u32; 12]; 6],
    pub mFwooshCountDown: i32,
    pub mTimeStopCounter: i32,
    pub mDroppedFirstCoin: bool,
    pub mFinalWaveSoundCounter: i32,
    pub mCobCannonCursorDelayCounter: i32,
    pub mCobCannonMouseX: i32,
    pub mCobCannonMouseY: i32,
    pub mKilledYeti: bool,
    pub mMustacheMode: bool,
    pub mSuperMowerMode: bool,
    pub mFutureMode: bool,
    pub mPinataMode: bool,
    pub mDanceMode: bool,
    pub mDaisyMode: bool,
    pub mSukhbirMode: bool,
    pub mPrevBoardResult: i32,
    pub mTriggeredLawnMowers: i32,
    pub mPlayTimeActiveLevel: u32,
    pub mPlayTimeInactiveLevel: u32,
    pub mMaxSunPlants: i32,
    pub mStartDrawTime: i64,
    pub mIntervalDrawTime: i64,
    pub mIntervalDrawCountStart: u32,
    pub mMinFPS: f32,
    pub mPreloadTime: i32,
    pub mGameID: isize,
    pub mGravesCleared: u32,
    pub mPlantsEaten: u32,
    pub mPlantsShoveled: u32,
    pub mPeaShooterUsed: bool,
    pub mCatapultPlantsUsed: bool,
    pub mMushroomAndCoffeeBeansOnly: bool,
    pub mMushroomsUsed: bool,
    pub mLevelCoinsCollected: u32,
    pub mGargantuarsKillsByCornCob: u32,
    pub mCoinsCollected: u32,
    pub mDiamondsCollected: u32,
    pub mPottedPlantsCollected: u32,
    pub mChocolateCollected: u32,
    pub mClip: bool,
    // Widget base class fields (for Board's Widget inheritance in C++)
    pub mUpdateCnt: u32,
    pub mX: i32,
    pub mY: i32,
    pub mWidth: i32,
    pub mHeight: i32,
    pub mDirty: bool,
}

impl Board {
    pub fn new(theApp: *mut LawnApp) -> Self {
        Board {
            mApp: theApp,
            mZombies: DataArray::new(),
            mPlants: DataArray::new(),
            mProjectiles: DataArray::new(),
            mCoins: DataArray::new(),
            mLawnMowers: DataArray::new(),
            mGridItems: DataArray::new(),
            mCursorObject: ptr::null_mut(),
            mCursorPreview: ptr::null_mut(),
            mAdvice: ptr::null_mut(),
            mSeedBank: ptr::null_mut(),
            mMenuButton: ptr::null_mut(),
            mStoreButton: ptr::null_mut(),
            mIgnoreMouseUp: false,
            mToolTip: ptr::null_mut(),
            mCutScene: ptr::null_mut(),
            mChallenge: ptr::null_mut(),
            mPaused: false,
            mGridSquareType: [[GridSquareType::GRIDSQUARE_GRASS; 6]; 9],
            mGridCelLook: [[0i32; 6]; 9],
            mGridCelFog: [[0i32; 7]; 9],
            mEnableGraveStones: false,
            mSpecialGraveStoneX: -1,
            mSpecialGraveStoneY: -1,
            mFogOffset: 0.0,
            mFogBlownCountDown: 0,
            mPlantRow: [0i32; 6],
            mWaveRowGotLawnMowered: [0i32; 6],
            mBonusLawnMowersRemaining: 0,
            mIceMinX: [0i32; 6],
            mIceTimer: [0i32; 6],
            mIceParticleID: [0u32; 6],
            mRowPickingArray: [TodSmoothArray::new(); 6],
            mZombiesInWave: [[-1i32; 50]; 100],
            mZombieAllowed: [false; 100],
            mSunCountDown: 0,
            mNumSunsFallen: 0,
            mShakeCounter: 0,
            mShakeAmountX: 0,
            mShakeAmountY: 0,
            mBackground: 0,
            mLevel: 0,
            mSodPosition: 0,
            mPrevMouseX: -1,
            mPrevMouseY: -1,
            mSunMoney: 50,
            mNumWaves: 0,
            mMainCounter: 0,
            mEffectCounter: 0,
            mBoardUpdateCounter: 0,
            mDrawCount: 0,
            mRiseFromGraveCounter: 0,
            mOutOfMoneyCounter: 0,
            mCurrentWave: 0,
            mTotalSpawnedWaves: 0,
            mTutorialState: 0,
            mTutorialParticleID: u32::MAX,
            mTutorialTimer: -1,
            mLastBungeeWave: 0,
            mZombieHealthToNextWave: 0,
            mZombieHealthWaveStart: 0,
            mZombieCountDown: 0,
            mZombieCountDownStart: 0,
            mHugeWaveCountDown: 0,
            mHelpDisplayed: [false; 100],
            mHelpIndex: -1,
            mFinalBossKilled: false,
            mShowShovel: false,
            mCoinBankFadeCount: 0,
            mDebugTextMode: 0,
            mLevelComplete: false,
            mBoardFadeOutCounter: -1,
            mNextSurvivalStageCounter: 0,
            mScoreNextMowerCounter: 0,
            mLevelAwardSpawned: false,
            mProgressMeterWidth: 0,
            mFlagRaiseCounter: 0,
            mIceTrapCounter: 0,
            mBoardRandSeed: 0,
            mPoolSparklyParticleID: u32::MAX,
            mFwooshID: [[u32::MAX; 12]; 6],
            mFwooshCountDown: 0,
            mTimeStopCounter: 0,
            mDroppedFirstCoin: false,
            mFinalWaveSoundCounter: 0,
            mCobCannonCursorDelayCounter: 0,
            mCobCannonMouseX: 0,
            mCobCannonMouseY: 0,
            mKilledYeti: false,
            mMustacheMode: false,
            mSuperMowerMode: false,
            mFutureMode: false,
            mPinataMode: false,
            mDanceMode: false,
            mDaisyMode: false,
            mSukhbirMode: false,
            mPrevBoardResult: 0,
            mTriggeredLawnMowers: 0,
            mPlayTimeActiveLevel: 0,
            mPlayTimeInactiveLevel: 0,
            mMaxSunPlants: 0,
            mStartDrawTime: 0,
            mIntervalDrawTime: 0,
            mIntervalDrawCountStart: 0,
            mMinFPS: 1000.0,
            mPreloadTime: 0,
            mGameID: 0,
            mGravesCleared: 0,
            mPlantsEaten: 0,
            mPlantsShoveled: 0,
            mPeaShooterUsed: false,
            mCatapultPlantsUsed: false,
            mMushroomAndCoffeeBeansOnly: true,
            mMushroomsUsed: false,
            mLevelCoinsCollected: 0,
            mGargantuarsKillsByCornCob: 0,
            mCoinsCollected: 0,
            mDiamondsCollected: 0,
            mPottedPlantsCollected: 0,
            mChocolateCollected: 0,
            mClip: false,
            mUpdateCnt: 0,
            mX: 0,
            mY: 0,
            mWidth: 800,
            mHeight: 600,
            mDirty: true,
        }
    }
}

// ===== Free functions (translated 1:1 from Board.cpp) =====

pub const NUM_ZOMBIE_TYPES: i32 = 34;

#[repr(C)]
pub struct ZombiePicker {
    pub mZombieCount: i32,
    pub mZombiePoints: i32,
    pub mZombieTypeCount: [i32; 34],
    pub mAllWavesZombieTypeCount: [i32; 34],
}

pub fn ZombiePickerInitForWave(theZombiePicker: &mut ZombiePicker) {
    theZombiePicker.mZombieCount = 0;
    theZombiePicker.mZombiePoints = 0;
    for i in 0..34usize {
        theZombiePicker.mZombieTypeCount[i] = 0;
    }
}

pub fn ZombiePickerInit(theZombiePicker: &mut ZombiePicker) {
    ZombiePickerInitForWave(theZombiePicker);
    for i in 0..34usize {
        theZombiePicker.mAllWavesZombieTypeCount[i] = 0;
    }
}

// 关卡波数定义（每个冒险关卡对应的总波数）
pub static gZombieWaves: [i32; 50] = [
    3,3,3,3,3,3,3,3,3,4, // 1-10
    4,4,4,4,4,4,4,4,4,5, // 11-20
    5,5,5,5,5,5,5,5,5,5, // 21-30
    5,5,5,5,5,5,5,5,5,5, // 31-40
    5,5,5,5,5,5,5,5,5,5, // 41-50
];

// ===== Board impl (methods from Board.cpp) =====

impl Board {
    pub unsafe fn IsAdventureMode(&self) -> bool {
        (*self.mApp).mGameMode as i32 == GameMode::GAMEMODE_ADVENTURE as i32
    }

    pub unsafe fn IsFirstTimeAdventureMode(&self) -> bool {
        // TODO: check player info
        false
    }

    pub unsafe fn IsSurvivalMode(&self) -> bool {
        let mode = (*self.mApp).mGameMode as i32;
        mode >= GameMode::GAMEMODE_SURVIVAL_NORMAL_STAGE_1 as i32
            && mode <= GameMode::GAMEMODE_SURVIVAL_ENDLESS_STAGE_5 as i32
    }

    pub unsafe fn IsWhackAZombieLevel(&self) -> bool {
        (*self.mApp).mGameMode as i32 == GameMode::GAMEMODE_CHALLENGE_WHACK_A_ZOMBIE as i32
    }

    pub unsafe fn IsMiniBossLevel(&self) -> bool {
        let mode = (*self.mApp).mGameMode as i32;
        mode == GameMode::GAMEMODE_CHALLENGE_FINAL_BOSS as i32
    }

    pub unsafe fn IsStormyNightLevel(&self) -> bool {
        (*self.mApp).mGameMode as i32 == GameMode::GAMEMODE_CHALLENGE_STORMY_NIGHT as i32
    }

    pub unsafe fn IsLittleTroubleLevel(&self) -> bool {
        (*self.mApp).mGameMode as i32 == GameMode::GAMEMODE_CHALLENGE_LITTLE_TROUBLE as i32
    }

    pub unsafe fn IsBungeeBlitzLevel(&self) -> bool {
        (*self.mApp).mGameMode as i32 == GameMode::GAMEMODE_CHALLENGE_BUNGEE_BLITZ as i32
    }

    pub unsafe fn IsShovelLevel(&self) -> bool {
        (*self.mApp).mGameMode as i32 == GameMode::GAMEMODE_CHALLENGE_SHOVEL as i32
    }

    pub unsafe fn IsWallnutBowlingLevel(&self) -> bool {
        let mode = (*self.mApp).mGameMode as i32;
        mode == GameMode::GAMEMODE_CHALLENGE_WALLNUT_BOWLING as i32
            || mode == GameMode::GAMEMODE_CHALLENGE_WALLNUT_BOWLING_2 as i32
    }

    pub unsafe fn IsScaryPotterLevel(&self) -> bool {
        let mode = (*self.mApp).mGameMode as i32;
        mode >= GameMode::GAMEMODE_SCARY_POTTER_1 as i32
            && mode <= GameMode::GAMEMODE_SCARY_POTTER_ENDLESS as i32
    }

    pub unsafe fn HasFinishedAdventure(&self) -> bool {
        // TODO
        false
    }

    pub unsafe fn IsSurvivalNormal(&self, theGameMode: i32) -> bool {
        theGameMode >= GameMode::GAMEMODE_SURVIVAL_NORMAL_STAGE_1 as i32
            && theGameMode <= GameMode::GAMEMODE_SURVIVAL_NORMAL_STAGE_5 as i32
    }

    pub unsafe fn IsFlagWave(&self, theWaveNumber: i32) -> bool {
        if self.IsFirstTimeAdventureMode() && self.mLevel == 1 {
            return false;
        }
        let aWavesPerFlag = self.GetNumWavesPerFlag();
        theWaveNumber % aWavesPerFlag == aWavesPerFlag - 1
    }

    pub unsafe fn GetNumWavesPerFlag(&self) -> i32 {
        if self.IsFirstTimeAdventureMode() && self.mNumWaves < 10 { self.mNumWaves } else { 10 }
    }

    pub unsafe fn TryToSaveGame(&self) {
        if self.mBoardFadeOutCounter > 0 { return; }
        // if (NeedSaveGame()) { LawnSaveGame(self, ...); }
    }

    pub fn MakeRenderOrder(theRenderLayer: RenderLayer, theRow: i32, theLayerOffset: i32) -> i32 {
        theRow * RenderLayer::RENDER_LAYER_ROW_OFFSET as i32 + theRenderLayer as i32 + theLayerOffset
    }

    pub unsafe fn InitLevel(&mut self) {
        self.mMainCounter = 0;
        self.mBoardUpdateCounter = 0;
        self.mEnableGraveStones = false;
        self.mSodPosition = 0;
        self.mPrevBoardResult = (*self.mApp).mBoardResult as i32;

        let aGameMode = (*self.mApp).mGameMode as i32;
        if aGameMode != GameMode::GAMEMODE_TREE_OF_WISDOM as i32 && aGameMode != GameMode::GAMEMODE_CHALLENGE_ZEN_GARDEN as i32 {
            // mApp->mMusic->StopAllMusic();
        }
        self.mLevel = if self.IsAdventureMode() {
            1 // TODO: mApp->mPlayerInfo->mLevel
        } else {
            0
        };
        self.PickBackground();
        // self.InitZombieWaves();  // calls PickZombieWaves internally

        // Initial sun
        if aGameMode == GameMode::GAMEMODE_CHALLENGE_BEGHOULED as i32 || aGameMode == GameMode::GAMEMODE_CHALLENGE_BEGHOULED_TWIST as i32
            || self.IsScaryPotterLevel() || self.IsWhackAZombieLevel()
        {
            self.mSunMoney = 0;
        } else if aGameMode == GameMode::GAMEMODE_CHALLENGE_LAST_STAND as i32 {
            self.mSunMoney = 5000;
        } else if self.IsIZombieLevel() {
            self.mSunMoney = 150;
        } else if self.IsFirstTimeAdventureMode() && self.mLevel == 1 {
            self.mSunMoney = 150;
        } else {
            self.mSunMoney = 50;
        }

        // Initialize row arrays
        for aRow in 0..MAX_GRID_SIZE_Y as usize {
            self.mWaveRowGotLawnMowered[aRow] = -100;
            self.mIceMinX[aRow] = BOARD_ICE_START;
            self.mIceTimer[aRow] = 0;
            self.mIceParticleID[aRow] = u32::MAX;
            self.mRowPickingArray[aRow] = TodSmoothArray::new();
            self.mRowPickingArray[aRow].m_item = aRow as i32;
        }
        self.mNumSunsFallen = 0;
        if !self.StageIsNight() {
            self.mSunCountDown = crate::sexy_tod_lib::tod_common::rand_range_int(425, 700);
        }

        // Initialize help display flags
        for i in 0..100usize { self.mHelpDisplayed[i] = false; }

        // Initialize seed bank
        (*self.mSeedBank).mNumPackets = self.GetNumSeedsInBank();
        (*self.mSeedBank).UpdateWidth();
        for i in 0..SEEDBANK_MAX as usize {
            let packet = &mut (*self.mSeedBank).mSeedPackets[i];
            packet.mIndex = i as i32;
            packet.mX = self.GetSeedPacketPositionX(i as i32);
            packet.mY = 8;
            packet.mPacketType = SeedType::SEED_NONE;
        }

        // Fixed seed packets for specific game modes
        if self.IsSlotMachineLevel() {
            // assert mSeedBank->mNumPackets == 3
            (*self.mSeedBank).mSeedPackets[0].SetPacketType(SeedType::SEED_SUNFLOWER);
            (*self.mSeedBank).mSeedPackets[1].SetPacketType(SeedType::SEED_PEASHOOTER);
            (*self.mSeedBank).mSeedPackets[2].SetPacketType(SeedType::SEED_SNOWPEA);
        } else if aGameMode == GameMode::GAMEMODE_CHALLENGE_ICE as i32 {
            // assert 6 packets
            (*self.mSeedBank).mSeedPackets[0].SetPacketType(SeedType::SEED_PEASHOOTER);
            (*self.mSeedBank).mSeedPackets[1].SetPacketType(SeedType::SEED_CHERRYBOMB);
            (*self.mSeedBank).mSeedPackets[2].SetPacketType(SeedType::SEED_WALLNUT);
            (*self.mSeedBank).mSeedPackets[3].SetPacketType(SeedType::SEED_REPEATER);
            (*self.mSeedBank).mSeedPackets[4].SetPacketType(SeedType::SEED_SNOWPEA);
            (*self.mSeedBank).mSeedPackets[5].SetPacketType(SeedType::SEED_CHOMPER);
        } else if aGameMode == GameMode::GAMEMODE_CHALLENGE_PUZZLE_I_ZOMBIE_1 as i32 {
            (*self.mSeedBank).mSeedPackets[0].SetPacketType(SeedType::SEED_ZOMBIE_NORMAL);
            (*self.mSeedBank).mSeedPackets[1].SetPacketType(SeedType::SEED_ZOMBIE_PAIL);
            (*self.mSeedBank).mSeedPackets[2].SetPacketType(SeedType::SEED_ZOMBIE_FOOTBALL);
        } else if aGameMode == GameMode::GAMEMODE_CHALLENGE_PUZZLE_I_ZOMBIE_2 as i32 {
            (*self.mSeedBank).mSeedPackets[0].SetPacketType(SeedType::SEED_ZOMBIE_NORMAL);
            (*self.mSeedBank).mSeedPackets[1].SetPacketType(SeedType::SEED_ZOMBIE_SCREEN_DOOR);
            (*self.mSeedBank).mSeedPackets[2].SetPacketType(SeedType::SEED_ZOMBIE_PAIL);
        } else if aGameMode == GameMode::GAMEMODE_CHALLENGE_PUZZLE_I_ZOMBIE_3 as i32 {
            (*self.mSeedBank).mSeedPackets[0].SetPacketType(SeedType::SEED_ZOMBIE_NORMAL);
            (*self.mSeedBank).mSeedPackets[1].SetPacketType(SeedType::SEED_ZOMBIE_PAIL);
            (*self.mSeedBank).mSeedPackets[2].SetPacketType(SeedType::SEED_ZOMBIE_DIGGER);
        } else if aGameMode == GameMode::GAMEMODE_CHALLENGE_PUZZLE_I_ZOMBIE_4 as i32 {
            (*self.mSeedBank).mSeedPackets[0].SetPacketType(SeedType::SEED_ZOMBIE_NORMAL);
            (*self.mSeedBank).mSeedPackets[1].SetPacketType(SeedType::SEED_ZOMBIE_PAIL);
            (*self.mSeedBank).mSeedPackets[2].SetPacketType(SeedType::SEED_ZOMBIE_LADDER);
        } else if aGameMode == GameMode::GAMEMODE_CHALLENGE_PUZZLE_I_ZOMBIE_5 as i32 {
            (*self.mSeedBank).mSeedPackets[0].SetPacketType(SeedType::SEED_ZOMBIE_NORMAL);
            (*self.mSeedBank).mSeedPackets[1].SetPacketType(SeedType::SEED_ZOMBIE_PAIL);
            (*self.mSeedBank).mSeedPackets[2].SetPacketType(SeedType::SEED_ZOMBIE_BUNGEE);
            (*self.mSeedBank).mSeedPackets[3].SetPacketType(SeedType::SEED_ZOMBIE_BALLOON);
        } else if aGameMode == GameMode::GAMEMODE_CHALLENGE_PUZZLE_I_ZOMBIE_6 as i32 {
            (*self.mSeedBank).mSeedPackets[0].SetPacketType(SeedType::SEED_ZOMBIE_NORMAL);
            (*self.mSeedBank).mSeedPackets[1].SetPacketType(SeedType::SEED_ZOMBIE_POLEVAULTER);
            (*self.mSeedBank).mSeedPackets[2].SetPacketType(SeedType::SEED_ZOMBIE_PAIL);
            (*self.mSeedBank).mSeedPackets[3].SetPacketType(SeedType::SEED_ZOMBIE_GARGANTUAR);
        } else if aGameMode == GameMode::GAMEMODE_CHALLENGE_PUZZLE_I_ZOMBIE_7 as i32 {
            (*self.mSeedBank).mSeedPackets[0].SetPacketType(SeedType::SEED_ZOMBIE_NORMAL);
            (*self.mSeedBank).mSeedPackets[1].SetPacketType(SeedType::SEED_ZOMBIE_POLEVAULTER);
            (*self.mSeedBank).mSeedPackets[2].SetPacketType(SeedType::SEED_ZOMBIE_PAIL);
            (*self.mSeedBank).mSeedPackets[3].SetPacketType(SeedType::SEED_ZOMBIE_DANCER);
        } else if aGameMode == GameMode::GAMEMODE_CHALLENGE_PUZZLE_I_ZOMBIE_8 as i32 {
            (*self.mSeedBank).mSeedPackets[0].SetPacketType(SeedType::SEED_ZOMBIE_IMP);
            (*self.mSeedBank).mSeedPackets[1].SetPacketType(SeedType::SEED_ZOMBIE_TRAFFIC_CONE);
            (*self.mSeedBank).mSeedPackets[2].SetPacketType(SeedType::SEED_ZOMBIE_PAIL);
            (*self.mSeedBank).mSeedPackets[3].SetPacketType(SeedType::SEED_ZOMBIE_BUNGEE);
            (*self.mSeedBank).mSeedPackets[4].SetPacketType(SeedType::SEED_ZOMBIE_DIGGER);
            (*self.mSeedBank).mSeedPackets[5].SetPacketType(SeedType::SEED_ZOMBIE_LADDER);
        } else if aGameMode == GameMode::GAMEMODE_CHALLENGE_PUZZLE_I_ZOMBIE_9 as i32 {
            (*self.mSeedBank).mSeedPackets[0].SetPacketType(SeedType::SEED_ZOMBIE_IMP);
            (*self.mSeedBank).mSeedPackets[1].SetPacketType(SeedType::SEED_ZOMBIE_TRAFFIC_CONE);
            (*self.mSeedBank).mSeedPackets[2].SetPacketType(SeedType::SEED_ZOMBIE_POLEVAULTER);
            (*self.mSeedBank).mSeedPackets[3].SetPacketType(SeedType::SEED_ZOMBIE_PAIL);
            (*self.mSeedBank).mSeedPackets[4].SetPacketType(SeedType::SEED_ZOMBIE_BUNGEE);
            (*self.mSeedBank).mSeedPackets[5].SetPacketType(SeedType::SEED_ZOMBIE_DIGGER);
            (*self.mSeedBank).mSeedPackets[6].SetPacketType(SeedType::SEED_ZOMBIE_LADDER);
            (*self.mSeedBank).mSeedPackets[7].SetPacketType(SeedType::SEED_ZOMBIE_FOOTBALL);
        } else if aGameMode == GameMode::GAMEMODE_CHALLENGE_PUZZLE_I_ZOMBIE_ENDLESS as i32 {
            (*self.mSeedBank).mSeedPackets[0].SetPacketType(SeedType::SEED_ZOMBIE_IMP);
            (*self.mSeedBank).mSeedPackets[1].SetPacketType(SeedType::SEED_ZOMBIE_TRAFFIC_CONE);
            (*self.mSeedBank).mSeedPackets[2].SetPacketType(SeedType::SEED_ZOMBIE_POLEVAULTER);
            (*self.mSeedBank).mSeedPackets[3].SetPacketType(SeedType::SEED_ZOMBIE_PAIL);
            (*self.mSeedBank).mSeedPackets[4].SetPacketType(SeedType::SEED_ZOMBIE_BUNGEE);
            (*self.mSeedBank).mSeedPackets[5].SetPacketType(SeedType::SEED_ZOMBIE_DIGGER);
            (*self.mSeedBank).mSeedPackets[6].SetPacketType(SeedType::SEED_ZOMBIE_LADDER);
            (*self.mSeedBank).mSeedPackets[7].SetPacketType(SeedType::SEED_ZOMBIE_FOOTBALL);
            (*self.mSeedBank).mSeedPackets[8].SetPacketType(SeedType::SEED_ZOMBIE_DANCER);
        } else if self.IsScaryPotterLevel() {
            (*self.mSeedBank).mSeedPackets[0].SetPacketType(SeedType::SEED_CHERRYBOMB);
        } else if self.IsWhackAZombieLevel() {
            (*self.mSeedBank).mSeedPackets[0].SetPacketType(SeedType::SEED_POTATOMINE);
            (*self.mSeedBank).mSeedPackets[1].SetPacketType(SeedType::SEED_GRAVEBUSTER);
            (*self.mSeedBank).mSeedPackets[2].SetPacketType(
                if self.IsAdventureMode() { SeedType::SEED_CHERRYBOMB } else { SeedType::SEED_ICESHROOM }
            );
        } else if aGameMode == GameMode::GAMEMODE_CHALLENGE_ZOMBIQUARIUM as i32 {
            (*self.mSeedBank).mSeedPackets[0].SetPacketType(SeedType::SEED_ZOMBIQUARIUM_SNORKLE);
            (*self.mSeedBank).mSeedPackets[1].SetPacketType(SeedType::SEED_ZOMBIQUARIUM_TROPHY);
        } else if !self.ChooseSeedsOnCurrentLevel() && !self.HasConveyorBeltSeedBank() {
            (*self.mSeedBank).mNumPackets = self.GetNumSeedsInBank();
            for i in 0..(*self.mSeedBank).mNumPackets as usize {
                (*self.mSeedBank).mSeedPackets[i].SetPacketType(unsafe { std::mem::transmute(i as i32) });
            }
        }

        // MarkAllDirty()
        self.mPaused = false;
        self.mOutOfMoneyCounter = 0;
        if self.StageHasFog() {
            self.mFogBlownCountDown = 200;
            self.mFogOffset = (1065 - self.LeftFogColumn() * 80) as f32;
        }
        // mChallenge->InitLevel();
    }

    // === Helper Methods ===
    pub unsafe fn StageIsNight(&self) -> bool {
        self.mBackground as i32 == BackgroundType::BACKGROUND_2_NIGHT as i32
    }

    pub unsafe fn StageHasFog(&self) -> bool {
        self.mBackground as i32 == BackgroundType::BACKGROUND_4_FOG as i32
    }

    pub unsafe fn StageHasGraveStones(&self) -> bool {
        self.mBackground as i32 == BackgroundType::BACKGROUND_2_NIGHT as i32
    }

    pub unsafe fn IsIZombieLevel(&self) -> bool {
        let mode = (*self.mApp).mGameMode as i32;
        mode >= GameMode::GAMEMODE_CHALLENGE_PUZZLE_I_ZOMBIE_1 as i32
            && mode <= GameMode::GAMEMODE_CHALLENGE_PUZZLE_I_ZOMBIE_ENDLESS as i32
    }

    pub unsafe fn IsSlotMachineLevel(&self) -> bool {
        (*self.mApp).mGameMode as i32 == GameMode::GAMEMODE_CHALLENGE_SLOT_MACHINE as i32
    }

    pub unsafe fn ChooseSeedsOnCurrentLevel(&self) -> bool {
        let mode = (*self.mApp).mGameMode as i32;
        mode == GameMode::GAMEMODE_ADVENTURE as i32
            || mode == GameMode::GAMEMODE_CHALLENGE_LAST_STAND as i32
        // In the full game this has more conditions
    }

    pub unsafe fn HasConveyorBeltSeedBank(&self) -> bool {
        false // stub
    }

    pub unsafe fn GetNumSeedsInBank(&self) -> i32 {
        if self.ChooseSeedsOnCurrentLevel() { 7 } else { 10 }
    }

    pub unsafe fn GetSeedPacketPositionX(&self, index: i32) -> i32 {
        index * 55 + 15 // stub - actual position calculation
    }

    pub unsafe fn LeftFogColumn(&self) -> i32 {
        0 // stub
    }

    pub unsafe fn StageHasZombieWalkInFromRight(&self) -> bool {
        true // stub
    }

    // === DataArray iteration helpers ===
    pub unsafe fn IterateZombies(&self, theItem: &mut *mut Zombie) -> bool {
        self.mZombies.iterate_next(theItem)
    }
    pub unsafe fn IteratePlants(&self, theItem: &mut *mut Plant) -> bool {
        self.mPlants.iterate_next(theItem)
    }
    pub unsafe fn IterateProjectiles(&self, theItem: &mut *mut Projectile) -> bool {
        self.mProjectiles.iterate_next(theItem)
    }
    pub unsafe fn IterateCoins(&self, theItem: &mut *mut Coin) -> bool {
        self.mCoins.iterate_next(theItem)
    }
    pub unsafe fn IterateLawnMowers(&self, theItem: &mut *mut LawnMower) -> bool {
        self.mLawnMowers.iterate_next(theItem)
    }
    pub unsafe fn IterateGridItems(&self, theItem: &mut *mut GridItem) -> bool {
        self.mGridItems.iterate_next(theItem)
    }

    pub unsafe fn PlaceRake(&mut self) {
        // if !mApp->mPlayerInfo->mPurchases[STORE_ITEM_RAKE] return;
        let mut aGridX = 7;
        if self.IsScaryPotterLevel() {
            let mut aGridItem: *mut GridItem = std::ptr::null_mut();
            while self.IterateGridItems(&mut aGridItem) {
                if (*aGridItem).mGridItemType as i32 == GridItemType::GRIDITEM_SCARY_POT as i32
                    && (*aGridItem).mGridX <= aGridX && (*aGridItem).mGridX > 0
                {
                    aGridX = (*aGridItem).mGridX - 1;
                }
            }
        } else {
            if !self.StageHasZombieWalkInFromRight()
                || (*self.mApp).mGameMode as i32 == GameMode::GAMEMODE_CHALLENGE_BEGHOULED as i32
                || (*self.mApp).mGameMode as i32 == GameMode::GAMEMODE_CHALLENGE_BEGHOULED_TWIST as i32
                || (*self.mApp).mGameMode as i32 == GameMode::GAMEMODE_CHALLENGE_BOBSLED_BONANZA as i32
            {
                return;
            }
        }

        let mut aPickCount = 0i32;
        let mut aPickArray: [crate::sexy_tod_lib::tod_common::TodWeightedArray; 6] = unsafe { std::mem::zeroed() };
        for aRow in 0..MAX_GRID_SIZE_Y as usize {
            if aRow != 5 && self.mPlantRow[aRow] == 1 {
                aPickArray[aPickCount as usize].m_weight = 1;
                aPickArray[aPickCount as usize].m_item = aRow as isize;
                aPickCount += 1;
            }
        }
        if aPickCount == 0 { return; }

        let aGridY = crate::sexy_tod_lib::tod_common::tod_pick_from_weighted_array(&aPickArray[..aPickCount as usize]);
        // mApp->mPlayerInfo->mPurchases[STORE_ITEM_RAKE]--;
        let aRake = self.mGridItems.data_array_alloc();
        (*aRake).mGridItemType = GridItemType::GRIDITEM_RAKE;
        (*aRake).mGridX = aGridX;
        (*aRake).mGridY = aGridY as i32;
        (*aRake).mPosX = self.GridToPixelX(aGridX, aGridY as i32) as f32;
        (*aRake).mPosY = self.GridToPixelY(aGridX, aGridY as i32) as f32;
        (*aRake).mRenderOrder = Board::MakeRenderOrder(RenderLayer::RENDER_LAYER_GRAVE_STONE, aGridY as i32, 9);
    }

    pub unsafe fn InitLawnMowers(&mut self) {
        let aGameMode = (*self.mApp).mGameMode as i32;
        if aGameMode == GameMode::GAMEMODE_CHALLENGE_BEGHOULED as i32
            || aGameMode == GameMode::GAMEMODE_CHALLENGE_BEGHOULED_TWIST as i32
            || aGameMode == GameMode::GAMEMODE_CHALLENGE_ZEN_GARDEN as i32
            || aGameMode == GameMode::GAMEMODE_TREE_OF_WISDOM as i32
            || aGameMode == GameMode::GAMEMODE_CHALLENGE_LAST_STAND as i32
            || aGameMode == GameMode::GAMEMODE_CHALLENGE_ZOMBIQUARIUM as i32
            || self.IsIZombieLevel()
        {
            return;
        }

        for aRow in 0..MAX_GRID_SIZE_Y as usize {
            if self.mPlantRow[aRow] != 0 /* PLANTROW_DIRT */ {
                let aLawnMower = self.mLawnMowers.data_array_alloc();
                // aLawnMower->LawnMowerInitialize(aRow);
                (*aLawnMower).mRow = aRow as i32;
                (*aLawnMower).mVisible = false;
            }
        }
    }

    pub unsafe fn StartLevel(&mut self) {
        self.mCoinBankFadeCount = 0;
        // mApp->mLastLevelStats->Reset();
        // mChallenge->StartLevel();

        let aSurvivalStage = (*self.mApp).mGameMode as u32 - GameMode::GAMEMODE_SURVIVAL_ENDLESS_STAGE_1 as u32;
        if aSurvivalStage <= 4 {
            // ReportAchievement check
        }

        if self.IsSurvivalMode() && !self.mChallenge.is_null() && (*self.mChallenge).mSurvivalStage > 0 {
            // FreezeEffectsForCutscene(false);
            // mApp->mSoundSystem->GamePause(false);
        }

        if aGameMode_check_early_return((*self.mApp).mGameMode as i32) {
            return;
        }
        // mApp->mMusic->StartGameMusic();
    }

    pub unsafe fn GetBottomLawnMower(&self) -> *mut LawnMower {
        let mut aLawnMower: *mut LawnMower = std::ptr::null_mut();
        let mut aBottomMower: *mut LawnMower = std::ptr::null_mut();
        while self.IterateLawnMowers(&mut aLawnMower) {
            if (*aLawnMower).mMowerState as i32 == MowerState::MOWER_TRIGGERED as i32
                || (*aLawnMower).mMowerState as i32 == MowerState::MOWER_TRIGGERED_SQUASHED as i32
            {
                continue;
            }
            if aBottomMower.is_null() || (*aBottomMower).mRow < (*aLawnMower).mRow {
                aBottomMower = aLawnMower;
            }
        }
        aBottomMower
    }

    pub unsafe fn UpdateLevelEndSequence(&mut self) {
        if self.mNextSurvivalStageCounter > 0 {
            // if !IsScaryPotterDaveTalking()
            {
                self.mNextSurvivalStageCounter -= 1;
            }

            if self.mNextSurvivalStageCounter == 1 && self.IsSurvivalMode() {
                self.TryToSaveGame();
            }

            if self.mNextSurvivalStageCounter == 0 {
                if self.IsScaryPotterLevel() {
                    if self.IsAdventureMode() { return; }
                    // if !IsFinalScaryPotterStage() {
                    //     mChallenge->PuzzleNextStageClear();
                    //     mChallenge->ScaryPotterPopulate();
                    // }
                } else if (*self.mApp).mGameMode as i32 == GameMode::GAMEMODE_CHALLENGE_LAST_STAND as i32 {
                    // ClearAdvice(ADVICE_NONE);
                } else {
                    self.mLevelComplete = true;
                    // RemoveZombiesForRepick();
                }
                return;
            }
        }

        if self.mBoardFadeOutCounter < 0 { return; }

        self.mBoardFadeOutCounter -= 1;
        if self.mBoardFadeOutCounter == 0 {
            self.mLevelComplete = true;
            return;
        }
        if self.mBoardFadeOutCounter == 300 {
            // Play sample SOUND_LIGHTFILL
        }

        if self.mScoreNextMowerCounter > 0 {
            self.mScoreNextMowerCounter -= 1;
            if self.mScoreNextMowerCounter != 0 { return; }
        }

        // if CanDropLoot() && !IsSurvivalStageWithRepick() {
        //     self.mScoreNextMowerCounter = 40;
        //     LawnMower* aLawnMower = GetBottomLawnMower();
        // }
    }

    // === Helpers ===
    pub unsafe fn GridToPixelX(&self, gridX: i32, _gridY: i32) -> i32 {
        gridX * 80 + 40 // stub
    }

    pub unsafe fn GridToPixelY(&self, _gridX: i32, gridY: i32) -> i32 {
        80 + gridY * 100 // stub
    }

    pub unsafe fn PickZombieWaves(&mut self) {
        // 设定关卡总波数
        if self.IsAdventureMode() {
            if self.IsWhackAZombieLevel() {
                self.mNumWaves = 8;
            } else {
                self.mNumWaves = gZombieWaves[clamp_int(self.mLevel - 1, 0, 49) as usize];
                if !self.IsFirstTimeAdventureMode() && !self.IsMiniBossLevel() {
                    if self.mNumWaves < 10 { self.mNumWaves = 20; } else { self.mNumWaves += 10; }
                }
            }
        } else {
            let aGameMode = (*self.mApp).mGameMode as i32;
            if self.IsSurvivalMode() || aGameMode == GameMode::GAMEMODE_CHALLENGE_LAST_STAND as i32 {
                self.mNumWaves = self.GetNumWavesPerSurvivalStage();
            } else if aGameMode == GameMode::GAMEMODE_CHALLENGE_ZEN_GARDEN as i32 || aGameMode == GameMode::GAMEMODE_TREE_OF_WISDOM as i32 || false /*IsSquirrelLevel*/ {
                self.mNumWaves = 0;
            } else if aGameMode == GameMode::GAMEMODE_CHALLENGE_WHACK_A_ZOMBIE as i32 {
                self.mNumWaves = 12;
            } else if aGameMode == GameMode::GAMEMODE_CHALLENGE_WALLNUT_BOWLING as i32 || aGameMode == GameMode::GAMEMODE_CHALLENGE_AIR_RAID as i32
                || aGameMode == GameMode::GAMEMODE_CHALLENGE_GRAVE_DANGER as i32 || aGameMode == GameMode::GAMEMODE_CHALLENGE_HIGH_GRAVITY as i32
                || aGameMode == GameMode::GAMEMODE_CHALLENGE_PORTAL_COMBAT as i32 || aGameMode == GameMode::GAMEMODE_CHALLENGE_WAR_AND_PEAS as i32
                || aGameMode == GameMode::GAMEMODE_CHALLENGE_INVISIGHOUL as i32 {
                self.mNumWaves = 20;
            } else if self.IsStormyNightLevel() || self.IsLittleTroubleLevel() || self.IsBungeeBlitzLevel()
                || aGameMode == GameMode::GAMEMODE_CHALLENGE_COLUMN as i32 || self.IsShovelLevel() || aGameMode == GameMode::GAMEMODE_CHALLENGE_WAR_AND_PEAS_2 as i32
                || aGameMode == GameMode::GAMEMODE_CHALLENGE_WALLNUT_BOWLING_2 as i32 || aGameMode == GameMode::GAMEMODE_CHALLENGE_POGO_PARTY as i32 {
                self.mNumWaves = 30;
            } else {
                self.mNumWaves = 40;
            }
        }

        let aZombiePicker = ZombiePicker {
            mZombieCount: 0, mZombiePoints: 0,
            mZombieTypeCount: [0i32; 34], mAllWavesZombieTypeCount: [0i32; 34],
        };
        let mut aZombiePicker = std::cell::UnsafeCell::new(aZombiePicker);
        let zp = aZombiePicker.get();
        ZombiePickerInit(&mut *zp);
        let aIntroZombieType = self.GetIntroducedZombieType();

        for aWave in 0..self.mNumWaves {
            ZombiePickerInitForWave(&mut *zp);
            self.mZombiesInWave[aWave as usize][0] = ZombieType::ZOMBIE_INVALID as i32;

            let aIsFlagWave = self.IsFlagWave(aWave);
            let aIsFinalWave = aWave == self.mNumWaves - 1;

            if self.IsBungeeBlitzLevel() && aIsFlagWave {
                for _ in 0..5 {
                    self.PutZombieInWave(ZombieType::ZOMBIE_BUNGEE as i32, aWave, &mut *zp);
                }
                if !aIsFinalWave { continue; }
            }

            let aZombiePoints = &mut (*zp).mZombiePoints;
            if (*self.mApp).mGameMode as i32 == GameMode::GAMEMODE_CHALLENGE_LAST_STAND as i32 {
                let survival_stage = if !self.mChallenge.is_null() { (*self.mChallenge).mSurvivalStage } else { 0 };
                *aZombiePoints = (survival_stage * self.GetNumWavesPerSurvivalStage() + aWave + 10) * 2 / 5 + 1;
            } else if self.IsSurvivalMode() && !self.mChallenge.is_null() && (*self.mChallenge).mSurvivalStage > 0 {
                let survival_stage = (*self.mChallenge).mSurvivalStage;
                *aZombiePoints = (survival_stage * self.GetNumWavesPerSurvivalStage() + aWave) * 2 / 5 + 1;
            } else if self.IsAdventureMode() && self.HasFinishedAdventure() && self.mLevel != 5 {
                *aZombiePoints = aWave * 2 / 5 + 1;
            } else {
                *aZombiePoints = aWave / 3 + 1;
            }

            if aIsFlagWave {
                let aPlainZombiesNum = cmp::min(*aZombiePoints, 8);
                *aZombiePoints = (*aZombiePoints as f32 * 2.5) as i32;
                if (*self.mApp).mGameMode as i32 != GameMode::GAMEMODE_CHALLENGE_WAR_AND_PEAS as i32
                    && (*self.mApp).mGameMode as i32 != GameMode::GAMEMODE_CHALLENGE_WAR_AND_PEAS_2 as i32
                {
                    for _ in 0..aPlainZombiesNum {
                        self.PutZombieInWave(ZombieType::ZOMBIE_NORMAL as i32, aWave, &mut *zp);
                    }
                    self.PutZombieInWave(ZombieType::ZOMBIE_FLAG as i32, aWave, &mut *zp);
                }
            }

            // Mode-specific multiplier
            if (*self.mApp).mGameMode as i32 == GameMode::GAMEMODE_CHALLENGE_COLUMN as i32 {
                *aZombiePoints *= 6;
            } else if self.IsLittleTroubleLevel() || self.IsWallnutBowlingLevel() {
                *aZombiePoints *= 4;
            } else if self.IsMiniBossLevel() {
                *aZombiePoints *= 3;
            } else if self.IsStormyNightLevel() && self.IsAdventureMode() {
                *aZombiePoints *= 3;
            } else if self.IsShovelLevel() || self.IsBungeeBlitzLevel()
                || (*self.mApp).mGameMode as i32 == GameMode::GAMEMODE_CHALLENGE_PORTAL_COMBAT as i32
                || (*self.mApp).mGameMode as i32 == GameMode::GAMEMODE_CHALLENGE_INVISIGHOUL as i32 {
                *aZombiePoints *= 2;
            }

            // Fixed zombie types
            if aIntroZombieType as i32 != ZombieType::ZOMBIE_INVALID as i32 && aIntroZombieType as i32 != ZombieType::ZOMBIE_DUCKY_TUBE as i32 {
                let mut aSpawnIntro = false;
                if aIntroZombieType as i32 == ZombieType::ZOMBIE_DIGGER as i32 || aIntroZombieType as i32 == ZombieType::ZOMBIE_BALLOON as i32 {
                    if aWave + 1 == 7 || aIsFinalWave { aSpawnIntro = true; }
                } else if aIntroZombieType as i32 == ZombieType::ZOMBIE_YETI as i32 {
                    if aWave == self.mNumWaves / 2 && !(*self.mApp).mSawYeti { aSpawnIntro = true; }
                } else if aWave == self.mNumWaves / 2 || aIsFinalWave {
                    aSpawnIntro = true;
                }
                if aSpawnIntro {
                    self.PutZombieInWave(aIntroZombieType as i32, aWave, &mut *zp);
                }
            }

            if self.mLevel == 50 && aIsFinalWave {
                self.PutZombieInWave(ZombieType::ZOMBIE_GARGANTUAR as i32, aWave, &mut *zp);
            }
            if self.IsAdventureMode() && aIsFinalWave {
                self.PutInMissingZombies(aWave, &mut *zp);
            }
            if (*self.mApp).mGameMode as i32 == GameMode::GAMEMODE_CHALLENGE_COLUMN as i32 {
                if aWave % 10 == 5 {
                    for _ in 0..10 { self.PutZombieInWave(ZombieType::ZOMBIE_LADDER as i32, aWave, &mut *zp); }
                }
                if aWave % 10 == 8 {
                    for _ in 0..10 { self.PutZombieInWave(ZombieType::ZOMBIE_JACK_IN_THE_BOX as i32, aWave, &mut *zp); }
                }
                if aWave == 19 { for _ in 0..3 { self.PutZombieInWave(ZombieType::ZOMBIE_GARGANTUAR as i32, aWave, &mut *zp); } }
                if aWave == 29 { for _ in 0..5 { self.PutZombieInWave(ZombieType::ZOMBIE_GARGANTUAR as i32, aWave, &mut *zp); } }
            }

            // Remaining zombie points -> random zombies
            while *aZombiePoints > 0 && (*zp).mZombieCount < MAX_ZOMBIES_IN_WAVE as i32 {
                let aZombieType = self.PickZombieType(*aZombiePoints, aWave, &mut *zp);
                self.PutZombieInWave(aZombieType as i32, aWave, &mut *zp);
            }
        }
    }

    pub unsafe fn GetLevelRandSeed(&self) -> i32 {
        let mut aRndSeed = (if !(*self.mApp).mPlayerInfo.is_null() { *((*self.mApp).mPlayerInfo as *const i32) } else { 0i32 }) + self.mBoardRandSeed;
        if self.IsAdventureMode() {
            aRndSeed += (if !(*self.mApp).mPlayerInfo.is_null() { *((*self.mApp).mPlayerInfo as *const i32).add(1) } else { 0i32 }) * 101 + self.mLevel;
        } else {
            aRndSeed += (if !self.mChallenge.is_null() { unsafe { (*self.mChallenge).mSurvivalStage } } else { 0i32 }) * 101 + (*self.mApp).mGameMode as i32;
        }
        aRndSeed
    }

    pub unsafe fn LoadBackgroundImages(&mut self) {
        match self.mBackground as i32 {
            x if x == BackgroundType::BACKGROUND_1_DAY as i32 => {
                // mLoadedResourceNames.push_back("DelayLoad_Background1");
            }
            x if x == BackgroundType::BACKGROUND_2_NIGHT as i32 => {}
            x if x == BackgroundType::BACKGROUND_3_POOL as i32 => {}
            x if x == BackgroundType::BACKGROUND_4_FOG as i32 => {}
            x if x == BackgroundType::BACKGROUND_5_ROOF as i32 => {}
            x if x == BackgroundType::BACKGROUND_6_BOSS as i32 => {}
            x if x == BackgroundType::BACKGROUND_GREENHOUSE as i32 => {}
            x if x == BackgroundType::BACKGROUND_TREEOFWISDOM as i32 => {}
            x if x == BackgroundType::BACKGROUND_ZOMBIQUARIUM as i32 => {}
            x if x == BackgroundType::BACKGROUND_MUSHROOM_GARDEN as i32 => {}
            _ => { /* TOD_ASSERT(false) */ }
        }
    }

    pub unsafe fn PickBackground(&mut self) {
        let aGameMode = unsafe { (*self.mApp).mGameMode };
        match aGameMode as i32 {
            x if x == GameMode::GAMEMODE_ADVENTURE as i32 => {
                if self.mLevel <= 1 * LEVELS_PER_AREA {
                    self.mBackground = BackgroundType::BACKGROUND_1_DAY as i32;
                } else if self.mLevel <= 2 * LEVELS_PER_AREA {
                    self.mBackground = BackgroundType::BACKGROUND_2_NIGHT as i32;
                } else if self.mLevel <= 3 * LEVELS_PER_AREA {
                    self.mBackground = BackgroundType::BACKGROUND_3_POOL as i32;
                } else if self.IsScaryPotterLevel() {
                    self.mBackground = BackgroundType::BACKGROUND_2_NIGHT as i32;
                } else if self.mLevel <= 4 * LEVELS_PER_AREA {
                    self.mBackground = BackgroundType::BACKGROUND_4_FOG as i32;
                } else if self.mLevel < FINAL_LEVEL {
                    self.mBackground = BackgroundType::BACKGROUND_5_ROOF as i32;
                } else if self.mLevel == FINAL_LEVEL {
                    self.mBackground = BackgroundType::BACKGROUND_6_BOSS as i32;
                } else {
                    self.mBackground = BackgroundType::BACKGROUND_1_DAY as i32;
                }
            }
            // Survival/Challenge stages map to backgrounds
            _ => {
                self.mBackground = BackgroundType::BACKGROUND_1_DAY as i32;
            }
        }
        self.LoadBackgroundImages();

        // Set plant rows based on background
        for y in 0..MAX_GRID_SIZE_Y as usize {
            self.mPlantRow[y] = if y < 5 { 1 } else { 0 }; // PLANTROW_NORMAL or DIRT
        }

        // Place grave stones
        let aLevelRNG = MTRand::with_seed(self.GetLevelRandSeed() as u32);
        // TODO: StageHasGraveStones check and AddGraveStones calls
    }

    pub unsafe fn InitZombieWavesForLevel(&mut self, theForLevel: i32) {
        if self.IsWhackAZombieLevel() || (self.IsWallnutBowlingLevel() && !self.IsFirstTimeAdventureMode()) {
            if let Some(c) = self.mChallenge.as_mut() {
                // c.InitZombieWaves();
            }
            return;
        }
        for aZombieType in (ZombieType::ZOMBIE_NORMAL as i32)..(ZombieType::NUM_ZOMBIE_TYPES as i32) {
            self.mZombieAllowed[aZombieType as usize] = self.CanZombieSpawnOnLevel(aZombieType as i32, theForLevel);
        }
    }

    pub unsafe fn PutZombieInWave(&mut self, theZombieType: i32, theWaveNumber: i32, theZombiePicker: &mut ZombiePicker) {
        let count = theZombiePicker.mZombieCount as usize;
        self.mZombiesInWave[theWaveNumber as usize][count] = theZombieType;
        theZombiePicker.mZombieCount += 1;
        if (theZombiePicker.mZombieCount as usize) < MAX_ZOMBIES_IN_WAVE as usize {
            self.mZombiesInWave[theWaveNumber as usize][theZombiePicker.mZombieCount as usize] = ZombieType::ZOMBIE_INVALID as i32;
        }
        // theZombiePicker.mZombiePoints -= GetZombieDefinition(theZombieType).mZombieValue;
        theZombiePicker.mZombieTypeCount[theZombieType as usize] += 1;
        theZombiePicker.mAllWavesZombieTypeCount[theZombieType as usize] += 1;
    }

    pub unsafe fn PutInMissingZombies(&mut self, theWaveNumber: i32, theZombiePicker: &mut ZombiePicker) {
        for aZombieType in 0..34i32 {
            if theZombiePicker.mZombieTypeCount[aZombieType as usize] <= 0
                && aZombieType != ZombieType::ZOMBIE_YETI as i32
                && self.CanZombieSpawnOnLevel(aZombieType, self.mLevel)
            {
                self.PutZombieInWave(aZombieType, theWaveNumber, theZombiePicker);
            }
        }
    }

    pub unsafe fn GetIntroducedZombieType(&self) -> i32 {
        // TODO: implement based on level
        ZombieType::ZOMBIE_INVALID as i32
    }

    pub unsafe fn PickZombieType(&self, _theZombiePoints: i32, _theWaveIndex: i32, _theZombiePicker: &mut ZombiePicker) -> i32 {
        // TODO: proper zombie type picking
        ZombieType::ZOMBIE_NORMAL as i32
    }

    pub unsafe fn CanZombieSpawnOnLevel(&self, _theZombieType: i32, _theLevel: i32) -> bool {
        // TODO: check level definitions
        true
    }

    pub fn GetNumWavesPerSurvivalStage(&self) -> i32 {
        10
    }

    // =========================================================================
    // ★ Board::Update() — 主更新循环 (from Board.cpp line 5809)
    // =========================================================================
    pub unsafe fn Update(&mut self) {
        // Widget::Update();
        self.mUpdateCnt += 1;
        self.MarkDirty();

        self.mBoardUpdateCounter += 1;
        if !self.mCutScene.is_null() {
            (*self.mCutScene).Update();
        }
        self.UpdateMousePosition();
        if (*self.mApp).mGameMode as i32 == GameMode::GAMEMODE_CHALLENGE_ZEN_GARDEN as i32 {
            // mApp->mZenGarden->ZenGardenUpdate();
        }
        // if IsScaryPotterDaveTalking() { mApp->UpdateCrazyDave(); }

        if self.mPaused {
            if !self.mChallenge.is_null() {
                (*self.mChallenge).Update();
            }
            if !self.mCursorPreview.is_null() {
                (*self.mCursorPreview).mVisible = false;
            }
            if !self.mCursorObject.is_null() {
                (*self.mCursorObject).mVisible = false;
            }
            return;
        }

        let aDisabled = !self.CanInteractWithBoardButtons() || self.mIgnoreMouseUp;
        if !self.mMenuButton.is_null() && !(*self.mMenuButton).mBtnNoDraw {
            (*self.mMenuButton).mDisabled = aDisabled;
        }
        if !self.mMenuButton.is_null() {
            (*self.mMenuButton).Update();
        }
        if !self.mStoreButton.is_null() {
            (*self.mStoreButton).mDisabled = aDisabled;
            (*self.mStoreButton).Update();
        }

        // mApp->mEffectSystem->Update();
        if !self.mAdvice.is_null() {
            (*self.mAdvice).Update();
        }
        self.UpdateTutorial();

        if self.mCobCannonCursorDelayCounter > 0 {
            self.mCobCannonCursorDelayCounter -= 1;
        }
        if self.mOutOfMoneyCounter > 0 {
            self.mOutOfMoneyCounter -= 1;
        }
        if self.mShakeCounter > 0 {
            self.mShakeCounter -= 1;
            if self.mShakeCounter == 0 {
                self.mX = 0;
                self.mY = 0;
            } else {
                // C++: if (!Rand(3)) mShakeAmountX = -mShakeAmountX;
                // Use counter as simple pseudo-random source
                if self.mShakeCounter % 3 == 0 {
                    self.mShakeAmountX = -self.mShakeAmountX;
                }
                // TodAnimateCurve for shake effect (stub)
                self.mX = self.mShakeAmountX;
                self.mY = self.mShakeAmountY;
            }
        }
        if self.mCoinBankFadeCount > 0 {
            // if mApp->GetDialog(DIALOG_PURCHASE_PACKET_SLOT) == nullptr
            self.mCoinBankFadeCount -= 1;
        }
        self.UpdateLayers();

        if self.mTimeStopCounter > 0 {
            return;
        }

        self.mEffectCounter += 1;
        if self.StageHasPool() && self.mIceTrapCounter == 0
            && (*self.mApp).mGameScene as i32 != GameScenes::SCENE_ZOMBIES_WON as i32
            && (self.mCutScene.is_null() || !(*self.mCutScene).IsSurvivalRepick())
        {
            // mApp->mPoolEffect->mPoolCounter++;
        }
        if self.mBackground as i32 == BackgroundType::BACKGROUND_3_POOL as i32
            && self.mPoolSparklyParticleID == u32::MAX
        {
            // int aRenderPosition = MakeRenderOrder(RENDER_LAYER_GROUND, 2, 0);
            // mApp->AddTodParticle(...);
        }

        self.UpdateGridItems();
        self.UpdateFwoosh();
        self.UpdateGame();
        self.UpdateFog();
        if !self.mChallenge.is_null() {
            (*self.mChallenge).Update();
        }
        // UpdateLevelEndSequence is called from UpdateGame's flow
        // mPrevMouseX/Y updated elsewhere
        self.mPrevMouseX = 0; // mApp->mWidgetManager->mLastMouseX
        self.mPrevMouseY = 0;
    }

    pub unsafe fn MarkDirty(&mut self) {
        // Corresponds to Widget::MarkDirty
        self.mDirty = true;
    }

    pub unsafe fn UpdateLayers(&mut self) {
        // WidgetManager::MarkAllDirty equivalent
        // for each dialog in mApp->mDialogList: BringToFront + MarkDirty
    }

    pub unsafe fn UpdateMousePosition(&mut self) {
        // stub: would update from WidgetManager
    }

    pub unsafe fn CanInteractWithBoardButtons(&self) -> bool {
        let aScene = (*self.mApp).mGameScene;
        aScene as i32 == GameScenes::SCENE_PLAYING as i32
            || aScene as i32 == GameScenes::SCENE_ZOMBIES_WON as i32
    }

    pub unsafe fn StageHasPool(&self) -> bool {
        self.mBackground as i32 == BackgroundType::BACKGROUND_3_POOL as i32
            || self.mBackground as i32 == BackgroundType::BACKGROUND_4_FOG as i32
    }

    pub unsafe fn RowCanHaveZombies(&self, theRow: i32) -> bool {
        if theRow < 0 || theRow >= MAX_GRID_SIZE_Y {
            return false;
        }
        ((*self.mApp).mGameMode as i32 == GameMode::GAMEMODE_CHALLENGE_RESODDED as i32 && theRow <= 4)
            || self.mPlantRow[theRow as usize] != 1  // PLANTROW_DIRT = 0, PLANTROW_NORMAL = 1
    }

    pub fn GetIceZPos(&self, theRow: i32) -> i32 {
        Self::MakeRenderOrder(RenderLayer::RENDER_LAYER_GROUND, theRow, 2)
    }

    // =========================================================================
    // ★ Board::UpdateGame() (from Board.cpp line 5747)
    // =========================================================================
    pub unsafe fn UpdateGame(&mut self) {
        self.UpdateGameObjects();
        if self.StageHasFog() && self.mFogBlownCountDown > 0 {
            let aMaxFogOffset = 1065.0 - self.LeftFogColumn() as f32 * 80.0;
            if (*self.mApp).mGameScene as i32 == GameScenes::SCENE_LEVEL_INTRO as i32 {
                // Fog animation during intro
                self.mFogOffset = aMaxFogOffset;
            } else if self.mFogBlownCountDown < 2000 {
                self.mFogOffset = aMaxFogOffset;
            } else if self.mFogOffset < aMaxFogOffset {
                self.mFogOffset = aMaxFogOffset;
            }
        }

        if (*self.mApp).mGameScene as i32 != GameScenes::SCENE_PLAYING as i32
            && (self.mCutScene.is_null() || !(*self.mCutScene).ShouldRunUpsellBoard())
        {
            return;
        }

        self.mMainCounter += 1;
        self.UpdateSunSpawning();
        self.UpdateZombieSpawning();
        self.UpdateIce();
        if self.mIceTrapCounter > 0 {
            self.mIceTrapCounter -= 1;
            if self.mIceTrapCounter == 0 && self.mPoolSparklyParticleID != u32::MAX {
                // TodParticleSystem* p = mApp->ParticleTryToGet(mPoolSparklyParticleID);
                // if (p) p->mDontUpdate = false;
            }
        }

        if self.mFogBlownCountDown > 0 {
            self.mFogBlownCountDown -= 1;
        }

        if self.mMainCounter == 1 && (*self.mApp).IsFirstTimeAdventureMode() {
            if self.mLevel == 1 {
                // SetTutorialState(TUTORIAL_LEVEL_1_PICK_UP_PEASHOOTER);
            } else if self.mLevel == 2 {
                // SetTutorialState(TUTORIAL_LEVEL_2_PICK_UP_SUNFLOWER);
                // DisplayAdvice("[ADVICE_PLANT_SUNFLOWER1]", ...);
                self.mTutorialTimer = 500;
            }
        }

        self.UpdateProgressMeter();
    }

    // =========================================================================
    // ★ 子函数 stubs (需后续实现完整逻辑)
    // =========================================================================

    pub unsafe fn UpdateGameObjects(&mut self) {
        // TODO: Iterate all zombies, plants, projectiles and update them
        // Corresponds to Board.cpp ~line 5780 area
    }

    pub unsafe fn UpdateSunSpawning(&mut self) {
        // TODO: Natural sun drops, sunflower sun production
    }

    pub unsafe fn UpdateZombieSpawning(&mut self) {
        // TODO: Spawn zombies based on wave definitions
    }

    pub unsafe fn UpdateIce(&mut self) {
        // TODO: Ice melting logic
        for aRow in 0..MAX_GRID_SIZE_Y as usize {
            if self.mIceTimer[aRow] > 0 {
                self.mIceTimer[aRow] -= 1;
                if self.mIceTimer[aRow] == 0 {
                    // Remove ice particles
                }
            }
        }
    }

    pub unsafe fn UpdateProgressMeter(&mut self) {
        // TODO: Update progress meter based on zombie health
    }

    pub unsafe fn UpdateTutorial(&mut self) {
        // TODO: Tutorial state machine
    }

    pub unsafe fn UpdateFog(&mut self) {
        // TODO: Fog scrolling logic
    }

    pub unsafe fn UpdateFwoosh(&mut self) {
        // TODO: Fwoosh (lawn mower trail) update
        if self.mFwooshCountDown > 0 {
            self.mFwooshCountDown -= 1;
        }
    }

    pub unsafe fn UpdateGridItems(&mut self) {
        // TODO: Update grid items (graves, mushrooms, etc.)
    }

    // =========================================================================
    // ★ Board helpers (predicate methods)
    // =========================================================================

    pub unsafe fn IsScaryPotterDaveTalking(&self) -> bool {
        (*self.mApp).IsScaryPotterLevel() && self.mNextSurvivalStageCounter > 0
            // && (*self.mApp).mCrazyDaveState != CrazyDaveState::CRAZY_DAVE_OFF
    }

    pub unsafe fn IsSurvivalStageWithRepick(&self) -> bool {
        (*self.mApp).is_survival_mode() && !self.IsFinalSurvivalStage()
    }

    pub unsafe fn IsFinalSurvivalStage(&self) -> bool {
        let mode = (*self.mApp).mGameMode as i32;
        mode == GameMode::GAMEMODE_SURVIVAL_NORMAL_STAGE_5 as i32
            || mode == GameMode::GAMEMODE_SURVIVAL_HARD_STAGE_5 as i32
            || mode == GameMode::GAMEMODE_SURVIVAL_ENDLESS_STAGE_5 as i32
    }

    pub unsafe fn IsFinalScaryPotterStage(&self) -> bool {
        if !(*self.mApp).IsScaryPotterLevel() { return false; }
        if (*self.mApp).is_adventure_mode() {
            return !self.mChallenge.is_null() && (*self.mChallenge).mSurvivalStage == 2;
        }
        // !IsEndlessScaryPotter
        (*self.mApp).mGameMode as i32 != GameMode::GAMEMODE_SCARY_POTTER_ENDLESS as i32
    }

    pub unsafe fn IsLastStandFinalStage(&self) -> bool {
        // TODO: check last stand stage
        false
    }

    pub unsafe fn CanDropLoot(&self) -> bool {
        (!self.mCutScene.is_null() && !(*self.mCutScene).ShouldRunUpsellBoard())
            && (!(*self.mApp).IsFirstTimeAdventureMode() || self.mLevel >= 11)
    }

    // =========================================================================
    // ★ Board::MouseDown() — 鼠标按下 (from Board.cpp line 4481)
    // =========================================================================
    pub unsafe fn MouseDown(&mut self, x: i32, y: i32, theClickCount: i32) {
        self.UpdateMousePosition();
        // Widget::MouseDown(x, y, theClickCount);
        self.mIgnoreMouseUp = !self.CanInteractWithBoardButtons();
        if self.mTimeStopCounter > 0 { return; }

        // HitResult aHitResult;
        // MouseHitTest(x, y, &aHitResult);
        // if mChallenge->MouseDown(x, y, theClickCount, &aHitResult) { return; }

        if !self.mMenuButton.is_null() && self.CanInteractWithBoardButtons() && theClickCount > 0 {
            // Play sample SOUND_GRAVEBUTTON
        }

        if (*self.mApp).mGameScene as i32 == GameScenes::SCENE_ZOMBIES_WON as i32 {
            if !self.mCutScene.is_null() {
                // mCutScene->ZombieWonClick();
            }
            return;
        }
        if (*self.mApp).mGameScene as i32 == GameScenes::SCENE_LEVEL_INTRO as i32 {
            if !self.mCutScene.is_null() {
                // mCutScene->MouseDown(x, y);
            }
        }

        // Cheat key handling
        if (*self.mApp).m_tod_cheat_keys && !(*self.mApp).IsScaryPotterLevel() && self.mNextSurvivalStageCounter > 0 {
            self.mNextSurvivalStageCounter = 2;
            for i in 0..MAX_GRID_SIZE_Y as usize {
                if self.mIceTimer[i] > 2 { self.mIceTimer[i] = 2; }
            }
        }

        // Cursor-based dispatch (stub)
        // TODO: Full mouse dispatch: CursorObject, SeedPacket, ZenGarden tools etc.
        // UpdateCursor();
    }

    pub unsafe fn MouseUp(&mut self, x: i32, y: i32, theClickCount: i32) {
        // Widget::MouseUp(x, y, theClickCount);
        if self.mIgnoreMouseUp {
            self.mIgnoreMouseUp = false;
            return;
        }

        // if mChallenge->MouseUp(x, y) && theClickCount > 0 { return; }

        if self.CanInteractWithBoardButtons() && theClickCount > 0 {
            // Menu button handling
            if !self.mMenuButton.is_null() {
                // if mMenuButton->IsMouseOver() && !GetDialog(DIALOG_GAME_OVER) ...
                // mMenuButton->mIsOver = false;
                // mMenuButton->mIsDown = false;
                // UpdateCursor();
                // ClearCursor();
            }
            // Store button handling
            if !self.mStoreButton.is_null() {
                // if mStoreButton->IsMouseOver() ...
            }
        }
        // UpdateCursor();
    }

    pub unsafe fn MouseMove(&mut self, _x: i32, _y: i32) {
        // Widget::MouseMove(x, y);
        // UpdateCursor();
    }

    pub unsafe fn MouseDrag(&mut self, _x: i32, _y: i32) {
        // Widget::MouseDrag(x, y);
    }

    // =========================================================================
    // ★ Board::Draw() — 主渲染 (from Board.cpp line 7616)
    // =========================================================================
    /// C++ Board::Draw (Board.cpp:7616)
    pub unsafe fn Draw(&mut self, g: &mut crate::sexy_app_framework::graphics::graphics::Graphics) {
        // [TODO]: if mApp->GetDialog(STORE/ALMANAC) return
        // [TODO]: g.SetLinearBlend(true)
        self.mDrawCount += 1;
        self.DrawGameObjects(g);
    }

    /// C++ Board::DrawGameObjects (Board.cpp:6191)
    /// 构建渲染列表并按 Z 顺序绘制所有游戏对象（简化版，无完整 RenderItem 排序）
    pub unsafe fn DrawGameObjects(&mut self, g: &mut crate::sexy_app_framework::graphics::graphics::Graphics) {
        // 1. 背景
        self.DrawBackdrop(g);

        // 2. 植物
        let mut aPlant: *mut Plant = std::ptr::null_mut();
        while self.IteratePlants(&mut aPlant) {
            if !(*aPlant).base.m_visible || (*aPlant).m_dead { continue; }
            (*aPlant).Draw(g);
        }

        // 3. 僵尸
        let mut aZombie: *mut crate::lawn::zombie::Zombie = std::ptr::null_mut();
        while self.IterateZombies(&mut aZombie) {
            if (*aZombie).IsDeadOrDying() && !(*aZombie).m_has_head { continue; }
            (*aZombie).Draw(g);
        }

        // 4. 弹丸
        let mut aProj: *mut crate::lawn::projectile::Projectile = std::ptr::null_mut();
        while self.IterateProjectiles(&mut aProj) {
            if (*aProj).m_dead { continue; }
            // [TODO]: (*aProj).Draw(g)
        }

        // 5. UI上层
        self.DrawTopRightUI(g);
        // [TODO]: 硬币, 割草机, 粒子, 网格物品, Fog, UI上层/下层
    }

    // =========================================================================
    // ★ Board::KeyDown() / KeyChar() — 键盘输入 (from Board.cpp)
    // =========================================================================
    pub unsafe fn KeyDown(&mut self, _key: i32) {
        // Widget::KeyDown(key);
        // DoTypingCheck(key);
    }

    pub unsafe fn KeyChar(&mut self, _c: char) {
        // Widget::KeyChar(c);
    }

    // =========================================================================
    // ★ 游戏对象管理方法
    // =========================================================================

    /// Board::AddPlant (from Board.cpp:2148)
    pub unsafe fn AddPlant(&mut self, theGridX: i32, theGridY: i32, theSeedType: SeedType, theImitaterType: SeedType) -> *mut Plant {
        // NewPlant creates a new plant and adds it to mPlants
        let aPlant = self.NewPlant(theGridX, theGridY, theSeedType as i32, theImitaterType as i32);
        // DoPlantingEffects(theGridX, theGridY, aPlant);
        if !self.mChallenge.is_null() {
            (*self.mChallenge).PlantAdded(aPlant);
        }

        // Track sun-producer count
        let aSunPlantsCount = self.CountPlantByType(SeedType::SEED_SUNSHROOM)
            + self.CountPlantByType(SeedType::SEED_SUNFLOWER);
        if aSunPlantsCount > self.mMaxSunPlants {
            self.mMaxSunPlants = aSunPlantsCount;
        }

        // Track used plant types for challenge/trophy conditions
        match theSeedType {
            SeedType::SEED_PEASHOOTER | SeedType::SEED_SNOWPEA | SeedType::SEED_REPEATER
            | SeedType::SEED_THREEPEATER | SeedType::SEED_SPLITPEA | SeedType::SEED_GATLINGPEA => {
                self.mPeaShooterUsed = true;
            }
            SeedType::SEED_CABBAGEPULT | SeedType::SEED_KERNELPULT
            | SeedType::SEED_MELONPULT | SeedType::SEED_WINTERMELON => {
                self.mCatapultPlantsUsed = true;
            }
            _ => {}
        }

        let aIsFungi = Plant::is_fungus(theSeedType);
        if !Plant::is_flying(theSeedType) && !aIsFungi {
            self.mMushroomAndCoffeeBeansOnly = false;
        }
        if aIsFungi {
            self.mMushroomsUsed = true;
        }

        aPlant
    }

    /// Board::NewPlant — 创建新植物实例 (from Board.cpp ~line 2100)
    pub unsafe fn NewPlant(&mut self, theGridX: i32, theGridY: i32, theSeedType: i32, theImitaterType: i32) -> *mut Plant {
        let aPlant = self.mPlants.data_array_alloc();
        if aPlant.is_null() { return std::ptr::null_mut(); }

        (*aPlant).m_seed_type = std::mem::transmute(theSeedType);
        (*aPlant).m_plant_col = theGridX;
        (*aPlant).base.m_row = theGridY;
        (*aPlant).m_plant_health = 300; // default health
        (*aPlant).m_plant_max_health = 300;
        (*aPlant).m_frame = 0;
        (*aPlant).m_anim_counter = 0;
        (*aPlant).m_frame_length = 12;
        (*aPlant).m_num_frames = 1;
        (*aPlant).m_state = PlantState::STATE_NOTREADY;
        (*aPlant).m_on_bungee_state = PlantOnBungeeState::NOT_ON_BUNGEE;
        (*aPlant).m_is_asleep = Plant::is_fungus((*aPlant).m_seed_type) && self.StageIsNight();
        (*aPlant).m_imitater_type = theImitaterType;
        (*aPlant).m_potted_plant_index = -1;
        (*aPlant).base.m_x = self.GridToPixelX(theGridX, theGridY);
        (*aPlant).base.m_y = self.GridToPixelY(theGridX, theGridY);
        (*aPlant).base.m_visible = true;
        (*aPlant).base.m_board = self as *mut Board as *mut std::ffi::c_void;
        // Set plant rect
        (*aPlant).m_plant_rect = Rect::new(10, 0, 60, 80);
        (*aPlant).m_plant_attack_rect = Rect::new(0, 0, 0, 0);

        aPlant
    }

    /// Board::AddProjectile (from Board.cpp:2403)
    pub unsafe fn AddProjectile(&mut self, theX: i32, theY: i32, theRenderOrder: i32, theRow: i32, theProjectileType: ProjectileType) -> *mut Projectile {
        let aProjectile = self.mProjectiles.data_array_alloc();
        if aProjectile.is_null() { return std::ptr::null_mut(); }
        (*aProjectile).ProjectileInitialize(theX, theY, theRenderOrder, theRow, theProjectileType);
        aProjectile
    }

    /// Board::AddCoin (from Board.cpp:2031)
    pub unsafe fn AddCoin(&mut self, theX: i32, theY: i32, theCoinType: CoinType, theCoinMotion: CoinMotion) -> *mut Coin {
        let aCoin = self.mCoins.data_array_alloc();
        if aCoin.is_null() { return std::ptr::null_mut(); }
        (*aCoin).CoinInitialize(theX, theY, theCoinType, theCoinMotion);
        if (*self.mApp).IsFirstTimeAdventureMode() && self.mLevel == 1 {
            // DisplayAdvice("[ADVICE_CLICK_ON_SUN]", ...);
        }
        aCoin
    }

    /// Board::AddZombie (from Board.cpp:2729)
    pub unsafe fn AddZombie(&mut self, theZombieType: ZombieType, theFromWave: i32) -> *mut Zombie {
        let aZombie = self.mZombies.data_array_alloc();
        if aZombie.is_null() { return std::ptr::null_mut(); }
        // ZombieInitialize simplified
        let aRow = if (*self.mApp).mGameMode as i32 == GameMode::GAMEMODE_CHALLENGE_RESODDED as i32 {
            crate::sexy_tod_lib::tod_common::rand_range_int(0, 4)
        } else {
            crate::sexy_tod_lib::tod_common::rand_range_int(0, MAX_GRID_SIZE_Y - 1)
        };
        self.AddZombieInRow(theZombieType, aRow, theFromWave)
    }

    /// Board::AddZombieInRow (from Board.cpp:2702)
    pub unsafe fn AddZombieInRow(&mut self, theZombieType: ZombieType, theRow: i32, theFromWave: i32) -> *mut Zombie {
        let aZombie = self.mZombies.data_array_alloc();
        if aZombie.is_null() { return std::ptr::null_mut(); }
        if theRow < 0 || theRow >= MAX_GRID_SIZE_Y || !self.RowCanHaveZombies(theRow) {
            return std::ptr::null_mut();
        }

        (*aZombie).m_zombie_type = theZombieType;
        (*aZombie).base.m_row = theRow;
        (*aZombie).m_from_wave = theFromWave;
        (*aZombie).m_pos_x = 780.0 + crate::sexy_tod_lib::tod_common::rand_range_int(0, ZOMBIE_START_RANDOM_OFFSET - 1) as f32;
        (*aZombie).m_pos_y = 0.0; // GetPosYBasedOnRow(theRow)
        (*aZombie).m_body_health = 270; // default
        (*aZombie).m_body_max_health = 270;
        (*aZombie).m_zombie_phase = ZombiePhase::PHASE_ZOMBIE_NORMAL;
        (*aZombie).m_anim_ticks_per_frame = 12;
        (*aZombie).m_anim_frames = 12;
        (*aZombie).m_dead = false;
        (*aZombie).m_has_head = true;
        (*aZombie).m_has_arm = true;
        (*aZombie).m_dropped_loot = false;
        (*aZombie).m_related_zombie_id = ZombieID::ZOMBIEID_NULL;
        (*aZombie).base.m_render_order = crate::lawn::zombie::RENDER_GROUP_SHIELD;
        (*aZombie).base.m_board = self as *mut Board as *mut std::ffi::c_void;
        (*aZombie).m_zombie_rect = Rect::new(36, 0, 42, 115);
        (*aZombie).m_zombie_attack_rect = Rect::new(50, 0, 20, 115);

        aZombie
    }

    /// Board::CanPlantAt (from Board.cpp:2779)
    pub unsafe fn CanPlantAt(&self, theGridX: i32, theGridY: i32, theSeedType: SeedType) -> PlantingReason {
        if theGridX < 0 || theGridX >= MAX_GRID_SIZE_X || theGridY < 0 || theGridY >= MAX_GRID_SIZE_Y {
            return PlantingReason::PLANTING_NOT_HERE;
        }

        // Challenge-specific checks
        if !self.mChallenge.is_null() {
            let aReason = (*self.mChallenge).CanPlantAt(theGridX, theGridY, theSeedType);
            if aReason != PlantingReason::PLANTING_OK {
                return aReason;
            }
        }

        // Basic grid checks
        let gridSquare = self.mGridSquareType[theGridX as usize][theGridY as usize];
        if gridSquare == GridSquareType::GRIDSQUARE_DIRT || gridSquare == GridSquareType::GRIDSQUARE_NONE {
            return PlantingReason::PLANTING_NOT_HERE;
        }

        // TODO: Full CanPlantAt logic with grave/highground/flowerpot/lilypad checks
        PlantingReason::PLANTING_OK
    }

    // === Board helpers for plant management ===

    pub unsafe fn CountPlantByType(&self, theSeedType: SeedType) -> i32 {
        let mut count = 0;
        let mut aPlant: *mut Plant = std::ptr::null_mut();
        while self.IteratePlants(&mut aPlant) {
            if (*aPlant).m_seed_type == theSeedType && !(*aPlant).m_dead {
                count += 1;
            }
        }
        count
    }

    pub unsafe fn IsPlantInCursor(&self) -> bool {
        if self.mCursorObject.is_null() { return false; }
        let ct = (*self.mCursorObject).mType;
        ct == CursorType::CURSOR_TYPE_PLANT_FROM_BANK
            || ct == CursorType::CURSOR_TYPE_PLANT_FROM_USABLE_COIN
            || ct == CursorType::CURSOR_TYPE_PLANT_FROM_GLOVE
            || ct == CursorType::CURSOR_TYPE_PLANT_FROM_DUPLICATOR
            || ct == CursorType::CURSOR_TYPE_PLANT_FROM_WHEEL_BARROW
    }

    pub unsafe fn GetPumpkinAt(&self, theGridX: i32, theGridY: i32) -> *mut Plant {
        let mut aPlant: *mut Plant = std::ptr::null_mut();
        while self.IteratePlants(&mut aPlant) {
            if (*aPlant).m_plant_col == theGridX && (*aPlant).base.m_row == theGridY
                && !(*aPlant).NotOnGround() && (*aPlant).m_seed_type == SeedType::SEED_PUMPKINSHELL
            {
                return aPlant;
            }
        }
        std::ptr::null_mut()
    }

    pub unsafe fn GetFlowerPotAt(&self, theGridX: i32, theGridY: i32) -> *mut Plant {
        let mut aPlant: *mut Plant = std::ptr::null_mut();
        while self.IteratePlants(&mut aPlant) {
            if (*aPlant).m_plant_col == theGridX && (*aPlant).base.m_row == theGridY
                && !(*aPlant).NotOnGround() && (*aPlant).m_seed_type == SeedType::SEED_FLOWERPOT
            {
                return aPlant;
            }
        }
        std::ptr::null_mut()
    }

    // =========================================================================
    // ★ 绘制系统常量
    // =========================================================================

    /// 渲染对象类型 (对应 C++ RenderObjectType 枚举)
    pub const RENDER_ITEM_NONE: i32 = 0;
    pub const RENDER_ITEM_PLANT: i32 = 1;
    pub const RENDER_ITEM_ZOMBIE: i32 = 2;
    pub const RENDER_ITEM_ZOMBIE_SHADOW: i32 = 3;
    pub const RENDER_ITEM_COIN: i32 = 4;
    pub const RENDER_ITEM_PROJECTILE: i32 = 5;
    pub const RENDER_ITEM_PROJECTILE_SHADOW: i32 = 6;
    pub const RENDER_ITEM_MOWER: i32 = 7;
    pub const RENDER_ITEM_PARTICLE: i32 = 8;
    pub const RENDER_ITEM_REANIMATION: i32 = 9;
    pub const RENDER_ITEM_GRID_ITEM: i32 = 10;
    pub const RENDER_ITEM_GRID_ITEM_OVERLAY: i32 = 11;
    pub const RENDER_ITEM_ZOMBIE_BUNGEE_TARGET: i32 = 12;
    pub const RENDER_ITEM_PLANT_OVERLAY: i32 = 13;
    pub const RENDER_ITEM_PLANT_MAGNET_ITEMS: i32 = 14;

    pub const MAX_RENDER_ITEMS: i32 = 2048;

    /// C++ Board::DrawBackdrop (Board.cpp:5967)
    pub unsafe fn DrawBackdrop(&self, g: &mut crate::sexy_app_framework::graphics::graphics::Graphics) {
        // [TODO]: Draw level background based on mBackground type
    }

    /// C++ Board::DrawTopRightUI (Board.cpp:7286)
    pub unsafe fn DrawTopRightUI(&self, _g: &mut crate::sexy_app_framework::graphics::graphics::Graphics) {
        // [TODO]: Draw menu button, store button, progress meter
    }
}

// Re-export constants
pub use crate::lawn::board_consts::*;

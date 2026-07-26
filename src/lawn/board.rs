// [TRANSLATION_NOTE]: Board.cpp -> Rust 模块
// 使用裸指针 + unsafe 模拟 C++ 跨结构体引用，保持 1:1 逻辑

use std::ptr;
use std::cmp;
use crate::const_enums::*;
use crate::lawn_app::LawnApp;
use crate::lawn::plant::Plant;
use crate::lawn::zombie::Zombie;
use crate::lawn::projectile::Projectile;
use crate::lawn::coin::Coin;
use crate::lawn::lawn_mower::LawnMower;
use crate::lawn::grid_item::GridItem;
use crate::lawn::cursor_object::{CursorObject, CursorPreview, MessageWidget, SeedBank, GameButton, ToolTipWidget};
use crate::lawn::cut_scene::CutScene;
use crate::lawn::challenge::Challenge;
use crate::sexy_app_framework::misc::mtrand::MTRand;
use crate::sexy_app_framework::common;
use crate::sexy_tod_lib::data_array::DataArray;
use crate::sexy_tod_lib::tod_common::TodSmoothArray;
use crate::lawn::board_consts::*;

pub static mut gShownMoreSunTutorial: bool = false;

pub fn BoardInitForPlayer() {
    unsafe { gShownMoreSunTutorial = false; }
}

fn get_saved_game_name(the_game_mode: i32, the_player_id: i32) -> String {
    format!("save_{:04}_{:04}.dat", the_game_mode, the_player_id)
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

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
use crate::lawn::projectile::Projectile;
use crate::lawn::coin::Coin;
use crate::lawn::lawn_mower::LawnMower;
use crate::lawn::grid_item::GridItem;
use crate::lawn::cursor_object::{CursorObject, CursorPreview, GameButton};
use crate::lawn::tool_tip_widget::ToolTipWidget;
use crate::lawn::message_widget::MessageWidget;
use crate::lawn::seed_packet::SeedBank;
use crate::lawn::cut_scene::CutScene;
use crate::lawn::challenge::Challenge;
use crate::sexy_app_framework::misc::mtrand::MTRand;
use crate::sexy_tod_lib::data_array::DataArray;
use crate::sexy_tod_lib::tod_common::{TodSmoothArray, TodWeightedGridArray, clamp_int};
use crate::sexy_app_framework::misc::rect::Rect;
use crate::lawn::board_consts::*;

pub static mut G_SHOWN_MORE_SUN_TUTORIAL: bool = false;

pub fn BoardInitForPlayer() {
    unsafe { G_SHOWN_MORE_SUN_TUTORIAL = false; }
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
    pub mGridCelOffset: [[[i32; 2]; 6]; 9],
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
        let mut board = Board {
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
            mGridCelOffset: [[[0i32; 2]; 6]; 9],
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
        };

    // C++ 构造函数中的网格初始化循环 (Board.cpp:92-106)
    // C++: for (int i = 0; i < MAX_GRID_SIZE_X; i++)
    // C++: {
    // C++:     for (int j = 0; j < MAX_GRID_SIZE_Y; j++)
    // C++:     {
    // C++:         mGridSquareType[i][j] = GridSquareType::GRIDSQUARE_GRASS;
    // C++:         mGridCelLook[i][j] = Rand(20);
    // C++:         mGridCelOffset[i][j][0] = Rand(10) - 5;
    // C++:         mGridCelOffset[i][j][1] = Rand(10) - 5;
    // C++:     }
    // C++:     for (int k = 0; k < MAX_GRID_SIZE_Y + 1; k++)
    // C++:     {
    // C++:         mGridCelFog[i][k] = 0;
    // C++:     }
    // C++: }
    for i in 0..MAX_GRID_SIZE_X as usize {
        for j in 0..MAX_GRID_SIZE_Y as usize {
            board.mGridSquareType[i][j] = GridSquareType::GRIDSQUARE_GRASS;
            // C++: mGridCelLook[i][j] = Rand(20);
            board.mGridCelLook[i][j] = crate::sexy_app_framework::common::rand_range(20);
            // C++: mGridCelOffset[i][j][0] = Rand(10) - 5;
            board.mGridCelOffset[i][j][0] = crate::sexy_app_framework::common::rand_range(10) - 5;
            // C++: mGridCelOffset[i][j][1] = Rand(10) - 5;
            board.mGridCelOffset[i][j][1] = crate::sexy_app_framework::common::rand_range(10) - 5;
        }
        for k in 0..MAX_GRID_SIZE_Y as usize + 1 {
            // C++: mGridCelFog[i][k] = 0;
            board.mGridCelFog[i][k] = 0;
        }
    }
    board
}
}

// ===== Free functions (translated 1:1 from Board.cpp) =====

pub const NUM_ZOMBIE_TYPES: i32 = 34;

/// C++ Board.h:120 struct BungeeDropGrid
#[derive(Clone, Copy)]
pub struct BungeeDropGrid {
    pub mGridArray: [TodWeightedGridArray; MAX_GRID_SIZE_X as usize * MAX_GRID_SIZE_Y as usize],
    pub mGridArrayCount: i32,
}

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
pub static G_ZOMBIE_WAVES: [i32; 50] = [
    3,3,3,3,3,3,3,3,3,4, // 1-10
    4,4,4,4,4,4,4,4,4,5, // 11-20
    5,5,5,5,5,5,5,5,5,5, // 21-30
    5,5,5,5,5,5,5,5,5,5, // 31-40
    5,5,5,5,5,5,5,5,5,5, // 41-50
];

// ===== Board impl (methods from Board.cpp) =====

impl Board {
    // =========================================================================
    // ★ 网格物品查询方法 (C++ Board.cpp:401)
    // =========================================================================

    /// C++ Board::GetGridItemAt — 查找指定坐标和类型的网格物品
    pub unsafe fn GetGridItemAt(&self, the_grid_item_type: GridItemType, the_grid_x: i32, the_grid_y: i32) -> *mut GridItem {
        let mut a_grid_item: *mut GridItem = std::ptr::null_mut();
        while self.IterateGridItems(&mut a_grid_item) {
            if (*a_grid_item).mGridX == the_grid_x
                && (*a_grid_item).mGridY == the_grid_y
                && (*a_grid_item).mGridItemType == the_grid_item_type
            {
                return a_grid_item;
            }
        }
        std::ptr::null_mut()
    }

    /// C++ Board::GetRake — 查找耙子
    pub unsafe fn GetRake(&self) -> *mut GridItem {
        let mut a_grid_item: *mut GridItem = std::ptr::null_mut();
        while self.IterateGridItems(&mut a_grid_item) {
            if (*a_grid_item).mGridItemType == GridItemType::GRIDITEM_RAKE {
                return a_grid_item;
            }
        }
        std::ptr::null_mut()
    }

    /// C++ Board::GetCraterAt
    pub unsafe fn GetCraterAt(&self, the_grid_x: i32, the_grid_y: i32) -> *mut GridItem {
        self.GetGridItemAt(GridItemType::GRIDITEM_CRATER, the_grid_x, the_grid_y)
    }

    /// C++ Board::GetGraveStoneAt
    pub unsafe fn GetGraveStoneAt(&self, the_grid_x: i32, the_grid_y: i32) -> *mut GridItem {
        self.GetGridItemAt(GridItemType::GRIDITEM_GRAVESTONE, the_grid_x, the_grid_y)
    }

    /// C++ Board::GetLadderAt
    pub unsafe fn GetLadderAt(&self, the_grid_x: i32, the_grid_y: i32) -> *mut GridItem {
        self.GetGridItemAt(GridItemType::GRIDITEM_LADDER, the_grid_x, the_grid_y)
    }

    /// C++ Board::GetScaryPotAt
    pub unsafe fn GetScaryPotAt(&self, the_grid_x: i32, the_grid_y: i32) -> *mut GridItem {
        self.GetGridItemAt(GridItemType::GRIDITEM_SCARY_POT, the_grid_x, the_grid_y)
    }

    // =========================================================================
    // ★ 像素坐标 ↔ 网格坐标转换 (C++ Board.cpp:8966)
    // =========================================================================

    /// C++ Board::PixelToGridX
    pub unsafe fn PixelToGridX(&self, the_x: i32, the_y: i32) -> i32 {
        // [TRANSLATION_NOTE]: 禅境花园模式需要委托给 ZenGarden
        // if (mApp->mGameMode == GAMEMODE_CHALLENGE_ZEN_GARDEN) ...
        // [TODO]: ZenGarden 像素坐标转换
        if the_x < LAWN_XMIN {
            return -1;
        }
        crate::sexy_tod_lib::tod_common::clamp_int((the_x - LAWN_XMIN) / 80, 0, MAX_GRID_SIZE_X - 1)
    }

    /// C++ Board::PixelToGridXKeepOnBoard
    pub unsafe fn PixelToGridXKeepOnBoard(&self, the_x: i32, the_y: i32) -> i32 {
        let a_grid_x = self.PixelToGridX(the_x, the_y);
        if a_grid_x < 0 { 0 } else { a_grid_x }
    }

    /// C++ Board::PixelToGridY
    pub unsafe fn PixelToGridY(&self, the_x: i32, the_y: i32) -> i32 {
        // [TODO]: ZenGarden 模式
        let a_grid_x = self.PixelToGridX(the_x, the_y);
        if a_grid_x == -1 || the_y < LAWN_YMIN {
            return -1;
        }
        if self.StageHasRoof() {
            let mut adjusted_y = the_y;
            if a_grid_x < 5 {
                adjusted_y -= (4 - a_grid_x) * 20;
            }
            crate::sexy_tod_lib::tod_common::clamp_int((adjusted_y - LAWN_YMIN) / 85, 0, MAX_GRID_SIZE_Y - 2)
        } else if self.StageHasPool() {
            crate::sexy_tod_lib::tod_common::clamp_int((the_y - LAWN_YMIN) / 85, 0, MAX_GRID_SIZE_Y - 1)
        } else {
            crate::sexy_tod_lib::tod_common::clamp_int((the_y - LAWN_YMIN) / 100, 0, MAX_GRID_SIZE_Y - 1)
        }
    }

    /// C++ Board::PixelToGridYKeepOnBoard
    pub unsafe fn PixelToGridYKeepOnBoard(&self, the_x: i32, the_y: i32) -> i32 {
        let a_grid_y = self.PixelToGridY(the_x, the_y);
        if a_grid_y < 0 { 0 } else { a_grid_y }
    }

    // =========================================================================
    // ★ 阳光经济系统 (C++ Board.cpp:8609)
    // =========================================================================

    /// C++ Board::AddSunMoney
    pub unsafe fn AddSunMoney(&mut self, the_amount: i32) {
        self.mSunMoney += the_amount;
        if self.mSunMoney > 9990 {
            self.mSunMoney = 9990;
        }
    }

    /// C++ Board::CanTakeSunMoney (Board.cpp:8659)
    pub unsafe fn CanTakeSunMoney(&self, the_amount: i32) -> bool {
        self.mSunMoney >= the_amount
    }

    /// C++ Board::TakeSunMoney
    pub unsafe fn TakeSunMoney(&mut self, the_amount: i32) -> bool {
        if self.CanTakeSunMoney(the_amount) {
            self.mSunMoney -= the_amount;
            return true;
        }
        // C++: mApp->PlaySample(SOUND_BUZZER);
        // [TODO]: 音效播放
        // C++: mOutOfMoneyCounter = 70;
        // [TODO]: 设置资金不足计数器
        false
    }

    /// C++ Board::CountSunBeingCollected (Board.cpp:8618)
    pub unsafe fn CountSunBeingCollected(&self) -> i32 {
        let mut a_count = 0;
        let mut a_coin: *mut crate::lawn::coin::Coin = std::ptr::null_mut();
        while self.IterateCoins(&mut a_coin) {
            if (*a_coin).m_is_being_collected && (*a_coin).IsSun() {
                a_count += crate::lawn::coin::Coin::GetCoinValue((*a_coin).m_type);
            }
        }
        a_count
    }

    /// C++ Board::CountCoinsBeingCollected (Board.cpp:8632)
    pub unsafe fn CountCoinsBeingCollected(&self) -> i32 {
        let mut a_count = 0;
        let mut a_coin: *mut crate::lawn::coin::Coin = std::ptr::null_mut();
        while self.IterateCoins(&mut a_coin) {
            if (*a_coin).m_is_being_collected && (*a_coin).IsMoney() {
                a_count += crate::lawn::coin::Coin::GetCoinValue((*a_coin).m_type);
            }
        }
        a_count
    }

    // =========================================================================
    // ★ 原 Board 方法
    // =========================================================================

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
        } else if aGameMode == GameMode::GAMEMODE_PUZZLE_I_ZOMBIE_1 as i32 {
            (*self.mSeedBank).mSeedPackets[0].SetPacketType(SeedType::SEED_ZOMBIE_NORMAL);
            (*self.mSeedBank).mSeedPackets[1].SetPacketType(SeedType::SEED_ZOMBIE_PAIL);
            (*self.mSeedBank).mSeedPackets[2].SetPacketType(SeedType::SEED_ZOMBIE_FOOTBALL);
        } else if aGameMode == GameMode::GAMEMODE_PUZZLE_I_ZOMBIE_2 as i32 {
            (*self.mSeedBank).mSeedPackets[0].SetPacketType(SeedType::SEED_ZOMBIE_NORMAL);
            (*self.mSeedBank).mSeedPackets[1].SetPacketType(SeedType::SEED_ZOMBIE_SCREEN_DOOR);
            (*self.mSeedBank).mSeedPackets[2].SetPacketType(SeedType::SEED_ZOMBIE_PAIL);
        } else if aGameMode == GameMode::GAMEMODE_PUZZLE_I_ZOMBIE_3 as i32 {
            (*self.mSeedBank).mSeedPackets[0].SetPacketType(SeedType::SEED_ZOMBIE_NORMAL);
            (*self.mSeedBank).mSeedPackets[1].SetPacketType(SeedType::SEED_ZOMBIE_PAIL);
            (*self.mSeedBank).mSeedPackets[2].SetPacketType(SeedType::SEED_ZOMBIE_DIGGER);
        } else if aGameMode == GameMode::GAMEMODE_PUZZLE_I_ZOMBIE_4 as i32 {
            (*self.mSeedBank).mSeedPackets[0].SetPacketType(SeedType::SEED_ZOMBIE_NORMAL);
            (*self.mSeedBank).mSeedPackets[1].SetPacketType(SeedType::SEED_ZOMBIE_PAIL);
            (*self.mSeedBank).mSeedPackets[2].SetPacketType(SeedType::SEED_ZOMBIE_LADDER);
        } else if aGameMode == GameMode::GAMEMODE_PUZZLE_I_ZOMBIE_5 as i32 {
            (*self.mSeedBank).mSeedPackets[0].SetPacketType(SeedType::SEED_ZOMBIE_NORMAL);
            (*self.mSeedBank).mSeedPackets[1].SetPacketType(SeedType::SEED_ZOMBIE_PAIL);
            (*self.mSeedBank).mSeedPackets[2].SetPacketType(SeedType::SEED_ZOMBIE_BUNGEE);
            (*self.mSeedBank).mSeedPackets[3].SetPacketType(SeedType::SEED_ZOMBIE_BALLOON);
        } else if aGameMode == GameMode::GAMEMODE_PUZZLE_I_ZOMBIE_6 as i32 {
            (*self.mSeedBank).mSeedPackets[0].SetPacketType(SeedType::SEED_ZOMBIE_NORMAL);
            (*self.mSeedBank).mSeedPackets[1].SetPacketType(SeedType::SEED_ZOMBIE_POLEVAULTER);
            (*self.mSeedBank).mSeedPackets[2].SetPacketType(SeedType::SEED_ZOMBIE_PAIL);
            (*self.mSeedBank).mSeedPackets[3].SetPacketType(SeedType::SEED_ZOMBIE_GARGANTUAR);
        } else if aGameMode == GameMode::GAMEMODE_PUZZLE_I_ZOMBIE_7 as i32 {
            (*self.mSeedBank).mSeedPackets[0].SetPacketType(SeedType::SEED_ZOMBIE_NORMAL);
            (*self.mSeedBank).mSeedPackets[1].SetPacketType(SeedType::SEED_ZOMBIE_POLEVAULTER);
            (*self.mSeedBank).mSeedPackets[2].SetPacketType(SeedType::SEED_ZOMBIE_PAIL);
            (*self.mSeedBank).mSeedPackets[3].SetPacketType(SeedType::SEED_ZOMBIE_DANCER);
        } else if aGameMode == GameMode::GAMEMODE_PUZZLE_I_ZOMBIE_8 as i32 {
            (*self.mSeedBank).mSeedPackets[0].SetPacketType(SeedType::SEED_ZOMBIE_IMP);
            (*self.mSeedBank).mSeedPackets[1].SetPacketType(SeedType::SEED_ZOMBIE_TRAFFIC_CONE);
            (*self.mSeedBank).mSeedPackets[2].SetPacketType(SeedType::SEED_ZOMBIE_PAIL);
            (*self.mSeedBank).mSeedPackets[3].SetPacketType(SeedType::SEED_ZOMBIE_BUNGEE);
            (*self.mSeedBank).mSeedPackets[4].SetPacketType(SeedType::SEED_ZOMBIE_DIGGER);
            (*self.mSeedBank).mSeedPackets[5].SetPacketType(SeedType::SEED_ZOMBIE_LADDER);
        } else if aGameMode == GameMode::GAMEMODE_PUZZLE_I_ZOMBIE_9 as i32 {
            (*self.mSeedBank).mSeedPackets[0].SetPacketType(SeedType::SEED_ZOMBIE_IMP);
            (*self.mSeedBank).mSeedPackets[1].SetPacketType(SeedType::SEED_ZOMBIE_TRAFFIC_CONE);
            (*self.mSeedBank).mSeedPackets[2].SetPacketType(SeedType::SEED_ZOMBIE_POLEVAULTER);
            (*self.mSeedBank).mSeedPackets[3].SetPacketType(SeedType::SEED_ZOMBIE_PAIL);
            (*self.mSeedBank).mSeedPackets[4].SetPacketType(SeedType::SEED_ZOMBIE_BUNGEE);
            (*self.mSeedBank).mSeedPackets[5].SetPacketType(SeedType::SEED_ZOMBIE_DIGGER);
            (*self.mSeedBank).mSeedPackets[6].SetPacketType(SeedType::SEED_ZOMBIE_LADDER);
            (*self.mSeedBank).mSeedPackets[7].SetPacketType(SeedType::SEED_ZOMBIE_FOOTBALL);
        } else if aGameMode == GameMode::GAMEMODE_PUZZLE_I_ZOMBIE_ENDLESS as i32 {
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
        mode >= GameMode::GAMEMODE_PUZZLE_I_ZOMBIE_1 as i32
            && mode <= GameMode::GAMEMODE_PUZZLE_I_ZOMBIE_ENDLESS as i32
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
                self.mNumWaves = G_ZOMBIE_WAVES[clamp_int(self.mLevel - 1, 0, 49) as usize];
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
        let aZombiePicker = std::cell::UnsafeCell::new(aZombiePicker);
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
        let _aLevelRNG = MTRand::with_seed(self.GetLevelRandSeed() as u32);
        // TODO: StageHasGraveStones check and AddGraveStones calls
    }

    pub unsafe fn InitZombieWavesForLevel(&mut self, theForLevel: i32) {
        if self.IsWhackAZombieLevel() || (self.IsWallnutBowlingLevel() && !self.IsFirstTimeAdventureMode()) {
            if let Some(_c) = self.mChallenge.as_mut() {
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

    pub unsafe fn StageHasRoof(&self) -> bool {
        self.mBackground as i32 == BackgroundType::BACKGROUND_5_ROOF as i32
            || self.mBackground as i32 == BackgroundType::BACKGROUND_6_BOSS as i32
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
    // ★ Board 核心游戏逻辑 (from Board.cpp)
    // =========================================================================

    /// C++ Board::DisplayAdvice (Board.cpp:1993)
    pub unsafe fn DisplayAdvice(&mut self, theAdvice: &str, theMessageStyle: i32, theHelpIndex: i32) {
        if theHelpIndex != -1 /* ADVICE_NONE */ {
            if self.mHelpDisplayed[theHelpIndex as usize] {
                return;
            }
            self.mHelpDisplayed[theHelpIndex as usize] = true;
        }
        if !self.mAdvice.is_null() {
            (*self.mAdvice).SetLabel(theAdvice, std::mem::transmute(theMessageStyle));
        }
        self.mHelpIndex = theHelpIndex;
    }

    /// C++ Board::DisplayAdviceAgain (Board.cpp:2007)
    pub unsafe fn DisplayAdviceAgain(&mut self, theAdvice: &str, theMessageStyle: i32, theHelpIndex: i32) {
        if theHelpIndex != -1 {
            self.mHelpDisplayed[theHelpIndex as usize] = false;
        }
        self.DisplayAdvice(theAdvice, theMessageStyle, theHelpIndex);
    }

    /// C++ Board::ClearAdviceImmediately (Board.cpp:2016)
    pub unsafe fn ClearAdviceImmediately(&mut self) {
        self.ClearAdvice(-1);
        if !self.mAdvice.is_null() {
            (*self.mAdvice).mDuration = 0;
        }
    }

    /// C++ Board::ClearAdvice (Board.cpp:2022)
    pub unsafe fn ClearAdvice(&mut self, theHelpIndex: i32) {
        if theHelpIndex == -1 || theHelpIndex == self.mHelpIndex {
            if !self.mAdvice.is_null() {
                (*self.mAdvice).ClearLabel();
            }
            self.mHelpIndex = -1;
        }
    }

    /// C++ Board::UpdateGameObjects (Board.cpp:5067)
    pub unsafe fn UpdateGameObjects(&mut self) {
        let mut aPlant: *mut Plant = std::ptr::null_mut();
        while self.IteratePlants(&mut aPlant) {
            (*aPlant).Update();
        }
        let mut aZombie: *mut Zombie = std::ptr::null_mut();
        while self.IterateZombies(&mut aZombie) {
            (*aZombie).Update();
        }
        let mut aProjectile: *mut Projectile = std::ptr::null_mut();
        while self.IterateProjectiles(&mut aProjectile) {
            (*aProjectile).Update();
        }
        let mut aCoin: *mut Coin = std::ptr::null_mut();
        while self.IterateCoins(&mut aCoin) {
            (*aCoin).Update();
        }
        let mut aMower: *mut LawnMower = std::ptr::null_mut();
        while self.IterateLawnMowers(&mut aMower) {
            (*aMower).Update();
        }
        // [TODO]: mCursorPreview->Update(); mCursorObject->Update();
        // [TODO]: mSeedBank->mSeedPackets[i].Update() for each packet
    }

    /// C++ Board::CanAddBobSled (Board.cpp:2689)
    pub unsafe fn CanAddBobSled(&self) -> bool {
        // C++: for (int aRow = 0; aRow < MAX_GRID_SIZE_Y; aRow++)
        for aRow in 0..MAX_GRID_SIZE_Y as usize {
            // C++: if (mIceTimer[aRow] > 0 && mIceMinX[aRow] < 700) return true;
            if self.mIceTimer[aRow] > 0 && self.mIceMinX[aRow] < 700 {
                return true;
            }
        }
        false
    }

    /// C++ Board::SetupBungeeDrop (Board.cpp:4881)
    pub unsafe fn SetupBungeeDrop(&self, theBungeeDropGrid: &mut BungeeDropGrid) {
        // C++: theBungeeDropGrid->mGridArrayCount = 0;
        theBungeeDropGrid.mGridArrayCount = 0;
        // C++: for (int aGridX = 4; aGridX < MAX_GRID_SIZE_X; aGridX++)
        for aGridX in 4..MAX_GRID_SIZE_X {
            // C++: for (int aGridY = 0; aGridY <= 4; aGridY++)
            for aGridY in 0..=4 {
                // C++: int aCount = theBungeeDropGrid->mGridArrayCount;
                let aCount = theBungeeDropGrid.mGridArrayCount;
                // C++: theBungeeDropGrid->mGridArray[aCount].mX = aGridX;
                theBungeeDropGrid.mGridArray[aCount as usize].m_x = aGridX;
                // C++: theBungeeDropGrid->mGridArray[aCount].mY = aGridY;
                theBungeeDropGrid.mGridArray[aCount as usize].m_y = aGridY;
                // C++: theBungeeDropGrid->mGridArray[aCount].mWeight = 10000;
                theBungeeDropGrid.mGridArray[aCount as usize].m_weight = 10000;
                // C++: theBungeeDropGrid->mGridArrayCount++;
                theBungeeDropGrid.mGridArrayCount += 1;
            }
        }
    }

    /// C++ Board::BungeeDropZombie (Board.cpp:4898)
    pub unsafe fn BungeeDropZombie(&mut self, theBungeeDropGrid: &mut BungeeDropGrid, theZombieType: ZombieType) {
        // C++: TodWeightedGridArray* aGrid = TodPickFromWeightedGridArray(theBungeeDropGrid->mGridArray, theBungeeDropGrid->mGridArrayCount);
        // C++: aGrid->mWeight = 1;
        let aGrid = {
            let aGridSlice = &mut theBungeeDropGrid.mGridArray[..theBungeeDropGrid.mGridArrayCount as usize];
            crate::sexy_tod_lib::tod_common::tod_pick_from_weighted_grid_array(aGridSlice)
        };
        let aGrid = match aGrid {
            Some(aGrid) => aGrid,
            None => return,
        };
        aGrid.m_weight = 1;

        // C++: Zombie* aBungeeZombie = AddZombie(ZombieType::ZOMBIE_BUNGEE, mCurrentWave);
        let aBungeeZombie = self.AddZombie(ZombieType::ZOMBIE_BUNGEE, self.mCurrentWave);
        // C++: Zombie* aZombie = AddZombie(theZombieType, mCurrentWave);
        let aZombie = self.AddZombie(theZombieType, self.mCurrentWave);
        // C++: TOD_ASSERT(aBungeeZombie && aZombie);
        if aBungeeZombie.is_null() || aZombie.is_null() {
            return;
        }

        // C++: aBungeeZombie->BungeeDropZombie(aZombie, aGrid->mX, aGrid->mY);
        (*aBungeeZombie).BungeeDropZombie(aZombie, aGrid.m_x, aGrid.m_y);
    }

    /// C++ Board::SpawnZombieWave (Board.cpp:5009)
    pub unsafe fn SpawnZombieWave(&mut self) {
        // C++: mChallenge->SpawnZombieWave();
        if !self.mChallenge.is_null() {
            (*self.mChallenge).SpawnZombieWave();
        }
        // C++: if (mApp->IsBungeeBlitzLevel())
        if (*self.mApp).IsBungeeBlitzLevel() {
            // C++: BungeeDropGrid aBungeeDropGrid;
            let mut aBungeeDropGrid = BungeeDropGrid {
                mGridArray: [TodWeightedGridArray { m_x: 0, m_y: 0, m_weight: 0 }; MAX_GRID_SIZE_X as usize * MAX_GRID_SIZE_Y as usize],
                mGridArrayCount: 0,
            };
            // C++: SetupBungeeDrop(&aBungeeDropGrid);
            self.SetupBungeeDrop(&mut aBungeeDropGrid);
            // C++: for (int i = 0; i < MAX_ZOMBIES_IN_WAVE; i++)
            for i in 0..MAX_ZOMBIES_IN_WAVE as usize {
                // C++: ZombieType aZombieType = mZombiesInWave[mCurrentWave][i];
                let aZombieType = self.mZombiesInWave[self.mCurrentWave as usize][i];
                // C++: if (aZombieType == ZombieType::ZOMBIE_INVALID) break;
                if aZombieType == ZombieType::ZOMBIE_INVALID as i32 {
                    break;
                }

                // C++: if (aZombieType == ZombieType::ZOMBIE_BUNGEE || aZombieType == ZombieType::ZOMBIE_ZAMBONI)
                if aZombieType == ZombieType::ZOMBIE_BUNGEE as i32 || aZombieType == ZombieType::ZOMBIE_ZAMBONI as i32 {
                    // C++: AddZombie(aZombieType, mCurrentWave);
                    self.AddZombie(std::mem::transmute::<i32, ZombieType>(aZombieType), self.mCurrentWave);
                } else {
                    // C++: BungeeDropZombie(&aBungeeDropGrid, aZombieType);
                    self.BungeeDropZombie(&mut aBungeeDropGrid, std::mem::transmute::<i32, ZombieType>(aZombieType));
                }
            }
        } else {
            // C++: TOD_ASSERT(mCurrentWave >= 0 && mCurrentWave < MAX_ZOMBIE_WAVES && mCurrentWave < mNumWaves);
            // C++: for (int i = 0; i < MAX_ZOMBIES_IN_WAVE; i++)
            for i in 0..MAX_ZOMBIES_IN_WAVE as usize {
                // C++: ZombieType aZombieType = mZombiesInWave[mCurrentWave][i];
                let aZombieType = self.mZombiesInWave[self.mCurrentWave as usize][i];
                // C++: if (aZombieType == ZombieType::ZOMBIE_INVALID) break;
                if aZombieType == ZombieType::ZOMBIE_INVALID as i32 {
                    break;
                }

                // C++: if (aZombieType == ZombieType::ZOMBIE_BOBSLED && !CanAddBobSled())
                if aZombieType == ZombieType::ZOMBIE_BOBSLED as i32 && !self.CanAddBobSled() {
                    // C++: for (int i = 0; i < MAX_ZOMBIE_FOLLOWERS; i++)
                    // C++: { AddZombie(ZombieType::ZOMBIE_NORMAL, mCurrentWave); }
                    for _ in 0..crate::lawn::zombie::MAX_ZOMBIE_FOLLOWERS as usize {
                        self.AddZombie(ZombieType::ZOMBIE_NORMAL, self.mCurrentWave);
                    }
                } else {
                    // C++: AddZombie(aZombieType, mCurrentWave);
                    self.AddZombie(std::mem::transmute::<i32, ZombieType>(aZombieType), self.mCurrentWave);
                }
            }
        }

        // C++: if (mCurrentWave == mNumWaves - 1 && !mApp->IsContinuousChallenge())
        if self.mCurrentWave == self.mNumWaves - 1 && !(*self.mApp).IsContinuousChallenge() {
            // C++: mRiseFromGraveCounter = 210;
            self.mRiseFromGraveCounter = 210;
        }
        // C++: if (IsFlagWave(mCurrentWave))
        if self.IsFlagWave(self.mCurrentWave) {
            // C++: mFlagRaiseCounter = FLAG_RAISE_TIME;
            self.mFlagRaiseCounter = FLAG_RAISE_TIME;
        }
        // C++: mCurrentWave++;
        self.mCurrentWave += 1;
        // C++: mTotalSpawnedWaves++;
        self.mTotalSpawnedWaves += 1;
    }

    /// C++ Board::GetSurvivalFlagsCompleted (Board.cpp:5118)
    pub unsafe fn GetSurvivalFlagsCompleted(&self) -> i32 {
        // C++: int aWavesPerFlag = GetNumWavesPerFlag();
        let aWavesPerFlag = self.GetNumWavesPerFlag();
        // C++: int aFlagsCompleted = mChallenge->mSurvivalStage * GetNumWavesPerSurvivalStage() / aWavesPerFlag;
        let aFlagsCompleted = if self.mChallenge.is_null() {
            0
        } else {
            (*self.mChallenge).mSurvivalStage * self.GetNumWavesPerSurvivalStage() / aWavesPerFlag
        };
        // C++: int aCurrentWave = mCurrentWave;
        let mut aCurrentWave = self.mCurrentWave;
        // C++: if (IsFlagWave(aCurrentWave - 1) && mBoardFadeOutCounter < 0 && !mNextSurvivalStageCounter)
        if self.IsFlagWave(aCurrentWave - 1) && self.mBoardFadeOutCounter < 0 && self.mNextSurvivalStageCounter == 0 {
            // C++: aCurrentWave -= 1;
            aCurrentWave -= 1;
        }
        // C++: return aCurrentWave / aWavesPerFlag + aFlagsCompleted;
        aCurrentWave / aWavesPerFlag + aFlagsCompleted
    }

    /// C++ Board::SurvivalSaveScore (Board.cpp:5130)
    pub unsafe fn SurvivalSaveScore(&mut self) {
        // C++: if (!mApp->IsSurvivalMode()) return;
        if !(*self.mApp).is_survival_mode() {
            return;
        }

        // C++: uint32_t aFlagsCompleted = GetSurvivalFlagsCompleted();
        let aFlagsCompleted = self.GetSurvivalFlagsCompleted() as u32;
        // C++: uint32_t& aFlagsRecord = mApp->mPlayerInfo->mChallengeRecords[mApp->GetCurrentChallengeIndex()];
        let aPlayerInfo = (*self.mApp).mPlayerInfo as *mut crate::lawn::system::player_info::PlayerInfo;
        let aChallengeIndex = (*self.mApp).GetCurrentChallengeIndex();
        // C++: if (aFlagsCompleted > aFlagsRecord)
        if aFlagsCompleted > (*aPlayerInfo).mChallengeRecords[aChallengeIndex as usize] {
            // C++: aFlagsRecord = aFlagsCompleted;
            (*aPlayerInfo).mChallengeRecords[aChallengeIndex as usize] = aFlagsCompleted;
            // C++: mApp->WriteCurrentUserConfig();
            (*self.mApp).WriteCurrentUserConfig();
        }
    }

    /// C++ Board::StopAllZombieSounds (Board.cpp:5108)
    pub unsafe fn StopAllZombieSounds(&mut self) {
        // C++: Zombie* aZombie = nullptr; while (IterateZombies(aZombie)) { aZombie->StopZombieSound(); }
        let mut aZombie: *mut Zombie = std::ptr::null_mut();
        while self.IterateZombies(&mut aZombie) {
            (*aZombie).StopZombieSound();
        }
    }

    /// C++ Board::ZombiesWon (Board.cpp:5158)
    pub unsafe fn ZombiesWon(&mut self, theZombie: *mut Zombie) {
        // C++: if (mApp->mGameScene == GameScenes::SCENE_ZOMBIES_WON) return;
        if (*self.mApp).mGameScene == GameScenes::SCENE_ZOMBIES_WON {
            return;
        }

        // C++: ClearAdvice(AdviceType::ADVICE_NONE);
        self.ClearAdvice(AdviceType::ADVICE_NONE as i32);
        // C++: mApp->mBoardResult = BoardResult::BOARDRESULT_LOST;
        (*self.mApp).mBoardResult = BoardResult::BOARDRESULT_LOST;

        // C++: Zombie* aZombie = nullptr;
        // C++: while (IterateZombies(aZombie))
        let mut aZombie: *mut Zombie = std::ptr::null_mut();
        while self.IterateZombies(&mut aZombie) {
            // C++: if (aZombie == theZombie) continue;
            if aZombie == theZombie {
                continue;
            }

            // C++: if (aZombie->GetZombieRect().mX < -50 ||
            // C++:     aZombie->mZombiePhase == ZombiePhase::PHASE_RISING_FROM_GRAVE ||
            // C++:     aZombie->mZombiePhase == ZombiePhase::PHASE_DANCER_RISING)
            if (*aZombie).GetZombieRect().m_x < -50
                || (*aZombie).m_zombie_phase == ZombiePhase::PHASE_RISING_FROM_GRAVE
                || (*aZombie).m_zombie_phase == ZombiePhase::PHASE_DANCER_RISING
            {
                // C++: if ((aZombie->mZombieType == ZombieType::ZOMBIE_GARGANTUAR ||
                // C++:      aZombie->mZombieType == ZombieType::ZOMBIE_REDEYE_GARGANTUAR) &&
                // C++:     aZombie->IsDeadOrDying() && aZombie->mPosX < 140)
                if ((*aZombie).m_zombie_type == ZombieType::ZOMBIE_GARGANTUAR
                    || (*aZombie).m_zombie_type == ZombieType::ZOMBIE_REDEYE_GARGANTUAR)
                    && (*aZombie).IsDeadOrDying()
                    && (*aZombie).m_pos_x < 140.0
                {
                    // C++: aZombie->DieNoLoot();
                    (*aZombie).DieNoLoot();
                }
            }
        }
        // C++: SurvivalSaveScore();
        self.SurvivalSaveScore();

        // C++: std::string aGameOverMsg;
        let aGameOverMsg: String;
        // C++: if (mApp->mGameMode == GameMode::GAMEMODE_CHALLENGE_ZOMBIQUARIUM)
        if (*self.mApp).mGameMode == GameMode::GAMEMODE_CHALLENGE_ZOMBIQUARIUM {
            // C++: aGameOverMsg = "[ZOMBIQUARIUM_DEATH_MESSAGE]";
            aGameOverMsg = "[ZOMBIQUARIUM_DEATH_MESSAGE]".to_string();
        }
        // C++: else if (mApp->mGameMode == GameMode::GAMEMODE_CHALLENGE_LAST_STAND)
        else if (*self.mApp).mGameMode == GameMode::GAMEMODE_CHALLENGE_LAST_STAND {
            // C++: std::string aFlagStr = mApp->Pluralize(GetSurvivalFlagsCompleted(), "[ONE_FLAG]", "[COUNT_FLAGS]");
            let aFlagStr = LawnApp::Pluralize(self.GetSurvivalFlagsCompleted(), "[ONE_FLAG]", "[COUNT_FLAGS]");
            // C++: aGameOverMsg = TodReplaceString("[LAST_STAND_DEATH_MESSAGE]", "{FLAGS}", aFlagStr);
            aGameOverMsg = crate::sexy_tod_lib::tod_common::tod_replace_string("[LAST_STAND_DEATH_MESSAGE]", "{FLAGS}", &aFlagStr);
        }
        // C++: else if (mApp->IsEndlessIZombie(mApp->mGameMode) || mApp->IsEndlessScaryPotter(mApp->mGameMode))
        else if (*self.mApp).IsEndlessIZombie((*self.mApp).mGameMode)
            || (*self.mApp).IsEndlessScaryPotter((*self.mApp).mGameMode)
        {
            // C++: aGameOverMsg = TodReplaceNumberString("[ENDLESS_PUZZLE_DEATH_MESSAGE]", "{STREAK}", mChallenge->mSurvivalStage);
            aGameOverMsg = crate::sexy_tod_lib::tod_common::tod_replace_number_string(
                "[ENDLESS_PUZZLE_DEATH_MESSAGE]",
                "{STREAK}",
                if self.mChallenge.is_null() { 0 } else { (*self.mChallenge).mSurvivalStage },
            );
        }
        // C++: else if (mApp->IsIZombieLevel())
        else if (*self.mApp).IsIZombieLevel() {
            // C++: aGameOverMsg = "[I_ZOMBIE_DEATH_MESSAGE]";
            aGameOverMsg = "[I_ZOMBIE_DEATH_MESSAGE]".to_string();
        }
        else {
            // C++: mApp->mGameScene = GameScenes::SCENE_ZOMBIES_WON;
            (*self.mApp).mGameScene = GameScenes::SCENE_ZOMBIES_WON;
            // C++: if (theZombie) { theZombie->WalkIntoHouse(); }
            if !theZombie.is_null() {
                // [TODO]: theZombie->WalkIntoHouse() — 僵尸走进房子动画尚未实现
            }

            // C++: ClearAdvice(AdviceType::ADVICE_NONE);
            self.ClearAdvice(AdviceType::ADVICE_NONE as i32);
            // C++: mCutScene->StartZombiesWon();
            if !self.mCutScene.is_null() {
                (*self.mCutScene).StartZombiesWon();
            }
            // C++: FreezeEffectsForCutscene(true);
            // [TODO]: FreezeEffectsForCutscene 尚未实现
            // C++: TutorialArrowRemove();
            // [TODO]: TutorialArrowRemove 尚未实现
            // C++: UpdateCursor();
            // [TODO]: UpdateCursor 尚未实现
            return;
        }

        // C++: GameOverDialog* aGameOverDialog = new GameOverDialog(aGameOverMsg, true);
        // C++: mApp->AddDialog(Dialogs::DIALOG_GAME_OVER, aGameOverDialog);
        // C++: mApp->mWidgetManager->SetFocus(aGameOverDialog);
        // [TODO]: GameOverDialog / AddDialog / WidgetManager 尚未实现

        // C++: mApp->mMusic->StopAllMusic();
        // [TODO]: mMusic->StopAllMusic() 尚未实现
        // C++: StopAllZombieSounds();
        self.StopAllZombieSounds();
        // C++: mApp->PlaySample(Sexy::SOUND_LOSEMUSIC);
        // [TODO]: mApp->PlaySample(SOUND_LOSEMUSIC) 尚未实现

        // C++: ReanimatorEnsureDefinitionLoaded(ReanimationType::REANIM_ZOMBIES_WON, true);
        // C++: Reanimation* aReanim = mApp->AddReanimation(-BOARD_OFFSET, 0,
        // C++:     MakeRenderOrder(RenderLayer::RENDER_LAYER_SCREEN_FADE, 0, 0),
        // C++:     ReanimationType::REANIM_ZOMBIES_WON);
        // C++: aReanim->mLoopType = ReanimLoopType::REANIM_PLAY_ONCE_AND_HOLD;
        // C++: aReanim->GetTrackInstanceByName("fullscreen")->mTrackColor = Color::Black;
        // C++: aReanim->SetFramesForLayer("anim_screen");
        // [TODO]: 游戏失败动画 Reanimation 尚未实现
    }

    /// C++ Board::NextWaveComing (Board.cpp:5322)
    pub unsafe fn NextWaveComing(&mut self) {
        if self.mCurrentWave + 1 == self.mNumWaves {
            // [TODO]: AddReanimation for final wave banner
            // [TODO]: mFinalWaveSoundCounter = 60
        }
        if self.mCurrentWave == 0 {
            // [TODO]: mApp->PlaySample(SOUND_AWOOGA)
        } else if self.IsFlagWave(self.mCurrentWave) {
            // [TODO]: mApp->PlaySample(SOUND_SIREN)
        }
    }

    /// C++ Board::UpdateSunSpawning (Board.cpp:5285)
    pub unsafe fn UpdateSunSpawning(&mut self) {
        let app = self.mApp;

        // C++: 提前返回条件（Board.cpp:5287-5302）
        if self.StageIsNight()
            || self.mLevelAwardSpawned // C++: HasLevelAwardDropped()
            || (*app).mGameMode as i32 == GameMode::GAMEMODE_CHALLENGE_RAINING_SEEDS as i32
            || (*app).mGameMode as i32 == GameMode::GAMEMODE_CHALLENGE_ICE as i32
            || (*app).mGameMode as i32 == GameMode::GAMEMODE_UPSELL as i32
            || (*app).mGameMode as i32 == GameMode::GAMEMODE_INTRO as i32
            || (*app).mGameMode as i32 == GameMode::GAMEMODE_CHALLENGE_ZOMBIQUARIUM as i32
            || (*app).mGameMode as i32 == GameMode::GAMEMODE_CHALLENGE_ZEN_GARDEN as i32
            || (*app).mGameMode as i32 == GameMode::GAMEMODE_TREE_OF_WISDOM as i32
            || (*app).mGameMode as i32 == GameMode::GAMEMODE_CHALLENGE_LAST_STAND as i32
            || (*app).IsIZombieLevel()
            || (*app).IsScaryPotterLevel()
            || (*app).IsSquirrelLevel()
            || self.HasConveyorBeltSeedBank()
            || self.mTutorialState as i32 == TutorialState::TUTORIAL_SLOT_MACHINE_PULL as i32
        {
            return;
        }

        // C++: 教程模式未种植物时不掉落阳光
        if self.mTutorialState as i32 == TutorialState::TUTORIAL_LEVEL_1_PICK_UP_PEASHOOTER as i32
            || self.mTutorialState as i32 == TutorialState::TUTORIAL_LEVEL_1_PLANT_PEASHOOTER as i32
        {
            if self.mPlants.m_size == 0 {
                return;
            }
        }

        self.mSunCountDown -= 1;
        if self.mSunCountDown != 0 {
            return;
        }

        self.mNumSunsFallen += 1;
        // C++: mSunCountDown = std::min(SUN_COUNTDOWN_MAX, SUN_COUNTDOWN + mNumSunsFallen * 10) + Rand(SUN_COUNTDOWN_RANGE);
        self.mSunCountDown = std::cmp::min(
            crate::lawn::board_consts::SUN_COUNTDOWN_MAX,
            crate::lawn::board_consts::SUN_COUNTDOWN + self.mNumSunsFallen * 10,
        ) + crate::sexy_app_framework::common::rand_int() % crate::lawn::board_consts::SUN_COUNTDOWN_RANGE;
        // C++: CoinType aSunType = mGameMode == GAMEMODE_CHALLENGE_SUNNY_DAY ? COIN_LARGESUN : COIN_SUN;
        let a_sun_type = if (*app).mGameMode as i32 == GameMode::GAMEMODE_CHALLENGE_SUNNY_DAY as i32 {
            CoinType::COIN_LARGESUN
        } else {
            CoinType::COIN_SUN
        };
        // C++: AddCoin(RandRangeInt(100, 649), 60, aSunType, CoinMotion::COIN_MOTION_FROM_SKY);
        self.AddCoin(
            crate::sexy_tod_lib::tod_common::rand_range_int(100, 649),
            60,
            a_sun_type,
            CoinMotion::COIN_MOTION_FROM_SKY,
        );
    }

    /// C++ Board::UpdateZombieSpawning (Board.cpp:5343)
    pub unsafe fn UpdateZombieSpawning(&mut self) {
        let app = self.mApp;
        if (*app).mGameMode as i32 == GameMode::GAMEMODE_UPSELL as i32
            || (*app).mGameMode as i32 == GameMode::GAMEMODE_INTRO as i32
        {
            return;
        }

        // 终波音效
        if self.mFinalWaveSoundCounter > 0 {
            self.mFinalWaveSoundCounter -= 1;
            if self.mFinalWaveSoundCounter == 0 {
                // [TODO]: mApp->PlaySample(SOUND_FINALWAVE)
            }
        }

        // 波次生成逻辑
        if self.mZombieCountDown > 0 {
            self.mZombieCountDown -= 1;
            if self.mZombieCountDown == 0 {
                self.SpawnZombieWave();
                // 设置下一波倒计时
                if self.mCurrentWave == self.mNumWaves {
                    self.mZombieCountDown = 0x7FFFFFFF; // no more waves
                } else {
                    self.mZombieCountDown = self.mZombieCountDownStart;
                    // [TODO]: Adjust countdown based on wave number
                }
            }
        }

        // 墓碑危机：墓碑生成僵尸
        // [TODO]: SpawnZombiesFromGraves logic

        // 水池/天空特殊生成
        // [TODO]: SpawnZombiesFromPool / SpawnZombiesFromSky
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

    /// C++ Board::UpdateProgressMeter (Board.cpp:5527)
    pub unsafe fn UpdateProgressMeter(&mut self) {
        if self.mNumWaves == 0 { return; }
        // Calculate progress based on zombie health remaining vs initial
        let mut aTotalHealth = 0;
        let mut aRemainingHealth = 0;
        let mut aZombie: *mut Zombie = std::ptr::null_mut();
        while self.IterateZombies(&mut aZombie) {
            if (*aZombie).m_from_wave == crate::lawn::zombie::ZOMBIE_WAVE_UI
                || (*aZombie).m_from_wave == crate::lawn::zombie::ZOMBIE_WAVE_DEBUG {
                continue;
            }
            aTotalHealth += (*aZombie).m_body_max_health + (*aZombie).m_helm_max_health + (*aZombie).m_shield_max_health;
            aRemainingHealth += (*aZombie).m_body_health.max(0) + (*aZombie).m_helm_health.max(0) + (*aZombie).m_shield_health.max(0);
        }
        aTotalHealth += self.mZombieHealthWaveStart;
        aRemainingHealth += self.mZombieHealthToNextWave;

        if aTotalHealth > 0 {
            self.mProgressMeterWidth = (aRemainingHealth * 1000 / aTotalHealth) as i32;
        }
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
    pub unsafe fn MouseDown(&mut self, _x: i32, _y: i32, theClickCount: i32) {
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

    pub unsafe fn MouseUp(&mut self, _x: i32, _y: i32, theClickCount: i32) {
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
    /// C++: return AddZombieInRow(theZombieType, PickRowForNewZombie(theZombieType), theFromWave);
    pub unsafe fn AddZombie(&mut self, theZombieType: ZombieType, theFromWave: i32) -> *mut Zombie {
        let a_row = self.PickRowForNewZombie(theZombieType);
        self.AddZombieInRow(theZombieType, a_row, theFromWave)
    }

    /// Board::AddZombieInRow (from Board.cpp:2702)
    pub unsafe fn AddZombieInRow(&mut self, theZombieType: ZombieType, theRow: i32, theFromWave: i32) -> *mut Zombie {
        // C++: if (mZombies.mSize >= mZombies.mMaxSize - 1) { TodTrace("Too many zombies!!"); return nullptr; }
        if self.mZombies.m_size >= self.mZombies.m_max_size - 1 {
            return std::ptr::null_mut();
        }

        // C++: if (theZombieType == ZombieType::ZOMBIE_YETI) {
        // C++:     if (mApp->IsAdventureMode() && mLevel == 40 && theFromWave >= 0)
        // C++:         ReportAchievement::GiveAchievement(mApp, Zombologist, true);
        // [TODO]: ReportAchievement 系统尚未翻译
        if theZombieType == ZombieType::ZOMBIE_YETI {
            if (*self.mApp).is_adventure_mode() && self.mLevel == 40 && theFromWave >= 0 {
                // ReportAchievement::GiveAchievement(...)
            }
        }

        // C++: bool aVariant = !Rand(5);
        let a_variant = crate::sexy_app_framework::common::rand_int() % 5 == 0;
        let a_zombie = self.mZombies.data_array_alloc();
        if a_zombie.is_null() {
            return std::ptr::null_mut();
        }
        // C++: aZombie->ZombieInitialize(theRow, theZombieType, aVariant, nullptr, theFromWave);
        (*a_zombie).ZombieInitialize(theRow, theZombieType, a_variant, std::ptr::null_mut(), theFromWave);
        // C++: if (theZombieType == ZombieType::ZOMBIE_BOBSLED && aZombie->IsOnBoard())
        if theZombieType == ZombieType::ZOMBIE_BOBSLED && (*a_zombie).IsOnBoard() {
            // C++: for (int _i = 0; _i < 3; _i++) { mZombies.DataArrayAlloc()->ZombieInitialize(theRow, ZOMBIE_BOBSLED, false, aZombie, theFromWave); }
            for _i in 0..3 {
                let a_sled_zombie = self.mZombies.data_array_alloc();
                if !a_sled_zombie.is_null() {
                    (*a_sled_zombie).ZombieInitialize(theRow, ZombieType::ZOMBIE_BOBSLED, false, a_zombie, theFromWave);
                }
            }
        }
        a_zombie
    }

    /// Board::AreEnemyZombiesOnScreen (from Board.cpp:286)
    pub unsafe fn AreEnemyZombiesOnScreen(&self) -> bool {
        let mut a_zombie: *mut Zombie = std::ptr::null_mut();
        while self.IterateZombies(&mut a_zombie) {
            if (*a_zombie).m_has_head && !(*a_zombie).IsDeadOrDying() && !(*a_zombie).m_mind_controlled {
                return true;
            }
        }
        false
    }

    /// Board::CountZombiesOnScreen (from Board.cpp:300)
    pub unsafe fn CountZombiesOnScreen(&self) -> i32 {
        let mut a_count = 0;
        let mut a_zombie: *mut Zombie = std::ptr::null_mut();
        while self.IterateZombies(&mut a_zombie) {
            if (*a_zombie).m_has_head
                && !(*a_zombie).IsDeadOrDying()
                && !(*a_zombie).m_mind_controlled
                && (*a_zombie).IsOnBoard()
            {
                a_count += 1;
            }
        }
        a_count
    }

    /// Board::GetLiveGargantuarCount (from Board.cpp:315)
    pub unsafe fn GetLiveGargantuarCount(&self) -> i32 {
        let mut a_count = 0;
        let mut a_zombie: *mut Zombie = std::ptr::null_mut();
        while self.IterateZombies(&mut a_zombie) {
            if (*a_zombie).m_has_head
                && !(*a_zombie).IsDeadOrDying()
                && (*a_zombie).IsOnBoard()
                && ((*a_zombie).m_zombie_type == ZombieType::ZOMBIE_GARGANTUAR
                    || (*a_zombie).m_zombie_type == ZombieType::ZOMBIE_REDEYE_GARGANTUAR)
            {
                a_count += 1;
            }
        }
        a_count
    }

    /// Board::CountUntriggerLawnMowers (from Board.cpp:328)
    pub unsafe fn CountUntriggerLawnMowers(&self) -> i32 {
        let mut a_count = 0;
        let mut a_lawn_mower: *mut LawnMower = std::ptr::null_mut();
        while self.IterateLawnMowers(&mut a_lawn_mower) {
            // C++: mMowerState != MOWER_TRIGGERED && mMowerState != MOWER_SQUISHED
            if (*a_lawn_mower).mMowerState != MowerState::MOWER_TRIGGERED
                && (*a_lawn_mower).mMowerState != MowerState::MOWER_TRIGGERED_SQUASHED
            {
                a_count += 1;
            }
        }
        a_count
    }

    /// Board::CanAddGraveStoneAt (from Board.cpp:459)
    pub unsafe fn CanAddGraveStoneAt(&self, the_grid_x: i32, the_grid_y: i32) -> bool {
        if self.mGridSquareType[the_grid_x as usize][the_grid_y as usize] != GridSquareType::GRIDSQUARE_GRASS
            && self.mGridSquareType[the_grid_x as usize][the_grid_y as usize] != GridSquareType::GRIDSQUARE_HIGH_GROUND
        {
            return false;
        }

        let mut a_grid_item: *mut GridItem = std::ptr::null_mut();
        while self.IterateGridItems(&mut a_grid_item) {
            if (*a_grid_item).mGridX == the_grid_x && (*a_grid_item).mGridY == the_grid_y {
                if (*a_grid_item).mGridItemType == GridItemType::GRIDITEM_GRAVESTONE
                    || (*a_grid_item).mGridItemType == GridItemType::GRIDITEM_CRATER
                    || (*a_grid_item).mGridItemType == GridItemType::GRIDITEM_LADDER
                {
                    return false;
                }
            }
        }
        true
    }

    /// Board::AddALadder (from Board.cpp:485)
    pub unsafe fn AddALadder(&mut self, the_grid_x: i32, the_grid_y: i32) -> *mut GridItem {
        let a_ladder = self.mGridItems.data_array_alloc();
        if a_ladder.is_null() { return std::ptr::null_mut(); }
        (*a_ladder).mGridItemType = GridItemType::GRIDITEM_LADDER;
        (*a_ladder).mRenderOrder = Board::MakeRenderOrder(RenderLayer::RENDER_LAYER_PLANT, the_grid_y, 800);
        (*a_ladder).mGridX = the_grid_x;
        (*a_ladder).mGridY = the_grid_y;
        a_ladder
    }

    /// Board::AddACrater (from Board.cpp:495)
    pub unsafe fn AddACrater(&mut self, the_grid_x: i32, the_grid_y: i32) -> *mut GridItem {
        let a_crater = self.mGridItems.data_array_alloc();
        if a_crater.is_null() { return std::ptr::null_mut(); }
        (*a_crater).mGridItemType = GridItemType::GRIDITEM_CRATER;
        (*a_crater).mRenderOrder = Board::MakeRenderOrder(RenderLayer::RENDER_LAYER_GROUND, the_grid_y, 1);
        (*a_crater).mGridX = the_grid_x;
        (*a_crater).mGridY = the_grid_y;
        a_crater
    }

    /// Board::AddAGraveStone (from Board.cpp:505)
    pub unsafe fn AddAGraveStone(&mut self, the_grid_x: i32, the_grid_y: i32) -> *mut GridItem {
        let a_grave_stone = self.mGridItems.data_array_alloc();
        if a_grave_stone.is_null() { return std::ptr::null_mut(); }
        (*a_grave_stone).mGridItemType = GridItemType::GRIDITEM_GRAVESTONE;
        // C++: aGraveStone->mGridItemCounter = -Rand(50);
        (*a_grave_stone).mGridItemCounter = -(crate::sexy_app_framework::common::rand_int() % 50);
        (*a_grave_stone).mRenderOrder = Board::MakeRenderOrder(RenderLayer::RENDER_LAYER_GRAVE_STONE, the_grid_y, 3);
        (*a_grave_stone).mGridX = the_grid_x;
        (*a_grave_stone).mGridY = the_grid_y;
        a_grave_stone
    }

    /// Board::AddGraveStones (from Board.cpp:516)
    pub unsafe fn AddGraveStones(&mut self, the_grid_x: i32, the_count: i32, the_level_rng: &mut MTRand) {
        // C++: TOD_ASSERT(theCount <= MAX_GRID_SIZE_Y);

        // C++: 统计本列可放置墓碑的格子数，超出部分修正 theCount（避免卡死）
        let mut a_grid_allow_grave_stones_count = 0;
        for y in 0..MAX_GRID_SIZE_Y {
            if self.CanAddGraveStoneAt(the_grid_x, y) {
                a_grid_allow_grave_stones_count += 1;
            }
        }
        let the_count = cmp::min(the_count, a_grid_allow_grave_stones_count);

        let mut i = 0;
        while i < the_count {
            // C++: int aGridY = theLevelRNG.Next((unsigned long)MAX_GRID_SIZE_Y);
            let a_grid_y = (the_level_rng.next() % MAX_GRID_SIZE_Y as u32) as i32;
            if self.CanAddGraveStoneAt(the_grid_x, a_grid_y) {
                self.AddAGraveStone(the_grid_x, a_grid_y);
                i += 1;
            }
        }
    }

    /// Board::GetGraveStonesCount (from Board.cpp:4789)
    pub unsafe fn GetGraveStonesCount(&self) -> i32 {
        let mut a_count = 0;
        let mut a_grid_item: *mut GridItem = std::ptr::null_mut();
        while self.IterateGridItems(&mut a_grid_item) {
            if (*a_grid_item).mGridItemType == GridItemType::GRIDITEM_GRAVESTONE {
                a_count += 1;
            }
        }
        a_count
    }


    /// C++ 全局函数 GetRectOverlap (Board.cpp:9123) — X 轴重叠量
    /// [TRANSLATION_NOTE]: C++ 为全局函数，此处作为 Board 关联函数提供。
    pub fn get_rect_overlap(rect1: crate::sexy_app_framework::misc::rect::Rect, rect2: crate::sexy_app_framework::misc::rect::Rect) -> i32 {
        // C++: return std::min(rect1.mX + rect1.mWidth, rect2.mX + rect2.mWidth) - std::max(rect1.mX, rect2.mX);
        (rect1.m_x + rect1.m_width).min(rect2.m_x + rect2.m_width) - rect1.m_x.max(rect2.m_x)
    }

    /// Board::GetTopPlantAt (from Board.cpp:2286) — 获取格子上指定优先级的顶层植物
    /// [TRANSLATION_NOTE]: PlantsOnLawn 分类（GetPlantsOnLawn）后续翻译；
    /// 此处按 seed type 简化分类（pumpkin/flying/under/normal）实现同等优先级语义。
    pub unsafe fn GetTopPlantAt(&self, the_grid_x: i32, the_grid_y: i32, the_priority: PlantPriority) -> *mut Plant {
        if the_grid_x < 0 || the_grid_x >= MAX_GRID_SIZE_X || the_grid_y < 0 || the_grid_y >= MAX_GRID_SIZE_Y {
            return std::ptr::null_mut();
        }

        // C++: mApp->IsWallnutBowlingLevel() && !mCutScene->IsInShovelTutorial() 时返回 nullptr
        // [TODO]: 保龄球关卡检查

        let mut a_pumpkin: *mut Plant = std::ptr::null_mut();
        let mut a_flying: *mut Plant = std::ptr::null_mut();
        let mut a_normal: *mut Plant = std::ptr::null_mut();
        let mut a_under: *mut Plant = std::ptr::null_mut();

        let mut a_plant: *mut Plant = std::ptr::null_mut();
        while self.IteratePlants(&mut a_plant) {
            if (*a_plant).m_plant_col != the_grid_x || (*a_plant).base.m_row != the_grid_y {
                continue;
            }
            let a_seed = (*a_plant).m_seed_type;
            if a_seed == SeedType::SEED_PUMPKINSHELL {
                if a_pumpkin.is_null() { a_pumpkin = a_plant; }
            } else if Plant::is_flying(a_seed) {
                if a_flying.is_null() { a_flying = a_plant; }
            } else if a_seed == SeedType::SEED_LILYPAD {
                if a_under.is_null() { a_under = a_plant; }
            } else {
                if a_normal.is_null() { a_normal = a_plant; }
            }
        }

        match the_priority {
            PlantPriority::TOPPLANT_EATING_ORDER => {
                if !a_pumpkin.is_null() { return a_pumpkin; }
                else if !a_normal.is_null() { return a_normal; }
                else { return a_under; }
            }
            PlantPriority::TOPPLANT_DIGGING_ORDER => {
                if !a_normal.is_null() { return a_normal; }
                else { return a_under; }
            }
            PlantPriority::TOPPLANT_BUNGEE_ORDER | PlantPriority::TOPPLANT_CATAPULT_ORDER
            | PlantPriority::TOPPLANT_ANY => {
                if !a_flying.is_null() { return a_flying; }
                else if !a_normal.is_null() { return a_normal; }
                else if !a_pumpkin.is_null() { return a_pumpkin; }
                else { return a_under; }
            }
            PlantPriority::TOPPLANT_ZEN_TOOL_ORDER => {
                if !a_flying.is_null() { return a_flying; }
                else if !a_pumpkin.is_null() { return a_pumpkin; }
                else if !a_normal.is_null() { return a_normal; }
                else { return a_under; }
            }
            PlantPriority::TOPPLANT_ONLY_NORMAL_POSITION => return a_normal,
            PlantPriority::TOPPLANT_ONLY_FLYING => return a_flying,
            PlantPriority::TOPPLANT_ONLY_PUMPKIN => return a_pumpkin,
            PlantPriority::TOPPLANT_ONLY_UNDER_PLANT => return a_under,
        }
    }
    /// Board::IsZombieTypePoolOnly (from Board.cpp:2566)
    pub unsafe fn IsZombieTypePoolOnly(&self, the_zombie_type: ZombieType) -> bool {
        // C++: ZOMBIE_SNORKEL || ZOMBIE_DOLPHIN_RIDER
        the_zombie_type == ZombieType::ZOMBIE_SNORKEL || the_zombie_type == ZombieType::ZOMBIE_DOLPHIN_RIDER
    }

    /// Board::RowCanHaveZombieType (from Board.cpp:2571) — 指定行能否刷出该僵尸
    pub unsafe fn RowCanHaveZombieType(&self, the_row: i32, the_zombie_type: ZombieType) -> bool {
        if !self.RowCanHaveZombies(the_row) {
            return false;
        }

        // C++: RESODDED 无草皮行前 5 波不刷怪
        if (*self.mApp).mGameMode as i32 == GameMode::GAMEMODE_CHALLENGE_RESODDED as i32
            && self.mPlantRow[the_row as usize] == PlantRowType::PLANTROW_DIRT as i32
            && self.mCurrentWave < 5
        {
            return false;
        }
        // C++: 水路不刷不能入水的僵尸（气球除外）
        if self.mPlantRow[the_row as usize] == PlantRowType::PLANTROW_POOL as i32
            && !Zombie::ZombieTypeCanGoInPool(the_zombie_type)
            && the_zombie_type != ZombieType::ZOMBIE_BALLOON
        {
            return false;
        }
        // C++: 高地不刷不能上高地的僵尸
        if self.mPlantRow[the_row as usize] == PlantRowType::PLANTROW_HIGH_GROUND as i32
            && !Zombie::ZombieTypeCanGoOnHighGround(the_zombie_type)
        {
            return false;
        }

        // C++: int aCurrentWave = mCurrentWave; LAST_STAND 时加上生存阶段偏移
        let mut a_current_wave = self.mCurrentWave;
        if (*self.mApp).mGameMode as i32 == GameMode::GAMEMODE_CHALLENGE_LAST_STAND as i32 {
            if !self.mChallenge.is_null() {
                // C++: aCurrentWave += mChallenge->mSurvivalStage * GetNumWavesPerSurvivalStage();
                a_current_wave += (*self.mChallenge).mSurvivalStage * self.GetNumWavesPerSurvivalStage();
            }
        }
        // C++: 非水路不能刷水路僵尸；前 5 小波水面仅潜水/海豚
        if self.mPlantRow[the_row as usize] == PlantRowType::PLANTROW_POOL as i32 {
            if a_current_wave < 5 && !self.IsZombieTypePoolOnly(the_zombie_type) {
                return false;
            }
        } else if self.IsZombieTypePoolOnly(the_zombie_type) {
            return false;
        }
        // C++: 雪橇僵尸仅在有冰道的行刷出
        if the_zombie_type == ZombieType::ZOMBIE_BOBSLED && self.mIceTimer[the_row as usize] == 0 {
            return false;
        }
        // C++: 第一行不出伽刚特尔（生存模式除外）
        if the_row == 0 && !self.IsSurvivalMode() {
            if the_zombie_type == ZombieType::ZOMBIE_GARGANTUAR
                || the_zombie_type == ZombieType::ZOMBIE_REDEYE_GARGANTUAR
            {
                return false;
            }
        }
        // C++: 非舞王僵尸或当前为泳池关卡则允许
        if the_zombie_type != ZombieType::ZOMBIE_DANCER || self.StageHasPool() {
            return true;
        }
        // C++: 舞王僵尸（非泳池）仅中间三行（保证伴舞能出现）
        self.RowCanHaveZombies(the_row - 1) && self.RowCanHaveZombies(the_row + 1)
    }

    /// Board::PickRowForNewZombie (from Board.cpp:2630) — 为新僵尸选择行（权重）
    pub unsafe fn PickRowForNewZombie(&mut self, the_zombie_type: ZombieType) -> i32 {
        // C++: 钉耙吸引状态 → 优先钉耙行
        let a_rake = self.GetRake();
        if !a_rake.is_null()
            && (*a_rake).mGridItemState == 26 /* GRIDITEM_STATE_RAKE_ATTRACTING */
            && self.RowCanHaveZombieType((*a_rake).mGridY, the_zombie_type)
        {
            // C++: aRake->mGridItemState = GRIDITEM_STATE_RAKE_WAITING;
            (*a_rake).mGridItemState = 27; /* GRIDITEM_STATE_RAKE_WAITING */
            crate::sexy_tod_lib::tod_common::tod_update_smooth_array_pick(
                &mut self.mRowPickingArray,
                MAX_GRID_SIZE_Y,
                (*a_rake).mGridY,
            );
            return (*a_rake).mGridY;
        }

        // C++: 遍历每一行，按规则设置出怪权重
        let mut a_row = 0;
        while a_row < MAX_GRID_SIZE_Y {
            if !self.RowCanHaveZombieType(a_row, the_zombie_type) {
                self.mRowPickingArray[a_row as usize].m_weight = 0.0;
            } else if (*self.mApp).mGameMode as i32 == GameMode::GAMEMODE_CHALLENGE_PORTAL_COMBAT as i32 {
                // C++: mChallenge->PortalCombatRowSpawnWeight(aRow)
                // [TODO]: 传送门关卡行权重
                self.mRowPickingArray[a_row as usize].m_weight = 1.0;
            } else if (*self.mApp).mGameMode as i32 == GameMode::GAMEMODE_CHALLENGE_INVISIGHOUL as i32
                && self.mCurrentWave <= 3
                && a_row == 5
            {
                // C++: 隐形食脑者前 3 波第六路不出怪
                self.mRowPickingArray[a_row as usize].m_weight = 0.0;
            } else {
                // C++: 丢车保护 — 计算该行被割草机清理后的波数
                let mut a_waves_mowered = self.mCurrentWave - self.mWaveRowGotLawnMowered[a_row as usize];
                if (*self.mApp).IsContinuousChallenge() && self.mCurrentWave == self.mNumWaves - 1 {
                    a_waves_mowered = 100;
                }

                if a_waves_mowered <= 1 {
                    self.mRowPickingArray[a_row as usize].m_weight = 0.01;
                } else if a_waves_mowered <= 2 {
                    self.mRowPickingArray[a_row as usize].m_weight = 0.5;
                } else {
                    self.mRowPickingArray[a_row as usize].m_weight = 1.0;
                }
            }
            a_row += 1;
        }

        // C++: return TodPickFromSmoothArray(mRowPickingArray, MAX_GRID_SIZE_Y);
        crate::sexy_tod_lib::tod_common::tod_pick_from_smooth_array(&mut self.mRowPickingArray, MAX_GRID_SIZE_Y)
    }
    /// Board::PlantingRequirementsMet (from Board.cpp:9566) — 进阶植物前置需求
    pub unsafe fn PlantingRequirementsMet(&self, the_seed_type: SeedType) -> bool {
        match the_seed_type {
            SeedType::SEED_GATLINGPEA => self.CountPlantByType(SeedType::SEED_REPEATER) > 0,
            SeedType::SEED_TWINSUNFLOWER => self.CountPlantByType(SeedType::SEED_SUNFLOWER) > 0,
            SeedType::SEED_GLOOMSHROOM => self.CountPlantByType(SeedType::SEED_FUMESHROOM) > 0,
            SeedType::SEED_CATTAIL => {
                // C++: CountEmptyPotsOrLilies(SEED_LILYPAD) — 空荷叶数量
                // [TODO]: CountEmptyPotsOrLilies 完整翻译
                true
            }
            SeedType::SEED_WINTERMELON => self.CountPlantByType(SeedType::SEED_MELONPULT) > 0,
            SeedType::SEED_GOLD_MAGNET => self.CountPlantByType(SeedType::SEED_MAGNETSHROOM) > 0,
            SeedType::SEED_SPIKEROCK => self.CountPlantByType(SeedType::SEED_SPIKEWEED) > 0,
            SeedType::SEED_COBCANNON => {
                // C++: HasValidCobCannonSpot() — 存在有效玉米炮位置
                // [TODO]: HasValidCobCannonSpot 完整翻译
                true
            }
            _ => true,
        }
    }

    /// Board::PlantUsesAcceleratedPricing (from Board.cpp:9659)
    pub unsafe fn PlantUsesAcceleratedPricing(&self, the_seed_type: SeedType) -> bool {
        // C++: Plant::IsUpgrade(theSeedType) && mApp->IsSurvivalEndless(mApp->mGameMode)
        Plant::is_upgrade(the_seed_type)
            && (*self.mApp).mGameMode as i32 >= GameMode::GAMEMODE_SURVIVAL_ENDLESS_STAGE_1 as i32
            && (*self.mApp).mGameMode as i32 <= GameMode::GAMEMODE_SURVIVAL_ENDLESS_STAGE_5 as i32
    }

    /// Board::GetCurrentPlantCost (from Board.cpp:9664) — 当前植物价格（加速定价）
    pub unsafe fn GetCurrentPlantCost(&self, the_seed_type: SeedType, the_imitater_type: SeedType) -> i32 {
        let mut a_cost = Plant::GetCost(the_seed_type, the_imitater_type);
        if self.PlantUsesAcceleratedPricing(the_seed_type) {
            // C++: aCost += CountPlantByType(theSeedType) * 50;
            a_cost += self.CountPlantByType(the_seed_type) * 50;
        }
        a_cost
    }

    /// C++ 全局函数 GetCircleRectOverlap (Board.cpp:9129) — 圆与矩形相交
    pub fn get_circle_rect_overlap(the_circle_x: i32, the_circle_y: i32, the_radius: i32, the_rect: crate::sexy_app_framework::misc::rect::Rect) -> bool {
        // C++: int aNearX = std::clamp(theCircleX, theRect.mX, theRect.mX + theRect.mWidth);
        let a_near_x = the_circle_x.clamp(the_rect.m_x, the_rect.m_x + the_rect.m_width);
        let a_near_y = the_circle_y.clamp(the_rect.m_y, the_rect.m_y + the_rect.m_height);
        let dx = the_circle_x - a_near_x;
        let dy = the_circle_y - a_near_y;
        dx * dx + dy * dy <= the_radius * the_radius
    }

    /// Board::KillAllZombiesInRadius (from Board.cpp:9583) — 半径内击杀僵尸
    pub unsafe fn KillAllZombiesInRadius(&mut self, the_row: i32, the_x: i32, the_y: i32, the_radius: i32, the_row_range: i32, the_burn: bool, the_damage_range_flags: u32) -> i32 {
        let mut a_killed_zombies = 0;
        let mut a_zombie: *mut Zombie = std::ptr::null_mut();
        while self.IterateZombies(&mut a_zombie) {
            if (*a_zombie).EffectedByDamage(the_damage_range_flags) {
                let a_zombie_rect = (*a_zombie).GetZombieRect();
                let mut a_row_dist = (*a_zombie).base.m_row - the_row;
                if (*a_zombie).m_zombie_type == ZombieType::ZOMBIE_BOSS {
                    a_row_dist = 0;
                }

                if a_row_dist <= the_row_range
                    && a_row_dist >= -the_row_range
                    && Self::get_circle_rect_overlap(the_x, the_y, the_radius, a_zombie_rect)
                {
                    if the_burn {
                        (*a_zombie).ApplyBurn();
                    } else {
                        // C++: theZombie->TakeDamage(1800, 18U);
                        (*a_zombie).TakeDamage(1800, 18);
                    }

                    a_killed_zombies += 1;
                }
            }
        }

        // C++: 摧毁范围内梯子
        let a_grid_x = self.PixelToGridXKeepOnBoard(the_x, the_y);
        let a_grid_y = self.PixelToGridYKeepOnBoard(the_x, the_y);
        let mut a_grid_item: *mut GridItem = std::ptr::null_mut();
        while self.IterateGridItems(&mut a_grid_item) {
            if (*a_grid_item).mGridItemType == GridItemType::GRIDITEM_LADDER {
                if crate::lawn::lawn_common::GridInRange(
                    (*a_grid_item).mGridX, (*a_grid_item).mGridY, a_grid_x, a_grid_y, the_row_range, the_row_range,
                ) {
                    (*a_grid_item).GridItemDie();
                }
            }
        }

        a_killed_zombies
    }
    /// Board::TutorialArrowShow — 显示教程箭头
    pub unsafe fn TutorialArrowShow(&mut self, the_x: f32, the_y: f32) {
        // [TODO]: 箭头 widget 显示
        let _ = the_x;
        let _ = the_y;
    }

    /// Board::TutorialArrowRemove — 移除教程箭头
    pub unsafe fn TutorialArrowRemove(&mut self) {
        // [TODO]: 箭头 widget 移除
    }

    /// Board::ToolHitTest — 工具命中植物
    pub unsafe fn ToolHitTest(&self, the_x: i32, the_y: i32) -> *mut Plant {
        // [TODO]: 完整命中检测（MouseHitTest 机制）
        let _ = the_x;
        let _ = the_y;
        std::ptr::null_mut()
    }

    /// Board::ClearCursor — 清除光标
    pub unsafe fn ClearCursor(&mut self) {
        if !self.mCursorObject.is_null() {
            (*self.mCursorObject).mCursorType = CursorType::CURSOR_TYPE_NORMAL;
            (*self.mCursorObject).mVisible = false;
        }
    }
    /// Board::PlantingPixelToGridX — 种植像素转网格 X
    pub unsafe fn PlantingPixelToGridX(&self, the_x: i32, the_y: i32, _the_seed_type: SeedType) -> i32 {
        self.PixelToGridX(the_x, the_y)
    }

    /// Board::PlantingPixelToGridY — 种植像素转网格 Y
    pub unsafe fn PlantingPixelToGridY(&self, the_x: i32, the_y: i32, _the_seed_type: SeedType) -> i32 {
        self.PixelToGridY(the_x, the_y)
    }

    /// Board::RefreshSeedPacketFromCursor — 光标归还种子包
    pub unsafe fn RefreshSeedPacketFromCursor(&mut self) {
        // [TODO]: 种子包返回动画
    }
    /// Board::SetTutorialState — 设置教程状态
    pub unsafe fn SetTutorialState(&mut self, the_state: i32) {
        self.mTutorialState = the_state;
        // [TODO]: 教程提示 DisplayAdvice（按状态显示对应提示）
    }
    /// Board::CountCoinByType — 统计指定类型金币数量
    pub unsafe fn CountCoinByType(&self, the_coin_type: CoinType) -> i32 {
        let mut a_count = 0;
        let mut a_coin: *mut Coin = std::ptr::null_mut();
        while self.IterateCoins(&mut a_coin) {
            if (*a_coin).m_type == the_coin_type {
                a_count += 1;
            }
        }
        a_count
    }
    /// Board::CountZombieByType — 统计指定类型僵尸数量
    pub unsafe fn CountZombieByType(&self, the_zombie_type: ZombieType) -> i32 {
        let mut a_count = 0;
        let mut a_zombie: *mut Zombie = std::ptr::null_mut();
        while self.IterateZombies(&mut a_zombie) {
            if (*a_zombie).m_zombie_type == the_zombie_type {
                a_count += 1;
            }
        }
        a_count
    }
    /// Board::GetBossZombie (from Board.cpp:9466)
    pub unsafe fn GetBossZombie(&self) -> *mut Zombie {
        let mut a_zombie: *mut Zombie = std::ptr::null_mut();
        while self.IterateZombies(&mut a_zombie) {
            if (*a_zombie).m_zombie_type == ZombieType::ZOMBIE_BOSS {
                return a_zombie;
            }
        }
        std::ptr::null_mut()
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
        let ct = (*self.mCursorObject).mCursorType;
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
    pub unsafe fn DrawBackdrop(&self, _g: &mut crate::sexy_app_framework::graphics::graphics::Graphics) {
        // [TODO]: Draw level background based on mBackground type
    }

    /// C++ Board::DrawUIBottom (Board.cpp:7322) — 底部 UI（波浪/覆盖/种子银行）
    pub unsafe fn DrawUIBottom(&self, g: &mut crate::sexy_app_framework::graphics::graphics::Graphics) {
        // C++: 水族馆波浪动画
        if self.mBackground == BackgroundType::BACKGROUND_ZOMBIQUARIUM as i32 {
            let a_wave_time = ((self.mMainCounter / 8) as i32 % 22 - 11).abs();
            // [TODO]: DRAWMODE_ADDITIVE + IMAGE_WAVESIDE/WAVECENTER cel 绘制
            let _ = a_wave_time;
        }

        // C++: 温室/水族馆覆盖层
        if self.mBackground == BackgroundType::BACKGROUND_GREENHOUSE as i32
            || self.mBackground == BackgroundType::BACKGROUND_ZOMBIQUARIUM as i32
        {
            // [TODO]: DRAWMODE_ADDITIVE + IMAGE_BACKGROUND_GREENHOUSE_OVERLAY
        }

        // C++: 种子银行绘制
        if (*self.mApp).mGameScene != GameScenes::SCENE_ZOMBIES_WON {
            if !self.mSeedBank.is_null() {
                if (*self.mSeedBank).BeginDraw(g) {
                    (*self.mSeedBank).Draw(g);
                    (*self.mSeedBank).EndDraw(g);
                }
            }

            // C++: 老虎机消息提示
            if !self.mAdvice.is_null() {
                (*self.mAdvice).Draw(g);
            }
        }

        self.DrawShovel(g);
        if !self.StageHasFog() {
            self.DrawTopRightUI(g);
        }
    }

    /// C++ Board::DrawShovel (Board.cpp:7645) — 铲子绘制
    pub unsafe fn DrawShovel(&self, _g: &mut crate::sexy_app_framework::graphics::graphics::Graphics) {
        // [TODO]: 铲子光标/拖拽绘制
    }
    /// C++ Board::DrawTopRightUI (Board.cpp:7286)
    pub unsafe fn DrawTopRightUI(&self, _g: &mut crate::sexy_app_framework::graphics::graphics::Graphics) {
        // [TODO]: Draw menu button, store button, progress meter
    }
}

// Re-export constants

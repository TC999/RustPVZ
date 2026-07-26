// [TRANSLATION_NOTE]: Board.cpp -> Rust 模块
// 使用裸指针 + unsafe 模拟 C++ 跨结构体引用，保持 1:1 逻辑

use std::ptr;
use std::cmp;
use crate::game_constants::*;
use crate::const_enums::*;
use crate::lawn_app::LawnApp;
use crate::lawn::plant::Plant;
use crate::lawn::zombie::Zombie;
use crate::lawn::projectile::Projectile;
use crate::lawn::coin::Coin;
use crate::lawn::lawn_mower::LawnMower;
use crate::lawn::grid_item::GridItem;
use crate::lawn::cursor_object::{CursorObject, CursorPreview, MessageWidget, GameButton, ToolTipWidget};
use crate::lawn::seed_packet::{SeedBank, SeedPacket};
use crate::lawn::cut_scene::CutScene;
use crate::lawn::challenge::Challenge;
use crate::sexy_app_framework::misc::mtrand::MTRand;
use crate::sexy_app_framework::common;
use crate::sexy_tod_lib::data_array::DataArray;
use crate::sexy_tod_lib::tod_common::{TodSmoothArray, clamp_int};
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
}

// Re-export constants
pub use crate::lawn::board_consts::*;

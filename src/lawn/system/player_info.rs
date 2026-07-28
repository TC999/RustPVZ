// [TRANSLATION_NOTE]: PlayerInfo.h + PlayerInfo.cpp -> Rust 翻译
// 玩家信息结构体，包含存档数据读写

#![allow(non_snake_case, dead_code)]

use crate::const_enums::*;
use crate::lawn::system::data_sync::DataSync;
use crate::lawn::lawn_common::{GetSavedGameName, GetLegacySavedGameName};
use crate::sexy_app_framework::common::get_app_data_path;

pub const MAX_POTTED_PLANTS: i32 = 200;
pub const PURCHASE_COUNT_OFFSET: i32 = 1000;
pub const ZOMBATAR_RECORD_SIZE: i32 = 0x48;
pub const MAX_ZOMBATAR_HEADS: i32 = 100;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum FacingDirection {
    FACING_RIGHT = 0,
    FACING_LEFT = 1,
}

// PottedPlantAge 枚举 — 从 ConstEnums.h 翻译
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PottedPlantAge {
    PLANTAGE_SPROUT = 0,
    PLANTAGE_SMALL = 1,
    PLANTAGE_MEDIUM = 2,
    PLANTAGE_FULL = 3,
    PLANTAGE_DEAD = 4,
    NUM_PLANT_AGES = 5,
}

// PottedPlantNeed 枚举 — 从 ConstEnums.h 翻译
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PottedPlantNeed {
    PLANTNEED_NONE = -1,
    PLANTNEED_WATER = 0,
    PLANTNEED_FERTILIZER = 1,
    PLANTNEED_BUGSPRAY = 2,
    PLANTNEED_PHONOGRAPH = 3,
    PLANTNEED_CHOCOLATE = 4,
    PLANTNEED_FUNGICIDE = 5,
    NUM_PLANT_NEEDS = 6,
}

#[derive(Clone)]
pub struct PottedPlant {
    pub mSeedType: SeedType,
    pub mWhichZenGarden: GardenType,
    pub mX: i32,
    pub mY: i32,
    pub mFacing: FacingDirection,
    pub mPadding1: u32,
    pub mLastWateredTime: i64,
    pub mDrawVariation: DrawVariation,
    pub mPlantAge: PottedPlantAge,
    pub mTimesFed: i32,
    pub mFeedingsPerGrow: i32,
    pub mPlantNeed: PottedPlantNeed,
    pub mPadding2: u32,
    pub mLastNeedFulfilledTime: i64,
    pub mLastFertilizedTime: i64,
    pub mLastChocolateTime: i64,
    pub mFutureAttribute: [i64; 1],
}

impl PottedPlant {
    pub fn InitializePottedPlant(&mut self, theSeedType: SeedType) {
        *self = unsafe { std::mem::zeroed() };
        self.mSeedType = theSeedType;
        self.mDrawVariation = DrawVariation::VARIATION_NORMAL;
        self.mLastWateredTime = 0;
        self.mFacing = FacingDirection::FACING_RIGHT;
        self.mPlantAge = PottedPlantAge::PLANTAGE_SPROUT;
        self.mTimesFed = 0;
        self.mWhichZenGarden = GardenType::GARDEN_MAIN;
        self.mFeedingsPerGrow = 4;
        self.mPlantNeed = PottedPlantNeed::PLANTNEED_NONE;
        self.mLastNeedFulfilledTime = 0;
        self.mLastFertilizedTime = 0;
        self.mLastChocolateTime = 0;
    }
}

#[derive(Clone)]
pub struct PlayerInfo {
    pub mName: String,
    pub mUseSeq: u32,
    pub mId: u32,
    pub mLevel: u32,
    pub mCoins: u32,
    pub mFinishedAdventure: u32,
    pub mChallengeRecords: [u32; 100],
    pub mPurchases: [u32; 80],
    pub mPlayTimeActivePlayer: u32,
    pub mPlayTimeInactivePlayer: u32,
    pub mHasUsedCheatKeys: u32,
    pub mHasWokenStinky: u32,
    pub mDidntPurchasePacketUpgrade: u32,
    pub mLastStinkyChocolateTime: u32,
    pub mStinkyPosX: u32,
    pub mStinkyPosY: u32,
    pub mHasUnlockedMinigames: u32,
    pub mHasUnlockedPuzzleMode: u32,
    pub mHasNewMiniGame: u32,
    pub mHasNewScaryPotter: u32,
    pub mHasNewIZombie: u32,
    pub mHasNewSurvival: u32,
    pub mHasUnlockedSurvivalMode: u32,
    pub mNeedsMessageOnGameSelector: u32,
    pub mNeedsMagicTacoReward: u32,
    pub mHasSeenStinky: u32,
    pub mHasSeenUpsell: u32,
    pub mPlaceHolderPlayerStats: u32,
    pub mNumPottedPlants: u32,
    pub mPottedPlant: Vec<PottedPlant>,
    pub mEarnedAchievements: [bool; 20],
    pub mShownAchievements: [bool; 20],
    pub mZombatarAccepted: u8,
    pub mZombatarHeadCount: u32,
    pub mZombatarData: Vec<u8>,
    pub mZombatarCreatedBefore: u8,
}

impl PlayerInfo {
    pub fn new() -> Self {
        let mut pi = PlayerInfo {
            mName: String::new(),
            mUseSeq: 0,
            mId: 0,
            mLevel: 0,
            mCoins: 0,
            mFinishedAdventure: 0,
            mChallengeRecords: [0u32; 100],
            mPurchases: [0u32; 80],
            mPlayTimeActivePlayer: 0,
            mPlayTimeInactivePlayer: 0,
            mHasUsedCheatKeys: 0,
            mHasWokenStinky: 0,
            mDidntPurchasePacketUpgrade: 0,
            mLastStinkyChocolateTime: 0,
            mStinkyPosX: 0,
            mStinkyPosY: 0,
            mHasUnlockedMinigames: 0,
            mHasUnlockedPuzzleMode: 0,
            mHasNewMiniGame: 0,
            mHasNewScaryPotter: 0,
            mHasNewIZombie: 0,
            mHasNewSurvival: 0,
            mHasUnlockedSurvivalMode: 0,
            mNeedsMessageOnGameSelector: 0,
            mNeedsMagicTacoReward: 0,
            mHasSeenStinky: 0,
            mHasSeenUpsell: 0,
            mPlaceHolderPlayerStats: 0,
            mNumPottedPlants: 0,
            mPottedPlant: Vec::new(),
            mEarnedAchievements: [false; 20],
            mShownAchievements: [false; 20],
            mZombatarAccepted: 0,
            mZombatarHeadCount: 0,
            mZombatarData: Vec::new(),
            mZombatarCreatedBefore: 0,
        };
        pi.Reset();
        pi
    }

    pub fn Reset(&mut self) {
        self.mLevel = 1;
        self.mCoins = 0;
        self.mFinishedAdventure = 0;
        self.mChallengeRecords = [0u32; 100];
        self.mPurchases = [0u32; 80];
        self.mPlayTimeActivePlayer = 0;
        self.mPlayTimeInactivePlayer = 0;
        self.mHasUsedCheatKeys = 0;
        self.mHasWokenStinky = 0;
        self.mDidntPurchasePacketUpgrade = 0;
        self.mLastStinkyChocolateTime = 0;
        self.mStinkyPosX = 0;
        self.mStinkyPosY = 0;
        self.mHasUnlockedMinigames = 0;
        self.mHasUnlockedPuzzleMode = 0;
        self.mHasNewMiniGame = 0;
        self.mHasNewScaryPotter = 0;
        self.mHasNewIZombie = 0;
        self.mHasNewSurvival = 0;
        self.mHasUnlockedSurvivalMode = 0;
        self.mNeedsMessageOnGameSelector = 0;
        self.mNeedsMagicTacoReward = 0;
        self.mHasSeenStinky = 0;
        self.mHasSeenUpsell = 0;
        self.mPlaceHolderPlayerStats = 0;
        self.mPottedPlant.clear();
        self.mNumPottedPlants = 0;
        self.mEarnedAchievements = [false; 20];
        self.mShownAchievements = [false; 20];
        self.mZombatarAccepted = 0;
        self.mZombatarHeadCount = 0;
        self.mZombatarData.clear();
        self.mZombatarCreatedBefore = 0;
    }

    pub fn SyncSummary(&mut self, theSync: &mut DataSync) {
        theSync.sync_string(&mut self.mName);
        theSync.sync_u32(&mut self.mUseSeq);
        theSync.sync_u32(&mut self.mId);
    }

    pub fn SyncDetails(&mut self, theSync: &mut DataSync) {
        let gUserVersion: u32 = 12;

        if theSync.is_reader() {
            self.Reset();
        }

        let mut aVersion = gUserVersion;
        theSync.sync_u32(&mut aVersion);
        theSync.set_version(aVersion as i32);
        if aVersion != gUserVersion {
            return;
        }

        theSync.sync_u32(&mut self.mLevel);
        theSync.sync_u32(&mut self.mCoins);
        theSync.sync_u32(&mut self.mFinishedAdventure);
        for i in 0..100 {
            theSync.sync_u32(&mut self.mChallengeRecords[i]);
        }
        for i in 0..80 {
            theSync.sync_u32(&mut self.mPurchases[i]);
        }
        theSync.sync_u32(&mut self.mPlayTimeActivePlayer);
        theSync.sync_u32(&mut self.mPlayTimeInactivePlayer);
        theSync.sync_u32(&mut self.mHasUsedCheatKeys);
        theSync.sync_u32(&mut self.mHasWokenStinky);
        theSync.sync_u32(&mut self.mDidntPurchasePacketUpgrade);
        theSync.sync_u32(&mut self.mLastStinkyChocolateTime);
        theSync.sync_u32(&mut self.mStinkyPosX);
        theSync.sync_u32(&mut self.mStinkyPosY);
        theSync.sync_u32(&mut self.mHasUnlockedMinigames);
        theSync.sync_u32(&mut self.mHasUnlockedPuzzleMode);
        theSync.sync_u32(&mut self.mHasNewMiniGame);
        theSync.sync_u32(&mut self.mHasNewScaryPotter);
        theSync.sync_u32(&mut self.mHasNewIZombie);
        theSync.sync_u32(&mut self.mHasNewSurvival);
        theSync.sync_u32(&mut self.mHasUnlockedSurvivalMode);
        theSync.sync_u32(&mut self.mNeedsMessageOnGameSelector);
        theSync.sync_u32(&mut self.mNeedsMagicTacoReward);
        theSync.sync_u32(&mut self.mHasSeenStinky);
        theSync.sync_u32(&mut self.mHasSeenUpsell);
        theSync.sync_u32(&mut self.mPlaceHolderPlayerStats);
        theSync.sync_u32(&mut self.mNumPottedPlants);

        // PottedPlant 数据同步
        let num_pots = self.mNumPottedPlants as usize;
        if num_pots > MAX_POTTED_PLANTS as usize {
            // TOD_ASSERT
        }
        self.mPottedPlant.resize(num_pots, unsafe { std::mem::zeroed() });
        for i in 0..num_pots {
            // 对 PottedPlant 的同步简化：直接读写字节
            let plant_bytes = unsafe {
                std::slice::from_raw_parts_mut(
                    &mut self.mPottedPlant[i] as *mut PottedPlant as *mut u8,
                    std::mem::size_of::<PottedPlant>(),
                )
            };
            theSync.sync_bytes(plant_bytes);
        }

        // Achievements
        for i in 0..20 {
            let mut aAchievementValue: u16 = if self.mEarnedAchievements[i] { 1 } else { 0 };
            theSync.sync_u16(&mut aAchievementValue);
            if theSync.is_reader() {
                self.mEarnedAchievements[i] = aAchievementValue != 0;
                self.mShownAchievements[i] = self.mEarnedAchievements[i];
            }
        }

        // Read path
        if theSync.is_reader() {
            // Zombatar 数据读取
            let mut aZombatarAccepted: u8 = 0;
            theSync.sync_u8(&mut aZombatarAccepted);
            self.mZombatarAccepted = if aZombatarAccepted != 0 { 1 } else { 0 };

            let mut aZombatarHeadCount: u32 = 0;
            theSync.sync_u32(&mut aZombatarHeadCount);
            if aZombatarHeadCount > MAX_ZOMBATAR_HEADS as u32 {
                // 数据错误，使用默认值
                self.mZombatarAccepted = 0;
                self.mZombatarHeadCount = 0;
                self.mZombatarData.clear();
                self.mZombatarCreatedBefore = 0;
                return;
            }

            self.mZombatarHeadCount = aZombatarHeadCount;
            let data_size = self.mZombatarHeadCount as usize * ZOMBATAR_RECORD_SIZE as usize;
            self.mZombatarData.resize(data_size, 0);
            if !self.mZombatarData.is_empty() {
                theSync.sync_bytes(&mut self.mZombatarData);
            }
            // 跳过 MiniGameFlags (20 bytes)
            let mut aMiniGameFlags = [0u8; 20];
            theSync.sync_bytes(&mut aMiniGameFlags);

            let mut aZombatarCreatedBefore: u8 = 0;
            theSync.sync_u8(&mut aZombatarCreatedBefore);
            self.mZombatarCreatedBefore = if aZombatarCreatedBefore != 0 { 1 } else { 0 };
        } else {
            // Write path
            let mut aZombatarAccepted: u8 = if self.mZombatarAccepted != 0 { 1 } else { 0 };
            theSync.sync_u8(&mut aZombatarAccepted);
            self.mZombatarAccepted = aZombatarAccepted;

            self.mZombatarHeadCount = self.mZombatarData.len() as u32 / ZOMBATAR_RECORD_SIZE as u32;
            if self.mZombatarHeadCount > MAX_ZOMBATAR_HEADS as u32 {
                self.mZombatarHeadCount = MAX_ZOMBATAR_HEADS as u32;
                self.mZombatarData.resize(self.mZombatarHeadCount as usize * ZOMBATAR_RECORD_SIZE as usize, 0);
            }
            let aZombatarDataBytes = self.mZombatarHeadCount * ZOMBATAR_RECORD_SIZE as u32;
            theSync.sync_u32(&mut self.mZombatarHeadCount);
            if aZombatarDataBytes > 0 {
                theSync.sync_bytes(&mut self.mZombatarData);
            }
            let mut aMiniGameFlags = [0u8; 20];
            for i in 0..20 {
                aMiniGameFlags[i] = if self.mChallengeRecords[i + 0x0F] > 0 { 1 } else { 0 };
            }
            theSync.sync_bytes(&mut aMiniGameFlags);

            let mut aZombatarCreatedBefore: u8 = if self.mZombatarCreatedBefore != 0 { 1 } else { 0 };
            theSync.sync_u8(&mut aZombatarCreatedBefore);
            self.mZombatarCreatedBefore = aZombatarCreatedBefore;
        }
    }

    pub fn LoadDetails(&mut self) {
        let aFileName = get_app_data_path(&format!("userdata/user{}.dat", self.mId));
        if let Ok(data) = std::fs::read(&aFileName) {
            let mut aReader = crate::lawn::system::data_sync::DataReader::new();
            aReader.open_memory(data, false);
            let mut aSync = DataSync::from_reader(&mut aReader);
            self.SyncDetails(&mut aSync);
        }
    }

    pub fn SaveDetails(&mut self) {
        let mut aWriter = crate::lawn::system::data_sync::DataWriter::new();
        aWriter.open_memory(0x20);
        let mut aSync = DataSync::from_writer(&mut aWriter);
        self.SyncDetails(&mut aSync);

        let _ = std::fs::create_dir_all(get_app_data_path("userdata"));
        let aFileName = get_app_data_path(&format!("userdata/user{}.dat", self.mId));
        let _ = std::fs::write(&aFileName, aWriter.into_data());
    }

    pub fn DeleteUserFiles(&mut self) {
        let aFilename = get_app_data_path(&format!("userdata/user{}.dat", self.mId));
        let _ = std::fs::remove_file(&aFilename);

        // 清理所有游戏存档文件
        for i in 0..(GameMode::NUM_GAME_MODES as i32) {
            let gm: GameMode = unsafe { std::mem::transmute(i) };
            let aFileName = GetSavedGameName(gm, self.mId as i32);
            let _ = std::fs::remove_file(&aFileName);
            let aLegacyFileName = GetLegacySavedGameName(gm, self.mId as i32);
            let _ = std::fs::remove_file(&aLegacyFileName);
        }
    }

    pub fn AddCoins(&mut self, theAmount: i32) {
        if theAmount > 0 || self.mCoins as i32 > -theAmount {
            self.mCoins = (self.mCoins as i32 + theAmount) as u32;
        } else {
            self.mCoins = 0;
        }
        self.mCoins = std::cmp::min(self.mCoins, 99999);
    }

    pub fn GetLevel(&self) -> i32 { self.mLevel as i32 }
    pub fn SetLevel(&mut self, theLevel: i32) { self.mLevel = theLevel as u32; }

    pub fn ResetChallengeRecord(&mut self, theGameMode: GameMode) {
        let aGameMode = theGameMode as i32 - GameMode::GAMEMODE_SURVIVAL_NORMAL_STAGE_1 as i32;
        if aGameMode >= 0 && (aGameMode as usize) < self.mChallengeRecords.len() {
            self.mChallengeRecords[aGameMode as usize] = 0;
        }
    }
}

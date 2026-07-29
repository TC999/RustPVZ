// [TRANSLATION_NOTE]: Challenge.h -> Rust stub
// board.rs 使用 mSurvivalStage 字段，保持可用

#![allow(non_snake_case, dead_code)]

use crate::const_enums::*;

pub const BEGHOULED_MAX_GRIDSIZEX: i32 = 8;
pub const BEGHOULED_MAX_GRIDSIZEY: i32 = 5;
pub const BEGHOULED_WINNING_SCORE: i32 = 75;
pub const SLOT_MACHINE_WINNING_SCORE: i32 = 2000;
pub const ZOMBIQUARIUM_WINNING_SCORE: i32 = 1000;
pub const I_ZOMBIE_WINNING_SCORE: i32 = 5;
pub const MAX_PORTALS: i32 = 4;
pub const MAX_SQUIRRELS: i32 = 7;
pub const MAX_SCARY_POTS: i32 = 54;
pub const STORM_FLASH_TIME: i32 = 150;
pub const MAX_PICK_GRID_SIZE: i32 = 50;

pub struct Challenge {
    pub mApp: *mut crate::lawn_app::LawnApp,
    pub mBoard: *mut crate::lawn::board::Board,
    pub mSurvivalStage: i32,
    pub mChallengeState: ChallengeState,
    pub mChallengeStateCounter: i32,
    pub mConveyorBeltCounter: i32,
    pub mChallengeScore: i32,
    pub mLastConveyorSeedType: SeedType,
    pub mReanimChallenge: ReanimationID,
    pub mChallengeGridX: i32,
    pub mChallengeGridY: i32,
}

impl Challenge {
    pub fn new() -> Self {
        Challenge {
            mApp: std::ptr::null_mut(),
            mBoard: std::ptr::null_mut(),
            mSurvivalStage: 0,
            mChallengeState: ChallengeState::STATECHALLENGE_NORMAL,
            mChallengeStateCounter: 0,
            mConveyorBeltCounter: 0,
            mChallengeScore: 0,
            mLastConveyorSeedType: SeedType::SEED_NONE,
            mReanimChallenge: ReanimationID::REANIMATIONID_NULL,
            mChallengeGridX: -1,
            mChallengeGridY: -1,
        }
    }

    // Helper
    unsafe fn board(&self) -> &'static mut crate::lawn::board::Board {
        &mut *(self.mBoard)
    }
    unsafe fn app(&self) -> &'static mut crate::lawn_app::LawnApp {
        &mut *(self.mApp)
    }

    /// C++ Challenge::InitLevel() (from Challenge.cpp:360)
    pub unsafe fn InitLevel(&mut self) {
        let app = self.app();
        let board = self.board();

        if app.mGameMode as i32 == GameMode::GAMEMODE_CHALLENGE_RAINING_SEEDS as i32 {
            self.mChallengeStateCounter = 100;
            // app->PlayFoley(FOLEY_RAIN);
        }
        if (*app).IsStormyNightLevel() {
            self.mChallengeState = ChallengeState::STATECHALLENGE_STORM_FLASH_2;
            self.mChallengeStateCounter = 100;
            // app->PlayFoley(FOLEY_RAIN);
        }
        if (*app).IsFinalBossLevel() {
            // board->mSeedBank->AddSeed(SEED_CABBAGEPULT);
            // board->mSeedBank->AddSeed(SEED_JALAPENO);
            // board->mSeedBank->AddSeed(SEED_CABBAGEPULT);
            // board->mSeedBank->AddSeed(SEED_ICESHROOM);
            self.mConveyorBeltCounter = 1000;
        }
        if app.mGameMode as i32 == GameMode::GAMEMODE_CHALLENGE_ZEN_GARDEN as i32 {
            // app->mZenGarden->mGardenType = GARDEN_MAIN;
            // app->mZenGarden->ZenGardenInitLevel();
        }
        if app.mGameMode as i32 == GameMode::GAMEMODE_CHALLENGE_COLUMN as i32 {
            // board->mSeedBank->AddSeed(SEED_POTATOMINE);
            // board->mSeedBank->AddSeed(SEED_TALLNUT);
            // board->mSeedBank->AddSeed(SEED_MELONPULT);
            // board->mSeedBank->AddSeed(SEED_MAGNETSHROOM);
            // board->mSeedBank->AddSeed(SEED_INSTANT_COFFEE);
            // board->mSeedBank->AddSeed(SEED_MELONPULT);
            self.mConveyorBeltCounter = 1000;
        }
        if app.mGameMode as i32 == GameMode::GAMEMODE_CHALLENGE_INVISIGHOUL as i32 {
            // board->mSeedBank->AddSeed(SEED_PEASHOOTER);
            // board->mSeedBank->AddSeed(SEED_ICESHROOM);
            self.mConveyorBeltCounter = 1000;
        }
        if (*app).IsIZombieLevel() {
            // IZombieInitLevel();
        }
        if (*app).IsScaryPotterLevel() {
            // ScaryPotterPopulate();
        }
        if (*app).IsFirstTimeAdventureMode() && board.mLevel == 5 {
            // board->NewPlant(5, 1, SEED_PEASHOOTER, SEED_NONE);
            // board->NewPlant(7, 2, SEED_PEASHOOTER, SEED_NONE);
            // board->NewPlant(6, 3, SEED_PEASHOOTER, SEED_NONE);
        }
        if app.mGameMode as i32 == GameMode::GAMEMODE_CHALLENGE_BEGHOULED_TWIST as i32 {
            self.mChallengeGridX = -1;
            self.mChallengeGridY = -1;
        }
        // Tree of wisdom init
    }

    /// C++ Challenge::StartLevel() (from Challenge.cpp:427)
    pub unsafe fn StartLevel(&mut self) {
        let app = self.app();
        let _board = self.board();

        if (*app).IsWhackAZombieLevel() {
            // board->mCursorObject->mCursorType = CURSOR_TYPE_HAMMER;
            // board->mZombieCountDown = 200;
            // Reanimation loading...
        }
        if (*app).is_wallnut_bowling_level() {
            // board->mZombieCountDown = 400;
        }
    }

    /// C++ Challenge::Update() (from Challenge.cpp:2128)
    pub unsafe fn Update(&mut self) {
        let app = self.app();
        let board = self.board();

        if (*app).IsStormyNightLevel() {
            // self.UpdateStormyNight();
        }

        if board.mPaused {
            if app.mGameMode as i32 == GameMode::GAMEMODE_CHALLENGE_BEGHOULED_TWIST as i32 {
                self.mChallengeGridX = -1;
                self.mChallengeGridY = -1;
            }
            return;
        }

        if app.mGameMode as i32 == GameMode::GAMEMODE_CHALLENGE_RAINING_SEEDS as i32
            || (*app).IsStormyNightLevel()
        {
            // self.UpdateRain();
        }

        if (*app).mGameScene as i32 != GameScenes::SCENE_PLAYING as i32
            && app.mGameMode as i32 != GameMode::GAMEMODE_TREE_OF_WISDOM as i32
        {
            return;
        }

        if board.HasConveyorBeltSeedBank() {
            self.UpdateConveyorBelt();
        }

        // Dispatch to mode-specific updates
        match app.mGameMode as i32 {
            x if x == GameMode::GAMEMODE_CHALLENGE_BEGHOULED as i32
                || x == GameMode::GAMEMODE_CHALLENGE_BEGHOULED_TWIST as i32 => {}
            x if (*app).IsScaryPotterLevel() => {}
            x if (*app).IsWhackAZombieLevel() => {}
            x if (*app).IsIZombieLevel() => {}
            x if (*app).IsSlotMachineLevel() => {}
            x if x == GameMode::GAMEMODE_CHALLENGE_SPEED as i32 => {
                board.UpdateGame();
            }
            x if x == GameMode::GAMEMODE_CHALLENGE_RAINING_SEEDS as i32 => {}
            x if x == GameMode::GAMEMODE_CHALLENGE_PORTAL_COMBAT as i32 => {}
            x if x == GameMode::GAMEMODE_CHALLENGE_ZOMBIQUARIUM as i32 => {}
            x if x == GameMode::GAMEMODE_TREE_OF_WISDOM as i32 => {}
            x if x == GameMode::GAMEMODE_CHALLENGE_LAST_STAND as i32 => {}
            _ => {}
        }

        // ReanimChallenge update
        // Reanimation* aReanim = app->ReanimationTryToGet(mReanimChallenge);
    }

    /// C++ Challenge::UpdateConveyorBelt() (from Challenge.cpp:1616)
    pub unsafe fn UpdateConveyorBelt(&mut self) {
        let board = self.board();
        if self.mConveyorBeltCounter > 0 {
            self.mConveyorBeltCounter -= 1;
            if self.mConveyorBeltCounter == 0 {
                self.mConveyorBeltCounter = 1000;
                // Add a random seed to the conveyor belt bank
                // mBoard->mSeedBank->AddSeed(PickConveyorBeltSeed());
            }
        }
    }

    pub unsafe fn ZombieAtePlant(&mut self, _thePlant: *mut crate::lawn::plant::Plant) {
        // Challenge::ZombieAtePlant - track plants eaten for challenges
    }

    pub unsafe fn PlantAdded(&mut self, _thePlant: *mut crate::lawn::plant::Plant) {
        // Challenge::PlantAdded - track plants for scoring
    }

    pub unsafe fn CanPlantAt(&self, _theGridX: i32, _theGridY: i32, _theSeedType: SeedType) -> PlantingReason {
        // Challenge-specific planting rules
        PlantingReason::PLANTING_OK
    }

    pub unsafe fn IsZombieSeedType(theSeedType: SeedType) -> bool {
        matches!(theSeedType,
            SeedType::SEED_ZOMBIE_NORMAL | SeedType::SEED_ZOMBIE_TRAFFIC_CONE
            | SeedType::SEED_ZOMBIE_PAIL | SeedType::SEED_ZOMBIE_SCREEN_DOOR
            | SeedType::SEED_ZOMBIE_FOOTBALL | SeedType::SEED_ZOMBIE_DANCER
            | SeedType::SEED_ZOMBIE_BALLOON | SeedType::SEED_ZOMBIE_DIGGER
            | SeedType::SEED_ZOMBIE_POGO | SeedType::SEED_ZOMBIE_BUNGEE
            | SeedType::SEED_ZOMBIE_LADDER | SeedType::SEED_ZOMBIE_GARGANTUAR
            | SeedType::SEED_ZOMBIE_IMP | SeedType::SEED_ZOMBIE_POLEVAULTER
        )
    }

    /// C++ Challenge::SpawnZombieWave (Challenge.cpp:2964)
    pub unsafe fn SpawnZombieWave(&mut self) {
        let board = self.board();
        let app = self.app();

        if (*app).IsContinuousChallenge() && board.mCurrentWave == board.mNumWaves {
            board.mCurrentWave = board.mNumWaves - 1;
            // [TODO]: Replace flag zombies with normal
        }

        let aIsFlagWave = board.IsFlagWave(board.mCurrentWave);

        if (*app).mGameMode as i32 == GameMode::GAMEMODE_CHALLENGE_GRAVE_DANGER as i32
            && board.mCurrentWave != board.mNumWaves - 1
        {
            if aIsFlagWave {
                // [TODO]: board.SpawnZombiesFromGraves()
            } else if board.mCurrentWave > 5 {
                // [TODO]: GraveDangerSpawnRandomGrave()
            }
        }

        // 生存模式夜间：最后一波补充墓碑 (inline is_survival_mode check)
        let mode = (*app).mGameMode as i32;
        let is_survival = mode >= GameMode::GAMEMODE_SURVIVAL_NORMAL_STAGE_1 as i32
            && mode <= GameMode::GAMEMODE_SURVIVAL_ENDLESS_STAGE_5 as i32;
        if is_survival && board.mBackground == 1
            && board.mCurrentWave == board.mNumWaves - 1
        {
            // [TODO]: Spawn random grave
        }

        if (*app).IsBungeeBlitzLevel() && aIsFlagWave {
            board.DisplayAdvice("[ADVICE_BUNGEES_INCOMING]", 0, -1);
        }
    }
}

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
        }
    }

    pub fn InitLevel(&mut self) {}
    pub fn StartLevel(&mut self) {}
    pub fn Update(&mut self) {}
    pub fn ZombieAtePlant(&mut self, _thePlant: *mut crate::lawn::plant::Plant) {}
    pub fn PlantAdded(&mut self, _thePlant: *mut crate::lawn::plant::Plant) {}
}

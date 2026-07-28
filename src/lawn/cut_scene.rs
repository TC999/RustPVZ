// [TRANSLATION_NOTE]: Cutscene.h -> Rust stub
// board.rs 引用此类型，保持基本结构可用

#![allow(non_snake_case, dead_code)]

use crate::const_enums::*;

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

    pub fn StartLevelIntro(&mut self) {}
    pub fn CancelIntro(&mut self) {}
    pub fn Update(&mut self) {}
    pub fn IsCutSceneOver(&self) -> bool { true }
    pub fn IsAfterSeedChooser(&self) -> bool { true }
    pub fn IsSurvivalRepick(&self) -> bool { false }
}

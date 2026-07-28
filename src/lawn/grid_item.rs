// [TRANSLATION_NOTE]: GridItem.h -> Rust struct with C++ naming

use crate::const_enums::*;
use super::game_object::GameObject;

pub struct GridItem {
    pub base: GameObject,
    pub mGridX: i32,
    pub mGridY: i32,
    pub mGridItemType: GridItemType,
    pub mPosX: f32,
    pub mPosY: f32,
    pub mAnimCounter: i32,
    pub mFrame: i32,
    pub mDead: bool,
    pub mReanimID: ReanimationID,
    pub mParticleID: ParticleID,
    pub mCoinID: CoinID,
    pub mGridItemReanimID: ReanimationID,
    pub mGridItemState: i32,
    pub mGridItemCounter: i32,
    pub mRenderOrder: i32,
    pub mSwingX: f32,
    pub mSwingY: f32,
    pub mDoorDir: i32,
    pub mDoorMoving: bool,
    pub mTinyUranMovX: i32,
    pub mTinyUranMovY: i32,
    pub mTinyUranCounter: i32,
    pub mCraterCounter: i32,
}

impl GridItem {
    pub fn new() -> Self {
        GridItem {
            base: GameObject::new(),
            mGridX: 0,
            mGridY: 0,
            mGridItemType: GridItemType::GRIDITEM_NONE,
            mPosX: 0.0,
            mPosY: 0.0,
            mAnimCounter: 0,
            mFrame: 0,
            mDead: false,
            mReanimID: ReanimationID::REANIMATIONID_NULL,
            mParticleID: ParticleID::PARTICLEID_NULL,
            mCoinID: CoinID::COINID_NULL,
            mGridItemReanimID: ReanimationID::REANIMATIONID_NULL,
            mGridItemState: 0,
            mGridItemCounter: 0,
            mRenderOrder: 0,
            mSwingX: 0.0,
            mSwingY: 0.0,
            mDoorDir: 0,
            mDoorMoving: false,
            mTinyUranMovX: 0,
            mTinyUranMovY: 0,
            mTinyUranCounter: 0,
            mCraterCounter: 0,
        }
    }
}

impl Default for GridItem {
    fn default() -> Self {
        Self::new()
    }
}

impl GridItem {
    pub unsafe fn Update(&mut self) {
        if self.mDead { return; }
        self.mGridItemCounter += 1;

        match self.mGridItemType {
            GridItemType::GRIDITEM_GRAVESTONE => {
                // Grave stones don't update
            }
            GridItemType::GRIDITEM_CRATER => {
                if self.mCraterCounter > 0 {
                    self.mCraterCounter -= 1;
                    if self.mCraterCounter == 0 {
                        // Crater becomes grass again
                        self.mDead = true;
                    }
                }
            }
            GridItemType::GRIDITEM_PORTAL_CIRCLE => {
                // Portal visual animation
                self.mAnimCounter += 1;
                if self.mAnimCounter >= 8 {
                    self.mAnimCounter = 0;
                    self.mFrame += 1;
                    if self.mFrame >= 4 { self.mFrame = 0; }
                }
            }
            GridItemType::GRIDITEM_LADDER => {
                // Update ladder position
            }
            _ => {}
        }
    }

    pub unsafe fn Draw(&self, _g: &mut crate::sexy_app_framework::graphics::graphics::Graphics) {
        if self.mDead { return; }
        // TODO: Draw based on mGridItemType
    }

    pub unsafe fn GridItemInitialize(&mut self, theGridX: i32, theGridY: i32, theGridItemType: GridItemType) {
        self.mGridX = theGridX;
        self.mGridY = theGridY;
        self.mGridItemType = theGridItemType;
        self.mDead = false;
        self.mPosX = theGridX as f32 * 80.0 + 40.0;
        self.mPosY = 80.0 + theGridY as f32 * 100.0;
        self.mRenderOrder = 0;
        self.mGridItemCounter = 0;
        self.mGridItemState = 0;

        match theGridItemType {
            GridItemType::GRIDITEM_GRAVESTONE => {
                self.mCraterCounter = 0;
            }
            GridItemType::GRIDITEM_CRATER => {
                self.mCraterCounter = 3000; // Time until crater disappears
            }
            _ => {}
        }
    }
}

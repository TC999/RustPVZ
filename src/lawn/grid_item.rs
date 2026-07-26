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

// [TRANSLATION_NOTE]: LawnMower.h -> Rust struct

use crate::const_enums::*;
use super::game_object::GameObject;

pub struct LawnMower {
    pub base: GameObject,
    pub mMowerType: LawnMowerType,
    pub mPosX: f32,
    pub mPosY: f32,
    pub mRow: i32,
    pub mMowerState: MowerState,
    pub mGroundY: i32,
    pub mAnimCounter: i32,
    pub mFrame: i32,
    pub mRollingInCounter: i32,
    pub mVelX: f32,
    pub mSquishCounter: i32,
    pub mDrivingCount: bool,
    pub mDead: bool,
    pub mLawnMowerAge: i32,
    pub mParticleID: ParticleID,
    pub mAttachmentID: AttachmentID,
    pub mVisible: bool,
    pub mRenderOrder: i32,
}

impl LawnMower {
    pub fn new() -> Self {
        LawnMower {
            base: GameObject::new(),
            mMowerType: LawnMowerType::LAWNMOWER_NORMAL,
            mPosX: 0.0,
            mPosY: 0.0,
            mRow: 0,
            mMowerState: MowerState::MOWER_READY,
            mGroundY: 0,
            mAnimCounter: 0,
            mFrame: 0,
            mRollingInCounter: 0,
            mVelX: 0.0,
            mSquishCounter: 0,
            mDrivingCount: false,
            mDead: false,
            mLawnMowerAge: 0,
            mParticleID: ParticleID::PARTICLEID_NULL,
            mAttachmentID: AttachmentID::ATTACHMENTID_NULL,
            mVisible: true,
            mRenderOrder: 0,
        }
    }
}

impl Default for LawnMower {
    fn default() -> Self {
        Self::new()
    }
}

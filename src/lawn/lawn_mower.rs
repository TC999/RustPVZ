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

impl LawnMower {
    pub unsafe fn Update(&mut self) {
        if self.mDead { return; }
        self.mLawnMowerAge += 1;

        match self.mMowerState {
            MowerState::MOWER_READY => {
                // Stationary, waiting for a zombie
            }
            MowerState::MOWER_TRIGGERED => {
                self.mVelX = 3.5;
                self.mPosX += self.mVelX;
                self.mAnimCounter += 1;
                if self.mAnimCounter >= 4 {
                    self.mAnimCounter = 0;
                    self.mFrame += 1;
                }
                // Check if offscreen
                if self.mPosX > 900.0 {
                    self.mDead = true;
                }
            }
            MowerState::MOWER_TRIGGERED_SQUASHED => {
                // Squished animation
                self.mSquishCounter += 1;
                if self.mSquishCounter > 50 {
                    self.mDead = true;
                }
            }
            MowerState::MOWER_OFF_LAWN => {
                // Rolling in at level start (using OFF_LAWN as closest equivalent)
                self.mRollingInCounter -= 1;
                if self.mRollingInCounter <= 0 {
                    self.mMowerState = MowerState::MOWER_READY;
                }
            }
            _ => {}
        }

        self.base.m_x = self.mPosX as i32;
        self.base.m_y = self.mPosY as i32;
    }

    pub unsafe fn Draw(&self, _g: &mut crate::sexy_app_framework::graphics::graphics::Graphics) {
        if !self.mVisible || self.mDead { return; }
        // TODO: Draw lawn mower sprite
    }

    pub unsafe fn Die(&mut self) {
        self.mDead = true;
        self.mVisible = false;
    }

    pub unsafe fn EnableSuperMower(&mut self, _enable: bool) {
        // TODO: Change to super mower appearance
    }
}

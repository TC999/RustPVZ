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
    /// C++ LawnMower::LawnMowerInitialize (LawnMower.cpp:30)
    pub unsafe fn LawnMowerInitialize(&mut self, theRow: i32) {
        self.mRow = theRow;
        self.mPosX = 40.0;
        self.mPosY = 0.0; // [TODO]: Set from Board row position
        self.base.m_row = theRow;
        self.mMowerState = MowerState::MOWER_OFF_LAWN;
        self.mRollingInCounter = 0;
        self.mDead = false;
        self.mVisible = true;
        self.mLawnMowerAge = 0;
        self.mAnimCounter = 0;
        self.mFrame = 0;
        self.mVelX = 0.0;
        self.mSquishCounter = 0;
        self.mDrivingCount = false;
        self.mParticleID = ParticleID::PARTICLEID_NULL;
        self.mAttachmentID = AttachmentID::ATTACHMENTID_NULL;
        self.mRenderOrder = 0;
        // [TODO]: Set ground Y from board
    }

    /// C++ LawnMower::Update (LawnMower.cpp:178)
    pub unsafe fn Update(&mut self) {
        if self.mDead { return; }
        self.mLawnMowerAge += 1;

        // 被压扁状态
        if self.mMowerState == MowerState::MOWER_TRIGGERED_SQUASHED {
            self.mSquishCounter += 1;
            if self.mSquishCounter > 50 {
                self.mDead = true;
            }
            return;
        }

        // 滚入场
        if self.mMowerState == MowerState::MOWER_OFF_LAWN {
            self.mRollingInCounter += 1;
            self.mPosX = -21.0; // approximate end position
            if self.mRollingInCounter >= 100 {
                self.mMowerState = MowerState::MOWER_READY;
            }
            self.base.m_x = self.mPosX as i32;
            self.base.m_y = self.mPosY as i32;
            return;
        }

        let app = unsafe { &mut *(self.base.m_app as *mut crate::lawn_app::LawnApp) };
        if (*app).mGameScene as i32 != GameScenes::SCENE_PLAYING as i32 {
            return;
        }

        // 触发后的移动
        if self.mMowerState == MowerState::MOWER_TRIGGERED {
            self.mVelX = 3.5;
            self.mPosX += self.mVelX;
            // 动画
            self.mAnimCounter += 1;
            if self.mAnimCounter >= 4 {
                self.mAnimCounter = 0;
                self.mFrame += 1;
            }
            // 超出屏幕
            if self.mPosX > 900.0 {
                self.mDead = true;
            }
        } else {
            // MOWER_READY: 碰撞检测
            // [TODO]: Iterate zombies on same row, check overlap with attack rect
            // if overlap > threshold → mMowerState = MOWER_TRIGGERED
        }

        self.base.m_x = self.mPosX as i32;
        self.base.m_y = self.mPosY as i32;
    }

    pub unsafe fn Draw(&self, _g: &mut crate::sexy_app_framework::graphics::graphics::Graphics) {
        if !self.mVisible || self.mDead { return; }
        // TODO: Draw lawn mower sprite based on mMowerType and mFrame
    }

    pub unsafe fn Die(&mut self) {
        self.mDead = true;
        self.mVisible = false;
    }

    pub unsafe fn EnableSuperMower(&mut self, _enable: bool) {
        // TODO: Change to super mower appearance
    }
}

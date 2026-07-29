// [TRANSLATION_NOTE]: LawnMower.cpp -> Rust 翻译
// 割草机系统 — 草坪、泳池、屋顶三种类型

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
    // C++ 扩展字段
    pub mReanimID: ReanimationID,
    pub mChompCounter: i32,
    pub mSquishedCounter: i32,
    pub mLastPortalX: i32,
}

// 辅助：获取全局 LawnApp
unsafe fn g_app() -> &'static mut crate::lawn_app::LawnApp {
    &mut *crate::lawn_app::G_LAWN_APP
}

impl LawnMower {
    pub fn new() -> Self {
        LawnMower {
            base: GameObject::new(),
            mMowerType: LawnMowerType::LAWNMOWER_NORMAL,
            mPosX: 0.0, mPosY: 0.0, mRow: 0,
            mMowerState: MowerState::MOWER_READY,
            mGroundY: 0, mAnimCounter: 0, mFrame: 0,
            mRollingInCounter: 0, mVelX: 0.0,
            mSquishCounter: 0, mDrivingCount: false,
            mDead: false, mLawnMowerAge: 0,
            mParticleID: ParticleID::PARTICLEID_NULL,
            mAttachmentID: AttachmentID::ATTACHMENTID_NULL,
            mVisible: true, mRenderOrder: 0,
            mReanimID: ReanimationID::REANIMATIONID_NULL,
            mChompCounter: 0, mSquishedCounter: 0, mLastPortalX: -1,
        }
    }

    // =========================================================================
    // LawnMowerInitialize — C++ 保真翻译 (LawnMower.cpp:30)
    // =========================================================================
    pub unsafe fn LawnMowerInitialize(&mut self, the_row: i32) {
        let app = g_app();

        self.mRow = the_row;
        self.mPosX = -160.0;
        self.mRenderOrder = crate::lawn::board::Board::MakeRenderOrder(
            RenderLayer::RENDER_LAYER_LAWN_MOWER, the_row, 0
        );
        // C++: mPosY = mBoard->GetPosYBasedOnRow(mPosX + 40.0f, theRow) + 23.0f;
        // [TODO]: GetPosYBasedOnRow 尚未实现
        self.mPosY = 80.0 + the_row as f32 * 100.0 + 23.0;
        self.mDead = false;
        self.mMowerState = MowerState::MOWER_READY;
        self.mVisible = true;
        self.mChompCounter = 0;
        self.mRollingInCounter = 0;
        self.mSquishedCounter = 0;
        self.mLastPortalX = -1;
        self.base.m_row = the_row;

        // C++: 根据关卡类型选择割草机类型
        let mut a_reanim_type = ReanimationType::REANIM_LAWNMOWER;
        let has_roof = (*app).m_board.as_ref().map_or(false, |b| b.StageHasRoof());
        let super_mower = (*app).m_board.as_ref().map_or(false, |b| b.mSuperMowerMode);
        if has_roof {
            self.mMowerType = LawnMowerType::LAWNMOWER_ROOF_CLEANER;
            a_reanim_type = ReanimationType::REANIM_ROOF_CLEANER;
        } else if false {
            // C++: mBoard->mPlantRow[mRow] == PLANTROW_POOL && mApp->mPlayerInfo->mPurchases[STORE_ITEM_POOL_CLEANER]
            // [TODO]: 泳道检测和商店购买检查
            // self.mMowerType = LAWNMOWER_POOL;
            // a_reanim_type = REANIM_POOL_CLEANER;
        } else {
            self.mMowerType = LawnMowerType::LAWNMOWER_NORMAL;
            a_reanim_type = ReanimationType::REANIM_LAWNMOWER;
        }

        // C++: 创建割草机动画
        let a_mower_reanim = app.AddReanimation(0.0, 18.0, self.mRenderOrder, a_reanim_type);
        if !a_mower_reanim.is_null() {
            // [TODO]: 设置动画属性（mAnimRate = 0, mLoopType = REANIM_LOOP, OverrideScale 0.85）
            self.mReanimID = app.ReanimationGetID(a_mower_reanim);
        }

        // C++: 超级割草机模式
        if super_mower && self.mMowerType == LawnMowerType::LAWNMOWER_NORMAL {
            self.EnableSuperMower(true);
        }

        self.base.m_x = self.mPosX as i32;
        self.base.m_y = self.mPosY as i32;
    }

    // =========================================================================
    // Update — C++ 保真翻译 (LawnMower.cpp:178)
    // =========================================================================
    pub unsafe fn Update(&mut self) {
        if self.mDead { return; }
        self.mLawnMowerAge += 1;

        // C++: 被压扁状态
        if self.mMowerState == MowerState::MOWER_TRIGGERED_SQUASHED {
            self.mSquishedCounter += 1;
            if self.mSquishedCounter > 30 {
                self.mDead = true;
            }
            return;
        }

        // C++: 滚入场
        if self.mMowerState == MowerState::MOWER_OFF_LAWN {
            self.mRollingInCounter += 1;
            if self.mRollingInCounter < 100 {
                self.mPosX = -160.0 + self.mRollingInCounter as f32 * 1.2;
            } else {
                self.mMowerState = MowerState::MOWER_READY;
                self.mPosX = -21.0;
            }
            self.base.m_x = self.mPosX as i32;
            self.base.m_y = self.mPosY as i32;
            return;
        }

        // C++: 池中割草机特殊更新
        if self.mMowerType == LawnMowerType::LAWNMOWER_POOL_CLEANER {
            self.UpdatePool();
        }

        // C++: 场景检查
        let app = g_app();
        if (*app).mGameScene as i32 != GameScenes::SCENE_PLAYING as i32 {
            // C++: 如果割草机正在移动，允许继续
            if self.mMowerState != MowerState::MOWER_TRIGGERED {
                return;
            }
        }

        // C++: 触发后移动
        if self.mMowerState == MowerState::MOWER_TRIGGERED {
            if self.mMowerType == LawnMowerType::LAWNMOWER_POOL_CLEANER {
                self.mVelX = 2.5;
            } else {
                self.mVelX = 3.5;
            }
            self.mPosX += self.mVelX;

            // C++: 动画帧
            self.mAnimCounter += 1;
            if self.mAnimCounter >= 4 {
                self.mAnimCounter = 0;
                self.mFrame += 1;
            }

            // C++: 咬合计数器
            if self.mChompCounter > 0 {
                self.mChompCounter -= 1;
            }

            // C++: 碰撞检测 — 碾过僵尸
            // [TODO]: Iterate zombies on same row, check overlap
            // if overlap -> MowZombie(theZombie)

            // C++: 超出屏幕
            if self.mPosX > 900.0 {
                self.mDead = true;
            }
        } else {
            // C++: MOWER_READY — 待触发状态
            // [TODO]: 碰撞检测 — 检查是否有僵尸到达割草机位置
        }

        self.base.m_x = self.mPosX as i32;
        self.base.m_y = self.mPosY as i32;
    }

    // =========================================================================
    // UpdatePool — C++ 保真翻译 (LawnMower.cpp:87)
    // =========================================================================
    pub unsafe fn UpdatePool(&mut self) {
        // C++: 泳池割草机 — 检测是否在水中,调整渲染层
        let is_pool_range = self.mPosX > 26.0 && self.mPosX < 660.0;
        if is_pool_range {
            self.base.m_render_order = crate::lawn::board::Board::MakeRenderOrder(
                RenderLayer::RENDER_LAYER_LAWN_MOWER, self.mRow, 1
            );
        } else {
            self.base.m_render_order = crate::lawn::board::Board::MakeRenderOrder(
                RenderLayer::RENDER_LAYER_LAWN_MOWER, self.mRow, 0
            );
        }
    }

    // =========================================================================
    // MowZombie — C++ 保真翻译 (LawnMower.cpp:143)
    // =========================================================================
    pub unsafe fn MowZombie(&mut self, the_zombie: *mut super::zombie::Zombie) {
        // C++: 割草机碾过僵尸
        // [TODO]: PlayFoley(FOLEY_MOWER)
        // [TODO]: theZombie->DieWithLard() 或 theZombie->MowZombie()
        // [TODO]: 产生粒子效果
        self.mChompCounter = 10;
    }

    // =========================================================================
    // StartMower — C++ 保真翻译 (LawnMower.cpp:381)
    // =========================================================================
    pub unsafe fn StartMower(&mut self) {
        self.mMowerState = MowerState::MOWER_TRIGGERED;
        // C++: 设置动画速度
        // [TODO]: aReanim->mAnimRate = 24.0
    }

    // =========================================================================
    // SquishMower — C++ 保真翻译 (LawnMower.cpp:405)
    // =========================================================================
    pub unsafe fn SquishMower(&mut self) {
        self.mMowerState = MowerState::MOWER_TRIGGERED_SQUASHED;
        self.mSquishedCounter = 0;
        // [TODO]: 播放被压扁动画
    }

    // =========================================================================
    // EnableSuperMower — C++ 保真翻译 (LawnMower.cpp:421)
    // =========================================================================
    pub unsafe fn EnableSuperMower(&mut self, _the_enable: bool) {
        // C++: 将割草机升级为超级割草机（金色外观）
        // [TODO]: 修改动画和外观
    }

    // =========================================================================
    // Draw — C++ 保真翻译 (LawnMower.cpp:274)
    // =========================================================================
    pub unsafe fn Draw(&self, g: &mut crate::sexy_app_framework::graphics::graphics::Graphics) {
        if !self.mVisible || self.mDead { return; }

        // C++: 根据割草机类型和状态绘制
        // [TODO]:
        // - 被压扁的割草机: 绘制翻转/压扁图像
        // - 正常割草机: 使用重动画绘制
        // - 泳池割草机: 水中绘制特殊版本
    }

    // =========================================================================
    // Die — C++ 保真翻译 (LawnMower.cpp:368)
    // =========================================================================
    pub unsafe fn Die(&mut self) {
        self.mDead = true;
        self.mVisible = false;
        // [TODO]: RemoveReanimation(mReanimID)
    }
}

impl Default for LawnMower {
    fn default() -> Self {
        Self::new()
    }
}

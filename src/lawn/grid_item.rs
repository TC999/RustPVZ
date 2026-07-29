// [TRANSLATION_NOTE]: GridItem.cpp -> Rust 翻译
// 网格物品系统 — 墓碑、弹坑、梯子、传送门、惊吓罐、大脑、耙子等

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
    pub mGridItemParticleID: ParticleSystemID,
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
    // 扩展字段 (从 C++ GridItem 翻译)
    pub mGoalX: f32,
    pub mGoalY: f32,
    pub mZombieType: ZombieType,
    pub mSeedType: SeedType,
    pub mScaryPotType: ScaryPotType,
    pub mHighlighted: bool,
    pub mTransparentCounter: i32,
    pub mSunCount: i32,
    pub mMotionTrailCount: i32,
}

// 辅助函数：获取全局 LawnApp
unsafe fn g_app() -> &'static mut crate::lawn_app::LawnApp {
    &mut *crate::lawn_app::G_LAWN_APP
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
            mGridItemParticleID: ParticleSystemID::PARTICLESYSTEMID_NULL,
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
            mGoalX: 0.0,
            mGoalY: 0.0,
            mZombieType: ZombieType::ZOMBIE_INVALID,
            mSeedType: SeedType::SEED_NONE,
            mScaryPotType: ScaryPotType::SCARYPOT_NONE,
            mHighlighted: false,
            mTransparentCounter: 0,
            mSunCount: 0,
            mMotionTrailCount: 0,
        }
    }

    /// C++ GridItem::GridItemDie (GridItem.cpp:64)
    pub unsafe fn GridItemDie(&mut self) {
        self.mDead = true;

        // C++: 释放重动画
        let app = g_app();
        let a_reanim = app.ReanimationTryToGet(self.mGridItemReanimID);
        if !a_reanim.is_null() {
            // [TODO]: a_reanim->ReanimationDie()
            self.mGridItemReanimID = ReanimationID::REANIMATIONID_NULL;
        }

        // C++: 释放粒子系统
        let a_particle = app.ParticleTryToGet(self.mGridItemParticleID);
        if !a_particle.is_null() {
            // [TODO]: a_particle->ParticleSystemDie()
        }
    }

    // =========================================================================
    // GridItemInitialize — C++ 保真翻译
    // 各类型网格物品的初始化逻辑
    // =========================================================================
    pub unsafe fn GridItemInitialize(&mut self, the_grid_x: i32, the_grid_y: i32, the_grid_item_type: GridItemType) {
        let app = g_app();

        self.mGridX = the_grid_x;
        self.mGridY = the_grid_y;
        self.mGridItemType = the_grid_item_type;
        self.mDead = false;
        self.mPosX = the_grid_x as f32 * 80.0 + 40.0;
        self.mPosY = 80.0 + the_grid_y as f32 * 100.0;
        self.mRenderOrder = 0;
        self.mGridItemCounter = 0;
        self.mGridItemState = 0;
        self.mGoalX = self.mPosX;
        self.mGoalY = self.mPosY;
        self.base.m_x = self.mPosX as i32;
        self.base.m_y = self.mPosY as i32;

        match the_grid_item_type {
            GridItemType::GRIDITEM_GRAVESTONE => {
                self.mCraterCounter = 0;
                // [TODO]: 随机墓碑类型 + 创建墓碑动画
                // ZombieType aZombieType = (ZombieType)RandRangeInt(ZOMBIE_NORMAL, ZOMBIE_SCREEN_DOOR + 1);
                // 创建 GRIDITEM_GRAVESTONE 的 Reanimation
            }
            GridItemType::GRIDITEM_CRATER => {
                self.mCraterCounter = 3000;
                // [TODO]: 创建弹坑粒子效果
            }
            GridItemType::GRIDITEM_PORTAL_CIRCLE => {
                // C++: UpdatePortal() / GridItemInitialize 传送门
                self.mAnimCounter = 0;
                self.mFrame = 0;
                // [TODO]: 创建传送门重动画
            }
            GridItemType::GRIDITEM_RAKE => {
                self.mGridItemState = 0; // GRIDITEM_STATE_RAKE_WAITING
                // [TODO]: 创建耙子重动画
            }
            GridItemType::GRIDITEM_SCARY_POT => {
                // [TODO]: 创建惊吓罐
                // mScaryPotType = (ScaryPotType)RandRangeInt(0, NUM_SCARY_POT_TYPES);
                // 创建惊吓罐重动画
            }
            GridItemType::GRIDITEM_BRAIN => {
                // [TODO]: 创建大脑
            }
            _ => {}
        }
    }

    // =========================================================================
    // Update — C++ 保真翻译 (GridItem.cpp:589)
    // =========================================================================
    pub unsafe fn Update(&mut self) {
        if self.mDead {
            return;
        }

        let app = g_app();

        // C++: 更新附件重动画和粒子
        let a_reanim = app.ReanimationTryToGet(self.mGridItemReanimID);
        if !a_reanim.is_null() {
            // [TODO]: a_reanim->Update()
        }

        let a_particle = app.ParticleTryToGet(self.mGridItemParticleID);
        if !a_particle.is_null() {
            // [TODO]: a_particle->Update()
        }

        // C++: 按类型分发
        match self.mGridItemType {
            GridItemType::GRIDITEM_GRAVESTONE => {
                // C++: 墓碑基本不更新，但可能有复活逻辑
                // [TODO]: 僵尸从墓碑爬出的逻辑
            }
            GridItemType::GRIDITEM_CRATER => {
                // C++: 弹坑倒计时消失
                if self.mCraterCounter > 0 {
                    self.mCraterCounter -= 1;
                    if self.mCraterCounter == 0 {
                        self.GridItemDie();
                    }
                }
            }
            GridItemType::GRIDITEM_PORTAL_CIRCLE => {
                // C++: UpdatePortal() — 传送门逻辑
                // [TODO]: 打开/关闭传送门，传送僵尸
                // 简单动画帧更新
                self.mAnimCounter += 1;
                if self.mAnimCounter >= 8 {
                    self.mAnimCounter = 0;
                    self.mFrame += 1;
                    if self.mFrame >= 4 {
                        self.mFrame = 0;
                    }
                }
            }
            GridItemType::GRIDITEM_SCARY_POT => {
                // C++: UpdateScaryPot() — 惊吓罐逻辑
                // [TODO]: 检测僵尸靠近，破碎并释放内容
                self.mGridItemCounter += 1;
            }
            GridItemType::GRIDITEM_RAKE => {
                // C++: UpdateRake() — 耙子逻辑
                // [TODO]: 检测僵尸，触发耙子攻击
                if self.mGridItemState == 0 || self.mGridItemState == 1 {
                    // 检测是否有僵尸在耙子位置
                    // 如果有，切换到吸引状态
                    self.mGridItemCounter += 1;
                }
            }
            GridItemType::GRIDITEM_BRAIN => {
                // C++: 大脑 — 处理透明闪烁
                if self.mTransparentCounter > 0 {
                    self.mTransparentCounter -= 1;
                }
            }
            _ => {}
        }
    }

    // =========================================================================
    // Draw — 分派到类型特定绘制 (C++ GridItem.cpp:94)
    // =========================================================================
    pub unsafe fn Draw(&self, g: &mut crate::sexy_app_framework::graphics::graphics::Graphics) {
        if self.mDead {
            return;
        }

        // C++: 按类型绘制
        match self.mGridItemType {
            GridItemType::GRIDITEM_GRAVESTONE => {
                // [TODO]: DrawGraveStone(g)
            }
            GridItemType::GRIDITEM_CRATER => {
                // [TODO]: DrawCrater(g)
            }
            GridItemType::GRIDITEM_LADDER => {
                // [TODO]: DrawLadder(g)
            }
            GridItemType::GRIDITEM_BRAIN => {
                // g->DrawImageF(IMAGE_BRAIN, mPosX, mPosY);
            }
            _ => {}
        }

        // C++: 绘制附加动画
        let app = g_app();
        let a_reanim = app.ReanimationTryToGet(self.mGridItemReanimID);
        if !a_reanim.is_null() {
            // [TODO]: a_reanim->Draw(g)
        }

        let a_particle = app.ParticleTryToGet(self.mGridItemParticleID);
        if !a_particle.is_null() {
            // [TODO]: a_particle->Draw(g)
        }
    }

    /// C++ GridItem::DrawGridItemOverlay (GridItem.cpp:82)
    pub unsafe fn DrawGridItemOverlay(&self, _g: &mut crate::sexy_app_framework::graphics::graphics::Graphics) {
        // [TODO]: DrawGridItemOverlay — 如 Stinky 巧克力提示
    }
}

impl Default for GridItem {
    fn default() -> Self {
        Self::new()
    }
}

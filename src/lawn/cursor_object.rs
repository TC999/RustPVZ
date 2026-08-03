// [TRANSLATION_NOTE]: CursorObject.cpp -> Rust 翻译
// 游戏内光标系统 — 铲子、水壶、肥料、手套、锤子、玉米加农炮目标等

use crate::const_enums::*;
use crate::sexy_app_framework::graphics::graphics::Graphics;

pub const CURSOR_UPDATE_INTERVAL: i32 = 10;

/// 游戏按钮（简化定义，对应 C++ GameButton）
pub struct GameButton {
    pub mId: i32,
    pub mX: i32,
    pub mY: i32,
    pub mWidth: i32,
    pub mHeight: i32,
    pub mLabel: String,
    pub mBtnNoDraw: bool,
    pub mDisabled: bool,
    pub mVisible: bool,
}

impl GameButton {
    pub fn SetLabel(&mut self, the_label: &str) {
        self.mLabel = the_label.to_string();
    }

    pub fn Resize(&mut self, the_x: i32, the_y: i32, the_width: i32, the_height: i32) {
        self.mX = the_x;
        self.mY = the_y;
        self.mWidth = the_width;
        self.mHeight = the_height;
    }
}

impl GameButton {
    pub fn new() -> Self {
        GameButton {
            mId: 0, mX: 0, mY: 0,
            mWidth: 0, mHeight: 0,
            mLabel: String::new(),
            mBtnNoDraw: false,
            mDisabled: false,
            mVisible: true,
        }
    }

    pub unsafe fn Update(&mut self) {
        // [TODO]: Update button state (hover, click, etc.)
    }
}

/// 游戏内光标对象（跟随鼠标显示的物品/工具）
pub struct CursorObject {
    pub mType: SeedType,
    pub mImitaterType: SeedType,
    pub mSeedBankIndex: i32,
    pub mX: i32,
    pub mY: i32,
    pub mCursorType: CursorType,
    pub mCoinID: CoinID,
    pub mDuplicatorPlantID: PlantID,
    pub mCobCannonPlantID: PlantID,
    pub mGlovePlantID: PlantID,
    pub mReanimCursorID: ReanimationID,
    pub mWidth: i32,
    pub mHeight: i32,
    pub mHammerDownCounter: i32,
    pub mVisible: bool,
    pub mCursorCount: i32,
}

impl CursorObject {
    pub fn new() -> Self {
        CursorObject {
            mType: SeedType::SEED_NONE,
            mImitaterType: SeedType::SEED_NONE,
            mSeedBankIndex: -1,
            mX: 0, mY: 0,
            mCursorType: CursorType::CURSOR_TYPE_NORMAL,
            mCoinID: CoinID::COINID_NULL,
            mDuplicatorPlantID: PlantID::PLANTID_NULL,
            mCobCannonPlantID: PlantID::PLANTID_NULL,
            mGlovePlantID: PlantID::PLANTID_NULL,
            mReanimCursorID: ReanimationID::REANIMATIONID_NULL,
            mWidth: 80, mHeight: 80,
            mHammerDownCounter: 0,
            mVisible: true,
            mCursorCount: 0,
        }
    }

    /// C++ CursorObject::Update() — C++ 保真翻译 (CursorObject.cpp:49)
    pub unsafe fn Update(&mut self) {
        let app = &mut *crate::lawn_app::G_LAWN_APP;

        // C++: 场景检查
        if (*app).mGameScene != GameScenes::SCENE_PLAYING {
            // [TODO]: && !mBoard->mCutScene->IsInShovelTutorial()
            self.mVisible = false;
            return;
        }

        // C++: 鼠标是否在窗口内
        // [TODO]: mApp->mWidgetManager->mMouseIn
        // 暂时假设鼠标在窗口内

        // C++: 更新光标动画
        let a_cursor_reanim = (*app).ReanimationTryToGet(self.mReanimCursorID);
        if !a_cursor_reanim.is_null() {
            // [TODO]: aCursorReanim->Update()
        }

        self.mVisible = true;
        // C++: mX = mApp->mWidgetManager->mLastMouseX - 25;
        // C++: mY = mApp->mWidgetManager->mLastMouseY - 35;
        // [TODO]: 获取鼠标位置
    }

    /// C++ CursorObject::Die() — C++ 保真翻译 (CursorObject.cpp:74)
    pub unsafe fn Die(&mut self) {
        let app = &mut *crate::lawn_app::G_LAWN_APP;
        (*app).RemoveReanimation(self.mReanimCursorID);
        self.mReanimCursorID = ReanimationID::REANIMATIONID_NULL;
    }

    /// C++ CursorObject::Draw(Graphics* g) — C++ 保真翻译 (CursorObject.cpp:80)
    pub unsafe fn Draw(&self, g: &mut Graphics) {
        // C++: switch (mCursorType)
        match self.mCursorType {
            CursorType::CURSOR_TYPE_SHOVEL => {
                // g->DrawImage(IMAGE_SHOVEL, 10, -30);
            }
            CursorType::CURSOR_TYPE_WATERING_CAN => {
                // [TODO]: 检查金色水壶
                // g->DrawImage(IMAGE_WATERINGCAN, -3, 12);
            }
            CursorType::CURSOR_TYPE_FERTILIZER => {
                // g->DrawImage(IMAGE_FERTILIZER, -15, 0);
            }
            CursorType::CURSOR_TYPE_BUG_SPRAY => {
                // g->DrawImage(IMAGE_BUG_SPRAY, -9, -1);
            }
            CursorType::CURSOR_TYPE_PHONOGRAPH => {
                // g->DrawImage(IMAGE_PHONOGRAPH, -17, 10);
            }
            CursorType::CURSOR_TYPE_CHOCOLATE => {
                // g->DrawImage(IMAGE_CHOCOLATE, -2, -8);
            }
            CursorType::CURSOR_TYPE_GLOVE => {
                // g->DrawImage(IMAGE_ZEN_GARDENGLOVE, -17, 15);
            }
            CursorType::CURSOR_TYPE_MONEY_SIGN => {
                // g->DrawImage(IMAGE_ZEN_MONEYSIGN, -17, -10);
            }
            CursorType::CURSOR_TYPE_TREE_FOOD => {
                // g->DrawImage(IMAGE_TREEFOOD, -15, 0);
            }
            CursorType::CURSOR_TYPE_WHEEELBARROW => {
                // [TODO]: 绘制手推车 + 盆栽植物
            }
            CursorType::CURSOR_TYPE_PLANT_FROM_GLOVE
            | CursorType::CURSOR_TYPE_PLANT_FROM_WHEEL_BARROW => {
                // [TODO]: 从手套/手推车中取出的植物绘制
            }
            CursorType::CURSOR_TYPE_PLANT_FROM_BANK
            | CursorType::CURSOR_TYPE_PLANT_FROM_USABLE_COIN
            | CursorType::CURSOR_TYPE_PLANT_FROM_DUPLICATOR => {
                // C++: Plant::DrawSeedType(g, mType, mImitaterType, VARIATION_NORMAL, aOffsetX, aOffsetY)
                // [TODO]: 绘制选中的种子/植物
            }
            CursorType::CURSOR_TYPE_HAMMER => {
                // C++: mApp->ReanimationGet(mReanimCursorID)->Draw(g)
                // [TODO]: 绘制锤子动画
            }
            CursorType::CURSOR_TYPE_COBCANNON_TARGET => {
                // [TODO]: 玉米加农炮瞄准目标
            }
            CursorType::CURSOR_TYPE_NORMAL => {
                // 不绘制
            }
        }
    }
}

/// 种植预览（鼠标悬停时显示植物放置在草坪上的位置）
pub struct CursorPreview {
    pub mX: i32,
    pub mY: i32,
    pub mGridX: i32,
    pub mGridY: i32,
    pub mVisible: bool,
    pub mWidth: i32,
    pub mHeight: i32,
}

impl CursorPreview {
    pub fn new() -> Self {
        CursorPreview {
            mX: 0, mY: 0,
            mGridX: 0, mGridY: 0,
            mVisible: false,
            mWidth: 80, mHeight: 80,
        }
    }

    /// C++ CursorPreview::Update() — C++ 保真翻译 (CursorPreview.cpp:238)
    pub unsafe fn Update(&mut self) {
        let app = &mut *crate::lawn_app::G_LAWN_APP;

        // C++: 场景检查
        if (*app).mGameScene != GameScenes::SCENE_PLAYING {
            self.mVisible = false;
            return;
        }

        // [TODO]: 获取当前种子类型和鼠标位置
        // C++ 完整逻辑:
        // 1. 获取鼠标在网格上的坐标 (PlantingPixelToGridX/Y)
        // 2. 检查是否可以种植 (CanPlantAt)
        // 3. 如果可以, 计算 GridToPixelX/Y 并设置 mVisible = true
        // 4. 否则 mVisible = false

        self.mVisible = false;
    }

    /// C++ CursorPreview::Draw() — C++ 保真翻译 (CursorPreview.cpp:278)
    pub unsafe fn Draw(&self, g: &mut Graphics) {
        if !self.mVisible {
            return;
        }

        // C++: 设置半透明颜色绘制预览
        g.SetColorizeImages(true);
        // g->SetColor(Color(255, 255, 255, 100));

        // [TODO]: 绘制植物预览
        // C++: Plant::DrawSeedType(g, ..., DrawVariation::VARIATION_NORMAL, ...)

        g.SetColorizeImages(false);
    }
}

impl Default for CursorObject {
    fn default() -> Self { Self::new() }
}

impl Default for CursorPreview {
    fn default() -> Self { Self::new() }
}

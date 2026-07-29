// [TRANSLATION_NOTE]: Coin.cpp -> Rust 翻译
// 金币/阳光/钻石/种子包等可收集物品

use crate::const_enums::*;
use super::game_object::GameObject;

#[derive(Clone)]
pub struct PottedPlant {
    pub m_potted_plant_index: i32,
    pub m_seed_type: SeedType,
    pub m_draw_variation: DrawVariation,
    pub m_age: i32,
    pub m_need: i32,
    pub m_watered: bool,
    pub m_last_watered_time: i32,
    pub m_last_fertilized_time: i32,
    pub m_last_bug_sprayed_time: i32,
    pub m_last_phonograph_time: i32,
    pub m_last_chocolate_time: i32,
    pub m_fertilizer_count: i32,
    pub m_bug_spray_count: i32,
    pub m_phonograph_count: i32,
    pub m_chocolate_count: i32,
    pub m_plant_health: i32,
    pub m_facing_left: bool,
    pub m_which_flower: i32,
    pub m_times_fertilized: i32,
    pub m_last_needy_time: i32,
    pub m_twice_watered: bool,
}

impl PottedPlant {
    pub fn new() -> Self {
        PottedPlant {
            m_potted_plant_index: 0,
            m_seed_type: SeedType::SEED_MARIGOLD,
            m_draw_variation: DrawVariation::VARIATION_NORMAL,
            m_age: 0,
            m_need: 0,
            m_watered: false,
            m_last_watered_time: -1,
            m_last_fertilized_time: -1,
            m_last_bug_sprayed_time: -1,
            m_last_phonograph_time: -1,
            m_last_chocolate_time: -1,
            m_fertilizer_count: 0,
            m_bug_spray_count: 0,
            m_phonograph_count: 0,
            m_chocolate_count: 0,
            m_plant_health: 0,
            m_facing_left: false,
            m_which_flower: 0,
            m_times_fertilized: 0,
            m_last_needy_time: 0,
            m_twice_watered: false,
        }
    }

    /// C++ PottedPlant::InitializePottedPlant (保真翻译)
    pub unsafe fn InitializePottedPlant(&mut self, _the_seed_type: SeedType) {
        // [TODO]: 盆栽植物初始化逻辑
        // C++ 中此方法设置物种、朝向、年龄、需求等
        // 当前保留结构，实际逻辑在后续翻译中补充
    }
}

#[derive(Clone)]
pub struct Coin {
    pub base: GameObject,
    pub m_pos_x: f32,
    pub m_pos_y: f32,
    pub m_vel_x: f32,
    pub m_vel_y: f32,
    pub m_scale: f32,
    pub m_dead: bool,
    pub m_fade_count: i32,
    pub m_collect_x: f32,
    pub m_collect_y: f32,
    pub m_ground_y: i32,
    pub m_coin_age: i32,
    pub m_is_being_collected: bool,
    pub m_disappear_counter: i32,
    pub m_type: CoinType,
    pub m_coin_motion: CoinMotion,
    pub m_attachment_id: AttachmentID,
    pub m_collection_distance: f32,
    pub m_usable_seed_type: SeedType,
    pub m_potted_plant_spec: PottedPlant,
    pub m_needs_bouncy_arrow: bool,
    pub m_has_bouncy_arrow: bool,
    pub m_hit_ground: bool,
    pub m_times_dropped: i32,
    pub m_width: i32,
    pub m_height: i32,
    pub m_render_order: i32,
}

// Helper: 获取全局 LawnApp
unsafe fn g_app() -> &'static mut crate::lawn_app::LawnApp {
    &mut *crate::lawn_app::G_LAWN_APP
}

impl Coin {
    pub fn new() -> Self {
        Coin {
            base: GameObject::new(),
            m_pos_x: 0.0,
            m_pos_y: 0.0,
            m_vel_x: 0.0,
            m_vel_y: 0.0,
            m_scale: 1.0,
            m_dead: false,
            m_fade_count: 0,
            m_collect_x: 0.0,
            m_collect_y: 0.0,
            m_ground_y: 0,
            m_coin_age: 0,
            m_is_being_collected: false,
            m_disappear_counter: 0,
            m_type: CoinType::COIN_NONE,
            m_coin_motion: CoinMotion::COIN_MOTION_FROM_SKY,
            m_attachment_id: AttachmentID::ATTACHMENTID_NULL,
            m_collection_distance: 0.0,
            m_usable_seed_type: SeedType::SEED_NONE,
            m_potted_plant_spec: PottedPlant::new(),
            m_needs_bouncy_arrow: false,
            m_has_bouncy_arrow: false,
            m_hit_ground: false,
            m_times_dropped: 0,
            m_width: 60,
            m_height: 60,
            m_render_order: 0,
        }
    }

    // =========================================================================
    // Coin 类型判断辅助函数 (C++ inline)
    // =========================================================================

    pub fn IsSun(&self) -> bool {
        self.m_type == CoinType::COIN_SUN || self.m_type == CoinType::COIN_LARGESUN
    }

    pub fn IsMoney(&self) -> bool {
        self.m_type == CoinType::COIN_SILVER
            || self.m_type == CoinType::COIN_GOLD
            || self.m_type == CoinType::COIN_DIAMOND
    }

    pub fn IsLevelAward(&self) -> bool {
        self.m_type == CoinType::COIN_FINAL_SEED_PACKET
            || self.m_type == CoinType::COIN_TROPHY
            || self.m_type == CoinType::COIN_AWARD_SILVER_SUNFLOWER
            || self.m_type == CoinType::COIN_AWARD_GOLD_SUNFLOWER
            || self.m_type == CoinType::COIN_SHOVEL
            || self.m_type == CoinType::COIN_CARKEYS
            || self.m_type == CoinType::COIN_ALMANAC
            || self.m_type == CoinType::COIN_VASE
            || self.m_type == CoinType::COIN_WATERING_CAN
            || self.m_type == CoinType::COIN_TACO
            || self.m_type == CoinType::COIN_NOTE
            || self.m_type == CoinType::COIN_AWARD_CHOCOLATE
            || self.m_type == CoinType::COIN_AWARD_MONEY_BAG
            || self.m_type == CoinType::COIN_AWARD_BAG_DIAMOND
            || self.m_type == CoinType::COIN_AWARD_PRESENT
            || self.m_type == CoinType::COIN_AWARD_PRESENT
            || self.m_type == CoinType::COIN_CHOCOLATE
            || self.m_type == CoinType::COIN_PRESENT_PLANT
    }

    pub fn IsPresentWithAdvice(&self) -> bool {
        self.m_type == CoinType::COIN_FINAL_SEED_PACKET
            || self.m_type == CoinType::COIN_SHOVEL
            || self.m_type == CoinType::COIN_CARKEYS
            || self.m_type == CoinType::COIN_ALMANAC
            || self.m_type == CoinType::COIN_TACO
            || self.m_type == CoinType::COIN_NOTE
            || self.m_type == CoinType::COIN_WATERING_CAN
            || self.m_type == CoinType::COIN_VASE
            || self.m_type == CoinType::COIN_AWARD_CHOCOLATE
            || self.m_type == CoinType::COIN_AWARD_MONEY_BAG
            || self.m_type == CoinType::COIN_AWARD_BAG_DIAMOND
            || self.m_type == CoinType::COIN_AWARD_PRESENT
            || self.m_type == CoinType::COIN_CHOCOLATE
            || self.m_type == CoinType::COIN_PRESENT_PLANT
    }

    // =========================================================================
    // CoinInitialize — C++ 保真翻译 (Coin.cpp:50)
    // =========================================================================
    pub unsafe fn CoinInitialize(&mut self, the_x: i32, the_y: i32, the_coin_type: CoinType, the_coin_motion: CoinMotion) {
        let app = g_app();

        // C++: mPosX = theX; mPosY = theY; (原始坐标)
        self.m_pos_x = the_x as f32;
        self.m_pos_y = the_y as f32;
        self.m_type = the_coin_type;
        self.m_collection_distance = 0.0;
        self.m_dead = false;
        self.m_width = 60;
        self.m_height = 60;
        self.m_disappear_counter = 0;
        self.m_is_being_collected = false;
        self.m_fade_count = 0;
        self.m_coin_motion = the_coin_motion;
        self.m_coin_age = 0;
        self.m_attachment_id = AttachmentID::ATTACHMENTID_NULL;
        self.m_render_order = crate::lawn::board::Board::MakeRenderOrder(RenderLayer::RENDER_LAYER_COIN_BANK, 0, 1);
        self.m_scale = 1.0;
        self.m_usable_seed_type = SeedType::SEED_NONE;
        self.m_needs_bouncy_arrow = false;
        self.m_has_bouncy_arrow = false;
        self.m_hit_ground = false;
        self.m_times_dropped = 0;
        self.m_potted_plant_spec.InitializePottedPlant(SeedType::SEED_NONE);
        self.m_ground_y = the_y;
        self.base.m_x = the_x;
        self.base.m_y = the_y;
        self.base.m_visible = true;

        // 初始化速度
        match the_coin_motion {
            CoinMotion::COIN_MOTION_FROM_SKY => {
                self.m_vel_x = crate::sexy_app_framework::common::rand_float(2.0) - 1.0;
                self.m_vel_y = -3.0;
                self.m_scale = 1.0;
            }
            CoinMotion::COIN_MOTION_FROM_PLANT => {
                self.m_vel_x = crate::sexy_app_framework::common::rand_float(2.0) + 1.0;
                self.m_vel_y = -4.0 - crate::sexy_app_framework::common::rand_float(3.0);
                self.m_scale = 0.75;
            }
            CoinMotion::COIN_MOTION_COIN => {
                self.m_vel_x = crate::sexy_app_framework::common::rand_float(4.0) - 2.0;
                self.m_vel_y = -7.0 - crate::sexy_app_framework::common::rand_float(3.0);
            }
            _ => {}
        }

        // C++: 创建阳光动画附件 (if IsSun())
        if self.IsSun() {
            let a_pos_x = self.m_width as f32 * 0.5;
            let a_pos_y = self.m_height as f32 * 0.5;
            let a_sun_reanim = app.AddReanimation(0.0, 0.0, 0, ReanimationType::REANIM_SUN);
            if !a_sun_reanim.is_null() {
                // [TRANSLATION_NOTE]: 使用 unsafe 裸指针操作 Reanimation 结构体
                // C++: aSunReanim->SetPosition / mLoopType / mAnimRate
                // [TODO]: 设置重动画属性
                // AttachReanim 将动画附加到金币上
                // AttachReanim(self.m_attachment_id, a_sun_reanim, a_pos_x, a_pos_y);
            }
        }
        // C++: COIN_SILVER
        else if self.m_type == CoinType::COIN_SILVER {
            self.m_pos_x -= 10.0;
            self.m_pos_y -= 8.0;
            let a_pos_x = 9.0;
            let a_pos_y = 9.0;
            let _a_coin_reanim = app.AddReanimation(0.0, 0.0, 0, ReanimationType::REANIM_COIN_SILVER);
            // [TODO]: 设置金币动画属性
        }
        // C++: COIN_GOLD
        else if self.m_type == CoinType::COIN_GOLD {
            self.m_pos_x -= 10.0;
            self.m_pos_y -= 8.0;
            let a_pos_x = 9.0;
            let a_pos_y = 9.0;
            let _a_coin_reanim = app.AddReanimation(0.0, 0.0, 0, ReanimationType::REANIM_COIN_GOLD);
            // [TODO]: 设置金币动画属性
        }
        // C++: COIN_DIAMOND
        else if self.m_type == CoinType::COIN_DIAMOND {
            self.m_pos_x -= 15.0;
            self.m_pos_y -= 15.0;
            let _a_coin_reanim = app.AddReanimation(0.0, 0.0, 0, ReanimationType::REANIM_DIAMOND);
            // [TODO]: 设置钻石动画属性
        }

        // C++: 风暴夜关卡渲染层
        if app.IsStormyNightLevel() {
            self.m_render_order = crate::lawn::board::Board::MakeRenderOrder(RenderLayer::RENDER_LAYER_ABOVE_UI, 0, 0);
        }

        // C++: 各类特殊金币类型的宽高/渲染层设置
        if self.m_type == CoinType::COIN_FINAL_SEED_PACKET {
            // mWidth = IMAGE_SEEDS->GetCelWidth();
            // mHeight = IMAGE_SEEDS->GetCelHeight();
            self.m_render_order = crate::lawn::board::Board::MakeRenderOrder(RenderLayer::RENDER_LAYER_ABOVE_UI, 0, 0);
        } else if self.m_type == CoinType::COIN_TROPHY {
            self.m_render_order = crate::lawn::board::Board::MakeRenderOrder(RenderLayer::RENDER_LAYER_ABOVE_UI, 0, 0);
        } else if self.m_type == CoinType::COIN_AWARD_SILVER_SUNFLOWER || self.m_type == CoinType::COIN_AWARD_GOLD_SUNFLOWER {
            self.m_render_order = crate::lawn::board::Board::MakeRenderOrder(RenderLayer::RENDER_LAYER_ABOVE_UI, 0, 0);
        } else if self.m_type == CoinType::COIN_SHOVEL
            || self.m_type == CoinType::COIN_CARKEYS
            || self.m_type == CoinType::COIN_ALMANAC
            || self.m_type == CoinType::COIN_VASE
            || self.m_type == CoinType::COIN_WATERING_CAN
            || self.m_type == CoinType::COIN_TACO
            || self.m_type == CoinType::COIN_NOTE
        {
            self.m_render_order = crate::lawn::board::Board::MakeRenderOrder(RenderLayer::RENDER_LAYER_ABOVE_UI, 0, 0);
        }
        // C++ 中还有更多类型... (见 Coin.cpp:190-470)
    }

    // =========================================================================
    // UpdateFade — C++ 保真翻译 (Coin.cpp:472)
    // =========================================================================
    pub unsafe fn UpdateFade(&mut self) {
        let app = g_app();
        // C++: if (IsEndlessIZombie || IsEndlessScaryPotter || mType == NOTE || !IsLevelAward())
        let is_endless_izombie = app.mGameMode as i32 == GameMode::GAMEMODE_CHALLENGE_PUZZLE_I_ZOMBIE_ENDLESS as i32;
        if is_endless_izombie
            || app.IsEndlessScaryPotter(app.mGameMode)
            || self.m_type == CoinType::COIN_NOTE
            || !self.IsLevelAward()
        {
            self.m_fade_count -= 1;
            if self.m_fade_count == 0 {
                self.Die();
            }
        }
    }

    // =========================================================================
    // GetColor — C++ 保真翻译 (Coin.cpp:777)
    // =========================================================================
    pub unsafe fn GetColor(&self) -> crate::sexy_app_framework::graphics::color::Color {
        use crate::sexy_app_framework::graphics::color::Color;
        use crate::sexy_tod_lib::tod_common::{tod_animate_curve_float, clamp_float};

        // C++: if ((IsSun() || IsMoney()) && mIsBeingCollected)
        if (self.IsSun() || self.IsMoney()) && self.m_is_being_collected {
            let a_alpha = clamp_float(
                self.m_collection_distance * 0.035,
                0.35,
                1.0,
            ) * 255.0;
            return Color::from_components_alpha(255, 255, 255, a_alpha as i32);
        }

        // C++: if (mFadeCount > 0)
        if self.m_fade_count > 0 {
            let a_alpha = tod_animate_curve_float(
                15,
                0,
                self.m_fade_count,
                255.0,
                0.0,
                TodCurves::CURVE_LINEAR,
            );
            return Color::from_components_alpha(255, 255, 255, a_alpha as i32);
        }

        Color::from_components_alpha(255, 255, 255, 255)
    }

    // =========================================================================
    // GetFinalSeedPacketType — C++ 保真翻译 (Coin.cpp:794)
    // =========================================================================
    pub unsafe fn GetFinalSeedPacketType(&self) -> SeedType {
        let app = g_app();
        if app.IsFirstTimeAdventureMode() {
            if let Some(ref board) = app.m_board {
                if board.mLevel <= 50 {
                    // C++: return mApp->GetAwardSeedForLevel(mBoard->mLevel);
                    // [TODO]: GetAwardSeedForLevel 尚未实现
                    return SeedType::SEED_NONE;
                }
            }
        }
        SeedType::SEED_NONE
    }

    // =========================================================================
    // UpdateFall — C++ 保真翻译 (Coin.cpp:484)
    // =========================================================================
    pub unsafe fn UpdateFall(&mut self) {
        // C++: COIN_MOTION_FROM_PRESENT
        if self.m_coin_motion == CoinMotion::COIN_MOTION_FROM_PRESENT {
            self.m_pos_x += self.m_vel_x;
            self.m_pos_y += self.m_vel_y;
            self.m_vel_x *= 0.95;
            self.m_vel_y *= 0.95;
            if self.m_coin_age >= 80 {
                self.Collect(self.m_collect_x, self.m_collect_y);
            }
            return;
        }

        // C++: 从弹跳位置下落
        if self.m_pos_y + self.m_vel_y < self.m_ground_y as f32 {
            self.m_pos_y += self.m_vel_y;
            match self.m_coin_motion {
                CoinMotion::COIN_MOTION_FROM_PLANT => {
                    self.m_vel_y += 0.09;
                }
                CoinMotion::COIN_MOTION_COIN | CoinMotion::COIN_MOTION_FROM_BOSS => {
                    self.m_vel_y += 0.15;
                }
                _ => {
                    self.m_vel_y += 0.3;
                }
            }

            self.m_pos_x += self.m_vel_x;
            let BOARD_WIDTH = 800.0; // [TODO]: 从 Board 获取
            if self.m_pos_x > BOARD_WIDTH - self.m_width as f32 && self.m_coin_motion != CoinMotion::COIN_MOTION_FROM_BOSS {
                self.m_pos_x = BOARD_WIDTH - self.m_width as f32;
                self.m_vel_x = -0.4 - crate::sexy_app_framework::common::rand_float(0.4);
            } else if self.m_pos_x < 0.0 {
                self.m_pos_x = 0.0;
                self.m_vel_x = 0.4 + crate::sexy_app_framework::common::rand_float(0.4);
            }
        } else {
            // C++: 落地弹跳
            if self.m_needs_bouncy_arrow && !self.m_has_bouncy_arrow {
                // [TODO]: 创建弹跳箭头粒子效果
            }

            self.m_pos_y = self.m_ground_y as f32;
            if self.m_vel_y > 0.5 {
                self.m_vel_y = -(self.m_vel_y * 0.5);
                self.m_times_dropped += 1;
            } else if self.m_vel_y > 0.0 {
                self.m_vel_y = 0.0;
                self.m_hit_ground = true;
            }
            if self.m_times_dropped >= 4 {
                self.m_hit_ground = true;
            }
        }
    }

    // =========================================================================
    // UpdateCollected — C++ 保真翻译
    // =========================================================================
    pub unsafe fn UpdateCollected(&mut self) {
        let dx = self.m_collect_x - self.m_pos_x;
        let dy = self.m_collect_y - self.m_pos_y;
        self.m_pos_x += dx * 0.1;
        self.m_pos_y += dy * 0.1;
        self.m_collection_distance = (dx * dx + dy * dy).sqrt();
        self.m_fade_count += 1;
        if self.m_fade_count > 50 {
            self.m_dead = true;
        }
    }

    // =========================================================================
    // Update — C++ 保真翻译 (Coin.cpp:735)
    // =========================================================================
    pub unsafe fn Update(&mut self) {
        if self.m_dead {
            return;
        }

        let app = g_app();

        // C++: 场景检查 — 只在 PLAYING/AWARD/upsell 场景中更新
        if app.mGameScene != GameScenes::SCENE_PLAYING
            && app.mGameScene != GameScenes::SCENE_AWARD
            && {
                // mBoard && !mBoard->mCutScene->ShouldRunUpsellBoard()
                // [TODO]: 检查 cutscene upsell
                true
            }
        {
            return;
        }

        self.m_coin_age += 1;

        // C++: 三级调度 — Fade / Fall / Collected
        if self.m_fade_count != 0 {
            self.UpdateFade();
        } else if !self.m_is_being_collected {
            self.UpdateFall();
        } else {
            self.UpdateCollected();
        }

        // C++: 附件更新
        if self.m_attachment_id != AttachmentID::ATTACHMENTID_NULL {
            let mut a_offset_x = 0.0;
            let mut a_offset_y = 0.0;
            if self.m_type == CoinType::COIN_DIAMOND {
                a_offset_x = 18.0 - 18.0 * self.m_scale;
                a_offset_y = 13.0 - 13.0 * self.m_scale;
            }

            // C++: AttachmentUpdateAndMove / AttachmentOverrideColor / AttachmentOverrideScale
            // _AttachmentUpdateAndMove(self.m_attachment_id, self.m_pos_x + a_offset_x, self.m_pos_y + a_offset_y);
            // _AttachmentOverrideColor(self.m_attachment_id, self.GetColor());
            // _AttachmentOverrideScale(self.m_attachment_id, self.m_scale);

            // C++: 运动中的银币/金币隐藏附件颜色
            if (!self.m_hit_ground || self.m_is_being_collected)
                && (self.m_type == CoinType::COIN_SILVER || self.m_type == CoinType::COIN_GOLD)
            {
                // AttachmentOverrideColor(self.m_attachment_id, Color(0, 0, 0, 0));
            }
        }

        self.base.m_x = self.m_pos_x as i32;
        self.base.m_y = self.m_pos_y as i32;
    }

    // =========================================================================
    // Draw — C++ 保真翻译 (Coin.cpp:804)
    // =========================================================================
    pub unsafe fn Draw(&self, g: &mut crate::sexy_app_framework::graphics::graphics::Graphics) {
        if self.m_dead {
            return;
        }

        // C++: g->SetColor(GetColor());
        g.SetColor(self.GetColor());

        // C++: 钻石发光效果
        if self.m_type == CoinType::COIN_DIAMOND {
            g.SetColorizeImages(true);
            // g->DrawImage(IMAGE_AWARDPICKUPGLOW, mPosX - 56, mPosY - 66);
            g.SetColorizeImages(false);
        }

        // C++: 礼物植物发光
        if self.m_type == CoinType::COIN_PRESENT_PLANT {
            g.SetColorizeImages(true);
            // g->DrawImage(IMAGE_AWARDPICKUPGLOW, mPosX - 50, mPosY - 64);
            g.SetColorizeImages(false);
        }

        // C++: 颁奖礼物收集时发光
        if self.m_type == CoinType::COIN_AWARD_PRESENT && self.m_is_being_collected {
            g.SetColorizeImages(true);
            // g->DrawImage(IMAGE_AWARDPICKUPGLOW, mPosX - 50, mPosY - 64);
            g.SetColorizeImages(false);
        }

        // C++: 巧克力发光
        if self.m_type == CoinType::COIN_CHOCOLATE || self.m_type == CoinType::COIN_AWARD_CHOCOLATE {
            g.SetColorizeImages(true);
            // g->DrawImage(IMAGE_AWARDPICKUPGLOW, mPosX - 56, mPosY - 50);
            g.SetColorizeImages(false);
        }

        // C++: 附件绘制
        if self.m_attachment_id != AttachmentID::ATTACHMENTID_NULL {
            // Graphics theAttachmentGraphics(*g);
            // MakeParentGraphicsFrame(&theAttachmentGraphics);
            // AttachmentDraw(mAttachmentID, &theAttachmentGraphics, false);
            // [TODO]: 附件绘制
        }

        // C++: 落地静止的银币/金币隐藏自身（附件已绘制）
        if (self.m_type == CoinType::COIN_SILVER || self.m_type == CoinType::COIN_GOLD)
            && self.m_hit_ground
            && !self.m_is_being_collected
        {
            return;
        }

        // C++: 钻石隐藏自身
        if self.m_type == CoinType::COIN_DIAMOND {
            return;
        }

        // C++: 关卡奖励闪烁
        if self.IsLevelAward() && !self.m_is_being_collected {
            // Color aFlashingColor = GetFlashingColor(mCoinAge, 75);
            // g->SetColor(aFlashingColor);
            // [TODO]: 闪烁效果
        }

        // C++: 银币/金币贴图
        if self.m_type == CoinType::COIN_SILVER || self.m_type == CoinType::COIN_GOLD {
            // [TODO]: DrawImage(IMAGE_COIN, mPosX, mPosY)
        }

        // [TODO]: 其他类型绘制
        // - COIN_SUN / COIN_LARGESUN
        // - COIN_FINAL_SEED_PACKET (种子包)
        // - COIN_TROPHY, COIN_SHOVEL, COIN_CARKEYS, COIN_ALMANAC 等
    }

    // =========================================================================
    // Collect / Die / MouseDown — 保真翻译
    // =========================================================================
    pub unsafe fn Collect(&mut self, the_collect_x: f32, the_collect_y: f32) {
        self.m_is_being_collected = true;
        self.m_collect_x = the_collect_x;
        self.m_collect_y = the_collect_y;
        self.m_fade_count = 0;
        self.m_collection_distance = 0.0;
    }

    pub unsafe fn Die(&mut self) {
        self.m_dead = true;
        // [TODO]: Remove attachment if any
    }

    pub unsafe fn MouseDown(&mut self, _x: i32, _y: i32, _click_count: i32) {
        if self.m_dead {
            return;
        }
        // C++: 点击金币开始收集
        // [TODO]: 检查鼠标命中
        self.m_is_being_collected = true;
    }

    // =========================================================================
    // 静态辅助函数
    // =========================================================================
    pub fn GetCoinValue(the_coin_type: CoinType) -> i32 {
        match the_coin_type {
            CoinType::COIN_SUN => 25,
            CoinType::COIN_LARGESUN => 50,
            CoinType::COIN_SILVER => 10,
            CoinType::COIN_GOLD => 100,
            CoinType::COIN_DIAMOND => 1000,
            CoinType::COIN_FINAL_SEED_PACKET => 100,
            _ => 0,
        }
    }
}

impl Default for Coin {
    fn default() -> Self {
        Self::new()
    }
}

// [TRANSLATION_NOTE]: ZenGarden.h + ZenGarden.cpp -> Rust 翻译
// 禅境花园系统 — 结构体和接口定义

#![allow(non_snake_case, dead_code)]

use crate::const_enums::*;
use crate::lawn_app::LawnApp;
use crate::lawn::board::Board;
use crate::lawn::plant::Plant;
use crate::lawn::grid_item::GridItem;
use crate::lawn::system::player_info::{PottedPlant, PottedPlantAge, PottedPlantNeed};
use crate::sexy_app_framework::graphics::graphics::Graphics;

pub const ZEN_MAX_GRIDSIZE_X: i32 = 8;
pub const ZEN_MAX_GRIDSIZE_Y: i32 = 4;
pub const STINKY_SLEEP_POS_Y: f32 = 461.0;

#[derive(Clone)]
pub struct SpecialGridPlacement {
    pub mPixelX: i32,
    pub mPixelY: i32,
    pub mGridX: i32,
    pub mGridY: i32,
}

pub struct ZenGarden {
    pub mApp: *mut LawnApp,
    pub mBoard: *mut Board,
    pub mGardenType: GardenType,
    pub mLoadedResourceNames: Vec<String>,
    pub mNowTime: i64,
}

impl ZenGarden {
    pub fn new() -> Self {
        ZenGarden {
            mApp: std::ptr::null_mut(),
            mBoard: std::ptr::null_mut(),
            mGardenType: GardenType::GARDEN_MAIN,
            mLoadedResourceNames: Vec::new(),
            mNowTime: 0,
        }
    }

        /// C++ ZenGarden::PottedPlantFromIndex (ZenGarden.cpp:278)
    pub fn PottedPlantFromIndex(&self, the_potted_plant_index: i32) -> *mut PottedPlant {
        unsafe {
            if self.mApp.is_null() || (*self.mApp).m_player_info.is_null() {
                return std::ptr::null_mut();
            }
            let the_player_info = &mut *(*self.mApp).m_player_info;
            if (the_potted_plant_index as usize) < the_player_info.mPottedPlant.len() {
                the_player_info.mPottedPlant.as_mut_ptr().add(the_potted_plant_index as usize)
            } else {
                std::ptr::null_mut()
            }
        }
    }

    /// C++ ZenGarden::PlantGetMinutesSinceHappy — 植物快乐分钟数
    pub fn PlantGetMinutesSinceHappy(&self, the_plant: *mut Plant) -> i32 {
        // [TODO]: 基于 mLastWateredTime/当前时间的完整计算
        let _ = the_plant;
        0
    }

    /// C++ ZenGarden::GetPlantSellPrice (ZenGarden.cpp:405) — 植物出售价格
    pub fn GetPlantSellPrice(&self, the_plant: *mut Plant) -> i32 {
        unsafe {
            if the_plant.is_null() {
                return 0;
            }
            let a_potted_plant = self.PottedPlantFromIndex((*the_plant).m_potted_plant_index);
            if a_potted_plant.is_null() {
                return 0;
            }

            if (*a_potted_plant).mSeedType == SeedType::SEED_MARIGOLD {
                match (*a_potted_plant).mPlantAge {
                    PottedPlantAge::PLANTAGE_SPROUT => return 150,
                    PottedPlantAge::PLANTAGE_SMALL => return 200,
                    PottedPlantAge::PLANTAGE_MEDIUM => return 250,
                    PottedPlantAge::PLANTAGE_FULL => return 300,
                    _ => return 0,
                }
            }

            match (*a_potted_plant).mPlantAge {
                PottedPlantAge::PLANTAGE_SPROUT => 150,
                PottedPlantAge::PLANTAGE_SMALL => 300,
                PottedPlantAge::PLANTAGE_MEDIUM => 500,
                PottedPlantAge::PLANTAGE_FULL => {
                    // C++: 夜间/水生植物售价更高
                    if crate::lawn::plant::Plant::is_nocturnal((*a_potted_plant).mSeedType)
                        || crate::lawn::plant::Plant::is_aquatic((*a_potted_plant).mSeedType)
                    {
                        1000
                    } else {
                        800
                    }
                }
                _ => 0,
            }
        }
    }

    /// C++ ZenGarden::WasPlantNeedFulfilledToday (ZenGarden.cpp:756) — 今天是否满足需求
    pub fn WasPlantNeedFulfilledToday(&self, the_potted_plant: *mut PottedPlant) -> bool {
        unsafe {
            if the_potted_plant.is_null() {
                return false;
            }
            // C++: int64 aNow = mNowTime; aNow - mLastNeedFulfilledTime < 3600
            let a_now = self.mNowTime;
            if a_now - (*the_potted_plant).mLastNeedFulfilledTime < 3600 {
                return true;
            }
            (*the_potted_plant).mPlantNeed == PottedPlantNeed::PLANTNEED_NONE
        }
    }



    /// C++ ZenGarden::PlantSetLaunchCounter (ZenGarden.cpp:192)
    pub fn PlantSetLaunchCounter(&self, the_plant: *mut Plant) {
        // C++: int aTime = PlantGetMinutesSinceHappy(thePlant);
        let a_time = self.PlantGetMinutesSinceHappy(the_plant);
        // C++: aCounterMax = TodAnimateCurve(5, 30, aTime, 3000, 15000, CURVE_LINEAR)
        let a_counter_max = crate::sexy_tod_lib::tod_common::tod_animate_curve(5, 30, a_time, 3000, 15000, crate::const_enums::TodCurves::CURVE_LINEAR);
        // C++: thePlant->mLaunchCounter = RandRangeInt(1800, aCounterMax)
        unsafe {
            if !the_plant.is_null() {
                (*the_plant).m_launch_counter = crate::sexy_tod_lib::tod_common::rand_range_int(1800, a_counter_max);
            }
        }
    }

    /// C++ ZenGarden::PlantCanHaveChocolate (ZenGarden.cpp:308)
    pub fn PlantCanHaveChocolate(&self, the_plant: *mut Plant) -> bool {
        unsafe {
            if the_plant.is_null() {
                return false;
            }
            let a_potted_plant = self.PottedPlantFromIndex((*the_plant).m_potted_plant_index);
            if a_potted_plant.is_null() {
                return false;
            }
            (*a_potted_plant).mPlantAge == PottedPlantAge::PLANTAGE_FULL
                && self.WasPlantNeedFulfilledToday(a_potted_plant)
                && !self.PlantHighOnChocolate(a_potted_plant)
        }
    }

    /// C++ ZenGarden::CanDropChocolate (ZenGarden.cpp:314)
    pub fn CanDropChocolate(&self) -> bool {
        // C++: HasPurchasedStinky() && mPurchases[STORE_ITEM_CHOCOLATE] < PURCHASE_COUNT_OFFSET + 10
        unsafe {
            if self.mApp.is_null() {
                return false;
            }
            (*self.mApp).HasPurchasedStinky()
                && if (*self.mApp).m_player_info.is_null() {
                    false
                } else {
                    (*(*self.mApp).m_player_info).mPurchases[StoreItem::STORE_ITEM_CHOCOLATE as usize]
                        < (crate::lawn::system::player_info::PURCHASE_COUNT_OFFSET + 10) as u32
                }
        }
    }

    /// C++ ZenGarden::CanDropPottedPlantLoot (ZenGarden.cpp:341)
    pub fn CanDropPottedPlantLoot(&self) -> bool {
        unsafe {
            !self.mApp.is_null()
                && (*self.mApp).HasFinishedAdventure()
                && !self.IsZenGardenFull(true)
        }
    }
    /// C++ ZenGarden::PlantFertilized (ZenGarden.cpp:518) — 植物施肥
    pub fn PlantFertilized(&mut self, the_plant: *mut Plant) {
        unsafe {
            if the_plant.is_null() {
                return;
            }
            let a_potted_plant = self.PottedPlantFromIndex((*the_plant).m_potted_plant_index);
            if a_potted_plant.is_null() {
                return;
            }
            (*a_potted_plant).mLastFertilizedTime = self.mNowTime;
            (*a_potted_plant).mPlantAge = std::mem::transmute::<i32, PottedPlantAge>((*a_potted_plant).mPlantAge as i32 + 1);
            (*a_potted_plant).mPlantNeed = PottedPlantNeed::PLANTNEED_NONE;
            (*a_potted_plant).mTimesFed = 0;

            if (*a_potted_plant).mPlantAge == PottedPlantAge::PLANTAGE_SMALL {
                // [TODO]: RemovePottedPlant + PlacePottedPlant
            } else {
                (*the_plant).m_state_countdown = 100;
                // [TODO]: mApp->PlayFoley(FOLEY_PLANTGROW)
            }

            let the_board = &mut *self.mBoard;
            let a_plant_x = (*the_plant).base.m_x;
            let a_plant_y = (*the_plant).base.m_y;
            if (*a_potted_plant).mPlantAge == PottedPlantAge::PLANTAGE_SMALL {
                the_board.AddCoin(a_plant_x + 40, a_plant_y, CoinType::COIN_GOLD, CoinMotion::COIN_MOTION_COIN);
            } else if (*a_potted_plant).mPlantAge == PottedPlantAge::PLANTAGE_MEDIUM {
                the_board.AddCoin(a_plant_x + 30, a_plant_y, CoinType::COIN_GOLD, CoinMotion::COIN_MOTION_COIN);
                the_board.AddCoin(a_plant_x + 50, a_plant_y, CoinType::COIN_GOLD, CoinMotion::COIN_MOTION_COIN);
            } else if (*a_potted_plant).mPlantAge == PottedPlantAge::PLANTAGE_FULL {
                if (*a_potted_plant).mSeedType == SeedType::SEED_MARIGOLD {
                    the_board.AddCoin(a_plant_x + 40, a_plant_y, CoinType::COIN_DIAMOND, CoinMotion::COIN_MOTION_COIN);
                } else {
                    the_board.AddCoin(a_plant_x + 10, a_plant_y, CoinType::COIN_DIAMOND, CoinMotion::COIN_MOTION_COIN);
                    the_board.AddCoin(a_plant_x + 70, a_plant_y, CoinType::COIN_DIAMOND, CoinMotion::COIN_MOTION_COIN);
                }
            }
        }
    }

    /// C++ ZenGarden::PlantFulfillNeed (ZenGarden.cpp:562) — 满足植物需求
    pub fn PlantFulfillNeed(&mut self, the_plant: *mut Plant) {
        unsafe {
            if the_plant.is_null() {
                return;
            }
            let a_potted_plant = self.PottedPlantFromIndex((*the_plant).m_potted_plant_index);
            if a_potted_plant.is_null() {
                return;
            }
            (*a_potted_plant).mLastNeedFulfilledTime = self.mNowTime;
            (*a_potted_plant).mPlantNeed = PottedPlantNeed::PLANTNEED_NONE;
            (*a_potted_plant).mTimesFed = 0;

            let the_board = &mut *self.mBoard;
            let a_plant_x = (*the_plant).base.m_x;
            let a_plant_y = (*the_plant).base.m_y;
            the_board.AddCoin(a_plant_x + 40, a_plant_y, CoinType::COIN_GOLD, CoinMotion::COIN_MOTION_COIN);
            if crate::lawn::plant::Plant::is_nocturnal((*the_plant).m_seed_type)
                || crate::lawn::plant::Plant::is_aquatic((*the_plant).m_seed_type)
            {
                the_board.AddCoin(a_plant_x + 10, a_plant_y, CoinType::COIN_GOLD, CoinMotion::COIN_MOTION_COIN);
                the_board.AddCoin(a_plant_x + 70, a_plant_y, CoinType::COIN_GOLD, CoinMotion::COIN_MOTION_COIN);
            }
        }
    }
    /// C++ ZenGarden::UpdatePlantState (ZenGarden.cpp:655) — 植物状态更新
    pub unsafe fn UpdatePlantState(&mut self, the_plant: *mut Plant) {
        unsafe {
            if the_plant.is_null() {
                return;
            }
            let a_original_state = (*the_plant).m_state;
            let a_potted_plant = self.PottedPlantFromIndex((*the_plant).m_potted_plant_index);
            if a_potted_plant.is_null() {
                return;
            }

            let a_plant_need = self.GetPlantsNeed(a_potted_plant);
            if a_plant_need == PottedPlantNeed::PLANTNEED_WATER {
                (*the_plant).m_state = crate::lawn::plant::PlantState::STATE_NOTREADY;
            } else if a_plant_need == PottedPlantNeed::PLANTNEED_NONE {
                if self.WasPlantNeedFulfilledToday(a_potted_plant) {
                    (*the_plant).m_state = crate::lawn::plant::PlantState::STATE_ZEN_GARDEN_HAPPY;
                } else if (*the_plant).m_is_asleep {
                    (*the_plant).m_state = crate::lawn::plant::PlantState::STATE_NOTREADY;
                } else {
                    (*the_plant).m_state = crate::lawn::plant::PlantState::STATE_ZEN_GARDEN_WATERED;
                }
            } else {
                (*the_plant).m_state = crate::lawn::plant::PlantState::STATE_ZEN_GARDEN_NEEDY;
            }

            if a_original_state == (*the_plant).m_state {
                return;
            }

            // [TODO]: Reanimation SetImageOverride("Pot_top")

            if a_original_state == crate::lawn::plant::PlantState::STATE_ZEN_GARDEN_HAPPY {
                self.RemoveHappyEffect(the_plant);
            }
            if (*the_plant).m_state == crate::lawn::plant::PlantState::STATE_ZEN_GARDEN_HAPPY {
                (*the_plant).SetSleeping(false);
                self.AddHappyEffect(the_plant);
            } else if crate::lawn::plant::Plant::is_nocturnal((*the_plant).m_seed_type) && !(*self.mBoard).StageIsNight() {
                (*the_plant).SetSleeping(true);
            }
        }
    }

    /// C++ ZenGarden::AddHappyEffect (ZenGarden.cpp:721) — 添加快乐发光
    pub unsafe fn AddHappyEffect(&mut self, the_plant: *mut Plant) {
        unsafe {
            if the_plant.is_null() {
                return;
            }
            let the_board = &*self.mBoard;
            let a_flower_pot = the_board.GetTopPlantAt((*the_plant).m_plant_col, (*the_plant).base.m_row, PlantPriority::TOPPLANT_ONLY_UNDER_PLANT);
            if a_flower_pot.is_null() {
                (*the_plant).AddAttachedParticle((*the_plant).base.m_x + 40, (*the_plant).base.m_y + 60, (*the_plant).base.m_render_order - 1, ParticleEffect::PARTICLE_POTTED_ZEN_GLOW);
            } else if crate::lawn::plant::Plant::is_aquatic((*the_plant).m_seed_type) {
                (*a_flower_pot).AddAttachedParticle((*a_flower_pot).base.m_x + 40, (*a_flower_pot).base.m_y + 61, (*a_flower_pot).base.m_render_order - 1, ParticleEffect::PARTICLE_POTTED_WATER_PLANT_GLOW);
            } else {
                (*a_flower_pot).AddAttachedParticle((*a_flower_pot).base.m_x + 40, (*a_flower_pot).base.m_y + 63, (*a_flower_pot).base.m_render_order - 1, ParticleEffect::PARTICLE_POTTED_ZEN_GLOW);
            }
        }
    }

    /// C++ ZenGarden::RemoveHappyEffect (ZenGarden.cpp:738) — 移除快乐发光
    pub unsafe fn RemoveHappyEffect(&mut self, the_plant: *mut Plant) {
        unsafe {
            if the_plant.is_null() {
                return;
            }
            let the_board = &*self.mBoard;
            let a_flower_pot = the_board.GetTopPlantAt((*the_plant).m_plant_col, (*the_plant).base.m_row, PlantPriority::TOPPLANT_ONLY_UNDER_PLANT);
            // [TODO]: 粒子销毁（ParticleTryToGet + ParticleSystemDie）
            let _ = a_flower_pot;
            let _ = (*the_plant).m_particle_id;
        }
    }
pub fn ZenGardenInitLevel(&mut self) {}
    pub fn DrawPottedPlantIcon(&self, _g: &mut Graphics, _x: f32, _y: f32, _thePottedPlant: *mut PottedPlant) {}
    pub fn DrawPottedPlant(&self, _g: &mut Graphics, _x: f32, _y: f32, _thePottedPlant: *mut PottedPlant, _theScale: f32, _theDrawPot: bool) {}
    pub fn IsZenGardenFull(&self, the_include_dropped_presents: bool) -> bool {
    // C++: 掉落礼物数量
    let mut a_num_dropped_presents = 0;
    if !self.mBoard.is_null() && the_include_dropped_presents {
        unsafe {
            let the_board = &*self.mBoard;
            a_num_dropped_presents += the_board.CountCoinByType(CoinType::COIN_AWARD_PRESENT);
            a_num_dropped_presents += the_board.CountCoinByType(CoinType::COIN_PRESENT_PLANT);
        }
    }

    // C++: 花园中盆栽数量
    let mut a_num_potted_plants_in_garden = 0;
    unsafe {
        if !self.mApp.is_null() && !(*self.mApp).m_player_info.is_null() {
            let the_player_info = &*(*self.mApp).m_player_info;
            for a_potted_plant in &the_player_info.mPottedPlant {
                if a_potted_plant.mWhichZenGarden == GardenType::GARDEN_MAIN {
                    a_num_potted_plants_in_garden += 1;
                }
            }
        }
    }

    a_num_dropped_presents + a_num_potted_plants_in_garden >= ZEN_MAX_GRIDSIZE_X * ZEN_MAX_GRIDSIZE_Y
}
    pub fn FindOpenZenGardenSpot(&self, the_spot_x: &mut i32, the_spot_y: &mut i32) {
    let mut a_picks: [crate::sexy_tod_lib::tod_common::TodWeightedGridArray; 32] = [crate::sexy_tod_lib::tod_common::TodWeightedGridArray { m_x: 0, m_y: 0, m_weight: 0 }; 32];
    let mut a_pick_count = 0;

    unsafe {
        let mut x = 0;
        while x < ZEN_MAX_GRIDSIZE_X {
            let mut y = 0;
            while y < ZEN_MAX_GRIDSIZE_Y {
                // C++: 戴夫遮挡区域（mCrazyDaveMessageIndex != -1 时 x<2 或 y<1）
                if (*self.mApp).m_crazy_dave_message_index != -1 && (x < 2 || y < 1) {
                    y += 1;
                    continue;
                }

                // C++: 格子已被盆栽占用则跳过
                let mut a_occupied = false;
                if !(*self.mApp).m_player_info.is_null() {
                    let the_player_info = &*(*self.mApp).m_player_info;
                    for a_potted_plant in &the_player_info.mPottedPlant {
                        if a_potted_plant.mWhichZenGarden == GardenType::GARDEN_MAIN
                            && a_potted_plant.mX == x
                            && a_potted_plant.mY == y
                        {
                            a_occupied = true;
                            break;
                        }
                    }
                }
                if !a_occupied {
                    a_picks[a_pick_count as usize].m_x = x;
                    a_picks[a_pick_count as usize].m_y = y;
                    a_picks[a_pick_count as usize].m_weight = 1;
                    a_pick_count += 1;
                }
                y += 1;
            }
            x += 1;
        }
    }

    let a_spot: *mut crate::sexy_tod_lib::tod_common::TodWeightedGridArray = match crate::sexy_tod_lib::tod_common::tod_pick_from_weighted_grid_array(&mut a_picks) {
        Some(g) => g,
        None => return,
    };
    unsafe {
        *the_spot_x = (*a_spot).m_x;
        *the_spot_y = (*a_spot).m_y;
    }
}
    pub fn AddPottedPlant(&mut self, the_potted_plant: *mut PottedPlant) {
    // C++: mPottedPlant[mNumPottedPlants] = *thePottedPlant; mNumPottedPlants++;
    unsafe {
        if the_potted_plant.is_null() || (*self.mApp).m_player_info.is_null() {
            return;
        }
        let the_player_info = &mut *(*self.mApp).m_player_info;
        let a_num = the_player_info.mNumPottedPlants as usize;
        if a_num < the_player_info.mPottedPlant.len() {
            the_player_info.mPottedPlant[a_num] = (*the_potted_plant).clone();
            the_player_info.mNumPottedPlants += 1;
        }
    }
}
    pub fn MovePlant(&mut self, _thePlant: *mut Plant, _theGridX: i32, _theGridY: i32) {}
    pub fn MouseDownWithMoneySign(&mut self, _thePlant: *mut Plant) {}
    pub fn PlacePottedPlant(&mut self, _thePottedPlantIndex: isize) -> *mut Plant { std::ptr::null_mut() }
    pub fn PlantPottedDrawHeightOffset(&self, _theSeedType: SeedType, _theScale: f32) -> f32 { 0.0 }
    pub fn ZenPlantOffsetX(_thePottedPlant: *mut PottedPlant) -> f32 { 0.0 }
    /// C++ ZenGarden::ZenGardenUpdate (ZenGarden.cpp:1724) — 禅境花园主循环
    pub unsafe fn ZenGardenUpdate(&mut self) {
        unsafe {
            // C++: if (mApp->GetDialog(DIALOG_STORE)) return;
            // [TODO]: GetDialog 检查

            // C++: mNowTime = mApp->GetNowTime();
            self.mNowTime = crate::sexy_app_framework::sexy_app_base::sdl_get_ticks() as i64;
            // C++: mApp->UpdateCrazyDave();
            (*self.mApp).UpdateCrazyDave();

            let the_board = &mut *self.mBoard;
            if !the_board.mCursorObject.is_null()
                && (*the_board.mCursorObject).mCursorType != CursorType::CURSOR_TYPE_NORMAL
            {
                if !the_board.mChallenge.is_null() {
                    (*the_board.mChallenge).mChallengeState = ChallengeState::STATECHALLENGE_NORMAL;
                    (*the_board.mChallenge).mChallengeStateCounter = 3000;
                }
            } else if the_board.mTutorialState == TutorialState::TUTORIAL_OFF as i32 {
                if !the_board.mChallenge.is_null() {
                    let a_challenge = &mut *the_board.mChallenge;
                    if a_challenge.mChallengeStateCounter > 0 {
                        a_challenge.mChallengeStateCounter -= 1;
                    }
                    if a_challenge.mChallengeState == ChallengeState::STATECHALLENGE_NORMAL && a_challenge.mChallengeStateCounter == 0 {
                        a_challenge.mChallengeState = ChallengeState::STATECHALLENGE_ZEN_FADING;
                        a_challenge.mChallengeStateCounter = 50;
                    }
                }
            }

            // C++: 更新植物需求 + 盆栽 + 工具 + 蜗牛
            self.UpdatePlantNeeds();
            let mut a_plant: *mut crate::lawn::plant::Plant = std::ptr::null_mut();
            while the_board.IteratePlants(&mut a_plant) {
                if (*a_plant).m_potted_plant_index != -1 {
                    self.PottedPlantUpdate(a_plant);
                }
            }
            let mut a_grid_item: *mut crate::lawn::grid_item::GridItem = std::ptr::null_mut();
            while the_board.IterateGridItems(&mut a_grid_item) {
                if (*a_grid_item).mGridItemType == GridItemType::GRIDITEM_ZEN_TOOL {
                    self.ZenToolUpdate(a_grid_item);
                } else if (*a_grid_item).mGridItemType == GridItemType::GRIDITEM_STINKY {
                    self.StinkyUpdate(a_grid_item);
                }
            }

            // C++: 教程推进：持续浇水 → 商店
            if the_board.mTutorialState == TutorialState::TUTORIAL_ZEN_GARDEN_KEEP_WATERING as i32
                && self.CountPlantsNeedingFertilizer() > 0
            {
                the_board.DisplayAdvice("[ADVICE_ZEN_GARDEN_VISIT_STORE]", 0, AdviceType::ADVICE_NONE as i32);
                the_board.mTutorialState = TutorialState::TUTORIAL_ZEN_GARDEN_VISIT_STORE as i32;
                if !the_board.mStoreButton.is_null() {
                    (*the_board.mStoreButton).mDisabled = false;
                    (*the_board.mStoreButton).mBtnNoDraw = false;
                }
            }
        }
    }

    /// C++ ZenGarden::UpdatePlantNeeds (ZenGarden.cpp:804)
    pub unsafe fn UpdatePlantNeeds(&mut self) {
        unsafe {
            if (*self.mApp).m_player_info.is_null() {
                return;
            }
            self.mNowTime = crate::sexy_app_framework::sexy_app_base::sdl_get_ticks() as i64;
            let a_num_potted_plants = (*(*self.mApp).m_player_info).mNumPottedPlants;
            let mut i = 0;
            while i < a_num_potted_plants as usize {
                let a_potted_plant = self.PottedPlantFromIndex(i as i32);
                if !a_potted_plant.is_null() {
                    self.RefreshPlantNeeds(a_potted_plant);
                }
                i += 1;
            }
        }
    }

    /// C++ ZenGarden::RefreshPlantNeeds (ZenGarden.cpp:783)
    pub unsafe fn RefreshPlantNeeds(&mut self, the_potted_plant: *mut PottedPlant) {
        unsafe {
            if the_potted_plant.is_null() {
                return;
            }
            // C++: 非满阶段或不需要刷新 → 跳过
            if (*the_potted_plant).mPlantAge != PottedPlantAge::PLANTAGE_FULL || !self.PlantShouldRefreshNeed(the_potted_plant) {
                return;
            }

            if crate::lawn::plant::Plant::is_aquatic((*the_potted_plant).mSeedType) {
                // C++: 水生：更新时间 + 随机需求（杀虫剂~留声机）
                (*the_potted_plant).mLastWateredTime = self.mNowTime;
                (*the_potted_plant).mPlantNeed = std::mem::transmute::<i32, PottedPlantNeed>(
                    crate::sexy_tod_lib::tod_common::rand_range_int(
                        PottedPlantNeed::PLANTNEED_BUGSPRAY as i32,
                        PottedPlantNeed::PLANTNEED_PHONOGRAPH as i32,
                    ),
                );
            } else {
                // C++: 普通：重置喂食次数 + 无需求（下次浇水时重新生成）
                (*the_potted_plant).mTimesFed = 0;
                (*the_potted_plant).mPlantNeed = PottedPlantNeed::PLANTNEED_NONE;
            }
        }
    }
    pub fn MouseDownWithFullWheelBarrow(&mut self, _x: i32, _y: i32) {}
    pub fn MouseDownWithEmptyWheelBarrow(&mut self, _thePlant: *mut Plant) {}
    pub fn GotoNextGarden(&mut self) {}
    pub fn GetPottedPlantInWheelbarrow(&self) -> *mut PottedPlant { std::ptr::null_mut() }
    pub fn RemovePottedPlant(&mut self, _thePlant: *mut Plant) {}
    pub fn GetSpecialGridPlacements(&self, _theCount: &mut i32) -> *const SpecialGridPlacement { std::ptr::null() }
    pub fn PixelToGridX(&self, _theX: i32, _theY: i32) -> i32 { 0 }
    pub fn PixelToGridY(&self, _theX: i32, _theY: i32) -> i32 { 0 }
    pub fn GridToPixelX(&self, _theGridX: i32, _theGridY: i32) -> i32 { 0 }
    pub fn GridToPixelY(&self, _theGridX: i32, _theGridY: i32) -> i32 { 0 }
    pub fn DrawBackdrop(&self, _g: &mut Graphics) {}
    pub fn MouseDownZenGarden(&mut self, _x: i32, _y: i32, _theClickCount: i32, _theHitResult: *mut std::ffi::c_void) -> bool { false }
    pub fn PlantWatered(&mut self, _thePlant: *mut Plant) {}

    pub fn IsPlantInGoldWateringCanRange(&self, _theX: i32, _theY: i32, _thePlant: *mut Plant) -> bool { false }
    /// C++ ZenGarden::WasPlantFertilizedInLastHour (ZenGarden.cpp:821)
    pub fn WasPlantFertilizedInLastHour(&self, the_potted_plant: *mut PottedPlant) -> bool {
        unsafe {
            if the_potted_plant.is_null() {
                return false;
            }
            // C++: mNowTime - mLastFertilizedTime < 3600
            self.mNowTime - (*the_potted_plant).mLastFertilizedTime < 3600
        }
    }

    /// C++ ZenGarden::PlantShouldRefreshNeed (ZenGarden.cpp:769) — 需求是否应刷新（跨天）
    pub fn PlantShouldRefreshNeed(&self, the_potted_plant: *mut PottedPlant) -> bool {
        unsafe {
            if the_potted_plant.is_null() {
                return false;
            }
            let a_now = self.mNowTime;
            // C++: 一小时内浇过水不刷新
            if a_now - (*the_potted_plant).mLastWateredTime < 3600 {
                return false;
            }
            // [TRANSLATION_NOTE]: 跨天判断（C++ 用 tm_year/tm_yday 比较）。
            // Rust 移植用 86400 秒（一天）近似，语义等价。
            a_now - (*the_potted_plant).mLastWateredTime >= 86400
        }
    }

    /// C++ ZenGarden::GetPlantsNeed (ZenGarden.cpp:826) — 植物当前需求
    pub fn GetPlantsNeed(&self, the_potted_plant: *mut PottedPlant) -> PottedPlantNeed {
        unsafe {
            if the_potted_plant.is_null() {
                return PottedPlantNeed::PLANTNEED_NONE;
            }
            // C++: 主花园夜间植物无需求
            if (*the_potted_plant).mPlantAge != PottedPlantAge::PLANTAGE_SPROUT
                && crate::lawn::plant::Plant::is_nocturnal((*the_potted_plant).mSeedType)
                && (*the_potted_plant).mWhichZenGarden == GardenType::GARDEN_MAIN
            {
                return PottedPlantNeed::PLANTNEED_NONE;
            }
            // C++: 独轮车中无需求
            if (*the_potted_plant).mWhichZenGarden == GardenType::GARDEN_WHEELBARROW {
                return PottedPlantNeed::PLANTNEED_NONE;
            }

            let a_now = self.mNowTime;
            let a_too_long_since_watering = a_now - (*the_potted_plant).mLastWateredTime > 15;
            let a_too_short_since_watering = a_now - (*the_potted_plant).mLastWateredTime < 3;

            // C++: 一小时内施过肥或需求已满足 → 无需求
            if self.WasPlantFertilizedInLastHour(the_potted_plant) || self.WasPlantNeedFulfilledToday(the_potted_plant) {
                return PottedPlantNeed::PLANTNEED_NONE;
            }
            // C++: 水生植物（非芽期）
            if crate::lawn::plant::Plant::is_aquatic((*the_potted_plant).mSeedType)
                && (*the_potted_plant).mPlantAge != PottedPlantAge::PLANTAGE_SPROUT
            {
                if (*the_potted_plant).mPlantAge == PottedPlantAge::PLANTAGE_FULL {
                    if self.PlantShouldRefreshNeed(the_potted_plant) {
                        return PottedPlantNeed::PLANTNEED_NONE;
                    }
                    return (*the_potted_plant).mPlantNeed;
                } else {
                    // C++: 非水族馆的水生植物无需求，水族馆需要化肥
                    if (*the_potted_plant).mWhichZenGarden != GardenType::GARDEN_AQUARIUM {
                        return PottedPlantNeed::PLANTNEED_NONE;
                    }
                    return PottedPlantNeed::PLANTNEED_FERTILIZER;
                }
            }
            // C++: 未超时未浇水 → 无需求
            if !a_too_long_since_watering {
                return PottedPlantNeed::PLANTNEED_NONE;
            }
            // C++: 喂食次数未满 → 需要水
            if (*the_potted_plant).mTimesFed < (*the_potted_plant).mFeedingsPerGrow {
                return PottedPlantNeed::PLANTNEED_WATER;
            }
            // C++: 浇水间隔过短 → 无需求
            if a_too_short_since_watering {
                return PottedPlantNeed::PLANTNEED_NONE;
            }
            // C++: 未满阶段 → 需要化肥
            if (*the_potted_plant).mPlantAge != PottedPlantAge::PLANTAGE_FULL {
                return PottedPlantNeed::PLANTNEED_FERTILIZER;
            }
            // C++: 跨天刷新 → 无需求
            if self.PlantShouldRefreshNeed(the_potted_plant) {
                return PottedPlantNeed::PLANTNEED_NONE;
            }
            // C++: 已有需求 → 返回
            if (*the_potted_plant).mPlantNeed != PottedPlantNeed::PLANTNEED_NONE {
                return (*the_potted_plant).mPlantNeed;
            }
            PottedPlantNeed::PLANTNEED_WATER
        }
    }
    /// C++ ZenGarden::MouseDownWithTool (ZenGarden.cpp:1068) — 工具点击
    pub unsafe fn MouseDownWithTool(&mut self, x: i32, y: i32, the_cursor_type: i32) {
        if the_cursor_type == CursorType::CURSOR_TYPE_WHEEELBARROW as i32 && !self.GetPottedPlantInWheelbarrow().is_null() {
            self.MouseDownWithFullWheelBarrow(x, y);
            unsafe { (*self.mBoard).ClearCursor(); }
            return;
        }

        if the_cursor_type == CursorType::CURSOR_TYPE_WATERING_CAN as i32
            || the_cursor_type == CursorType::CURSOR_TYPE_FERTILIZER as i32
            || the_cursor_type == CursorType::CURSOR_TYPE_BUG_SPRAY as i32
            || the_cursor_type == CursorType::CURSOR_TYPE_PHONOGRAPH as i32
            || the_cursor_type == CursorType::CURSOR_TYPE_CHOCOLATE as i32
        {
            self.MouseDownWithFeedingTool(x, y, the_cursor_type);
            return;
        }

        unsafe {
            let the_board = &*self.mBoard;
            let a_plant = the_board.ToolHitTest(x, y);
            if a_plant.is_null() || (*a_plant).m_potted_plant_index == -1 {
                // C++: mApp->PlayFoley(FOLEY_DROP)
                (*self.mBoard).ClearCursor();
                return;
            }

            if the_cursor_type == CursorType::CURSOR_TYPE_MONEY_SIGN as i32 {
                self.MouseDownWithMoneySign(a_plant);
            } else if the_cursor_type == CursorType::CURSOR_TYPE_WHEEELBARROW as i32 {
                self.MouseDownWithEmptyWheelBarrow(a_plant);
                (*self.mBoard).ClearCursor();
            } else if the_cursor_type == CursorType::CURSOR_TYPE_GLOVE as i32 {
                // C++: 手套拿起植物
                let board_mut = &mut *self.mBoard;
                if !board_mut.mCursorObject.is_null() {
                    (*(board_mut.mCursorObject)).mType = (*a_plant).m_seed_type;
                    (*(board_mut.mCursorObject)).mCursorType = CursorType::CURSOR_TYPE_PLANT_FROM_GLOVE;
                }
                // [TODO]: mGlovePlantID = DataArrayGetID(aPlant)
            }
        }
    }
    /// C++ ZenGarden::MouseDownWithFeedingTool (ZenGarden.cpp:893) — 工具效果
    pub unsafe fn MouseDownWithFeedingTool(&mut self, x: i32, y: i32, the_cursor_type: i32) {
        unsafe {
            let mut a_plant_to_feed: *mut crate::lawn::plant::Plant = std::ptr::null_mut();
            let the_board = &*self.mBoard;
            let mut a_plant: *mut crate::lawn::plant::Plant = std::ptr::null_mut();
            while the_board.IteratePlants(&mut a_plant) {
                if (*a_plant).m_highlighted && (*a_plant).m_potted_plant_index != -1 {
                    a_plant_to_feed = a_plant;
                    break;
                }
            }

            if the_cursor_type == CursorType::CURSOR_TYPE_CHOCOLATE as i32 {
                // C++: 巧克力：先喂 Stinky，再喂植物
                let a_stinky = self.GetStinky();
                // [TODO]: GridItem mHighlighted 字段（Stinky 高亮检查）
                if !a_stinky.is_null() {
                    self.WakeStinky();
                    if !(*self.mApp).m_player_info.is_null() {
                        let a_player = &mut *(*self.mApp).m_player_info;
                        a_player.mLastStinkyChocolateTime = self.mNowTime as u32;
                        a_player.mPurchases[StoreItem::STORE_ITEM_CHOCOLATE as usize] -= 1;
                    }
                    // [TODO]: AddTodParticle(PARTICLE_PRESENT_PICKUP)
                }

                if !a_plant_to_feed.is_null() && !(*self.mApp).m_player_info.is_null() {
                    (*(*self.mApp).m_player_info).mPurchases[StoreItem::STORE_ITEM_CHOCOLATE as usize] -= 1;
                    self.FeedChocolateToPlant(a_plant_to_feed);
                }
            }

            if !a_plant_to_feed.is_null() {
                // C++: 创建工具 GridItem
                let board_mut = &mut *self.mBoard;
                let a_zen_tool = board_mut.mGridItems.data_array_alloc();
                if a_zen_tool.is_null() {
                    return;
                }
                (*a_zen_tool).mGridItemType = GridItemType::GRIDITEM_ZEN_TOOL;
                (*a_zen_tool).mGridX = (*a_plant_to_feed).m_plant_col;
                (*a_zen_tool).mGridY = (*a_plant_to_feed).base.m_row;
                (*a_zen_tool).mPosX = (*a_plant_to_feed).base.m_x as f32 + 40.0;
                (*a_zen_tool).mPosY = (*a_plant_to_feed).base.m_y as f32 + 40.0;
                (*a_zen_tool).mRenderOrder = crate::lawn::board::Board::MakeRenderOrder(RenderLayer::RENDER_LAYER_ABOVE_UI, 0, 0);

                let a_player = &mut *(*self.mApp).m_player_info;
                if the_cursor_type == CursorType::CURSOR_TYPE_WATERING_CAN as i32 {
                    if a_player.mPurchases[StoreItem::STORE_ITEM_GOLD_WATERINGCAN as usize] != 0 {
                        (*a_zen_tool).mPosX = x as f32;
                        (*a_zen_tool).mPosY = y as f32;
                        (*a_zen_tool).mGridItemState = 8; /* GRIDITEM_STATE_ZEN_TOOL_GOLD_WATERING_CAN */
                    } else {
                        (*a_zen_tool).mGridItemState = 7; /* GRIDITEM_STATE_ZEN_TOOL_WATERING_CAN */
                    }
                    // [TODO]: AddReanimation(REANIM_ZENGARDEN_WATERINGCAN) + FOLEY_WATERING
                } else if the_cursor_type == CursorType::CURSOR_TYPE_FERTILIZER as i32 {
                    (*a_zen_tool).mGridItemState = 15; /* GRIDITEM_STATE_ZEN_TOOL_FERTILIZER */
                    a_player.mPurchases[StoreItem::STORE_ITEM_FERTILIZER as usize] -= 1;
                    // [TODO]: AddReanimation(REANIM_ZENGARDEN_FERTILIZER) + FOLEY_FERTILIZER
                } else if the_cursor_type == CursorType::CURSOR_TYPE_BUG_SPRAY as i32 {
                    (*a_zen_tool).mGridItemState = 16; /* GRIDITEM_STATE_ZEN_TOOL_BUG_SPRAY */
                    a_player.mPurchases[StoreItem::STORE_ITEM_BUG_SPRAY as usize] -= 1;
                    // [TODO]: AddReanimation(REANIM_ZENGARDEN_BUGSPRAY) + FOLEY_BUGSPRAY
                } else if the_cursor_type == CursorType::CURSOR_TYPE_PHONOGRAPH as i32 {
                    (*a_zen_tool).mGridItemState = 17; /* GRIDITEM_STATE_ZEN_TOOL_PHONOGRAPH */
                    // [TODO]: AddReanimation(REANIM_ZENGARDEN_PHONOGRAPH) + FOLEY_PHONOGRAPH
                }
            }

            (*self.mBoard).ClearCursor();
        }
    }

    /// C++ ZenGarden::FeedChocolateToPlant (ZenGarden.cpp:999) — 喂巧克力
    pub unsafe fn FeedChocolateToPlant(&mut self, the_plant: *mut crate::lawn::plant::Plant) {
        unsafe {
            if the_plant.is_null() {
                return;
            }
            let a_potted_plant = self.PottedPlantFromIndex((*the_plant).m_potted_plant_index);
            if a_potted_plant.is_null() {
                return;
            }
            // C++: mLastChocolateTime = mNowTime; mLaunchCounter = 60;
            (*a_potted_plant).mLastChocolateTime = self.mNowTime;
            (*the_plant).m_launch_counter = 60;
            // [TODO]: AddTodParticle(PARTICLE_PRESENT_PICKUP)
        }
    }

    /// C++ ZenGarden::DoFeedingTool (ZenGarden.cpp:1007) — 工具生效
    pub unsafe fn DoFeedingTool(&mut self, x: i32, y: i32, the_tool_type: i32) {
        unsafe {
            if the_tool_type == 8 /* GRIDITEM_STATE_ZEN_TOOL_GOLD_WATERING_CAN */ {
                // C++: 金水壶范围浇水
                let the_board = &*self.mBoard;
                let mut a_plant: *mut crate::lawn::plant::Plant = std::ptr::null_mut();
                while the_board.IteratePlants(&mut a_plant) {
                    if self.IsPlantInGoldWateringCanRange(x, y, a_plant) {
                        let a_potted_plant = self.PottedPlantFromIndex((*a_plant).m_potted_plant_index);
                        if !a_potted_plant.is_null() && self.GetPlantsNeed(a_potted_plant) == PottedPlantNeed::PLANTNEED_WATER {
                            self.PlantWatered(a_plant);
                        }
                    }
                }
                return;
            }

            // C++: 普通工具按格子生效
            let a_grid_x = self.PixelToGridX(x, y);
            let a_grid_y = self.PixelToGridY(x, y);
            let the_board = &*self.mBoard;
            let a_plant = the_board.GetTopPlantAt(a_grid_x, a_grid_y, PlantPriority::TOPPLANT_ZEN_TOOL_ORDER);
            if !a_plant.is_null() {
                let a_potted_plant = self.PottedPlantFromIndex((*a_plant).m_potted_plant_index);
                if !a_potted_plant.is_null() {
                    let a_need = self.GetPlantsNeed(a_potted_plant);
                    if a_need == PottedPlantNeed::PLANTNEED_WATER && the_tool_type == 7 /* WATERING_CAN */ {
                        self.PlantWatered(a_plant);
                    } else if a_need == PottedPlantNeed::PLANTNEED_FERTILIZER && the_tool_type == 15 /* FERTILIZER */ {
                        self.PlantFertilized(a_plant);
                    } else if a_need == PottedPlantNeed::PLANTNEED_BUGSPRAY && the_tool_type == 16 /* BUG_SPRAY */ {
                        self.PlantFulfillNeed(a_plant);
                    } else if a_need == PottedPlantNeed::PLANTNEED_PHONOGRAPH && the_tool_type == 17 /* PHONOGRAPH */ {
                        self.PlantFulfillNeed(a_plant);
                    }
                }

                // C++: 教程进度
                // [TODO]: TUTORIAL_ZEN_GARDEN_FERTILIZE_PLANTS 检查 + 化肥补给 + 提示
            }
        }
    }

    pub fn DrawPlantOverlay(&self, _g: &mut Graphics, _thePlant: *mut Plant) {}
    /// C++ ZenGarden::PottedPlantUpdate (ZenGarden.cpp:2257) — 盆栽养成循环
    pub unsafe fn PottedPlantUpdate(&mut self, the_plant: *mut Plant) {
        unsafe {
            if the_plant.is_null() {
                return;
            }
            let a_potted_plant = self.PottedPlantFromIndex((*the_plant).m_potted_plant_index);
            if a_potted_plant.is_null() {
                return;
            }
            let a_now = self.mNowTime;
            // C++: 时间戳异常时重置
            if (*a_potted_plant).mLastWateredTime > a_now
                || (*a_potted_plant).mLastNeedFulfilledTime > a_now
                || (*a_potted_plant).mLastFertilizedTime > a_now
                || (*a_potted_plant).mLastChocolateTime > a_now
            {
                self.ResetPlantTimers(a_potted_plant);
            }

            if (*the_plant).m_is_asleep {
                return;
            }
            if (*the_plant).m_state_countdown > 0 {
                (*the_plant).m_state_countdown -= 1;
            }
            if (*a_potted_plant).mPlantAge == PottedPlantAge::PLANTAGE_FULL && self.WasPlantNeedFulfilledToday(a_potted_plant) {
                self.PlantUpdateProduction(the_plant);
            }
            self.UpdatePlantEffectState(the_plant);
        }
    }

    /// C++ ZenGarden::ResetPlantTimers (ZenGarden.cpp:2249)
    pub unsafe fn ResetPlantTimers(&mut self, the_potted_plant: *mut PottedPlant) {
        unsafe {
            if the_potted_plant.is_null() {
                return;
            }
            (*the_potted_plant).mLastWateredTime = 0;
            (*the_potted_plant).mLastNeedFulfilledTime = 0;
            (*the_potted_plant).mLastFertilizedTime = 0;
            (*the_potted_plant).mLastChocolateTime = 0;
        }
    }

    /// C++ ZenGarden::UpdatePlantEffectState (ZenGarden.cpp:657) — 植物效果状态
    pub unsafe fn UpdatePlantEffectState(&mut self, the_plant: *mut Plant) {
        unsafe {
            if the_plant.is_null() {
                return;
            }
            let a_original_state = (*the_plant).m_state;
            let a_potted_plant = self.PottedPlantFromIndex((*the_plant).m_potted_plant_index);
            if a_potted_plant.is_null() {
                return;
            }

            let a_plant_need = self.GetPlantsNeed(a_potted_plant);
            if a_plant_need == PottedPlantNeed::PLANTNEED_WATER {
                (*the_plant).m_state = crate::lawn::plant::PlantState::STATE_NOTREADY;
            } else if a_plant_need == PottedPlantNeed::PLANTNEED_NONE {
                if self.WasPlantNeedFulfilledToday(a_potted_plant) {
                    (*the_plant).m_state = crate::lawn::plant::PlantState::STATE_ZEN_GARDEN_HAPPY;
                } else if (*the_plant).m_is_asleep {
                    (*the_plant).m_state = crate::lawn::plant::PlantState::STATE_NOTREADY;
                } else {
                    (*the_plant).m_state = crate::lawn::plant::PlantState::STATE_ZEN_GARDEN_WATERED;
                }
            } else {
                (*the_plant).m_state = crate::lawn::plant::PlantState::STATE_ZEN_GARDEN_NEEDY;
            }
            if a_original_state == (*the_plant).m_state {
                return;
            }

            // [TODO]: 花盆 Pot_top 贴图切换

            if a_original_state == crate::lawn::plant::PlantState::STATE_ZEN_GARDEN_HAPPY {
                self.RemoveHappyEffect(the_plant);
            }
            if (*the_plant).m_state == crate::lawn::plant::PlantState::STATE_ZEN_GARDEN_HAPPY {
                (*the_plant).SetSleeping(false);
                self.AddHappyEffect(the_plant);
            } else if crate::lawn::plant::Plant::is_nocturnal((*the_plant).m_seed_type) && !(*self.mBoard).StageIsNight() {
                (*the_plant).SetSleeping(true);
            }
        }
    }

    /// C++ ZenGarden::PlantHighOnChocolate (ZenGarden.cpp:2338)
    pub fn PlantHighOnChocolate(&self, the_potted_plant: *mut PottedPlant) -> bool {
        unsafe {
            if the_potted_plant.is_null() {
                return false;
            }
            // C++: mNowTime - mLastChocolateTime < 300
            self.mNowTime - (*the_potted_plant).mLastChocolateTime < 300
        }
    }

    /// C++ ZenGarden::IsStinkyHighOnChocolate (ZenGarden.cpp:2332)
    pub fn IsStinkyHighOnChocolate(&self) -> bool {
        unsafe {
            if self.mApp.is_null() || (*self.mApp).m_player_info.is_null() {
                return false;
            }
            // C++: mNowTime - mLastStinkyChocolateTime < 3600
            self.mNowTime as u32 - (*(*self.mApp).m_player_info).mLastStinkyChocolateTime < 3600
        }
    }
    pub fn PlantUpdateProduction(&mut self, _thePlant: *mut Plant) {}
    pub fn ShowTutorialArrowOnWateringCan(&self) {}
    /// C++ ZenGarden::PlantCanBeWatered (ZenGarden.cpp:592)
    pub fn PlantCanBeWatered(&self, the_plant: *mut Plant) -> bool {
        unsafe {
            if the_plant.is_null() || (*the_plant).m_potted_plant_index == -1 {
                return false;
            }
            let a_potted_plant = self.PottedPlantFromIndex((*the_plant).m_potted_plant_index);
            if a_potted_plant.is_null() {
                return false;
            }
            self.GetPlantsNeed(a_potted_plant) == PottedPlantNeed::PLANTNEED_WATER
        }
    }
    /// C++ ZenGarden::PlantsNeedWater (ZenGarden.cpp:579)
    pub fn PlantsNeedWater(&self) -> bool {
        unsafe {
            if self.mApp.is_null() || (*self.mApp).m_player_info.is_null() {
                return false;
            }
            let the_player_info = &*(*self.mApp).m_player_info;
            let mut i = 0;
            while i < the_player_info.mNumPottedPlants as usize {
                let a_potted_plant = self.PottedPlantFromIndex(i as i32);
                if !a_potted_plant.is_null() && self.GetPlantsNeed(a_potted_plant) == PottedPlantNeed::PLANTNEED_WATER {
                    return true;
                }
                i += 1;
            }
        }
        false
    }
    pub fn ZenGardenStart(&mut self) {}
    pub fn CanUseGameObject(&self, _theObjectType: i32) -> bool { false }
    pub fn ZenToolUpdate(&mut self, _theZenTool: *mut GridItem) {}
    pub fn AddStinky(&mut self) {}
    /// C++ ZenGarden::StinkyUpdate (ZenGarden.cpp:1507) — 蜗牛更新（状态机）
    pub unsafe fn StinkyUpdate(&mut self, the_stinky: *mut GridItem) {
        unsafe {
            if the_stinky.is_null() {
                return;
            }
            // C++: Reanimation* aStinkyReanim = mApp->ReanimationGet(mGridItemReanimID);
            // [TODO]: Reanimation 获取

            let a_stinky_high_on_chocolate = self.IsStinkyHighOnChocolate();
            // [TODO]: UpdateStinkyMotionTrail(theStinky, aStinkyHighOnChocolate)
            let _ = a_stinky_high_on_chocolate;

            if (*the_stinky).mGridItemState == 24 /* GRIDITEM_STINKY_FALLING_ASLEEP */ {
                // C++: if (aStinkyReanim->mLoopCount > 0) StinkyFinishFallingAsleep(theStinky, 20);
                // [TODO]: Reanimation mLoopCount 检查
                self.StinkyFinishFallingAsleep(the_stinky, 20);
                return;
            }

            if (*the_stinky).mGridItemState == 23 /* GRIDITEM_STINKY_SLEEPING */ {
                // [TODO]: FindReanimAttachment + AssignRenderGroupToPrefix（巧克力高亮）

                if self.ShouldStinkyBeAwake() {
                    self.StinkyWakeUp(the_stinky);
                }
                return;
            }

            if (*the_stinky).mGridItemState == 25 /* GRIDITEM_STINKY_WAKING_UP */ {
                // C++: mLoopCount > 0 时 → WALKING_LEFT + PlayReanim("anim_crawl") + StinkyPickGoal
                // [TODO]: Reanimation mLoopCount 检查
                (*the_stinky).mGridItemState = 19; /* GRIDITEM_STINKY_WALKING_LEFT */
                self.StinkyPickGoal(the_stinky);
                return;
            }

            if !self.ShouldStinkyBeAwake() {
                // C++: 走向睡眠位置
                if (*the_stinky).mPosY < STINKY_SLEEP_POS_Y {
                    if (*the_stinky).mGoalY != STINKY_SLEEP_POS_Y {
                        (*the_stinky).mGoalY = STINKY_SLEEP_POS_Y + 10.0;
                    }
                } else if (*the_stinky).mGridItemState == 19 /* WALKING_LEFT */ {
                    self.StinkyStartFallingAsleep(the_stinky);
                    return;
                } else if (*the_stinky).mGridItemState == 21 /* WALKING_RIGHT */ {
                    (*the_stinky).mGridItemState = 20; /* TURNING_LEFT */
                    (*the_stinky).mMotionTrailCount = 0;
                    (*the_stinky).mGoalX = (*the_stinky).mPosX;
                    (*the_stinky).mGoalY = (*the_stinky).mPosY;
                    return;
                }
            }

            // C++: 移动/拾取/转向逻辑
            if (*the_stinky).mGridItemCounter > 0 {
                (*the_stinky).mGridItemCounter -= 1;
            }

            // C++: 收集附近金币（距离 < 20）
            let the_board_borrow = &*self.mBoard;
            let mut a_coin: *mut crate::lawn::coin::Coin = std::ptr::null_mut();
            while the_board_borrow.IterateCoins(&mut a_coin) {
                if !(*a_coin).m_is_being_collected
                    && crate::sexy_tod_lib::tod_common::distance_2d(
                        (*a_coin).m_pos_x,
                        (*a_coin).m_pos_y + 30.0,
                        (*the_stinky).mPosX,
                        (*the_stinky).mPosY,
                    ) < 20.0
                {
                    (*a_coin).PlayCollectSound();
                    (*a_coin).Collect((*a_coin).m_pos_x, (*a_coin).m_pos_y);
                }
            }

            // C++: 移动速度
            let a_delta_x = (*the_stinky).mPosX - (*the_stinky).mGoalX;
            let a_delta_y = (*the_stinky).mPosY - (*the_stinky).mGoalY;
            let mut a_speed_y: f32 = 0.5;
            let mut a_speed_x: f32 = 0.5; // [TODO]: Reanimation GetTrackVelocity("_ground") * 15
            if a_stinky_high_on_chocolate {
                a_speed_y = 1.0;
                a_speed_x = a_speed_x.max(0.5);
            }
            // [TODO]: 巧克力光标时停步（aSpeedY/X = 0）
            // C++: aSpeedY *= TodAnimateCurveFloatTime(20, 5, |aDeltaY|, 1.0, 0.2, CURVE_LINEAR);
            a_speed_y *= crate::sexy_tod_lib::tod_common::tod_animate_curve_float_time(
                20.0, 5.0, a_delta_y.abs(), 1.0, 0.2, crate::const_enums::TodCurves::CURVE_LINEAR,
            );

            // C++: X 方向移动（左/右）
            if (*the_stinky).mGridItemState == 19 /* WALKING_LEFT */ {
                (*the_stinky).mPosX -= a_speed_x;
                if (*the_stinky).mPosX < (*the_stinky).mGoalX {
                    (*the_stinky).mPosX = (*the_stinky).mGoalX;
                }
            } else if (*the_stinky).mGridItemState == 21 /* WALKING_RIGHT */ {
                (*the_stinky).mPosX += a_speed_x;
                if (*the_stinky).mPosX > (*the_stinky).mGoalX {
                    (*the_stinky).mPosX = (*the_stinky).mGoalX;
                }
            }

            // C++: Y 方向移动 + 到达目标
            if (*the_stinky).mGridItemState == 19 || (*the_stinky).mGridItemState == 21 {
                if a_delta_y.abs() < a_speed_y {
                    (*the_stinky).mPosY = (*the_stinky).mGoalY;
                } else if a_delta_y > 0.0 {
                    (*the_stinky).mPosY -= a_speed_y;
                } else {
                    (*the_stinky).mPosY += a_speed_y;
                }

                if a_delta_x.abs() < 5.0 && a_delta_y.abs() < 5.0 {
                    self.StinkyPickGoal(the_stinky);
                } else if (*the_stinky).mGridItemCounter == 0 {
                    self.StinkyPickGoal(the_stinky);
                }
            }

            // C++: 转身完成 → 继续行走
            if (*the_stinky).mGridItemState == 20 /* TURNING_LEFT */ {
                (*the_stinky).mGridItemState = 19; /* WALKING_LEFT */
            } else if (*the_stinky).mGridItemState == 22 /* TURNING_RIGHT */ {
                (*the_stinky).mGridItemState = 21; /* WALKING_RIGHT */
            }
        }
    }    /// C++ ZenGarden::OpenStore (ZenGarden.cpp:2360) — 打开商店
    pub unsafe fn OpenStore(&mut self) {
        self.LeaveGarden();
        // [TODO]: StoreScreen* aStore = mApp->ShowStoreScreen();

        // [TODO]: TUTORIAL_ZEN_GARDEN_VISIT_STORE 教程：SetupForIntro(2600) + 化肥补给
        // [TODO]: aStore->mBackButton->SetLabel("[STORE_BACK_TO_GAME]")
        // [TODO]: aStore->mPage = STORE_PAGE_ZEN1; aStore->WaitForResult(true)

        // [TODO]: aStore->mGoToTreeNow → KillBoard + PreNewGame(TREE_OF_WISDOM)
        let _go_to_tree_now = false;
        if _go_to_tree_now {
            (*self.mApp).KillBoard();
            (*self.mApp).PreNewGame(GameMode::GAMEMODE_TREE_OF_WISDOM, false);
        } else {
            // C++: mNowTime = mApp->GetNowTime(); mNowTM = mApp->GetLocalTime(mNowTime);
            // [TODO]: GetNowTime 时间同步
            // C++: mApp->mMusic->MakeSureMusicIsPlaying(MUSIC_TUNE_ZEN_GARDEN)
            // [TODO]: TUTORIAL_ZEN_GARDEN_VISIT_STORE → FERTILIZE_PLANTS + 提示
            self.AddStinky();
        }
    }    /// C++ ZenGarden::GetStinky (ZenGarden.cpp:1789)
    pub unsafe fn GetStinky(&self) -> *mut GridItem {
        unsafe {
            let the_board = &*self.mBoard;
            let mut a_grid_item: *mut GridItem = std::ptr::null_mut();
            while the_board.IterateGridItems(&mut a_grid_item) {
                if (*a_grid_item).mGridItemType == GridItemType::GRIDITEM_STINKY {
                    return a_grid_item;
                }
            }
        }
        std::ptr::null_mut()
    }    pub fn StinkyPickGoal(&mut self, _theStinky: *mut GridItem) {}
    pub fn SetupForZenTutorial(&mut self) {}
    pub fn HasPurchasedStinky(&self) -> bool { false }
    /// C++ ZenGarden::CountPlantsNeedingFertilizer (ZenGarden.cpp:603)
    pub fn CountPlantsNeedingFertilizer(&self) -> i32 {
        let mut a_count = 0;
        unsafe {
            if self.mApp.is_null() || (*self.mApp).m_player_info.is_null() {
                return 0;
            }
            let the_player_info = &*(*self.mApp).m_player_info;
            let mut i = 0;
            while i < the_player_info.mNumPottedPlants as usize {
                let a_potted_plant = self.PottedPlantFromIndex(i as i32);
                if !a_potted_plant.is_null() && self.GetPlantsNeed(a_potted_plant) == PottedPlantNeed::PLANTNEED_FERTILIZER {
                    a_count += 1;
                }
                i += 1;
            }
        }
        a_count
    }
    /// C++ ZenGarden::AllPlantsHaveBeenFertilized (ZenGarden.cpp:617)
    pub fn AllPlantsHaveBeenFertilized(&self) -> bool {
        unsafe {
            if self.mApp.is_null() || (*self.mApp).m_player_info.is_null() {
                return false;
            }
            let the_player_info = &*(*self.mApp).m_player_info;
            let mut i = 0;
            while i < the_player_info.mNumPottedPlants as usize {
                let a_potted_plant = self.PottedPlantFromIndex(i as i32);
                if !a_potted_plant.is_null() && self.GetPlantsNeed(a_potted_plant) == PottedPlantNeed::PLANTNEED_FERTILIZER {
                    return false;
                }
                i += 1;
            }
        }
        true
    }
    /// C++ ZenGarden::WakeStinky (ZenGarden.cpp:2322)
    pub unsafe fn WakeStinky(&mut self) {
        unsafe {
            if (*self.mApp).m_player_info.is_null() {
                return;
            }
            let a_player = &mut *(*self.mApp).m_player_info;
            // C++: mPurchases[STINKY] = 时间戳
            let a_time = self.mNowTime as u32;
            a_player.mPurchases[StoreItem::STORE_ITEM_STINKY_THE_SNAIL as usize] = if a_time == 0 { 1 } else { a_time };
            // [TODO]: mApp->PlaySample(SOUND_TAP); ClearAdvice(ADVICE_STINKY_SLEEPING)
            a_player.mHasWokenStinky = 1;
        }
    }    pub fn ShouldStinkyBeAwake(&self) -> bool { false }
    pub fn IsStinkySleeping(&self) -> bool { true }
    pub fn PickRandomSeedType() -> SeedType { SeedType::SEED_SUNFLOWER }
    pub fn StinkyWakeUp(&mut self, _theStinky: *mut GridItem) {}
    pub fn StinkyStartFallingAsleep(&mut self, _theStinky: *mut GridItem) {}
    pub fn StinkyFinishFallingAsleep(&mut self, _theStinky: *mut GridItem, _theBlendTime: i32) {}
    pub fn AdvanceCrazyDaveDialog(&mut self) {}
    pub fn LeaveGarden(&mut self) {}
}

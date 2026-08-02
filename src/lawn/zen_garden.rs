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

    /// C++ ZenGarden::WasPlantNeedFulfilledToday — 今天是否满足需求
    pub fn WasPlantNeedFulfilledToday(&self, the_potted_plant: *mut PottedPlant) -> bool {
        // [TODO]: mLastNeedFulfilledTime 与当日时间比较
        let _ = the_potted_plant;
        true
    }

    /// C++ ZenGarden::PlantHighOnChocolate — 植物是否处于巧克力亢奋
    pub fn PlantHighOnChocolate(&self, the_potted_plant: *mut PottedPlant) -> bool {
        // [TODO]: mLastChocolateTime 检查
        let _ = the_potted_plant;
        false
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
    pub fn MouseDownWithTool(&mut self, _x: i32, _y: i32, _theCursorType: i32) {}
    pub fn MovePlant(&mut self, _thePlant: *mut Plant, _theGridX: i32, _theGridY: i32) {}
    pub fn MouseDownWithMoneySign(&mut self, _thePlant: *mut Plant) {}
    pub fn PlacePottedPlant(&mut self, _thePottedPlantIndex: isize) -> *mut Plant { std::ptr::null_mut() }
    pub fn PlantPottedDrawHeightOffset(&self, _theSeedType: SeedType, _theScale: f32) -> f32 { 0.0 }
    pub fn ZenPlantOffsetX(_thePottedPlant: *mut PottedPlant) -> f32 { 0.0 }
    pub fn GetPlantSellPrice(&self, _thePlant: *mut Plant) -> i32 { 0 }
    pub fn ZenGardenUpdate(&mut self) {}
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
    pub fn GetPlantsNeed(&self, _thePottedPlant: *mut PottedPlant) -> PottedPlantNeed { PottedPlantNeed::PLANTNEED_NONE }
    pub fn MouseDownWithFeedingTool(&mut self, _x: i32, _y: i32, _theCursorType: i32) {}
    pub fn DrawPlantOverlay(&self, _g: &mut Graphics, _thePlant: *mut Plant) {}
    pub fn PottedPlantUpdate(&mut self, _thePlant: *mut Plant) {}
    pub fn AddHappyEffect(&mut self, _thePlant: *mut Plant) {}
    pub fn RemoveHappyEffect(&mut self, _thePlant: *mut Plant) {}
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
    pub fn UpdatePlantEffectState(&mut self, _thePlant: *mut Plant) {}
    pub fn CanUseGameObject(&self, _theObjectType: i32) -> bool { false }
    pub fn ZenToolUpdate(&mut self, _theZenTool: *mut GridItem) {}
    pub fn DoFeedingTool(&mut self, _x: i32, _y: i32, _theToolType: i32) {}
    pub fn AddStinky(&mut self) {}
    pub fn StinkyUpdate(&mut self, _theStinky: *mut GridItem) {}
    pub fn OpenStore(&mut self) {}
    pub fn GetStinky(&self) -> *mut GridItem { std::ptr::null_mut() }
    pub fn StinkyPickGoal(&mut self, _theStinky: *mut GridItem) {}
    pub fn PlantShouldRefreshNeed(&self, _thePottedPlant: *mut PottedPlant) -> bool { false }
    pub fn WasPlantFertilizedInLastHour(&self, _thePottedPlant: *mut PottedPlant) -> bool { false }
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
    pub fn WakeStinky(&mut self) {}
    pub fn ShouldStinkyBeAwake(&self) -> bool { false }
    pub fn IsStinkySleeping(&self) -> bool { true }
    pub fn PickRandomSeedType() -> SeedType { SeedType::SEED_SUNFLOWER }
    pub fn StinkyWakeUp(&mut self, _theStinky: *mut GridItem) {}
    pub fn StinkyStartFallingAsleep(&mut self, _theStinky: *mut GridItem) {}
    pub fn StinkyFinishFallingAsleep(&mut self, _theStinky: *mut GridItem, _theBlendTime: i32) {}
    pub fn AdvanceCrazyDaveDialog(&mut self) {}
    pub fn LeaveGarden(&mut self) {}
    pub fn FeedChocolateToPlant(&mut self, _thePlant: *mut Plant) {}
}

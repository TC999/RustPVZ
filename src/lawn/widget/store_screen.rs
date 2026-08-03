// [TRANSLATION_NOTE]: StoreScreen.h + StoreScreen.cpp -> Rust stub

#![allow(non_snake_case, dead_code)]

use crate::lawn_app::LawnApp;
use crate::const_enums::*;
use crate::lawn::widget::lawn_dialog::LawnDialog;
use crate::sexy_app_framework::graphics::graphics::Graphics;

pub struct StoreScreen {
    pub base: LawnDialog,
    pub mApp: *mut LawnApp,
    pub mPage: i32,
    pub mEasyBuyingCheat: bool,
    pub mBubbleCountDown: i32,
}

impl StoreScreen {
    pub fn new(theApp: *mut LawnApp) -> Self {
        StoreScreen {
            base: LawnDialog::new(theApp, 0, true, "", "", "", 0),
            mApp: theApp,
            mPage: 0,
            mEasyBuyingCheat: false,
            mBubbleCountDown: 0,
        }
    }
        /// C++: static StoreItem gStoreItemSpots[NUM_STORE_PAGES][MAX_PAGE_SPOTS]
    pub const STORE_ITEM_SPOTS: [[i32; 8]; 4] = [
        [
            StoreItem::STORE_ITEM_PACKET_UPGRADE as i32, StoreItem::STORE_ITEM_POOL_CLEANER as i32, StoreItem::STORE_ITEM_RAKE as i32, StoreItem::STORE_ITEM_ROOF_CLEANER as i32,
            StoreItem::STORE_ITEM_PLANT_GATLINGPEA as i32, StoreItem::STORE_ITEM_PLANT_TWINSUNFLOWER as i32, StoreItem::STORE_ITEM_PLANT_GLOOMSHROOM as i32, StoreItem::STORE_ITEM_PLANT_CATTAIL as i32,
        ],
        [
            StoreItem::STORE_ITEM_PLANT_SPIKEROCK as i32, StoreItem::STORE_ITEM_PLANT_GOLD_MAGNET as i32, StoreItem::STORE_ITEM_PLANT_WINTERMELON as i32, StoreItem::STORE_ITEM_PLANT_COBCANNON as i32,
            StoreItem::STORE_ITEM_PLANT_IMITATER as i32, StoreItem::STORE_ITEM_FIRSTAID as i32, STORE_ITEM_INVALID, STORE_ITEM_INVALID,
        ],
        [
            StoreItem::STORE_ITEM_POTTED_MARIGOLD_1 as i32, StoreItem::STORE_ITEM_POTTED_MARIGOLD_2 as i32, StoreItem::STORE_ITEM_POTTED_MARIGOLD_3 as i32, StoreItem::STORE_ITEM_GOLD_WATERINGCAN as i32,
            StoreItem::STORE_ITEM_FERTILIZER as i32, StoreItem::STORE_ITEM_BUG_SPRAY as i32, StoreItem::STORE_ITEM_PHONOGRAPH as i32, StoreItem::STORE_ITEM_GARDENING_GLOVE as i32,
        ],
        [
            StoreItem::STORE_ITEM_MUSHROOM_GARDEN as i32, StoreItem::STORE_ITEM_AQUARIUM_GARDEN as i32, StoreItem::STORE_ITEM_WHEEL_BARROW as i32, StoreItem::STORE_ITEM_STINKY_THE_SNAIL as i32,
            StoreItem::STORE_ITEM_TREE_OF_WISDOM as i32, StoreItem::STORE_ITEM_TREE_FOOD as i32, STORE_ITEM_INVALID, STORE_ITEM_INVALID,
        ],
    ];

    /// C++ StoreScreen::GetStoreItemType (StoreScreen.cpp:160)
    pub fn GetStoreItemType(&self, the_spot_index: i32) -> i32 {
        if self.mPage < 4 /* NUM_STORE_PAGES */ && the_spot_index < 8 /* MAX_PAGE_SPOTS */ {
            // C++: SLOT_UPGRADES 页第 7 位 + 试用版 → PVZ
            if self.mPage == 0 /* STORE_PAGE_SLOT_UPGRADES */ && the_spot_index == 6 {
                // [TODO]: IsTrialStageLocked
                if false {
                    return StoreItem::STORE_ITEM_PVZ as i32;
                }
            }
            return Self::STORE_ITEM_SPOTS[self.mPage as usize][the_spot_index as usize];
        }
        STORE_ITEM_INVALID
    }

    /// C++ StoreScreen::IsFullVersionOnly (StoreScreen.cpp:177)
    pub fn IsFullVersionOnly(&self, the_store_item: i32) -> bool {
        // [TODO]: IsTrialStageLocked
        if true {
            return false;
        }

        if the_store_item == StoreItem::STORE_ITEM_PACKET_UPGRADE as i32 {
            unsafe {
                if !(*self.mApp).m_player_info.is_null()
                    && (*(*self.mApp).m_player_info).mPurchases[StoreItem::STORE_ITEM_PACKET_UPGRADE as usize] >= 2
                {
                    return true;
                }
            }
        }

        the_store_item == StoreItem::STORE_ITEM_PLANT_TWINSUNFLOWER as i32
    }

    /// C++ StoreScreen::IsPottedPlant (StoreScreen.cpp:188)
    pub fn IsPottedPlant(&self, the_store_item: i32) -> bool {
        the_store_item == StoreItem::STORE_ITEM_POTTED_MARIGOLD_1 as i32
            || the_store_item == StoreItem::STORE_ITEM_POTTED_MARIGOLD_2 as i32
            || the_store_item == StoreItem::STORE_ITEM_POTTED_MARIGOLD_3 as i32
    }

    /// C++ StoreScreen::IsComingSoon (StoreScreen.cpp:193)
    pub fn IsComingSoon(&self, the_store_item: i32) -> bool {
        if self.IsFullVersionOnly(the_store_item) {
            return true;
        } else if the_store_item == StoreItem::STORE_ITEM_WHEEL_BARROW as i32 {
            unsafe {
                return !(*self.mApp).m_player_info.is_null()
                    && (*(*self.mApp).m_player_info).mPurchases[StoreItem::STORE_ITEM_MUSHROOM_GARDEN as usize] == 0
                    && (*(*self.mApp).m_player_info).mPurchases[StoreItem::STORE_ITEM_AQUARIUM_GARDEN as usize] == 0;
            }
        } else if self.IsPottedPlant(the_store_item) {
            return unsafe { !(*self.mApp).HasFinishedAdventure() };
        } else if the_store_item == StoreItem::STORE_ITEM_TREE_FOOD as i32 {
            unsafe {
                return !(*self.mApp).m_player_info.is_null()
                    && ((*(*self.mApp).m_player_info).mPurchases[StoreItem::STORE_ITEM_TREE_OF_WISDOM as usize] == 0
                        || (*(*self.mApp).m_player_info).mPurchases[StoreItem::STORE_ITEM_TREE_FOOD as usize]
                            < crate::lawn::system::player_info::PURCHASE_COUNT_OFFSET as u32);
            }
        }
        false
    }

    /// C++ StoreScreen::IsItemSoldOut (StoreScreen.cpp:206)
    pub fn IsItemSoldOut(&self, the_store_item: i32) -> bool {
        unsafe {
            if (*self.mApp).m_player_info.is_null() {
                return false;
            }
            let a_player = &*(*self.mApp).m_player_info;
            if the_store_item == STORE_ITEM_INVALID {
                return false;
            } else if the_store_item == StoreItem::STORE_ITEM_PACKET_UPGRADE as i32 {
                return a_player.mPurchases[StoreItem::STORE_ITEM_PACKET_UPGRADE as usize] >= 4;
            } else if the_store_item == StoreItem::STORE_ITEM_FERTILIZER as i32
                || the_store_item == StoreItem::STORE_ITEM_BUG_SPRAY as i32
            {
                return a_player.mPurchases[the_store_item as usize] > (crate::lawn::system::player_info::PURCHASE_COUNT_OFFSET + 15) as u32;
            } else if the_store_item == StoreItem::STORE_ITEM_TREE_FOOD as i32 {
                return a_player.mPurchases[StoreItem::STORE_ITEM_TREE_FOOD as usize] >= (crate::lawn::system::player_info::PURCHASE_COUNT_OFFSET + 10) as u32;
            } else if the_store_item == StoreItem::STORE_ITEM_BONUS_LAWN_MOWER as i32 {
                return a_player.mPurchases[StoreItem::STORE_ITEM_BONUS_LAWN_MOWER as usize] >= 2;
            } else if self.IsPottedPlant(the_store_item) {
                // [TODO]: GetCurrentDaysSince2000 每日一盆限制
                // [TODO]: mApp->mZenGarden->IsZenGardenFull(true)
                return a_player.mPurchases[the_store_item as usize] != 0;
            }
            a_player.mPurchases[the_store_item as usize] != 0
        }
    }

    /// C++ StoreScreen::IsItemUnavailable (StoreScreen.cpp:226)
    pub fn IsItemUnavailable(&self, the_store_item: i32) -> bool {
        // C++: mEasyBuyingCheat
        if self.mEasyBuyingCheat {
            return false;
        }

        let a_finished_adventure = unsafe { (*self.mApp).HasFinishedAdventure() };
        let a_level = unsafe {
            if (*self.mApp).m_player_info.is_null() {
                0
            } else {
                (*(*self.mApp).m_player_info).GetLevel()
            }
        };

        if the_store_item == StoreItem::STORE_ITEM_ROOF_CLEANER as i32 {
            // [TODO]: IsTrialStageLocked
            return !a_finished_adventure && a_level < 42;
        }
        if the_store_item == StoreItem::STORE_ITEM_PLANT_GLOOMSHROOM as i32
            || the_store_item == StoreItem::STORE_ITEM_PLANT_CATTAIL as i32
        {
            // [TODO]: IsTrialStageLocked
            return !a_finished_adventure && a_level < 35;
        }
        if the_store_item == StoreItem::STORE_ITEM_PLANT_SPIKEROCK as i32
            || the_store_item == StoreItem::STORE_ITEM_PLANT_GOLD_MAGNET as i32
        {
            return !a_finished_adventure && a_level < 41;
        }
        if the_store_item == StoreItem::STORE_ITEM_PLANT_WINTERMELON as i32
            || the_store_item == StoreItem::STORE_ITEM_PLANT_COBCANNON as i32
            || the_store_item == StoreItem::STORE_ITEM_PLANT_IMITATER as i32
            || the_store_item == StoreItem::STORE_ITEM_FIRSTAID as i32
        {
            return !a_finished_adventure;
        }
        false
    }
pub fn Draw(&self, g: &mut Graphics) { self.base.Draw(g); }
    /// C++ StoreScreen::GetItemCost (StoreScreen.cpp:883)
    pub unsafe fn GetItemCost(&self, the_store_item: i32) -> i32 {
        if the_store_item == StoreItem::STORE_ITEM_BONUS_LAWN_MOWER as i32 {
            unsafe {
                return if (*self.mApp).m_player_info.is_null() {
                    200
                } else if (*(*self.mApp).m_player_info).mPurchases[StoreItem::STORE_ITEM_BONUS_LAWN_MOWER as usize] != 0 {
                    500
                } else {
                    200
                };
            }
        }
        match the_store_item {
            x if x == StoreItem::STORE_ITEM_PLANT_GATLINGPEA as i32 => 500,
            x if x == StoreItem::STORE_ITEM_PLANT_TWINSUNFLOWER as i32 => 500,
            x if x == StoreItem::STORE_ITEM_PLANT_GLOOMSHROOM as i32 => 750,
            x if x == StoreItem::STORE_ITEM_PLANT_CATTAIL as i32 => 1000,
            x if x == StoreItem::STORE_ITEM_PLANT_WINTERMELON as i32 => 1000,
            x if x == StoreItem::STORE_ITEM_PLANT_GOLD_MAGNET as i32 => 300,
            x if x == StoreItem::STORE_ITEM_PLANT_SPIKEROCK as i32 => 750,
            x if x == StoreItem::STORE_ITEM_PLANT_COBCANNON as i32 => 2000,
            x if x == StoreItem::STORE_ITEM_PLANT_IMITATER as i32 => 3000,
            x if x == StoreItem::STORE_ITEM_POTTED_MARIGOLD_1 as i32
                || x == StoreItem::STORE_ITEM_POTTED_MARIGOLD_2 as i32
                || x == StoreItem::STORE_ITEM_POTTED_MARIGOLD_3 as i32 => 250,
            x if x == StoreItem::STORE_ITEM_GOLD_WATERINGCAN as i32 => 1000,
            x if x == StoreItem::STORE_ITEM_FERTILIZER as i32 => 75,
            x if x == StoreItem::STORE_ITEM_BUG_SPRAY as i32 => 100,
            x if x == StoreItem::STORE_ITEM_PHONOGRAPH as i32 => 1500,
            x if x == StoreItem::STORE_ITEM_GARDENING_GLOVE as i32 => 100,
            x if x == StoreItem::STORE_ITEM_MUSHROOM_GARDEN as i32 => 3000,
            x if x == StoreItem::STORE_ITEM_WHEEL_BARROW as i32 => 20,
            x if x == StoreItem::STORE_ITEM_STINKY_THE_SNAIL as i32 => 300,
            x if x == StoreItem::STORE_ITEM_PACKET_UPGRADE as i32 => {
                // C++: 0→75, 1→500, 2→2000, 3→8000
                unsafe {
                    if (*self.mApp).m_player_info.is_null() {
                        75
                    } else {
                        match (*(*self.mApp).m_player_info).mPurchases[StoreItem::STORE_ITEM_PACKET_UPGRADE as usize] {
                            0 => 75,
                            1 => 500,
                            2 => 2000,
                            _ => 8000,
                        }
                    }
                }
            }
            x if x == StoreItem::STORE_ITEM_POOL_CLEANER as i32 => 100,
            x if x == StoreItem::STORE_ITEM_ROOF_CLEANER as i32 => 300,
            x if x == StoreItem::STORE_ITEM_RAKE as i32 => 20,
            x if x == StoreItem::STORE_ITEM_AQUARIUM_GARDEN as i32 => 3000,
            x if x == StoreItem::STORE_ITEM_TREE_OF_WISDOM as i32 => 1000,
            x if x == StoreItem::STORE_ITEM_TREE_FOOD as i32 => 250,
            x if x == StoreItem::STORE_ITEM_FIRSTAID as i32 => 200,
            _ => 0,
        }
    }

    /// C++ StoreScreen::CanAffordItem (StoreScreen.cpp:924)
    pub unsafe fn CanAffordItem(&self, the_store_item: i32) -> bool {
        unsafe {
            if (*self.mApp).m_player_info.is_null() {
                return false;
            }
            (*(*self.mApp).m_player_info).mCoins >= self.GetItemCost(the_store_item) as u32
        }
    }

    /// C++ StoreScreen::PurchaseItem (StoreScreen.cpp:930)
    pub unsafe fn PurchaseItem(&mut self, the_store_item: i32) {
        // C++: SetCursor(POINTER); CrazyDaveStopTalking(); mBubbleCountDown = 0;
        self.mBubbleCountDown = 0;
        // [TODO]: mApp->CrazyDaveStopTalking()

        if !self.CanAffordItem(the_store_item) {
            // [TODO]: DoDialog(DIALOG_NOT_ENOUGH_MONEY) 钱不够提示
            return;
        }

        // [TODO]: DoDialog(DIALOG_STORE_PURCHASE) 确认购买 + WaitForResult(ID_OK)

        unsafe {
            if (*self.mApp).m_player_info.is_null() {
                return;
            }
            let a_player = &mut *(*self.mApp).m_player_info;
            // C++: AddCoins(-GetItemCost(theStoreItem))
            a_player.AddCoins(-self.GetItemCost(the_store_item));

            if the_store_item == StoreItem::STORE_ITEM_PACKET_UPGRADE as i32 {
                a_player.mPurchases[the_store_item as usize] += 1;
                // C++: 显示新种子槽数提示（6 + mPurchases）
                // [TODO]: DoDialog(DIALOG_UPGRADED) + mSeedBank->UpdateWidth()
            } else if the_store_item == StoreItem::STORE_ITEM_BONUS_LAWN_MOWER as i32 {
                a_player.mPurchases[the_store_item as usize] += 1;
            } else if the_store_item == StoreItem::STORE_ITEM_RAKE as i32 {
                // C++: 耙子 3 个
                a_player.mPurchases[the_store_item as usize] = 3;
            } else if the_store_item == StoreItem::STORE_ITEM_STINKY_THE_SNAIL as i32 {
                // C++: 记录购买时间
                a_player.mPurchases[the_store_item as usize] = crate::sexy_app_framework::sexy_app_base::sdl_get_ticks().max(1);
            } else if the_store_item == StoreItem::STORE_ITEM_FERTILIZER as i32
                || the_store_item == StoreItem::STORE_ITEM_BUG_SPRAY as i32
            {
                // C++: 化肥/杀虫剂批量购买（≥OFFSET 后 +5）
                if a_player.mPurchases[the_store_item as usize] < crate::lawn::system::player_info::PURCHASE_COUNT_OFFSET as u32 {
                    a_player.mPurchases[the_store_item as usize] = crate::lawn::system::player_info::PURCHASE_COUNT_OFFSET as u32;
                }
                a_player.mPurchases[the_store_item as usize] += 5;
            } else if the_store_item == StoreItem::STORE_ITEM_TREE_FOOD as i32 {
                // C++: 树肥批量购买
                if a_player.mPurchases[the_store_item as usize] < crate::lawn::system::player_info::PURCHASE_COUNT_OFFSET as u32 {
                    a_player.mPurchases[the_store_item as usize] = crate::lawn::system::player_info::PURCHASE_COUNT_OFFSET as u32;
                }
                a_player.mPurchases[the_store_item as usize] += 1;
            } else if self.IsPottedPlant(the_store_item) {
                // [TODO]: mApp->mZenGarden->AddPottedPlant 盆栽加入花园
                a_player.mPurchases[the_store_item as usize] += 1;
            } else {
                // C++: 植物/道具购买
                a_player.mPurchases[the_store_item as usize] += 1;
            }
        }

        // [TODO]: mApp->PlaySample(SOUND_BUTTONCLICK)
    }    pub fn Update(&mut self) {}
}

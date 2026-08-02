// [TRANSLATION_NOTE]: SeedChooserScreen.h + SeedChooserScreen.cpp -> Rust stub

#![allow(non_snake_case, dead_code)]

use crate::lawn_app::LawnApp;
use crate::const_enums::*;
use crate::lawn::widget::lawn_dialog::LawnDialog;
use crate::sexy_app_framework::graphics::graphics::Graphics;

/// C++: struct ChosenSeed — 已选种子
#[derive(Clone)]
pub struct ChosenSeed {
    pub mSeedType: SeedType,
    pub mSeedState: i32,
    pub mSeedPositionInChooser: i32,
}

pub struct SeedChooserScreen {
    pub base: LawnDialog,
    pub mApp: *mut LawnApp,
    pub mBoard: *mut crate::lawn::board::Board,
    pub mImitaterButton: *mut crate::lawn::widget::game_button::GameButton,
    pub mStartButton: *mut crate::lawn::widget::game_button::GameButton,
    pub mRandomButton: *mut crate::lawn::widget::game_button::GameButton,
    pub mViewLawnButton: *mut crate::lawn::widget::game_button::GameButton,
    pub mAlmanacButton: *mut crate::lawn::widget::game_button::GameButton,
    pub mStoreButton: *mut crate::lawn::widget::game_button::GameButton,
    pub mToolTip: *mut crate::lawn::tool_tip_widget::ToolTipWidget,
    pub mMenuButton: *mut crate::lawn::widget::game_button::GameButton,
    pub mSeedPackets: [crate::lawn::seed_packet::SeedPacket; 10],
    pub mChosenSeeds: Vec<ChosenSeed>,
    pub mNumChosenSeeds: i32,
    pub mStartButtonCounter: i32,
    pub mChosenSeedIndex: i32,
    pub mChooseMode: i32,
    pub mRepickWarningDialog: *mut crate::lawn::widget::lawn_dialog::LawnDialog,
}

impl SeedChooserScreen {
    pub fn new(theApp: *mut LawnApp) -> Self {
        SeedChooserScreen {
            base: LawnDialog::new(theApp, 0, true, "", "", "", 0),
            mApp: theApp,
            mBoard: std::ptr::null_mut(),
            mImitaterButton: std::ptr::null_mut(),
            mStartButton: std::ptr::null_mut(),
            mRandomButton: std::ptr::null_mut(),
            mViewLawnButton: std::ptr::null_mut(),
            mAlmanacButton: std::ptr::null_mut(),
            mStoreButton: std::ptr::null_mut(),
            mToolTip: std::ptr::null_mut(),
            mMenuButton: std::ptr::null_mut(),
            mSeedPackets: std::array::from_fn(|_| crate::lawn::seed_packet::SeedPacket::new()),
            mChosenSeeds: Vec::new(),
            mNumChosenSeeds: 0,
            mStartButtonCounter: 0,
            mChosenSeedIndex: -1,
            mChooseMode: 0,
            mRepickWarningDialog: std::ptr::null_mut(),
        }
    }
    pub fn Draw(&self, g: &mut Graphics) { self.base.Draw(g); }
    /// C++ SeedChooserScreen::Has7Rows (SeedChooserScreen.cpp:275)
    pub fn Has7Rows(&self) -> bool {
        // C++: HasFinishedAdventure() || mPurchases[STORE_ITEM_PLANT_GATLINGPEA]
        //      || 任何已解锁升级种子（TWINSUNFLOWER..COBCANNON 除 SPIKEROCK）
        unsafe {
            if (*self.mApp).HasFinishedAdventure() {
                return true;
            }
            if !(*self.mApp).m_player_info.is_null() {
                let the_player_info = &*(*self.mApp).m_player_info;
                if the_player_info.mPurchases[StoreItem::STORE_ITEM_PLANT_GATLINGPEA as usize] != 0 {
                    return true;
                }
            }
            let mut a_seed_type = SeedType::SEED_TWINSUNFLOWER as i32;
            while a_seed_type < SeedType::SEED_COBCANNON as i32 {
                if a_seed_type != SeedType::SEED_SPIKEROCK as i32 && (*self.mApp).HasSeedType(std::mem::transmute::<i32, SeedType>(a_seed_type)) {
                    return true;
                }
                a_seed_type += 1;
            }
        }
        false
    }

    /// C++ SeedChooserScreen::GetSeedPositionInChooser (SeedChooserScreen.cpp:284)
    pub fn GetSeedPositionInChooser(&self, the_index: i32, x: &mut i32, y: &mut i32) {
        if the_index == SeedType::SEED_IMITATER as i32 {
            unsafe {
                *x = (*self.mImitaterButton).mX;
                *y = (*self.mImitaterButton).mY;
            }
            return;
        }

        let a_row = the_index / 8;
        let a_col = the_index % 8;

        *x = a_col * 53 + 22;
        if self.Has7Rows() {
            *y = a_row * 70 + 123;
        } else {
            *y = a_row * 73 + 128;
        }
    }

    /// C++ SeedChooserScreen::GetSeedPositionInBank (SeedChooserScreen.cpp:308)
    pub fn GetSeedPositionInBank(&self, the_index: i32, x: &mut i32, y: &mut i32) {
        unsafe {
            let the_board = &*self.mBoard;
            let a_seed_bank = the_board.mSeedBank;
            let a_bank_x = if a_seed_bank.is_null() { 0 } else { (*a_seed_bank).mX };
            let a_bank_y = if a_seed_bank.is_null() { 0 } else { (*a_seed_bank).mY };
            *x = a_bank_x - self.base.base.mX + the_board.GetSeedPacketPositionX(the_index);
            *y = a_bank_y - self.base.base.mY + 8;
        }
    }

    /// C++ SeedChooserScreen::SeedNotAllowedToPick (SeedChooserScreen.cpp:334)
    pub fn SeedNotAllowedToPick(&self, the_seed_type: SeedType) -> bool {
        // C++: LAST_STAND 模式禁用产阳光植物
        unsafe {
            (*self.mApp).mGameMode as i32 == GameMode::GAMEMODE_CHALLENGE_LAST_STAND as i32
                && (the_seed_type == SeedType::SEED_SUNFLOWER
                    || the_seed_type == SeedType::SEED_SUNSHROOM
                    || the_seed_type == SeedType::SEED_TWINSUNFLOWER
                    || the_seed_type == SeedType::SEED_SEASHROOM
                    || the_seed_type == SeedType::SEED_PUFFSHROOM)
        }
    }

    /// C++ SeedChooserScreen::SeedNotAllowedDuringTrial (SeedChooserScreen.cpp:340)
    pub fn SeedNotAllowedDuringTrial(&self, the_seed_type: SeedType) -> bool {
        // C++: mApp->IsTrialStageLocked() && (SEED_SQUASH || SEED_THREEPEATER)
        // [TODO]: IsTrialStageLocked 完整翻译（试用版限制）
        false
            && (the_seed_type == SeedType::SEED_SQUASH || the_seed_type == SeedType::SEED_THREEPEATER)
    }

    /// C++ SeedChooserScreen::CheckSeedUpgrade (SeedChooserScreen.cpp:607)
    pub unsafe fn CheckSeedUpgrade(&self, the_seed_type_to: SeedType, the_seed_type_from: SeedType) -> bool {
        // C++: 生存模式或无冲突直接允许
        if (*self.mApp).is_survival_mode() || !self.PickedPlantType(the_seed_type_to) || self.PickedPlantType(the_seed_type_from) {
            return true;
        }

        // C++: 显示升级警告对话框
        // [TODO]: DisplayRepickWarningDialog（TodStringTranslate + TodReplaceString 后弹窗）
        true
    }

    /// C++ SeedChooserScreen::PickedPlantType — 是否已选该种子
    pub fn PickedPlantType(&self, the_seed_type: SeedType) -> bool {
        let mut i = 0;
        while i < self.mNumChosenSeeds {
            if self.mChosenSeeds[i as usize].mSeedType == the_seed_type {
                return true;
            }
            i += 1;
        }
        false
    }
    pub fn Update(&mut self) {}
}

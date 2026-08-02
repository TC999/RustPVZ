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
    pub mX: i32,
    pub mY: i32,
    pub mStartX: i32,
    pub mStartY: i32,
    pub mEndX: i32,
    pub mEndY: i32,
    pub mSeedIndexInBank: i32,
    pub mCrazyDavePicked: bool,
    pub mTimeStartMotion: i32,
    pub mTimeEndMotion: i32,
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
    pub mSeedsInBank: i32,
    pub mSeedsInFlight: i32,
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
            mSeedsInBank: 0,
            mSeedsInFlight: 0,
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

    /// C++ SeedChooserScreen::PickFromWeightedArrayUsingSpecialRandSeed (SeedChooserScreen.cpp:207)
    pub fn PickFromWeightedArrayUsingSpecialRandSeed(&self, the_array: &[crate::sexy_tod_lib::tod_common::TodWeightedArray], the_count: i32, the_level_rng: &mut crate::sexy_app_framework::misc::mtrand::MTRand) -> isize {
        let mut a_total_weight = 0;
        let mut i = 0;
        while i < the_count {
            a_total_weight += the_array[i as usize].m_weight;
            i += 1;
        }

        // C++: int aRndResult = theLevelRNG.Next(aTotalWeight);
        let a_rnd_result = the_level_rng.next() % a_total_weight as u32;

        let mut a_weight = 0;
        let mut j = 0;
        while j < the_count {
            a_weight += the_array[j as usize].m_weight;
            if a_weight > a_rnd_result as i32 {
                return the_array[j as usize].m_item;
            }
            j += 1;
        }
        0
    }

    /// C++ SeedChooserScreen::SeedNotRecommendedToPick (SeedChooserScreen.cpp:326)
    pub fn SeedNotRecommendedToPick(&self, the_seed_type: SeedType) -> u32 {
        // C++: aRecFlags = mBoard->SeedNotRecommendedForLevel(theSeedType);
        // [TODO]: SeedNotRecommendedForLevel 完整翻译
        let a_rec_flags: u32 = 0;
        // C++: if (TestBit(aRecFlags, NOT_RECOMMENDED_NOCTURNAL) && PickedPlantType(SEED_INSTANT_COFFEE))
        // C++:     SetBit(aRecFlags, NOT_RECOMMENDED_NOCTURNAL, false);
        let _ = the_seed_type;
        a_rec_flags
    }

    /// C++ SeedChooserScreen::CrazyDavePickSeeds (SeedChooserScreen.cpp:225) — 疯狂戴夫选种
    pub unsafe fn CrazyDavePickSeeds(&mut self) {
        let mut a_seed_array: [crate::sexy_tod_lib::tod_common::TodWeightedArray; 100] = [crate::sexy_tod_lib::tod_common::TodWeightedArray { m_item: 0, m_weight: 0 }; 100];

        // C++: 遍历 PEASHOOTER..NUM_SEEDS_IN_CHOOSER 设置权重
        let mut a_seed_type = SeedType::SEED_PEASHOOTER as i32;
        while a_seed_type < NUM_SEEDS_IN_CHOOSER {
            let the_seed_type = std::mem::transmute::<i32, SeedType>(a_seed_type);
            a_seed_array[a_seed_type as usize].m_item = a_seed_type as isize;
            if !(*self.mApp).HasSeedType(the_seed_type)
                || self.SeedNotRecommendedToPick(the_seed_type) != 0
                || self.SeedNotAllowedToPick(the_seed_type)
                || crate::lawn::plant::Plant::is_upgrade(the_seed_type)
                || the_seed_type == SeedType::SEED_IMITATER
                || the_seed_type == SeedType::SEED_UMBRELLA
                || the_seed_type == SeedType::SEED_BLOVER
            {
                a_seed_array[a_seed_type as usize].m_weight = 0;
            } else {
                a_seed_array[a_seed_type as usize].m_weight = 1;
            }
            a_seed_type += 1;
        }

        // C++: 蹦极/投石车关卡解锁伞
        let the_board = &mut *self.mBoard;
        if the_board.mZombieAllowed[ZombieType::ZOMBIE_BUNGEE as usize]
            || the_board.mZombieAllowed[ZombieType::ZOMBIE_CATAPULT as usize]
        {
            a_seed_array[SeedType::SEED_UMBRELLA as usize].m_weight = 1;
        }
        // C++: 气球/雾关卡解锁三叶草
        if the_board.mZombieAllowed[ZombieType::ZOMBIE_BALLOON as usize] || the_board.StageHasFog() {
            a_seed_array[SeedType::SEED_BLOVER as usize].m_weight = 1;
        }
        // C++: 屋顶关卡禁用火炬树桩
        if the_board.StageHasRoof() {
            a_seed_array[SeedType::SEED_TORCHWOOD as usize].m_weight = 0;
        }

        // C++: MTRand aLevelRNG = MTRand(mBoard->GetLevelRandSeed());
        let mut a_level_rng = crate::sexy_app_framework::misc::mtrand::MTRand::new();
        a_level_rng.srand_u32(the_board.GetLevelRandSeed() as u32);

        // C++: 选 3 个种子放入银行
        let mut i = 0;
        while i < 3 {
            let a_picked_seed = std::mem::transmute::<i32, SeedType>(self.PickFromWeightedArrayUsingSpecialRandSeed(&a_seed_array, NUM_SEEDS_IN_CHOOSER, &mut a_level_rng) as i32);
            a_seed_array[a_picked_seed as usize].m_weight = 0;
            let a_chosen_seed = &mut self.mChosenSeeds[a_picked_seed as usize];

            let a_pos_x = the_board.GetSeedPacketPositionX(i);
            a_chosen_seed.mX = a_pos_x;
            a_chosen_seed.mY = 8;
            a_chosen_seed.mStartX = a_pos_x;
            a_chosen_seed.mStartY = 8;
            a_chosen_seed.mEndX = a_pos_x;
            a_chosen_seed.mEndY = 8;
            a_chosen_seed.mSeedState = ChosenSeedState::SEED_IN_BANK as i32;
            a_chosen_seed.mSeedIndexInBank = i;
            a_chosen_seed.mCrazyDavePicked = true;
            self.mSeedsInBank += 1;
            i += 1;
        }
    }

    /// C++ SeedChooserScreen::LandFlyingSeed (SeedChooserScreen.cpp:476)
    pub unsafe fn LandFlyingSeed(&mut self, the_chosen_seed: &mut ChosenSeed) {
        if the_chosen_seed.mSeedState == ChosenSeedState::SEED_FLYING_TO_BANK as i32 {
            the_chosen_seed.mX = the_chosen_seed.mEndX;
            the_chosen_seed.mY = the_chosen_seed.mEndY;
            the_chosen_seed.mTimeStartMotion = 0;
            the_chosen_seed.mTimeEndMotion = 0;
            the_chosen_seed.mSeedState = ChosenSeedState::SEED_IN_BANK as i32;
            self.mSeedsInFlight -= 1;
        } else if the_chosen_seed.mSeedState == ChosenSeedState::SEED_FLYING_TO_CHOOSER as i32 {
            the_chosen_seed.mX = the_chosen_seed.mEndX;
            the_chosen_seed.mY = the_chosen_seed.mEndY;
            the_chosen_seed.mTimeStartMotion = 0;
            the_chosen_seed.mTimeEndMotion = 0;
            the_chosen_seed.mSeedState = ChosenSeedState::SEED_IN_CHOOSER as i32;
            self.mSeedsInFlight -= 1;
            if the_chosen_seed.mSeedType == SeedType::SEED_IMITATER {
                the_chosen_seed.mSeedState = ChosenSeedState::SEED_PACKET_HIDDEN as i32;
                // C++: theChosenSeed.mImitaterType = SEED_NONE;
                // [TODO]: mImitaterType 字段 + UpdateImitaterButton()
            }
        }
    }

    /// C++ SeedChooserScreen::FlyersAreComming (SeedChooserScreen.cpp:577) — 关卡是否有气球僵尸
    pub unsafe fn FlyersAreComming(&self) -> bool {
        let the_board = &*self.mBoard;
        let mut a_wave = 0;
        while a_wave < the_board.mNumWaves {
            let mut an_index = 0;
            while an_index < crate::lawn::board_consts::MAX_ZOMBIES_IN_WAVE {
                let a_zombie_type = the_board.mZombiesInWave[a_wave as usize][an_index as usize];
                if a_zombie_type == ZombieType::ZOMBIE_INVALID as i32 {
                    break;
                }
                if a_zombie_type == ZombieType::ZOMBIE_BALLOON as i32 {
                    return true;
                }
                an_index += 1;
            }
            a_wave += 1;
        }
        false
    }

    /// C++ SeedChooserScreen::FlyProtectionCurrentlyPlanted (SeedChooserScreen.cpp:594) — 是否已种防空植物
    pub unsafe fn FlyProtectionCurrentlyPlanted(&self) -> bool {
        let the_board = &*self.mBoard;
        let mut a_plant: *mut crate::lawn::plant::Plant = std::ptr::null_mut();
        while the_board.IteratePlants(&mut a_plant) {
            if (*a_plant).m_seed_type == SeedType::SEED_CATTAIL || (*a_plant).m_seed_type == SeedType::SEED_CACTUS {
                return true;
            }
        }
        false
    }

    /// C++ SeedChooserScreen::CloseSeedChooser — 关闭选种界面
    pub unsafe fn CloseSeedChooser(&mut self) {
        // [TODO]: 关闭界面并开始关卡
    }

    /// C++ SeedChooserScreen::PickRandomSeeds (SeedChooserScreen.cpp:709)
    pub unsafe fn PickRandomSeeds(&mut self) {
        // C++: 为剩余种子包随机选种
        let the_board = &mut *self.mBoard;
        let a_num_packets = if the_board.mSeedBank.is_null() { 0 } else { (*the_board.mSeedBank).mNumPackets };
        let mut an_index = self.mSeedsInBank;
        while an_index < a_num_packets {
            let a_seed_type: SeedType = loop {
                // C++: aSeedType = Rand(mApp->GetSeedsAvailable())
                let a_try_seed = std::mem::transmute::<i32, SeedType>(crate::sexy_app_framework::common::rand_int() % (*self.mApp).GetSeedsAvailable());
                if !(*self.mApp).HasSeedType(a_try_seed)
                    || a_try_seed == SeedType::SEED_IMITATER
                    || self.mChosenSeeds[a_try_seed as usize].mSeedState != ChosenSeedState::SEED_IN_CHOOSER as i32
                {
                    continue;
                }
                break a_try_seed;
            };
            let mut a_chosen_seed = self.mChosenSeeds[a_seed_type as usize].clone();
            a_chosen_seed.mTimeStartMotion = 0;
            a_chosen_seed.mTimeEndMotion = 0;
            a_chosen_seed.mStartX = a_chosen_seed.mX;
            a_chosen_seed.mStartY = a_chosen_seed.mY;
            let mut a_end_x = a_chosen_seed.mEndX;
            let mut a_end_y = a_chosen_seed.mEndY;
            self.GetSeedPositionInBank(an_index, &mut a_end_x, &mut a_end_y);
            a_chosen_seed.mEndX = a_end_x;
            a_chosen_seed.mEndY = a_end_y;
            a_chosen_seed.mSeedState = ChosenSeedState::SEED_IN_BANK as i32;
            a_chosen_seed.mSeedIndexInBank = an_index;
            self.mChosenSeeds[a_seed_type as usize] = a_chosen_seed;
            self.mSeedsInBank += 1;
            an_index += 1;
        }

        // C++: 落地所有飞行种子 + 关闭
        let mut a_seed_flying = SeedType::SEED_PEASHOOTER as i32;
        while a_seed_flying < NUM_SEEDS_IN_CHOOSER {
            let mut seed = self.mChosenSeeds[a_seed_flying as usize].clone();
            self.LandFlyingSeed(&mut seed);
            self.mChosenSeeds[a_seed_flying as usize] = seed;
            a_seed_flying += 1;
        }
        self.CloseSeedChooser();
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

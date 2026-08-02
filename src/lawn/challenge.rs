// [TRANSLATION_NOTE]: Challenge.h -> Rust stub
// board.rs 使用 mSurvivalStage 字段，保持可用

#![allow(non_snake_case, dead_code)]

use crate::const_enums::*;

pub const BEGHOULED_MAX_GRIDSIZEX: i32 = 8;
pub const BEGHOULED_MAX_GRIDSIZEY: i32 = 5;
pub const BEGHOULED_WINNING_SCORE: i32 = 75;
pub const SLOT_MACHINE_WINNING_SCORE: i32 = 2000;
pub const ZOMBIQUARIUM_WINNING_SCORE: i32 = 1000;
pub const I_ZOMBIE_WINNING_SCORE: i32 = 5;
pub const MAX_PORTALS: i32 = 4;
pub const MAX_SQUIRRELS: i32 = 7;
pub const MAX_SCARY_POTS: i32 = 54;
pub const STORM_FLASH_TIME: i32 = 150;
pub const MAX_PICK_GRID_SIZE: i32 = 50;

pub const NUM_BEGHOULED_UPGRADES: i32 = 3;


/// C++: struct BeghouledBoardState — 宝石迷阵棋盘状态
pub struct BeghouledBoardState {
    pub m_seed_type: [[SeedType; 6]; 9],
}

impl BeghouledBoardState {
    pub fn new() -> Self {
        BeghouledBoardState {
            m_seed_type: [[SeedType::SEED_NONE; 6]; 9],
        }
    }
}

/// C++: enum BeghouledUpgrade
pub const BEGHOULED_UPGRADE_REPEATER: i32 = 0;
pub const BEGHOULED_UPGRADE_FUMESHROOM: i32 = 1;
pub const BEGHOULED_UPGRADE_TALLNUT: i32 = 2;
pub struct Challenge {
    pub mApp: *mut crate::lawn_app::LawnApp,
    pub mBoard: *mut crate::lawn::board::Board,
    pub mSurvivalStage: i32,
    pub mChallengeState: ChallengeState,
    pub mChallengeStateCounter: i32,
    pub mConveyorBeltCounter: i32,
    pub mChallengeScore: i32,
    pub mLastConveyorSeedType: SeedType,
    pub mReanimChallenge: ReanimationID,
    pub mChallengeGridX: i32,
    pub mChallengeGridY: i32,
    // C++ Challenge.h — Beghouled 相关字段
    pub m_beghouled_eated: [[i32; 6]; 9],
    pub m_beghouled_purcased_upgrade: [i32; 3],
    pub m_beghouled_mouse_capture: bool,
    pub m_beghouled_mouse_down_x: i32,
    pub m_beghouled_mouse_down_y: i32,
    pub m_beghouled_matches_this_move: i32,
    pub m_scary_potter_pots: i32,
}

impl Challenge {
    pub fn new() -> Self {
        Challenge {
            mApp: std::ptr::null_mut(),
            mBoard: std::ptr::null_mut(),
            mSurvivalStage: 0,
            mChallengeState: ChallengeState::STATECHALLENGE_NORMAL,
            mChallengeStateCounter: 0,
            mConveyorBeltCounter: 0,
            mChallengeScore: 0,
            mLastConveyorSeedType: SeedType::SEED_NONE,
            mReanimChallenge: ReanimationID::REANIMATIONID_NULL,
            mChallengeGridX: -1,
            mChallengeGridY: -1,
            m_beghouled_eated: [[0i32; 6]; 9],
            m_beghouled_purcased_upgrade: [0i32; 3],
            m_beghouled_mouse_capture: false,
            m_beghouled_mouse_down_x: 0,
            m_beghouled_mouse_down_y: 0,
            m_beghouled_matches_this_move: 0,
            m_scary_potter_pots: 0,
        }
    }

    // Helper
    unsafe fn board(&self) -> &'static mut crate::lawn::board::Board {
        &mut *(self.mBoard)
    }
    unsafe fn app(&self) -> &'static mut crate::lawn_app::LawnApp {
        &mut *(self.mApp)
    }

    // =========================================================================
    // ★ Beghouled（宝石迷阵）棋盘逻辑 — C++ Challenge.cpp 保真翻译
    // =========================================================================

    /// C++: Challenge::LoadBeghouledBoardState (Challenge.cpp:349)
    pub unsafe fn LoadBeghouledBoardState(&self, the_board_state: &mut BeghouledBoardState) {
        // C++: 清空棋盘
        for i in 0..crate::lawn::board_consts::MAX_GRID_SIZE_X {
            for j in 0..crate::lawn::board_consts::MAX_GRID_SIZE_Y {
                the_board_state.m_seed_type[i as usize][j as usize] = SeedType::SEED_NONE;
            }
        }

        // C++: 从 Board 现有植物填充棋盘
        let the_board = &mut *self.mBoard;
        let mut a_plant: *mut crate::lawn::plant::Plant = std::ptr::null_mut();
        while the_board.IteratePlants(&mut a_plant) {
            the_board_state.m_seed_type[(*a_plant).m_plant_col as usize][(*a_plant).base.m_row as usize] = (*a_plant).m_seed_type;
        }
    }

    /// C++: Challenge::BeghouledGetPlantAt (Challenge.cpp:746)
    pub unsafe fn BeghouledGetPlantAt(&self, the_grid_x: i32, the_grid_y: i32, the_board_state: &BeghouledBoardState) -> SeedType {
        if the_grid_x < 0 || the_grid_x > BEGHOULED_MAX_GRIDSIZEX || the_grid_y < 0 || the_grid_y > BEGHOULED_MAX_GRIDSIZEY {
            return SeedType::SEED_NONE;
        }
        the_board_state.m_seed_type[the_grid_x as usize][the_grid_y as usize]
    }

    /// C++: Challenge::BeghouledRemoveHorizontalMatch (Challenge.cpp:754)
    pub unsafe fn BeghouledRemoveHorizontalMatch(&mut self, mut the_grid_x: i32, the_grid_y: i32, the_board_state: &mut BeghouledBoardState) {
        let a_seed_type = self.BeghouledGetPlantAt(the_grid_x, the_grid_y, the_board_state);
        loop {
            let the_board = &mut *self.mBoard;
            let a_plant = the_board.GetTopPlantAt(the_grid_x, the_grid_y, PlantPriority::TOPPLANT_ANY);
            if !a_plant.is_null() {
                (*a_plant).Die();
            }
            the_grid_x += 1;
            if self.BeghouledGetPlantAt(the_grid_x, the_grid_y, the_board_state) != a_seed_type {
                break;
            }
        }
    }

    /// C++: Challenge::BeghouledRemoveVerticalMatch (Challenge.cpp:769)
    pub unsafe fn BeghouledRemoveVerticalMatch(&mut self, the_grid_x: i32, mut the_grid_y: i32, the_board_state: &mut BeghouledBoardState) {
        let a_seed_type = self.BeghouledGetPlantAt(the_grid_x, the_grid_y, the_board_state);
        loop {
            let the_board = &mut *self.mBoard;
            let a_plant = the_board.GetTopPlantAt(the_grid_x, the_grid_y, PlantPriority::TOPPLANT_ANY);
            if !a_plant.is_null() {
                (*a_plant).Die();
            }
            the_grid_y += 1;
            if self.BeghouledGetPlantAt(the_grid_x, the_grid_y, the_board_state) != a_seed_type {
                break;
            }
        }
    }

    /// C++: Challenge::BeghouledFallIntoSquare (Challenge.cpp:783)
    pub unsafe fn BeghouledFallIntoSquare(&mut self, the_grid_x: i32, the_grid_y: i32, the_board_state: &mut BeghouledBoardState) {
        if self.m_beghouled_eated[the_grid_x as usize][the_grid_y as usize] != 0 {
            return;
        }

        let mut a_grid_y = the_grid_y - 1;
        while a_grid_y >= 0 {
            let the_board = &mut *self.mBoard;
            let a_plant = the_board.GetTopPlantAt(the_grid_x, a_grid_y, PlantPriority::TOPPLANT_ANY);
            if !a_plant.is_null() {
                // C++: aPlant->mRow = theGridY; aPlant->mRenderOrder = aPlant->CalcRenderOrder();
                (*a_plant).base.m_row = the_grid_y;
                // [TODO]: CalcRenderOrder
                the_board_state.m_seed_type[the_grid_x as usize][the_grid_y as usize] = (*a_plant).m_seed_type;
                the_board_state.m_seed_type[the_grid_x as usize][a_grid_y as usize] = SeedType::SEED_NONE;
                self.BeghouledStartFalling(ChallengeState::STATECHALLENGE_BEGHOULED_FALLING);
                break;
            }
            a_grid_y -= 1;
        }
    }

    /// C++: Challenge::BeghouledMakePlantsFall (Challenge.cpp:803)
    pub unsafe fn BeghouledMakePlantsFall(&mut self, the_board_state: &mut BeghouledBoardState) {
        let mut a_grid_y = BEGHOULED_MAX_GRIDSIZEY - 1;
        while a_grid_y >= 0 {
            let mut a_grid_x = 0;
            while a_grid_x < BEGHOULED_MAX_GRIDSIZEX {
                if self.BeghouledGetPlantAt(a_grid_x, a_grid_y, the_board_state) == SeedType::SEED_NONE {
                    self.BeghouledFallIntoSquare(a_grid_x, a_grid_y, the_board_state);
                }
                a_grid_x += 1;
            }
            a_grid_y -= 1;
        }
    }

    /// C++: Challenge::BeghouledHorizontalMatchLength (Challenge.cpp:941)
    pub unsafe fn BeghouledHorizontalMatchLength(&self, the_grid_x: i32, the_grid_y: i32, the_board_state: &BeghouledBoardState) -> i32 {
        let a_seed_type = self.BeghouledGetPlantAt(the_grid_x, the_grid_y, the_board_state);
        if a_seed_type == SeedType::SEED_NONE
            || self.BeghouledGetPlantAt(the_grid_x - 1, the_grid_y, the_board_state) == a_seed_type
        {
            return 0;
        }

        let mut a_length = 1;
        while self.BeghouledGetPlantAt(the_grid_x + a_length, the_grid_y, the_board_state) == a_seed_type {
            a_length += 1;
        }
        a_length
    }

    /// C++: Challenge::BeghouledVerticalMatchLength (Challenge.cpp:953)
    pub unsafe fn BeghouledVerticalMatchLength(&self, the_grid_x: i32, the_grid_y: i32, the_board_state: &BeghouledBoardState) -> i32 {
        let a_seed_type = self.BeghouledGetPlantAt(the_grid_x, the_grid_y, the_board_state);
        if a_seed_type == SeedType::SEED_NONE
            || self.BeghouledGetPlantAt(the_grid_x, the_grid_y - 1, the_board_state) == a_seed_type
        {
            return 0;
        }

        let mut a_length = 1;
        while self.BeghouledGetPlantAt(the_grid_x, the_grid_y + a_length, the_board_state) == a_seed_type {
            a_length += 1;
        }
        a_length
    }

    /// C++: Challenge::BeghouledBoardHasMatch (Challenge.cpp:965)
    pub unsafe fn BeghouledBoardHasMatch(&self, the_board_state: &BeghouledBoardState) -> bool {
        let mut a_col = 0;
        while a_col < 8 {
            let mut a_row = 0;
            while a_row < 5 {
                if self.BeghouledHorizontalMatchLength(a_col, a_row, the_board_state) >= 3
                    || self.BeghouledVerticalMatchLength(a_col, a_row, the_board_state) >= 3
                {
                    return true;
                }
                a_row += 1;
            }
            a_col += 1;
        }
        false
    }

    /// C++: Challenge::BeghouledPickSeed (Challenge.cpp:979)
    pub unsafe fn BeghouledPickSeed(&mut self, the_grid_x: i32, the_grid_y: i32, the_board_state: &mut BeghouledBoardState, the_allow_matches: bool) -> SeedType {
        let mut a_count = 0;
        let mut a_pick_array: [SeedType; 6] = [SeedType::SEED_NONE; 6];

        let mut i = 0;
        while i < 6 {
            let mut a_seed_type = match i {
                0 => SeedType::SEED_PUFFSHROOM,
                1 => SeedType::SEED_STARFRUIT,
                2 => SeedType::SEED_MAGNETSHROOM,
                3 => SeedType::SEED_SNOWPEA,
                4 => SeedType::SEED_WALLNUT,
                5 => SeedType::SEED_PEASHOOTER,
                _ => SeedType::SEED_NONE,
            };

            // C++: 购买升级替换种子
            if self.m_beghouled_purcased_upgrade[BEGHOULED_UPGRADE_REPEATER as usize] != 0 && a_seed_type == SeedType::SEED_PEASHOOTER {
                a_seed_type = SeedType::SEED_REPEATER;
            }
            if self.m_beghouled_purcased_upgrade[BEGHOULED_UPGRADE_FUMESHROOM as usize] != 0 && a_seed_type == SeedType::SEED_PUFFSHROOM {
                a_seed_type = SeedType::SEED_FUMESHROOM;
            }
            if self.m_beghouled_purcased_upgrade[BEGHOULED_UPGRADE_TALLNUT as usize] != 0 && a_seed_type == SeedType::SEED_WALLNUT {
                a_seed_type = SeedType::SEED_TALLNUT;
            }

            the_board_state.m_seed_type[the_grid_x as usize][the_grid_y as usize] = a_seed_type;

            if the_allow_matches || !self.BeghouledBoardHasMatch(the_board_state) {
                a_pick_array[a_count as usize] = a_seed_type;
                a_count += 1;
            }
            i += 1;
        }

        the_board_state.m_seed_type[the_grid_x as usize][the_grid_y as usize] = SeedType::SEED_NONE;
        // C++: return TodPickFromArray(aPickArray, aCount);
        crate::sexy_tod_lib::tod_common::tod_pick_from_array(&a_pick_array[..a_count as usize])
    }

    /// C++: Challenge::BeghouledFillHoles (Challenge.cpp:1026)
    pub unsafe fn BeghouledFillHoles(&mut self, the_board_state: &mut BeghouledBoardState, the_allow_matches: bool) {
        let mut a_col = 0;
        while a_col < BEGHOULED_MAX_GRIDSIZEX {
            let mut a_row = 0;
            while a_row < BEGHOULED_MAX_GRIDSIZEY {
                if the_board_state.m_seed_type[a_col as usize][a_row as usize] == SeedType::SEED_NONE
                    && self.m_beghouled_eated[a_col as usize][a_row as usize] == 0
                {
                    let a_seed = self.BeghouledPickSeed(a_col, a_row, the_board_state, the_allow_matches);
                    the_board_state.m_seed_type[a_col as usize][a_row as usize] = a_seed;
                }
                a_row += 1;
            }
            a_col += 1;
        }
    }

    /// C++: Challenge::BeghouledCreatePlants (Challenge.cpp:1040)
    pub unsafe fn BeghouledCreatePlants(&mut self, the_old_board_state: &BeghouledBoardState, the_new_board_state: &BeghouledBoardState) {
        let mut a_col = 0;
        while a_col < BEGHOULED_MAX_GRIDSIZEX {
            let mut a_fall_y = 80;
            let mut a_row = BEGHOULED_MAX_GRIDSIZEY - 1;
            while a_row >= 0 {
                let a_seed_type = the_new_board_state.m_seed_type[a_col as usize][a_row as usize];
                if the_old_board_state.m_seed_type[a_col as usize][a_row as usize] == SeedType::SEED_NONE && a_seed_type != SeedType::SEED_NONE {
                    a_fall_y -= 100;
                    let the_board = &mut *self.mBoard;
                    let a_plant = the_board.NewPlant(a_col, a_row, a_seed_type as i32, SeedType::SEED_NONE as i32);
                    if !a_plant.is_null() {
                        (*a_plant).base.m_y = a_fall_y;
                    }
                    self.BeghouledStartFalling(ChallengeState::STATECHALLENGE_BEGHOULED_FALLING);
                }
                a_row -= 1;
            }
            a_col += 1;
        }
    }

    /// C++: Challenge::BeghouledMakeStartBoard (Challenge.cpp:1058)
    pub unsafe fn BeghouledMakeStartBoard(&mut self) {
        let mut a_empty_board_state = BeghouledBoardState::new();
        self.LoadBeghouledBoardState(&mut a_empty_board_state);
        let mut a_board_state = BeghouledBoardState::new();
        self.LoadBeghouledBoardState(&mut a_board_state);

        self.BeghouledFillHoles(&mut a_board_state, false);
        if !self.BeghouledBoardHasMatch(&a_board_state) {
            self.BeghouledCreatePlants(&a_empty_board_state, &a_board_state);
        }
    }

    /// C++: Challenge::BeghouledPopulateBoard (Challenge.cpp:1074)
    pub unsafe fn BeghouledPopulateBoard(&mut self) {
        let mut a_empty_board_state = BeghouledBoardState::new();
        self.LoadBeghouledBoardState(&mut a_empty_board_state);
        let a_allow_generated_cascades = self.BeghouledBoardHasMatch(&a_empty_board_state);

        let mut a_board_state = BeghouledBoardState::new();
        let mut i = 0;
        while i < 2 {
            self.LoadBeghouledBoardState(&mut a_board_state);
            self.BeghouledFillHoles(&mut a_board_state, a_allow_generated_cascades);
            // C++: 填充后若存在可消除的移动则跳出
            if self.BeghouledCheckForPossibleMoves(&mut a_board_state) {
                break;
            }
            i += 1;
        }

        self.BeghouledCreatePlants(&a_empty_board_state, &a_board_state);
    }

    /// C++: Challenge::BeghouledCheckForPossibleMoves (Challenge.cpp:1093)
    pub unsafe fn BeghouledCheckForPossibleMoves(&mut self, the_board_state: &mut BeghouledBoardState) -> bool {
        let a_game_mode = (*self.mApp).mGameMode;

        let mut a_row = 0;
        while a_row < BEGHOULED_MAX_GRIDSIZEY {
            let mut a_col = 0;
            while a_col < BEGHOULED_MAX_GRIDSIZEX {
                if a_game_mode == GameMode::GAMEMODE_CHALLENGE_BEGHOULED {
                    if self.BeghouledIsValidMove(a_col, a_row, a_col + 1, a_row, the_board_state)
                        || self.BeghouledIsValidMove(a_col, a_row, a_col, a_row + 1, the_board_state)
                    {
                        return true;
                    }
                } else if a_game_mode == GameMode::GAMEMODE_CHALLENGE_BEGHOULED_TWIST {
                    // [TODO]: BeghouledTwistMoveCausesMatch
                    return true;
                } else {
                    return false;
                }
                a_col += 1;
            }
            a_row += 1;
        }

        false
    }

    /// C++: Challenge::BeghouledIsValidMove (Challenge.cpp:662)
    pub unsafe fn BeghouledIsValidMove(&mut self, the_from_x: i32, the_from_y: i32, the_to_x: i32, the_to_y: i32, the_board_state: &mut BeghouledBoardState) -> bool {
        if the_from_x < 0 || the_from_x > BEGHOULED_MAX_GRIDSIZEX || the_to_x < 0 || the_to_x > BEGHOULED_MAX_GRIDSIZEX
            || the_from_y < 0 || the_from_y > BEGHOULED_MAX_GRIDSIZEY || the_to_y < 0 || the_to_y > BEGHOULED_MAX_GRIDSIZEY
            || self.m_beghouled_eated[the_from_x as usize][the_from_y as usize] != 0
            || self.m_beghouled_eated[the_to_x as usize][the_to_y as usize] != 0
        {
            return false;
        }

        let a_seed_from = the_board_state.m_seed_type[the_from_x as usize][the_from_y as usize];
        let a_seed_to = the_board_state.m_seed_type[the_to_x as usize][the_to_y as usize];
        if a_seed_from == SeedType::SEED_NONE {
            return false;
        }

        // C++: 交换后检查是否产生匹配，然后恢复
        the_board_state.m_seed_type[the_from_x as usize][the_from_y as usize] = a_seed_to;
        the_board_state.m_seed_type[the_to_x as usize][the_to_y as usize] = a_seed_from;

        let a_valid = self.BeghouledBoardHasMatch(the_board_state);

        the_board_state.m_seed_type[the_from_x as usize][the_from_y as usize] = a_seed_from;
        the_board_state.m_seed_type[the_to_x as usize][the_to_y as usize] = a_seed_to;

        a_valid
    }

    /// C++: Challenge::BeghouledStartFalling (Challenge.cpp:655)
    pub unsafe fn BeghouledStartFalling(&mut self, _the_state: ChallengeState) {
        // C++: 状态切换 + 掉落动画
        self.mChallengeState = ChallengeState::STATECHALLENGE_BEGHOULED_FALLING;
        // [TODO]: mChallengeStateCounter / 掉落动画
    }
    /// C++ Challenge::InitLevel() (from Challenge.cpp:360)
    pub unsafe fn InitLevel(&mut self) {
        let app = self.app();
        let board = self.board();

        if app.mGameMode as i32 == GameMode::GAMEMODE_CHALLENGE_RAINING_SEEDS as i32 {
            self.mChallengeStateCounter = 100;
            // app->PlayFoley(FOLEY_RAIN);
        }
        if (*app).IsStormyNightLevel() {
            self.mChallengeState = ChallengeState::STATECHALLENGE_STORM_FLASH_2;
            self.mChallengeStateCounter = 100;
            // app->PlayFoley(FOLEY_RAIN);
        }
        if (*app).IsFinalBossLevel() {
            // board->mSeedBank->AddSeed(SEED_CABBAGEPULT);
            // board->mSeedBank->AddSeed(SEED_JALAPENO);
            // board->mSeedBank->AddSeed(SEED_CABBAGEPULT);
            // board->mSeedBank->AddSeed(SEED_ICESHROOM);
            self.mConveyorBeltCounter = 1000;
        }
        if app.mGameMode as i32 == GameMode::GAMEMODE_CHALLENGE_ZEN_GARDEN as i32 {
            // app->mZenGarden->mGardenType = GARDEN_MAIN;
            // app->mZenGarden->ZenGardenInitLevel();
        }
        if app.mGameMode as i32 == GameMode::GAMEMODE_CHALLENGE_COLUMN as i32 {
            // board->mSeedBank->AddSeed(SEED_POTATOMINE);
            // board->mSeedBank->AddSeed(SEED_TALLNUT);
            // board->mSeedBank->AddSeed(SEED_MELONPULT);
            // board->mSeedBank->AddSeed(SEED_MAGNETSHROOM);
            // board->mSeedBank->AddSeed(SEED_INSTANT_COFFEE);
            // board->mSeedBank->AddSeed(SEED_MELONPULT);
            self.mConveyorBeltCounter = 1000;
        }
        if app.mGameMode as i32 == GameMode::GAMEMODE_CHALLENGE_INVISIGHOUL as i32 {
            // board->mSeedBank->AddSeed(SEED_PEASHOOTER);
            // board->mSeedBank->AddSeed(SEED_ICESHROOM);
            self.mConveyorBeltCounter = 1000;
        }
        if (*app).IsIZombieLevel() {
            // IZombieInitLevel();
        }
        if (*app).IsScaryPotterLevel() {
            // ScaryPotterPopulate();
        }
        if (*app).IsFirstTimeAdventureMode() && board.mLevel == 5 {
            // board->NewPlant(5, 1, SEED_PEASHOOTER, SEED_NONE);
            // board->NewPlant(7, 2, SEED_PEASHOOTER, SEED_NONE);
            // board->NewPlant(6, 3, SEED_PEASHOOTER, SEED_NONE);
        }
        if app.mGameMode as i32 == GameMode::GAMEMODE_CHALLENGE_BEGHOULED_TWIST as i32 {
            self.mChallengeGridX = -1;
            self.mChallengeGridY = -1;
        }
        // Tree of wisdom init
    }

    /// C++ Challenge::StartLevel() (from Challenge.cpp:427)
    pub unsafe fn StartLevel(&mut self) {
        let app = self.app();
        let _board = self.board();

        if (*app).IsWhackAZombieLevel() {
            // board->mCursorObject->mCursorType = CURSOR_TYPE_HAMMER;
            // board->mZombieCountDown = 200;
            // Reanimation loading...
        }
        if (*app).is_wallnut_bowling_level() {
            // board->mZombieCountDown = 400;
        }
    }

    /// C++ Challenge::Update() (from Challenge.cpp:2128)
    pub unsafe fn Update(&mut self) {
        let app = self.app();
        let board = self.board();

        if (*app).IsStormyNightLevel() {
            // self.UpdateStormyNight();
        }

        if board.mPaused {
            if app.mGameMode as i32 == GameMode::GAMEMODE_CHALLENGE_BEGHOULED_TWIST as i32 {
                self.mChallengeGridX = -1;
                self.mChallengeGridY = -1;
            }
            return;
        }

        if app.mGameMode as i32 == GameMode::GAMEMODE_CHALLENGE_RAINING_SEEDS as i32
            || (*app).IsStormyNightLevel()
        {
            // self.UpdateRain();
        }

        if (*app).mGameScene as i32 != GameScenes::SCENE_PLAYING as i32
            && app.mGameMode as i32 != GameMode::GAMEMODE_TREE_OF_WISDOM as i32
        {
            return;
        }

        if board.HasConveyorBeltSeedBank() {
            self.UpdateConveyorBelt();
        }

        // C++ Challenge.cpp:2155 — Beghouled 更新
        if app.mGameMode as i32 == GameMode::GAMEMODE_CHALLENGE_BEGHOULED as i32
            || app.mGameMode as i32 == GameMode::GAMEMODE_CHALLENGE_BEGHOULED_TWIST as i32
        {
            self.UpdateBeghouled();
        }

        // C++: 恐怖罐更新
        if (*app).IsScaryPotterLevel() {
            self.ScaryPotterUpdate();
        }

        // C++: 恐怖罐/打地鼠关卡中种子银行从屏幕下方滑入
        if (*app).IsScaryPotterLevel() || (*app).IsWhackAZombieLevel() {
            let a_seed_bank = board.mSeedBank;
            if !a_seed_bank.is_null() && (*a_seed_bank).mY < 0 {
                if board.mSunMoney + board.CountSunBeingCollected() > 0
                    || (*a_seed_bank).mY > 80 /* IMAGE_SEEDBANK->mWidth */
                {
                    (*a_seed_bank).mY += 2;
                    if (*a_seed_bank).mY > 0 {
                        (*a_seed_bank).mY = 0;
                    }
                }
            }
        }

        // C++: 打地鼠更新
        if (*app).IsWhackAZombieLevel() {
            self.WhackAZombieUpdate();
        }
        // C++: IZombie 更新
        if (*app).IsIZombieLevel() {
            self.IZombieUpdate();
        }
        // C++: 老虎机更新
        if (*app).IsSlotMachineLevel() {
            self.UpdateSlotMachine();
        }
        // C++: 极速模式
        if app.mGameMode as i32 == GameMode::GAMEMODE_CHALLENGE_SPEED as i32 {
            board.UpdateGame();
        }
        // C++: 种子雨
        if app.mGameMode as i32 == GameMode::GAMEMODE_CHALLENGE_RAINING_SEEDS as i32 {
            self.UpdateRainingSeeds();
        }
        // C++: 传送门
        if app.mGameMode as i32 == GameMode::GAMEMODE_CHALLENGE_PORTAL_COMBAT as i32 {
            self.UpdatePortalCombat();
        }
        // C++: 僵尸水族馆
        if app.mGameMode as i32 == GameMode::GAMEMODE_CHALLENGE_ZOMBIQUARIUM as i32 {
            self.ZombiquariumUpdate();
        }
        // C++: 智慧树
        if app.mGameMode as i32 == GameMode::GAMEMODE_TREE_OF_WISDOM as i32 {
            self.TreeOfWisdomUpdate();
        }
        // C++: 冰道关卡 3000 帧提示（音效）
        if app.mGameMode as i32 == GameMode::GAMEMODE_CHALLENGE_ICE as i32 && board.mMainCounter == 3000 {
            // [TODO]: mApp->PlayFoley(FOLEY_FLOOP); mApp->PlaySample(SOUND_LOSEMUSIC)
        }
        // C++: 最后一战
        if app.mGameMode as i32 == GameMode::GAMEMODE_CHALLENGE_LAST_STAND as i32 {
            self.LastStandUpdate();
        }

        // ReanimChallenge update
        // Reanimation* aReanim = app->ReanimationTryToGet(mReanimChallenge);
        // [TODO]: Reanimation 更新
    }

    // =========================================================================
    // ★ ScaryPotter（恐怖罐）系列 — C++ Challenge.cpp 保真翻译
    // =========================================================================

    /// C++: Challenge::ScaryPotterDontPlaceInCol (Challenge.cpp:3692)
    pub unsafe fn ScaryPotterDontPlaceInCol(&mut self, the_col: i32, the_grid_array: &mut [crate::sexy_tod_lib::tod_common::TodWeightedGridArray], the_grid_array_count: i32) {
        let mut i = 0;
        while i < the_grid_array_count {
            if the_grid_array[i as usize].m_x == the_col {
                the_grid_array[i as usize].m_weight = 0;
            }
            i += 1;
        }
    }

    /// C++: Challenge::ScaryPotterFillColumnWithPlant (Challenge.cpp:3703)
    pub unsafe fn ScaryPotterFillColumnWithPlant(&mut self, the_col: i32, the_seed_type: SeedType, the_grid_array: &mut [crate::sexy_tod_lib::tod_common::TodWeightedGridArray], the_grid_array_count: i32) {
        self.ScaryPotterDontPlaceInCol(the_col, the_grid_array, the_grid_array_count);

        // C++: for (int i = 0; i < MAX_GRID_SIZE_Y - 1; i++)
        let mut i = 0;
        while i < crate::lawn::board_consts::MAX_GRID_SIZE_Y - 1 {
            let the_board = &mut *self.mBoard;
            let a_plant = the_board.NewPlant(the_col, i, the_seed_type as i32, SeedType::SEED_NONE as i32);
            if !a_plant.is_null() && the_seed_type == SeedType::SEED_POTATOMINE {
                (*a_plant).m_state_countdown = 10;
            }
            i += 1;
        }
    }

    /// C++: Challenge::ScaryPotterPlacePot (Challenge.cpp:3718)
    pub unsafe fn ScaryPotterPlacePot(&mut self, the_scary_pot_type: ScaryPotType, the_zombie_type: ZombieType, the_seed_type: SeedType, the_count: i32, the_grid_array: &mut [crate::sexy_tod_lib::tod_common::TodWeightedGridArray], the_grid_array_count: i32) {
        let a_pot_type = the_scary_pot_type;
        let mut the_count = the_count;
        while the_count > 0 {
            // C++: TodWeightedGridArray* aGrid = TodPickFromWeightedGridArray(theGridArray, theGridArrayCount);
            let a_grid: *mut crate::sexy_tod_lib::tod_common::TodWeightedGridArray = match crate::sexy_tod_lib::tod_common::tod_pick_from_weighted_grid_array(the_grid_array) {
                Some(g) => g,
                None => std::ptr::null_mut(),
            };
            if a_grid.is_null() {
                break;
            }

            let the_board = &mut *self.mBoard;
            let a_scary_pot = the_board.mGridItems.data_array_alloc();
            if a_scary_pot.is_null() {
                break;
            }
            // C++: aScaryPot->mGridItemType = GRIDITEM_SCARY_POT;
            (*a_scary_pot).mGridItemType = GridItemType::GRIDITEM_SCARY_POT;
            // C++: aScaryPot->mGridItemState = GRIDITEM_STATE_SCARY_POT_QUESTION;
            (*a_scary_pot).mGridItemState = 3; /* GRIDITEM_STATE_SCARY_POT_QUESTION */
            (*a_scary_pot).mGridX = (*a_grid).m_x;
            (*a_scary_pot).mGridY = (*a_grid).m_y;
            (*a_grid).m_weight = 0;
            (*a_scary_pot).mRenderOrder = crate::lawn::board::Board::MakeRenderOrder(RenderLayer::RENDER_LAYER_PLANT, (*a_grid).m_y, 0);
            (*a_scary_pot).mZombieType = the_zombie_type;
            (*a_scary_pot).mSeedType = the_seed_type;
            (*a_scary_pot).mScaryPotType = a_pot_type;
            if a_pot_type == ScaryPotType::SCARYPOT_SUN {
                // C++: aScaryPot->mSunCount = Rand(3) + 1;
                (*a_scary_pot).mSunCount = crate::sexy_app_framework::common::rand_int() % 3 + 1;
            }
            the_count -= 1;
        }
    }

    /// C++: Challenge::ScaryPotterChangePotType (Challenge.cpp:3742)
    pub unsafe fn ScaryPotterChangePotType(&mut self, the_pot_type: i32, the_count: i32) {
        let mut a_pot_array: [crate::sexy_tod_lib::tod_common::TodWeightedArray; 54] = [
            crate::sexy_tod_lib::tod_common::TodWeightedArray { m_item: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedArray { m_item: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedArray { m_item: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedArray { m_item: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedArray { m_item: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedArray { m_item: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedArray { m_item: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedArray { m_item: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedArray { m_item: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedArray { m_item: 0, m_weight: 0 },
            crate::sexy_tod_lib::tod_common::TodWeightedArray { m_item: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedArray { m_item: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedArray { m_item: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedArray { m_item: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedArray { m_item: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedArray { m_item: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedArray { m_item: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedArray { m_item: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedArray { m_item: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedArray { m_item: 0, m_weight: 0 },
            crate::sexy_tod_lib::tod_common::TodWeightedArray { m_item: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedArray { m_item: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedArray { m_item: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedArray { m_item: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedArray { m_item: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedArray { m_item: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedArray { m_item: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedArray { m_item: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedArray { m_item: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedArray { m_item: 0, m_weight: 0 },
            crate::sexy_tod_lib::tod_common::TodWeightedArray { m_item: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedArray { m_item: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedArray { m_item: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedArray { m_item: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedArray { m_item: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedArray { m_item: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedArray { m_item: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedArray { m_item: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedArray { m_item: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedArray { m_item: 0, m_weight: 0 },
            crate::sexy_tod_lib::tod_common::TodWeightedArray { m_item: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedArray { m_item: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedArray { m_item: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedArray { m_item: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedArray { m_item: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedArray { m_item: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedArray { m_item: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedArray { m_item: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedArray { m_item: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedArray { m_item: 0, m_weight: 0 },
            crate::sexy_tod_lib::tod_common::TodWeightedArray { m_item: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedArray { m_item: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedArray { m_item: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedArray { m_item: 0, m_weight: 0 },
        ];
        let mut a_pot_count = 0;

        let the_board = &mut *self.mBoard;
        let mut a_grid_item: *mut crate::lawn::grid_item::GridItem = std::ptr::null_mut();
        while the_board.IterateGridItems(&mut a_grid_item) {
            if (*a_grid_item).mGridItemType == GridItemType::GRIDITEM_SCARY_POT {
                // C++: 选择指定类型的罐子（LEAF 对应 SCARYPOT_SEED，ZOMBIE 对应伽刚特尔）
                if (the_pot_type == 4 /* GRIDITEM_STATE_SCARY_POT_LEAF */ && (*a_grid_item).mScaryPotType == ScaryPotType::SCARYPOT_SEED)
                    || (the_pot_type == 5 /* GRIDITEM_STATE_SCARY_POT_ZOMBIE */ && (*a_grid_item).mZombieType == ZombieType::ZOMBIE_GARGANTUAR)
                {
                    a_pot_array[a_pot_count].m_item = a_grid_item as isize;
                    a_pot_array[a_pot_count].m_weight = 1;
                    a_pot_count += 1;
                }
            }
        }

        // C++: theCount = std::min(theCount, aPotCount);
        let the_count = the_count.min(a_pot_count as i32);

        let mut i = 0;
        while i < the_count {
            // C++: TodPickArrayItemFromWeightedArray(aPotArray, aPotCount)
            let a_scary_pot_array = crate::sexy_tod_lib::tod_common::tod_pick_array_item_from_weighted_array(&mut a_pot_array[..a_pot_count as usize]);
            if a_scary_pot_array.is_null() {
                break;
            }
            (*a_scary_pot_array).m_weight = 0;
            // C++: ((GridItem*)aScaryPotArray->mItem)->mGridItemState = thePotType;
            let a_pot_grid = (*a_scary_pot_array).m_item as *mut crate::lawn::grid_item::GridItem;
            (*a_pot_grid).mGridItemState = the_pot_type;
            i += 1;
        }
    }

    /// C++: Challenge::ScaryPotterCountPots (Challenge.cpp:4052)
    pub unsafe fn ScaryPotterCountPots(&self) -> i32 {
        let mut a_count = 0;
        let the_board = &*self.mBoard;
        let mut a_grid_item: *mut crate::lawn::grid_item::GridItem = std::ptr::null_mut();
        while the_board.IterateGridItems(&mut a_grid_item) {
            if (*a_grid_item).mGridItemType == GridItemType::GRIDITEM_SCARY_POT {
                a_count += 1;
            }
        }
        a_count
    }

    /// C++: Challenge::ScaryPotterIsCompleted (Challenge.cpp:4003)
    pub unsafe fn ScaryPotterIsCompleted(&self) -> i32 {
        let the_board = &*self.mBoard;
        let mut a_grid_item: *mut crate::lawn::grid_item::GridItem = std::ptr::null_mut();
        while the_board.IterateGridItems(&mut a_grid_item) {
            if (*a_grid_item).mGridItemType == GridItemType::GRIDITEM_SCARY_POT {
                return 0;
            }
        }

        if the_board.AreEnemyZombiesOnScreen() { 0 } else { 1 }
    }

    /// C++: Challenge::ScaryPotterStart (Challenge.cpp:3995)
    pub unsafe fn ScaryPotterStart(&mut self) {
        if (*self.mApp).is_adventure_mode() {
            let the_board = &mut *self.mBoard;
            the_board.DisplayAdvice("[ADVICE_USE_SHOVEL_ON_POTS]", 0, AdviceType::ADVICE_USE_SHOVEL_ON_POTS as i32);
        }
    }

    /// C++: Challenge::ScaryPotterMalletPot (Challenge.cpp:4039)
    pub unsafe fn ScaryPotterMalletPot(&mut self, the_scary_pot: *mut crate::lawn::grid_item::GridItem) {
        if the_scary_pot.is_null() {
            return;
        }
        self.mChallengeGridX = (*the_scary_pot).mGridX;
        self.mChallengeGridY = (*the_scary_pot).mGridY;
        let the_board = &mut *self.mBoard;
        let a_x_pos = the_board.GridToPixelX((*the_scary_pot).mGridX, (*the_scary_pot).mGridY);
        let a_y_pos = the_board.GridToPixelY((*the_scary_pot).mGridX, (*the_scary_pot).mGridY);
        // [TODO]: AddReanimation(aXPos, aYPos, RENDER_LAYER_TOP, REANIM_HAMMER) + PlayReanim("anim_pot_open")
        self.mChallengeState = ChallengeState::STATECHALLENGE_SCARY_POTTER_MALLETING;
        // [TODO]: mApp->PlayFoley(FOLEY_SWING)
    }

    /// C++: Challenge::ScaryPotterOpenPot (Challenge.cpp:4109)
    pub unsafe fn ScaryPotterOpenPot(&mut self, the_scary_pot: *mut crate::lawn::grid_item::GridItem) {
        if the_scary_pot.is_null() {
            return;
        }
        // C++: 按罐子类型发放内容（种子/僵尸/阳光）
        let the_board = &mut *self.mBoard;
        let a_pos_x = the_board.GridToPixelX((*the_scary_pot).mGridX, (*the_scary_pot).mGridY);
        let a_pos_y = the_board.GridToPixelY((*the_scary_pot).mGridX, (*the_scary_pot).mGridY);
        match (*the_scary_pot).mScaryPotType {
            ScaryPotType::SCARYPOT_SEED => {
                // [TODO]: 释放种子植物（NewPlant）
                let _ = a_pos_x;
                let _ = a_pos_y;
            }
            ScaryPotType::SCARYPOT_ZOMBIE => {
                // [TODO]: AddZombieInRow(僵尸类型) + GridItemDie
            }
            ScaryPotType::SCARYPOT_SUN => {
                // [TODO]: 按 mSunCount 掉落阳光
                let _ = a_pos_x;
            }
            ScaryPotType::SCARYPOT_NONE => {}
        }
        // [TODO]: GridItemDie + 粒子
    }
    /// C++: Challenge::ScaryPotterPopulate (Challenge.cpp:3772) — 生成恐怖罐棋盘
    pub unsafe fn ScaryPotterPopulate(&mut self) {
        let mut a_grid_array: [crate::sexy_tod_lib::tod_common::TodWeightedGridArray; 54] = [
            crate::sexy_tod_lib::tod_common::TodWeightedGridArray { m_x: 0, m_y: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedGridArray { m_x: 0, m_y: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedGridArray { m_x: 0, m_y: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedGridArray { m_x: 0, m_y: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedGridArray { m_x: 0, m_y: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedGridArray { m_x: 0, m_y: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedGridArray { m_x: 0, m_y: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedGridArray { m_x: 0, m_y: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedGridArray { m_x: 0, m_y: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedGridArray { m_x: 0, m_y: 0, m_weight: 0 },
            crate::sexy_tod_lib::tod_common::TodWeightedGridArray { m_x: 0, m_y: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedGridArray { m_x: 0, m_y: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedGridArray { m_x: 0, m_y: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedGridArray { m_x: 0, m_y: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedGridArray { m_x: 0, m_y: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedGridArray { m_x: 0, m_y: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedGridArray { m_x: 0, m_y: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedGridArray { m_x: 0, m_y: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedGridArray { m_x: 0, m_y: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedGridArray { m_x: 0, m_y: 0, m_weight: 0 },
            crate::sexy_tod_lib::tod_common::TodWeightedGridArray { m_x: 0, m_y: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedGridArray { m_x: 0, m_y: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedGridArray { m_x: 0, m_y: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedGridArray { m_x: 0, m_y: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedGridArray { m_x: 0, m_y: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedGridArray { m_x: 0, m_y: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedGridArray { m_x: 0, m_y: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedGridArray { m_x: 0, m_y: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedGridArray { m_x: 0, m_y: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedGridArray { m_x: 0, m_y: 0, m_weight: 0 },
            crate::sexy_tod_lib::tod_common::TodWeightedGridArray { m_x: 0, m_y: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedGridArray { m_x: 0, m_y: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedGridArray { m_x: 0, m_y: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedGridArray { m_x: 0, m_y: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedGridArray { m_x: 0, m_y: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedGridArray { m_x: 0, m_y: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedGridArray { m_x: 0, m_y: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedGridArray { m_x: 0, m_y: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedGridArray { m_x: 0, m_y: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedGridArray { m_x: 0, m_y: 0, m_weight: 0 },
            crate::sexy_tod_lib::tod_common::TodWeightedGridArray { m_x: 0, m_y: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedGridArray { m_x: 0, m_y: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedGridArray { m_x: 0, m_y: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedGridArray { m_x: 0, m_y: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedGridArray { m_x: 0, m_y: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedGridArray { m_x: 0, m_y: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedGridArray { m_x: 0, m_y: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedGridArray { m_x: 0, m_y: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedGridArray { m_x: 0, m_y: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedGridArray { m_x: 0, m_y: 0, m_weight: 0 },
            crate::sexy_tod_lib::tod_common::TodWeightedGridArray { m_x: 0, m_y: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedGridArray { m_x: 0, m_y: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedGridArray { m_x: 0, m_y: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedGridArray { m_x: 0, m_y: 0, m_weight: 0 },
        ];
        let mut a_grid_array_count = 0;

        // C++: 遍历 9x5 网格建立权重数组
        let mut a_grid_x = 0;
        while a_grid_x < crate::lawn::board_consts::MAX_GRID_SIZE_X {
            let mut a_grid_y = 0;
            while a_grid_y < crate::lawn::board_consts::MAX_GRID_SIZE_Y - 1 {
                a_grid_array[a_grid_array_count as usize].m_x = a_grid_x;
                a_grid_array[a_grid_array_count as usize].m_y = a_grid_y;
                a_grid_array[a_grid_array_count as usize].m_weight = 1;
                a_grid_array_count += 1;
                a_grid_y += 1;
            }
            a_grid_x += 1;
        }

        // C++: 冒险模式第 35 关（3 阶段）
        if (*self.mApp).is_adventure_mode() && !self.mBoard.is_null() && (*self.mBoard).mLevel == 35 {
            match self.mSurvivalStage {
                0 => {
                    self.ScaryPotterDontPlaceInCol(0, &mut a_grid_array, a_grid_array_count);
                    self.ScaryPotterDontPlaceInCol(1, &mut a_grid_array, a_grid_array_count);
                    self.ScaryPotterDontPlaceInCol(2, &mut a_grid_array, a_grid_array_count);
                    self.ScaryPotterDontPlaceInCol(3, &mut a_grid_array, a_grid_array_count);
                    self.ScaryPotterDontPlaceInCol(4, &mut a_grid_array, a_grid_array_count);
                    self.ScaryPotterDontPlaceInCol(5, &mut a_grid_array, a_grid_array_count);
                    self.ScaryPotterPlacePot(ScaryPotType::SCARYPOT_SEED, ZombieType::ZOMBIE_INVALID, SeedType::SEED_PEASHOOTER, 5, &mut a_grid_array, a_grid_array_count);
                    self.ScaryPotterPlacePot(ScaryPotType::SCARYPOT_SEED, ZombieType::ZOMBIE_INVALID, SeedType::SEED_SQUASH, 5, &mut a_grid_array, a_grid_array_count);
                    self.ScaryPotterPlacePot(ScaryPotType::SCARYPOT_ZOMBIE, ZombieType::ZOMBIE_NORMAL, SeedType::SEED_NONE, 4, &mut a_grid_array, a_grid_array_count);
                    self.ScaryPotterPlacePot(ScaryPotType::SCARYPOT_ZOMBIE, ZombieType::ZOMBIE_PAIL, SeedType::SEED_NONE, 1, &mut a_grid_array, a_grid_array_count);
                }
                1 => {
                    self.ScaryPotterDontPlaceInCol(0, &mut a_grid_array, a_grid_array_count);
                    self.ScaryPotterDontPlaceInCol(1, &mut a_grid_array, a_grid_array_count);
                    self.ScaryPotterDontPlaceInCol(2, &mut a_grid_array, a_grid_array_count);
                    self.ScaryPotterDontPlaceInCol(3, &mut a_grid_array, a_grid_array_count);
                    self.ScaryPotterDontPlaceInCol(4, &mut a_grid_array, a_grid_array_count);
                    self.ScaryPotterPlacePot(ScaryPotType::SCARYPOT_SEED, ZombieType::ZOMBIE_INVALID, SeedType::SEED_PEASHOOTER, 4, &mut a_grid_array, a_grid_array_count);
                    self.ScaryPotterPlacePot(ScaryPotType::SCARYPOT_SEED, ZombieType::ZOMBIE_INVALID, SeedType::SEED_SNOWPEA, 5, &mut a_grid_array, a_grid_array_count);
                    self.ScaryPotterPlacePot(ScaryPotType::SCARYPOT_SEED, ZombieType::ZOMBIE_INVALID, SeedType::SEED_SQUASH, 4, &mut a_grid_array, a_grid_array_count);
                    self.ScaryPotterPlacePot(ScaryPotType::SCARYPOT_ZOMBIE, ZombieType::ZOMBIE_NORMAL, SeedType::SEED_NONE, 5, &mut a_grid_array, a_grid_array_count);
                    self.ScaryPotterPlacePot(ScaryPotType::SCARYPOT_ZOMBIE, ZombieType::ZOMBIE_PAIL, SeedType::SEED_NONE, 1, &mut a_grid_array, a_grid_array_count);
                    self.ScaryPotterPlacePot(ScaryPotType::SCARYPOT_ZOMBIE, ZombieType::ZOMBIE_FOOTBALL, SeedType::SEED_NONE, 1, &mut a_grid_array, a_grid_array_count);
                    self.ScaryPotterChangePotType(4 /* LEAF */, 2);
                }
                2 => {
                    self.ScaryPotterDontPlaceInCol(0, &mut a_grid_array, a_grid_array_count);
                    self.ScaryPotterDontPlaceInCol(1, &mut a_grid_array, a_grid_array_count);
                    self.ScaryPotterDontPlaceInCol(2, &mut a_grid_array, a_grid_array_count);
                    self.ScaryPotterDontPlaceInCol(3, &mut a_grid_array, a_grid_array_count);
                    self.ScaryPotterPlacePot(ScaryPotType::SCARYPOT_SEED, ZombieType::ZOMBIE_INVALID, SeedType::SEED_PEASHOOTER, 5, &mut a_grid_array, a_grid_array_count);
                    self.ScaryPotterPlacePot(ScaryPotType::SCARYPOT_SEED, ZombieType::ZOMBIE_INVALID, SeedType::SEED_SNOWPEA, 5, &mut a_grid_array, a_grid_array_count);
                    self.ScaryPotterPlacePot(ScaryPotType::SCARYPOT_SEED, ZombieType::ZOMBIE_INVALID, SeedType::SEED_HYPNOSHROOM, 5, &mut a_grid_array, a_grid_array_count);
                    self.ScaryPotterPlacePot(ScaryPotType::SCARYPOT_ZOMBIE, ZombieType::ZOMBIE_NORMAL, SeedType::SEED_NONE, 6, &mut a_grid_array, a_grid_array_count);
                    self.ScaryPotterPlacePot(ScaryPotType::SCARYPOT_ZOMBIE, ZombieType::ZOMBIE_PAIL, SeedType::SEED_NONE, 2, &mut a_grid_array, a_grid_array_count);
                    self.ScaryPotterPlacePot(ScaryPotType::SCARYPOT_ZOMBIE, ZombieType::ZOMBIE_DANCER, SeedType::SEED_NONE, 1, &mut a_grid_array, a_grid_array_count);
                    self.ScaryPotterPlacePot(ScaryPotType::SCARYPOT_ZOMBIE, ZombieType::ZOMBIE_JACK_IN_THE_BOX, SeedType::SEED_NONE, 1, &mut a_grid_array, a_grid_array_count);
                    self.ScaryPotterChangePotType(4 /* LEAF */, 3);
                }
                _ => {}
            }
        } else {
            // C++: 各恐怖罐挑战关卡配置
            match (*self.mApp).mGameMode {
                GameMode::GAMEMODE_SCARY_POTTER_1 => {
                    self.ScaryPotterDontPlaceInCol(0, &mut a_grid_array, a_grid_array_count);
                    self.ScaryPotterDontPlaceInCol(1, &mut a_grid_array, a_grid_array_count);
                    self.ScaryPotterDontPlaceInCol(2, &mut a_grid_array, a_grid_array_count);
                    self.ScaryPotterDontPlaceInCol(3, &mut a_grid_array, a_grid_array_count);
                    self.ScaryPotterPlacePot(ScaryPotType::SCARYPOT_SEED, ZombieType::ZOMBIE_INVALID, SeedType::SEED_PEASHOOTER, 5, &mut a_grid_array, a_grid_array_count);
                    self.ScaryPotterPlacePot(ScaryPotType::SCARYPOT_SEED, ZombieType::ZOMBIE_INVALID, SeedType::SEED_SNOWPEA, 5, &mut a_grid_array, a_grid_array_count);
                    self.ScaryPotterPlacePot(ScaryPotType::SCARYPOT_SEED, ZombieType::ZOMBIE_INVALID, SeedType::SEED_SQUASH, 5, &mut a_grid_array, a_grid_array_count);
                    self.ScaryPotterPlacePot(ScaryPotType::SCARYPOT_ZOMBIE, ZombieType::ZOMBIE_NORMAL, SeedType::SEED_NONE, 6, &mut a_grid_array, a_grid_array_count);
                    self.ScaryPotterPlacePot(ScaryPotType::SCARYPOT_ZOMBIE, ZombieType::ZOMBIE_PAIL, SeedType::SEED_NONE, 3, &mut a_grid_array, a_grid_array_count);
                    self.ScaryPotterPlacePot(ScaryPotType::SCARYPOT_ZOMBIE, ZombieType::ZOMBIE_JACK_IN_THE_BOX, SeedType::SEED_NONE, 1, &mut a_grid_array, a_grid_array_count);
                    self.ScaryPotterChangePotType(4 /* LEAF */, 2);
                }
                GameMode::GAMEMODE_SCARY_POTTER_2 => {
                    self.ScaryPotterDontPlaceInCol(0, &mut a_grid_array, a_grid_array_count);
                    self.ScaryPotterDontPlaceInCol(1, &mut a_grid_array, a_grid_array_count);
                    self.ScaryPotterDontPlaceInCol(2, &mut a_grid_array, a_grid_array_count);
                    self.ScaryPotterDontPlaceInCol(8, &mut a_grid_array, a_grid_array_count);
                    self.ScaryPotterPlacePot(ScaryPotType::SCARYPOT_SEED, ZombieType::ZOMBIE_INVALID, SeedType::SEED_LEFTPEATER, 7, &mut a_grid_array, a_grid_array_count);
                    self.ScaryPotterPlacePot(ScaryPotType::SCARYPOT_SEED, ZombieType::ZOMBIE_INVALID, SeedType::SEED_SNOWPEA, 3, &mut a_grid_array, a_grid_array_count);
                    self.ScaryPotterPlacePot(ScaryPotType::SCARYPOT_SEED, ZombieType::ZOMBIE_INVALID, SeedType::SEED_WALLNUT, 3, &mut a_grid_array, a_grid_array_count);
                    self.ScaryPotterPlacePot(ScaryPotType::SCARYPOT_SEED, ZombieType::ZOMBIE_INVALID, SeedType::SEED_POTATOMINE, 2, &mut a_grid_array, a_grid_array_count);
                    self.ScaryPotterPlacePot(ScaryPotType::SCARYPOT_ZOMBIE, ZombieType::ZOMBIE_NORMAL, SeedType::SEED_NONE, 6, &mut a_grid_array, a_grid_array_count);
                    self.ScaryPotterPlacePot(ScaryPotType::SCARYPOT_ZOMBIE, ZombieType::ZOMBIE_PAIL, SeedType::SEED_NONE, 3, &mut a_grid_array, a_grid_array_count);
                    self.ScaryPotterPlacePot(ScaryPotType::SCARYPOT_ZOMBIE, ZombieType::ZOMBIE_JACK_IN_THE_BOX, SeedType::SEED_NONE, 1, &mut a_grid_array, a_grid_array_count);
                    self.ScaryPotterChangePotType(4 /* LEAF */, 2);
                }
                GameMode::GAMEMODE_SCARY_POTTER_3 => {
                    self.ScaryPotterDontPlaceInCol(0, &mut a_grid_array, a_grid_array_count);
                    self.ScaryPotterDontPlaceInCol(1, &mut a_grid_array, a_grid_array_count);
                    self.ScaryPotterDontPlaceInCol(2, &mut a_grid_array, a_grid_array_count);
                    self.ScaryPotterPlacePot(ScaryPotType::SCARYPOT_SEED, ZombieType::ZOMBIE_INVALID, SeedType::SEED_LEFTPEATER, 6, &mut a_grid_array, a_grid_array_count);
                    self.ScaryPotterPlacePot(ScaryPotType::SCARYPOT_SEED, ZombieType::ZOMBIE_INVALID, SeedType::SEED_SNOWPEA, 4, &mut a_grid_array, a_grid_array_count);
                    self.ScaryPotterPlacePot(ScaryPotType::SCARYPOT_SEED, ZombieType::ZOMBIE_INVALID, SeedType::SEED_SQUASH, 2, &mut a_grid_array, a_grid_array_count);
                    self.ScaryPotterPlacePot(ScaryPotType::SCARYPOT_SEED, ZombieType::ZOMBIE_INVALID, SeedType::SEED_HYPNOSHROOM, 3, &mut a_grid_array, a_grid_array_count);
                    self.ScaryPotterPlacePot(ScaryPotType::SCARYPOT_SEED, ZombieType::ZOMBIE_INVALID, SeedType::SEED_WALLNUT, 3, &mut a_grid_array, a_grid_array_count);
                    self.ScaryPotterPlacePot(ScaryPotType::SCARYPOT_ZOMBIE, ZombieType::ZOMBIE_NORMAL, SeedType::SEED_NONE, 8, &mut a_grid_array, a_grid_array_count);
                    self.ScaryPotterPlacePot(ScaryPotType::SCARYPOT_ZOMBIE, ZombieType::ZOMBIE_PAIL, SeedType::SEED_NONE, 2, &mut a_grid_array, a_grid_array_count);
                    self.ScaryPotterPlacePot(ScaryPotType::SCARYPOT_ZOMBIE, ZombieType::ZOMBIE_DANCER, SeedType::SEED_NONE, 1, &mut a_grid_array, a_grid_array_count);
                    self.ScaryPotterPlacePot(ScaryPotType::SCARYPOT_ZOMBIE, ZombieType::ZOMBIE_JACK_IN_THE_BOX, SeedType::SEED_NONE, 1, &mut a_grid_array, a_grid_array_count);
                    self.ScaryPotterChangePotType(4 /* LEAF */, 2);
                }
                GameMode::GAMEMODE_SCARY_POTTER_4 => {
                    self.ScaryPotterDontPlaceInCol(0, &mut a_grid_array, a_grid_array_count);
                    self.ScaryPotterDontPlaceInCol(1, &mut a_grid_array, a_grid_array_count);
                    self.ScaryPotterPlacePot(ScaryPotType::SCARYPOT_SEED, ZombieType::ZOMBIE_INVALID, SeedType::SEED_PUFFSHROOM, 11, &mut a_grid_array, a_grid_array_count);
                    self.ScaryPotterPlacePot(ScaryPotType::SCARYPOT_SEED, ZombieType::ZOMBIE_INVALID, SeedType::SEED_HYPNOSHROOM, 4, &mut a_grid_array, a_grid_array_count);
                    self.ScaryPotterPlacePot(ScaryPotType::SCARYPOT_SEED, ZombieType::ZOMBIE_INVALID, SeedType::SEED_LEFTPEATER, 4, &mut a_grid_array, a_grid_array_count);
                    self.ScaryPotterPlacePot(ScaryPotType::SCARYPOT_ZOMBIE, ZombieType::ZOMBIE_JACK_IN_THE_BOX, SeedType::SEED_NONE, 8, &mut a_grid_array, a_grid_array_count);
                    self.ScaryPotterPlacePot(ScaryPotType::SCARYPOT_ZOMBIE, ZombieType::ZOMBIE_NORMAL, SeedType::SEED_NONE, 7, &mut a_grid_array, a_grid_array_count);
                    self.ScaryPotterPlacePot(ScaryPotType::SCARYPOT_ZOMBIE, ZombieType::ZOMBIE_FOOTBALL, SeedType::SEED_NONE, 1, &mut a_grid_array, a_grid_array_count);
                    self.ScaryPotterChangePotType(4 /* LEAF */, 2);
                }
                GameMode::GAMEMODE_SCARY_POTTER_5 => {
                    self.ScaryPotterDontPlaceInCol(0, &mut a_grid_array, a_grid_array_count);
                    self.ScaryPotterDontPlaceInCol(1, &mut a_grid_array, a_grid_array_count);
                    self.ScaryPotterPlacePot(ScaryPotType::SCARYPOT_SEED, ZombieType::ZOMBIE_INVALID, SeedType::SEED_LEFTPEATER, 6, &mut a_grid_array, a_grid_array_count);
                    self.ScaryPotterPlacePot(ScaryPotType::SCARYPOT_SEED, ZombieType::ZOMBIE_INVALID, SeedType::SEED_PUMPKINSHELL, 3, &mut a_grid_array, a_grid_array_count);
                    self.ScaryPotterPlacePot(ScaryPotType::SCARYPOT_SEED, ZombieType::ZOMBIE_INVALID, SeedType::SEED_SQUASH, 4, &mut a_grid_array, a_grid_array_count);
                    self.ScaryPotterPlacePot(ScaryPotType::SCARYPOT_SEED, ZombieType::ZOMBIE_INVALID, SeedType::SEED_HYPNOSHROOM, 2, &mut a_grid_array, a_grid_array_count);
                    self.ScaryPotterPlacePot(ScaryPotType::SCARYPOT_SEED, ZombieType::ZOMBIE_INVALID, SeedType::SEED_SNOWPEA, 2, &mut a_grid_array, a_grid_array_count);
                    self.ScaryPotterPlacePot(ScaryPotType::SCARYPOT_SEED, ZombieType::ZOMBIE_INVALID, SeedType::SEED_MAGNETSHROOM, 3, &mut a_grid_array, a_grid_array_count);
                    self.ScaryPotterPlacePot(ScaryPotType::SCARYPOT_ZOMBIE, ZombieType::ZOMBIE_NORMAL, SeedType::SEED_NONE, 6, &mut a_grid_array, a_grid_array_count);
                    self.ScaryPotterPlacePot(ScaryPotType::SCARYPOT_ZOMBIE, ZombieType::ZOMBIE_PAIL, SeedType::SEED_NONE, 5, &mut a_grid_array, a_grid_array_count);
                    self.ScaryPotterPlacePot(ScaryPotType::SCARYPOT_ZOMBIE, ZombieType::ZOMBIE_JACK_IN_THE_BOX, SeedType::SEED_NONE, 1, &mut a_grid_array, a_grid_array_count);
                    self.ScaryPotterPlacePot(ScaryPotType::SCARYPOT_ZOMBIE, ZombieType::ZOMBIE_FOOTBALL, SeedType::SEED_NONE, 3, &mut a_grid_array, a_grid_array_count);
                    self.ScaryPotterChangePotType(4 /* LEAF */, 2);
                }
                GameMode::GAMEMODE_SCARY_POTTER_6 => {
                    self.ScaryPotterDontPlaceInCol(0, &mut a_grid_array, a_grid_array_count);
                    self.ScaryPotterDontPlaceInCol(1, &mut a_grid_array, a_grid_array_count);
                    self.ScaryPotterPlacePot(ScaryPotType::SCARYPOT_SEED, ZombieType::ZOMBIE_INVALID, SeedType::SEED_LEFTPEATER, 7, &mut a_grid_array, a_grid_array_count);
                    self.ScaryPotterPlacePot(ScaryPotType::SCARYPOT_SEED, ZombieType::ZOMBIE_INVALID, SeedType::SEED_SQUASH, 2, &mut a_grid_array, a_grid_array_count);
                    self.ScaryPotterPlacePot(ScaryPotType::SCARYPOT_SEED, ZombieType::ZOMBIE_INVALID, SeedType::SEED_TALLNUT, 5, &mut a_grid_array, a_grid_array_count);
                    self.ScaryPotterPlacePot(ScaryPotType::SCARYPOT_SEED, ZombieType::ZOMBIE_INVALID, SeedType::SEED_THREEPEATER, 2, &mut a_grid_array, a_grid_array_count);
                    self.ScaryPotterPlacePot(ScaryPotType::SCARYPOT_SEED, ZombieType::ZOMBIE_INVALID, SeedType::SEED_TORCHWOOD, 4, &mut a_grid_array, a_grid_array_count);
                    self.ScaryPotterPlacePot(ScaryPotType::SCARYPOT_ZOMBIE, ZombieType::ZOMBIE_NORMAL, SeedType::SEED_NONE, 7, &mut a_grid_array, a_grid_array_count);
                    self.ScaryPotterPlacePot(ScaryPotType::SCARYPOT_ZOMBIE, ZombieType::ZOMBIE_POLEVAULTER, SeedType::SEED_NONE, 5, &mut a_grid_array, a_grid_array_count);
                    self.ScaryPotterPlacePot(ScaryPotType::SCARYPOT_ZOMBIE, ZombieType::ZOMBIE_FOOTBALL, SeedType::SEED_NONE, 2, &mut a_grid_array, a_grid_array_count);
                    self.ScaryPotterPlacePot(ScaryPotType::SCARYPOT_ZOMBIE, ZombieType::ZOMBIE_JACK_IN_THE_BOX, SeedType::SEED_NONE, 1, &mut a_grid_array, a_grid_array_count);
                    self.ScaryPotterChangePotType(4 /* LEAF */, 2);
                }
                GameMode::GAMEMODE_SCARY_POTTER_7 => {
                    self.ScaryPotterDontPlaceInCol(0, &mut a_grid_array, a_grid_array_count);
                    self.ScaryPotterDontPlaceInCol(1, &mut a_grid_array, a_grid_array_count);
                    self.ScaryPotterDontPlaceInCol(2, &mut a_grid_array, a_grid_array_count);
                    self.ScaryPotterPlacePot(ScaryPotType::SCARYPOT_SEED, ZombieType::ZOMBIE_INVALID, SeedType::SEED_SPIKEWEED, 13, &mut a_grid_array, a_grid_array_count);
                    self.ScaryPotterPlacePot(ScaryPotType::SCARYPOT_SEED, ZombieType::ZOMBIE_INVALID, SeedType::SEED_WALLNUT, 3, &mut a_grid_array, a_grid_array_count);
                    self.ScaryPotterPlacePot(ScaryPotType::SCARYPOT_SEED, ZombieType::ZOMBIE_INVALID, SeedType::SEED_SQUASH, 3, &mut a_grid_array, a_grid_array_count);
                    self.ScaryPotterPlacePot(ScaryPotType::SCARYPOT_ZOMBIE, ZombieType::ZOMBIE_NORMAL, SeedType::SEED_NONE, 10, &mut a_grid_array, a_grid_array_count);
                    self.ScaryPotterPlacePot(ScaryPotType::SCARYPOT_ZOMBIE, ZombieType::ZOMBIE_PAIL, SeedType::SEED_NONE, 1, &mut a_grid_array, a_grid_array_count);
                    self.ScaryPotterChangePotType(4 /* LEAF */, 2);
                }
                GameMode::GAMEMODE_SCARY_POTTER_8 => {
                    self.ScaryPotterDontPlaceInCol(0, &mut a_grid_array, a_grid_array_count);
                    self.ScaryPotterDontPlaceInCol(1, &mut a_grid_array, a_grid_array_count);
                    self.ScaryPotterPlacePot(ScaryPotType::SCARYPOT_SEED, ZombieType::ZOMBIE_INVALID, SeedType::SEED_PUFFSHROOM, 7, &mut a_grid_array, a_grid_array_count);
                    self.ScaryPotterPlacePot(ScaryPotType::SCARYPOT_SEED, ZombieType::ZOMBIE_INVALID, SeedType::SEED_WALLNUT, 3, &mut a_grid_array, a_grid_array_count);
                    self.ScaryPotterPlacePot(ScaryPotType::SCARYPOT_SEED, ZombieType::ZOMBIE_INVALID, SeedType::SEED_SQUASH, 5, &mut a_grid_array, a_grid_array_count);
                    self.ScaryPotterPlacePot(ScaryPotType::SCARYPOT_SEED, ZombieType::ZOMBIE_INVALID, SeedType::SEED_LEFTPEATER, 4, &mut a_grid_array, a_grid_array_count);
                    self.ScaryPotterPlacePot(ScaryPotType::SCARYPOT_ZOMBIE, ZombieType::ZOMBIE_JACK_IN_THE_BOX, SeedType::SEED_NONE, 8, &mut a_grid_array, a_grid_array_count);
                    self.ScaryPotterPlacePot(ScaryPotType::SCARYPOT_ZOMBIE, ZombieType::ZOMBIE_NORMAL, SeedType::SEED_NONE, 4, &mut a_grid_array, a_grid_array_count);
                    self.ScaryPotterPlacePot(ScaryPotType::SCARYPOT_ZOMBIE, ZombieType::ZOMBIE_POGO, SeedType::SEED_NONE, 4, &mut a_grid_array, a_grid_array_count);
                    self.ScaryPotterChangePotType(4 /* LEAF */, 2);
                }
                GameMode::GAMEMODE_SCARY_POTTER_9 => {
                    self.ScaryPotterDontPlaceInCol(0, &mut a_grid_array, a_grid_array_count);
                    self.ScaryPotterDontPlaceInCol(1, &mut a_grid_array, a_grid_array_count);
                    self.ScaryPotterPlacePot(ScaryPotType::SCARYPOT_SEED, ZombieType::ZOMBIE_INVALID, SeedType::SEED_LEFTPEATER, 6, &mut a_grid_array, a_grid_array_count);
                    self.ScaryPotterPlacePot(ScaryPotType::SCARYPOT_SEED, ZombieType::ZOMBIE_INVALID, SeedType::SEED_SNOWPEA, 2, &mut a_grid_array, a_grid_array_count);
                    self.ScaryPotterPlacePot(ScaryPotType::SCARYPOT_SEED, ZombieType::ZOMBIE_INVALID, SeedType::SEED_PEASHOOTER, 2, &mut a_grid_array, a_grid_array_count);
                    self.ScaryPotterPlacePot(ScaryPotType::SCARYPOT_SEED, ZombieType::ZOMBIE_INVALID, SeedType::SEED_THREEPEATER, 2, &mut a_grid_array, a_grid_array_count);
                    self.ScaryPotterPlacePot(ScaryPotType::SCARYPOT_SEED, ZombieType::ZOMBIE_INVALID, SeedType::SEED_SQUASH, 5, &mut a_grid_array, a_grid_array_count);
                    self.ScaryPotterPlacePot(ScaryPotType::SCARYPOT_SEED, ZombieType::ZOMBIE_INVALID, SeedType::SEED_POTATOMINE, 1, &mut a_grid_array, a_grid_array_count);
                    self.ScaryPotterPlacePot(ScaryPotType::SCARYPOT_SEED, ZombieType::ZOMBIE_INVALID, SeedType::SEED_WALLNUT, 1, &mut a_grid_array, a_grid_array_count);
                    self.ScaryPotterPlacePot(ScaryPotType::SCARYPOT_SEED, ZombieType::ZOMBIE_INVALID, SeedType::SEED_PLANTERN, 1, &mut a_grid_array, a_grid_array_count);
                    self.ScaryPotterPlacePot(ScaryPotType::SCARYPOT_ZOMBIE, ZombieType::ZOMBIE_NORMAL, SeedType::SEED_NONE, 8, &mut a_grid_array, a_grid_array_count);
                    self.ScaryPotterPlacePot(ScaryPotType::SCARYPOT_ZOMBIE, ZombieType::ZOMBIE_PAIL, SeedType::SEED_NONE, 5, &mut a_grid_array, a_grid_array_count);
                    self.ScaryPotterPlacePot(ScaryPotType::SCARYPOT_ZOMBIE, ZombieType::ZOMBIE_JACK_IN_THE_BOX, SeedType::SEED_NONE, 1, &mut a_grid_array, a_grid_array_count);
                    self.ScaryPotterPlacePot(ScaryPotType::SCARYPOT_ZOMBIE, ZombieType::ZOMBIE_GARGANTUAR, SeedType::SEED_NONE, 1, &mut a_grid_array, a_grid_array_count);
                    self.ScaryPotterChangePotType(4 /* LEAF */, 2);
                }
                GameMode::GAMEMODE_SCARY_POTTER_ENDLESS => {
                    // C++: int aNumExtraGargantuars = ClampInt(mSurvivalStage / 10, 0, 8);
                    let a_num_extra_gargantuars = crate::sexy_tod_lib::tod_common::clamp_int(self.mSurvivalStage / 10, 0, 8);
                    self.ScaryPotterDontPlaceInCol(0, &mut a_grid_array, a_grid_array_count);
                    self.ScaryPotterDontPlaceInCol(1, &mut a_grid_array, a_grid_array_count);
                    self.ScaryPotterPlacePot(ScaryPotType::SCARYPOT_SEED, ZombieType::ZOMBIE_INVALID, SeedType::SEED_LEFTPEATER, 6, &mut a_grid_array, a_grid_array_count);
                    self.ScaryPotterPlacePot(ScaryPotType::SCARYPOT_SEED, ZombieType::ZOMBIE_INVALID, SeedType::SEED_SNOWPEA, 2, &mut a_grid_array, a_grid_array_count);
                    self.ScaryPotterPlacePot(ScaryPotType::SCARYPOT_SEED, ZombieType::ZOMBIE_INVALID, SeedType::SEED_PEASHOOTER, 1, &mut a_grid_array, a_grid_array_count);
                    self.ScaryPotterPlacePot(ScaryPotType::SCARYPOT_SEED, ZombieType::ZOMBIE_INVALID, SeedType::SEED_THREEPEATER, 2, &mut a_grid_array, a_grid_array_count);
                    self.ScaryPotterPlacePot(ScaryPotType::SCARYPOT_SEED, ZombieType::ZOMBIE_INVALID, SeedType::SEED_SQUASH, 5, &mut a_grid_array, a_grid_array_count);
                    self.ScaryPotterPlacePot(ScaryPotType::SCARYPOT_SEED, ZombieType::ZOMBIE_INVALID, SeedType::SEED_POTATOMINE, 1, &mut a_grid_array, a_grid_array_count);
                    self.ScaryPotterPlacePot(ScaryPotType::SCARYPOT_SEED, ZombieType::ZOMBIE_INVALID, SeedType::SEED_WALLNUT, 1, &mut a_grid_array, a_grid_array_count);
                    self.ScaryPotterPlacePot(ScaryPotType::SCARYPOT_SEED, ZombieType::ZOMBIE_INVALID, SeedType::SEED_PLANTERN, 1, &mut a_grid_array, a_grid_array_count);
                    self.ScaryPotterPlacePot(ScaryPotType::SCARYPOT_SUN, ZombieType::ZOMBIE_INVALID, SeedType::SEED_NONE, 1, &mut a_grid_array, a_grid_array_count);
                    self.ScaryPotterPlacePot(ScaryPotType::SCARYPOT_ZOMBIE, ZombieType::ZOMBIE_NORMAL, SeedType::SEED_NONE, 8 - a_num_extra_gargantuars, &mut a_grid_array, a_grid_array_count);
                    self.ScaryPotterPlacePot(ScaryPotType::SCARYPOT_ZOMBIE, ZombieType::ZOMBIE_PAIL, SeedType::SEED_NONE, 5, &mut a_grid_array, a_grid_array_count);
                    self.ScaryPotterPlacePot(ScaryPotType::SCARYPOT_ZOMBIE, ZombieType::ZOMBIE_JACK_IN_THE_BOX, SeedType::SEED_NONE, 1, &mut a_grid_array, a_grid_array_count);
                    self.ScaryPotterPlacePot(ScaryPotType::SCARYPOT_ZOMBIE, ZombieType::ZOMBIE_GARGANTUAR, SeedType::SEED_NONE, 1 + a_num_extra_gargantuars, &mut a_grid_array, a_grid_array_count);
                    self.ScaryPotterChangePotType(4 /* LEAF */, 2);
                    // [TODO]: mSurvivalStage == 15 成就 ChinaShop
                }
                _ => {}
            }
        }

        // C++: mScaryPotterPots = ScaryPotterCountPots();
        self.m_scary_potter_pots = self.ScaryPotterCountPots();
    }
    /// C++ Challenge::UpdateBeghouled (Challenge.cpp:1413) — 宝石迷阵更新
    pub unsafe fn UpdateBeghouled(&mut self) {
        // [TODO]: Beghouled 交互状态机（拖动/消除/掉落）
        self.BeghouledCheckStuckState();
    }

    /// C++ Challenge::ScaryPotterUpdate (Challenge.cpp:4017) — 恐怖罐更新
    pub unsafe fn ScaryPotterUpdate(&mut self) {
        // C++: if (mChallengeState == STATECHALLENGE_SCARY_POTTER_MALLETING)
        if self.mChallengeState == ChallengeState::STATECHALLENGE_SCARY_POTTER_MALLETING {
            // [TODO]: Reanimation(mReanimChallenge)->mLoopCount > 0 检查
            {
                let the_board = &mut *self.mBoard;
                let a_scary_pot = the_board.GetGridItemAt(GridItemType::GRIDITEM_SCARY_POT, self.mChallengeGridX, self.mChallengeGridY);
                if !a_scary_pot.is_null() {
                    self.ScaryPotterOpenPot(a_scary_pot);
                }

                self.mChallengeGridX = 0;
                self.mChallengeGridY = 0;
                // [TODO]: aMalletReanim->ReanimationDie();
                self.mChallengeState = ChallengeState::STATECHALLENGE_NORMAL;
            }
        }
    }

    /// C++ Challenge::WhackAZombiePlaceGraves (Challenge.cpp:2733) — 加权放置墓碑
    pub unsafe fn WhackAZombiePlaceGraves(&mut self, the_grave_count: i32) {
        let mut a_picks: [crate::sexy_tod_lib::tod_common::TodWeightedGridArray; 36] = [
            crate::sexy_tod_lib::tod_common::TodWeightedGridArray { m_x: 0, m_y: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedGridArray { m_x: 0, m_y: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedGridArray { m_x: 0, m_y: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedGridArray { m_x: 0, m_y: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedGridArray { m_x: 0, m_y: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedGridArray { m_x: 0, m_y: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedGridArray { m_x: 0, m_y: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedGridArray { m_x: 0, m_y: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedGridArray { m_x: 0, m_y: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedGridArray { m_x: 0, m_y: 0, m_weight: 0 },
            crate::sexy_tod_lib::tod_common::TodWeightedGridArray { m_x: 0, m_y: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedGridArray { m_x: 0, m_y: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedGridArray { m_x: 0, m_y: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedGridArray { m_x: 0, m_y: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedGridArray { m_x: 0, m_y: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedGridArray { m_x: 0, m_y: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedGridArray { m_x: 0, m_y: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedGridArray { m_x: 0, m_y: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedGridArray { m_x: 0, m_y: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedGridArray { m_x: 0, m_y: 0, m_weight: 0 },
            crate::sexy_tod_lib::tod_common::TodWeightedGridArray { m_x: 0, m_y: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedGridArray { m_x: 0, m_y: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedGridArray { m_x: 0, m_y: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedGridArray { m_x: 0, m_y: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedGridArray { m_x: 0, m_y: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedGridArray { m_x: 0, m_y: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedGridArray { m_x: 0, m_y: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedGridArray { m_x: 0, m_y: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedGridArray { m_x: 0, m_y: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedGridArray { m_x: 0, m_y: 0, m_weight: 0 },
            crate::sexy_tod_lib::tod_common::TodWeightedGridArray { m_x: 0, m_y: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedGridArray { m_x: 0, m_y: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedGridArray { m_x: 0, m_y: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedGridArray { m_x: 0, m_y: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedGridArray { m_x: 0, m_y: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedGridArray { m_x: 0, m_y: 0, m_weight: 0 },
        ];
        let mut a_pick_count = 0;

        // C++: 从第 3 列开始遍历
        let mut a_col = 3;
        while a_col < crate::lawn::board_consts::MAX_GRID_SIZE_X {
            let mut a_row = 0;
            while a_row < crate::lawn::board_consts::MAX_GRID_SIZE_Y {
                let the_board = &mut *self.mBoard;
                if the_board.CanAddGraveStoneAt(a_col, a_row) {
                    let a_has_plant = !the_board.GetTopPlantAt(a_col, a_row, PlantPriority::TOPPLANT_ANY).is_null();
                    // C++: 有权植物 → 1，无植物 → 100000
                    a_picks[a_pick_count as usize].m_weight = if a_has_plant { 1 } else { 100000 };
                    a_picks[a_pick_count as usize].m_x = a_col;
                    a_picks[a_pick_count as usize].m_y = a_row;
                    a_pick_count += 1;
                }
                a_row += 1;
            }
            a_col += 1;
        }

        // C++: theGraveCount = std::min(theGraveCount, aPickCount);
        let the_grave_count = the_grave_count.min(a_pick_count);
        if a_pick_count == 0 || the_grave_count <= 0 {
            return;
        }

        let mut i = 0;
        while i < the_grave_count {
            // C++: TodPickFromWeightedGridArray(aPicks, aPickCount)
            let a_pick: *mut crate::sexy_tod_lib::tod_common::TodWeightedGridArray = match crate::sexy_tod_lib::tod_common::tod_pick_from_weighted_grid_array(&mut a_picks) {
                Some(g) => g,
                None => std::ptr::null_mut(),
            };
            if a_pick.is_null() {
                break;
            }

            // C++: 移除该格植物
            let the_board = &mut *self.mBoard;
            let mut a_plant: *mut crate::lawn::plant::Plant = std::ptr::null_mut();
            while the_board.IteratePlants(&mut a_plant) {
                if (*a_plant).m_plant_col == (*a_pick).m_x && (*a_plant).base.m_row == (*a_pick).m_y {
                    (*a_plant).Die();
                }
            }

            the_board.AddAGraveStone((*a_pick).m_x, (*a_pick).m_y);
            (*a_pick).m_weight = 0;
            i += 1;
        }
    }

    /// C++ Challenge::WhackAZombieSpawning (Challenge.cpp:2775) — 打地鼠僵尸生成
    pub unsafe fn WhackAZombieSpawning(&mut self) {
        let the_board = &mut *self.mBoard;
        if the_board.mCurrentWave == the_board.mNumWaves && the_board.mZombieCountDown == 0 {
            return;
        }

        the_board.mZombieCountDown -= 1;
        if the_board.mZombieCountDown == 100 && the_board.mCurrentWave > 0 {
            // C++: int aNumGraves = 5 - mBoard->GetGraveStonesCount();
            let a_num_graves = 5 - the_board.GetGraveStonesCount();
            self.WhackAZombiePlaceGraves(a_num_graves.max(1));
        }
        if the_board.mZombieCountDown == 5 {
            the_board.NextWaveComing();
        }
        if the_board.mZombieCountDown == 0 {
            the_board.mZombieCountDown = 2000;
            the_board.mZombieCountDownStart = the_board.mZombieCountDown;
            if the_board.mCurrentWave < the_board.mNumWaves {
                the_board.mCurrentWave += 1;
            }
            self.mChallengeStateCounter = if the_board.mCurrentWave == the_board.mNumWaves { 300 } else { 1 };
        } else if the_board.mZombieCountDown < 300 {
            return;
        }

        self.mChallengeStateCounter -= 1;
        if self.mChallengeStateCounter == 0 {
            // C++: 阶段 = ClampInt((mCurrentWave - 1) * 6 / 12, 0, 5)
            let a_phase = crate::sexy_tod_lib::tod_common::clamp_int((the_board.mCurrentWave - 1) * 6 / 12, 0, 5);
            // C++: 各阶段概率表
            let a_double_chance: [i32; 6] = [0, 30, 10, 10, 15, 18];
            let a_triple_chance: [i32; 6] = [0, 0, 0, 0, 10, 13];
            let a_pail_chance: [i32; 6] = [0, 0, 0, 10, 15, 15];
            let a_cone_chance: [i32; 6] = [0, 0, 30, 30, 30, 30];
            let mut a_zombie_count = 1;
            let mut a_zombie_type = ZombieType::ZOMBIE_NORMAL;
            let a_num_hit = crate::sexy_app_framework::common::rand_int() % 100;
            let a_type_hit = crate::sexy_app_framework::common::rand_int() % 100;
            let a_is_final_wave = the_board.mCurrentWave == the_board.mNumWaves;

            // C++: 确定僵尸数量
            if a_is_final_wave {
                a_zombie_count = 20;
            } else if a_num_hit < a_triple_chance[a_phase as usize] {
                a_zombie_count = 3;
            } else if a_num_hit < a_triple_chance[a_phase as usize] + a_double_chance[a_phase as usize] {
                a_zombie_count = 2;
            }

            // C++: 确定僵尸类型
            if a_type_hit < a_pail_chance[a_phase as usize] && a_zombie_count < 3 {
                a_zombie_type = ZombieType::ZOMBIE_PAIL;
            } else if a_type_hit < a_pail_chance[a_phase as usize] + a_cone_chance[a_phase as usize] {
                a_zombie_type = ZombieType::ZOMBIE_TRAFFIC_CONE;
            }

            // C++: 收集可出怪的墓碑（排除墓碑吞噬者）
            let mut a_grid_picks: [crate::sexy_tod_lib::tod_common::TodWeightedArray; 54] = [
                crate::sexy_tod_lib::tod_common::TodWeightedArray { m_item: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedArray { m_item: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedArray { m_item: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedArray { m_item: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedArray { m_item: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedArray { m_item: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedArray { m_item: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedArray { m_item: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedArray { m_item: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedArray { m_item: 0, m_weight: 0 },
                crate::sexy_tod_lib::tod_common::TodWeightedArray { m_item: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedArray { m_item: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedArray { m_item: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedArray { m_item: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedArray { m_item: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedArray { m_item: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedArray { m_item: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedArray { m_item: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedArray { m_item: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedArray { m_item: 0, m_weight: 0 },
                crate::sexy_tod_lib::tod_common::TodWeightedArray { m_item: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedArray { m_item: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedArray { m_item: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedArray { m_item: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedArray { m_item: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedArray { m_item: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedArray { m_item: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedArray { m_item: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedArray { m_item: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedArray { m_item: 0, m_weight: 0 },
                crate::sexy_tod_lib::tod_common::TodWeightedArray { m_item: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedArray { m_item: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedArray { m_item: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedArray { m_item: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedArray { m_item: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedArray { m_item: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedArray { m_item: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedArray { m_item: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedArray { m_item: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedArray { m_item: 0, m_weight: 0 },
                crate::sexy_tod_lib::tod_common::TodWeightedArray { m_item: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedArray { m_item: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedArray { m_item: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedArray { m_item: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedArray { m_item: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedArray { m_item: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedArray { m_item: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedArray { m_item: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedArray { m_item: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedArray { m_item: 0, m_weight: 0 },
                crate::sexy_tod_lib::tod_common::TodWeightedArray { m_item: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedArray { m_item: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedArray { m_item: 0, m_weight: 0 }, crate::sexy_tod_lib::tod_common::TodWeightedArray { m_item: 0, m_weight: 0 },
            ];
            let mut a_grid_picks_count = 0;

            let mut a_grid_item: *mut crate::lawn::grid_item::GridItem = std::ptr::null_mut();
            while the_board.IterateGridItems(&mut a_grid_item) {
                if (*a_grid_item).mGridItemType == GridItemType::GRIDITEM_GRAVESTONE {
                    // C++: 有墓碑吞噬者的墓碑不参与
                    let a_plant = the_board.GetTopPlantAt((*a_grid_item).mGridX, (*a_grid_item).mGridY, PlantPriority::TOPPLANT_ONLY_NORMAL_POSITION);
                    if a_plant.is_null() || (*a_plant).m_seed_type != SeedType::SEED_GRAVEBUSTER {
                        a_grid_picks[a_grid_picks_count as usize].m_item = a_grid_item as isize;
                        a_grid_picks[a_grid_picks_count as usize].m_weight = 1;
                        a_grid_picks_count += 1;
                    }
                }
            }

            // C++: float aMaxSpeed = TodAnimateCurve(1, 12, mCurrentWave, 1, 3, CURVE_EASE_IN);
            let mut a_max_speed = crate::sexy_tod_lib::tod_common::tod_animate_curve(1, 12, the_board.mCurrentWave, 1, 3, crate::const_enums::TodCurves::CURVE_EASE_IN) as f32;
            a_zombie_count = a_zombie_count.min(a_grid_picks_count);

            let mut i = 0;
            while i < a_zombie_count {
                // C++: TodPickArrayItemFromWeightedArray(aGridPicks, aGridPicksCount)
                let a_grid = crate::sexy_tod_lib::tod_common::tod_pick_array_item_from_weighted_array(&mut a_grid_picks[..a_grid_picks_count as usize]);
                if a_grid.is_null() {
                    break;
                }
                let a_grave_stone = (*a_grid).m_item as *mut crate::lawn::grid_item::GridItem;
                (*a_grid).m_weight = 0;

                if a_is_final_wave {
                    // C++: aZombieType = Rand(2) == 0 ? ZOMBIE_TRAFFIC_CONE : ZOMBIE_PAIL;
                    a_zombie_type = if crate::sexy_app_framework::common::rand_int() % 2 == 0 {
                        ZombieType::ZOMBIE_TRAFFIC_CONE
                    } else {
                        ZombieType::ZOMBIE_PAIL
                    };
                    a_max_speed = 2.0;
                }

                let a_zombie = the_board.AddZombie(a_zombie_type, the_board.mCurrentWave);
                if a_zombie.is_null() {
                    break;
                }

                // C++: aZombie->RiseFromGrave(aGraveStone->mGridX, aGraveStone->mGridY);
                (*a_zombie).RiseFromGrave((*a_grave_stone).mGridX, (*a_grave_stone).mGridY);
                (*a_zombie).m_phase_counter = 50;
                // C++: mVelX = RandRangeFloat(0.5f, aMaxSpeed);
                (*a_zombie).m_vel_x = crate::sexy_tod_lib::tod_common::rand_range_float(0.5, a_max_speed);
                (*a_zombie).UpdateAnimSpeed();
                i += 1;
            }

            // C++: 设置下一波间隔
            let a_state_counter_min = crate::sexy_tod_lib::tod_common::tod_animate_curve(1, 12, the_board.mCurrentWave, 100, 30, crate::const_enums::TodCurves::CURVE_LINEAR);
            let a_state_counter_max = crate::sexy_tod_lib::tod_common::tod_animate_curve(1, 12, the_board.mCurrentWave, 200, 60, crate::const_enums::TodCurves::CURVE_LINEAR);
            self.mChallengeStateCounter = crate::sexy_tod_lib::tod_common::rand_range_int(a_state_counter_min, a_state_counter_max);
            if a_is_final_wave {
                the_board.mZombieCountDown = 0;
                self.mChallengeStateCounter = 0;
            }
        }
    }

    /// C++ Challenge::WhackAZombieUpdate (Challenge.cpp:5146) — 打地鼠更新
    pub unsafe fn WhackAZombieUpdate(&mut self) {
        let the_board = &mut *self.mBoard;
        // C++: 教程状态机
        if the_board.mSunMoney > 0 && the_board.mTutorialState == TutorialState::TUTORIAL_OFF as i32 {
            the_board.SetTutorialState(TutorialState::TUTORIAL_WHACK_A_ZOMBIE_BEFORE_PICK_SEED as i32);
            the_board.mTutorialTimer = 1500;
        }
        if the_board.mTutorialState == TutorialState::TUTORIAL_WHACK_A_ZOMBIE_BEFORE_PICK_SEED as i32
            && the_board.mTutorialTimer == 0
        {
            the_board.SetTutorialState(TutorialState::TUTORIAL_WHACK_A_ZOMBIE_PICK_SEED as i32);
            the_board.mTutorialTimer = 400;
        }
        if the_board.mTutorialState == TutorialState::TUTORIAL_WHACK_A_ZOMBIE_PICK_SEED as i32
            && the_board.mTutorialTimer == 0
        {
            the_board.SetTutorialState(TutorialState::TUTORIAL_WHACK_A_ZOMBIE_COMPLETED as i32);
        }
    }

    /// C++ Challenge::IZombieSeedTypeToZombieType (Challenge.cpp:4233) — 种子转僵尸类型
    pub fn IZombieSeedTypeToZombieType(the_seed_type: SeedType) -> ZombieType {
        match the_seed_type {
            SeedType::SEED_ZOMBIE_NORMAL => ZombieType::ZOMBIE_NORMAL,
            SeedType::SEED_ZOMBIE_TRAFFIC_CONE => ZombieType::ZOMBIE_TRAFFIC_CONE,
            SeedType::SEED_ZOMBIE_POLEVAULTER => ZombieType::ZOMBIE_POLEVAULTER,
            SeedType::SEED_ZOMBIE_PAIL => ZombieType::ZOMBIE_PAIL,
            SeedType::SEED_ZOMBIE_LADDER => ZombieType::ZOMBIE_LADDER,
            SeedType::SEED_ZOMBIE_DIGGER => ZombieType::ZOMBIE_DIGGER,
            SeedType::SEED_ZOMBIE_BUNGEE => ZombieType::ZOMBIE_BUNGEE,
            SeedType::SEED_ZOMBIE_FOOTBALL => ZombieType::ZOMBIE_FOOTBALL,
            SeedType::SEED_ZOMBIE_BALLOON => ZombieType::ZOMBIE_BALLOON,
            SeedType::SEED_ZOMBIE_SCREEN_DOOR => ZombieType::ZOMBIE_DOOR,
            SeedType::SEED_ZOMBONI => ZombieType::ZOMBIE_ZAMBONI,
            SeedType::SEED_ZOMBIE_POGO => ZombieType::ZOMBIE_POGO,
            SeedType::SEED_ZOMBIE_DANCER => ZombieType::ZOMBIE_DANCER,
            SeedType::SEED_ZOMBIE_GARGANTUAR => ZombieType::ZOMBIE_GARGANTUAR,
            SeedType::SEED_ZOMBIE_IMP => ZombieType::ZOMBIE_IMP,
            _ => ZombieType::ZOMBIE_NORMAL,
        }
    }

    /// C++ Challenge::IZombiePlaceZombie (Challenge.cpp:4258) — 放置僵尸
    pub unsafe fn IZombiePlaceZombie(&mut self, the_zombie_type: ZombieType, the_grid_x: i32, the_grid_y: i32) {
        let the_board = &mut *self.mBoard;
        let a_zombie = the_board.AddZombieInRow(the_zombie_type, the_grid_y, 0);
        if a_zombie.is_null() {
            return;
        }

        if the_zombie_type == ZombieType::ZOMBIE_BUNGEE {
            // C++: 蹦极僵尸定点放置
            (*a_zombie).m_target_col = the_grid_x;
            (*a_zombie).SetRow(the_grid_y);
            (*a_zombie).m_pos_x = the_board.GridToPixelX(the_grid_x, the_grid_y) as f32;
            (*a_zombie).m_pos_y = (*a_zombie).GetPosYBasedOnRow(the_grid_y);
            (*a_zombie).base.m_render_order = crate::lawn::board::Board::MakeRenderOrder(RenderLayer::RENDER_LAYER_GRAVE_STONE, the_grid_y, 7);
        } else {
            // C++: aZombie->mPosX = mBoard->GridToPixelX(theGridX, theGridY) - 30.0f;
            (*a_zombie).m_pos_x = the_board.GridToPixelX(the_grid_x, the_grid_y) as f32 - 30.0;
        }
    }

    /// C++ Challenge::IZombieStart (Challenge.cpp:4640)
    pub unsafe fn IZombieStart(&mut self) {
        let the_board = &mut *self.mBoard;
        the_board.DisplayAdvice("[ADVICE_I_ZOMBIE_EAT_ALL_BRAINS]", 0, AdviceType::ADVICE_I_ZOMBIE_EAT_ALL_BRAINS as i32);
    }

    /// C++ Challenge::IZombieUpdate (Challenge.cpp:4645) — IZombie 更新
    pub unsafe fn IZombieUpdate(&mut self) {
        let the_board = &mut *self.mBoard;
        let a_sun_money = the_board.mSunMoney + the_board.CountSunBeingCollected();

        // C++: 空闲僵尸随机变速
        let mut a_zombie: *mut crate::lawn::zombie::Zombie = std::ptr::null_mut();
        while the_board.IterateZombies(&mut a_zombie) {
            if !(*a_zombie).IsDeadOrDying()
                && (*a_zombie).m_zombie_phase != ZombiePhase::PHASE_POLEVAULTER_IN_VAULT
                && !(*a_zombie).m_is_eating
                && (*a_zombie).m_just_got_shot_counter < -500
            {
                (*a_zombie).PickRandomSpeed();
            }
        }

        // C++: 活跃植物检测（倭瓜/大嘴花）
        let mut an_active = false;
        let mut a_plant: *mut crate::lawn::plant::Plant = std::ptr::null_mut();
        while the_board.IteratePlants(&mut a_plant) {
            let a_state = (*a_plant).m_state;
            if a_state == crate::lawn::plant::PlantState::STATE_SQUASH_FALLING
                || a_state == crate::lawn::plant::PlantState::STATE_SQUASH_DONE_FALLING
                || a_state == crate::lawn::plant::PlantState::STATE_CHOMPER_BITING
                || a_state == crate::lawn::plant::PlantState::STATE_CHOMPER_BITING_GOT_ONE
            {
                an_active = true;
            }
        }
        // [TODO]: IterateParticles + PARTICLE_POTATO_MINE 检测（粒子系统未翻译）

        // C++: 可用阳光检测
        let mut a_has_available_sun = false;
        let mut a_coin_for_sun: *mut crate::lawn::coin::Coin = std::ptr::null_mut();
        while the_board.IterateCoins(&mut a_coin_for_sun) {
            if (*a_coin_for_sun).IsSun() && !(*a_coin_for_sun).m_is_being_collected && !(*a_coin_for_sun).m_dead {
                a_has_available_sun = true;
                break;
            }
        }

        // C++: 无僵尸 + 阳光不足 50 + 无活跃动作 → 僵尸获胜
        if the_board.mZombies.m_size == 0
            && a_sun_money < 50
            && !the_board.mLevelAwardSpawned
            && !an_active
            && !a_has_available_sun
        {
            let mut a_coin: *mut crate::lawn::coin::Coin = std::ptr::null_mut();
            while the_board.IterateCoins(&mut a_coin) {
                if (*a_coin).IsMoney() {
                    (*a_coin).Die();
                }
            }

            the_board.ZombiesWon(std::ptr::null_mut());
        }
    }

    /// C++ Challenge::IZombieSetPlantFilterEffect (Challenge.cpp:4726) — 植物滤镜
    pub unsafe fn IZombieSetPlantFilterEffect(&mut self, the_plant: *mut crate::lawn::plant::Plant, _the_filter_effect: FilterEffect) {
        if the_plant.is_null() {
            return;
        }
        // C++: 遍历植物 4 个 Reanimation（body/head/head2/head3）设置 mFilterEffect
        // [TODO]: Reanimation 系统翻译后实现
        let _ = (*the_plant).m_body_reanim_id;
        let _ = (*the_plant).m_head_reanim_id;
        let _ = (*the_plant).m_head_reanim_id2;
        let _ = (*the_plant).m_head_reanim_id3;
    }

    /// C++ Challenge::UpdateRainingSeeds (Challenge.cpp:1919) — 种子雨更新
    pub unsafe fn UpdateRainingSeeds(&mut self) {
        let the_board = &mut *self.mBoard;
        if the_board.mLevelAwardSpawned {
            return;
        }
        self.mChallengeStateCounter -= 1;
        if self.mChallengeStateCounter != 0 {
            return;
        }

        self.mChallengeStateCounter = crate::sexy_tod_lib::tod_common::rand_range_int(500, 999);

        // C++: AddCoin(..., COIN_USABLE_SEED_PACKET, COIN_MOTION_FROM_SKY_SLOW)
        let a_coin = the_board.AddCoin(
            crate::sexy_tod_lib::tod_common::rand_range_int(100, 649),
            60,
            CoinType::COIN_USABLE_SEED_PACKET,
            CoinMotion::COIN_MOTION_FROM_SKY_SLOW,
        );

        // C++: 随机选种子，排除升级/向日葵/咖啡/荷叶等
        let the_app = self.app();
        let mut a_seed_type;
        loop {
            a_seed_type = std::mem::transmute::<i32, SeedType>(crate::sexy_app_framework::common::rand_int() % (*the_app).GetSeedsAvailable());
            let a_not_recommended = {
                // [TODO]: SeedNotRecommendedForLevel 完整翻译
                false
            };
            if a_not_recommended
                || !(*the_app).HasSeedType(a_seed_type)
                || crate::lawn::plant::Plant::is_upgrade(a_seed_type)
                || a_seed_type == SeedType::SEED_SUNFLOWER
                || a_seed_type == SeedType::SEED_TWINSUNFLOWER
                || a_seed_type == SeedType::SEED_INSTANT_COFFEE
                || a_seed_type == SeedType::SEED_UMBRELLA
                || a_seed_type == SeedType::SEED_SUNSHROOM
                || a_seed_type == SeedType::SEED_IMITATER
            {
                continue;
            }
            break;
        }

        // C++: 荷叶数量越多，掉落荷叶概率越高
        if crate::sexy_app_framework::common::rand_int() % 100
            < crate::sexy_tod_lib::tod_common::tod_animate_curve(0, 18, the_board.CountPlantByType(SeedType::SEED_LILYPAD), 30, 1, crate::const_enums::TodCurves::CURVE_LINEAR)
        {
            a_seed_type = SeedType::SEED_LILYPAD;
        }

        if !a_coin.is_null() {
            (*a_coin).m_usable_seed_type = a_seed_type;
        }
    }

    /// C++ Challenge::UpdatePortalCombat — 传送门更新
    pub unsafe fn UpdatePortalCombat(&mut self) {
        // [TODO]: PortalCombat 系列
    }

    /// C++ Challenge::ZombiquariumSpawnSnorkle (Challenge.cpp:3545)
    pub unsafe fn ZombiquariumSpawnSnorkle(&mut self) -> *mut crate::lawn::zombie::Zombie {
        // C++: AddZombieInRow(ZOMBIE_SNORKEL, 0, 0)
        let the_board = &mut *self.mBoard;
        let a_zombie = the_board.AddZombieInRow(ZombieType::ZOMBIE_SNORKEL, 0, 0);
        if !a_zombie.is_null() {
            (*a_zombie).m_pos_x = crate::sexy_tod_lib::tod_common::rand_range_float(50.0, 650.0);
            (*a_zombie).m_pos_y = crate::sexy_tod_lib::tod_common::rand_range_float(100.0, 400.0);
        }
        a_zombie
    }

    /// C++ Challenge::ZombiquariumPacketClicked (Challenge.cpp:3553) — 点击种子包
    pub unsafe fn ZombiquariumPacketClicked(&mut self, the_seed_packet: *mut crate::lawn::seed_packet::SeedPacket) {
        let the_board = &mut *self.mBoard;
        let a_cost = the_board.GetCurrentPlantCost((*the_seed_packet).mPacketType, SeedType::SEED_NONE);
        if !the_board.CanTakeSunMoney(a_cost) {
            return;
        }

        if (*the_seed_packet).mPacketType == SeedType::SEED_ZOMBIQUARIUM_SNORKLE {
            if the_board.CountZombiesOnScreen() > 100 {
                return;
            }

            if the_board.mTutorialState == TutorialState::TUTORIAL_ZOMBIQUARIUM_BUY_SNORKEL as i32 {
                the_board.ClearAdvice(AdviceType::ADVICE_ZOMBIQUARIUM_BUY_SNORKEL as i32);
                the_board.TutorialArrowRemove();
                the_board.mTutorialState = TutorialState::TUTORIAL_ZOMBIQUARIUM_BOUGHT_SNORKEL as i32;
            }

            let a_zombie = self.ZombiquariumSpawnSnorkle();
            // [TODO]: mApp->PlayFoley(FOLEY_ZOMBIESPLASH); AddTodParticle(PARTICLE_PLANTING_POOL)
            let _ = a_zombie;
        } else if (*the_seed_packet).mPacketType == SeedType::SEED_ZOMBIQUARIUM_TROPHY {
            // C++: SpawnLevelAward(2, 0)
            // [TODO]: SpawnLevelAward
            the_board.ClearAdvice(AdviceType::ADVICE_NONE as i32);
        }

        the_board.TakeSunMoney(a_cost);
    }

    /// C++ Challenge::ZombiquariumDropBrain (Challenge.cpp:3584) — 投放大脑
    pub unsafe fn ZombiquariumDropBrain(&mut self, x: i32, y: i32) {
        let the_board = &mut *self.mBoard;
        the_board.ClearAdvice(AdviceType::ADVICE_ZOMBIQUARIUM_CLICK_TO_FEED as i32);
        // C++: GridItem* aBrain = mBoard->mGridItems.DataArrayAlloc();
        let a_brain = the_board.mGridItems.data_array_alloc();
        if a_brain.is_null() {
            return;
        }
        (*a_brain).mGridItemType = GridItemType::GRIDITEM_BRAIN;
        (*a_brain).mRenderOrder = 400000;
        (*a_brain).mGridX = 0;
        (*a_brain).mGridY = 0;
        (*a_brain).mGridItemCounter = 0;
        (*a_brain).mPosX = (x - 15) as f32;
        (*a_brain).mPosY = (y - 15) as f32;
        // [TODO]: mApp->PlaySample(SOUND_TAP)
    }

    /// C++ Challenge::ZombiquariumMouseDown (Challenge.cpp:3598) — 水族馆内点击
    pub unsafe fn ZombiquariumMouseDown(&mut self, x: i32, y: i32) {
        if x < 80 || x > 720 || y < 90 || y > 430 {
            return;
        }

        // C++: 统计大脑数量（最多 3 个）
        let mut a_brains_count = 0;
        let the_board = &mut *self.mBoard;
        let mut a_grid_item: *mut crate::lawn::grid_item::GridItem = std::ptr::null_mut();
        while the_board.IterateGridItems(&mut a_grid_item) {
            if (*a_grid_item).mGridItemType == GridItemType::GRIDITEM_BRAIN {
                a_brains_count += 1;
            }
        }
        if a_brains_count < 3 && the_board.TakeSunMoney(5) {
            self.ZombiquariumDropBrain(x, y);
        }
    }

    /// C++ Challenge::ZombiquariumUpdate (Challenge.cpp:3618) — 僵尸水族馆更新
    pub unsafe fn ZombiquariumUpdate(&mut self) {
        let the_board = &mut *self.mBoard;
        // C++: 无僵尸且未掉奖励 → 僵尸获胜
        if the_board.mZombies.m_size == 0 && !the_board.mLevelAwardSpawned {
            the_board.ZombiesWon(std::ptr::null_mut());
            return;
        }

        // C++: 提示收集阳光
        // [TODO]: mBoard->mAdvice->IsBeingDisplayed() 检查 + DisplayAdvice(ADVICE_ZOMBIQUARIUM_COLLECT_SUN)

        // C++: 进度条
        let a_score = crate::sexy_tod_lib::tod_common::clamp_int(the_board.mSunMoney, 0, ZOMBIQUARIUM_WINNING_SCORE);
        the_board.mProgressMeterWidth = crate::sexy_tod_lib::tod_common::tod_animate_curve(
            0, ZOMBIQUARIUM_WINNING_SCORE, a_score, 0, crate::lawn::board_consts::PROGRESS_METER_COUNTER,
            crate::const_enums::TodCurves::CURVE_LINEAR,
        );
        if a_score >= ZOMBIQUARIUM_WINNING_SCORE - 100 {
            the_board.DisplayAdvice("[ADVICE_ALMOST_THERE]", 0, AdviceType::ADVICE_ALMOST_THERE as i32);
        }

        // C++: 教程：得分 ≥110 提示买潜水僵尸
        if a_score >= 110 && the_board.mTutorialState == TutorialState::TUTORIAL_OFF as i32 {
            the_board.mTutorialState = TutorialState::TUTORIAL_ZOMBIQUARIUM_BUY_SNORKEL as i32;
            let a_pos_x = if the_board.mSeedBank.is_null() { 0.0 } else { unsafe { (*the_board.mSeedBank).mX as f32 } };
            let a_pos_y = if the_board.mSeedBank.is_null() { 0.0 } else { unsafe { (*the_board.mSeedBank).mY as f32 } };
            the_board.TutorialArrowShow(a_pos_x, a_pos_y);
            the_board.DisplayAdvice("[ADVICE_ZOMBIQUARIUM_BUY_SNORKEL]", 0, AdviceType::ADVICE_ZOMBIQUARIUM_BUY_SNORKEL as i32);
        } else if a_score < 100 && the_board.mTutorialState == TutorialState::TUTORIAL_ZOMBIQUARIUM_BUY_SNORKEL as i32 {
            the_board.TutorialArrowRemove();
            the_board.ClearAdvice(AdviceType::ADVICE_ZOMBIQUARIUM_BUY_SNORKEL as i32);
            the_board.mTutorialState = TutorialState::TUTORIAL_OFF as i32;
        }

        // C++: 教程：得分满 → 提示点击奖杯
        if a_score >= ZOMBIQUARIUM_WINNING_SCORE && the_board.mTutorialState == TutorialState::TUTORIAL_ZOMBIQUARIUM_BOUGHT_SNORKEL as i32 {
            the_board.mTutorialState = TutorialState::TUTORIAL_ZOMBIQUARIUM_CLICK_TROPHY as i32;
            the_board.TutorialArrowShow(0.0, 0.0);
            the_board.DisplayAdvice("[ADVICE_ZOMBIQUARIUM_CLICK_TROPHY]", 0, AdviceType::ADVICE_ZOMBIQUARIUM_CLICK_TROPHY as i32);
        } else if a_score < ZOMBIQUARIUM_WINNING_SCORE && the_board.mTutorialState == TutorialState::TUTORIAL_ZOMBIQUARIUM_CLICK_TROPHY as i32 {
            the_board.TutorialArrowRemove();
            the_board.ClearAdvice(AdviceType::ADVICE_ZOMBIQUARIUM_CLICK_TROPHY as i32);
            the_board.mTutorialState = TutorialState::TUTORIAL_ZOMBIQUARIUM_BOUGHT_SNORKEL as i32;
        }

        // C++: 大脑下落
        let mut a_grid_item: *mut crate::lawn::grid_item::GridItem = std::ptr::null_mut();
        while the_board.IterateGridItems(&mut a_grid_item) {
            if (*a_grid_item).mGridItemType == GridItemType::GRIDITEM_BRAIN {
                (*a_grid_item).mGridItemCounter += 1;
                (*a_grid_item).mPosY += 0.15;
                if (*a_grid_item).mPosY >= 500.0 {
                    (*a_grid_item).GridItemDie();
                }
            }
        }
    }

    /// C++ Challenge::TreeOfWisdomUpdate — 智慧树更新
    pub unsafe fn TreeOfWisdomUpdate(&mut self) {
        // [TODO]: TreeOfWisdom 系列
    }

    /// C++ Challenge::LastStandUpdate (Challenge.cpp:5097) — 最后一战更新
    pub unsafe fn LastStandUpdate(&mut self) {
        let the_board = &mut *self.mBoard;
        // C++: 显示开始/继续猛攻按钮
        if the_board.mNextSurvivalStageCounter == 0
            && self.mChallengeState == ChallengeState::STATECHALLENGE_NORMAL
            && !the_board.mStoreButton.is_null()
            && (*the_board.mStoreButton).mBtnNoDraw
        {
            let a_button = the_board.mStoreButton;
            (*a_button).mBtnNoDraw = false;
            (*a_button).mDisabled = false;

            if self.mSurvivalStage == 0 {
                (*a_button).SetLabel("[START_ONSLAUGHT]");
                (*a_button).Resize(300, 559, 210, 46);
            } else {
                (*a_button).SetLabel("[CONTINUE_ONSLAUGHT]");
                (*a_button).Resize(270, 559, 257, 46);
            }
        }

        // C++: 猛攻状态下累计计数
        if self.mChallengeState == ChallengeState::STATECHALLENGE_LAST_STAND_ONSLAUGHT
            && (*self.mApp).mGameScene == GameScenes::SCENE_PLAYING
        {
            self.mChallengeStateCounter += 1;
        }
    }

    /// C++ Challenge::LastStandCompletedStage (Challenge.cpp:5121) — 阶段完成
    pub unsafe fn LastStandCompletedStage(&mut self) {
        let the_app = self.app();
        // [TODO]: mApp->PlaySample(SOUND_HUGE_WAVE)
        self.mChallengeState = ChallengeState::STATECHALLENGE_NORMAL;
        let the_board = &mut *self.mBoard;
        if !the_board.mSeedBank.is_null() {
            (*(*the_board).mSeedBank).RefreshAllPackets();
        }

        // C++: 消化/装填/充磁植物状态压缩
        let mut a_plant: *mut crate::lawn::plant::Plant = std::ptr::null_mut();
        while the_board.IteratePlants(&mut a_plant) {
            if (*a_plant).m_state == crate::lawn::plant::PlantState::STATE_CHOMPER_DIGESTING
                || (*a_plant).m_state == crate::lawn::plant::PlantState::STATE_COBCANNON_ARMING
                || (*a_plant).m_state == crate::lawn::plant::PlantState::STATE_MAGNETSHROOM_SUCKING
                || (*a_plant).m_state == crate::lawn::plant::PlantState::STATE_MAGNETSHROOM_CHARGING
            {
                (*a_plant).m_state_countdown = (*a_plant).m_state_countdown.min(200);
            }
        }

        // C++: 提示成功防守 + 下一阶段
        let a_flag_str = crate::lawn_app::LawnApp::Pluralize(the_board.GetSurvivalFlagsCompleted(), "[ONE_FLAG]", "[COUNT_FLAGS]");
        let a_msg = crate::sexy_tod_lib::tod_common::tod_replace_string("[SUCCESSFULLY_DEFENDED]", "{FLAGS}", &a_flag_str);
        the_board.DisplayAdvice(&a_msg, 0, AdviceType::ADVICE_NONE as i32);

        self.mSurvivalStage += 1;
        the_board.mLevelComplete = false;
        the_board.InitZombieWavesForLevel(the_board.mLevel);
    }

    /// C++ Challenge::UpdateConveyorBelt() (from Challenge.cpp:1616)
    /// C++: Challenge::SlotMachineGetHandleRect (Challenge.cpp:1339)
    pub unsafe fn SlotMachineGetHandleRect(&self) -> crate::sexy_app_framework::misc::rect::Rect {
        // C++: return Rect(mBoard->mSeedBank->mX + 473, mBoard->mSeedBank->mY, 55, 80);
        let the_board = &*self.mBoard;
        let a_seed_bank = the_board.mSeedBank;
        let (a_bank_x, a_bank_y) = if a_seed_bank.is_null() {
            (0, 0)
        } else {
            unsafe { ((*a_seed_bank).mX, (*a_seed_bank).mY) }
        };
        crate::sexy_app_framework::misc::rect::Rect::new(a_bank_x + 473, a_bank_y, 55, 80)
    }

    /// C++: Challenge::UpdateSlotMachine (Challenge.cpp:2001) — 老虎机更新
    pub unsafe fn UpdateSlotMachine(&mut self) {
        let the_board = &mut *self.mBoard;

        // C++: int aSunMoney = ClampInt(mBoard->mSunMoney, 0, 2000);
        let a_sun_money = crate::sexy_tod_lib::tod_common::clamp_int(the_board.mSunMoney, 0, 2000);
        if a_sun_money >= SLOT_MACHINE_WINNING_SCORE - 100 {
            the_board.DisplayAdvice("[ADVICE_ALMOST_THERE]", 0 /* MESSAGE_STYLE_HINT_FAST */, AdviceType::ADVICE_ALMOST_THERE as i32);
        }
        if a_sun_money >= 2000 {
            // [TODO]: SpawnLevelAward(4, 2)
            the_board.ClearAdvice(AdviceType::ADVICE_NONE as i32);
        }
        // C++: mBoard->mProgressMeterWidth = TodAnimateCurve(0, SLOT_MACHINE_WINNING_SCORE, aSunMoney, 0, PROGRESS_METER_COUNTER, CURVE_LINEAR);
        the_board.mProgressMeterWidth = crate::sexy_tod_lib::tod_common::tod_animate_curve(
            0, SLOT_MACHINE_WINNING_SCORE, a_sun_money, 0, crate::lawn::board_consts::PROGRESS_METER_COUNTER,
            crate::const_enums::TodCurves::CURVE_LINEAR,
        );

        // C++: 提示收集阳光
        // [TODO]: mBoard->mAdvice->IsBeingDisplayed() 检查 + DisplayAdvice(ADVICE_SLOT_MACHINE_COLLECT_SUN)

        // C++: if (mChallengeState == STATECHALLENGE_SLOT_MACHINE_ROLLING)
        if self.mChallengeState == ChallengeState::STATECHALLENGE_SLOT_MACHINE_ROLLING {
            let mut a_machine_finished = true;
            let a_seed_bank = the_board.mSeedBank;
            if !a_seed_bank.is_null() {
                // C++: 三个滚轮倒计时
                let mut i = 0;
                while i < 3 {
                    let a_packet = &mut (*(*a_seed_bank).mSeedPackets.as_mut_ptr().add(i));
                    if a_packet.mSlotMachineCountDown > 0 {
                        a_packet.mSlotMachineCountDown -= 1;
                        if a_packet.mSlotMachineCountDown == 0 {
                            a_packet.mPacketType = a_packet.mSlotMachiningNextSeed;
                            // [TODO]: Reanimation 滚轮动画
                        }
                    }
                    if a_packet.mSlotMachineCountDown > 0 {
                        a_machine_finished = false;
                    }
                    i += 1;
                }
            }

            if a_machine_finished {
                self.mChallengeState = ChallengeState::STATECHALLENGE_NORMAL;
                let a_seed_bank = the_board.mSeedBank;

                let a_packet1 = (*(*a_seed_bank).mSeedPackets.as_ptr()).mPacketType;
                let a_packet2 = (*(*a_seed_bank).mSeedPackets.as_ptr().add(1)).mPacketType;
                let a_packet3 = (*(*a_seed_bank).mSeedPackets.as_ptr().add(2)).mPacketType;
                if a_packet1 != a_packet2 || a_packet2 != a_packet3 {
                    // C++: 两个相同
                    if a_packet1 == a_packet2 || a_packet2 == a_packet3 || a_packet1 == a_packet3 {
                        // [TODO]: mApp->PlayFoley(FOLEY_ART_CHALLENGE)
                        let a_seed_type = if a_packet1 == a_packet2 || a_packet1 == a_packet3 { a_packet1 } else { a_packet2 };
                        if a_seed_type == SeedType::SEED_SLOT_MACHINE_DIAMOND {
                            the_board.DisplayAdvice("[ADVICE_SLOT_MACHINE_2_DIAMONDS]", 0, AdviceType::ADVICE_NONE as i32);
                            the_board.AddCoin(360, 85, CoinType::COIN_DIAMOND, CoinMotion::COIN_MOTION_COIN);
                        } else if a_seed_type == SeedType::SEED_SLOT_MACHINE_SUN {
                            the_board.DisplayAdvice("[ADVICE_SLOT_MACHINE_2_SUNS]", 0, AdviceType::ADVICE_NONE as i32);
                            let mut i = 0;
                            while i < 4 {
                                the_board.AddCoin(320 + i * 15, 85, CoinType::COIN_SUN, CoinMotion::COIN_MOTION_COIN);
                                i += 1;
                            }
                        } else {
                            the_board.DisplayAdvice("[ADVICE_SLOT_MACHINE_2_OF_A_KIND]", 0, AdviceType::ADVICE_NONE as i32);
                            let a_coin = the_board.AddCoin(360, 85, CoinType::COIN_USABLE_SEED_PACKET, CoinMotion::COIN_MOTION_COIN);
                            if !a_coin.is_null() {
                                (*a_coin).m_usable_seed_type = a_seed_type;
                            }
                        }
                    }
                } else {
                    // C++: 三个相同（Jackpot）
                    // [TODO]: mApp->PlayFoley(FOLEY_ART_CHALLENGE)
                    if a_packet1 == SeedType::SEED_SLOT_MACHINE_DIAMOND {
                        the_board.DisplayAdvice("[ADVICE_SLOT_MACHINE_DIAMOND_JACKPOT]", 0, AdviceType::ADVICE_NONE as i32);
                        let mut i = 0;
                        while i < 5 {
                            the_board.AddCoin(320 + i * 12, 85, CoinType::COIN_DIAMOND, CoinMotion::COIN_MOTION_COIN);
                            i += 1;
                        }
                    } else if a_packet1 == SeedType::SEED_SLOT_MACHINE_SUN {
                        the_board.DisplayAdvice("[ADVICE_SLOT_MACHINE_SUN_JACKPOT]", 0, AdviceType::ADVICE_NONE as i32);
                        let mut i = 0;
                        while i < 20 {
                            the_board.AddCoin(320 + i * 3, 85, CoinType::COIN_SUN, CoinMotion::COIN_MOTION_COIN);
                            i += 1;
                        }
                    } else {
                        the_board.DisplayAdvice("[ADVICE_SLOT_MACHINE_3_OF_A_KIND]", 0, AdviceType::ADVICE_NONE as i32);
                        let mut i = 0;
                        while i < 3 {
                            let a_coin = the_board.AddCoin(320 + i * 20, 85, CoinType::COIN_USABLE_SEED_PACKET, CoinMotion::COIN_MOTION_COIN);
                            if !a_coin.is_null() {
                                (*a_coin).m_usable_seed_type = a_packet1;
                            }
                            i += 1;
                        }
                    }
                }
            }
        }
    }

    /// C++: Challenge::BeghouledCheckStuckState (Challenge.cpp:2094)
    pub unsafe fn BeghouledCheckStuckState(&mut self) {
        if self.mChallengeState != ChallengeState::STATECHALLENGE_NORMAL {
            return;
        }
        let the_board = &mut *self.mBoard;
        if the_board.mLevelAwardSpawned {
            return;
        }

        let mut a_board_state = BeghouledBoardState::new();
        self.LoadBeghouledBoardState(&mut a_board_state);
        if !self.BeghouledCheckForPossibleMoves(&mut a_board_state) {
            self.mChallengeState = ChallengeState::STATECHALLENGE_BEGHOULED_NO_MATCHES;
            self.mChallengeStateCounter = 500;
            the_board.DisplayAdviceAgain("[ADVICE_BEGHOULED_NO_MOVES]", 0, AdviceType::ADVICE_BEGHOULED_NO_MOVES as i32);
        }
    }

    /// C++: Challenge::ZombieAtePlant (Challenge.cpp:2109) — 僵尸吃掉宝石植物
    pub unsafe fn ZombieAtePlant(&mut self, the_plant: *mut crate::lawn::plant::Plant) {
        if (*self.mApp).mGameMode != GameMode::GAMEMODE_CHALLENGE_BEGHOULED
            && (*self.mApp).mGameMode != GameMode::GAMEMODE_CHALLENGE_BEGHOULED_TWIST
        {
            return;
        }

        let the_board = &mut *self.mBoard;
        // C++: mBeghouledEated[thePlant->mPlantCol][thePlant->mRow] = true;
        if !the_plant.is_null() {
            self.m_beghouled_eated[(*the_plant).m_plant_col as usize][(*the_plant).base.m_row as usize] = 1;
        }

        // C++: 吃满 4 个植物解锁陨石按钮
        let a_seed_bank = the_board.mSeedBank;
        if !a_seed_bank.is_null() && (*a_seed_bank).mNumPackets == 4 {
            (*a_seed_bank).mNumPackets += 1;
            (*(*a_seed_bank).mSeedPackets.as_mut_ptr().add(4)).SetPacketType(SeedType::SEED_BEGHOULED_BUTTON_CRATER);
            the_board.DisplayAdvice("[ADVICE_BEGHOULED_USE_CRATER_1]", 0, AdviceType::ADVICE_BEGHOULED_USE_CRATER_1 as i32);
        }

        self.BeghouledCheckStuckState();
        // [TODO]: BeghouledUpdateCraters
    }
    pub unsafe fn UpdateConveyorBelt(&mut self) {
        let _board = self.board();
        if self.mConveyorBeltCounter > 0 {
            self.mConveyorBeltCounter -= 1;
            if self.mConveyorBeltCounter == 0 {
                self.mConveyorBeltCounter = 1000;
                // Add a random seed to the conveyor belt bank
                // mBoard->mSeedBank->AddSeed(PickConveyorBeltSeed());
            }
        }
    }

    pub unsafe fn PlantAdded(&mut self, _thePlant: *mut crate::lawn::plant::Plant) {
        // Challenge::PlantAdded - track plants for scoring
    }

    pub unsafe fn CanPlantAt(&self, _theGridX: i32, _theGridY: i32, _theSeedType: SeedType) -> PlantingReason {
        // Challenge-specific planting rules
        PlantingReason::PLANTING_OK
    }

    pub unsafe fn IsZombieSeedType(theSeedType: SeedType) -> bool {
        matches!(theSeedType,
            SeedType::SEED_ZOMBIE_NORMAL | SeedType::SEED_ZOMBIE_TRAFFIC_CONE
            | SeedType::SEED_ZOMBIE_PAIL | SeedType::SEED_ZOMBIE_SCREEN_DOOR
            | SeedType::SEED_ZOMBIE_FOOTBALL | SeedType::SEED_ZOMBIE_DANCER
            | SeedType::SEED_ZOMBIE_BALLOON | SeedType::SEED_ZOMBIE_DIGGER
            | SeedType::SEED_ZOMBIE_POGO | SeedType::SEED_ZOMBIE_BUNGEE
            | SeedType::SEED_ZOMBIE_LADDER | SeedType::SEED_ZOMBIE_GARGANTUAR
            | SeedType::SEED_ZOMBIE_IMP | SeedType::SEED_ZOMBIE_POLEVAULTER
        )
    }

    /// C++ Challenge::SpawnZombieWave (Challenge.cpp:2964)
    pub unsafe fn SpawnZombieWave(&mut self) {
        let board = self.board();
        let app = self.app();

        if (*app).IsContinuousChallenge() && board.mCurrentWave == board.mNumWaves {
            board.mCurrentWave = board.mNumWaves - 1;
            // [TODO]: Replace flag zombies with normal
        }

        let aIsFlagWave = board.IsFlagWave(board.mCurrentWave);

        if (*app).mGameMode as i32 == GameMode::GAMEMODE_CHALLENGE_GRAVE_DANGER as i32
            && board.mCurrentWave != board.mNumWaves - 1
        {
            if aIsFlagWave {
                // [TODO]: board.SpawnZombiesFromGraves()
            } else if board.mCurrentWave > 5 {
                // [TODO]: GraveDangerSpawnRandomGrave()
            }
        }

        // 生存模式夜间：最后一波补充墓碑 (inline is_survival_mode check)
        let mode = (*app).mGameMode as i32;
        let is_survival = mode >= GameMode::GAMEMODE_SURVIVAL_NORMAL_STAGE_1 as i32
            && mode <= GameMode::GAMEMODE_SURVIVAL_ENDLESS_STAGE_5 as i32;
        if is_survival && board.mBackground == 1
            && board.mCurrentWave == board.mNumWaves - 1
        {
            // [TODO]: Spawn random grave
        }

        if (*app).IsBungeeBlitzLevel() && aIsFlagWave {
            board.DisplayAdvice("[ADVICE_BUNGEES_INCOMING]", 0, -1);
        }
    }
}

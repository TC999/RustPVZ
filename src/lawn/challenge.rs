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

        // Dispatch to mode-specific updates
        match app.mGameMode as i32 {
            x if x == GameMode::GAMEMODE_CHALLENGE_BEGHOULED as i32
                || x == GameMode::GAMEMODE_CHALLENGE_BEGHOULED_TWIST as i32 => {}
            _x if (*app).IsScaryPotterLevel() => {}
            _x if (*app).IsWhackAZombieLevel() => {}
            _x if (*app).IsIZombieLevel() => {}
            _x if (*app).IsSlotMachineLevel() => {}
            x if x == GameMode::GAMEMODE_CHALLENGE_SPEED as i32 => {
                board.UpdateGame();
            }
            x if x == GameMode::GAMEMODE_CHALLENGE_RAINING_SEEDS as i32 => {}
            x if x == GameMode::GAMEMODE_CHALLENGE_PORTAL_COMBAT as i32 => {}
            x if x == GameMode::GAMEMODE_CHALLENGE_ZOMBIQUARIUM as i32 => {}
            x if x == GameMode::GAMEMODE_TREE_OF_WISDOM as i32 => {}
            x if x == GameMode::GAMEMODE_CHALLENGE_LAST_STAND as i32 => {}
            _ => {}
        }

        // ReanimChallenge update
        // Reanimation* aReanim = app->ReanimationTryToGet(mReanimChallenge);
    }

    /// C++ Challenge::UpdateConveyorBelt() (from Challenge.cpp:1616)
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

    pub unsafe fn ZombieAtePlant(&mut self, _thePlant: *mut crate::lawn::plant::Plant) {
        // Challenge::ZombieAtePlant - track plants eaten for challenges
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

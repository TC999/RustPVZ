// [TRANSLATION_NOTE]: Board.h -> Rust struct
// Board 是游戏主面板，管理所有游戏对象

use crate::const_enums::*;
use crate::sexy_app_framework::misc::rect::Rect;
use crate::sexy_tod_lib::data_array::DataArray;
use super::plant::Plant;
use super::zombie::Zombie;
use super::projectile::Projectile;
use super::coin::Coin;
use super::lawn_mower::LawnMower;
use super::grid_item::GridItem;

pub const MAX_GRID_SIZE_X: i32 = 9;
pub const MAX_GRID_SIZE_Y: i32 = 6;
pub const MAX_ZOMBIES_IN_WAVE: i32 = 50;
pub const MAX_ZOMBIE_WAVES: i32 = 100;
pub const MAX_GRAVE_STONES: i32 = MAX_GRID_SIZE_X * MAX_GRID_SIZE_Y;
pub const MAX_POOL_GRID_SIZE: i32 = 10;
pub const MAX_RENDER_ITEMS: i32 = 2048;
pub const PROGRESS_METER_COUNTER: i32 = 150;

pub struct Board {
    pub m_app: *mut crate::lawn_app::LawnApp,
    pub m_zombies: DataArray<Zombie>,
    pub m_plants: DataArray<Plant>,
    pub m_projectiles: DataArray<Projectile>,
    pub m_coins: DataArray<Coin>,
    pub m_lawn_mowers: DataArray<LawnMower>,
    pub m_grid_items: DataArray<GridItem>,
    pub m_paused: bool,
    pub m_level: i32,
    pub m_main_counter: u32,
    pub m_current_wave: i32,
    pub m_sun_money: i32,
    pub m_zombie_count_down: i32,
    pub m_level_complete: bool,
}

impl Board {
    pub fn new() -> Self {
        Board {
            m_app: std::ptr::null_mut(),
            m_zombies: DataArray::new(),
            m_plants: DataArray::new(),
            m_projectiles: DataArray::new(),
            m_coins: DataArray::new(),
            m_lawn_mowers: DataArray::new(),
            m_grid_items: DataArray::new(),
            m_paused: false,
            m_level: 0,
            m_main_counter: 0,
            m_current_wave: 0,
            m_sun_money: 50,
            m_zombie_count_down: 0,
            m_level_complete: false,
        }
    }
}

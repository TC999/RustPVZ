// [TRANSLATION_NOTE]: Projectile.cpp -> Rust 翻译
// 投射物系统 — 豌豆、冰豆、西瓜、玉米、星星、刺等

use crate::const_enums::*;
use super::game_object::GameObject;

#[derive(Clone, Copy)]
pub struct ProjectileDefinition {
    pub m_projectile_type: ProjectileType,
    pub m_image_row: i32,
    pub m_damage: i32,
}

impl ProjectileDefinition {
    pub fn new() -> Self {
        ProjectileDefinition {
            m_projectile_type: ProjectileType::PROJECTILE_PEA,
            m_image_row: 0,
            m_damage: 0,
        }
    }
}

#[derive(Clone)]
pub struct Projectile {
    pub base: GameObject,
    pub m_frame: i32,
    pub m_num_frames: i32,
    pub m_anim_counter: i32,
    pub m_pos_x: f32,
    pub m_pos_y: f32,
    pub m_pos_z: f32,
    pub m_vel_x: f32,
    pub m_vel_y: f32,
    pub m_vel_z: f32,
    pub m_acc_z: f32,
    pub m_shadow_y: f32,
    pub m_dead: bool,
    pub m_anim_ticks_per_frame: i32,
    pub m_motion_type: i32,
    pub m_projectile_type: ProjectileType,
    pub m_projectile_age: i32,
    pub m_click_backoff_counter: i32,
    pub m_rotation: f32,
    pub m_rotation_speed: f32,
    pub m_on_high_ground: bool,
    pub m_damage_range_flags: i32,
    pub m_hit_torchwood_grid_x: i32,
    pub m_attachment_id: AttachmentID,
    pub m_cob_target_x: f32,
    pub m_cob_target_row: i32,
    pub m_target_zombie_id: ZombieID,
    pub m_last_portal_x: i32,
    // 投射物定义引用 (C++: GetProjectileDef 返回静态数组)
    pub m_def_damage: i32,
    pub m_def_image_row: i32,
}

// ProjectileMotion 常量 (C++ ConstEnums.h:790)
pub const MOTION_STRAIGHT: i32 = 0;      // 水平向右
pub const MOTION_LOBBED: i32 = 1;        // 抛物线
pub const MOTION_THREEPEATER: i32 = 2;   // 偏转向右
pub const MOTION_BEE: i32 = 3;
pub const MOTION_BEE_BACKWARDS: i32 = 4;
pub const MOTION_PUFF: i32 = 5;          // 水平向右（一段时间后消失）
pub const MOTION_BACKWARDS: i32 = 6;     // 水平向左
pub const MOTION_STAR: i32 = 7;          // 斜向运动
pub const MOTION_FLOAT_OVER: i32 = 8;    // 缓慢漂浮向右（无碰撞）
pub const MOTION_HOMING: i32 = 9;        // 追踪
// 投射物定义表 (C++ Projectile::GetProjectileDef)
pub const PROJECTILE_DEFS: [ProjectileDefinition; 23] = [
    ProjectileDefinition { m_projectile_type: ProjectileType::PROJECTILE_PEA,         m_image_row: 0, m_damage: 20 },
    ProjectileDefinition { m_projectile_type: ProjectileType::PROJECTILE_SNOWPEA,     m_image_row: 1, m_damage: 20 },
    ProjectileDefinition { m_projectile_type: ProjectileType::PROJECTILE_CABBAGE,     m_image_row: 2, m_damage: 40 },
    ProjectileDefinition { m_projectile_type: ProjectileType::PROJECTILE_MELON,       m_image_row: 3, m_damage: 60 },
    ProjectileDefinition { m_projectile_type: ProjectileType::PROJECTILE_WINTERMELON, m_image_row: 3, m_damage: 60 },
    ProjectileDefinition { m_projectile_type: ProjectileType::PROJECTILE_COB,         m_image_row: 5, m_damage: 40 },
    ProjectileDefinition { m_projectile_type: ProjectileType::PROJECTILE_BUTTER,      m_image_row: 0, m_damage: 20 },
    ProjectileDefinition { m_projectile_type: ProjectileType::PROJECTILE_KERNEL,      m_image_row: 4, m_damage: 20 },
    ProjectileDefinition { m_projectile_type: ProjectileType::PROJECTILE_SPIKE,       m_image_row: 7, m_damage: 20 },
    ProjectileDefinition { m_projectile_type: ProjectileType::PROJECTILE_SPIKEROCK,   m_image_row: 8, m_damage: 20 },
    ProjectileDefinition { m_projectile_type: ProjectileType::PROJECTILE_PUFF,        m_image_row: 6, m_damage: 1 },
    ProjectileDefinition { m_projectile_type: ProjectileType::PROJECTILE_FUME,        m_image_row: 0, m_damage: 1 },
    ProjectileDefinition { m_projectile_type: ProjectileType::PROJECTILE_CACTUS,      m_image_row: 0, m_damage: 20 },
    ProjectileDefinition { m_projectile_type: ProjectileType::PROJECTILE_BLOVER,      m_image_row: 0, m_damage: 0 },
    ProjectileDefinition { m_projectile_type: ProjectileType::PROJECTILE_STAR,        m_image_row: 9, m_damage: 20 },
    ProjectileDefinition { m_projectile_type: ProjectileType::PROJECTILE_GARLIC,      m_image_row: 0, m_damage: 0 },
    ProjectileDefinition { m_projectile_type: ProjectileType::PROJECTILE_FIREBALL,    m_image_row: 0, m_damage: 40 },
    ProjectileDefinition { m_projectile_type: ProjectileType::PROJECTILE_ICE,         m_image_row: 0, m_damage: 0 },
    ProjectileDefinition { m_projectile_type: ProjectileType::PROJECTILE_FIRE,        m_image_row: 0, m_damage: 0 },
    ProjectileDefinition { m_projectile_type: ProjectileType::PROJECTILE_BOBSLED,     m_image_row: 0, m_damage: 0 },
    ProjectileDefinition { m_projectile_type: ProjectileType::PROJECTILE_FIREBALL_BOSS, m_image_row: 0, m_damage: 80 },
    ProjectileDefinition { m_projectile_type: ProjectileType::PROJECTILE_ZOMBIE_PEA,  m_image_row: 0, m_damage: 20 },
    ProjectileDefinition { m_projectile_type: ProjectileType::PROJECTILE_COBBIG,      m_image_row: 0, m_damage: 120 },
];

unsafe fn g_app() -> &'static mut crate::lawn_app::LawnApp {
    &mut *crate::lawn_app::G_LAWN_APP
}

impl Projectile {
    pub fn new() -> Self {
        Projectile {
            base: GameObject::new(),
            m_frame: 0, m_num_frames: 1, m_anim_counter: 0,
            m_pos_x: 0.0, m_pos_y: 0.0, m_pos_z: 0.0,
            m_vel_x: 0.0, m_vel_y: 0.0, m_vel_z: 0.0, m_acc_z: 0.0,
            m_shadow_y: 0.0, m_dead: false,
            m_anim_ticks_per_frame: 0, m_motion_type: 0,
            m_projectile_type: ProjectileType::PROJECTILE_PEA,
            m_projectile_age: 0, m_click_backoff_counter: 0,
            m_rotation: 0.0, m_rotation_speed: 0.0,
            m_on_high_ground: false, m_damage_range_flags: 0,
            m_hit_torchwood_grid_x: 0,
            m_attachment_id: AttachmentID::ATTACHMENTID_NULL,
            m_cob_target_x: 0.0, m_cob_target_row: 0,
            m_target_zombie_id: ZombieID::ZOMBIEID_NULL, m_last_portal_x: 0,
            m_def_damage: 20, m_def_image_row: 0,
        }
    }

    // =========================================================================
    // GetProjectileDef — C++ 保真翻译 (Projectile.cpp:33)
    // =========================================================================
    pub fn GetProjectileDef(the_type: ProjectileType) -> &'static ProjectileDefinition {
        let idx = the_type as usize;
        if idx < PROJECTILE_DEFS.len() {
            &PROJECTILE_DEFS[idx]
        } else {
            &PROJECTILE_DEFS[0]
        }
    }

    // =========================================================================
    // ProjectileInitialize — C++ 保真翻译 (Projectile.cpp:62)
    // =========================================================================
    pub unsafe fn ProjectileInitialize(&mut self, the_x: i32, the_y: i32, the_render_order: i32, the_row: i32, the_projectile_type: ProjectileType) {
        let def = Self::GetProjectileDef(the_projectile_type);
        self.m_pos_x = the_x as f32;
        self.m_pos_y = the_y as f32;
        self.base.m_render_order = the_render_order;
        self.base.m_row = the_row;
        self.m_projectile_type = the_projectile_type;
        self.m_dead = false;
        self.m_projectile_age = 0;
        self.m_def_damage = def.m_damage;
        self.m_def_image_row = def.m_image_row;

        match the_projectile_type {
            ProjectileType::PROJECTILE_PEA | ProjectileType::PROJECTILE_SNOWPEA
            | ProjectileType::PROJECTILE_FIREBALL
            | ProjectileType::PROJECTILE_SPIKE | ProjectileType::PROJECTILE_SPIKEROCK => {
                self.m_motion_type = 0; // MOTION_NORMAL
                self.m_vel_x = 4.0;
                self.m_anim_ticks_per_frame = 3;
                self.m_num_frames = 4;
            }
            ProjectileType::PROJECTILE_CABBAGE | ProjectileType::PROJECTILE_MELON
            | ProjectileType::PROJECTILE_WINTERMELON | ProjectileType::PROJECTILE_KERNEL
            | ProjectileType::PROJECTILE_BUTTER | ProjectileType::PROJECTILE_COB => {
                self.m_motion_type = 1; // MOTION_LOBBED
                self.m_vel_x = 3.0;
                self.m_vel_z = -5.0;
                self.m_acc_z = 0.18;
                self.m_anim_ticks_per_frame = 3;
                self.m_num_frames = 4;
            }
            ProjectileType::PROJECTILE_PUFF => {
                self.m_motion_type = 0;
                self.m_vel_x = 3.5;
                self.m_num_frames = 3;
            }
            ProjectileType::PROJECTILE_STAR => {
                self.m_motion_type = 0;
                self.m_vel_x = 4.0;
                self.m_rotation_speed = 0.3;
                self.m_anim_ticks_per_frame = 2;
                self.m_num_frames = 4;
            }
            _ => {
                self.m_vel_x = 3.0;
                self.m_num_frames = 1;
            }
        }
    }

    // =========================================================================
    // Update — C++ 保真翻译 (Projectile.cpp:936)
    // =========================================================================
    pub unsafe fn Update(&mut self) {
        if self.m_dead { return; }

        let app = g_app();
        if (*app).mGameScene != GameScenes::SCENE_PLAYING {
            return;
        }

        let mut a_time = 20;
        match self.m_projectile_type {
            ProjectileType::PROJECTILE_PEA | ProjectileType::PROJECTILE_SNOWPEA
            | ProjectileType::PROJECTILE_CABBAGE | ProjectileType::PROJECTILE_MELON
            | ProjectileType::PROJECTILE_WINTERMELON | ProjectileType::PROJECTILE_KERNEL
            | ProjectileType::PROJECTILE_BUTTER | ProjectileType::PROJECTILE_COB
            | ProjectileType::PROJECTILE_SPIKE => {
                a_time = 0;
            }
            _ => {}
        }
        if self.m_projectile_age > a_time {
            self.base.m_render_order = crate::lawn::board::Board::MakeRenderOrder(
                RenderLayer::RENDER_LAYER_PROJECTILE, self.base.m_row, 0
            );
        }

        if self.m_click_backoff_counter > 0 {
            self.m_click_backoff_counter -= 1;
        }
        self.m_rotation += self.m_rotation_speed;

        self.m_projectile_age += 1;
        self.UpdateMotion();
        // [TODO]: AttachmentUpdateAndMove(mAttachmentID, mPosX, mPosY + mPosZ)
    }

    // =========================================================================
    // UpdateMotion — C++ 保真翻译
    // =========================================================================
    unsafe fn UpdateMotion(&mut self) {
        // 动画帧更新
        if self.m_anim_ticks_per_frame > 0 && self.m_num_frames > 0 {
            self.m_anim_counter = (self.m_anim_counter + 1) % (self.m_num_frames * self.m_anim_ticks_per_frame);
            self.m_frame = self.m_anim_counter / self.m_anim_ticks_per_frame;
        }

        // 运动更新
        if self.m_motion_type == 1 {
            // MOTION_LOBBED — 抛物线
            self.m_pos_x += self.m_vel_x;
            self.m_pos_z += self.m_vel_z;
            self.m_vel_z += self.m_acc_z;
        } else {
            self.m_pos_x += self.m_vel_x;
            self.m_pos_y += self.m_vel_y;
            self.m_pos_z += self.m_vel_z;
            self.m_vel_z += self.m_acc_z;
        }

        self.base.m_x = self.m_pos_x as i32;
        self.base.m_y = (self.m_pos_y + self.m_pos_z) as i32;

        // [TODO]: 碰撞检测 — 遍历僵尸检查矩形重叠
        // [TODO]: if hit → DoImpact(theZombie)

        // 超出屏幕判定
        if self.m_pos_x > 950.0 || self.m_pos_x < -150.0 || self.m_pos_y > 750.0 || self.m_pos_z > 500.0 {
            self.m_dead = true;
        }
    }

    // =========================================================================
    // Draw — C++ 保真翻译 (Projectile.cpp:971)
    // =========================================================================
    pub unsafe fn Draw(&self, g: &mut crate::sexy_app_framework::graphics::graphics::Graphics) {
        if self.m_dead { return; }

        let _def = Self::GetProjectileDef(self.m_projectile_type);

        // C++: 根据投射物类型选择图片、缩放和帧
        // [TODO]: 使用 IMAGE_PROJECTILEPEA / IMAGE_PROJECTILESNOWPEA 等图片资源
        // 下面是 C++ 绘制的结构：
        //
        // Image* aImage = nullptr;
        // float aScale = 1.0f;
        // switch (mProjectileType) {
        //     case PROJECTILE_PEA:         aImage = IMAGE_PROJECTILEPEA; break;
        //     case PROJECTILE_SNOWPEA:     aImage = IMAGE_PROJECTILESNOWPEA; break;
        //     case PROJECTILE_CABBAGE:     aImage = IMAGE_REANIM_CABBAGEPULT_CABBAGE; break;
        //     case PROJECTILE_MELON:       aImage = IMAGE_REANIM_MELONPULT_MELON; break;
        //     case PROJECTILE_WINTERMELON: aImage = IMAGE_REANIM_WINTERMELON_PROJECTILE; break;
        //     case PROJECTILE_KERNEL:      aImage = IMAGE_REANIM_CORNPULT_KERNAL; break;
        //     case PROJECTILE_BUTTER:      aImage = IMAGE_REANIM_CORNPULT_BUTTER; aScale = 0.8f; break;
        //     case PROJECTILE_SPIKE:       aImage = IMAGE_PROJECTILECACTUS; break;
        //     case PROJECTILE_STAR:        aImage = IMAGE_PROJECTILE_STAR; break;
        //     case PROJECTILE_PUFF:        aImage = IMAGE_PUFFSHROOM_PUFF1; break;
        //     case PROJECTILE_BASKETBALL:  aImage = IMAGE_REANIM_ZOMBIE_CATAPULT_BASKETBALL; break;
        //     case PROJECTILE_COBBIG:      aImage = IMAGE_REANIM_COBCANNON_COB; aScale = 0.9f; break;
        // }
        //
        // 根据 aImage 和帧绘制投射物精灵
        // bool aMirror = (mMotionType == MOTION_BEE_BACKWARDS);
        // if (aImage) { 绘制图像帧  }
    }

    // =========================================================================
    // DrawShadow — C++ 保真翻译 (Projectile.cpp:1069)
    // =========================================================================
    pub unsafe fn DrawShadow(&self, _g: &mut crate::sexy_app_framework::graphics::graphics::Graphics) {
        // C++: 如果是抛物线运动 (MOTION_LOBBED)，在 (mPosX, mShadowY) 绘制阴影椭圆
        // [TODO]: 绘制阴影
    }

    // =========================================================================
    // DoImpact — C++ 保真翻译 (Projectile.cpp:819)
    // =========================================================================
    /// C++ Projectile::GetProjectileRect (Projectile.cpp:1163)
    pub fn GetProjectileRect(&self) -> crate::sexy_app_framework::misc::rect::Rect {
        if self.m_projectile_type == ProjectileType::PROJECTILE_PEA
            || self.m_projectile_type == ProjectileType::PROJECTILE_SNOWPEA
            || self.m_projectile_type == ProjectileType::PROJECTILE_ZOMBIE_PEA
        {
            return crate::sexy_app_framework::misc::rect::Rect::new(
                self.base.m_x - 15, self.base.m_y, self.base.m_width + 15, self.base.m_height,
            );
        } else if self.m_projectile_type == ProjectileType::PROJECTILE_COBBIG {
            return crate::sexy_app_framework::misc::rect::Rect::new(
                self.base.m_x + self.base.m_width / 2 - 115,
                self.base.m_y + self.base.m_height / 2 - 115,
                230, 230,
            );
        } else if self.m_projectile_type == ProjectileType::PROJECTILE_MELON
            || self.m_projectile_type == ProjectileType::PROJECTILE_WINTERMELON
        {
            return crate::sexy_app_framework::misc::rect::Rect::new(
                self.base.m_x + 20, self.base.m_y, 60, self.base.m_height,
            );
        } else if self.m_projectile_type == ProjectileType::PROJECTILE_FIREBALL {
            return crate::sexy_app_framework::misc::rect::Rect::new(
                self.base.m_x, self.base.m_y, self.base.m_width - 10, self.base.m_height,
            );
        } else if self.m_projectile_type == ProjectileType::PROJECTILE_SPIKE {
            return crate::sexy_app_framework::misc::rect::Rect::new(
                self.base.m_x - 25, self.base.m_y, self.base.m_width + 25, self.base.m_height,
            );
        } else {
            return crate::sexy_app_framework::misc::rect::Rect::new(
                self.base.m_x, self.base.m_y, self.base.m_width, self.base.m_height,
            );
        }
    }

    /// C++ Projectile::FindCollisionTargetPlant (Projectile.cpp:156) — 僵尸豌豆命中植物
    pub unsafe fn FindCollisionTargetPlant(&self) -> *mut super::plant::Plant {
        let a_projectile_rect = self.GetProjectileRect();
        let the_board = self.board();
        let mut a_plant: *mut super::plant::Plant = std::ptr::null_mut();
        while the_board.IteratePlants(&mut a_plant) {
            if (*a_plant).base.m_row != self.base.m_row {
                continue;
            }

            if self.m_projectile_type == ProjectileType::PROJECTILE_ZOMBIE_PEA {
                // C++: 僵尸豌豆不能击中低矮植物
                if (*a_plant).m_seed_type == SeedType::SEED_PUFFSHROOM
                    || (*a_plant).m_seed_type == SeedType::SEED_SUNSHROOM
                    || (*a_plant).m_seed_type == SeedType::SEED_POTATOMINE
                    || (*a_plant).m_seed_type == SeedType::SEED_SPIKEWEED
                    || (*a_plant).m_seed_type == SeedType::SEED_SPIKEROCK
                    || (*a_plant).m_seed_type == SeedType::SEED_LILYPAD
                {
                    continue;
                }
            }

            let a_plant_rect = (*a_plant).GetPlantRect();
            if crate::lawn::board::Board::get_rect_overlap(a_projectile_rect, a_plant_rect) > 8 {
                if self.m_projectile_type == ProjectileType::PROJECTILE_ZOMBIE_PEA {
                    return the_board.GetTopPlantAt((*a_plant).m_plant_col, (*a_plant).base.m_row, PlantPriority::TOPPLANT_EATING_ORDER);
                } else {
                    return the_board.GetTopPlantAt((*a_plant).m_plant_col, (*a_plant).base.m_row, PlantPriority::TOPPLANT_CATAPULT_ORDER);
                }
            }
        }

        std::ptr::null_mut()
    }

    /// C++ Projectile::PeaAboutToHitTorchwood (Projectile.cpp:194) — “卡火炬”检测
    pub unsafe fn PeaAboutToHitTorchwood(&self) -> bool {
        if self.m_motion_type != MOTION_STRAIGHT {
            return false;
        }

        if self.m_projectile_type != ProjectileType::PROJECTILE_PEA
            && self.m_projectile_type != ProjectileType::PROJECTILE_SNOWPEA
        {
            return false;
        }

        let the_board = self.board();
        let mut a_plant: *mut super::plant::Plant = std::ptr::null_mut();
        while the_board.IteratePlants(&mut a_plant) {
            if (*a_plant).m_seed_type == SeedType::SEED_TORCHWOOD
                && (*a_plant).base.m_row == self.base.m_row
                && !(*a_plant).NotOnGround()
                && self.m_hit_torchwood_grid_x != (*a_plant).m_plant_col
            {
                let a_plant_attack_rect = (*a_plant).GetPlantAttackRect(super::plant::PlantWeapon::WEAPON_PRIMARY);
                let mut a_projectile_rect = self.GetProjectileRect();
                a_projectile_rect.m_x += 40;

                if crate::lawn::board::Board::get_rect_overlap(a_plant_attack_rect, a_projectile_rect) > 10 {
                    return true;
                }
            }
        }

        false
    }

    /// C++ Projectile::FindCollisionTarget (Projectile.cpp:221) — 寻找碰撞僵尸
    pub unsafe fn FindCollisionTarget(&self) -> *mut super::zombie::Zombie {
        if self.PeaAboutToHitTorchwood() {
            return std::ptr::null_mut();
        }

        let a_projectile_rect = self.GetProjectileRect();
        let mut a_best_zombie: *mut super::zombie::Zombie = std::ptr::null_mut();
        let mut a_min_x = 0;

        let the_board = self.board();
        let mut a_zombie: *mut super::zombie::Zombie = std::ptr::null_mut();
        while the_board.IterateZombies(&mut a_zombie) {
            if ((*a_zombie).m_zombie_type == ZombieType::ZOMBIE_BOSS || (*a_zombie).base.m_row == self.base.m_row)
                && (*a_zombie).EffectedByDamage(self.m_damage_range_flags as u32)
            {
                // C++: 潜泳中且豌豆飞得够高时跳过
                if (*a_zombie).m_zombie_phase == ZombiePhase::PHASE_SNORKEL_WALKING_IN_POOL
                    && self.m_pos_z >= 45.0
                {
                    continue;
                }

                // C++: 星星刚射出时不打挖地僵尸
                if self.m_projectile_type == ProjectileType::PROJECTILE_STAR
                    && self.m_projectile_age < 25
                    && self.m_vel_x >= 0.0
                    && (*a_zombie).m_zombie_type == ZombieType::ZOMBIE_DIGGER
                {
                    continue;
                }

                let a_zombie_rect = (*a_zombie).GetZombieRect();
                if crate::lawn::board::Board::get_rect_overlap(a_projectile_rect, a_zombie_rect) > 0 {
                    if a_best_zombie.is_null() || (*a_zombie).base.m_x < a_min_x {
                        a_best_zombie = a_zombie;
                        a_min_x = (*a_zombie).base.m_x;
                    }
                }
            }
        }

        a_best_zombie
    }

    /// C++ Projectile::CheckForCollision (Projectile.cpp:260) — 碰撞检查
    pub unsafe fn CheckForCollision(&mut self) {
        if self.m_motion_type == MOTION_PUFF && self.m_projectile_age >= 75 {
            self.Die();
            return;
        }

        if self.m_pos_x > 800.0 /* WIDE_BOARD_WIDTH */ || (self.m_pos_x + self.base.m_width as f32) < 0.0 {
            self.Die();
            return;
        }

        if self.m_motion_type == MOTION_HOMING {
            // C++: Zombie* aZombie = mBoard->ZombieTryToGet(mTargetZombieID);
            // [TODO]: ZombieTryToGet(mTargetZombieID) 目标获取
            let the_board = self.board();
            let a_zombie: *mut super::zombie::Zombie = std::ptr::null_mut();
            if !a_zombie.is_null() && (*a_zombie).EffectedByDamage(self.m_damage_range_flags as u32) {
                let a_projectile_rect = self.GetProjectileRect();
                let a_zombie_rect = (*a_zombie).GetZombieRect();
                if crate::lawn::board::Board::get_rect_overlap(a_projectile_rect, a_zombie_rect) >= 0
                    && self.m_pos_y > a_zombie_rect.m_y as f32
                    && self.m_pos_y < (a_zombie_rect.m_y + a_zombie_rect.m_height) as f32
                {
                    self.DoImpact(a_zombie);
                }
            }
            return;
        }

        if self.m_projectile_type == ProjectileType::PROJECTILE_STAR
            && (self.m_pos_y > 600.0 || self.m_pos_y < 0.0)
        {
            self.Die();
            return;
        }

        if (self.m_projectile_type == ProjectileType::PROJECTILE_PEA
            || self.m_projectile_type == ProjectileType::PROJECTILE_STAR)
            && self.m_shadow_y - self.m_pos_y > 90.0
        {
            return;
        }

        if self.m_motion_type == MOTION_FLOAT_OVER {
            return;
        }

        if self.m_projectile_type == ProjectileType::PROJECTILE_ZOMBIE_PEA {
            let a_plant = self.FindCollisionTargetPlant();
            if !a_plant.is_null() {
                let a_projectile_def = Self::GetProjectileDef(self.m_projectile_type);
                (*a_plant).m_plant_health -= a_projectile_def.m_damage;
                (*a_plant).m_eaten_flash_countdown = (*a_plant).m_eaten_flash_countdown.max(25);
                // [TODO]: mApp->PlayFoley(FOLEY_SPLAT); AddTodParticle(PARTICLE_PEA_SPLAT)
                self.Die();
            }
            return;
        }

        let a_zombie = self.FindCollisionTarget();
        if !a_zombie.is_null() {
            if (*a_zombie).m_on_high_ground && self.CantHitHighGround() {
                return;
            }
            self.DoImpact(a_zombie);
        }
    }

    /// C++ Projectile::CantHitHighGround (Projectile.cpp:333)
    pub fn CantHitHighGround(&self) -> bool {
        if self.m_motion_type == MOTION_BACKWARDS || self.m_motion_type == MOTION_HOMING {
            return false;
        }

        (self.m_projectile_type == ProjectileType::PROJECTILE_PEA
            || self.m_projectile_type == ProjectileType::PROJECTILE_SNOWPEA
            || self.m_projectile_type == ProjectileType::PROJECTILE_STAR
            || self.m_projectile_type == ProjectileType::PROJECTILE_PUFF
            || self.m_projectile_type == ProjectileType::PROJECTILE_FIREBALL)
            && !self.m_on_high_ground
    }

    /// C++ Projectile::CheckForHighGround (Projectile.cpp:347)
    pub unsafe fn CheckForHighGround(&mut self) {
        let a_shadow_delta = self.m_shadow_y - self.m_pos_y;

        if self.m_projectile_type == ProjectileType::PROJECTILE_PEA
            || self.m_projectile_type == ProjectileType::PROJECTILE_SNOWPEA
            || self.m_projectile_type == ProjectileType::PROJECTILE_FIREBALL
            || self.m_projectile_type == ProjectileType::PROJECTILE_SPIKE
            || self.m_projectile_type == ProjectileType::PROJECTILE_COBBIG
        {
            if a_shadow_delta < 28.0 {
                self.DoImpact(std::ptr::null_mut());
                return;
            }
        }

        if self.m_projectile_type == ProjectileType::PROJECTILE_PUFF && a_shadow_delta < 0.0 {
            self.DoImpact(std::ptr::null_mut());
            return;
        }

        if self.m_projectile_type == ProjectileType::PROJECTILE_STAR && a_shadow_delta < 23.0 {
            self.DoImpact(std::ptr::null_mut());
            return;
        }

        if self.CantHitHighGround() {
            let the_board = self.board();
            let a_grid_x = the_board.PixelToGridXKeepOnBoard(self.m_pos_x as i32 + 30, self.m_pos_y as i32);
            if the_board.mGridSquareType[a_grid_x as usize][self.base.m_row as usize]
                == GridSquareType::GRIDSQUARE_HIGH_GROUND
            {
                self.DoImpact(std::ptr::null_mut());
            }
        }
    }

    /// C++ Projectile::IsSplashDamage (Projectile.cpp:386)
    pub fn IsSplashDamage(&self, the_zombie: Option<&super::zombie::Zombie>) -> bool {
        if self.m_projectile_type == ProjectileType::PROJECTILE_FIREBALL && the_zombie.is_some() {
            if let Some(z) = the_zombie {
                if unsafe { z.IsFireResistant() } {
                    return false;
                }
            }
        }

        self.m_projectile_type == ProjectileType::PROJECTILE_MELON
            || self.m_projectile_type == ProjectileType::PROJECTILE_WINTERMELON
            || self.m_projectile_type == ProjectileType::PROJECTILE_FIREBALL
    }

    /// C++ Projectile::GetDamageFlags (Projectile.cpp:397)
    pub fn GetDamageFlags(&self, the_zombie: Option<&super::zombie::Zombie>) -> u32 {
        let mut a_damage_flags: u32 = 0;

        if self.IsSplashDamage(the_zombie) {
            a_damage_flags |= 1 << DamageFlags::DAMAGE_HITS_SHIELD_AND_BODY as i32;
        } else if self.m_motion_type == MOTION_LOBBED || self.m_motion_type == MOTION_BACKWARDS {
            a_damage_flags |= 1 << DamageFlags::DAMAGE_BYPASSES_SHIELD as i32;
        } else if self.m_motion_type == MOTION_STAR && self.m_vel_x < 0.0 {
            a_damage_flags |= 1 << DamageFlags::DAMAGE_BYPASSES_SHIELD as i32;
        }

        // C++: if (mProjectileType == PROJECTILE_SNOWPEA || mProjectileType == PROJECTILE_WINTERMELON)
        // C++:     SetBit(aDamageFlags, DAMAGE_FREEZE, true);
        if self.m_projectile_type == ProjectileType::PROJECTILE_SNOWPEA
            || self.m_projectile_type == ProjectileType::PROJECTILE_WINTERMELON
        {
            a_damage_flags |= 1 << DamageFlags::DAMAGE_FREEZE as i32;
        }

        a_damage_flags
    }

    /// C++ Projectile::IsZombieHitBySplash (Projectile.cpp:422)
    pub unsafe fn IsZombieHitBySplash(&self, the_zombie: &super::zombie::Zombie) -> bool {
        let mut a_projectile_rect = self.GetProjectileRect();
        if self.m_projectile_type == ProjectileType::PROJECTILE_FIREBALL {
            a_projectile_rect.m_width = 100;
        }

        let mut a_row_deviation = the_zombie.base.m_row - self.base.m_row;
        let a_zombie_rect = the_zombie.GetZombieRect();
        if the_zombie.IsFireResistant() && self.m_projectile_type == ProjectileType::PROJECTILE_FIREBALL {
            return false;
        }

        if the_zombie.m_zombie_type == ZombieType::ZOMBIE_BOSS {
            a_row_deviation = 0;
        }
        if self.m_projectile_type == ProjectileType::PROJECTILE_FIREBALL {
            if a_row_deviation != 0 {
                return false;
            }
        } else if a_row_deviation > 1 || a_row_deviation < -1 {
            return false;
        }

        let the_board = self.board();
        the_zombie.EffectedByDamage(self.m_damage_range_flags as u32)
            && crate::lawn::board::Board::get_rect_overlap(a_projectile_rect, a_zombie_rect) >= 0
    }

    /// C++ Projectile::DoSplashDamage (Projectile.cpp:456) — 溅射伤害
    pub unsafe fn DoSplashDamage(&mut self, the_zombie: *mut super::zombie::Zombie) {
        let a_projectile_def = Self::GetProjectileDef(self.m_projectile_type);

        let the_board = self.board();
        let mut a_zombies_get_splashed = 0;
        let mut a_zombie: *mut super::zombie::Zombie = std::ptr::null_mut();
        while the_board.IterateZombies(&mut a_zombie) {
            if a_zombie != the_zombie && self.IsZombieHitBySplash(&*a_zombie) {
                a_zombies_get_splashed += 1;
            }
        }

        let a_original_damage = a_projectile_def.m_damage;
        let mut a_splash_damage = a_projectile_def.m_damage / 3;
        let mut a_max_splash_damage_amount = a_original_damage * 7;
        if self.m_projectile_type == ProjectileType::PROJECTILE_FIREBALL {
            a_max_splash_damage_amount = a_original_damage;
        }
        let a_splash_damage_amount = a_splash_damage * a_zombies_get_splashed;
        if a_splash_damage_amount > a_max_splash_damage_amount {
            // C++: aSplashDamage = aOriginalDamage * aMaxSplashDamageAmount / (aSplashDamageAmount * 3);
            a_splash_damage = a_original_damage * a_max_splash_damage_amount / (a_splash_damage_amount * 3);
            a_splash_damage = a_splash_damage.max(1);
        }

        a_zombie = std::ptr::null_mut();
        while the_board.IterateZombies(&mut a_zombie) {
            if self.IsZombieHitBySplash(&*a_zombie) {
                let a_damage_flags = self.GetDamageFlags(Some(&*a_zombie));
                if a_zombie == the_zombie {
                    (*a_zombie).TakeDamage(a_original_damage, a_damage_flags);
                } else {
                    (*a_zombie).TakeDamage(a_splash_damage, a_damage_flags);
                }
            }
        }
    }

    /// C++ Projectile::PlayImpactSound (Projectile.cpp:773)
    pub unsafe fn PlayImpactSound(&mut self, the_zombie: *mut super::zombie::Zombie) {
        let mut a_play_helm_sound = true;
        let mut a_play_splat_sound = true;
        let app = self.app();
        if self.m_projectile_type == ProjectileType::PROJECTILE_KERNEL {
            // [TODO]: mApp->PlayFoley(FOLEY_KERNEL_SPLAT)
            a_play_helm_sound = false;
            a_play_splat_sound = false;
        } else if self.m_projectile_type == ProjectileType::PROJECTILE_BUTTER {
            // [TODO]: mApp->PlayFoley(FOLEY_BUTTER)
            a_play_splat_sound = false;
        } else if self.m_projectile_type == ProjectileType::PROJECTILE_FIREBALL
            && self.IsSplashDamage(if the_zombie.is_null() { None } else { Some(&*the_zombie) })
        {
            // [TODO]: mApp->PlayFoley(FOLEY_IGNITE)
            a_play_helm_sound = false;
            a_play_splat_sound = false;
        } else if self.m_projectile_type == ProjectileType::PROJECTILE_MELON
            || self.m_projectile_type == ProjectileType::PROJECTILE_WINTERMELON
        {
            // [TODO]: mApp->PlayFoley(FOLEY_MELONIMPACT)
            a_play_splat_sound = false;
        }

        if a_play_helm_sound && !the_zombie.is_null() {
            if (*the_zombie).m_helm_type == 2 /* HELMTYPE_PAIL */ {
                // [TODO]: mApp->PlayFoley(FOLEY_SHIELD_HIT)
                a_play_splat_sound = false;
            } else if (*the_zombie).m_helm_type == 1 /* HELMTYPE_TRAFFIC_CONE */
                || (*the_zombie).m_helm_type == 3 /* HELMTYPE_DIGGER */
                || (*the_zombie).m_helm_type == 4 /* HELMTYPE_FOOTBALL */
            {
                // [TODO]: mApp->PlayFoley(FOLEY_PLASTIC_HIT)
            }
        }

        if a_play_splat_sound {
            // [TODO]: mApp->PlayFoley(FOLEY_SPLAT)
        }
        let _ = app;
    }

    /// C++ Projectile::ConvertToFireball (Projectile.cpp:1193) — 豌豆穿过火炬木变火球
    pub unsafe fn ConvertToFireball(&mut self, the_grid_x: i32) {
        if self.m_hit_torchwood_grid_x == the_grid_x {
            return;
        }

        self.m_projectile_type = ProjectileType::PROJECTILE_FIREBALL;
        self.m_hit_torchwood_grid_x = the_grid_x;
        // [TODO]: mApp->PlayFoley(FOLEY_FIREPEA)
        // [TODO]: AddReanimation(REANIM_FIRE_PEA) + AttachReanim
    }

    /// C++ Projectile::ConvertToPea (Projectile.cpp:1217) — 火球变回豌豆
    pub unsafe fn ConvertToPea(&mut self, the_grid_x: i32) {
        if self.m_hit_torchwood_grid_x == the_grid_x {
            return;
        }

        // C++: AttachmentDie(mAttachmentID);
        self.m_projectile_type = ProjectileType::PROJECTILE_PEA;
        self.m_hit_torchwood_grid_x = the_grid_x;
        // [TODO]: mApp->PlayFoley(FOLEY_THROW)
    }
    pub unsafe fn DoImpact(&mut self, the_zombie: *mut super::zombie::Zombie) {
        // C++ Projectile.cpp:819 — PlayImpactSound → 伤害结算 → 粒子 → Die
        self.PlayImpactSound(the_zombie);

        if self.IsSplashDamage(if the_zombie.is_null() { None } else { Some(&*the_zombie) }) {
            // C++: if (mProjectileType == PROJECTILE_FIREBALL && theZombie) theZombie->RemoveColdEffects();
            if self.m_projectile_type == ProjectileType::PROJECTILE_FIREBALL && !the_zombie.is_null() {
                (*the_zombie).RemoveColdEffects();
            }
            self.DoSplashDamage(the_zombie);
        } else if !the_zombie.is_null() {
            // C++: unsigned int aDamageFlags = GetDamageFlags(theZombie);
            // C++: theZombie->TakeDamage(GetProjectileDef().mDamage, aDamageFlags);
            let a_damage_flags = self.GetDamageFlags(Some(&*the_zombie));
            let a_damage = Self::GetProjectileDef(self.m_projectile_type).m_damage;
            (*the_zombie).TakeDamage(a_damage, a_damage_flags);
        }

        // C++: float aLastPosX = mPosX - mVelX; float aLastPosY = mPosY + mPosZ - mVelY - mVelZ;
        let a_last_pos_x = self.m_pos_x - self.m_vel_x;
        let a_last_pos_y = self.m_pos_y + self.m_pos_z - self.m_vel_y - self.m_vel_z;
        let mut a_effect = ParticleEffect::PARTICLE_NONE;
        let mut a_splat_pos_x = self.m_pos_x + 12.0;
        let mut a_splat_pos_y = self.m_pos_y + 12.0;
        match self.m_projectile_type {
            ProjectileType::PROJECTILE_MELON => {
                // [TODO]: mApp->AddTodParticle(aLastPosX + 30, aLastPosY + 30, mRenderOrder + 1, PARTICLE_MELONSPLASH)
            }
            ProjectileType::PROJECTILE_WINTERMELON => {
                // [TODO]: mApp->AddTodParticle(aLastPosX + 30, aLastPosY + 30, mRenderOrder + 1, PARTICLE_WINTERMELON)
            }
            ProjectileType::PROJECTILE_COBBIG => {
                // C++: PARTICLE_BLASTMARK + PARTICLE_POPCORNSPLASH + PlaySample(SOUND_DOOMSHROOM) + mBoard->ShakeBoard(3, -4)
                // [TODO]: 粒子/音效/震屏
            }
            ProjectileType::PROJECTILE_PEA => {
                a_splat_pos_x -= 15.0;
                a_effect = ParticleEffect::PARTICLE_PEA_SPLAT;
            }
            ProjectileType::PROJECTILE_SNOWPEA => {
                a_splat_pos_x -= 15.0;
                a_effect = ParticleEffect::PARTICLE_SNOWPEA_SPLAT;
            }
            ProjectileType::PROJECTILE_FIREBALL => {
                // C++: 溅射时 AddReanimation(REANIM_JALAPENO_FIRE) + OverrideScale(0.7, 0.4)
                // [TODO]: Reanimation 火焰
            }
            ProjectileType::PROJECTILE_STAR => {
                a_effect = ParticleEffect::PARTICLE_STAR_SPLAT;
            }
            ProjectileType::PROJECTILE_PUFF => {
                a_splat_pos_x -= 20.0;
                a_effect = ParticleEffect::PARTICLE_PUFF_SPLAT;
            }
            ProjectileType::PROJECTILE_CABBAGE => {
                a_splat_pos_x = a_last_pos_x - 38.0;
                a_splat_pos_y = a_last_pos_y + 23.0;
                a_effect = ParticleEffect::PARTICLE_CABBAGE_SPLAT;
            }
            ProjectileType::PROJECTILE_BUTTER => {
                a_splat_pos_x = a_last_pos_x - 20.0;
                a_splat_pos_y = a_last_pos_y + 63.0;
                a_effect = ParticleEffect::PARTICLE_BUTTER_SPLAT;

                // C++: if (theZombie) theZombie->ApplyButter();
                if !the_zombie.is_null() {
                    (*the_zombie).ApplyButter();
                }
            }
            _ => {}
        }

        // C++: if (aEffect != PARTICLE_NONE)
        if a_effect != ParticleEffect::PARTICLE_NONE {
            if !the_zombie.is_null() {
                // C++: 计算溅射粒子相对僵尸的坐标
                let mut a_pos_x = a_splat_pos_x + 52.0 - (*the_zombie).base.m_x as f32;
                let mut a_pos_y = a_splat_pos_y - (*the_zombie).base.m_y as f32;
                if (*the_zombie).m_zombie_phase == ZombiePhase::PHASE_SNORKEL_WALKING_IN_POOL
                    || (*the_zombie).m_zombie_phase == ZombiePhase::PHASE_DOLPHIN_WALKING_IN_POOL
                {
                    a_pos_y += 60.0;
                }
                if self.m_motion_type == MOTION_BACKWARDS {
                    a_pos_x -= 80.0;
                } else if self.m_pos_x > (*the_zombie).base.m_x as f32 + 40.0
                    && self.m_motion_type != MOTION_LOBBED
                {
                    a_pos_x -= 60.0;
                }

                // C++: aPosY = ClampFloat(aPosY, 20.0f, 100.0f);
                a_pos_y = crate::sexy_tod_lib::tod_common::clamp_float(a_pos_y, 20.0, 100.0);
                (*the_zombie).AddAttachedParticle(a_pos_x as i32, a_pos_y as i32, a_effect);
            } else {
                // [TODO]: mApp->AddTodParticle(aSplatPosX, aSplatPosY, mRenderOrder + 1, aEffect)
            }
        }

        // C++: Die();
        self.Die();
    }

    // =========================================================================
    // Die — C++ 保真翻译 (Projectile.cpp:1148)
    // =========================================================================
    pub unsafe fn Die(&mut self) {
        self.m_dead = true;
        // [TODO]: Remove attachment if any
    }

    unsafe fn board(&self) -> &'static mut super::board::Board {
        &mut *(self.base.m_board as *mut super::board::Board)
    }

    unsafe fn app(&self) -> &'static mut crate::lawn_app::LawnApp {
        &mut *(self.base.m_app as *mut crate::lawn_app::LawnApp)
    }
}

impl Default for Projectile {
    fn default() -> Self {
        Self::new()
    }
}

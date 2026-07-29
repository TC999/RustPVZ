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

// 投射物定义表 (C++ Projectile::GetProjectileDef)
pub const PROJECTILE_DEFS: [ProjectileDefinition; 18] = [
    ProjectileDefinition { m_projectile_type: ProjectileType::PROJECTILE_PEA,        m_image_row: 0, m_damage: 20 },
    ProjectileDefinition { m_projectile_type: ProjectileType::PROJECTILE_SNOWPEA,    m_image_row: 1, m_damage: 20 },
    ProjectileDefinition { m_projectile_type: ProjectileType::PROJECTILE_CABBAGE,    m_image_row: 2, m_damage: 40 },
    ProjectileDefinition { m_projectile_type: ProjectileType::PROJECTILE_MELON,      m_image_row: 3, m_damage: 60 },
    ProjectileDefinition { m_projectile_type: ProjectileType::PROJECTILE_WINTERMELON,m_image_row: 3, m_damage: 60 },
    ProjectileDefinition { m_projectile_type: ProjectileType::PROJECTILE_KERNEL,     m_image_row: 4, m_damage: 20 },
    ProjectileDefinition { m_projectile_type: ProjectileType::PROJECTILE_COB,        m_image_row: 5, m_damage: 40 },
    ProjectileDefinition { m_projectile_type: ProjectileType::PROJECTILE_PUFF,       m_image_row: 6, m_damage: 1 },
    ProjectileDefinition { m_projectile_type: ProjectileType::PROJECTILE_SPIKE,      m_image_row: 7, m_damage: 20 },
    ProjectileDefinition { m_projectile_type: ProjectileType::PROJECTILE_SPIKEROCK,  m_image_row: 8, m_damage: 20 },
    ProjectileDefinition { m_projectile_type: ProjectileType::PROJECTILE_FIREBALL,   m_image_row: 0, m_damage: 40 },
    ProjectileDefinition { m_projectile_type: ProjectileType::PROJECTILE_STAR,       m_image_row: 9, m_damage: 20 },
    ProjectileDefinition { m_projectile_type: ProjectileType::PROJECTILE_BUTTER,     m_image_row: 0, m_damage: 20 },
    ProjectileDefinition { m_projectile_type: ProjectileType::PROJECTILE_FUME,       m_image_row: 0, m_damage: 1 },
    ProjectileDefinition { m_projectile_type: ProjectileType::PROJECTILE_CACTUS,     m_image_row: 0, m_damage: 20 },
    ProjectileDefinition { m_projectile_type: ProjectileType::PROJECTILE_GARLIC,     m_image_row: 0, m_damage: 0 },
    ProjectileDefinition { m_projectile_type: ProjectileType::PROJECTILE_BOBSLED,    m_image_row: 0, m_damage: 0 },
    ProjectileDefinition { m_projectile_type: ProjectileType::PROJECTILE_FIREBALL_BOSS, m_image_row: 0, m_damage: 80 },
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
    pub unsafe fn DoImpact(&mut self, _the_zombie: *mut super::zombie::Zombie) {
        // C++: 根据投射物类型处理伤害和效果
        // [TODO]: 播放命中音效
        // [TODO]: 造成伤害
        // [TODO]: 减速/黄油/溅射等特殊效果
        // [TODO]: 粒子效果
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

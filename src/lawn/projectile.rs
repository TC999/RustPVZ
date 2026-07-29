// [TRANSLATION_NOTE]: Projectile.h -> Rust struct

use crate::const_enums::*;
use super::game_object::GameObject;

#[derive(Clone)]
pub struct ProjectileDefinition {
    pub m_projectile_type: ProjectileType,
    pub m_image_row: i32,
    pub m_damage: i32,
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
}

impl Projectile {
    pub fn new() -> Self {
        Projectile {
            base: GameObject::new(),
            m_frame: 0,
            m_num_frames: 0,
            m_anim_counter: 0,
            m_pos_x: 0.0,
            m_pos_y: 0.0,
            m_pos_z: 0.0,
            m_vel_x: 0.0,
            m_vel_y: 0.0,
            m_vel_z: 0.0,
            m_acc_z: 0.0,
            m_shadow_y: 0.0,
            m_dead: false,
            m_anim_ticks_per_frame: 0,
            m_motion_type: 0,
            m_projectile_type: ProjectileType::PROJECTILE_PEA,
            m_projectile_age: 0,
            m_click_backoff_counter: 0,
            m_rotation: 0.0,
            m_rotation_speed: 0.0,
            m_on_high_ground: false,
            m_damage_range_flags: 0,
            m_hit_torchwood_grid_x: 0,
            m_attachment_id: AttachmentID::ATTACHMENTID_NULL,
            m_cob_target_x: 0.0,
            m_cob_target_row: 0,
            m_target_zombie_id: ZombieID::ZOMBIEID_NULL,
            m_last_portal_x: 0,
        }
    }

    /// C++ Projectile::ProjectileInitialize (Projectile.cpp:62)
    pub unsafe fn ProjectileInitialize(&mut self, theX: i32, theY: i32, theRenderOrder: i32, theRow: i32, theProjectileType: ProjectileType) {
        self.m_pos_x = theX as f32;
        self.m_pos_y = theY as f32;
        self.base.m_render_order = theRenderOrder;
        self.base.m_row = theRow;
        self.m_projectile_type = theProjectileType;
        self.m_dead = false;
        self.m_projectile_age = 0;

        // Set motion type and velocity based on projectile type (C++ ProjectileInitialize)
        match theProjectileType {
            ProjectileType::PROJECTILE_PEA | ProjectileType::PROJECTILE_SNOWPEA
            | ProjectileType::PROJECTILE_FIREBALL
            | ProjectileType::PROJECTILE_SPIKE | ProjectileType::PROJECTILE_SPIKEROCK => {
                self.m_motion_type = 0; // MOTION_NORMAL
                self.m_vel_x = 4.0;
                self.m_anim_ticks_per_frame = 3;
            }
            ProjectileType::PROJECTILE_CABBAGE | ProjectileType::PROJECTILE_MELON
            | ProjectileType::PROJECTILE_WINTERMELON | ProjectileType::PROJECTILE_KERNEL
            | ProjectileType::PROJECTILE_BUTTER | ProjectileType::PROJECTILE_COB => {
                self.m_motion_type = 1; // MOTION_LOBBED
                self.m_vel_x = 3.0;
                self.m_vel_z = -5.0;
                self.m_acc_z = 0.18;
                self.m_anim_ticks_per_frame = 3;
            }
            ProjectileType::PROJECTILE_PUFF => {
                self.m_motion_type = 0; // MOTION_NORMAL
                self.m_vel_x = 3.5;
            }
            ProjectileType::PROJECTILE_STAR => {
                self.m_motion_type = 0;
                self.m_vel_x = 4.0;
                self.m_rotation_speed = 0.3;
                self.m_anim_ticks_per_frame = 2;
            }
            _ => {
                self.m_vel_x = 3.0;
            }
        }
    }

    /// C++ Projectile::Update (Projectile.cpp:936)
    pub unsafe fn Update(&mut self) {
        if self.m_dead { return; }

        let app = self.app();
        let board = self.board();
        if (*app).mGameScene as i32 != GameScenes::SCENE_PLAYING as i32
            && (!board.mCutScene.is_null() && !(*board.mCutScene).ShouldRunUpsellBoard())
        {
            return;
        }

        let mut aTime = 20;
        match self.m_projectile_type {
            ProjectileType::PROJECTILE_PEA | ProjectileType::PROJECTILE_SNOWPEA
            | ProjectileType::PROJECTILE_CABBAGE | ProjectileType::PROJECTILE_MELON
            | ProjectileType::PROJECTILE_WINTERMELON | ProjectileType::PROJECTILE_KERNEL
            | ProjectileType::PROJECTILE_BUTTER | ProjectileType::PROJECTILE_COB
            | ProjectileType::PROJECTILE_SPIKE => {
                aTime = 0;
            }
            _ => {}
        }
        if self.m_projectile_age > aTime {
            self.base.m_render_order = super::board::Board::MakeRenderOrder(
                RenderLayer::RENDER_LAYER_PROJECTILE, self.base.m_row, 0
            );
        }

        if self.m_click_backoff_counter > 0 {
            self.m_click_backoff_counter -= 1;
        }
        self.m_rotation += self.m_rotation_speed;

        self.UpdateMotion();
        // [TODO]: AttachmentUpdateAndMove(mAttachmentID, mPosX, mPosY + mPosZ)
    }

    /// C++ Projectile::UpdateMotion + UpdateNormalMotion + UpdateLobMotion
    unsafe fn UpdateMotion(&mut self) {
        // 动画帧更新
        if self.m_anim_ticks_per_frame > 0 {
            self.m_anim_counter = (self.m_anim_counter + 1) % (self.m_num_frames.max(1) * self.m_anim_ticks_per_frame);
            self.m_frame = self.m_anim_counter / self.m_anim_ticks_per_frame;
        }

        // 运动更新
        if self.m_motion_type == 1 {
            // MOTION_LOBBED — 抛射运动
            self.m_pos_x += self.m_vel_x;
            self.m_pos_z += self.m_vel_z;
            self.m_vel_z += self.m_acc_z;
        } else {
            // MOTION_NORMAL — 直线运动
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

    /// C++ Projectile::DoImpact + PlayImpactSound (Projectile.cpp:819)
    unsafe fn DoImpact(&mut self, _theZombie: *mut super::zombie::Zombie) {
        // [TODO]: PlayImpactSound — 根据弹丸类型和僵尸头盔播放音效
        // [TODO]: 溅射伤害/单目标伤害
        // [TODO]: 粒子效果（豌豆/冰/西瓜/玉米/黄油溅射）
        // [TODO]: 特殊效果（黄油减速、冰西瓜冰冻）
        self.Die();
    }

    /// C++ Projectile::Die (Projectile.cpp:1148)
    pub unsafe fn Die(&mut self) {
        self.m_dead = true;
        // [TODO]: Remove attachment if any
    }

    /// C++ Projectile::Draw (Projectile.cpp:971)
    pub unsafe fn Draw(&self, _g: &mut crate::sexy_app_framework::graphics::graphics::Graphics) {
        if self.m_dead { return; }
        // [TODO]: Draw projectile sprite by type (IMAGE_PROJECTILEPEA, etc.)
        // Apply rotation, scale, and shadow
    }

    /// C++ Projectile::DrawShadow (Projectile.cpp:1069)
    pub unsafe fn DrawShadow(&self, _g: &mut crate::sexy_app_framework::graphics::graphics::Graphics) {
        // [TODO]: Draw shadow ellipse on ground
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

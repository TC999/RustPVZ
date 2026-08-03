// [TRANSLATION_NOTE]: Attachment.h + Attachment.cpp -> Rust
// 附件系统：管理附加到对象上的粒子/动画/轨迹等效果
// 依赖的未实现类型（Reanimation、TodParticleSystem、Trail）用 *mut u8 替代以保持可编译

use crate::const_enums::{EffectType, AttachmentID};
use crate::sexy_app_framework::misc::sexy_matrix::SexyTransform2D;
use crate::sexy_app_framework::misc::sexy_vector::SexyVector2;
use crate::sexy_app_framework::graphics::color::Color;
use crate::sexy_app_framework::graphics::graphics::Graphics;

pub const MAX_EFFECTS_PER_ATTACHMENT: usize = 16;

// ---- 前置类型声明（未实现的 TodLib 类型用 void 指针替代）----
pub type Reanimation = u8;
pub type TodParticleSystem = u8;
pub type Trail = u8;

#[derive(Clone, Copy)]
pub struct AttachEffect {
    pub m_effect_id: u32,
    pub m_effect_type: EffectType,
    pub m_offset: SexyTransform2D,
    pub m_dont_draw_if_parent_hidden: bool,
    pub m_dont_propogate_color: bool,
}

impl AttachEffect {
    pub fn new() -> Self {
        AttachEffect {
            m_effect_id: 0,
            m_effect_type: EffectType::EFFECT_OTHER,
            m_offset: SexyTransform2D::new_with_identity(true),
            m_dont_draw_if_parent_hidden: false,
            m_dont_propogate_color: false,
        }
    }
}

pub struct AttacherInfo {
    pub m_reanim_name: String,
    pub m_track_name: String,
    pub m_anim_rate: f32,
    pub m_loop_type: i32, // ReanimLoopType
}

impl AttacherInfo {
    pub fn new() -> Self {
        AttacherInfo {
            m_reanim_name: String::new(),
            m_track_name: String::new(),
            m_anim_rate: 1.0,
            m_loop_type: 0,
        }
    }
}

pub struct Attachment {
    pub m_effect_array: [AttachEffect; MAX_EFFECTS_PER_ATTACHMENT],
    pub m_num_effects: i32,
    pub m_dead: bool,
}

impl Attachment {
    pub fn new() -> Self {
        Attachment {
            m_effect_array: [AttachEffect::new(); MAX_EFFECTS_PER_ATTACHMENT],
            m_num_effects: 0,
            m_dead: false,
        }
    }

    pub fn process_delete_queue(&mut self) {}
    pub fn update(&mut self) {
        // PruneDeadEffects
        let mut i = 0;
        while i < self.m_num_effects {
            let removed = false;
            // 检查 effect 是否死亡（根据类型不同检查方式不同）
            // C++ 中会检查 Reanimation/粒子/Trail 的 mDead 标志
            match self.m_effect_array[i as usize].m_effect_type {
                EffectType::EFFECT_REANIM => {
                    // if (FindReanimById(effect.mEffectID) == null) -> remove
                }
                EffectType::EFFECT_PARTICLE => {
                    // if (FindParticleById(effect.mEffectID) == null) -> remove
                }
                EffectType::EFFECT_TRAIL => {
                    // if (FindTrailById(effect.mEffectID) == null) -> remove
                }
                _ => {}
            }

            if removed {
                // Shift remaining effects
                for j in (i as usize)..(self.m_num_effects as usize - 1) {
                    self.m_effect_array[j] = self.m_effect_array[j + 1];
                }
                self.m_num_effects -= 1;
            } else {
                i += 1;
            }
        }
    }

    pub fn set_position(&mut self, _the_position: &SexyVector2) {
        // 更新所有 effect 的位置
        for i in 0..self.m_num_effects {
            let effect = &self.m_effect_array[i as usize];
            match effect.m_effect_type {
                EffectType::EFFECT_REANIM => {
                    // reanim->SetPosition
                }
                EffectType::EFFECT_PARTICLE => {
                    // particle->SetPosition
                }
                EffectType::EFFECT_TRAIL => {
                    // trail->AddPoint
                }
                _ => {}
            }
        }
    }

    pub fn set_matrix(&mut self, _the_matrix: &SexyTransform2D) {
        // 设置变换矩阵
    }

    pub fn override_color(&mut self, _the_color: &Color) {
        // 覆盖颜色
    }

    pub fn override_scale(&mut self, _the_scale: f32) {
        // 覆盖缩放
    }

    /// C++ Attachment::Draw (Attachment.cpp:458) — 绘制所有附加效果
    pub fn draw(&mut self, g: &mut Graphics, the_parent_hidden: bool) {
        let a_effect_system = crate::sexy_tod_lib::effect_system::g_effect_system();
        if a_effect_system.is_null() {
            return;
        }
        unsafe {
            let mut i = 0;
            while i < self.m_num_effects as usize {
                let a_attach_effect = &self.m_effect_array[i];
                // C++: 父隐藏时跳过
                if the_parent_hidden && a_attach_effect.m_dont_draw_if_parent_hidden {
                    i += 1;
                    continue;
                }

                match a_attach_effect.m_effect_type {
                    EffectType::EFFECT_REANIM => {
                        // C++: aReanimations.DataArrayTryToGet(mEffectID)->Draw(g)
                        let a_holder = (*a_effect_system).m_reanimation_holder.as_ref();
                        if let Some(a_holder) = a_holder {
                            let a_reanim = a_holder.m_animations.data_array_try_to_get(a_attach_effect.m_effect_id);
                            if !a_reanim.is_null() {
                                (*(a_reanim as *mut crate::sexy_tod_lib::reanimator::Reanimation)).draw(g);
                            }
                        }
                    }
                    EffectType::EFFECT_PARTICLE => {
                        // C++: aParticleSystems.DataArrayTryToGet(mEffectID)->Draw(g)
                        // [TODO]: TodParticleSystem 绘制
                        let _ = a_attach_effect.m_effect_id;
                    }
                    EffectType::EFFECT_TRAIL => {
                        // [TODO]: Trail 绘制
                    }
                    EffectType::EFFECT_ATTACHMENT => {
                        // C++: 递归附件绘制
                        // [TODO]: AttachmentHolder try_to_get + Draw
                    }
                    EffectType::EFFECT_OTHER => {}
                }
                i += 1;
            }
        }
    }

    pub fn attachment_die(&mut self) {
        self.m_dead = true;
        // 让所有 effect 开始死亡动画
    }

    pub fn detach(&mut self) {
        self.m_num_effects = 0;
    }

    pub fn cross_fade(&mut self, _the_cross_fade_name: &str) {
        // 交叉淡出
    }

    pub fn propogate_color(&mut self, _the_color: &Color, _the_enable_additive_color: bool, _the_additive_color: &Color, _the_enable_overlay_color: bool, _the_overlay_color: &Color) {
        // 传播颜色到所有子效果
    }
}

impl Default for Attachment {
    fn default() -> Self {
        Attachment::new()
    }
}

// ---- 全局函数 ----
fn prune_dead_effects(the_attachment: &mut Attachment) {
    let mut i = 0;
    while i < the_attachment.m_num_effects {
        let mut dead = false;
        match the_attachment.m_effect_array[i as usize].m_effect_type {
            EffectType::EFFECT_REANIM => { dead = true; } // 简化
            EffectType::EFFECT_PARTICLE => { dead = true; }
            EffectType::EFFECT_TRAIL => { dead = true; }
            _ => {}
        }
        if dead {
            for j in (i as usize)..(the_attachment.m_num_effects as usize - 1) {
                the_attachment.m_effect_array[j] = the_attachment.m_effect_array[j + 1];
            }
            the_attachment.m_num_effects -= 1;
        } else {
            i += 1;
        }
    }
}

pub fn create_effect_attachment(
    _the_attachment_id: &mut AttachmentID,
    _the_effect_type: EffectType,
    _the_data_id: u32,
    _the_offset_x: f32,
    _the_offset_y: f32,
) -> *mut AttachEffect {
    // 简化实现：不实际分配 attachment，返回 null
    std::ptr::null_mut()
}

pub fn find_first_attachment(_the_attachment_id: &mut AttachmentID) -> *mut AttachEffect {
    std::ptr::null_mut()
}

pub fn is_full_of_attachments(_the_attachment_id: &mut AttachmentID) -> bool {
    false
}

// ---- AttachmentHolder ----
use crate::sexy_tod_lib::data_array::DataArray;

pub struct AttachmentHolder {
    pub m_attachments: DataArray<Attachment>,
}

impl AttachmentHolder {
    pub fn process_delete_queue(&mut self) {}
    pub fn update(&mut self) {}

    pub fn new() -> Self {
        AttachmentHolder {
            m_attachments: DataArray::new(),
        }
    }

    pub fn initialize_holder(&mut self) {
        self.m_attachments.data_array_initialize(64, "AttachmentHolder");
    }

    pub fn dispose_holder(&mut self) {
        unsafe { self.m_attachments.data_array_dispose(); }
    }

    pub fn alloc_attachment(&mut self) -> *mut Attachment {
        unsafe { self.m_attachments.data_array_alloc() }
    }
}

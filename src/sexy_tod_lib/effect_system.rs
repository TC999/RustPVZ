// [TRANSLATION_NOTE]: EffectSystem.h + EffectSystem.cpp -> Rust
// 效果系统：统一管理粒子、轨迹、动画、附件等效果子系统

use crate::sexy_app_framework::graphics::graphics::{Graphics, Image};
use crate::sexy_app_framework::graphics::color::Color;
use crate::sexy_app_framework::misc::rect::Rect;
use crate::sexy_app_framework::misc::sexy_matrix::SexyMatrix3;
use crate::sexy_tod_lib::data_array::DataArray;

// ---- 前置声明（各 Holder 的桩类型，后续填充）----
pub struct TodParticleHolder {
    pub m_particles: DataArray<u8>, // 实际应为 DataArray<TodParticleSystem>
}

impl TodParticleHolder {
    pub fn new() -> Self { TodParticleHolder { m_particles: DataArray::new() } }
    pub fn initialize_holder(&mut self) { self.m_particles.data_array_initialize(64u32, "TodParticleHolder"); }
    pub fn dispose_holder(&mut self) { unsafe { self.m_particles.data_array_dispose(); } }
    pub fn process_delete_queue(&mut self) {}
    pub fn update(&mut self) {}
}

pub struct ReanimationHolder {
    pub m_animations: DataArray<u8>, // 实际应为 DataArray<Reanimation>
}

impl ReanimationHolder {
    pub fn new() -> Self { ReanimationHolder { m_animations: DataArray::new() } }
    pub fn initialize_holder(&mut self) { self.m_animations.data_array_initialize(64u32, "ReanimationHolder"); }
    pub fn dispose_holder(&mut self) { unsafe { self.m_animations.data_array_dispose(); } }
    pub fn process_delete_queue(&mut self) {}
    pub fn update(&mut self) {}
}

// TrailHolder 在 trail.rs 中定义

// ---- TodTriangleGroup ----
pub const MAX_TRIANGLES: i32 = 256;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct TodTriVertex {
    pub x: f32,
    pub y: f32,
    pub u: f32,
    pub v: f32,
    pub color: u32,
}

pub struct TodTriangleGroup {
    pub m_image: *mut Image,
    pub m_vert_array: [[TodTriVertex; 3]; MAX_TRIANGLES as usize],
    pub m_triangle_count: i32,
    pub m_draw_mode: i32,
}

impl TodTriangleGroup {
    pub fn new() -> Self {
        TodTriangleGroup {
            m_image: std::ptr::null_mut(),
            m_vert_array: [[TodTriVertex { x: 0.0, y: 0.0, u: 0.0, v: 0.0, color: 0 }; 3]; MAX_TRIANGLES as usize],
            m_triangle_count: 0,
            m_draw_mode: 0,
        }
    }

    pub fn draw_group(&self, _g: &mut Graphics) {
        // placeholder: 实际会调用 TodDrawTriangle_* 系列函数
    }

    pub fn add_triangle(&mut self, _g: &mut Graphics, _the_image: *mut Image, _the_matrix: &SexyMatrix3, _the_clip_rect: &Rect, _the_color: &Color, _the_draw_mode: i32, _the_src_rect: &Rect) {
        if self.m_triangle_count < MAX_TRIANGLES {
            self.m_triangle_count += 1;
        }
    }
}

// ---- 全局变量 ----
pub static mut G_TOD_TRIANGLE_DRAW_ADDITIVE: bool = false;
pub static mut G_EFFECT_SYSTEM: *mut EffectSystem = std::ptr::null_mut();

// ---- TodDrawTriangle 函数（简化桩）----
// C++ 中这些函数通过宏生成数十个变体，涵盖不同像素格式/混合模式
// 此处简化为一个统一的桩函数
pub fn tod_draw_triangle(
    _p_verts: *mut u8,        // SWVertex*
    _p_frame_buffer: *mut u8,
    _bytepitch: u32,
    _texture_info: *mut u8,   // SWTextureInfo*
    _global_diffuse: *mut u8, // SWDiffuse&
) {
    // placeholder
}

// ---- EffectSystem ----
use crate::sexy_tod_lib::trail::TrailHolder;
use crate::sexy_tod_lib::attachment::AttachmentHolder;

pub struct EffectSystem {
    pub m_particle_holder: Option<Box<TodParticleHolder>>,
    pub m_trail_holder: Option<Box<TrailHolder>>,
    pub m_reanimation_holder: Option<Box<ReanimationHolder>>,
    pub m_attachment_holder: Option<Box<AttachmentHolder>>,
}

impl EffectSystem {
    pub fn new() -> Self {
        EffectSystem {
            m_particle_holder: None,
            m_trail_holder: None,
            m_reanimation_holder: None,
            m_attachment_holder: None,
        }
    }

    pub fn effect_system_initialize(&mut self) {
        let mut particle = Box::new(TodParticleHolder::new());
        particle.initialize_holder();
        self.m_particle_holder = Some(particle);

        let mut trail = Box::new(TrailHolder::new());
        trail.initialize_holder();
        self.m_trail_holder = Some(trail);

        let mut reanim = Box::new(ReanimationHolder::new());
        reanim.initialize_holder();
        self.m_reanimation_holder = Some(reanim);

        let mut attachment = Box::new(AttachmentHolder::new());
        attachment.initialize_holder();
        self.m_attachment_holder = Some(attachment);
    }

    /// C++ EffectSystem::ProcessDeleteQueue (EffectSystem.cpp:79) — 删除队列处理
    pub fn process_delete_queue(&mut self) {
        if let Some(a_particle_holder) = self.m_particle_holder.as_mut() {
            a_particle_holder.process_delete_queue();
        }
        if let Some(a_trail_holder) = self.m_trail_holder.as_mut() {
            a_trail_holder.process_delete_queue();
        }
        if let Some(a_reanimation_holder) = self.m_reanimation_holder.as_mut() {
            a_reanimation_holder.process_delete_queue();
        }
        if let Some(a_attachment_holder) = self.m_attachment_holder.as_mut() {
            a_attachment_holder.process_delete_queue();
        }
    }
    pub fn effect_system_dispose(&mut self) {
        self.m_particle_holder = None;
        self.m_trail_holder = None;
        self.m_reanimation_holder = None;
        self.m_attachment_holder = None;
    }

    pub fn effect_system_free_all(&mut self) {
        self.effect_system_dispose();
        self.effect_system_initialize();
    }



    pub fn update(&mut self) {
        if let Some(ref mut p) = self.m_particle_holder {
            p.update();
        }
        if let Some(ref mut r) = self.m_reanimation_holder {
            r.update();
        }
    }
}

impl Drop for EffectSystem {
    fn drop(&mut self) {
        self.effect_system_dispose();
    }
}

// ---- gEffectSystem 全局访问 ----
pub fn get_effect_system() -> Option<&'static mut EffectSystem> {
    unsafe {
        if G_EFFECT_SYSTEM.is_null() {
            None
        } else {
            Some(&mut *G_EFFECT_SYSTEM)
        }
    }
}

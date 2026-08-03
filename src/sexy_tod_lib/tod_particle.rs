// [TRANSLATION_NOTE]: TodParticle.h + TodParticle.cpp -> Rust
// 粒子系统：粒子定义、发射、更新与渲染

use crate::sexy_app_framework::graphics::graphics::{Graphics, Image};
use crate::sexy_tod_lib::trail::FloatParameterTrack;

pub const MAX_PARTICLES_SIZE: i32 = 900;
pub const MAX_PARTICLE_FIELDS: i32 = 4;

// ============================================================
// 枚举
// ============================================================
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(i32)]
pub enum ParticleFlags {
    PARTICLE_RANDOM_LAUNCH_SPIN, PARTICLE_ALIGN_LAUNCH_SPIN, PARTICLE_ALIGN_TO_PIXELS,
    PARTICLE_SYSTEM_LOOPS, PARTICLE_PARTICLE_LOOPS, PARTICLE_PARTICLES_DONT_FOLLOW,
    PARTICLE_RANDOM_START_TIME, PARTICLE_DIE_IF_OVERLOADED, PARTICLE_ADDITIVE,
    PARTICLE_FULLSCREEN, PARTICLE_SOFTWARE_ONLY, PARTICLE_HARDWARE_ONLY,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(i32)]
pub enum ParticleFieldType {
    FIELD_INVALID, FIELD_FRICTION, FIELD_ACCELERATION, FIELD_ATTRACTOR,
    FIELD_MAX_VELOCITY, FIELD_VELOCITY, FIELD_POSITION, FIELD_SYSTEM_POSITION,
    FIELD_GROUND_CONSTRAINT, FIELD_SHAKE, FIELD_CIRCLE, FIELD_AWAY,
    PARTICLE_FIELD_COUNT,
}

// ============================================================
// 粒子定义数据结构
// ============================================================

/// 粒子场
#[repr(C)]
pub struct ParticleField {
    pub m_field_type: ParticleFieldType,
    pub m_x: FloatParameterTrack,
    pub m_y: FloatParameterTrack,
}

impl ParticleField {
    pub fn new() -> Self {
        ParticleField {
            m_field_type: ParticleFieldType::FIELD_INVALID,
            m_x: FloatParameterTrack::new(),
            m_y: FloatParameterTrack::new(),
        }
    }
}

/// 发射器定义
#[repr(C)]
pub struct TodEmitterDefinition {
    pub m_image: *mut Image,
    pub m_image_row: i32,
    pub m_image_col: i32,
    pub m_image_frames: i32,
    pub m_animated: bool,
    pub m_particle_flags: u32,
    pub m_emitter_type: i32,
    pub m_name: String,
    pub m_system_duration: FloatParameterTrack,
    pub m_on_duration: FloatParameterTrack,
    pub m_cross_fade_duration: FloatParameterTrack,
    pub m_spawn_rate: FloatParameterTrack,
    pub m_spawn_min_active: FloatParameterTrack,
    pub m_spawn_max_active: FloatParameterTrack,
    pub m_spawn_max_launched: FloatParameterTrack,
    pub m_emitter_radius: FloatParameterTrack,
    pub m_emitter_offset_x: FloatParameterTrack,
    pub m_emitter_offset_y: FloatParameterTrack,
    pub m_emitter_box_x: FloatParameterTrack,
    pub m_emitter_box_y: FloatParameterTrack,
    pub m_particle_field_defs: *mut ParticleField,
    pub m_particle_field_def_count: i32,
    pub m_launch_speed: FloatParameterTrack,
    pub m_launch_speed_z: FloatParameterTrack,
    pub m_launch_angle: FloatParameterTrack,
    pub m_launch_angle_z: FloatParameterTrack,
    pub m_launch_angle_range: FloatParameterTrack,
    pub m_launch_angle_range_z: FloatParameterTrack,
    pub m_launch_min_opacity: FloatParameterTrack,
    pub m_launch_max_opacity: FloatParameterTrack,
    pub m_launch_scale: FloatParameterTrack,
    pub m_duration: FloatParameterTrack,
    pub m_velocity_center_x: FloatParameterTrack,
    pub m_velocity_center_y: FloatParameterTrack,
    pub m_velocity_center_z: FloatParameterTrack,
    pub m_alpha_center: FloatParameterTrack,
    pub m_scale_center: FloatParameterTrack,
    pub m_launch_speed_center: FloatParameterTrack,
    pub m_launch_angle_center: FloatParameterTrack,
    pub m_launch_min_opacity_center: FloatParameterTrack,
}

impl TodEmitterDefinition {
    pub fn new() -> Self {
        TodEmitterDefinition {
            m_image: std::ptr::null_mut(), m_image_row: 0, m_image_col: 0,
            m_image_frames: 1, m_animated: false, m_particle_flags: 0,
            m_emitter_type: 0, m_name: String::new(),
            m_system_duration: FloatParameterTrack::new(),
            m_on_duration: FloatParameterTrack::new(),
            m_cross_fade_duration: FloatParameterTrack::new(),
            m_spawn_rate: FloatParameterTrack::new(),
            m_spawn_min_active: FloatParameterTrack::new(),
            m_spawn_max_active: FloatParameterTrack::new(),
            m_spawn_max_launched: FloatParameterTrack::new(),
            m_emitter_radius: FloatParameterTrack::new(),
            m_emitter_offset_x: FloatParameterTrack::new(),
            m_emitter_offset_y: FloatParameterTrack::new(),
            m_emitter_box_x: FloatParameterTrack::new(),
            m_emitter_box_y: FloatParameterTrack::new(),
            m_particle_field_defs: std::ptr::null_mut(),
            m_particle_field_def_count: 0,
            m_launch_speed: FloatParameterTrack::new(),
            m_launch_speed_z: FloatParameterTrack::new(),
            m_launch_angle: FloatParameterTrack::new(),
            m_launch_angle_z: FloatParameterTrack::new(),
            m_launch_angle_range: FloatParameterTrack::new(),
            m_launch_angle_range_z: FloatParameterTrack::new(),
            m_launch_min_opacity: FloatParameterTrack::new(),
            m_launch_max_opacity: FloatParameterTrack::new(),
            m_launch_scale: FloatParameterTrack::new(),
            m_duration: FloatParameterTrack::new(),
            m_velocity_center_x: FloatParameterTrack::new(),
            m_velocity_center_y: FloatParameterTrack::new(),
            m_velocity_center_z: FloatParameterTrack::new(),
            m_alpha_center: FloatParameterTrack::new(),
            m_scale_center: FloatParameterTrack::new(),
            m_launch_speed_center: FloatParameterTrack::new(),
            m_launch_angle_center: FloatParameterTrack::new(),
            m_launch_min_opacity_center: FloatParameterTrack::new(),
        }
    }
}

/// 粒子定义
#[repr(C)]
pub struct TodParticleDefinition {
    pub m_emitter_defs: *mut TodEmitterDefinition,
    pub m_emitter_def_count: i32,
    pub m_image: *mut Image,
}

impl TodParticleDefinition {
    pub fn new() -> Self {
        TodParticleDefinition {
            m_emitter_defs: std::ptr::null_mut(),
            m_emitter_def_count: 0,
            m_image: std::ptr::null_mut(),
        }
    }
}

// ============================================================
// 运行时粒子实例
// ============================================================

/// 单个粒子
pub struct Particle {
    pub m_dead: bool,
    pub m_x: f32, pub m_y: f32, pub m_z: f32,
    pub m_vx: f32, pub m_vy: f32, pub m_vz: f32,
    pub m_scale: f32,
    pub m_alpha: f32,
    pub m_rotation: f32,
    pub m_age: f32,
    pub m_duration: f32,
    pub m_particle_field_idx: i32,
    pub m_image: *mut Image,
    pub m_frame: f32,
}

impl Particle {
    /// C++ ParticleDraw — 粒子绘制（基础版：缩放/透明度，旋转 TODO）
    pub fn draw(&self, g: &mut Graphics) {
        if self.m_dead || self.m_image.is_null() {
            return;
        }
        let a_image = unsafe { &*self.m_image };
        let a_dest_w = (a_image.m_width as f32 * self.m_scale) as i32;
        let a_dest_h = (a_image.m_height as f32 * self.m_scale) as i32;
        let a_dest_x = (self.m_x - a_dest_w as f32 / 2.0) as i32;
        let a_dest_y = (self.m_y - a_dest_h as f32 / 2.0) as i32;
        g.DrawImageDestSrc(
            a_image,
            &crate::sexy_app_framework::misc::rect::Rect::new(a_dest_x, a_dest_y, a_dest_w, a_dest_h),
            &crate::sexy_app_framework::misc::rect::Rect::new(0, 0, a_image.m_width, a_image.m_height),
        );
        // [TODO]: 旋转（mRotation）与透明度（mAlpha）绘制
    }    pub fn update(&mut self) {
        // C++ TodParticle::Update — 粒子运动与寿命
        if self.m_dead {
            return;
        }
        self.m_age += 1.0;
        // C++: 位置积分（速度；轨道插值 TODO）
        self.m_x += self.m_vx;
        self.m_y += self.m_vy;
        self.m_z += self.m_vz;
        // C++: 速度阻尼近似（无阻力轨道）
        self.m_vx *= 0.96;
        self.m_vy *= 0.96;
        // C++: 寿命结束死亡
        if self.m_duration > 0.0 && self.m_age >= self.m_duration {
            self.m_dead = true;
        }
    }
    pub fn new() -> Self {
        Particle {
            m_dead: true,
            m_x: 0.0, m_y: 0.0, m_z: 0.0,
            m_vx: 0.0, m_vy: 0.0, m_vz: 0.0,
            m_scale: 1.0, m_alpha: 1.0, m_rotation: 0.0,
            m_age: 0.0, m_duration: 0.0,
            m_particle_field_idx: 0,
            m_image: std::ptr::null_mut(),
            m_frame: 0.0,
        }
    }
}

/// 粒子发射器实例
pub struct TodParticleEmitter {
    pub m_definition: *mut TodEmitterDefinition,
    pub m_particles: Vec<Particle>,
    pub m_active_count: i32,
    pub m_age: f32,
    pub m_dead: bool,
    pub m_x: f32, pub m_y: f32,
}

impl TodParticleEmitter {
    pub fn new() -> Self {
        TodParticleEmitter {
            m_definition: std::ptr::null_mut(),
            m_particles: Vec::new(),
            m_active_count: 0,
            m_age: 0.0, m_dead: false,
            m_x: 0.0, m_y: 0.0,
        }
    }
    /// C++ TodParticleEmitter::SpawnParticle (TodParticle.cpp:333) — 发射粒子
    /// [TRANSLATION_NOTE]: 轨道求值（FloatTrackEvaluate）未翻译，用随机值近似
    pub unsafe fn SpawnParticle(&mut self) -> *mut Particle {
        // C++: 复用死亡的粒子或创建新粒子
        let mut a_particle = None;
        for a_existing in self.m_particles.iter_mut() {
            if a_existing.m_dead {
                a_particle = Some(a_existing as *mut Particle);
                break;
            }
        }
        let a_particle = match a_particle {
            Some(p) => p,
            None => {
                self.m_particles.push(Particle::new());
                self.m_particles.last_mut().unwrap() as *mut Particle
            }
        };

        // C++: 初始化粒子（轨道求值近似：随机）
        let a_particle = &mut *a_particle;
        a_particle.m_dead = false;
        a_particle.m_age = 0.0;
        a_particle.m_duration = (crate::sexy_app_framework::common::rand_int() % 60 + 30) as f32;
        a_particle.m_x = self.m_x;
        a_particle.m_y = self.m_y;
        a_particle.m_z = 0.0;
        // C++: 发射角度随机 + 初始速度
        let a_angle = (crate::sexy_app_framework::common::rand_int() % 628) as f32 / 100.0;
        let a_speed = (crate::sexy_app_framework::common::rand_int() % 50 + 10) as f32 * 0.01;
        a_particle.m_vx = a_angle.cos() * a_speed;
        a_particle.m_vy = a_angle.sin() * a_speed;
        a_particle.m_vz = 0.0;
        a_particle.m_scale = 1.0;
        a_particle.m_alpha = 1.0;
        a_particle.m_rotation = 0.0;
        self.m_active_count += 1;
        a_particle
    }
    pub fn update(&mut self) {
        // C++ TodParticleEmitter::Update (TodParticle.cpp:788) — 发射器更新
        if self.m_dead {
            return;
        }
        self.m_age += 1.0;
        // C++: mSystemAge >= mSystemDuration → 死亡（循环标志 TODO）
        // [TODO]: mSystemDuration（FloatParameterTrack 轨道求值）与 PARTICLE_SYSTEM_LOOPS 循环
        if !self.m_definition.is_null() {
            unsafe {
                if ((*self.m_definition).m_particle_flags & (1 << 3 /* PARTICLE_SYSTEM_LOOPS */)) != 0 {
                    // C++: 循环发射器重置年龄（时长 TODO）
                }
            }
        }
        // C++: 更新已有粒子（遍历）
        for a_particle in self.m_particles.iter_mut() {
            if !a_particle.m_dead {
                a_particle.update();
            }
        }
        // [TODO]: 粒子发射（SpawnParticle）
    }
    pub fn draw(&self, g: &mut Graphics) {
        // C++: 遍历粒子绘制
        for a_particle in &self.m_particles {
            a_particle.draw(g);
        }
    }
}

/// 粒子系统实例
pub struct TodParticleSystem {
    pub m_definition: *mut TodParticleDefinition,
    pub m_emitters: Vec<TodParticleEmitter>,
    pub m_age: f32,
    pub m_dead: bool,
    pub m_x: f32, pub m_y: f32,
    pub m_render_order: i32,
    pub m_is_attachment: bool,
    pub m_dont_update: bool,
}

impl TodParticleSystem {
    pub fn new() -> Self {
        TodParticleSystem {
            m_definition: std::ptr::null_mut(),
            m_emitters: Vec::new(),
            m_age: 0.0, m_dead: false,
            m_x: 0.0, m_y: 0.0,
            m_render_order: 0, m_is_attachment: false,
            m_dont_update: false,
        }
    }
    pub fn update(&mut self) {
        // C++ TodParticleSystem::Update (TodParticle.cpp:726)
        if self.m_dont_update {
            return;
        }
        let mut an_emitter_alive = false;
        for an_emitter in self.m_emitters.iter_mut() {
            an_emitter.update();
            if !an_emitter.m_dead {
                an_emitter_alive = true;
            }
        }
        if !an_emitter_alive {
            self.m_dead = true;
        }
    }
    pub fn draw(&self, _g: &mut Graphics) {
        // [TODO]: 粒子系统渲染（逐发射器）
    }
    pub fn set_position(&mut self, x: f32, y: f32) { self.m_x = x; self.m_y = y; }
    pub fn set_scale(&mut self, _scale: f32) {}
}

// ============================================================
// 全局函数（桩）
// ============================================================

pub fn particle_load_definitions() {}
pub fn particle_ensure_definition_loaded(_the_particle_effect: i32) {}

pub fn particle_draw_system(_g: &mut Graphics, _system: &TodParticleSystem) {}
pub fn particle_update_system(system: &mut TodParticleSystem) {
    system.update();
}

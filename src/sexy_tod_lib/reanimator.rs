// [TRANSLATION_NOTE]: Reanimator.h + Reanimator.cpp -> Rust
// 动画系统：管理动画定义、实例、变换和绘制

use crate::const_enums::ReanimationType;
use crate::sexy_app_framework::graphics::graphics::{Graphics, Image};
use crate::sexy_app_framework::graphics::color::Color;
use crate::sexy_tod_lib::definition::{DefField, DefMap, DefFieldType};
use crate::sexy_app_framework::misc::sexy_matrix::SexyTransform2D;
use crate::sexy_tod_lib::data_array::DataArray;

pub const DEFAULT_FIELD_PLACEHOLDER: f32 = -10000.0;
pub const SECONDS_PER_UPDATE: f64 = 0.01;

// ============================================================
// 枚举
// ============================================================
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(i32)]
pub enum ReanimFlags {
    REANIM_NO_ATLAS,
    REANIM_FAST_DRAW_IN_SW_MODE,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(i32)]
pub enum ReanimLoopType {
    REANIM_LOOP = 0,
    REANIM_LOOP_FULL_LAST_FRAME = 1,
    REANIM_PLAY_ONCE = 2,
    REANIM_PLAY_ONCE_AND_HOLD = 3,
    REANIM_PLAY_ONCE_FULL_LAST_FRAME = 4,
    REANIM_PLAY_ONCE_FULL_LAST_FRAME_AND_HOLD = 5,
}

// ============================================================
// 数据结构
// ============================================================

/// 动画变换（位置/旋转/缩放/颜色等）
#[repr(C)]
#[derive(Clone)]
pub struct ReanimatorTransform {
    pub m_trans_x: f32,
    pub m_trans_y: f32,
    pub m_skew_x: f32,
    pub m_skew_y: f32,
    pub m_scale_x: f32,
    pub m_scale_y: f32,
    pub m_frame: f32,
    pub m_alpha: f32,
    pub m_color: Color,
    pub m_additive_color: Color,
    pub m_overlay_color: Color,
    pub m_image: *mut Image,
    pub m_font: *mut u8, // *mut _Font
    pub m_shake_x: f32,
    pub m_shake_y: f32,
    pub m_shake_interp: f32,
    pub m_scale_interp: f32,
    pub m_shake_magnitude: f32,
}

impl ReanimatorTransform {
    pub fn new() -> Self {
        ReanimatorTransform {
            m_trans_x: 0.0, m_trans_y: 0.0, m_skew_x: 0.0, m_skew_y: 0.0,
            m_scale_x: 1.0, m_scale_y: 1.0, m_frame: 0.0, m_alpha: 1.0,
            m_color: Color::from_components(255, 255, 255),
            m_additive_color: Color::new(),
            m_overlay_color: Color::new(),
            m_image: std::ptr::null_mut(),
            m_font: std::ptr::null_mut(),
            m_shake_x: 0.0, m_shake_y: 0.0, m_shake_interp: 0.0,
            m_scale_interp: 0.0, m_shake_magnitude: 0.0,
        }
    }
}

/// 动画变换数组
pub struct ReanimatorFrameTime {
    pub m_anim_frame_before_int: i32,
    pub m_anim_frame_after_int: i32,
    pub m_fraction: f32,
}

pub struct ReanimatorTransformArray {
    pub m_transforms: *mut ReanimatorTransform,
    pub count: i32,
}

/// 动画轨道
pub struct ReanimatorTrack {
    pub m_name: String,
    pub m_transforms: ReanimatorTransformArray,
}

impl ReanimatorTrack {
    pub fn new() -> Self {
        ReanimatorTrack {
            m_name: String::new(),
            m_transforms: ReanimatorTransformArray { m_transforms: std::ptr::null_mut(), count: 0 },
        }
    }
}

/// 动画轨道数组
pub struct ReanimatorTrackArray {
    pub tracks: *mut ReanimatorTrack,
    pub count: i32,
}

/// 动画定义
pub struct ReanimatorDefinition {
    pub m_tracks: ReanimatorTrackArray,
    pub m_anim_ratio: f32,
    pub m_fps: f32,
}

impl ReanimatorDefinition {
    pub fn new() -> Self {
        ReanimatorDefinition {
            m_tracks: ReanimatorTrackArray { tracks: std::ptr::null_mut(), count: 0 },
            m_anim_ratio: 1.0,
            m_fps: 12.0,
        }
    }
}

// ============================================================
// ReanimatorTrackInstance - 轨道实例
// ============================================================
#[derive(Clone)]
pub struct ReanimatorTrackInstance {
    pub m_blend_counter: i32,
    pub m_blend_time: i32,
    pub m_blend_transform: ReanimatorTransform,
    pub m_shake_override: f32,
    pub m_shake_x: f32,
    pub m_shake_y: f32,
    pub m_render_group: i32,
    pub m_track_color: crate::sexy_app_framework::graphics::color::Color,
    pub m_ignore_clip_rect: bool,
    pub m_truncate_disappearing_frames: bool,
    pub m_ignore_color_override: bool,
    pub m_ignore_extra_additive_color: bool,
}

impl ReanimatorTrackInstance {
    pub fn new() -> Self {
        ReanimatorTrackInstance {
            m_blend_counter: 0,
            m_blend_time: 0,
            m_blend_transform: ReanimatorTransform::new(),
            m_shake_override: 0.0,
            m_shake_x: 0.0,
            m_shake_y: 0.0,
            m_render_group: 0,
            m_track_color: crate::sexy_app_framework::graphics::color::Color::from_components(255, 255, 255),
            m_ignore_clip_rect: false,
            m_truncate_disappearing_frames: false,
            m_ignore_color_override: false,
            m_ignore_extra_additive_color: false,
        }
    }
}

// ============================================================
// Reanimation - 动画实例
// ============================================================
pub struct Reanimation {
    pub m_definition: *mut ReanimatorDefinition,
    pub m_reanim_type: ReanimationType,
    pub m_pos_x: f32,
    pub m_pos_y: f32,
    pub m_anim_time: f32,
    pub m_frame_base: f32,
    pub m_anim_rate: f32,
    pub m_loop_type: ReanimLoopType,
    pub m_loop_count: i32,
    pub m_dead: bool,
    pub m_dead_no_effect: bool,
    pub m_render_order: i32,
    pub m_overlay_color: Color,
    pub m_color_override: Color,
    pub m_enable_additive_color: bool,
    pub m_additive_color_override: Color,
    pub m_enable_overlay_color: bool,
    pub m_overlay_color_override: Color,
    pub m_x: f32,
    pub m_y: f32,
    pub m_scale: f32,
    pub m_last_processed_time: f32,
    pub m_is_attachment: bool,
    pub m_attractor_system: *mut u8,
    pub m_particle_attachment_holder: *mut u8,
    pub m_attachment_id: u32,
    pub m_extra_additive_color: Color,
    pub m_extra_overlay_color: Color,
    pub m_extra_overlay_alpha: f32,
    pub m_frame_start: i32,
    pub m_frame_count: i32,
    pub m_track_instances: Vec<ReanimatorTrackInstance>,
}

impl Reanimation {
    pub fn new() -> Self {
        Reanimation {
            m_definition: std::ptr::null_mut(),
            m_reanim_type: ReanimationType::REANIM_NONE,
            m_pos_x: 0.0,
            m_pos_y: 0.0,
            m_anim_time: 0.0, m_frame_base: 0.0, m_anim_rate: 1.0,
            m_loop_type: ReanimLoopType::REANIM_PLAY_ONCE,
            m_loop_count: 0,
            m_dead: false, m_dead_no_effect: false,
            m_render_order: 0,
            m_overlay_color: Color::new(),
            m_color_override: Color::from_components(255, 255, 255),
            m_enable_additive_color: false,
            m_additive_color_override: Color::new(),
            m_enable_overlay_color: false,
            m_overlay_color_override: Color::new(),
            m_x: 0.0, m_y: 0.0, m_scale: 1.0,
            m_last_processed_time: 0.0,
            m_is_attachment: false,
            m_attractor_system: std::ptr::null_mut(),
            m_particle_attachment_holder: std::ptr::null_mut(),
            m_attachment_id: std::u32::MAX,
            m_extra_additive_color: Color::new(),
            m_extra_overlay_color: Color::new(),
            m_extra_overlay_alpha: 0.0,
            m_frame_start: 0,
            m_frame_count: 1,
            m_track_instances: Vec::new(),
        }
    }

    // ================================================================
    // ★ 核心动画方法 (C++ Reanimation 1:1 映射)
    // ================================================================

    /// C++ Reanimation::Play (指定同步轨道)
    pub fn reanimation_play(&mut self, _synced: &str) {
        // [TODO]: Reset animation time based on synced track
    }

    pub fn reanimation_play_with_sync(&mut self, _synced: &str, _frame_base: f32) {
        // [TODO]: Play with explicit frame base
    }

    /// C++ Reanimation::SetPosition — 设置动画位置
    pub fn set_position(&mut self, the_pos_x: f32, the_pos_y: f32) {
        self.m_x = the_pos_x;
        self.m_y = the_pos_y;
    }

    /// C++ Reanimation::OverrideScale — 覆盖缩放
    pub fn override_scale(&mut self, the_scale_x: f32, the_scale_y: f32) {
        self.m_scale = the_scale_x;
        // [TODO]: mScaleY 独立缩放（当前用统一缩放）
        let _ = the_scale_y;
    }

    /// C++ Reanimation::StartBlend (Reanimator.cpp:1064) — 混合过渡
    /// [TRANSLATION_NOTE]: mTrackInstances 未翻译，混合过渡简化
    pub fn start_blend(&mut self, _the_blend_time: i32) {
        // C++: 为每个非空白帧轨道记录混合变换（mBlendTransform/mBlendTime/mBlendCounter）
    }

    /// C++ Reanimation::ReanimationInitialize (Reanimator.cpp:387)
    pub fn reanimation_initialize(&mut self, the_x: f32, the_y: f32, the_definition: *mut ReanimatorDefinition) {
        self.m_definition = the_definition;
        self.m_pos_x = the_x;
        self.m_pos_y = the_y;
        self.m_anim_time = 0.0;
        self.m_anim_rate = 1.0;
        self.m_loop_type = ReanimLoopType::REANIM_LOOP;
        self.m_dead = false;
        // C++: mTrackInstances = new ReanimatorTrackInstance[mTrackCount]
        let a_track_count = if the_definition.is_null() { 0 } else { unsafe { (*the_definition).m_tracks.count as usize } };
        self.m_track_instances = vec![ReanimatorTrackInstance::new(); a_track_count];
    }

    /// C++ Reanimation::ReanimationInitializeType (Reanimator.cpp:348)
    pub fn reanimation_initialize_type(&mut self, the_x: f32, the_y: f32, the_reanim_type: ReanimationType) {
        // C++: ReanimatorEnsureDefinitionLoaded(theReanimType, false) → gReanimatorDefArray[theReanimType]
        crate::sexy_tod_lib::reanimator::reanimator_ensure_definition_loaded(the_reanim_type);
        let a_def = unsafe { G_REANIMATOR_DEF_ARRAY[the_reanim_type as usize] };
        self.m_reanim_type = the_reanim_type;
        self.reanimation_initialize(the_x, the_y, a_def);
    }
    /// C++ Reanimation::Update — 推进动画时间（完整 loop 处理）
    pub fn reanimation_update(&mut self) {
        if self.m_frame_count == 0 || self.m_dead {
            return;
        }

        // C++: mLastFrameTime = mAnimTime; mAnimTime += SECONDS_PER_UPDATE * mAnimRate / mFrameCount;
        self.m_last_processed_time = self.m_anim_time;
        self.m_anim_time += 0.01 * self.m_anim_rate / self.m_frame_count as f32;

        if self.m_anim_rate > 0.0 {
            // C++: 正向播放的循环/结束处理
            match self.m_loop_type {
                ReanimLoopType::REANIM_LOOP | ReanimLoopType::REANIM_LOOP_FULL_LAST_FRAME => {
                    while self.m_anim_time >= 1.0 {
                        self.m_loop_count += 1;
                        self.m_anim_time -= 1.0;
                    }
                }
                ReanimLoopType::REANIM_PLAY_ONCE | ReanimLoopType::REANIM_PLAY_ONCE_FULL_LAST_FRAME => {
                    if self.m_anim_time >= 1.0 {
                        self.m_loop_count = 1;
                        self.m_anim_time = 1.0;
                        self.m_dead = true;
                    }
                }
                ReanimLoopType::REANIM_PLAY_ONCE_AND_HOLD | ReanimLoopType::REANIM_PLAY_ONCE_FULL_LAST_FRAME_AND_HOLD => {
                    if self.m_anim_time >= 1.0 {
                        self.m_loop_count = 1;
                        self.m_anim_time = 1.0;
                    }
                }
            }
        } else {
            // C++: 反向播放的循环/结束处理
            match self.m_loop_type {
                ReanimLoopType::REANIM_LOOP | ReanimLoopType::REANIM_LOOP_FULL_LAST_FRAME => {
                    while self.m_anim_time < 0.0 {
                        self.m_loop_count += 1;
                        self.m_anim_time += 1.0;
                    }
                }
                ReanimLoopType::REANIM_PLAY_ONCE | ReanimLoopType::REANIM_PLAY_ONCE_FULL_LAST_FRAME => {
                    if self.m_anim_time < 0.0 {
                        self.m_loop_count = 1;
                        self.m_anim_time = 0.0;
                        self.m_dead = true;
                    }
                }
                ReanimLoopType::REANIM_PLAY_ONCE_AND_HOLD | ReanimLoopType::REANIM_PLAY_ONCE_FULL_LAST_FRAME_AND_HOLD => {
                    if self.m_anim_time < 0.0 {
                        self.m_loop_count = 1;
                        self.m_anim_time = 0.0;
                    }
                }
            }
        }
    }

    /// C++ Reanimation::DrawTrack (Reanimator.cpp:637) — 绘制单条轨道（简化版）
    /// [TRANSLATION_NOTE]: mTrackInstances/ReanimAtlas 未翻译，颜色混合/图集绘制 TODO，
    /// 保留核心：变换插值 → 图像帧 → 基础绘制
    pub fn draw_track(&self, g: &mut Graphics, the_track_index: i32) -> bool {
        unsafe {
            if self.m_definition.is_null() {
                return false;
            }
            let mut a_transform = ReanimatorTransform::new();
            self.get_current_transform(the_track_index, &mut a_transform);
            let a_image_frame = crate::sexy_tod_lib::tod_common::float_round_to_int(a_transform.m_frame);
            if a_image_frame < 0 {
                return false;
            }

            // C++: 颜色混合（mTrackColor × mColorOverride × Graphics 颜色）
            let a_track_color = self.m_track_instances.get(the_track_index as usize).map_or(
                crate::sexy_app_framework::graphics::color::Color::from_components(255, 255, 255),
                |t| t.m_track_color,
            );
            let a_ignore_color_override = self.m_track_instances.get(the_track_index as usize).map_or(false, |t| t.m_ignore_color_override);
            let mut a_color = if a_ignore_color_override {
                a_track_color
            } else {
                // C++: ColorsMultiply(aTrackColor, mColorOverride) 正片叠底
                let mut a_c = crate::sexy_app_framework::graphics::color::Color::from_components(
                    a_track_color.m_red * self.m_color_override.m_red / 255,
                    a_track_color.m_green * self.m_color_override.m_green / 255,
                    a_track_color.m_blue * self.m_color_override.m_blue / 255,
                );
                a_c.m_alpha = a_track_color.m_alpha * self.m_color_override.m_alpha / 255;
                a_c
            };
            // C++: aImageAlpha = ClampInt(FloatRoundToInt(aTransform.mAlpha * aColor.mAlpha), 0, 255)
            let a_image_alpha = crate::sexy_tod_lib::tod_common::clamp_int(crate::sexy_tod_lib::tod_common::float_round_to_int(a_transform.m_alpha * a_color.m_alpha as f32), 0, 255);
            if a_image_alpha <= 0 {
                return false;
            }
            a_color.m_alpha = a_image_alpha;
            if a_transform.m_image.is_null() {
                return false;
            }

            // [TODO]: 额外叠加色/覆盖色（mEnableExtraAdditiveDraw/mEnableExtraOverlayDraw）
            // [TODO]: 图集（mReanimAtlas）与裁剪

            // C++: ReanimBltMatrix — 用矩阵绘制图像（倾斜/缩放）
            let mut a_matrix = crate::sexy_app_framework::misc::sexy_matrix::SexyMatrix3::new();
            Self::matrix_from_transform(&a_transform, &mut a_matrix);

            let a_image = &*a_transform.m_image;
            let a_img_w = a_image.m_width;
            let a_img_h = a_image.m_height;
            // C++: 绘制 dest rect（简化：忽略倾斜，用 trans+scale）
            let a_dest_x = (a_transform.m_trans_x) as i32;
            let a_dest_y = (a_transform.m_trans_y) as i32;
            let a_dest_w = (a_img_w as f32 * a_transform.m_scale_x) as i32;
            let a_dest_h = (a_img_h as f32 * a_transform.m_scale_y) as i32;
            g.DrawImageDestSrc(a_image, &crate::sexy_app_framework::misc::rect::Rect::new(a_dest_x, a_dest_y, a_dest_w, a_dest_h), &crate::sexy_app_framework::misc::rect::Rect::new(0, 0, a_img_w, a_img_h));
            let _ = a_matrix;
        }
        true
    }

    /// C++ Reanimation::DrawRenderGroup (Reanimator.cpp:919) — 绘制渲染组轨道
    /// [TRANSLATION_NOTE]: mTrackInstances 未翻译（render group 过滤/附件），
    /// 简化：绘制所有轨道
    pub fn draw_render_group(&self, g: &mut Graphics, _render_group: i32) {
        if self.m_dead {
            return;
        }
        unsafe {
            if self.m_definition.is_null() {
                return;
            }
            let a_track_count = (*self.m_definition).m_tracks.count as i32;
            let mut a_track_index = 0;
            while a_track_index < a_track_count {
                // C++: aTrackInstance->mRenderGroup == theRenderGroup 过滤
                let a_render_group = self.m_track_instances.get(a_track_index as usize).map_or(0, |t| t.m_render_group);
                if a_render_group == _render_group {
                    self.draw_track(g, a_track_index);
                    // [TODO]: AttachmentDraw（附件动画）
                }
                a_track_index += 1;
            }
        }
    }

    /// C++ Reanimation::Draw (Reanimator.cpp:941)
    pub fn reanimation_draw(&self, g: &mut Graphics) {
        self.draw_render_group(g, 0 /* RENDER_GROUP_NORMAL */);
    }

    pub fn reanimation_set_position(&mut self, _x: f32, _y: f32) {
        self.m_x = _x; self.m_y = _y;
    }

    pub fn reanimation_set_scale(&mut self, _scale: f32) { self.m_scale = _scale; }

    pub fn reanimation_get_frames(&self) -> f32 {
        // [TODO]: Calculate total frames from definition
        0.0
    }

    /// C++ Reanimation::GetFramesForLayer (Reanimator.cpp:1021) — 层的帧范围
    pub fn get_frames_for_layer(&self, the_track_name: &str, the_frame_start: &mut i32, the_frame_count: &mut i32) {
        unsafe {
            if self.m_definition.is_null() || (*self.m_definition).m_tracks.count == 0 {
                *the_frame_start = 0;
                *the_frame_count = 0;
                return;
            }
            let a_track_index = self.find_track_index(the_track_name);
            let a_def = &*self.m_definition;
            let a_track = &*a_def.m_tracks.tracks.add(a_track_index as usize);
            *the_frame_start = 0;
            *the_frame_count = 1;
            let mut i = 0;
            while i < a_track.m_transforms.count as usize {
                if (*a_track.m_transforms.m_transforms.add(i)).m_frame >= 0.0 {
                    *the_frame_start = i as i32;
                    break;
                }
                i += 1;
            }
            let mut j = *the_frame_start;
            while (j as usize) < a_track.m_transforms.count as usize {
                if (*a_track.m_transforms.m_transforms.add(j as usize)).m_frame >= 0.0 {
                    *the_frame_count = j - *the_frame_start + 1;
                }
                j += 1;
            }
        }
    }

    /// C++ Reanimation::SetFramesForLayer (Reanimator.cpp:1046) — 将动画设置为特定层的帧
    pub fn set_frames_for_layer(&mut self, the_layer: &str) {
        // C++: 正向动画从 0 开始，反向从 0.9999999 开始
        if self.m_anim_rate >= 0.0 {
            self.m_anim_time = 0.0;
        } else {
            self.m_anim_time = 0.9999999;
        }
        let mut a_frame_start = 0;
        let mut a_frame_count = 0;
        self.get_frames_for_layer(the_layer, &mut a_frame_start, &mut a_frame_count);
        self.m_frame_start = a_frame_start;
        self.m_frame_count = a_frame_count;
    }

    /// C++ Reanimation::TrackExists (Reanimator.cpp:1056) — 检查轨道是否存在
    pub fn track_exists(&self, the_track_name: &str) -> bool {
        unsafe {
            if self.m_definition.is_null() {
                return false;
            }
            let a_def = &*self.m_definition;
            let mut a_track_index = 0;
            while a_track_index < a_def.m_tracks.count as usize {
                let a_track = &*a_def.m_tracks.tracks.add(a_track_index);
                if a_track.m_name.eq_ignore_ascii_case(the_track_name) {
                    return true;
                }
                a_track_index += 1;
            }
        }
        false
    }

    /// C++ Reanimation::FindTrackIndex (Reanimator.cpp:947) — 查找轨道索引
    pub fn find_track_index(&self, the_track_name: &str) -> i32 {
        unsafe {
            if self.m_definition.is_null() {
                return 0;
            }
            let a_def = &*self.m_definition;
            let mut a_track_index = 0;
            while a_track_index < a_def.m_tracks.count as usize {
                let a_track = &*a_def.m_tracks.tracks.add(a_track_index);
                if a_track.m_name.eq_ignore_ascii_case(the_track_name) {
                    return a_track_index as i32;
                }
                a_track_index += 1;
            }
        }
        0
    }

    /// C++ Reanimation::Draw (Reanimator.cpp:941) — 绘制动画（RENDER_GROUP_NORMAL）
    pub fn draw(&self, g: &mut Graphics) {
        self.draw_render_group(g, 0);
    }

    /// C++ Reanimation::GetCurrentTransform (Reanimator.cpp:546)
    pub fn get_current_transform(&self, the_track_index: i32, the_transform_current: &mut ReanimatorTransform) {
        let mut a_frame_time = ReanimatorFrameTime { m_anim_frame_before_int: 0, m_anim_frame_after_int: 0, m_fraction: 0.0 };
        self.get_frame_time(&mut a_frame_time);
        self.get_transform_at_time(the_track_index, the_transform_current, &a_frame_time);
        // [TODO]: 轨道混合（mTrackInstances blend）——TrackInstance 未翻译，暂不混合
    }

    /// C++ Reanimation::GetTransformAtTime (Reanimator.cpp:560) — 两帧间插值
    pub fn get_transform_at_time(&self, the_track_index: i32, the_transform: &mut ReanimatorTransform, the_frame_time: &ReanimatorFrameTime) {
        unsafe {
            if self.m_definition.is_null() {
                return;
            }
            let a_definition = &*self.m_definition;
            if (the_track_index as usize) >= a_definition.m_tracks.count as usize {
                return;
            }
            let a_track = &*a_definition.m_tracks.tracks.add(the_track_index as usize);
            let a_trans_count = a_track.m_transforms.count as usize;
            let a_before = the_frame_time.m_anim_frame_before_int as usize;
            let a_after = the_frame_time.m_anim_frame_after_int as usize;
            if a_before >= a_trans_count || a_after >= a_trans_count {
                return;
            }
            let a_trans_before = &*a_track.m_transforms.m_transforms.add(a_before);
            let a_trans_after = &*a_track.m_transforms.m_transforms.add(a_after);
            let a_frac = the_frame_time.m_fraction;

            the_transform.m_trans_x = crate::sexy_tod_lib::tod_common::float_lerp(a_trans_before.m_trans_x, a_trans_after.m_trans_x, a_frac);
            the_transform.m_trans_y = crate::sexy_tod_lib::tod_common::float_lerp(a_trans_before.m_trans_y, a_trans_after.m_trans_y, a_frac);
            the_transform.m_skew_x = crate::sexy_tod_lib::tod_common::float_lerp(a_trans_before.m_skew_x, a_trans_after.m_skew_x, a_frac);
            the_transform.m_skew_y = crate::sexy_tod_lib::tod_common::float_lerp(a_trans_before.m_skew_y, a_trans_after.m_skew_y, a_frac);
            the_transform.m_scale_x = crate::sexy_tod_lib::tod_common::float_lerp(a_trans_before.m_scale_x, a_trans_after.m_scale_x, a_frac);
            the_transform.m_scale_y = crate::sexy_tod_lib::tod_common::float_lerp(a_trans_before.m_scale_y, a_trans_after.m_scale_y, a_frac);
            the_transform.m_alpha = crate::sexy_tod_lib::tod_common::float_lerp(a_trans_before.m_alpha, a_trans_after.m_alpha, a_frac);
            the_transform.m_image = a_trans_before.m_image;
            the_transform.m_font = a_trans_before.m_font;

            // C++: 非空白帧→空白帧过渡截断（mTruncateDisappearingFrames TODO）
            the_transform.m_frame = a_trans_before.m_frame;
        }
    }

    /// C++ Reanimation::GetFrameTime (Reanimator.cpp:896)
    pub fn get_frame_time(&self, the_frame_time: &mut ReanimatorFrameTime) {
        {
            let mut a_frame_count = self.m_frame_count;
            if self.m_loop_type != ReanimLoopType::REANIM_PLAY_ONCE_FULL_LAST_FRAME
                && self.m_loop_type != ReanimLoopType::REANIM_LOOP_FULL_LAST_FRAME
                && self.m_loop_type != ReanimLoopType::REANIM_PLAY_ONCE_FULL_LAST_FRAME_AND_HOLD
            {
                a_frame_count = self.m_frame_count - 1;
            }
            let a_anim_position = self.m_frame_start as f32 + self.m_anim_time * a_frame_count as f32;
            let a_anim_frame_before = a_anim_position.floor();
            the_frame_time.m_fraction = a_anim_position - a_anim_frame_before;
            the_frame_time.m_anim_frame_before_int = crate::sexy_tod_lib::tod_common::float_round_to_int(a_anim_frame_before);
            if the_frame_time.m_anim_frame_before_int >= self.m_frame_start + self.m_frame_count - 1 {
                the_frame_time.m_anim_frame_before_int = self.m_frame_start + self.m_frame_count - 1;
                the_frame_time.m_anim_frame_after_int = the_frame_time.m_anim_frame_before_int;
            } else {
                the_frame_time.m_anim_frame_after_int = the_frame_time.m_anim_frame_before_int + 1;
            }
        }
    }

    /// C++ Reanimation::MatrixFromTransform (Reanimator.cpp:585) — 变换转矩阵
    pub fn matrix_from_transform(the_transform: &ReanimatorTransform, the_matrix: &mut crate::sexy_app_framework::misc::sexy_matrix::SexyMatrix3) {
        // C++: 倾斜角度转弧度
        let a_skew_x = -(the_transform.m_skew_x).to_radians();
        let a_skew_y = -(the_transform.m_skew_y).to_radians();

        the_matrix.m[0][0] = a_skew_x.cos() * the_transform.m_scale_x;
        the_matrix.m[1][0] = -a_skew_x.sin() * the_transform.m_scale_x;
        the_matrix.m[2][0] = 0.0;
        the_matrix.m[0][1] = a_skew_y.sin() * the_transform.m_scale_y;
        the_matrix.m[1][1] = a_skew_y.cos() * the_transform.m_scale_y;
        the_matrix.m[2][1] = 0.0;
        the_matrix.m[0][2] = the_transform.m_trans_x;
        the_matrix.m[1][2] = the_transform.m_trans_y;
        the_matrix.m[2][2] = 1.0;
    }

    /// C++ Reanimation::AssignRenderGroupToTrack — 将轨道分配到渲染组
    pub fn assign_render_group_to_track(&mut self, _the_track_name: &str, _the_render_group: i32) {
        // [TODO]: Set render group override for this track
    }

    /// C++ Reanimation::AssignRenderGroupToPrefix — 按前缀分配渲染组
    pub fn assign_render_group_to_prefix(&mut self, _the_prefix: &str, _the_render_group: i32) {
        // [TODO]: Find all tracks with matching prefix, assign group
    }

    /// C++ Reanimation::SetImageOverride — 设置轨道的图像覆盖
    pub fn set_image_override(&mut self, _the_track_name: &str, _the_image: *mut Image) {
        // [TODO]: Set image override for this track
    }

    /// C++ Reanimation::GetImageOverride — 获取轨道的图像覆盖
    pub fn get_image_override(&self, _the_track_name: &str) -> *mut Image {
        std::ptr::null_mut()
    }

    /// C++ Reanimation::AttachToAnotherReanimation — 附加到父动画
    pub fn attach_to_another_reanimation(&mut self, _the_parent: *mut Reanimation, _the_parent_track: &str) {
        // [TODO]: Set parent reference and track name
    }

    /// C++ Reanimation::SetTruncateDisappearingFrames — 设置截断消逝帧
    pub fn set_truncate_disappearing_frames(&mut self, _the_track_name: Option<&str>, _the_truncate: bool) {
        // [TODO]: Mark specified track for truncation
    }

    /// C++ Reanimation::IsAnimPlaying — 检查动画是否正在播放
    pub fn is_anim_playing(&self, _the_track_name: &str) -> bool {
        // [TODO]: Check if track animation hasn't ended
        true
    }

    /// C++ Reanimation::ReanimShowPrefix (辅助函数) — 显示指定前缀的轨道
    pub fn reanim_show_prefix(&mut self, _the_prefix: &str, _the_render_group: i32) {
        self.assign_render_group_to_prefix(_the_prefix, _the_render_group);
    }

    /// C++ Reanimation::ReanimShowTrack — 显示指定名称的轨道
    pub fn reanim_show_track(&mut self, _the_track_name: &str, _the_render_group: i32) {
        self.assign_render_group_to_track(_the_track_name, _the_render_group);
    }

    /// 获取动画时间（秒）
    pub fn get_anim_time(&self) -> f32 { self.m_anim_time }
}

impl Default for Reanimation {
    fn default() -> Self {
        Reanimation::new()
    }
}

// ============================================================
// ReanimationHolder
// ============================================================
pub struct ReanimationHolder {
    pub m_animations: DataArray<Reanimation>,
}

impl ReanimationHolder {
    pub fn new() -> Self { ReanimationHolder { m_animations: DataArray::new() } }
    pub fn initialize_holder(&mut self) { self.m_animations.data_array_initialize(64u32, "ReanimationHolder"); }
    pub fn dispose_holder(&mut self) { unsafe { self.m_animations.data_array_dispose(); } }
    pub fn alloc_reanimation(&mut self, _the_render_order: i32, _the_reanim_type: ReanimationType, _the_definition: *mut ReanimatorDefinition) -> *mut Reanimation {
        unsafe { self.m_animations.data_array_alloc() }
    }
    pub fn process_delete_queue(&mut self) {
        // 删除标记为 dead 的动画
    }
    pub fn update(&mut self) {
        // C++ ReanimationHolder::Update — 遍历动画并更新
        unsafe {
            let a_size = self.m_animations.m_size as usize;
            if self.m_animations.m_block.is_null() {
                return;
            }
            let mut i = 0;
            while i < a_size {
                let a_item = &mut *self.m_animations.m_block.add(i);
                if a_item.m_id != 0 {
                    let a_reanim = &mut a_item.m_item;
                    if !a_reanim.m_dead {
                        a_reanim.reanimation_update();
                    }
                }
                i += 1;
            }
        }
    }
    pub fn find_reanimation(&self, _the_reanim_type: ReanimationType) -> *mut Reanimation {
        std::ptr::null_mut()
    }
}

// ============================================================
// 全局函数（桩）
// ============================================================

/// C++ ReanimationParams (Reanimator.h:104)
pub struct ReanimationParams {
    pub m_reanimation_type: ReanimationType,
    pub m_reanim_file_name: &'static str,
    pub m_reanim_param_flags: u32,
}

/// C++ gLawnReanimationArray (Reanimator.cpp:40) — 144 项动画参数表
pub static G_LAWN_REANIMATION_ARRAY: [ReanimationParams; 144] = [
    ReanimationParams { m_reanimation_type: ReanimationType::REANIM_LOADBAR_SPROUT, m_reanim_file_name: "reanim/LoadBar_sprout.reanim", m_reanim_param_flags: 1 },
    ReanimationParams { m_reanimation_type: ReanimationType::REANIM_LOADBAR_ZOMBIEHEAD, m_reanim_file_name: "reanim/LoadBar_Zombiehead.reanim", m_reanim_param_flags: 1 },
    ReanimationParams { m_reanimation_type: ReanimationType::REANIM_SODROLL, m_reanim_file_name: "reanim/SodRoll.reanim", m_reanim_param_flags: 0 },
    ReanimationParams { m_reanimation_type: ReanimationType::REANIM_FINAL_WAVE, m_reanim_file_name: "reanim/FinalWave.reanim", m_reanim_param_flags: 1 },
    ReanimationParams { m_reanimation_type: ReanimationType::REANIM_PEASHOOTER, m_reanim_file_name: "reanim/PeaShooterSingle.reanim", m_reanim_param_flags: 0 },
    ReanimationParams { m_reanimation_type: ReanimationType::REANIM_WALLNUT, m_reanim_file_name: "reanim/Wallnut.reanim", m_reanim_param_flags: 0 },
    ReanimationParams { m_reanimation_type: ReanimationType::REANIM_LILYPAD, m_reanim_file_name: "reanim/Lilypad.reanim", m_reanim_param_flags: 0 },
    ReanimationParams { m_reanimation_type: ReanimationType::REANIM_SUNFLOWER, m_reanim_file_name: "reanim/SunFlower.reanim", m_reanim_param_flags: 0 },
    ReanimationParams { m_reanimation_type: ReanimationType::REANIM_LAWNMOWER, m_reanim_file_name: "reanim/LawnMower.reanim", m_reanim_param_flags: 0 },
    ReanimationParams { m_reanimation_type: ReanimationType::REANIM_READYSETPLANT, m_reanim_file_name: "reanim/StartReadySetPlant.reanim", m_reanim_param_flags: 1 },
    ReanimationParams { m_reanimation_type: ReanimationType::REANIM_CHERRYBOMB, m_reanim_file_name: "reanim/CherryBomb.reanim", m_reanim_param_flags: 0 },
    ReanimationParams { m_reanimation_type: ReanimationType::REANIM_SQUASH, m_reanim_file_name: "reanim/Squash.reanim", m_reanim_param_flags: 0 },
    ReanimationParams { m_reanimation_type: ReanimationType::REANIM_DOOMSHROOM, m_reanim_file_name: "reanim/DoomShroom.reanim", m_reanim_param_flags: 0 },
    ReanimationParams { m_reanimation_type: ReanimationType::REANIM_SNOWPEA, m_reanim_file_name: "reanim/SnowPea.reanim", m_reanim_param_flags: 0 },
    ReanimationParams { m_reanimation_type: ReanimationType::REANIM_REPEATER, m_reanim_file_name: "reanim/PeaShooter.reanim", m_reanim_param_flags: 0 },
    ReanimationParams { m_reanimation_type: ReanimationType::REANIM_SUNSHROOM, m_reanim_file_name: "reanim/SunShroom.reanim", m_reanim_param_flags: 0 },
    ReanimationParams { m_reanimation_type: ReanimationType::REANIM_TALLNUT, m_reanim_file_name: "reanim/Tallnut.reanim", m_reanim_param_flags: 0 },
    ReanimationParams { m_reanimation_type: ReanimationType::REANIM_FUMESHROOM, m_reanim_file_name: "reanim/Fumeshroom.reanim", m_reanim_param_flags: 0 },
    ReanimationParams { m_reanimation_type: ReanimationType::REANIM_PUFFSHROOM, m_reanim_file_name: "reanim/Puffshroom.reanim", m_reanim_param_flags: 0 },
    ReanimationParams { m_reanimation_type: ReanimationType::REANIM_HYPNOSHROOM, m_reanim_file_name: "reanim/Hypnoshroom.reanim", m_reanim_param_flags: 0 },
    ReanimationParams { m_reanimation_type: ReanimationType::REANIM_CHOMPER, m_reanim_file_name: "reanim/Chomper.reanim", m_reanim_param_flags: 0 },
    ReanimationParams { m_reanimation_type: ReanimationType::REANIM_ZOMBIE, m_reanim_file_name: "reanim/Zombie.reanim", m_reanim_param_flags: 0 },
    ReanimationParams { m_reanimation_type: ReanimationType::REANIM_SUN, m_reanim_file_name: "reanim/Sun.reanim", m_reanim_param_flags: 0 },
    ReanimationParams { m_reanimation_type: ReanimationType::REANIM_POTATOMINE, m_reanim_file_name: "reanim/PotatoMine.reanim", m_reanim_param_flags: 0 },
    ReanimationParams { m_reanimation_type: ReanimationType::REANIM_SPIKEWEED, m_reanim_file_name: "reanim/Caltrop.reanim", m_reanim_param_flags: 0 },
    ReanimationParams { m_reanimation_type: ReanimationType::REANIM_SPIKEROCK, m_reanim_file_name: "reanim/SpikeRock.reanim", m_reanim_param_flags: 0 },
    ReanimationParams { m_reanimation_type: ReanimationType::REANIM_THREEPEATER, m_reanim_file_name: "reanim/ThreePeater.reanim", m_reanim_param_flags: 0 },
    ReanimationParams { m_reanimation_type: ReanimationType::REANIM_MARIGOLD, m_reanim_file_name: "reanim/Marigold.reanim", m_reanim_param_flags: 0 },
    ReanimationParams { m_reanimation_type: ReanimationType::REANIM_ICESHROOM, m_reanim_file_name: "reanim/IceShroom.reanim", m_reanim_param_flags: 0 },
    ReanimationParams { m_reanimation_type: ReanimationType::REANIM_ZOMBIE_FOOTBALL, m_reanim_file_name: "reanim/Zombie_football.reanim", m_reanim_param_flags: 0 },
    ReanimationParams { m_reanimation_type: ReanimationType::REANIM_ZOMBIE_NEWSPAPER, m_reanim_file_name: "reanim/Zombie_paper.reanim", m_reanim_param_flags: 0 },
    ReanimationParams { m_reanimation_type: ReanimationType::REANIM_ZOMBIE_ZAMBONI, m_reanim_file_name: "reanim/Zombie_zamboni.reanim", m_reanim_param_flags: 0 },
    ReanimationParams { m_reanimation_type: ReanimationType::REANIM_SPLASH, m_reanim_file_name: "reanim/splash.reanim", m_reanim_param_flags: 0 },
    ReanimationParams { m_reanimation_type: ReanimationType::REANIM_JALAPENO, m_reanim_file_name: "reanim/Jalapeno.reanim", m_reanim_param_flags: 0 },
    ReanimationParams { m_reanimation_type: ReanimationType::REANIM_JALAPENO_FIRE, m_reanim_file_name: "reanim/fire.reanim", m_reanim_param_flags: 0 },
    ReanimationParams { m_reanimation_type: ReanimationType::REANIM_COIN_SILVER, m_reanim_file_name: "reanim/Coin_silver.reanim", m_reanim_param_flags: 0 },
    ReanimationParams { m_reanimation_type: ReanimationType::REANIM_ZOMBIE_CHARRED, m_reanim_file_name: "reanim/Zombie_charred.reanim", m_reanim_param_flags: 0 },
    ReanimationParams { m_reanimation_type: ReanimationType::REANIM_ZOMBIE_CHARRED_IMP, m_reanim_file_name: "reanim/Zombie_charred_imp.reanim", m_reanim_param_flags: 0 },
    ReanimationParams { m_reanimation_type: ReanimationType::REANIM_ZOMBIE_CHARRED_DIGGER, m_reanim_file_name: "reanim/Zombie_charred_digger.reanim", m_reanim_param_flags: 0 },
    ReanimationParams { m_reanimation_type: ReanimationType::REANIM_ZOMBIE_CHARRED_ZAMBONI, m_reanim_file_name: "reanim/Zombie_charred_zamboni.reanim", m_reanim_param_flags: 0 },
    ReanimationParams { m_reanimation_type: ReanimationType::REANIM_ZOMBIE_CHARRED_CATAPULT, m_reanim_file_name: "reanim/Zombie_charred_catapult.reanim", m_reanim_param_flags: 0 },
    ReanimationParams { m_reanimation_type: ReanimationType::REANIM_ZOMBIE_CHARRED_GARGANTUAR, m_reanim_file_name: "reanim/Zombie_charred_gargantuar.reanim", m_reanim_param_flags: 0 },
    ReanimationParams { m_reanimation_type: ReanimationType::REANIM_SCRAREYSHROOM, m_reanim_file_name: "reanim/ScaredyShroom.reanim", m_reanim_param_flags: 0 },
    ReanimationParams { m_reanimation_type: ReanimationType::REANIM_PUMPKIN, m_reanim_file_name: "reanim/Pumpkin.reanim", m_reanim_param_flags: 0 },
    ReanimationParams { m_reanimation_type: ReanimationType::REANIM_PLANTERN, m_reanim_file_name: "reanim/Plantern.reanim", m_reanim_param_flags: 0 },
    ReanimationParams { m_reanimation_type: ReanimationType::REANIM_TORCHWOOD, m_reanim_file_name: "reanim/Torchwood.reanim", m_reanim_param_flags: 0 },
    ReanimationParams { m_reanimation_type: ReanimationType::REANIM_SPLITPEA, m_reanim_file_name: "reanim/SplitPea.reanim", m_reanim_param_flags: 0 },
    ReanimationParams { m_reanimation_type: ReanimationType::REANIM_SEASHROOM, m_reanim_file_name: "reanim/SeaShroom.reanim", m_reanim_param_flags: 0 },
    ReanimationParams { m_reanimation_type: ReanimationType::REANIM_BLOVER, m_reanim_file_name: "reanim/Blover.reanim", m_reanim_param_flags: 0 },
    ReanimationParams { m_reanimation_type: ReanimationType::REANIM_FLOWER_POT, m_reanim_file_name: "reanim/Pot.reanim", m_reanim_param_flags: 0 },
    ReanimationParams { m_reanimation_type: ReanimationType::REANIM_CACTUS, m_reanim_file_name: "reanim/Cactus.reanim", m_reanim_param_flags: 0 },
    ReanimationParams { m_reanimation_type: ReanimationType::REANIM_DANCER, m_reanim_file_name: "reanim/Zombie_disco.reanim", m_reanim_param_flags: 0 },
    ReanimationParams { m_reanimation_type: ReanimationType::REANIM_TANGLEKELP, m_reanim_file_name: "reanim/Tanglekelp.reanim", m_reanim_param_flags: 0 },
    ReanimationParams { m_reanimation_type: ReanimationType::REANIM_STARFRUIT, m_reanim_file_name: "reanim/Starfruit.reanim", m_reanim_param_flags: 0 },
    ReanimationParams { m_reanimation_type: ReanimationType::REANIM_POLEVAULTER, m_reanim_file_name: "reanim/Zombie_polevaulter.reanim", m_reanim_param_flags: 0 },
    ReanimationParams { m_reanimation_type: ReanimationType::REANIM_BALLOON, m_reanim_file_name: "reanim/Zombie_balloon.reanim", m_reanim_param_flags: 0 },
    ReanimationParams { m_reanimation_type: ReanimationType::REANIM_GARGANTUAR, m_reanim_file_name: "reanim/Zombie_gargantuar.reanim", m_reanim_param_flags: 0 },
    ReanimationParams { m_reanimation_type: ReanimationType::REANIM_IMP, m_reanim_file_name: "reanim/Zombie_imp.reanim", m_reanim_param_flags: 0 },
    ReanimationParams { m_reanimation_type: ReanimationType::REANIM_DIGGER, m_reanim_file_name: "reanim/Zombie_digger.reanim", m_reanim_param_flags: 0 },
    ReanimationParams { m_reanimation_type: ReanimationType::REANIM_DIGGER_DIRT, m_reanim_file_name: "reanim/Digger_rising_dirt.reanim", m_reanim_param_flags: 0 },
    ReanimationParams { m_reanimation_type: ReanimationType::REANIM_ZOMBIE_DOLPHINRIDER, m_reanim_file_name: "reanim/Zombie_dolphinrider.reanim", m_reanim_param_flags: 0 },
    ReanimationParams { m_reanimation_type: ReanimationType::REANIM_POGO, m_reanim_file_name: "reanim/Zombie_pogo.reanim", m_reanim_param_flags: 0 },
    ReanimationParams { m_reanimation_type: ReanimationType::REANIM_BACKUP_DANCER, m_reanim_file_name: "reanim/Zombie_backup.reanim", m_reanim_param_flags: 0 },
    ReanimationParams { m_reanimation_type: ReanimationType::REANIM_BOBSLED, m_reanim_file_name: "reanim/Zombie_bobsled.reanim", m_reanim_param_flags: 0 },
    ReanimationParams { m_reanimation_type: ReanimationType::REANIM_JACKINTHEBOX, m_reanim_file_name: "reanim/Zombie_jackbox.reanim", m_reanim_param_flags: 0 },
    ReanimationParams { m_reanimation_type: ReanimationType::REANIM_SNORKEL, m_reanim_file_name: "reanim/Zombie_snorkle.reanim", m_reanim_param_flags: 0 },
    ReanimationParams { m_reanimation_type: ReanimationType::REANIM_BUNGEE, m_reanim_file_name: "reanim/Zombie_bungi.reanim", m_reanim_param_flags: 0 },
    ReanimationParams { m_reanimation_type: ReanimationType::REANIM_CATAPULT, m_reanim_file_name: "reanim/Zombie_catapult.reanim", m_reanim_param_flags: 0 },
    ReanimationParams { m_reanimation_type: ReanimationType::REANIM_LADDER, m_reanim_file_name: "reanim/Zombie_ladder.reanim", m_reanim_param_flags: 0 },
    ReanimationParams { m_reanimation_type: ReanimationType::REANIM_PUFF, m_reanim_file_name: "reanim/Puff.reanim", m_reanim_param_flags: 0 },
    ReanimationParams { m_reanimation_type: ReanimationType::REANIM_SLEEPING, m_reanim_file_name: "reanim/Z.reanim", m_reanim_param_flags: 0 },
    ReanimationParams { m_reanimation_type: ReanimationType::REANIM_GRAVE_BUSTER, m_reanim_file_name: "reanim/Gravebuster.reanim", m_reanim_param_flags: 0 },
    ReanimationParams { m_reanimation_type: ReanimationType::REANIM_ZOMBIES_WON, m_reanim_file_name: "reanim/ZombiesWon.reanim", m_reanim_param_flags: 1 },
    ReanimationParams { m_reanimation_type: ReanimationType::REANIM_MAGNETSHROOM, m_reanim_file_name: "reanim/Magnetshroom.reanim", m_reanim_param_flags: 0 },
    ReanimationParams { m_reanimation_type: ReanimationType::REANIM_BOSS, m_reanim_file_name: "reanim/Zombie_boss.reanim", m_reanim_param_flags: 0 },
    ReanimationParams { m_reanimation_type: ReanimationType::REANIM_CABBAGEPULT, m_reanim_file_name: "reanim/Cabbagepult.reanim", m_reanim_param_flags: 0 },
    ReanimationParams { m_reanimation_type: ReanimationType::REANIM_KERNELPULT, m_reanim_file_name: "reanim/Cornpult.reanim", m_reanim_param_flags: 0 },
    ReanimationParams { m_reanimation_type: ReanimationType::REANIM_MELONPULT, m_reanim_file_name: "reanim/Melonpult.reanim", m_reanim_param_flags: 0 },
    ReanimationParams { m_reanimation_type: ReanimationType::REANIM_COFFEEBEAN, m_reanim_file_name: "reanim/Coffeebean.reanim", m_reanim_param_flags: 1 },
    ReanimationParams { m_reanimation_type: ReanimationType::REANIM_UMBRELLALEAF, m_reanim_file_name: "reanim/Umbrellaleaf.reanim", m_reanim_param_flags: 0 },
    ReanimationParams { m_reanimation_type: ReanimationType::REANIM_GATLINGPEA, m_reanim_file_name: "reanim/GatlingPea.reanim", m_reanim_param_flags: 0 },
    ReanimationParams { m_reanimation_type: ReanimationType::REANIM_CATTAIL, m_reanim_file_name: "reanim/Cattail.reanim", m_reanim_param_flags: 0 },
    ReanimationParams { m_reanimation_type: ReanimationType::REANIM_GLOOMSHROOM, m_reanim_file_name: "reanim/GloomShroom.reanim", m_reanim_param_flags: 0 },
    ReanimationParams { m_reanimation_type: ReanimationType::REANIM_BOSS_ICEBALL, m_reanim_file_name: "reanim/Zombie_boss_iceball.reanim", m_reanim_param_flags: 1 },
    ReanimationParams { m_reanimation_type: ReanimationType::REANIM_BOSS_FIREBALL, m_reanim_file_name: "reanim/Zombie_boss_fireball.reanim", m_reanim_param_flags: 1 },
    ReanimationParams { m_reanimation_type: ReanimationType::REANIM_COBCANNON, m_reanim_file_name: "reanim/CobCannon.reanim", m_reanim_param_flags: 0 },
    ReanimationParams { m_reanimation_type: ReanimationType::REANIM_GARLIC, m_reanim_file_name: "reanim/Garlic.reanim", m_reanim_param_flags: 0 },
    ReanimationParams { m_reanimation_type: ReanimationType::REANIM_GOLD_MAGNET, m_reanim_file_name: "reanim/GoldMagnet.reanim", m_reanim_param_flags: 0 },
    ReanimationParams { m_reanimation_type: ReanimationType::REANIM_WINTER_MELON, m_reanim_file_name: "reanim/WinterMelon.reanim", m_reanim_param_flags: 0 },
    ReanimationParams { m_reanimation_type: ReanimationType::REANIM_TWIN_SUNFLOWER, m_reanim_file_name: "reanim/TwinSunflower.reanim", m_reanim_param_flags: 0 },
    ReanimationParams { m_reanimation_type: ReanimationType::REANIM_POOL_CLEANER, m_reanim_file_name: "reanim/PoolCleaner.reanim", m_reanim_param_flags: 0 },
    ReanimationParams { m_reanimation_type: ReanimationType::REANIM_ROOF_CLEANER, m_reanim_file_name: "reanim/RoofCleaner.reanim", m_reanim_param_flags: 0 },
    ReanimationParams { m_reanimation_type: ReanimationType::REANIM_FIRE_PEA, m_reanim_file_name: "reanim/FirePea.reanim", m_reanim_param_flags: 0 },
    ReanimationParams { m_reanimation_type: ReanimationType::REANIM_IMITATER, m_reanim_file_name: "reanim/Imitater.reanim", m_reanim_param_flags: 0 },
    ReanimationParams { m_reanimation_type: ReanimationType::REANIM_YETI, m_reanim_file_name: "reanim/Zombie_yeti.reanim", m_reanim_param_flags: 0 },
    ReanimationParams { m_reanimation_type: ReanimationType::REANIM_BOSS_DRIVER, m_reanim_file_name: "reanim/Zombie_Boss_driver.reanim", m_reanim_param_flags: 0 },
    ReanimationParams { m_reanimation_type: ReanimationType::REANIM_LAWN_MOWERED_ZOMBIE, m_reanim_file_name: "reanim/LawnMoweredZombie.reanim", m_reanim_param_flags: 0 },
    ReanimationParams { m_reanimation_type: ReanimationType::REANIM_CRAZY_DAVE, m_reanim_file_name: "reanim/CrazyDave.reanim", m_reanim_param_flags: 1 },
    ReanimationParams { m_reanimation_type: ReanimationType::REANIM_TEXT_FADE_ON, m_reanim_file_name: "reanim/TextFadeOn.reanim", m_reanim_param_flags: 0 },
    ReanimationParams { m_reanimation_type: ReanimationType::REANIM_HAMMER, m_reanim_file_name: "reanim/Hammer.reanim", m_reanim_param_flags: 0 },
    ReanimationParams { m_reanimation_type: ReanimationType::REANIM_SLOT_MACHINE_HANDLE, m_reanim_file_name: "reanim/SlotMachine.reanim", m_reanim_param_flags: 0 },
    ReanimationParams { m_reanimation_type: ReanimationType::REANIM_CREDITS_FOOTBALL, m_reanim_file_name: "reanim/Credits_Football.reanim", m_reanim_param_flags: 1 },
    ReanimationParams { m_reanimation_type: ReanimationType::REANIM_CREDITS_JACKBOX, m_reanim_file_name: "reanim/Credits_Jackbox.reanim", m_reanim_param_flags: 1 },
    ReanimationParams { m_reanimation_type: ReanimationType::REANIM_SELECTOR_SCREEN, m_reanim_file_name: "reanim/SelectorScreen.reanim", m_reanim_param_flags: 3 },
    ReanimationParams { m_reanimation_type: ReanimationType::REANIM_PORTAL_CIRCLE, m_reanim_file_name: "reanim/Portal_Circle.reanim", m_reanim_param_flags: 0 },
    ReanimationParams { m_reanimation_type: ReanimationType::REANIM_PORTAL_SQUARE, m_reanim_file_name: "reanim/Portal_Square.reanim", m_reanim_param_flags: 0 },
    ReanimationParams { m_reanimation_type: ReanimationType::REANIM_ZENGARDEN_SPROUT, m_reanim_file_name: "reanim/ZenGarden_sprout.reanim", m_reanim_param_flags: 0 },
    ReanimationParams { m_reanimation_type: ReanimationType::REANIM_ZENGARDEN_WATERINGCAN, m_reanim_file_name: "reanim/ZenGarden_wateringcan.reanim", m_reanim_param_flags: 1 },
    ReanimationParams { m_reanimation_type: ReanimationType::REANIM_ZENGARDEN_FERTILIZER, m_reanim_file_name: "reanim/ZenGarden_fertilizer.reanim", m_reanim_param_flags: 1 },
    ReanimationParams { m_reanimation_type: ReanimationType::REANIM_ZENGARDEN_BUGSPRAY, m_reanim_file_name: "reanim/ZenGarden_bugspray.reanim", m_reanim_param_flags: 1 },
    ReanimationParams { m_reanimation_type: ReanimationType::REANIM_ZENGARDEN_PHONOGRAPH, m_reanim_file_name: "reanim/ZenGarden_phonograph.reanim", m_reanim_param_flags: 1 },
    ReanimationParams { m_reanimation_type: ReanimationType::REANIM_DIAMOND, m_reanim_file_name: "reanim/Diamond.reanim", m_reanim_param_flags: 0 },
    ReanimationParams { m_reanimation_type: ReanimationType::REANIM_ZOMBIE_HAND, m_reanim_file_name: "reanim/Zombie_hand.reanim", m_reanim_param_flags: 1 },
    ReanimationParams { m_reanimation_type: ReanimationType::REANIM_STINKY, m_reanim_file_name: "reanim/Stinky.reanim", m_reanim_param_flags: 0 },
    ReanimationParams { m_reanimation_type: ReanimationType::REANIM_RAKE, m_reanim_file_name: "reanim/Rake.reanim", m_reanim_param_flags: 0 },
    ReanimationParams { m_reanimation_type: ReanimationType::REANIM_RAIN_CIRCLE, m_reanim_file_name: "reanim/Rain_circle.reanim", m_reanim_param_flags: 0 },
    ReanimationParams { m_reanimation_type: ReanimationType::REANIM_RAIN_SPLASH, m_reanim_file_name: "reanim/Rain_splash.reanim", m_reanim_param_flags: 0 },
    ReanimationParams { m_reanimation_type: ReanimationType::REANIM_ZOMBIE_SURPRISE, m_reanim_file_name: "reanim/Zombie_surprise.reanim", m_reanim_param_flags: 0 },
    ReanimationParams { m_reanimation_type: ReanimationType::REANIM_COIN_GOLD, m_reanim_file_name: "reanim/Coin_gold.reanim", m_reanim_param_flags: 0 },
    ReanimationParams { m_reanimation_type: ReanimationType::REANIM_TREEOFWISDOM, m_reanim_file_name: "reanim/TreeOfWisdom.reanim", m_reanim_param_flags: 1 },
    ReanimationParams { m_reanimation_type: ReanimationType::REANIM_TREEOFWISDOM_CLOUDS, m_reanim_file_name: "reanim/TreeOfWisdomClouds.reanim", m_reanim_param_flags: 1 },
    ReanimationParams { m_reanimation_type: ReanimationType::REANIM_TREEOFWISDOM_TREEFOOD, m_reanim_file_name: "reanim/TreeFood.reanim", m_reanim_param_flags: 1 },
    ReanimationParams { m_reanimation_type: ReanimationType::REANIM_CREDITS_MAIN, m_reanim_file_name: "reanim/Credits_Main.reanim", m_reanim_param_flags: 3 },
    ReanimationParams { m_reanimation_type: ReanimationType::REANIM_CREDITS_MAIN2, m_reanim_file_name: "reanim/Credits_Main2.reanim", m_reanim_param_flags: 3 },
    ReanimationParams { m_reanimation_type: ReanimationType::REANIM_CREDITS_MAIN3, m_reanim_file_name: "reanim/Credits_Main3.reanim", m_reanim_param_flags: 3 },
    ReanimationParams { m_reanimation_type: ReanimationType::REANIM_ZOMBIE_CREDITS_DANCE, m_reanim_file_name: "reanim/Zombie_credits_dance.reanim", m_reanim_param_flags: 0 },
    ReanimationParams { m_reanimation_type: ReanimationType::REANIM_CREDITS_STAGE, m_reanim_file_name: "reanim/Credits_stage.reanim", m_reanim_param_flags: 1 },
    ReanimationParams { m_reanimation_type: ReanimationType::REANIM_CREDITS_BIGBRAIN, m_reanim_file_name: "reanim/Credits_BigBrain.reanim", m_reanim_param_flags: 1 },
    ReanimationParams { m_reanimation_type: ReanimationType::REANIM_CREDITS_FLOWER_PETALS, m_reanim_file_name: "reanim/Credits_Flower_petals.reanim", m_reanim_param_flags: 1 },
    ReanimationParams { m_reanimation_type: ReanimationType::REANIM_CREDITS_INFANTRY, m_reanim_file_name: "reanim/Credits_Infantry.reanim", m_reanim_param_flags: 1 },
    ReanimationParams { m_reanimation_type: ReanimationType::REANIM_CREDITS_THROAT, m_reanim_file_name: "reanim/Credits_Throat.reanim", m_reanim_param_flags: 1 },
    ReanimationParams { m_reanimation_type: ReanimationType::REANIM_CREDITS_CRAZYDAVE, m_reanim_file_name: "reanim/Credits_CrazyDave.reanim", m_reanim_param_flags: 1 },
    ReanimationParams { m_reanimation_type: ReanimationType::REANIM_CREDITS_BOSSDANCE, m_reanim_file_name: "reanim/Credits_Bossdance.reanim", m_reanim_param_flags: 1 },
    ReanimationParams { m_reanimation_type: ReanimationType::REANIM_ZOMBIE_CREDITS_SCREEN_DOOR, m_reanim_file_name: "reanim/Zombie_Credits_Screendoor.reanim", m_reanim_param_flags: 1 },
    ReanimationParams { m_reanimation_type: ReanimationType::REANIM_ZOMBIE_CREDITS_CONEHEAD, m_reanim_file_name: "reanim/Zombie_Credits_Conehead.reanim", m_reanim_param_flags: 1 },
    ReanimationParams { m_reanimation_type: ReanimationType::REANIM_CREDITS_ZOMBIEARMY1, m_reanim_file_name: "reanim/Credits_ZombieArmy1.reanim", m_reanim_param_flags: 1 },
    ReanimationParams { m_reanimation_type: ReanimationType::REANIM_CREDITS_ZOMBIEARMY2, m_reanim_file_name: "reanim/Credits_ZombieArmy2.reanim", m_reanim_param_flags: 1 },
    ReanimationParams { m_reanimation_type: ReanimationType::REANIM_CREDITS_TOMBSTONES, m_reanim_file_name: "reanim/Credits_Tombstones.reanim", m_reanim_param_flags: 1 },
    ReanimationParams { m_reanimation_type: ReanimationType::REANIM_CREDITS_SOLARPOWER, m_reanim_file_name: "reanim/Credits_SolarPower.reanim", m_reanim_param_flags: 1 },
    ReanimationParams { m_reanimation_type: ReanimationType::REANIM_CREDITS_ANYHOUR, m_reanim_file_name: "reanim/Credits_Anyhour.reanim", m_reanim_param_flags: 3 },
    ReanimationParams { m_reanimation_type: ReanimationType::REANIM_CREDITS_WEARETHEUNDEAD, m_reanim_file_name: "reanim/Credits_WeAreTheUndead.reanim", m_reanim_param_flags: 1 },
    ReanimationParams { m_reanimation_type: ReanimationType::REANIM_CREDITS_DISCOLIGHTS, m_reanim_file_name: "reanim/Credits_DiscoLights.reanim", m_reanim_param_flags: 1 },
    ReanimationParams { m_reanimation_type: ReanimationType::REANIM_FLAG, m_reanim_file_name: "reanim/Zombie_FlagPole.reanim", m_reanim_param_flags: 0 },
    ReanimationParams { m_reanimation_type: ReanimationType::REANIM_ZOMBATAR_HEAD, m_reanim_file_name: "reanim/zombatar_zombie_head.reanim", m_reanim_param_flags: 0 },
];
pub static mut G_REANIMATOR_DEF_COUNT: i32 = 0;

/// C++ gReanimatorDefArray — 动画定义数组（按 ReanimationType 索引）
pub static mut G_REANIMATOR_DEF_ARRAY: [*mut ReanimatorDefinition; 144] = [std::ptr::null_mut(); 144];

/// C++ ReanimatorLoadDefinitions (Reanimator.cpp:1195)
pub fn reanimator_load_definitions() {
    // C++: gReanimationParamArraySize = theReanimationParamArraySize;
    //      gReanimatorDefArray = new ReanimatorDefinition[theReanimationParamArraySize];
    // [TODO]: gLawnReanimationArray 参数表（ReanimationLawn.cpp）翻译后接入
    unsafe {
        G_REANIMATOR_DEF_COUNT = 0;
        // C++: for each → ReanimatorEnsureDefinitionLoaded(type, true) 预加载
    }
}

/// C++ ReanimationFillInMissingData (Reanimator.cpp:200) — 用前一帧数据填充未定义数据
pub fn reanimation_fill_in_missing_data_float(the_prev: f32, the_value: &mut f32) {
    if *the_value == 100000.0 /* NO_VALUE */ {
        *the_value = the_prev;
    }
}

pub fn reanimation_fill_in_missing_data_ptr(the_prev: *mut std::ffi::c_void, the_value: &mut *mut std::ffi::c_void) {
    if the_value.is_null() {
        *the_value = the_prev;
    }
}

/// C++ gReanimatorDefMap (Definition.cpp:141-169) — Reanimator 定义字段表
/// [TRANSLATION_NOTE]: 偏移用 std::mem::offset_of!（Rust 结构布局）
pub static G_REANIMATOR_TRANSFORM_DEF_FIELDS: [DefField; 11] = [
    DefField { m_field_name: "x", m_field_offset: std::mem::offset_of!(ReanimatorTransform, m_trans_x) as i32, m_field_type: DefFieldType::DT_FLOAT, m_extra_data: std::ptr::null() },
    DefField { m_field_name: "y", m_field_offset: std::mem::offset_of!(ReanimatorTransform, m_trans_y) as i32, m_field_type: DefFieldType::DT_FLOAT, m_extra_data: std::ptr::null() },
    DefField { m_field_name: "kx", m_field_offset: std::mem::offset_of!(ReanimatorTransform, m_skew_x) as i32, m_field_type: DefFieldType::DT_FLOAT, m_extra_data: std::ptr::null() },
    DefField { m_field_name: "ky", m_field_offset: std::mem::offset_of!(ReanimatorTransform, m_skew_y) as i32, m_field_type: DefFieldType::DT_FLOAT, m_extra_data: std::ptr::null() },
    DefField { m_field_name: "sx", m_field_offset: std::mem::offset_of!(ReanimatorTransform, m_scale_x) as i32, m_field_type: DefFieldType::DT_FLOAT, m_extra_data: std::ptr::null() },
    DefField { m_field_name: "sy", m_field_offset: std::mem::offset_of!(ReanimatorTransform, m_scale_y) as i32, m_field_type: DefFieldType::DT_FLOAT, m_extra_data: std::ptr::null() },
    DefField { m_field_name: "f", m_field_offset: std::mem::offset_of!(ReanimatorTransform, m_frame) as i32, m_field_type: DefFieldType::DT_FLOAT, m_extra_data: std::ptr::null() },
    DefField { m_field_name: "a", m_field_offset: std::mem::offset_of!(ReanimatorTransform, m_alpha) as i32, m_field_type: DefFieldType::DT_FLOAT, m_extra_data: std::ptr::null() },
    DefField { m_field_name: "i", m_field_offset: std::mem::offset_of!(ReanimatorTransform, m_image) as i32, m_field_type: DefFieldType::DT_IMAGE, m_extra_data: std::ptr::null() },
    DefField { m_field_name: "font", m_field_offset: std::mem::offset_of!(ReanimatorTransform, m_font) as i32, m_field_type: DefFieldType::DT_FONT, m_extra_data: std::ptr::null() },
    DefField { m_field_name: "", m_field_offset: 0, m_field_type: DefFieldType::DT_INVALID, m_extra_data: std::ptr::null() },
];

pub static G_REANIMATOR_TRANSFORM_DEF_MAP: DefMap = DefMap {
    m_map_fields: G_REANIMATOR_TRANSFORM_DEF_FIELDS.as_ptr(),
    m_def_size: std::mem::size_of::<ReanimatorTransform>() as i32,
    m_constructor_func: None,
};

pub static G_REANIMATOR_TRACK_DEF_FIELDS: [DefField; 3] = [
    DefField { m_field_name: "name", m_field_offset: std::mem::offset_of!(ReanimatorTrack, m_name) as i32, m_field_type: DefFieldType::DT_STRING, m_extra_data: std::ptr::null() },
    DefField { m_field_name: "t", m_field_offset: std::mem::offset_of!(ReanimatorTrack, m_transforms) as i32, m_field_type: DefFieldType::DT_ARRAY, m_extra_data: (&G_REANIMATOR_TRANSFORM_DEF_MAP as *const DefMap) as *const u8 },
    DefField { m_field_name: "", m_field_offset: 0, m_field_type: DefFieldType::DT_INVALID, m_extra_data: std::ptr::null() },
];

pub static G_REANIMATOR_TRACK_DEF_MAP: DefMap = DefMap {
    m_map_fields: G_REANIMATOR_TRACK_DEF_FIELDS.as_ptr(),
    m_def_size: std::mem::size_of::<ReanimatorTrack>() as i32,
    m_constructor_func: None,
};

pub static G_REANIMATOR_DEF_FIELDS: [DefField; 3] = [
    DefField { m_field_name: "track", m_field_offset: std::mem::offset_of!(ReanimatorDefinition, m_tracks) as i32, m_field_type: DefFieldType::DT_ARRAY, m_extra_data: (&G_REANIMATOR_TRACK_DEF_MAP as *const DefMap) as *const u8 },
    DefField { m_field_name: "fps", m_field_offset: std::mem::offset_of!(ReanimatorDefinition, m_fps) as i32, m_field_type: DefFieldType::DT_FLOAT, m_extra_data: std::ptr::null() },
    DefField { m_field_name: "", m_field_offset: 0, m_field_type: DefFieldType::DT_INVALID, m_extra_data: std::ptr::null() },
];

pub static G_REANIMATOR_DEF_MAP: DefMap = DefMap {
    m_map_fields: G_REANIMATOR_DEF_FIELDS.as_ptr(),
    m_def_size: std::mem::size_of::<ReanimatorDefinition>() as i32,
    m_constructor_func: None,
};

/// C++ ReanimationLoadDefinition (Reanimator.cpp:217) — 加载动画定义
/// XML 源路径可用（compile_and_load）；二进制缓存 TODO
pub fn reanimation_load_definition(the_file_name: &str, the_definition: &mut ReanimatorDefinition) -> bool {
    // C++: DefinitionLoadXML(theFileName, &gReanimatorDefMap, theDefinition)
    let a_xml_file_path = format!("properties/{}", the_file_name);
    crate::sexy_tod_lib::definition::definition_compile_and_load(&a_xml_file_path, &G_REANIMATOR_DEF_MAP, the_definition as *mut ReanimatorDefinition as *mut u8)
}

/// C++ ReanimatorEnsureDefinitionLoaded (Reanimator.cpp:1160)
pub fn reanimator_ensure_definition_loaded(the_reanim_type: ReanimationType) {
    let the_index = the_reanim_type as usize;
    if the_index >= G_LAWN_REANIMATION_ARRAY.len() {
        return;
    }

    // C++: 已加载则返回（gReanimatorDefArray[type] 非空）
    unsafe {
        if !G_REANIMATOR_DEF_ARRAY[the_index].is_null() {
            return;
        }
    }

    // C++: aReanimParams = &gReanimationParamArray[theReanimType]
    let a_reanim_params = &G_LAWN_REANIMATION_ARRAY[the_index];

    // C++: ReanimationLoadDefinition(aReanimParams->mReanimFileName, aReanimDef)
    let mut a_def = Box::new(ReanimatorDefinition::new());
    if reanimation_load_definition(a_reanim_params.m_reanim_file_name, &mut *a_def) {
        unsafe {
            G_REANIMATOR_DEF_ARRAY[the_index] = Box::into_raw(a_def);
        }
    }
    // C++: 失败时 TodErrorMessageBox（发布版退出）；Rust 移植：静默返回
}
pub fn reanim_do_transforms_draw(_g: &mut Graphics, _reanim: &Reanimation) {}

pub fn reanim_loader_load_from_resource_manager() {}

// ============================================================
// 插值辅助函数
// ============================================================
pub fn reanimator_transform_interpolate(
    _result: &mut ReanimatorTransform,
    _a: &ReanimatorTransform,
    _b: &ReanimatorTransform,
    _frac: f32,
) {
    // 插值两个变换之间的所有字段
}

pub fn reanimator_transform_get_global_matrix(_result: &mut SexyTransform2D, _the_transform: &ReanimatorTransform) {
}

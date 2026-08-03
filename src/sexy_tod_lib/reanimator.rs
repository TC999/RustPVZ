// [TRANSLATION_NOTE]: Reanimator.h + Reanimator.cpp -> Rust
// 动画系统：管理动画定义、实例、变换和绘制

use crate::const_enums::ReanimationType;
use crate::sexy_app_framework::graphics::graphics::{Graphics, Image};
use crate::sexy_app_framework::graphics::color::Color;
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

    /// C++ Reanimation::ReanimationInitialize (Reanimator.cpp:387)
    pub fn reanimation_initialize(&mut self, the_x: f32, the_y: f32, the_definition: *mut ReanimatorDefinition) {
        self.m_definition = the_definition;
        self.m_pos_x = the_x;
        self.m_pos_y = the_y;
        self.m_anim_time = 0.0;
        self.m_anim_rate = 1.0;
        self.m_loop_type = ReanimLoopType::REANIM_LOOP;
        self.m_dead = false;
        // C++: mTrackInstances 分配等（未翻译）
        // C++: mLastTrackKeyframe[mTrackCount] 初始化 TODO
    }

    /// C++ Reanimation::ReanimationInitializeType (Reanimator.cpp:348)
    pub fn reanimation_initialize_type(&mut self, the_x: f32, the_y: f32, the_reanim_type: ReanimationType) {
        // C++: ReanimatorEnsureDefinitionLoaded(theReanimType, false) → gReanimatorDefArray[theReanimType]
        crate::sexy_tod_lib::reanimator::reanimator_ensure_definition_loaded(the_reanim_type);
        // [TODO]: 全局定义表（gReanimatorDefArray）查询；当前 m_definition 由外部设置
        self.m_reanim_type = the_reanim_type;
        self.reanimation_initialize(the_x, the_y, self.m_definition);
    }
    /// C++ Reanimation::Update — 推进动画时间（完整 loop 处理）
    pub fn reanimation_update(&mut self) {
        if self.m_dead {
            return;
        }
        self.m_last_processed_time = self.m_anim_time;
        self.m_anim_time += self.m_anim_rate * 0.01;

        let a_track_count = unsafe {
            if self.m_definition.is_null() {
                0.0
            } else {
                (*self.m_definition).m_tracks.count as f32
            }
        };

        // C++: if (mAnimTime >= mDefinition->mTrackCount || mAnimTime < 0)
        if self.m_anim_time >= a_track_count || self.m_anim_time < 0.0 {
            if self.m_loop_type == ReanimLoopType::REANIM_LOOP {
                // C++: mAnimTime = fmod(mAnimTime, mDefinition->mTrackCount);
                if a_track_count > 0.0 {
                    self.m_anim_time = self.m_anim_time.rem_euclid(a_track_count);
                }
            } else if self.m_anim_time >= a_track_count {
                if self.m_loop_type == ReanimLoopType::REANIM_PLAY_ONCE {
                    // C++: 单次播放结束 → 死亡
                    self.m_dead = true;
                } else if self.m_loop_type == ReanimLoopType::REANIM_PLAY_ONCE_AND_HOLD {
                    // C++: 播放一次并保持最后一帧
                    self.m_anim_time = a_track_count - 0.01;
                    self.m_frame_base = a_track_count - 1.0;
                }
            } else {
                self.m_anim_time = 0.0;
            }
        }
        // C++: 其他 loop 类型（LOOP_FULL_LAST_FRAME / PLAY_ONCE_FULL_LAST_FRAME）由帧计算处理
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

            let a_image_alpha = crate::sexy_tod_lib::tod_common::clamp_int(crate::sexy_tod_lib::tod_common::float_round_to_int(a_transform.m_alpha), 0, 255);
            if a_image_alpha <= 0 {
                return false;
            }
            if a_transform.m_image.is_null() {
                return false;
            }

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
                self.draw_track(g, a_track_index);
                // [TODO]: AttachmentDraw（附件动画）
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

    /// C++ Reanimation::SetFramesForLayer — 将动画设置为特定层的帧
    pub fn set_frames_for_layer(&mut self, _the_layer: &str) {
        // [TODO]: Find the track index for the layer, set frame range
    }

    /// C++ Reanimation::TrackExists — 检查指定名称的轨道是否存在
    pub fn track_exists(&self, _the_track_name: &str) -> bool {
        // [TODO]: Search m_definition->m_tracks for matching name
        false
    }

    /// C++ Reanimation::FindTrackIndex — 查找轨道索引
    pub fn find_track_index(&self, _the_track_name: &str) -> i32 {
        // [TODO]: Linear search in definition tracks
        -1
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

pub fn reanimator_load_definitions() {}
pub fn reanimator_ensure_definition_loaded(_the_reanim_type: ReanimationType) {}
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

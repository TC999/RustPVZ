// [TRANSLATION_NOTE]: WidgetManager.h + WidgetManager.cpp -> Rust 翻译
// C++ WidgetManager 类（控件管理器，继承 WidgetContainer）映射为 Rust struct。
// 控件列表 mWidgets 使用 *mut dyn WidgetTrait 以支持动态分发。
// C++ 的 WidgetContainer 基类字段（mWidth/mHeight/mUpdateCnt/mWidgetFlags 等）
// 已合并到本结构中，保持字段名与 C++ 对齐。

use std::collections::LinkedList;

use crate::sexy_app_framework::graphics::graphics::{Graphics, MemoryImage};
use crate::sexy_app_framework::misc::rect::Rect;
use crate::sexy_app_framework::widget::widget_traits::{WidgetContainerTrait, WidgetTrait};

/// WIDGETFLAGS 常量（对应 C++ Widget.h 中的 enum）
pub const WIDGETFLAGS_UPDATE: i32 = 0x00000001;
pub const WIDGETFLAGS_DRAW: i32 = 0x00000002;
pub const WIDGETFLAGS_CLIP: i32 = 0x00000004;
pub const WIDGETFLAGS_ALLOW_MOUSE: i32 = 0x00000008;
pub const WIDGETFLAGS_ALLOW_FOCUS: i32 = 0x00000010;
pub const WIDGETFLAGS_ALLOW_FINGER: i32 = 0x00000020;
pub const WIDGETFLAGS_NOT_VISIBLE: i32 = 0x00000040;
pub const WIDGETFLAGS_MOUSE_PREPEND: i32 = 0x00000080;
pub const WIDGETFLAGS_MOUSE_TRACK: i32 = 0x00000100;
pub const WIDGETFLAGS_MOUSE_OVER: i32 = 0x00000200;
pub const WIDGETFLAGS_MOUSE_DOWN: i32 = 0x00000400;
pub const WIDGETFLAGS_MOUSE_OVER_AND_DOWN: i32 = 0x00000600;

/// PreModalInfo（对应 C++ struct PreModalInfo）
pub struct PreModalInfo {
    pub m_base_modal_widget: *mut dyn WidgetTrait,
    pub m_prev_base_modal_widget: *mut dyn WidgetTrait,
    pub m_prev_focus_widget: *mut dyn WidgetTrait,
}

pub type PreModalInfoList = LinkedList<PreModalInfo>;
pub type DeferredOverlayVector = Vec<(*mut dyn WidgetTrait, i32)>;

/// 哨兵控件 — 用作"空指针"替代（C++ 中为 nullptr）
/// [TRANSLATION_NOTE]: nightly 无法直接构造 null trait object 指针（vtable 必须非 null），
/// 故用一个永不参与绘制的哨兵实例表示空指针，`is_null()` 通过地址比较实现。
pub struct NullWidget;

impl WidgetContainerTrait for NullWidget {
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
    fn add_widget(&mut self, _widget: *mut dyn WidgetTrait) {}
    fn remove_widget(&mut self, _widget: *mut dyn WidgetTrait) {}
    fn remove_all_widgets(&mut self) {}
    fn get_widget_at(&self, _x: i32, _y: i32) -> Option<*mut dyn WidgetTrait> { None }
}

impl WidgetTrait for NullWidget {
    fn set_visible(&mut self, _is_visible: bool) {}
    fn is_visible(&self) -> bool { false }
    fn set_disabled(&mut self, _is_disabled: bool) {}
    fn is_disabled(&self) -> bool { true }
    fn resize(&mut self, _x: i32, _y: i32, _width: i32, _height: i32) {}
    fn get_rect(&self) -> Rect { Rect::new(0, 0, 0, 0) }
    fn draw(&self, _g: &mut dyn crate::sexy_app_framework::widget::widget_traits::GraphicsTrait) {}
    fn update(&mut self) {}
}

pub static mut NULL_WIDGET_INSTANCE: NullWidget = NullWidget;

/// 生成"空"的 `*mut dyn WidgetTrait`（指向哨兵对象）
pub fn null_widget_ptr() -> *mut dyn WidgetTrait {
    unsafe { &mut NULL_WIDGET_INSTANCE as *mut dyn WidgetTrait }
}

/// 判断 WidgetTrait 指针是否为"空"（哨兵）
pub fn widget_ptr_is_null(ptr: *mut dyn WidgetTrait) -> bool {
    std::ptr::addr_eq(ptr, null_widget_ptr())
}

pub struct WidgetManager {
    // WidgetContainer 基类字段
    pub m_widgets: Vec<*mut dyn WidgetTrait>,
    pub m_width: i32,
    pub m_height: i32,
    pub m_update_cnt: u32,
    pub m_dirty: bool,
    pub m_widget_flags: i32,
    // WidgetManager 自身字段
    pub m_default_tab: *mut dyn WidgetTrait,
    pub m_cur_g: *mut Graphics,
    pub m_app: *mut crate::sexy_app_framework::sexy_app_base::SexyAppBase,
    pub m_image: *mut MemoryImage,
    pub m_transient_image: *mut MemoryImage,
    pub m_last_had_transients: bool,
    pub m_popup_command_widget: *mut dyn WidgetTrait,
    pub m_deferred_overlay_widgets: DeferredOverlayVector,
    pub m_min_deferred_overlay_priority: i32,
    pub m_has_focus: bool,
    pub m_focus_widget: *mut dyn WidgetTrait,
    pub m_last_down_widget: *mut dyn WidgetTrait,
    pub m_over_widget: *mut dyn WidgetTrait,
    pub m_base_modal_widget: *mut dyn WidgetTrait,
    pub m_pre_modal_info_list: PreModalInfoList,
    pub m_mouse_dest_rect: Rect,
    pub m_mouse_source_rect: Rect,
    pub m_mouse_in: bool,
    pub m_last_mouse_x: i32,
    pub m_last_mouse_y: i32,
    pub m_down_buttons: i32,
    pub m_actual_down_buttons: i32,
    pub m_key_down: [bool; 0xFF],
    pub m_last_down_button_id: i32,
}

impl WidgetManager {
    pub fn new(the_app: *mut crate::sexy_app_framework::sexy_app_base::SexyAppBase) -> Self {
        WidgetManager {
            m_widgets: Vec::new(),
            m_width: 0,
            m_height: 0,
            m_update_cnt: 0,
            m_dirty: true,
            // C++: WIDGETFLAGS_UPDATE | WIDGETFLAGS_DRAW | WIDGETFLAGS_CLIP |
            //      WIDGETFLAGS_ALLOW_MOUSE | WIDGETFLAGS_ALLOW_FOCUS
            m_widget_flags: WIDGETFLAGS_UPDATE | WIDGETFLAGS_DRAW | WIDGETFLAGS_CLIP
                | WIDGETFLAGS_ALLOW_MOUSE | WIDGETFLAGS_ALLOW_FOCUS,
            m_default_tab: null_widget_ptr(),
            m_cur_g: std::ptr::null_mut(),
            m_app: the_app,
            m_image: std::ptr::null_mut(),
            m_transient_image: std::ptr::null_mut(),
            m_last_had_transients: false,
            m_popup_command_widget: null_widget_ptr(),
            m_deferred_overlay_widgets: Vec::new(),
            m_min_deferred_overlay_priority: 0x7FFFFFFF,
            m_has_focus: true,
            m_focus_widget: null_widget_ptr(),
            m_last_down_widget: null_widget_ptr(),
            m_over_widget: null_widget_ptr(),
            m_base_modal_widget: null_widget_ptr(),
            m_pre_modal_info_list: LinkedList::new(),
            m_mouse_dest_rect: Rect::new(0, 0, 0, 0),
            m_mouse_source_rect: Rect::new(0, 0, 0, 0),
            m_mouse_in: false,
            m_last_mouse_x: 0,
            m_last_mouse_y: 0,
            m_down_buttons: 0,
            m_actual_down_buttons: 0,
            m_key_down: [false; 0xFF],
            m_last_down_button_id: 0,
        }
    }

    pub fn free_resources(&mut self) {
        // C++: 空实现
    }

    pub fn add_widget(&mut self, widget: *mut dyn WidgetTrait) {
        if !widget_ptr_is_null(widget) {
            self.m_widgets.push(widget);
        }
    }

    pub fn remove_widget(&mut self, widget: *mut dyn WidgetTrait) {
        self.m_widgets.retain(|w| !std::ptr::addr_eq(*w, widget));
    }

    pub fn remove_all_widgets(&mut self) {
        self.m_widgets.clear();
    }

    pub fn mark_all_dirty(&mut self) {
        self.m_dirty = true;
        for &widget in &self.m_widgets {
            unsafe {
                if !widget_ptr_is_null(widget) {
                    (*widget).mark_dirty();
                }
            }
        }
    }

    pub fn mark_dirty(&mut self) {
        self.m_dirty = true;
    }

    /// C++: WidgetManager::Resize
    pub fn resize(&mut self, the_mouse_dest_rect: &Rect, the_mouse_source_rect: &Rect) {
        self.m_width = the_mouse_dest_rect.m_width + 2 * the_mouse_dest_rect.m_x;
        self.m_height = the_mouse_dest_rect.m_height + 2 * the_mouse_dest_rect.m_y;
        self.m_mouse_dest_rect = *the_mouse_dest_rect;
        self.m_mouse_source_rect = *the_mouse_source_rect;
    }

    /// C++: WidgetManager::RemapMouse
    pub fn remap_mouse(&self, the_x: &mut i32, the_y: &mut i32) {
        if self.m_mouse_source_rect.m_width != 0 && self.m_mouse_source_rect.m_height != 0 {
            *the_x = (*the_x - self.m_mouse_source_rect.m_x) * self.m_mouse_dest_rect.m_width
                / self.m_mouse_source_rect.m_width
                + self.m_mouse_dest_rect.m_x;
            *the_y = (*the_y - self.m_mouse_source_rect.m_y) * self.m_mouse_dest_rect.m_height
                / self.m_mouse_source_rect.m_height
                + self.m_mouse_dest_rect.m_y;
        }
    }

    /// C++: WidgetManager::DrawScreen
    pub fn draw_screen(&mut self) -> bool {
        let mut drew_stuff = false;
        let mut a_dirty_count = 0;

        // Survey
        for &widget in &self.m_widgets {
            unsafe {
                if !widget_ptr_is_null(widget) {
                    if (*widget).is_dirty() {
                        a_dirty_count += 1;
                    }
                }
            }
        }

        self.m_min_deferred_overlay_priority = 0x7FFFFFFF;
        self.m_deferred_overlay_widgets.clear();

        if a_dirty_count > 0 {
            let is_3d = unsafe {
                if self.m_app.is_null() {
                    false
                } else {
                    (*self.m_app).is_3d_accelerated()
                }
            };

            let widget_list = self.m_widgets.clone();
            for widget in widget_list {
                unsafe {
                    if widget_ptr_is_null(widget) {
                        continue;
                    }
                    let w = &mut *widget;
                    if w.is_dirty() && w.is_visible() {
                        let mut g = Graphics::new();
                        // C++: g.Translate(-mMouseDestRect.mX, -mMouseDestRect.mY)
                        g.translate(-self.m_mouse_dest_rect.m_x, -self.m_mouse_dest_rect.m_y);
                        let rect = w.get_rect();
                        g.translate(rect.m_x, rect.m_y);
                        // C++: aWidget->DrawAll(&aModalFlags, &aClipG)
                        w.draw(&mut g);
                        w.mark_clean();
                        a_dirty_count += 1;
                        drew_stuff = true;
                    }
                }
            }
            let _ = is_3d;
        }

        self.m_cur_g = std::ptr::null_mut();
        drew_stuff
    }

    /// C++: WidgetManager::UpdateFrame
    pub fn update_frame(&mut self) -> bool {
        // C++: mUpdateCnt++; mLastWMUpdateCount = mUpdateCnt; UpdateAll(&aModalFlags);
        self.m_update_cnt += 1;
        let widgets = self.m_widgets.clone();
        for widget in widgets {
            unsafe {
                if !widget_ptr_is_null(widget) {
                    (*widget).update();
                }
            }
        }
        self.m_dirty
    }

    /// C++: WidgetManager::UpdateFrameF
    pub fn update_frame_f(&mut self, _the_frac: f32) -> bool {
        self.m_update_cnt += 1;
        self.m_dirty
    }

    /// C++: WidgetManager::MousePosition
    pub fn mouse_position(&mut self, x: i32, y: i32) {
        let mut the_x = x;
        let mut the_y = y;
        self.remap_mouse(&mut the_x, &mut the_y);
        self.m_last_mouse_x = the_x;
        self.m_last_mouse_y = the_y;
    }

    /// C++: WidgetManager::MouseDown
    pub fn mouse_down(&mut self, x: i32, y: i32, the_click_count: i32) -> bool {
        let mut the_x = x;
        let mut the_y = y;
        self.remap_mouse(&mut the_x, &mut the_y);
        self.m_last_mouse_x = the_x;
        self.m_last_mouse_y = the_y;

        let mut a_widget = self.get_widget_at(the_x, the_y);
        if widget_ptr_is_null(a_widget) {
            a_widget = self.m_base_modal_widget;
        }
        if !widget_ptr_is_null(a_widget) {
            unsafe {
                (*a_widget).mouse_down(the_x, the_y, the_click_count);
            }
            self.m_last_down_widget = a_widget;
        }
        self.m_down_buttons |= 1;
        self.m_actual_down_buttons |= 1;
        self.m_last_down_button_id = the_click_count;
        true
    }

    /// C++: WidgetManager::MouseUp
    pub fn mouse_up(&mut self, x: i32, y: i32, the_click_count: i32) -> bool {
        let mut the_x = x;
        let mut the_y = y;
        self.remap_mouse(&mut the_x, &mut the_y);
        self.m_last_mouse_x = the_x;
        self.m_last_mouse_y = the_y;

        if !widget_ptr_is_null(self.m_last_down_widget) {
            unsafe {
                (*self.m_last_down_widget).mouse_up(the_x, the_y);
            }
            self.m_last_down_widget = null_widget_ptr();
        }
        self.m_down_buttons &= !1;
        self.m_actual_down_buttons &= !1;
        let _ = the_click_count;
        true
    }

    /// C++: WidgetManager::MouseMove
    pub fn mouse_move(&mut self, x: i32, y: i32) -> bool {
        let mut the_x = x;
        let mut the_y = y;
        self.remap_mouse(&mut the_x, &mut the_y);
        self.m_last_mouse_x = the_x;
        self.m_last_mouse_y = the_y;
        self.m_mouse_in = true;

        let a_widget = self.get_widget_at(the_x, the_y);
        if !std::ptr::addr_eq(self.m_over_widget, a_widget) {
            if !widget_ptr_is_null(self.m_over_widget) {
                unsafe {
                    (*self.m_over_widget).mouse_exit(the_x, the_y);
                }
            }
            self.m_over_widget = a_widget;
            if !widget_ptr_is_null(a_widget) {
                unsafe {
                    (*a_widget).mouse_enter(the_x, the_y);
                }
            }
        } else if !widget_ptr_is_null(a_widget) {
            unsafe {
                (*a_widget).mouse_move(the_x, the_y);
            }
        }
        true
    }

    /// C++: WidgetManager::MouseDrag
    pub fn mouse_drag(&mut self, x: i32, y: i32) -> bool {
        let mut the_x = x;
        let mut the_y = y;
        self.remap_mouse(&mut the_x, &mut the_y);
        self.m_last_mouse_x = the_x;
        self.m_last_mouse_y = the_y;

        if !widget_ptr_is_null(self.m_last_down_widget) {
            unsafe {
                (*self.m_last_down_widget).mouse_drag(the_x, the_y);
            }
            return true;
        }
        false
    }

    /// C++: WidgetManager::MouseExit
    pub fn mouse_exit(&mut self, _x: i32, _y: i32) -> bool {
        if !widget_ptr_is_null(self.m_over_widget) {
            unsafe {
                (*self.m_over_widget).mouse_exit(0, 0);
            }
            self.m_over_widget = null_widget_ptr();
        }
        self.m_mouse_in = false;
        true
    }

    /// C++: WidgetManager::MouseWheel
    pub fn mouse_wheel(&mut self, _the_delta: i32) {
        // 滚轮事件（C++ 中转发给 mOverWidget 及其父链）
    }

    /// C++: WidgetManager::KeyDown
    pub fn key_down(&mut self, key: u32) -> bool {
        if key < 0xFF {
            self.m_key_down[key as usize] = true;
        }
        if !widget_ptr_is_null(self.m_focus_widget) {
            unsafe {
                return (*self.m_focus_widget).key_down(key);
            }
        }
        false
    }

    /// C++: WidgetManager::KeyUp
    pub fn key_up(&mut self, key: u32) -> bool {
        if key < 0xFF {
            self.m_key_down[key as usize] = false;
        }
        if !widget_ptr_is_null(self.m_focus_widget) {
            unsafe {
                return (*self.m_focus_widget).key_up(key);
            }
        }
        false
    }

    /// C++: WidgetManager::KeyChar
    pub fn key_char(&mut self, the_char: char) -> bool {
        if !widget_ptr_is_null(self.m_focus_widget) {
            unsafe {
                return (*self.m_focus_widget).key_char(the_char);
            }
        }
        false
    }

    /// C++: WidgetManager::KeyText
    pub fn key_text(&mut self, _the_text: &str) -> bool {
        false
    }

    /// C++: WidgetManager::DoMouseUps()
    pub fn do_mouse_ups(&mut self) {
        if !widget_ptr_is_null(self.m_last_down_widget) && self.m_down_buttons != 0 {
            unsafe {
                (*self.m_last_down_widget).mouse_up(self.m_last_mouse_x, self.m_last_mouse_y);
            }
            self.m_down_buttons = 0;
            self.m_last_down_widget = null_widget_ptr();
        }
    }

    /// C++: WidgetManager::GetAnyWidgetAt
    pub fn get_any_widget_at(&self, x: i32, y: i32) -> *mut dyn WidgetTrait {
        // C++: 从后往前遍历（后添加的在上面）
        for widget in self.m_widgets.iter().rev() {
            unsafe {
                if widget_ptr_is_null(*widget) {
                    continue;
                }
                let rect = (**widget).get_rect();
                if rect.contains_point(x, y) {
                    return *widget;
                }
            }
        }
        null_widget_ptr()
    }

    /// C++: WidgetManager::GetWidgetAt
    pub fn get_widget_at(&self, x: i32, y: i32) -> *mut dyn WidgetTrait {
        let a_widget = self.get_any_widget_at(x, y);
        if !widget_ptr_is_null(a_widget) {
            unsafe {
                if (*a_widget).is_disabled() {
                    return null_widget_ptr();
                }
            }
        }
        a_widget
    }

    /// C++: WidgetManager::SetFocus
    pub fn set_focus(&mut self, a_widget: *mut dyn WidgetTrait) {
        if std::ptr::addr_eq(a_widget, self.m_focus_widget) {
            return;
        }
        unsafe {
            if !widget_ptr_is_null(self.m_focus_widget) {
                (*self.m_focus_widget).lost_focus();
            }
            if !widget_ptr_is_null(a_widget) {
                self.m_focus_widget = a_widget;
                if self.m_has_focus {
                    (*self.m_focus_widget).got_focus();
                }
            } else {
                self.m_focus_widget = null_widget_ptr();
            }
        }
    }

    /// C++: WidgetManager::GotFocus
    pub fn got_focus(&mut self) {
        if !self.m_has_focus {
            self.m_has_focus = true;
            unsafe {
                if !widget_ptr_is_null(self.m_focus_widget) {
                    (*self.m_focus_widget).got_focus();
                }
            }
        }
    }

    /// C++: WidgetManager::LostFocus
    pub fn lost_focus(&mut self) {
        if self.m_has_focus {
            self.m_actual_down_buttons = 0;
            for a_key_num in 0..0xFF {
                if self.m_key_down[a_key_num] {
                    self.key_up(a_key_num as u32);
                }
            }
            self.m_has_focus = false;
            unsafe {
                if !widget_ptr_is_null(self.m_focus_widget) {
                    (*self.m_focus_widget).lost_focus();
                }
            }
        }
    }

    pub fn is_left_button_down(&self) -> bool {
        (self.m_actual_down_buttons & 1) != 0
    }

    pub fn is_middle_button_down(&self) -> bool {
        (self.m_actual_down_buttons & 4) != 0
    }

    pub fn is_right_button_down(&self) -> bool {
        (self.m_actual_down_buttons & 2) != 0
    }

    /// C++: WidgetManager::AddBaseModal(Widget*)
    pub fn add_base_modal(&mut self, the_widget: *mut dyn WidgetTrait) {
        let a_pre_modal_info = PreModalInfo {
            m_base_modal_widget: the_widget,
            m_prev_base_modal_widget: self.m_base_modal_widget,
            m_prev_focus_widget: self.m_focus_widget,
        };
        self.m_pre_modal_info_list.push_back(a_pre_modal_info);
        self.m_base_modal_widget = the_widget;
    }

    /// C++: WidgetManager::RemoveBaseModal
    pub fn remove_base_modal(&mut self, _the_widget: *mut dyn WidgetTrait) {
        if self.m_pre_modal_info_list.len() > 0 {
            self.m_pre_modal_info_list.pop_back();
        }
        self.m_base_modal_widget = null_widget_ptr();
    }
}

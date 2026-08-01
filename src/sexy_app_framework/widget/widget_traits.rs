// [TRANSLATION_NOTE]: WidgetContainer.h -> Rust struct

use std::any::Any;
use crate::sexy_app_framework::graphics::graphics::Graphics;
use crate::sexy_app_framework::misc::rect::Rect;

pub trait WidgetContainerTrait {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    
    fn add_widget(&mut self, widget: *mut dyn WidgetTrait);
    fn remove_widget(&mut self, widget: *mut dyn WidgetTrait);
    fn remove_all_widgets(&mut self);
    fn get_widget_at(&self, x: i32, y: i32) -> Option<*mut dyn WidgetTrait>;
}

pub trait WidgetTrait: WidgetContainerTrait {
    fn set_visible(&mut self, is_visible: bool);
    fn is_visible(&self) -> bool;
    fn set_disabled(&mut self, is_disabled: bool);
    fn is_disabled(&self) -> bool;
    
    fn resize(&mut self, x: i32, y: i32, width: i32, height: i32);
    fn get_rect(&self) -> Rect;
    
    fn draw(&self, g: &mut Graphics);
    fn update(&mut self);
    
    fn mouse_down(&mut self, _x: i32, _y: i32, _click_count: i32) {}
    fn mouse_move(&mut self, _x: i32, _y: i32) {}
    fn mouse_up(&mut self, _x: i32, _y: i32) {}
    fn mouse_drag(&mut self, _x: i32, _y: i32) {}
    fn mouse_enter(&mut self, _x: i32, _y: i32) {}
    fn mouse_exit(&mut self, _x: i32, _y: i32) {}
    fn got_focus(&mut self) {}
    fn lost_focus(&mut self) {}
    fn mark_dirty(&mut self) {}
    fn mark_clean(&mut self) {}
    fn is_dirty(&self) -> bool { false }
    fn key_down(&mut self, _key: u32) -> bool { false }
    fn key_up(&mut self, _key: u32) -> bool { false }
    fn key_char(&mut self, _c: char) -> bool { false }
}

pub trait GraphicsTrait {
    fn draw_rect(&mut self, rect: &Rect);
    fn fill_rect(&mut self, rect: &Rect);
    fn set_color(&mut self, color: &crate::sexy_app_framework::graphics::color::Color);
    fn draw_string(&self, text: &str, x: i32, y: i32);
    fn draw_image(&self, image: &dyn ImageTrait, x: i32, y: i32);
}

pub trait ImageTrait {
    fn get_width(&self) -> i32;
    fn get_height(&self) -> i32;
}

pub trait FontTrait {
    fn draw_string(&self, g: &mut dyn GraphicsTrait, x: i32, y: i32, text: &str);
    fn string_width(&self, text: &str) -> i32;
    fn get_height(&self) -> i32;
}

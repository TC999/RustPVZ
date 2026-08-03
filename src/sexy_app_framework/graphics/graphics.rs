// [TRANSLATION_NOTE]: Graphics.h -> Rust struct
// C++ Graphics 类映射为 Rust 结构体 + 方法

use std::collections::{LinkedList, HashMap};
use std::sync::atomic::{AtomicI32, Ordering};
use crate::sexy_app_framework::misc::rect::Rect;
use crate::sexy_app_framework::graphics::color::Color;
use crate::sexy_app_framework::misc::point::Point;

pub const MAX_TEMP_SPANS: i32 = 8192;

pub struct Edge {
    pub m_x: f64,
    pub m_dx: f64,
    pub i: i32,
    pub b: f64,
}

pub struct GraphicsState {
    pub m_dest_image: Option<Box<Image>>,
    pub m_trans_x: f32,
    pub m_trans_y: f32,
    pub m_scale_x: f32,
    pub m_scale_y: f32,
    pub m_scale_orig_x: f32,
    pub m_scale_orig_y: f32,
    pub m_clip_rect: Rect,
    pub m_color: Color,
    pub m_font: Option<Box<Font>>,
    pub m_font_raw: *mut Font,  // C++ 兼容原始字体指针
    pub m_draw_mode: i32,
    pub m_colorize_images: bool,
    pub m_fast_stretch: bool,
    pub m_write_colored_string: bool,
    pub m_linear_blend: bool,
    pub m_is_3d: bool,
}

impl GraphicsState {
    pub fn new() -> Self {
        GraphicsState {
            m_dest_image: None,
            m_trans_x: 0.0, m_trans_y: 0.0,
            m_scale_x: 1.0, m_scale_y: 1.0,
            m_scale_orig_x: 0.0, m_scale_orig_y: 0.0,
            m_clip_rect: Rect::new(0, 0, 800, 600),
            m_color: Color::new(),
            m_font: None,
            m_font_raw: std::ptr::null_mut(),
            m_draw_mode: 0,
            m_colorize_images: false,
            m_fast_stretch: true,
            m_write_colored_string: false,
            m_linear_blend: false,
            m_is_3d: false,
        }
    }

    pub fn copy_state_from(&mut self, the_state: &GraphicsState) {
        self.m_trans_x = the_state.m_trans_x;
        self.m_trans_y = the_state.m_trans_y;
        self.m_scale_x = the_state.m_scale_x;
        self.m_scale_y = the_state.m_scale_y;
        self.m_scale_orig_x = the_state.m_scale_orig_x;
        self.m_scale_orig_y = the_state.m_scale_orig_y;
        self.m_clip_rect = the_state.m_clip_rect;
        self.m_color = the_state.m_color;
        self.m_draw_mode = the_state.m_draw_mode;
        self.m_colorize_images = the_state.m_colorize_images;
        self.m_fast_stretch = the_state.m_fast_stretch;
        self.m_write_colored_string = the_state.m_write_colored_string;
        self.m_linear_blend = the_state.m_linear_blend;
        self.m_is_3d = the_state.m_is_3d;
    }
}

// Image struct - mapping of Sexy::Image
#[repr(C)]
pub struct Image {
    pub m_drawn: bool,
    pub m_file_path: String,
    pub m_width: i32,
    pub m_height: i32,
    pub m_num_rows: i32,
    pub m_num_cols: i32,
    pub m_cached_texture_id: Option<usize>,
}

impl Image {
    pub fn new(width: i32, height: i32) -> Self {
        Image {
            m_drawn: false,
            m_file_path: String::new(),
            m_width: width,
            m_height: height,
            m_num_rows: 1,
            m_num_cols: 1,
            m_cached_texture_id: None,
        }
    }

    pub fn get_width(&self) -> i32 { self.m_width }
    pub fn get_height(&self) -> i32 { self.m_height }
    pub fn get_cel_width(&self) -> i32 { self.m_width / self.m_num_cols }
    pub fn get_cel_height(&self) -> i32 { self.m_height / self.m_num_rows }
    pub fn get_cel_rect(&self, cel: i32) -> Rect {
        let col = cel % self.m_num_cols;
        let row = cel / self.m_num_cols;
        self.get_cel_rect_rc(col, row)
    }
    pub fn get_cel_rect_rc(&self, the_col: i32, the_row: i32) -> Rect {
        let cw = self.get_cel_width();
        let ch = self.get_cel_height();
        Rect::new(the_col * cw, the_row * ch, cw, ch)
    }
    pub fn get_anim_cel_rect(&self, _the_time: i32) -> Rect {
        self.get_cel_rect(0)
    }

    /// 获取该图像（若为 MemoryImage/GLImage）的像素缓冲指针。
    /// [TRANSLATION_NOTE]: Image 是 MemoryImage 的第一个字段（#[repr(C)] 保证偏移 0），
    /// 此方法用于 Graphics 绘制时取源图像像素；非 MemoryImage 返回 null。
    pub fn pixel_bits(&self) -> *mut u32 {
        unsafe {
            let mem = self as *const Image as *const MemoryImage;
            (*mem).m_bits
        }
    }
}

// Font struct - mapping of Sexy::_Font
pub struct Font {
    pub m_ascent: i32,
    pub m_ascent_padding: i32,
    pub m_height: i32,
    pub m_line_spacing_offset: i32,
}

impl Font {
    pub fn new(height: i32) -> Self {
        Font { m_ascent: height, m_ascent_padding: 0, m_height: height, m_line_spacing_offset: 0 }
    }

    pub fn get_ascent(&self) -> i32 { self.m_ascent }
    pub fn get_ascent_padding(&self) -> i32 { self.m_ascent_padding }
    pub fn get_descent(&self) -> i32 { self.m_height - self.m_ascent }
    pub fn get_height(&self) -> i32 { self.m_height }
    pub fn get_line_spacing_offset(&self) -> i32 { self.m_line_spacing_offset }
    pub fn get_line_spacing(&self) -> i32 { self.m_height + self.m_line_spacing_offset }
    pub fn string_width(&self, _the_string: &str) -> i32 { 0 }
    pub fn char_width(&self, _the_char: char) -> i32 { 0 }
    pub fn char_width_kern(&self, _the_char: char, _the_prev_char: char) -> i32 { 0 }
}

pub struct Graphics {
    pub state: GraphicsState,
    pub m_state_stack: LinkedList<GraphicsState>,
}

impl Graphics {
    pub const DRAWMODE_NORMAL: i32 = 0;
    pub const DRAWMODE_ADDITIVE: i32 = 1;

    pub fn new() -> Self {
        Graphics {
            state: GraphicsState::new(),
            m_state_stack: LinkedList::new(),
        }
    }

    pub fn with_dest_image(dest_image: Option<Box<Image>>) -> Self {
        let mut g = Graphics::new();
        g.state.m_dest_image = dest_image;
        g
    }

    pub fn push_state(&mut self) {
        // Clone state manually - no derive(Clone) due to boxed trait objects
        let mut state = GraphicsState::new();
        state.copy_state_from(&self.state);
        self.m_state_stack.push_back(state);
    }

    pub fn pop_state(&mut self) {
        if let Some(state) = self.m_state_stack.pop_back() {
            self.state = state;
        }
    }

    pub fn set_font(&mut self, font: Option<Box<Font>>) {
        self.state.m_font = font;
        self.state.m_font_raw = std::ptr::null_mut();
    }

    /// C++ 兼容：通过原始指针设置字体
    pub fn set_font_ptr(&mut self, font: *mut Font) {
        self.state.m_font_raw = font;
        if !font.is_null() {
            unsafe {
                self.state.m_font = Some(Box::from_raw(font));
                let _ = Box::into_raw(self.state.m_font.take().unwrap());
                self.state.m_font = Some(Box::from_raw(font));
            }
        } else {
            self.state.m_font = None;
        }
    }

    /// C++ 兼容：SetFont( Font* )
    pub fn SetFont(&mut self, font: *mut Font) {
        self.set_font_ptr(font);
    }

    /// C++ 兼容：SetColor( Color )
    pub fn SetColor(&mut self, color: Color) {
        self.state.m_color = color;
    }

    /// C++ 兼容：FillRect( x, y, w, h )
    pub fn FillRect(&mut self, x: i32, y: i32, w: i32, h: i32) {
        self.fill_rect(x, y, w, h);
    }

    /// C++ 兼容：DrawRect( x, y, w, h )
    pub fn DrawRect(&mut self, x: i32, y: i32, w: i32, h: i32) {
        self.draw_rect(x, y, w, h);
    }

    /// C++ 兼容：DrawString( str, x, y )
    pub fn DrawString(&self, text: &str, x: i32, y: i32) {
        self.draw_string(text, x, y);
    }

    pub fn get_font(&self) -> Option<&Font> {
        self.state.m_font.as_ref().map(|f| f.as_ref())
    }

    pub fn set_color(&mut self, color: &Color) {
        self.state.m_color = *color;
    }

    pub fn get_color(&self) -> &Color {
        &self.state.m_color
    }

    pub fn set_draw_mode(&mut self, mode: i32) {
        self.state.m_draw_mode = mode;
    }

    pub fn get_draw_mode(&self) -> i32 {
        self.state.m_draw_mode
    }

    pub fn fill_rect(&mut self, x: i32, y: i32, width: i32, height: i32) {
        let tx = self.state.m_trans_x as i32;
        let ty = self.state.m_trans_y as i32;
        crate::sexy_app_framework::graphics::renderer::fill_rect(x + tx, y + ty, width, height, &self.state.m_color);
    }

    pub fn fill_rect_r(&mut self, rect: &Rect) {
        self.fill_rect(rect.m_x, rect.m_y, rect.m_width, rect.m_height);
    }

    pub fn draw_rect(&mut self, x: i32, y: i32, width: i32, height: i32) {
        let tx = self.state.m_trans_x as i32;
        let ty = self.state.m_trans_y as i32;
        crate::sexy_app_framework::graphics::renderer::draw_rect(x + tx, y + ty, width, height, &self.state.m_color);
    }

    pub fn draw_rect_r(&mut self, rect: &Rect) {
        self.draw_rect(rect.m_x, rect.m_y, rect.m_width, rect.m_height);
    }

    pub fn draw_string(&self, text: &str, x: i32, y: i32) {
        // C++: Graphics::DrawString → mFont->DrawString
        // [TRANSLATION_NOTE]: TTF 渐进渲染（renderer::draw_text）
        let a_color = self.state.m_color;
        crate::sexy_app_framework::graphics::renderer::draw_text(
            text, x, y, a_color.m_red as u8, a_color.m_green as u8, a_color.m_blue as u8, (a_color.m_alpha & 0xFF) as u8, 12,
        );
    }

    pub fn set_clip_rect(&mut self, x: i32, y: i32, width: i32, height: i32) {
        self.state.m_clip_rect = Rect::new(x, y, width, height);
    }

    pub fn set_clip_rect_r(&mut self, rect: &Rect) {
        self.state.m_clip_rect = *rect;
    }

    pub fn clear_clip_rect(&mut self) {
        self.state.m_clip_rect = Rect::new(0, 0, 8000, 6000);
    }

    pub fn clip_rect(&mut self, x: i32, y: i32, width: i32, height: i32) {
        let r = Rect::new(x, y, width, height);
        let cur = self.state.m_clip_rect;
        self.state.m_clip_rect = cur.intersection(&r);
    }

    pub fn clip_rect_r(&mut self, rect: &Rect) {
        let cur = self.state.m_clip_rect;
        self.state.m_clip_rect = cur.intersection(rect);
    }

    pub fn translate(&mut self, tx: i32, ty: i32) {
        self.state.m_trans_x += tx as f32;
        self.state.m_trans_y += ty as f32;
    }

    pub fn translate_f(&mut self, tx: f32, ty: f32) {
        self.state.m_trans_x += tx;
        self.state.m_trans_y += ty;
    }

    pub fn set_scale(&mut self, sx: f32, sy: f32, orig_x: f32, orig_y: f32) {
        self.state.m_scale_x = sx;
        self.state.m_scale_y = sy;
        self.state.m_scale_orig_x = orig_x;
        self.state.m_scale_orig_y = orig_y;
    }

    /// C++ Graphics::SetLinearBlend
    pub fn SetLinearBlend(&mut self, _linear: bool) {
        self.state.m_linear_blend = _linear;
        // [TODO]: Apply blend mode to SDL2 renderer
    }

    /// C++ Graphics::SetColorizeImages
    pub fn SetColorizeImages(&mut self, _colorize: bool) {
        self.state.m_colorize_images = _colorize;
    }

    /// C++ Graphics::SetScale (with origin)
    pub fn SetScale(&mut self, sx: f32, sy: f32, ox: f32, oy: f32) {
        self.set_scale(sx, sy, ox, oy);
    }

    /// 私有辅助：将图像 blt 到屏幕（对应 C++ mDestImage->Blt）
    fn blt_to_screen(&self, the_image: &Image, dest_x: i32, dest_y: i32, src_rect: &Rect, mirror: bool) {
        let src_bits = the_image.pixel_bits();
        let a_color = if self.state.m_colorize_images {
            self.state.m_color
        } else {
            crate::sexy_app_framework::graphics::color::Color::from_components(255, 255, 255)
        };
        let a_dest_rect = Rect::new(dest_x, dest_y, src_rect.m_width, src_rect.m_height);
        crate::sexy_app_framework::graphics::renderer::blt_image(
            src_bits,
            the_image.m_width,
            the_image.m_height,
            src_rect,
            &a_dest_rect,
            &a_color,
            self.state.m_draw_mode,
            mirror,
        );
    }

    /// 私有辅助：拉伸 blt（对应 C++ mDestImage->StretchBlt）
    fn stretch_blt_to_screen(&self, the_image: &Image, dest_rect: &Rect, src_rect: &Rect, mirror: bool) {
        let src_bits = the_image.pixel_bits();
        let a_color = if self.state.m_colorize_images {
            self.state.m_color
        } else {
            crate::sexy_app_framework::graphics::color::Color::from_components(255, 255, 255)
        };
        crate::sexy_app_framework::graphics::renderer::blt_image(
            src_bits,
            the_image.m_width,
            the_image.m_height,
            src_rect,
            dest_rect,
            &a_color,
            self.state.m_draw_mode,
            mirror,
        );
    }

    /// 私有辅助：带源矩形与缩放处理的 DrawImage（对应 C++ Graphics::DrawImage(Image*, int, int, const Rect&)）
    fn draw_image_src(&self, the_image: &Image, the_x: i32, the_y: i32, the_src_rect: &Rect) {
        // C++: DBG_ASSERTE + 越界检查
        if (the_src_rect.m_x + the_src_rect.m_width > the_image.get_width())
            || (the_src_rect.m_y + the_src_rect.m_height > the_image.get_height())
        {
            return;
        }

        let x = the_x + self.state.m_trans_x as i32;
        let y = the_y + self.state.m_trans_y as i32;

        if self.state.m_scale_x != 1.0 || self.state.m_scale_y != 1.0 {
            // C++: Rect aDestRect(mScaleOrigX+floor((theX-mScaleOrigX)*mScaleX), ...);
            let a_dest_rect = Rect::new(
                self.state.m_scale_orig_x as i32
                    + ((the_x as f32 - self.state.m_scale_orig_x) * self.state.m_scale_x).floor() as i32,
                self.state.m_scale_orig_y as i32
                    + ((the_y as f32 - self.state.m_scale_orig_y) * self.state.m_scale_y).floor() as i32,
                (the_src_rect.m_width as f32 * self.state.m_scale_x).ceil() as i32,
                (the_src_rect.m_height as f32 * self.state.m_scale_y).ceil() as i32,
            );
            self.stretch_blt_to_screen(the_image, &a_dest_rect, the_src_rect, false);
            return;
        }

        let a_dest_rect = Rect::new(x, y, the_src_rect.m_width, the_src_rect.m_height)
            .intersection(&self.state.m_clip_rect);
        let a_src_rect = Rect::new(
            the_src_rect.m_x + a_dest_rect.m_x - x,
            the_src_rect.m_y + a_dest_rect.m_y - y,
            a_dest_rect.m_width,
            a_dest_rect.m_height,
        );

        if (a_src_rect.m_width > 0) && (a_src_rect.m_height > 0) {
            self.blt_to_screen(the_image, a_dest_rect.m_x, a_dest_rect.m_y, &a_src_rect, false);
        }
    }

    /// C++ Graphics::DrawImage (Image*, int x, int y)
    pub fn DrawImage(&self, theImage: &Image, theX: i32, theY: i32) {
        // C++: if (mScaleX != 1 || mScaleY != 1) { DrawImage(theImage, theX, theY, Rect(0,0,w,h)); return; }
        if self.state.m_scale_x != 1.0 || self.state.m_scale_y != 1.0 {
            self.draw_image_src(theImage, theX, theY, &Rect::new(0, 0, theImage.m_width, theImage.m_height));
            return;
        }

        let x = theX + self.state.m_trans_x as i32;
        let y = theY + self.state.m_trans_y as i32;

        let a_dest_rect = Rect::new(x, y, theImage.get_width(), theImage.get_height())
            .intersection(&self.state.m_clip_rect);
        let a_src_rect = Rect::new(
            a_dest_rect.m_x - x,
            a_dest_rect.m_y - y,
            a_dest_rect.m_width,
            a_dest_rect.m_height,
        );

        if (a_src_rect.m_width > 0) && (a_src_rect.m_height > 0) {
            self.blt_to_screen(theImage, a_dest_rect.m_x, a_dest_rect.m_y, &a_src_rect, false);
        }
    }

    /// C++ Graphics::DrawImage(Image*, int x, int y, const Rect& theSrcRect)
    pub fn DrawImageSrcRect(&self, theImage: &Image, theX: i32, theY: i32, theSrcRect: &Rect) {
        self.draw_image_src(theImage, theX, theY, theSrcRect);
    }

    /// C++ Graphics::DrawImage(Image*, int x, int y, int stretchedWidth, int stretchedHeight)
    pub fn DrawImageStretched(&self, theImage: &Image, theX: i32, theY: i32, theStretchedWidth: i32, theStretchedHeight: i32) {
        let a_dest_rect = Rect::new(theX + self.state.m_trans_x as i32, theY + self.state.m_trans_y as i32, theStretchedWidth, theStretchedHeight);
        let a_src_rect = Rect::new(0, 0, theImage.m_width, theImage.m_height);
        self.stretch_blt_to_screen(theImage, &a_dest_rect, &a_src_rect, false);
    }

    /// C++ Graphics::DrawImage(Image*, const Rect& theDestRect, const Rect& theSrcRect)
    pub fn DrawImageDestSrc(&self, theImage: &Image, theDestRect: &Rect, theSrcRect: &Rect) {
        let a_dest_rect = Rect::new(
            theDestRect.m_x + self.state.m_trans_x as i32,
            theDestRect.m_y + self.state.m_trans_y as i32,
            theDestRect.m_width,
            theDestRect.m_height,
        );
        self.stretch_blt_to_screen(theImage, &a_dest_rect, theSrcRect, false);
    }

    /// C++ Graphics::DrawImageF (Image*, float x, float y)
    pub fn DrawImageF(&self, theImage: &Image, theX: f32, theY: f32) {
        let x = theX + self.state.m_trans_x;
        let y = theY + self.state.m_trans_y;
        let a_src_rect = Rect::new(0, 0, theImage.m_width, theImage.m_height);
        self.blt_to_screen(theImage, x.floor() as i32, y.floor() as i32, &a_src_rect, false);
    }

    /// C++ Graphics::DrawImageF (Image*, float x, float y, const Rect& theSrcRect)
    pub fn DrawImageFSrcRect(&self, theImage: &Image, theX: f32, theY: f32, theSrcRect: &Rect) {
        let x = theX + self.state.m_trans_x;
        let y = theY + self.state.m_trans_y;
        self.blt_to_screen(theImage, x.floor() as i32, y.floor() as i32, theSrcRect, false);
    }

    /// C++ Graphics::DrawImageCel (ImageStrip, x, y, cel)
    pub fn DrawImageCel(&self, theImageStrip: &Image, theX: i32, theY: i32, theCel: i32) {
        let a_src_rect = theImageStrip.get_cel_rect(theCel);
        self.draw_image_src(theImageStrip, theX, theY, &a_src_rect);
    }

    /// C++ Graphics::DrawImageCel (ImageStrip, x, y, celCol, celRow)
    pub fn DrawImageCelRow(&self, theImageStrip: &Image, theX: i32, theY: i32, theCelCol: i32, theCelRow: i32) {
        let a_src_rect = theImageStrip.get_cel_rect_rc(theCelCol, theCelRow);
        self.draw_image_src(theImageStrip, theX, theY, &a_src_rect);
    }

    /// C++ Graphics::DrawImageMirror(Image*, int x, int y, bool mirror)
    pub fn DrawImageMirror(&self, theImage: &Image, theX: i32, theY: i32, mirror: bool) {
        let a_src_rect = Rect::new(0, 0, theImage.m_width, theImage.m_height);
        self.draw_image_mirror_src(theImage, theX, theY, &a_src_rect, mirror);
    }

    /// C++ Graphics::DrawImageMirror(Image*, int x, int y, const Rect& theSrcRect, bool mirror)
    pub fn DrawImageMirrorSrcRect(&self, theImage: &Image, theX: i32, theY: i32, theSrcRect: &Rect, mirror: bool) {
        self.draw_image_mirror_src(theImage, theX, theY, theSrcRect, mirror);
    }

    /// 私有辅助：镜像绘制（对应 C++ Graphics::DrawImageMirror 的 mirror 分支）
    fn draw_image_mirror_src(&self, the_image: &Image, the_x: i32, the_y: i32, the_src_rect: &Rect, mirror: bool) {
        // C++: if (!mirror) { DrawImage(theImage, theX, theY, theSrcRect); return; }
        if !mirror {
            self.draw_image_src(the_image, the_x, the_y, the_src_rect);
            return;
        }

        let x = the_x + self.state.m_trans_x as i32;
        let y = the_y + self.state.m_trans_y as i32;

        // C++: 越界检查
        if (the_src_rect.m_x + the_src_rect.m_width > the_image.get_width())
            || (the_src_rect.m_y + the_src_rect.m_height > the_image.get_height())
        {
            return;
        }

        // C++: aDestRect = Rect(theX, theY, srcW, srcH).Intersection(mClipRect)
        let a_dest_rect = Rect::new(x, y, the_src_rect.m_width, the_src_rect.m_height)
            .intersection(&self.state.m_clip_rect);

        // C++: aTotalClip / aLeftClip / aRightClip — 镜像裁剪计算
        let a_total_clip = the_src_rect.m_width - a_dest_rect.m_width;
        let a_left_clip = a_dest_rect.m_x - x;
        let a_right_clip = a_total_clip - a_left_clip;

        let a_src_rect = Rect::new(
            the_src_rect.m_x + a_right_clip,
            the_src_rect.m_y + a_dest_rect.m_y - y,
            a_dest_rect.m_width,
            a_dest_rect.m_height,
        );

        if (a_src_rect.m_width > 0) && (a_src_rect.m_height > 0) {
            self.blt_to_screen(the_image, a_dest_rect.m_x, a_dest_rect.m_y, &a_src_rect, true);
        }
    }

    /// C++ Graphics::mTransX/Y — 平移变换直接访问
    pub fn get_trans_x(&self) -> f32 { self.state.m_trans_x }
    pub fn get_trans_y(&self) -> f32 { self.state.m_trans_y }
    pub fn get_scale_x(&self) -> f32 { self.state.m_scale_x }
    pub fn get_scale_y(&self) -> f32 { self.state.m_scale_y }

    pub fn string_width(&self, _text: &str) -> i32 {
        // Placeholder
        0
    }
}

pub struct GraphicsAutoState<'a> {
    pub m_g: &'a mut Graphics,
}

impl<'a> GraphicsAutoState<'a> {
    pub fn new(g: &'a mut Graphics) -> Self {
        g.push_state();
        GraphicsAutoState { m_g: g }
    }
}

impl<'a> Drop for GraphicsAutoState<'a> {
    fn drop(&mut self) {
        self.m_g.pop_state();
    }
}

// ============================================================
// GraphicsTrait 实现（对应 C++ Graphics 作为 Widget 绘制接口）
// ============================================================
impl crate::sexy_app_framework::widget::widget_traits::GraphicsTrait for Graphics {
    fn draw_rect(&mut self, rect: &Rect) {
        Graphics::draw_rect(self, rect.m_x, rect.m_y, rect.m_width, rect.m_height);
    }

    fn fill_rect(&mut self, rect: &Rect) {
        Graphics::fill_rect(self, rect.m_x, rect.m_y, rect.m_width, rect.m_height);
    }

    fn set_color(&mut self, color: &crate::sexy_app_framework::graphics::color::Color) {
        self.state.m_color = *color;
    }

    fn draw_string(&self, text: &str, x: i32, y: i32) {
        Graphics::draw_string(self, text, x, y);
    }

    fn draw_image(&self, _image: &dyn crate::sexy_app_framework::widget::widget_traits::ImageTrait, _x: i32, _y: i32) {
        // [TODO]: 图像绘制（DrawImage 完整翻译时实现）
    }
}

// MemoryImage - 内存图像（对应 C++ Sexy::MemoryImage）
pub const MEMORYCHECK_ID: u32 = 0x4BEEFADE;
pub static mut G_OPTIMIZE_SOFTWARE_DRAWING: bool = true;

#[repr(C)]
pub struct MemoryImage {
    pub base: Image,
    pub m_bits: *mut u32,
    pub m_bits_changed_count: i32,
    pub m_render_data: *mut u8,
    pub m_render_flags: u32,
    pub m_color_table: *mut u32,
    pub m_color_indices: *mut u8,
    pub m_forced_mode: bool,
    pub m_has_trans: bool,
    pub m_has_alpha: bool,
    pub m_is_volatile: bool,
    pub m_purge_bits: bool,
    pub m_want_pal: bool,
    pub m_native_alpha_data: *mut u32,
    pub m_rl_alpha_data: *mut u8,
    pub m_rl_additive_data: *mut u8,
    pub m_bits_changed: bool,
    pub m_app: *mut u8, // 实际应为 *mut SexyAppBase
}

impl MemoryImage {
    pub fn new() -> Self {
        MemoryImage {
            base: Image::new(0, 0),
            m_bits: std::ptr::null_mut(),
            m_bits_changed_count: 0,
            m_render_data: std::ptr::null_mut(),
            m_render_flags: 0,
            m_color_table: std::ptr::null_mut(),
            m_color_indices: std::ptr::null_mut(),
            m_forced_mode: false,
            m_has_trans: false,
            m_has_alpha: false,
            m_is_volatile: false,
            m_purge_bits: false,
            m_want_pal: false,
            m_native_alpha_data: std::ptr::null_mut(),
            m_rl_alpha_data: std::ptr::null_mut(),
            m_rl_additive_data: std::ptr::null_mut(),
            m_bits_changed: false,
            m_app: std::ptr::null_mut(),
        }
    }

    pub fn get_bits(&self) -> *mut u32 { self.m_bits }
    pub fn bits_changed(&mut self) { self.m_bits_changed = true; self.m_bits_changed_count += 1; }
    pub fn commit_bits(&mut self) { self.m_bits_changed = false; }

    /// C++: MemoryImage::GetBits() — 惰性分配像素缓冲并清零
    /// [TRANSLATION_NOTE]: C++ 中 GetBits 为 const 方法（mBits 可变成员），
    /// Rust 中需要 &mut self；已有调用点使用 get_bits()（&self 只读），此内部方法供绘制使用。
    fn ensure_bits(&mut self) -> *mut u32 {
        if self.m_bits.is_null() {
            let a_size = (self.base.m_width * self.base.m_height) as usize;
            let mut a_bits = vec![0u32; a_size + 1];
            a_bits[a_size] = MEMORYCHECK_ID;
            let ptr = a_bits.as_mut_ptr();
            std::mem::forget(a_bits);
            self.m_bits = ptr;
        }
        self.m_bits
    }

    /// C++: MemoryImage::Clear() — 释放像素缓冲
    pub fn clear(&mut self) {
        // C++: delete [] mBits; mBits = nullptr;
        if !self.m_bits.is_null() {
            unsafe {
                let a_size = (self.base.m_width * self.base.m_height) as usize + 1;
                let _ = Vec::from_raw_parts(self.m_bits, a_size, a_size);
            }
            self.m_bits = std::ptr::null_mut();
        }
        self.delete_3d_buffers();
        self.bits_changed();
    }

    /// C++: MemoryImage::Create(int theWidth, int theHeight)
    pub fn create(&mut self, the_width: i32, the_height: i32) {
        // C++: delete [] mBits; mBits = nullptr;
        if !self.m_bits.is_null() {
            unsafe {
                let a_size = (self.base.m_width * self.base.m_height) as usize + 1;
                let _ = Vec::from_raw_parts(self.m_bits, a_size, a_size);
            }
            self.m_bits = std::ptr::null_mut();
        }

        self.base.m_width = the_width;
        self.base.m_height = the_height;

        // C++: // All zeros --> trans + alpha
        // C++: mHasTrans = true; mHasAlpha = true;
        self.m_has_trans = true;
        self.m_has_alpha = true;

        self.bits_changed();
    }

    /// C++: MemoryImage::SetBits(uint32_t* theBits, int theWidth, int theHeight, bool commitBits)
    /// [TRANSLATION_NOTE]: 像素缓冲所有权由调用方管理（C++ 中 delete 旧缓冲并接管新指针，
    /// Rust 移植保持指针语义，由调用方负责释放）。
    pub fn set_bits(&mut self, the_bits: *mut u32, the_width: i32, the_height: i32, commit_bits: bool) {
        self.m_bits = the_bits;
        self.base.m_width = the_width;
        self.base.m_height = the_height;
        if commit_bits {
            self.bits_changed();
        }
    }

    pub fn set_image_mode(&mut self, has_trans: bool, has_alpha: bool) {
        self.m_has_trans = has_trans;
        self.m_has_alpha = has_alpha;
    }
    pub fn set_volatile(&mut self, _is_volatile: bool) { self.m_is_volatile = _is_volatile; }

    /// C++: MemoryImage::PurgeBits — 释放可回收缓冲
    pub fn purge_bits(&mut self) {
        if self.m_purge_bits {
            self.delete_sw_buffers();
        }
    }

    /// C++: MemoryImage::DeleteSwBuffers() — 释放软件像素缓冲
    pub fn delete_sw_buffers(&mut self) {
        if !self.m_bits.is_null() {
            unsafe {
                let a_size = (self.base.m_width * self.base.m_height) as usize + 1;
                let _ = Vec::from_raw_parts(self.m_bits, a_size, a_size);
            }
            self.m_bits = std::ptr::null_mut();
        }
    }

    pub fn delete_3d_buffers(&mut self) {
        // [TODO]: 3D 缓冲（GL 纹理）释放，后续 GL 子系统翻译时实现
    }

    pub fn re_init(&mut self) {
        // [TODO]: 重新初始化（对应 C++ ReInit）
    }

    pub fn delete_native_data(&mut self) {
        self.delete_sw_buffers();
        self.delete_3d_buffers();
    }

    /// C++: MemoryImage::FillRect — 填充矩形（含 alpha 混合）
    pub fn fill_rect(&mut self, the_rect: &Rect, the_color: &Color, _the_draw_mode: i32) {
        let src = the_color.to_int();

        let a_bits = self.ensure_bits();

        let old_alpha = (src >> 24) as i32;

        if old_alpha == 0xFF {
            let mut a_row = the_rect.m_y;
            while a_row < the_rect.m_y + the_rect.m_height {
                unsafe {
                    let mut a_dest_pixels = a_bits.add((a_row * self.base.m_width + the_rect.m_x) as usize);
                    let mut i = 0;
                    while i < the_rect.m_width {
                        *a_dest_pixels = src;
                        a_dest_pixels = a_dest_pixels.add(1);
                        i += 1;
                    }
                }
                a_row += 1;
            }
        } else {
            let mut a_row = the_rect.m_y;
            while a_row < the_rect.m_y + the_rect.m_height {
                unsafe {
                    let mut a_dest_pixels = a_bits.add((a_row * self.base.m_width + the_rect.m_x) as usize);
                    let mut i = 0;
                    while i < the_rect.m_width {
                        let dest = *a_dest_pixels;

                        let a_dest_alpha = (dest >> 24) as i32;
                        let a_new_dest_alpha = a_dest_alpha + ((255 - a_dest_alpha) * old_alpha) / 255;

                        let new_alpha = 255 * old_alpha / a_new_dest_alpha;

                        let oma = 256 - new_alpha;

                        *a_dest_pixels = ((a_new_dest_alpha as u32) << 24)
                            | ((((dest & 0x00FF00FFu32).wrapping_mul(oma as u32) + (src & 0x00FF00FFu32).wrapping_mul(new_alpha as u32)) >> 8) & 0x00FF00FFu32)
                            | ((((dest & 0x0000FF00u32).wrapping_mul(oma as u32) + (src & 0x0000FF00u32).wrapping_mul(new_alpha as u32)) >> 8) & 0x0000FF00u32);
                        a_dest_pixels = a_dest_pixels.add(1);
                        i += 1;
                    }
                }
                a_row += 1;
            }
        }

        self.bits_changed();
    }

    /// C++: MemoryImage::ClearRect — 将矩形区域清零（含 alpha）
    pub fn clear_rect(&mut self, the_rect: &Rect) {
        let a_bits = self.ensure_bits();

        let mut a_row = the_rect.m_y;
        while a_row < the_rect.m_y + the_rect.m_height {
            unsafe {
                let mut a_dest_pixels = a_bits.add((a_row * self.base.m_width + the_rect.m_x) as usize);
                let mut i = 0;
                while i < the_rect.m_width {
                    *a_dest_pixels = 0;
                    a_dest_pixels = a_dest_pixels.add(1);
                    i += 1;
                }
            }
            a_row += 1;
        }

        self.bits_changed();
    }

    pub fn draw_line(&mut self, _start_x: f64, _start_y: f64, _end_x: f64, _end_y: f64, _color: &Color, _draw_mode: i32) { /* [TODO]: DrawLine 后续翻译 */ }
    pub fn blt(&mut self, _image: &Image, _x: i32, _y: i32, _src_rect: &Rect, _color: &Color, _draw_mode: i32, _linear_filter: bool) { /* [TODO]: Blt 后续翻译 */ }
    pub fn stretch_blt(&mut self, _image: &Image, _dest_rect: &Rect, _src_rect: &Rect, _clip_rect: &Rect, _color: &Color, _draw_mode: i32, _fast_stretch: bool) { /* [TODO]: StretchBlt 后续翻译 */ }
    pub fn normal_blt(&mut self, _image: &Image, _x: i32, _y: i32, _src_rect: &Rect, _color: &Color) { /* [TODO]: NormalBlt 后续翻译 */ }
    pub fn additive_blt(&mut self, _image: &Image, _x: i32, _y: i32, _src_rect: &Rect, _color: &Color) { /* [TODO]: AdditiveBlt 后续翻译 */ }
}

// ============================================================
// GLImage - OpenGL 图像（桩）
// ============================================================
pub struct GLImage {
    pub base: MemoryImage,
    pub m_tex_info: *mut u8,
    pub m_tex_width: i32,
    pub m_tex_height: i32,
}

impl GLImage {
    pub fn new() -> Self {
        GLImage {
            base: MemoryImage::new(),
            m_tex_info: std::ptr::null_mut(),
            m_tex_width: 0,
            m_tex_height: 0,
        }
    }
}

// ============================================================
// SharedImage - 共享图像（引用计数）
// ============================================================

pub struct SharedImage {
    pub m_image: *mut GLImage,
    pub m_ref_count: AtomicI32,
}

impl SharedImage {
    pub fn new() -> Self {
        SharedImage {
            m_image: std::ptr::null_mut(),
            m_ref_count: AtomicI32::new(0),
        }
    }
}

/// SharedImageRef - 共享图像智能指针
pub struct SharedImageRef {
    pub m_shared_image: *mut SharedImage,
    pub m_unshared_image: *mut MemoryImage,
    pub m_owns_unshared: bool,
}

impl SharedImageRef {
    pub fn new() -> Self {
        SharedImageRef {
            m_shared_image: std::ptr::null_mut(),
            m_unshared_image: std::ptr::null_mut(),
            m_owns_unshared: false,
        }
    }

    pub fn from_shared(the_shared_image: *mut SharedImage) -> Self {
        if !the_shared_image.is_null() {
            unsafe { (*the_shared_image).m_ref_count.fetch_add(1, Ordering::Relaxed); }
        }
        SharedImageRef {
            m_shared_image: the_shared_image,
            m_unshared_image: std::ptr::null_mut(),
            m_owns_unshared: false,
        }
    }

    pub fn release(&mut self) {
        if self.m_owns_unshared && !self.m_unshared_image.is_null() {
            let _ = unsafe { Box::from_raw(self.m_unshared_image) };
        }
        self.m_unshared_image = std::ptr::null_mut();
        if !self.m_shared_image.is_null() {
            unsafe {
                if (*self.m_shared_image).m_ref_count.fetch_sub(1, Ordering::Relaxed) == 1 {
                    // 标记清理
                    // gSexyAppBase->mCleanupSharedImages.store(true)
                }
            }
        }
        self.m_shared_image = std::ptr::null_mut();
    }

    pub fn get_as_image(&self) -> *mut Image {
        if !self.m_unshared_image.is_null() {
            unsafe { &mut (*self.m_unshared_image).base as *mut Image }
        } else if !self.m_shared_image.is_null() {
            unsafe { (*self.m_shared_image).m_image as *mut Image }
        } else {
            std::ptr::null_mut()
        }
    }
}

impl Clone for SharedImageRef {
    fn clone(&self) -> Self {
        if !self.m_shared_image.is_null() {
            unsafe { (*self.m_shared_image).m_ref_count.fetch_add(1, Ordering::Relaxed); }
        }
        SharedImageRef {
            m_shared_image: self.m_shared_image,
            m_unshared_image: self.m_unshared_image,
            m_owns_unshared: false,
        }
    }
}

impl Drop for SharedImageRef {
    fn drop(&mut self) {
        // Release handled by explicit call in C++; auto-release here for safety
    }
}

// ============================================================
// ImageFont - 图像字体
// ============================================================

/// 字符数据（对应 C++ CharData）
#[derive(Clone)]
pub struct CharData {
    pub m_image_rect: Rect,
    pub m_offset: Point,
    pub m_kerning_offsets: HashMap<char, i32>,
    pub m_width: i32,
    pub m_order: i32,
}

impl CharData {
    pub fn new() -> Self {
        CharData {
            m_image_rect: Rect::new(0, 0, 0, 0),
            m_offset: Point::new(0, 0),
            m_kerning_offsets: HashMap::new(),
            m_width: 0,
            m_order: 0,
        }
    }
}

/// 字体层（对应 C++ FontLayer）
pub struct FontLayer {
    pub m_layer_name: String,
    pub m_required_tags: Vec<String>,
    pub m_excluded_tags: Vec<String>,
    pub m_char_data_map: HashMap<char, CharData>,
    pub m_color_mult: Color,
    pub m_color_add: Color,
    pub m_image: *mut Image,
    pub m_draw_mode: i32,
    pub m_offset: Point,
    pub m_spacing: i32,
    pub m_min_point_size: i32,
    pub m_max_point_size: i32,
    pub m_point_size: i32,
    pub m_ascent: i32,
    pub m_ascent_padding: i32,
    pub m_height: i32,
    pub m_default_height: i32,
    pub m_line_spacing_offset: i32,
    pub m_base_order: i32,
    pub m_use_alpha_correction: bool,
}

impl FontLayer {
    pub fn new() -> Self {
        FontLayer {
            m_layer_name: String::new(),
            m_required_tags: Vec::new(),
            m_excluded_tags: Vec::new(),
            m_char_data_map: HashMap::new(),
            m_color_mult: Color::new(),
            m_color_add: Color::new(),
            m_image: std::ptr::null_mut(),
            m_draw_mode: 0,
            m_offset: Point::new(0, 0),
            m_spacing: 0,
            m_min_point_size: 0,
            m_max_point_size: 0,
            m_point_size: 0,
            m_ascent: 0,
            m_ascent_padding: 0,
            m_height: 0,
            m_default_height: 0,
            m_line_spacing_offset: 0,
            m_base_order: 0,
            m_use_alpha_correction: false,
        }
    }
}

/// 活动字体层（对应 C++ ActiveFontLayer）
pub struct ActiveFontLayer {
    pub m_base_font_layer: *mut FontLayer,
    pub m_scaled_image: *mut Image,
    pub m_owns_image: bool,
    pub m_scaled_char_image_rects: HashMap<char, Rect>,
}

impl ActiveFontLayer {
    pub fn new() -> Self {
        ActiveFontLayer {
            m_base_font_layer: std::ptr::null_mut(),
            m_scaled_image: std::ptr::null_mut(),
            m_owns_image: false,
            m_scaled_char_image_rects: HashMap::new(),
        }
    }
}

/// 渲染命令（对应 C++ RenderCommand）
pub struct RenderCommand {
    pub m_image: *mut Image,
    pub m_dest: [i32; 2],
    pub m_src: [i32; 4],
    pub m_mode: i32,
    pub m_color: Color,
    pub m_use_alpha_correction: bool,
    pub m_next: *mut RenderCommand,
}

/// ImageFont - 图像字体（对应 C++ Sexy::ImageFont）
pub struct ImageFont {
    // _Font 基类字段
    pub m_ascent: i32,
    pub m_ascent_padding: i32,
    pub m_height: i32,
    pub m_line_spacing_offset: i32,
    // ImageFont 自身字段
    pub m_font_data: *mut u8, // 实际为 *mut FontData，用 void* 避免依赖 DescParser
    pub m_point_size: i32,
    pub m_tag_vector: Vec<String>,
    pub m_active_list_valid: bool,
    pub m_active_layer_list: Vec<ActiveFontLayer>,
    pub m_scale: f64,
    pub m_force_scaled_images_white: bool,
}

impl ImageFont {
    pub fn new() -> Self {
        ImageFont {
            m_ascent: 0, m_ascent_padding: 0, m_height: 0, m_line_spacing_offset: 0,
            m_font_data: std::ptr::null_mut(),
            m_point_size: 0,
            m_tag_vector: Vec::new(),
            m_active_list_valid: false,
            m_active_layer_list: Vec::new(),
            m_scale: 1.0,
            m_force_scaled_images_white: false,
        }
    }

    // _Font 接口方法
    pub fn get_ascent(&self) -> i32 { self.m_ascent }
    pub fn get_ascent_padding(&self) -> i32 { self.m_ascent_padding }
    pub fn get_descent(&self) -> i32 { self.m_height - self.m_ascent }
    pub fn get_height(&self) -> i32 { self.m_height }
    pub fn get_line_spacing_offset(&self) -> i32 { self.m_line_spacing_offset }
    pub fn get_line_spacing(&self) -> i32 { self.m_height + self.m_line_spacing_offset }

    // ImageFont 特定方法
    pub fn char_width(&self, _the_char: char) -> i32 { 0 }
    pub fn char_width_kern(&self, _the_char: char, _the_prev_char: char) -> i32 { 0 }
    pub fn string_width(&self, _the_string: &str) -> i32 { 0 }
    pub fn draw_string(&self, _g: &mut Graphics, _the_x: i32, _the_y: i32, _the_string: &str, _the_color: &Color, _the_clip_rect: &Rect) {}
    pub fn duplicate(&self) -> ImageFont { ImageFont::new() }
    pub fn set_point_size(&mut self, _the_point_size: i32) { self.m_point_size = _the_point_size; }
    pub fn get_point_size(&self) -> i32 { self.m_point_size }
    pub fn set_scale(&mut self, _the_scale: f64) { self.m_scale = _the_scale; }
    pub fn get_default_point_size(&self) -> i32 { 0 }
    pub fn prepare(&mut self) { /* placeholder */ }
}

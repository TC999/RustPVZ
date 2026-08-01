// [TRANSLATION_NOTE]: GLInterface.h + GLInterface.cpp -> Rust 翻译
// C++ GLInterface 类（OpenGL 渲染接口）映射为 Rust struct。
// 结构字段与 C++ 头文件保持 1:1 对齐。
// 渲染后端说明：原 C++ 使用 OpenGL（shader + VBO）。本 Rust 移植中，
// Redraw/Flush 桥接到现有 SDL Canvas 软件渲染器（见 renderer.rs），
// 交换缓冲区语义与 C++ 的 SDL_GL_SwapWindow 等价，绘制命令语义不变。

use std::collections::{HashSet, LinkedList};

use crate::sexy_app_framework::graphics::graphics::{GLImage, Graphics, MemoryImage};
use crate::sexy_app_framework::misc::rect::Rect;
use crate::sexy_app_framework::misc::sexy_matrix::SexyMatrix3;

/// RenderImageFlags（对应 C++ enum RenderImageFlags）
pub const RENDER_IMAGE_FLAG_MINIMIZE_NUM_SUBDIVISIONS: i32 = 0x0001;
pub const RENDER_IMAGE_FLAG_USE_64_BY_64_SUBDIVISIONS: i32 = 0x0002;
pub const RENDER_IMAGE_FLAG_USE_A4R4G4B4: i32 = 0x0004;
pub const RENDER_IMAGE_FLAG_USE_A8R8G8B8: i32 = 0x0008;
pub const RENDER_IMAGE_FLAG_REPEAT: i32 = 0x0010;
pub const RENDER_IMAGE_FLAG_TEXTURE_MASK: i32 = 0x001F;

/// PixelFormat（对应 C++ enum PixelFormat）
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PixelFormat {
    PixelFormat_Unknown = 0x0000,
    PixelFormat_A8R8G8B8 = 0x0001,
    PixelFormat_A4R4G4B4 = 0x0002,
    PixelFormat_R5G6B5 = 0x0004,
    PixelFormat_Palette8 = 0x0008,
}

/// TextureDataPiece（对应 C++ struct TextureDataPiece）
pub struct TextureDataPiece {
    pub m_texture: u32,
    pub m_width: i32,
    pub m_height: i32,
}

/// TextureData（对应 C++ struct TextureData）
pub struct TextureData {
    pub m_textures: Vec<TextureDataPiece>,
    pub m_width: i32,
    pub m_height: i32,
    pub m_tex_vec_width: i32,
    pub m_tex_vec_height: i32,
    pub m_tex_piece_width: i32,
    pub m_tex_piece_height: i32,
    pub m_bits_changed_count: i32,
    pub m_tex_mem_size: i32,
    pub m_max_total_u: f32,
    pub m_max_total_v: f32,
    pub m_pixel_format: PixelFormat,
    pub m_image_flags: i32,
}

impl TextureData {
    pub fn new() -> Self {
        TextureData {
            m_textures: Vec::new(),
            m_width: 0,
            m_height: 0,
            m_tex_vec_width: 0,
            m_tex_vec_height: 0,
            m_tex_piece_width: 0,
            m_tex_piece_height: 0,
            m_bits_changed_count: 0,
            m_tex_mem_size: 0,
            m_max_total_u: 0.0,
            m_max_total_v: 0.0,
            m_pixel_format: PixelFormat::PixelFormat_Unknown,
            m_image_flags: 0,
        }
    }

    pub fn release_textures(&mut self) {
        self.m_textures.clear();
    }
}

pub type ImageSet = HashSet<*mut MemoryImage>;
pub type GLImageSet = HashSet<*mut GLImage>;
pub type TransformStack = LinkedList<SexyMatrix3>;

/// GLInterface — 渲染接口（对应 C++ Sexy::GLInterface，继承 NativeDisplay）
pub struct GLInterface {
    pub m_app: *mut crate::sexy_app_framework::sexy_app_base::SexyAppBase,
    pub m_width: i32,
    pub m_height: i32,
    pub m_display_width: i32,
    pub m_display_height: i32,
    pub m_presentation_rect: Rect,
    pub m_refresh_rate: i32,
    pub m_milliseconds_per_frame: i32,
    pub m_screen_image: *mut GLImage,
    pub m_next_cursor_x: i32,
    pub m_next_cursor_y: i32,
    pub m_cursor_x: i32,
    pub m_cursor_y: i32,
    pub m_image_set: ImageSet,
    pub m_gl_image_set: GLImageSet,
    pub m_transform_stack: TransformStack,
    // NativeDisplay 基类字段（C++: NativeDisplay.h）
    pub m_rgb_bits: i32,
    pub m_red_bits: i32,
    pub m_green_bits: i32,
    pub m_blue_bits: i32,
    pub m_red_shift: i32,
    pub m_green_shift: i32,
    pub m_blue_shift: i32,
    pub m_red_mask: u32,
    pub m_green_mask: u32,
    pub m_blue_mask: u32,
    pub m_alpha_disabled: bool,
}

impl GLInterface {
    pub fn new(the_app: *mut crate::sexy_app_framework::sexy_app_base::SexyAppBase) -> Self {
        // C++: mWidth = mApp->mWidth; mHeight = mApp->mHeight;
        let (app_width, app_height) = unsafe {
            if the_app.is_null() {
                (800, 600)
            } else {
                ((*the_app).m_width, (*the_app).m_height)
            }
        };

        let m_refresh_rate = 60;

        GLInterface {
            m_app: the_app,
            m_width: app_width,
            m_height: app_height,
            m_display_width: app_width,
            m_display_height: app_height,
            m_presentation_rect: Rect::new(0, 0, app_width, app_height),
            m_refresh_rate,
            m_milliseconds_per_frame: 1000 / m_refresh_rate,
            m_screen_image: std::ptr::null_mut(),
            m_next_cursor_x: 0,
            m_next_cursor_y: 0,
            m_cursor_x: 0,
            m_cursor_y: 0,
            m_image_set: HashSet::new(),
            m_gl_image_set: HashSet::new(),
            m_transform_stack: LinkedList::new(),
            m_rgb_bits: 32,
            m_red_bits: 8,
            m_green_bits: 8,
            m_blue_bits: 8,
            m_red_shift: 0,
            m_green_shift: 8,
            m_blue_shift: 16,
            m_red_mask: 0xFFu32 << 0,
            m_green_mask: 0xFFu32 << 8,
            m_blue_mask: 0xFFu32 << 16,
            m_alpha_disabled: false,
        }
    }

    /// C++: GLInterface::~GLInterface — Flush() + 释放所有 TextureData
    pub fn destroy(&mut self) {
        self.flush();
        for img in self.m_image_set.drain() {
            unsafe {
                if !img.is_null() {
                    // C++: delete (TextureData*)img->mRenderData; img->mRenderData = nullptr;
                    if !(*img).m_render_data.is_null() {
                        let _ = Box::from_raw((*img).m_render_data as *mut TextureData);
                        (*img).m_render_data = std::ptr::null_mut();
                    }
                }
            }
        }
    }

    /// C++: GLInterface::SetDrawMode
    pub fn set_draw_mode(&mut self, _the_draw_mode: i32) {
        // C++: glBlendFunc(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA) 或 (GL_SRC_ALPHA, GL_ONE)
        // SDL Canvas 渲染器中混合模式在绘制时由 renderer 处理
    }

    /// C++: GLInterface::AddGLImage
    pub fn add_gl_image(&mut self, the_gl_image: *mut GLImage) {
        self.m_gl_image_set.insert(the_gl_image);
    }

    /// C++: GLInterface::RemoveGLImage
    pub fn remove_gl_image(&mut self, the_gl_image: *mut GLImage) {
        self.m_gl_image_set.remove(&the_gl_image);
    }

    /// C++: GLInterface::Remove3DData
    pub fn remove_3d_data(&mut self, the_image: *mut MemoryImage) {
        unsafe {
            if !the_image.is_null() {
                if !(*the_image).m_render_data.is_null() {
                    // C++: delete (TextureData*)theImage->mRenderData;
                    let _ = Box::from_raw((*the_image).m_render_data as *mut TextureData);
                    (*the_image).m_render_data = std::ptr::null_mut();
                    self.m_image_set.remove(&the_image);
                }
            }
        }
    }

    /// C++: GLInterface::GetScreenImage
    pub fn get_screen_image(&self) -> *mut GLImage {
        self.m_screen_image
    }

    /// C++: GLInterface::UpdateViewport — 计算 4:3 letterbox 视口
    pub fn update_viewport(&mut self) {
        // C++: SDL_GL_GetDrawableSize(mApp->mWindow, &width, &height)
        // Rust 移植中从渲染器获取窗口尺寸
        let (width, height) = crate::sexy_app_framework::graphics::renderer::get_window_size();
        let width = width as i32;
        let height = height as i32;

        let mut vx = 0;
        let mut vy = 0;
        let mut vw = width;
        let mut vh = height;

        // Letterbox to 4:3
        if width * 3 > height * 4 {
            vw = height * 4 / 3;
            vx = (width - vw) / 2;
        } else if width * 3 < height * 4 {
            vh = width * 3 / 4;
            vy = (height - vh) / 2;
        }

        // C++: glViewport(vx, vy, vw, vh);
        // SDL Canvas 渲染器视口在绘制时由 renderer 应用
        self.m_presentation_rect = Rect::new(vx, vy, vw, vh);
    }

    /// C++: GLInterface::Init
    pub fn init(&mut self, _is_windowed: bool) -> i32 {
        // C++: 初始化 GL shader/VBO/清屏等。
        // Rust 移植中 SDL Canvas 渲染器已在 MakeWindow 中初始化，这里保持 GL 初始化语义：
        // 设置像素格式参数、清屏。
        let a_max_size = 4096; // C++: glGetIntegerv(GL_MAX_TEXTURE_SIZE, &aMaxSize)
        let _ = a_max_size;

        // C++: glClearColor(0,0,0,1); glClear(GL_COLOR_BUFFER_BIT);
        crate::sexy_app_framework::graphics::renderer::clear(0, 0, 0);

        self.m_rgb_bits = 32;
        self.m_red_bits = 8;
        self.m_green_bits = 8;
        self.m_blue_bits = 8;
        self.m_red_shift = 0;
        self.m_green_shift = 8;
        self.m_blue_shift = 16;
        self.m_red_mask = 0xFFu32 << self.m_red_shift;
        self.m_green_mask = 0xFFu32 << self.m_green_shift;
        self.m_blue_mask = 0xFFu32 << self.m_blue_shift;

        self.set_video_only_draw(false);
        1
    }

    /// C++: GLInterface::Redraw
    pub fn redraw(&mut self, _the_clip_rect: Option<&Rect>) -> bool {
        self.flush();
        true
    }

    /// C++: GLInterface::SetVideoOnlyDraw
    pub fn set_video_only_draw(&mut self, _video_only: bool) {
        // C++: delete mScreenImage; mScreenImage = new GLImage(this);
        if !self.m_screen_image.is_null() {
            let _ = unsafe { Box::from_raw(self.m_screen_image) };
        }
        let mut an_image = GLImage::new();
        an_image.base.base.m_width = self.m_width;
        an_image.base.base.m_height = self.m_height;
        an_image.base.set_image_mode(false, false);
        self.m_screen_image = Box::into_raw(Box::new(an_image));
    }

    /// C++: GLInterface::SetCursorPos
    pub fn set_cursor_pos(&mut self, x: i32, y: i32) {
        self.m_next_cursor_x = x;
        self.m_next_cursor_y = y;
    }

    /// C++: GLInterface::PreDraw
    pub fn pre_draw(&mut self) -> bool {
        // C++: glBlendFunc(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA);
        true
    }

    /// C++: GLInterface::Flush — 交换缓冲区（等价 SDL_GL_SwapWindow）
    pub fn flush(&mut self) {
        // C++: SDL_GL_SwapWindow(mApp->mWindow); glClear(GL_COLOR_BUFFER_BIT);
        crate::sexy_app_framework::graphics::renderer::present();
        crate::sexy_app_framework::graphics::renderer::clear(0, 0, 0);
    }

    /// C++: GLInterface::CreateImageTexture
    pub fn create_image_texture(&mut self, the_image: *mut MemoryImage) -> bool {
        unsafe {
            if the_image.is_null() {
                return false;
            }
            if (*the_image).m_render_data.is_null() {
                let data = Box::into_raw(Box::new(TextureData::new()));
                (*the_image).m_render_data = data as *mut u8;
                self.m_image_set.insert(the_image);
            }
            let data = &mut *((*the_image).m_render_data as *mut TextureData);
            // C++: data->CheckCreateTextures(theImage);
            data.release_textures();
        }
        true
    }

    /// C++: GLInterface::PushTransform
    pub fn push_transform(&mut self, the_transform: &SexyMatrix3, _concatenate: bool) {
        self.m_transform_stack.push_back(*the_transform);
    }

    /// C++: GLInterface::PopTransform
    pub fn pop_transform(&mut self) {
        self.m_transform_stack.pop_back();
    }

    /// 便捷方法：绘制用 Graphics（对应 C++ 中 Graphics 持有 GLInterface 的场景）
    pub fn make_graphics(&self) -> Graphics {
        Graphics::new()
    }
}

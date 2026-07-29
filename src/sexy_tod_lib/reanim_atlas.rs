// [TRANSLATION_NOTE]: ReanimAtlas.h + ReanimAtlas.cpp -> Rust
// 动画图集：将多个动画图像合并到一张纹理中

use crate::sexy_app_framework::graphics::graphics::{Image, MemoryImage, MEMORYCHECK_ID};

#[derive(Clone)]
pub struct ReanimAtlasImage {
    pub m_x: i32,
    pub m_y: i32,
    pub m_width: i32,
    pub m_height: i32,
    pub m_original_image: *mut Image,
}

impl ReanimAtlasImage {
    pub fn new() -> Self {
        ReanimAtlasImage {
            m_x: 0, m_y: 0, m_width: 0, m_height: 0,
            m_original_image: std::ptr::null_mut(),
        }
    }
}

pub fn sort_by_non_increasing_height(image1: &ReanimAtlasImage, image2: &ReanimAtlasImage) -> bool {
    image1.m_height > image2.m_height
}

pub struct ReanimAtlas {
    pub m_image_array: Vec<ReanimAtlasImage>,
    pub m_memory_image: *mut MemoryImage,
}

impl ReanimAtlas {
    pub fn new() -> Self {
        ReanimAtlas {
            m_image_array: Vec::new(),
            m_memory_image: std::ptr::null_mut(),
        }
    }

    pub fn get_encoded_reanim_atlas(&self, the_image: *mut Image) -> *mut ReanimAtlasImage {
        if the_image.is_null() || the_image as usize > 1000 {
            return std::ptr::null_mut();
        }
        let a_atlas_index = (the_image as usize).wrapping_sub(1);
        if a_atlas_index >= self.m_image_array.len() {
            return std::ptr::null_mut();
        }
        // self.m_image_array 的所有权问题：返回指向 Vec 内部元素的指针
        unsafe {
            self.m_image_array.as_ptr().add(a_atlas_index) as *mut ReanimAtlasImage
        }
    }
}

impl Drop for ReanimAtlas {
    fn drop(&mut self) {
        if !self.m_memory_image.is_null() {
            let _ = unsafe { Box::from_raw(self.m_memory_image) };
        }
    }
}

pub fn reanim_atlas_make_blank_memory_image(the_width: i32, the_height: i32) -> *mut MemoryImage {
    let mut a_image = Box::new(MemoryImage::new());
    let a_bits_count = (the_width * the_height) as usize;

    let layout = std::alloc::Layout::array::<u32>(a_bits_count + 1).unwrap();
    a_image.m_bits = unsafe { std::alloc::alloc(layout) as *mut u32 };
    a_image.base.m_width = the_width;
    a_image.base.m_height = the_height;
    a_image.m_has_trans = true;
    a_image.m_has_alpha = true;
    a_image.m_render_flags = 0;
    if !a_image.m_bits.is_null() {
        unsafe {
            std::ptr::write_bytes(a_image.m_bits, 0, a_bits_count + 1);
            *a_image.m_bits.add(a_bits_count) = MEMORYCHECK_ID;
        }
    }

    Box::into_raw(a_image)
}

// 全局函数：图集的图像编码
// C++ 中使用 Image* 的低位作为图集索引（Image* 实际上是索引值 + 1）
// 在 Rust 中用 usize 存储
pub fn reanim_atlas_encode_id(the_atlas_index: i32) -> *mut Image {
    (the_atlas_index + 1) as *mut Image
}

pub fn reanim_atlas_decode_id(the_image: *mut Image) -> i32 {
    if the_image.is_null() || the_image as usize > 1000 {
        return -1;
    }
    (the_image as usize).wrapping_sub(1) as i32
}

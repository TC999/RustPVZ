// [TRANSLATION_NOTE]: Definition.h + Definition.cpp -> Rust
// 定义系统：管理粒子/轨迹/动画等资源的 XML 定义加载与缓存
// 核心数据结构 + 简化实现

use crate::sexy_tod_lib::tod_list::TodList;
use crate::sexy_tod_lib::trail::FloatParameterTrack;
use crate::sexy_app_framework::graphics::graphics::{Image, Font};
use crate::sexy_app_framework::misc::sexy_vector::SexyVector2;
use crate::sexy_app_framework::misc::xml_parser::XMLParser;

// ============================================================
// 核心数据结构
// ============================================================

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(i32)]
pub enum DefFieldType {
    DT_INVALID,
    DT_INT,
    DT_FLOAT,
    DT_STRING,
    DT_ENUM,
    DT_VECTOR2,
    DT_ARRAY,
    DT_TRACK_FLOAT,
    DT_FLAGS,
    DT_IMAGE,
    DT_FONT,
}

/// 定义符号（枚举/标志映射）
pub struct DefSymbol {
    pub m_symbol_value: i32,
    pub m_symbol_name: Option<&'static str>,
}

/// 结构字段描述
pub struct DefField {
    pub m_field_name: &'static str,
    pub m_field_offset: i32,
    pub m_field_type: DefFieldType,
    pub m_extra_data: *const u8,
}

/// 定义结构图
#[repr(C)]
pub struct DefMap {
    pub m_map_fields: *const DefField,
    pub m_def_size: i32,
    pub m_constructor_func: Option<fn(*mut u8) -> *mut u8>,
}

unsafe impl Send for DefMap {}
unsafe impl Sync for DefMap {}

/// 定义数组
#[repr(C)]
pub struct DefinitionArrayDef {
    pub m_array_data: *mut u8,
    pub m_array_count: i32,
}

/// 压缩定义数据头
#[repr(C)]
pub struct CompressedDefinitionHeader {
    pub m_cookie: u32,
    pub m_uncompressed_size: u32,
}

/// 定义资源路径映射
pub struct DefLoadResPath {
    pub m_prefix: &'static str,
    pub m_directory: &'static str,
}

// ============================================================
// 构造函数（桩）
// ============================================================

pub fn tod_particle_definition_constructor(the_pointer: *mut u8) -> *mut u8 { the_pointer }
pub fn tod_emitter_definition_constructor(the_pointer: *mut u8) -> *mut u8 { the_pointer }
pub fn particle_field_constructor(the_pointer: *mut u8) -> *mut u8 { the_pointer }
pub fn trail_definition_constructor(the_pointer: *mut u8) -> *mut u8 { the_pointer }
pub fn reanimator_transform_constructor(the_pointer: *mut u8) -> *mut u8 { the_pointer }
pub fn reanimator_track_constructor(the_pointer: *mut u8) -> *mut u8 { the_pointer }
pub fn reanimator_definition_constructor(the_pointer: *mut u8) -> *mut u8 { the_pointer }

// ============================================================
// 全局 DefMap 实例（简化：用静态变量表示）
// ============================================================

pub static G_PARTICLE_FIELD_DEF_MAP: DefMap = DefMap {
    m_map_fields: std::ptr::null(),
    m_def_size: 0,
    m_constructor_func: None,
};
pub static G_EMITTER_DEF_MAP: DefMap = DefMap {
    m_map_fields: std::ptr::null(),
    m_def_size: 0,
    m_constructor_func: None,
};
pub static G_PARTICLE_DEF_MAP: DefMap = DefMap {
    m_map_fields: std::ptr::null(),
    m_def_size: 0,
    m_constructor_func: None,
};
pub static G_TRAIL_DEF_MAP: DefMap = DefMap {
    m_map_fields: std::ptr::null(),
    m_def_size: 0,
    m_constructor_func: None,
};
pub static G_REANIMATOR_TRANSFORM_DEF_MAP: DefMap = DefMap {
    m_map_fields: std::ptr::null(),
    m_def_size: 0,
    m_constructor_func: None,
};
pub static G_REANIMATOR_TRACK_DEF_MAP: DefMap = DefMap {
    m_map_fields: std::ptr::null(),
    m_def_size: 0,
    m_constructor_func: None,
};
pub static G_REANIMATOR_DEF_MAP: DefMap = DefMap {
    m_map_fields: std::ptr::null(),
    m_def_size: 0,
    m_constructor_func: None,
};

// ============================================================
// 工具函数
// ============================================================

pub fn definition_get_compiled_file_path_from_xml_file_path(the_xml_file_path: &str) -> String {
    // 将 .xml 扩展名替换为 .compiled
    if the_xml_file_path.ends_with(".xml") {
        let base = &the_xml_file_path[..the_xml_file_path.len() - 4];
        format!("{}.compiled", base)
    } else {
        format!("{}.compiled", the_xml_file_path)
    }
}

pub fn is_file_in_pak_file(_the_file_path: &str) -> bool {
    false
}

pub fn definition_is_compiled(the_xml_file_path: &str) -> bool {
    let compiled_path = definition_get_compiled_file_path_from_xml_file_path(the_xml_file_path);
    std::path::Path::new(&compiled_path).exists()
}

pub fn def_symbol_value_from_string(the_symbol_map: &[DefSymbol], the_name: &str, the_result_value: &mut i32) -> bool {
    for sym in the_symbol_map {
        if let Some(name) = sym.m_symbol_name {
            if name == the_name {
                *the_result_value = sym.m_symbol_value;
                return true;
            }
        }
    }
    false
}

// ============================================================
// XML 定义读取函数（桩）
// ============================================================

pub fn definition_xml_error(_xml_parser: &mut XMLParser, _format: &str) {
    // placeholder
}

/// C++ DefinitionReadXMLString (Definition.cpp:744) — 读取当前元素的文本内容
pub fn definition_read_xml_string(xml_parser: &mut XMLParser, the_value: &mut String) -> bool {
    // C++: 循环读取元素直到 TEXT 节点（跳过子元素 START/END）
    let mut a_element = crate::sexy_app_framework::misc::xml_parser::XMLElement::new();
    loop {
        if !xml_parser.NextElement(&mut a_element) {
            return false;
        }
        // C++: TYPE_TEXT（元素内含文本）；Rust 映射为 TYPE_ELEMENT + mValue
        if a_element.mType == crate::sexy_app_framework::misc::xml_parser::XMLElement::TYPE_ELEMENT {
            *the_value = a_element.mValue.clone();
            return true;
        }
    }
}

/// C++ DefinitionReadIntField (Definition.cpp:752)
pub fn definition_read_int_field(xml_parser: &mut XMLParser, the_value: &mut i32) -> bool {
    let mut a_string_value = String::new();
    if !definition_read_xml_string(xml_parser, &mut a_string_value) {
        return false;
    }
    match a_string_value.trim().parse::<i32>() {
        Ok(v) => {
            *the_value = v;
            true
        }
        Err(_) => {
            definition_xml_error(xml_parser, &format!("Can't parse int value '{}'", a_string_value));
            false
        }
    }
}

/// C++ DefinitionReadFloatField (Definition.cpp:765)
pub fn definition_read_float_field(xml_parser: &mut XMLParser, the_value: &mut f32) -> bool {
    let mut a_string_value = String::new();
    if !definition_read_xml_string(xml_parser, &mut a_string_value) {
        return false;
    }
    match a_string_value.trim().parse::<f32>() {
        Ok(v) => {
            *the_value = v;
            true
        }
        Err(_) => {
            definition_xml_error(xml_parser, &format!("Can't parse float value '{}'", a_string_value));
            false
        }
    }
}

/// C++ DefinitionReadStringField (Definition.cpp:778)
pub fn definition_read_string_field(xml_parser: &mut XMLParser, the_value: *mut *const u8) -> bool {
    let mut a_string_value = String::new();
    if !definition_read_xml_string(xml_parser, &mut a_string_value) {
        return false;
    }
    // C++: DefinitionAlloc + memcpy；Rust 用 Box::leak 持�有 C 风格字符串
    let a_c_string = std::ffi::CString::new(a_string_value).unwrap_or_default();
    let a_ptr = a_c_string.into_raw();
    unsafe {
        *the_value = a_ptr as *const u8;
    }
    true
}

/// C++ DefinitionReadEnumField (Definition.cpp:790)
pub fn definition_read_enum_field(xml_parser: &mut XMLParser, the_value: &mut i32, the_symbol_map: &[DefSymbol]) -> bool {
    let mut a_string_value = String::new();
    if !definition_read_xml_string(xml_parser, &mut a_string_value) {
        return false;
    }
    if def_symbol_value_from_string(the_symbol_map, a_string_value.trim(), the_value) {
        return true;
    }
    definition_xml_error(xml_parser, &format!("Unknown enum value '{}'", a_string_value));
    false
}

/// C++ DefinitionReadVector2Field (Definition.cpp:810) — "x, y" 或两个子元素
pub fn definition_read_vector2_field(xml_parser: &mut XMLParser, the_value: &mut SexyVector2) -> bool {
    let mut a_string_value = String::new();
    if !definition_read_xml_string(xml_parser, &mut a_string_value) {
        return false;
    }
    // C++: sscanf("%f, %f")
    let a_trim = a_string_value.trim();
    let a_parts: Vec<&str> = a_trim.split(',').collect();
    if a_parts.len() == 2 {
        if let (Ok(x), Ok(y)) = (a_parts[0].trim().parse::<f32>(), a_parts[1].trim().parse::<f32>()) {
            the_value.x = x;
            the_value.y = y;
            return true;
        }
    }
    definition_xml_error(xml_parser, &format!("Can't parse vector2 '{}'", a_string_value));
    false
}

/// C++ DefinitionReadFlagField (Definition.cpp:709) — 标志位读取
pub fn definition_read_flag_field(xml_parser: &mut XMLParser, the_value: &str, the_flags: &mut u32, the_symbol_map: &[DefSymbol]) -> bool {
    // C++: 空格分隔的符号列表，逐位设置
    let mut a_value = the_value.to_string();
    if a_value.starts_with("| ") {
        a_value = a_value[3..].to_string();
    }
    for a_part in a_value.split('|') {
        let a_part = a_part.trim();
        if a_part.is_empty() {
            continue;
        }
        let mut a_symbol_value = 0;
        if !def_symbol_value_from_string(the_symbol_map, a_part, &mut a_symbol_value) {
            definition_xml_error(xml_parser, &format!("Unknown flag '{}'", a_part));
            return false;
        }
        *the_flags |= a_symbol_value as u32;
    }
    true
}

/// C++ DefinitionReadArrayField (Definition.cpp:823) — 数组元素（倍增扩容）
pub fn definition_read_array_field(xml_parser: &mut XMLParser, the_array: &mut DefinitionArrayDef, the_field: &DefField) -> bool {
    // C++: theField->mExtraData 指向元素 DefMap
    let a_def_map = the_field.m_extra_data as *const DefMap;
    if a_def_map.is_null() {
        return false;
    }
    unsafe {
        let a_def_map = &*a_def_map;
        // C++: 内存分配（DefinitionAlloc 等价：Vec 泄漏）
        fn alloc_bytes(size: usize) -> *mut u8 {
            let mut a_vec = Vec::<u8>::with_capacity(size);
            let a_ptr = a_vec.as_mut_ptr();
            std::mem::forget(a_vec);
            a_ptr
        }
        fn free_bytes(ptr: *mut u8, size: usize) {
            if !ptr.is_null() {
                unsafe { drop(Vec::from_raw_parts(ptr, 0, size)); }
            }
        }
        if the_array.m_array_count == 0 {
            // C++: 首次分配
            the_array.m_array_count = 1;
            the_array.m_array_data = alloc_bytes(a_def_map.m_def_size as usize);
        } else {
            // C++: 2 的幂次时倍增扩容
            if the_array.m_array_count == 1 || (the_array.m_array_count & (the_array.m_array_count - 1)) == 0 {
                let an_old_data = the_array.m_array_data;
                let a_new_size = (2 * the_array.m_array_count * a_def_map.m_def_size) as usize;
                let a_new_data = alloc_bytes(a_new_size);
                std::ptr::copy_nonoverlapping(an_old_data, a_new_data, (the_array.m_array_count * a_def_map.m_def_size) as usize);
                free_bytes(an_old_data, (the_array.m_array_count * a_def_map.m_def_size) as usize);
                the_array.m_array_data = a_new_data;
            }
            the_array.m_array_count += 1;
        }

        let a_element = the_array.m_array_data.add((a_def_map.m_def_size * (the_array.m_array_count - 1)) as usize);
        if definition_load_map(xml_parser, a_def_map, a_element) {
            return true;
        }
    }
    definition_xml_error(xml_parser, "failed to read sub def");
    false
}

/// C++ DefinitionReadFloatTrackField — 浮点轨道（节点列表）
pub fn definition_read_float_track_field(xml_parser: &mut XMLParser, the_track: &mut FloatParameterTrack) -> bool {
    let mut a_string_value = String::new();
    if !definition_read_xml_string(xml_parser, &mut a_string_value) {
        return false;
    }
    // C++: 节点由空格分隔的 "值(,值)" 组成
    let a_nodes: Vec<&str> = a_string_value.split(|c: char| c.is_whitespace()).filter(|s| !s.is_empty()).collect();
    for a_node in a_nodes {
        let a_parts: Vec<&str> = a_node.split(',').collect();
        if let Ok(a_value) = a_parts[0].trim().parse::<f32>() {
            let a_count = the_track.m_count as usize;
            if a_count < 16 {
                the_track.m_nodes[a_count] = a_value;
                the_track.m_count += 1;
            }
        }
    }
    true
}

/// C++ DefinitionReadImageField — 图像字段（ResourceManager 加载）
pub fn definition_read_image_field(xml_parser: &mut XMLParser, the_image: *mut *mut Image) -> bool {
    let mut a_string_value = String::new();
    if !definition_read_xml_string(xml_parser, &mut a_string_value) {
        return false;
    }
    // [TODO]: gSexyAppBase->mResourceManager->GetImage
    unsafe {
        *the_image = std::ptr::null_mut();
    }
    let _ = a_string_value;
    true
}

/// C++ DefinitionReadFontField
pub fn definition_read_font_field(_xml_parser: &mut XMLParser, the_font: *mut *mut Font) -> bool {
    unsafe {
        *the_font = std::ptr::null_mut();
    }
    true
}

/// C++ DefinitionReadField (Definition.cpp:1113) — 字段分发主循环
pub fn definition_read_field(xml_parser: &mut XMLParser, the_def_map: &DefMap, the_definition: *mut u8, the_done: &mut bool) -> bool {
    if xml_parser.HasFailed() {
        return false;
    }

    let mut a_xml_element = crate::sexy_app_framework::misc::xml_parser::XMLElement::new();
    if !xml_parser.NextElement(&mut a_xml_element) || a_xml_element.mType == crate::sexy_app_framework::misc::xml_parser::XMLElement::TYPE_END {
        *the_done = true;
        return true;
    }
    if a_xml_element.mType != crate::sexy_app_framework::misc::xml_parser::XMLElement::TYPE_START {
        definition_xml_error(xml_parser, "Missing element start");
        return false;
    }

    // C++: 遍历 DefMap 字段表（mFieldName == "" 结束）
    let mut a_field_index = 0;
    unsafe {
        loop {
            let a_field = &*the_def_map.m_map_fields.add(a_field_index);
            if a_field.m_field_name.is_empty() {
                break;
            }
            let a_p_var = the_definition.add(a_field.m_field_offset as usize) as *mut std::ffi::c_void;

            // C++: 元素名匹配
            if a_xml_element.mValue.eq_ignore_ascii_case(a_field.m_field_name) {
                let a_success = match a_field.m_field_type {
                    DefFieldType::DT_INT => {
                        let a_value = &mut *(a_p_var as *mut i32);
                        definition_read_int_field(xml_parser, a_value)
                    }
                    DefFieldType::DT_FLOAT => {
                        let a_value = &mut *(a_p_var as *mut f32);
                        definition_read_float_field(xml_parser, a_value)
                    }
                    DefFieldType::DT_STRING => {
                        let a_value = a_p_var as *mut *const u8;
                        definition_read_string_field(xml_parser, a_value)
                    }
                    DefFieldType::DT_ENUM => {
                        let a_value = &mut *(a_p_var as *mut i32);
                        let a_symbols = a_field.m_extra_data as *const DefSymbol;
                        let a_symbols = std::slice::from_raw_parts(a_symbols, 64);
                        definition_read_enum_field(xml_parser, a_value, a_symbols)
                    }
                    DefFieldType::DT_VECTOR2 => {
                        let a_value = &mut *(a_p_var as *mut SexyVector2);
                        definition_read_vector2_field(xml_parser, a_value)
                    }
                    DefFieldType::DT_ARRAY => {
                        let a_value = &mut *(a_p_var as *mut DefinitionArrayDef);
                        definition_read_array_field(xml_parser, a_value, a_field)
                    }
                    DefFieldType::DT_TRACK_FLOAT => {
                        let a_value = &mut *(a_p_var as *mut FloatParameterTrack);
                        definition_read_float_track_field(xml_parser, a_value)
                    }
                    DefFieldType::DT_FLAGS => {
                        let a_value = &mut *(a_p_var as *mut u32);
                        let a_symbols = a_field.m_extra_data as *const DefSymbol;
                        let a_symbols = std::slice::from_raw_parts(a_symbols, 64);
                        definition_read_flag_field(xml_parser, &a_xml_element.mValue, a_value, a_symbols)
                    }
                    DefFieldType::DT_IMAGE => {
                        let a_value = a_p_var as *mut *mut Image;
                        definition_read_image_field(xml_parser, a_value)
                    }
                    DefFieldType::DT_FONT => {
                        let a_value = a_p_var as *mut *mut Font;
                        definition_read_font_field(xml_parser, a_value)
                    }
                    _ => false,
                };
                if a_success {
                    return true;
                }
                definition_xml_error(xml_parser, &format!("Failed to read '{}' field", a_xml_element.mValue));
                return false;
            }
            a_field_index += 1;
        }
    }
    definition_xml_error(xml_parser, &format!("Ignoring unknown element '{}'", a_xml_element.mValue));
    false
}

/// C++ DefinitionLoadMap (Definition.cpp:1185) — 结构体定义加载
pub fn definition_load_map(xml_parser: &mut XMLParser, the_def_map: &DefMap, the_definition: *mut u8) -> bool {
    // C++: 构造函数
    if let Some(a_constructor) = the_def_map.m_constructor_func {
        a_constructor(the_definition);
    }
    loop {
        let mut a_done = false;
        if !definition_read_field(xml_parser, the_def_map, the_definition, &mut a_done) {
            return false;
        }
        if a_done {
            break;
        }
    }
    true
}

// ============================================================
// 文件加载/编译（桩）
// ============================================================

pub fn definition_load_xml(the_filename: &str, _the_def_map: &DefMap, _the_definition: *mut u8) -> bool {
    let compiled_path = definition_get_compiled_file_path_from_xml_file_path(the_filename);
    if definition_is_compiled(the_filename) {
        return definition_read_compiled_file(&compiled_path, _the_def_map, _the_definition);
    }
    false
}

pub fn definition_read_compiled_file(_the_compiled_file_path: &str, _the_def_map: &DefMap, _the_definition: *mut u8) -> bool {
    // C++: zlib 解压 + CRC 校验 + DefMapReadFromCache 指针修复
    // [TODO]: 二进制缓存解析（DefMapReadFromCache 逐字段读取）
    let _ = (_the_compiled_file_path, _the_def_map, _the_definition);
    false
}

pub fn definition_compile_and_load(the_xml_file_path: &str, the_def_map: &DefMap, the_definition: *mut u8) -> bool {
    // C++: 编译缓存路径（compiled/<xml>.compiled）
    let a_compiled_file_path = definition_get_compiled_file_path_from_xml_file_path(the_xml_file_path);

    // C++: 优先读二进制缓存
    if definition_read_compiled_file(&a_compiled_file_path, the_def_map, the_definition) {
        return true;
    }

    // C++: 失败 → 从 XML 源编译
    if definition_compile_file(the_xml_file_path, &a_compiled_file_path, the_def_map, the_definition) {
        return true;
    }

    false
}


pub fn definition_load_image(_the_image: *mut *mut Image, _the_name: &str) -> bool {
    false
}

pub fn definition_load_font(_the_font: *mut *mut Font, _the_name: &str) -> bool {
    false
}

pub fn definition_fill_with_defaults(_the_def_map: &DefMap, _the_definition: *mut u8) {
    // placeholder
}

// ============================================================
// 缓存读写（桩）
// ============================================================

pub fn def_map_write_to_cache(_write_ptr: &mut *mut u8, _the_def_map: &DefMap, _the_definition: *mut u8) {}
pub fn def_write_to_cache_string(_write_ptr: &mut *mut u8, _the_value: *const *const u8) {}
pub fn def_write_to_cache_array(_write_ptr: &mut *mut u8, _the_value: &DefinitionArrayDef, _the_def_map: &DefMap) {}
pub fn def_write_to_cache_float_track(_write_ptr: &mut *mut u8, _the_value: &FloatParameterTrack) {}
pub fn def_write_to_cache_image(_write_ptr: &mut *mut u8, _the_value: *mut *mut Image) {}
pub fn def_write_to_cache_font(_write_ptr: &mut *mut u8, _the_value: *mut *mut Font) {}

pub fn definition_compress_compiled_buffer(_the_buffer: *mut u8, _the_buffer_size: u32, _the_result_size: &mut u32) -> *mut u8 {
    std::ptr::null_mut()
}

pub fn definition_uncompress_compiled_buffer(_the_compressed_buffer: *mut u8, _the_compressed_buffer_size: usize, _the_uncompressed_size: &mut usize, _the_compiled_file_path: &str) -> *mut u8 {
    std::ptr::null_mut()
}

pub fn def_map_read_from_cache(_read_ptr: &mut *mut u8, _the_def_map: &DefMap, _the_definition: *mut u8) -> bool {
    false
}

// ============================================================
// 编译文件（桩）
// ============================================================

pub fn definition_write_compiled_file(_the_compiled_file_path: &str, _the_def_map: &DefMap, _the_definition: *mut u8) -> bool {
    false
}

pub fn definition_compile_file(the_xml_file_path: &str, _the_compiled_file_path: &str, the_def_map: &DefMap, the_definition: *mut u8) -> bool {
    let mut a_xml_parser = XMLParser::new();
    if !a_xml_parser.OpenFile(the_xml_file_path) {
        return false;
    }
    if !definition_load_map(&mut a_xml_parser, the_def_map, the_definition) {
        return false;
    }
    // [TODO]: DefinitionWriteCompiledFile（缓存写盘）
    let _ = _the_compiled_file_path;
    true
}

// ============================================================
// 内存管理
// ============================================================

pub fn definition_alloc(the_size: i32) -> *mut u8 {
    let layout = std::alloc::Layout::array::<u8>(the_size as usize).unwrap();
    unsafe { std::alloc::alloc(layout) }
}

pub fn definition_free_array_field(_the_array: &mut DefinitionArrayDef, _the_def_map: &DefMap) {}
pub fn definition_free_map(_the_def_map: &DefMap, _the_definition: *mut u8) {}

// ============================================================
// 哈希计算（桩）
// ============================================================

pub fn definition_calc_hash_symbol_map(_a_schema_hash: i32, _the_symbol_map: &[DefSymbol]) -> u32 {
    0
}

pub fn definition_calc_hash_def_map(_a_schema_hash: i32, _the_def_map: &DefMap, _the_progress_maps: &mut TodList<*const DefMap>) -> u32 {
    0
}

pub fn definition_calc_hash(_the_def_map: &DefMap) -> u32 {
    0
}

// ============================================================
// 大小计算
// ============================================================

pub fn def_get_size_string(_the_value: *const *const u8) -> u32 { 0 }
pub fn definition_get_array_size(_the_value: &DefinitionArrayDef, _the_def_map: &DefMap) -> u32 { 0 }
pub fn def_get_size_float_track(_the_value: &FloatParameterTrack) -> u32 { 0 }
pub fn def_get_size_image(_the_value: *mut *mut Image) -> u32 { 0 }
pub fn def_get_size_font(_the_value: *mut *mut Font) -> u32 { 0 }
pub fn definition_get_deep_size(_the_def_map: &DefMap, _the_definition: *mut u8) -> u32 { 0 }
pub fn definition_get_size(_the_def_map: &DefMap, _the_definition: *mut u8) -> u32 { 0 }

// ============================================================
// FloatTrack 函数（委托至 trail 模块）
// ============================================================


// [TRANSLATION_NOTE]: Common.h -> Rust 模块
// C++ 全局函数和类型定义翻译为 Rust 函数和类型别名
// 使用 Mutex/Atomic 替代 static mut 以保证线程安全

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::Mutex;

// Type aliases matching C++ typedefs
pub type Uchar = u8;
pub type Ushort = u16;
pub type Uint = u32;
pub type Ulong = u32;
pub type Int64 = i64;

pub type DefinesMap = HashMap<String, String>;
pub type CharVector = Vec<u8>;

pub const SEXY_RAND_MAX: u32 = 0x7FFFFFFF;

// Global debug flag
pub static G_DEBUG: AtomicBool = AtomicBool::new(false);

// Random number generation (simple LCG for the global Rand/SRand functions)
static G_RAND_SEED: AtomicU32 = AtomicU32::new(0);

pub fn rand_int() -> i32 {
    let old = G_RAND_SEED.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |seed| {
        Some(seed.wrapping_mul(214013).wrapping_add(2531011))
    });
    match old {
        Ok(seed) => ((seed >> 16) & SEXY_RAND_MAX) as i32,
        Err(seed) => ((seed >> 16) & SEXY_RAND_MAX) as i32,
    }
}

pub fn rand_range(range: i32) -> i32 {
    if range <= 0 {
        0
    } else {
        rand_int() % range
    }
}

pub fn rand_float(range: f32) -> f32 {
    (rand_int() as f32 / SEXY_RAND_MAX as f32) * range
}

pub fn srand(seed: u32) {
    G_RAND_SEED.store(seed, Ordering::SeqCst);
}

static G_APP_DATA_FOLDER: Mutex<String> = Mutex::new(String::new());
static G_RESOURCE_FOLDER: Mutex<String> = Mutex::new(String::new());

pub fn get_app_data_folder() -> String {
    #[cfg(target_os = "windows")]
    {
        std::env::var("APPDATA").unwrap_or_else(|_| String::from("."))
    }
    #[cfg(not(target_os = "windows"))]
    {
        std::env::var("HOME").unwrap_or_else(|_| String::from("."))
    }
}

pub fn set_app_data_folder(the_path: &str) {
    *G_APP_DATA_FOLDER.lock().unwrap() = the_path.to_string();
}

pub fn get_app_data_path(relative_path: &str) -> String {
    let base = G_APP_DATA_FOLDER.lock().unwrap().clone();
    if base.is_empty() {
        get_app_data_folder() + "\\" + relative_path
    } else {
        base + "\\" + relative_path
    }
}

pub fn get_resource_folder() -> String {
    G_RESOURCE_FOLDER.lock().unwrap().clone()
}

pub fn set_resource_folder(the_path: &str) {
    *G_RESOURCE_FOLDER.lock().unwrap() = the_path.to_string();
}

pub fn get_resource_path(relative_path: &str) -> String {
    let base = get_resource_folder();
    if base.is_empty() {
        relative_path.to_string()
    } else {
        base + "\\" + relative_path
    }
}

pub fn string_to_upper(s: &str) -> String {
    s.to_uppercase()
}

pub fn string_to_lower(s: &str) -> String {
    s.to_lowercase()
}

pub fn upper(data: &str) -> String {
    data.to_uppercase()
}

pub fn lower(data: &str) -> String {
    data.to_lowercase()
}

pub fn trim(s: &str) -> String {
    s.trim().to_string()
}

pub fn string_to_int(s: &str, out_val: &mut i32) -> bool {
    match s.trim().parse::<i32>() {
        Ok(v) => {
            *out_val = v;
            true
        }
        Err(_) => false,
    }
}

pub fn string_to_double(s: &str, out_val: &mut f64) -> bool {
    match s.trim().parse::<f64>() {
        Ok(v) => {
            *out_val = v;
            true
        }
        Err(_) => false,
    }
}

pub fn str_find_no_case(haystack: &str, needle: &str) -> i32 {
    let lower = haystack.to_lowercase();
    let find = needle.to_lowercase();
    match lower.find(&find) {
        Some(pos) => pos as i32,
        None => -1,
    }
}

pub fn str_prefix_no_case(s: &str, prefix: &str, max_length: i32) -> bool {
    let len = (prefix.len() as i32).min(max_length) as usize;
    if len > s.len() {
        return false;
    }
    s[..len].to_lowercase() == prefix[..len].to_lowercase()
}

pub fn comma_seperate(value: i32) -> String {
    let s = value.to_string();
    let mut result = String::new();
    let len = s.len();
    for (i, c) in s.chars().enumerate() {
        if i > 0 && (len - i) % 3 == 0 {
            result.push(',');
        }
        result.push(c);
    }
    result
}

pub fn evaluate(s: &str, defines_map: &DefinesMap) -> String {
    let mut result = s.to_string();
    for (key, value) in defines_map {
        let search = format!("{{{}}}", key);
        result = result.replace(&search, value);
    }
    result
}

pub fn xml_decode_string(s: &str) -> String {
    s.replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&apos;", "'")
}

pub fn xml_encode_string(s: &str) -> String {
    s.replace("&", "&amp;")
        .replace("<", "&lt;")
        .replace(">", "&gt;")
        .replace("\"", "&quot;")
        .replace("'", "&apos;")
}

pub fn deltree(path: &str) -> bool {
    let p = std::path::Path::new(path);
    if p.is_dir() {
        std::fs::remove_dir_all(p).is_ok()
    } else if p.is_file() {
        std::fs::remove_file(p).is_ok()
    } else {
        true
    }
}

pub fn file_exists(file_name: &str) -> bool {
    std::path::Path::new(file_name).exists()
}

pub fn mk_dir(dir: &str) {
    let _ = std::fs::create_dir_all(dir);
}

pub fn get_file_name(path: &str, no_extension: bool) -> String {
    let p = std::path::Path::new(path);
    if no_extension {
        match p.file_stem() {
            Some(s) => s.to_string_lossy().to_string(),
            None => String::new(),
        }
    } else {
        match p.file_name() {
            Some(s) => s.to_string_lossy().to_string(),
            None => String::new(),
        }
    }
}

pub fn get_file_dir(path: &str, with_slash: bool) -> String {
    let p = std::path::Path::new(path);
    match p.parent() {
        Some(parent) => {
            let s = parent.to_string_lossy().to_string();
            if with_slash && !s.ends_with('\\') && !s.ends_with('/') {
                s + "\\"
            } else {
                s
            }
        }
        None => String::new(),
    }
}

pub fn remove_trailing_slash(directory: &str) -> String {
    let trimmed = directory.trim_end_matches(|c| c == '\\' || c == '/');
    trimmed.to_string()
}

pub fn get_cur_dir() -> String {
    std::env::current_dir()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_default()
}

pub fn get_full_path(rel_path: &str) -> String {
    let p = std::path::Path::new(rel_path);
    if p.is_absolute() {
        rel_path.to_string()
    } else {
        let cur = std::env::current_dir().unwrap_or_default();
        cur.join(rel_path).to_string_lossy().to_string()
    }
}

pub fn is_path_rooted(path: &str) -> bool {
    std::path::Path::new(path).is_absolute()
}

// Memory read/write helpers (unsafe - mirroring C++ pointer manipulation)
pub unsafe fn smem_read(src: &mut *mut u8, dst: &mut [u8]) {
    let size = dst.len();
    unsafe {
        std::ptr::copy_nonoverlapping(*src, dst.as_mut_ptr(), size);
        *src = src.add(size);
    }
}

pub unsafe fn smem_read_str(src: &mut *mut u8, the_string: &mut String) {
    let mut len: i32 = 0;
    unsafe {
        smem_read(
            src,
            std::slice::from_raw_parts_mut(&mut len as *mut i32 as *mut u8, std::mem::size_of::<i32>()),
        );
    }
    let len = len as usize;
    if len > 0 {
        unsafe {
            let bytes = std::slice::from_raw_parts(*src, len);
            *the_string = String::from_utf8_lossy(bytes).to_string();
            *src = src.add(len);
        }
    }
}

pub unsafe fn smem_write(dst: &mut *mut u8, src: &[u8]) {
    let size = src.len();
    unsafe {
        std::ptr::copy_nonoverlapping(src.as_ptr(), *dst, size);
        *dst = dst.add(size);
    }
}

pub unsafe fn smem_write_str(dst: &mut *mut u8, the_string: &str) {
    let len = the_string.len() as i32;
    unsafe {
        smem_write(
            dst,
            std::slice::from_raw_parts(&len as *const i32 as *const u8, std::mem::size_of::<i32>()),
        );
    }
    if len > 0 {
        unsafe { smem_write(dst, the_string.as_bytes()); }
    }
}

// UTF-8 helpers
pub fn utf8_decode_next(s: &str, offset: &mut usize, out_char: &mut char) -> bool {
    if *offset >= s.len() {
        return false;
    }
    let remaining = &s[*offset..];
    match remaining.chars().next() {
        Some(c) => {
            *out_char = c;
            *offset += c.len_utf8();
            true
        }
        None => false,
    }
}

pub fn utf8_next_boundary(s: &str, offset: usize) -> usize {
    if offset >= s.len() {
        return s.len();
    }
    let mut c = '\0';
    let mut next = offset;
    if utf8_decode_next(s, &mut next, &mut c) {
        return next;
    }
    offset + 1
}

pub fn utf8_prev_boundary(s: &str, offset: usize) -> usize {
    if offset == 0 || s.is_empty() {
        return 0;
    }
    let mut pos = std::cmp::min(offset, s.len()) - 1;
    while pos > 0 && (s.as_bytes()[pos] & 0xC0) == 0x80 {
        pos -= 1;
    }
    pos
}

pub fn utf8_byte_offset_for_code_point(s: &str, code_point_index: usize) -> usize {
    let mut offset = 0;
    let mut c = '\0';
    for _ in 0..code_point_index {
        if !utf8_decode_next(s, &mut offset, &mut c) {
            return s.len();
        }
    }
    offset
}

pub fn utf8_code_point_count(s: &str) -> usize {
    s.chars().count()
}

pub fn utf8_code_point_at(s: &str, offset: usize) -> char {
    if offset >= s.len() {
        return '\0';
    }
    s[offset..].chars().next().unwrap_or('\0')
}

pub fn is_opening_punctuation(c: char) -> bool {
    matches!(
        c,
        '〈' | '《' | '「' | '『' | '【' | '〔' | '〖' | '〘' | '〚' | '（' | '［' | '｛' | '\u{2018}' | '\u{201A}' | '\u{201B}' | '\u{201C}'
    )
}

pub fn is_closing_punctuation(c: char) -> bool {
    matches!(
        c,
        '〉' | '》' | '」' | '』' | '】' | '〕' | '〗' | '〙' | '〛' | '）' | '］' | '｝' | '\u{2019}' | '\u{201D}' | '、' | '。' | '，' | '．' | '！' | '？' | '：' | '；'
    )
}

// Byte swap helpers
pub const fn byte_swap16(v: u16) -> u16 {
    (v >> 8) | (v << 8)
}

pub const fn byte_swap32(v: u32) -> u32 {
    ((v & 0x000000FF) << 24)
        | ((v & 0x0000FF00) << 8)
        | ((v & 0x00FF0000) >> 8)
        | ((v & 0xFF000000) >> 24)
}

pub const fn byte_swap64(v: u64) -> u64 {
    ((v & 0x00000000000000FF) << 56)
        | ((v & 0x000000000000FF00) << 40)
        | ((v & 0x0000000000FF0000) << 24)
        | ((v & 0x00000000FF000000) << 8)
        | ((v & 0x000000FF00000000) >> 8)
        | ((v & 0x0000FF0000000000) >> 24)
        | ((v & 0x00FF000000000000) >> 40)
        | ((v & 0xFF00000000000000) >> 56)
}

pub const fn from_le16(v: u16) -> u16 {
    if cfg!(target_endian = "little") {
        v
    } else {
        byte_swap16(v)
    }
}
pub const fn to_le16(v: u16) -> u16 {
    from_le16(v)
}
pub const fn from_le32(v: u32) -> u32 {
    if cfg!(target_endian = "little") {
        v
    } else {
        byte_swap32(v)
    }
}
pub const fn to_le32(v: u32) -> u32 {
    from_le32(v)
}
pub const fn from_le64(v: u64) -> u64 {
    if cfg!(target_endian = "little") {
        v
    } else {
        byte_swap64(v)
    }
}
pub const fn to_le64(v: u64) -> u64 {
    from_le64(v)
}
pub const fn from_be16(v: u16) -> u16 {
    if cfg!(target_endian = "big") {
        v
    } else {
        byte_swap16(v)
    }
}
pub const fn to_be16(v: u16) -> u16 {
    from_be16(v)
}
pub const fn from_be32(v: u32) -> u32 {
    if cfg!(target_endian = "big") {
        v
    } else {
        byte_swap32(v)
    }
}
pub const fn to_be32(v: u32) -> u32 {
    from_be32(v)
}
pub const fn from_be64(v: u64) -> u64 {
    if cfg!(target_endian = "big") {
        v
    } else {
        byte_swap64(v)
    }
}
pub const fn to_be64(v: u64) -> u64 {
    from_be64(v)
}

// Inline trim functions
pub fn inline_l_trim(data: &mut String, chars: &str) {
    let trimmed = data.trim_start_matches(|c: char| chars.contains(c));
    *data = trimmed.to_string();
}

pub fn inline_r_trim(data: &mut String, chars: &str) {
    let trimmed = data.trim_end_matches(|c: char| chars.contains(c));
    *data = trimmed.to_string();
}

pub fn inline_trim(data: &mut String, chars: &str) {
    let trimmed = data.trim_matches(|c: char| chars.contains(c));
    *data = trimmed.to_string();
}

pub struct StringLessNoCase;

impl StringLessNoCase {
    pub fn new() -> Self {
        StringLessNoCase
    }

    pub fn compare(&self, s1: &str, s2: &str) -> bool {
        s1.to_lowercase() < s2.to_lowercase()
    }
}

// PathFromU8 / PathToU8 helpers
pub fn path_from_u8(s: &str) -> PathBuf {
    PathBuf::from(s)
}

pub fn path_to_u8(p: &PathBuf) -> String {
    p.to_string_lossy().to_string()
}

// unreachable macro
#[macro_export]
macro_rules! unreachable_stmt {
    () => {
        unsafe { std::hint::unreachable_unchecked() }
    };
}

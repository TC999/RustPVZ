// [TRANSLATION_NOTE]: ModVal.h -> Rust
// 模值修改工具（调试用）。在非调试模式下直接返回值。

pub fn mod_val_int(_the_file_name: &str, the_int: i32) -> i32 {
    the_int
}

pub fn mod_val_double(_the_file_name: &str, the_double: f64) -> f64 {
    the_double
}

pub fn mod_val_float(_the_file_name: &str, the_float: f32) -> f32 {
    the_float
}

pub fn mod_val_str<'a>(_the_file_name: &str, the_str: &'a str) -> &'a str {
    the_str
}

pub fn reparse_mod_values() -> bool {
    false
}

pub fn add_mod_val_enum(_the_enum_name: &str, _the_val: i32) {
}

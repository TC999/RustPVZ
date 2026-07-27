// [TRANSLATION_NOTE]: TodDebug.h + TodDebug.cpp -> Rust 模块
// 调试工具：日志、断言、内存分配包装

use std::sync::atomic::{AtomicU64, Ordering};

pub struct TodHesitationBracket {
    pub m_message: [u8; 256],
    pub m_bracket_start_time: i32,
}

impl TodHesitationBracket {
    pub fn new(_format: &str) -> Self {
        TodHesitationBracket {
            m_message: [0u8; 256],
            m_bracket_start_time: 0,
        }
    }

    pub fn end_bracket(&mut self) {}
}

impl Drop for TodHesitationBracket {
    fn drop(&mut self) {}
}

static mut G_LOG_FILE_NAME: [u8; 512] = [0u8; 512];
static mut G_DEBUG_DATA_FOLDER: [u8; 512] = [0u8; 512];

pub fn tod_error_message_box(the_message: &str, the_title: &str) {
    panic!("Error Box\n--{}--\n{}", the_title, the_message);
}

pub fn tod_trace_memory() {}

pub fn tod_malloc(the_size: i32) -> *mut u8 {
    debug_assert!(the_size > 0);
    if the_size <= 0 {
        return std::ptr::null_mut();
    }
    let layout = std::alloc::Layout::from_size_align(the_size as usize, 1).unwrap();
    unsafe { std::alloc::alloc(layout) }
}

pub fn tod_free(the_block: *mut u8) {
    if !the_block.is_null() {
        unsafe {
            let layout = std::alloc::Layout::from_size_align(1, 1).unwrap();
            std::alloc::dealloc(the_block, layout);
        }
    }
}

pub fn tod_assert_failed(the_condition: &str, the_file: &str, the_line: u32, the_msg: &str) {
    let a_buffer;
    if !the_condition.is_empty() {
        a_buffer = format!("\n{}({})\nassertion failed: '{}'\n{}", the_file, the_line, the_condition, the_msg);
    } else {
        a_buffer = format!("\n{}({})\nassertion failed: {}", the_file, the_line, the_msg);
    }

    tod_trace("%s", &a_buffer);
    tod_error_message_box(&a_buffer, "Assertion failed");
}

pub fn _tod_assert(condition: bool, file: &str, line: u32, msg: &str) {
    if !condition {
        tod_assert_failed("", file, line, msg);
    }
}

pub fn tod_log_ln(the_format: &str) {
    if !the_format.is_empty() {
        tod_log_string_ln(the_format);
    }
}

pub fn tod_log_string_ln(the_msg: &str) {
    #[cfg(debug_assertions)]
    {
        let log_filename = unsafe {
            let len = G_LOG_FILE_NAME.iter().position(|&c| c == 0).unwrap_or(512);
            std::str::from_utf8(&G_LOG_FILE_NAME[..len]).unwrap_or("")
        };
        if !log_filename.is_empty() {
            use std::io::Write;
            if let Ok(mut f) = std::fs::OpenOptions::new()
                .append(true)
                .create(true)
                .open(log_filename)
            {
                let _ = writeln!(f, "{}", the_msg);
            }
        }
    }
}

pub fn tod_trace(the_format: &str, _arg: &str) {
    if !the_format.is_empty() {
        print!("{}", _arg);
    }
}

pub fn tod_trace_and_log_ln(the_format: &str) {
    if the_format.is_empty() {
        return;
    }

    print!("{}", the_format);
    tod_log_string_ln(the_format);
}

pub fn tod_trace_without_spamming(the_format: &str) {
    static mut G_LAST_TRACE_TIME: AtomicU64 = AtomicU64::new(0);
    let a_time = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    let g_last = unsafe { G_LAST_TRACE_TIME.load(Ordering::Relaxed) };
    if a_time < g_last {
        return;
    }
    unsafe { G_LAST_TRACE_TIME.store(a_time, Ordering::Relaxed) };

    if !the_format.is_empty() {
        print!("{}", the_format);
    }
}

pub fn tod_assert_init_for_app() {
    let a_relative_user_path = crate::sexy_app_framework::common::get_app_data_path("userdata/");
    crate::sexy_app_framework::common::mk_dir(&crate::sexy_app_framework::common::get_app_data_path("userdata"));
    unsafe {
        let bytes = a_relative_user_path.as_bytes();
        let len = bytes.len().min(511);
        G_DEBUG_DATA_FOLDER[..len].copy_from_slice(&bytes[..len]);
        G_DEBUG_DATA_FOLDER[len] = 0;

        let log_path = format!("{}log.txt", a_relative_user_path);
        let log_bytes = log_path.as_bytes();
        let log_len = log_bytes.len().min(511);
        G_LOG_FILE_NAME[..log_len].copy_from_slice(&log_bytes[..log_len]);
        G_LOG_FILE_NAME[log_len] = 0;
    }

    let start_time = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    tod_log_ln(&format!("Started {}", start_time));
}

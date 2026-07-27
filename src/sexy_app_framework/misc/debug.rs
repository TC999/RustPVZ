// [TRANSLATION_NOTE]: Debug.h -> Rust
// 调试断言宏的 Rust 等价

pub static mut G_IN_ASSERT: bool = false;

#[macro_export]
macro_rules! dbg_asserte {
    ($exp:expr) => {
        if cfg!(debug_assertions) {
            unsafe { $crate::sexy_app_framework::misc::debug::G_IN_ASSERT = true; }
            debug_assert!($exp);
            unsafe { $crate::sexy_app_framework::misc::debug::G_IN_ASSERT = false; }
        }
    };
}

#[macro_export]
macro_rules! dbg_assert {
    ($exp:expr) => {
        if cfg!(debug_assertions) {
            unsafe { $crate::sexy_app_framework::misc::debug::G_IN_ASSERT = true; }
            debug_assert!($exp);
            unsafe { $crate::sexy_app_framework::misc::debug::G_IN_ASSERT = false; }
        }
    };
}

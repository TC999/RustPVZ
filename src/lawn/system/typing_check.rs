// [TRANSLATION_NOTE]: TypingCheck.h + TypingCheck.cpp -> Rust 模块
// 键盘输入短语检查工具

use crate::sexy_app_framework::misc::key_codes::{get_key_code_from_name, KEYCODE_UNKNOWN};

pub struct TypingCheck {
    m_phrase: String,
    m_recent_typing: String,
}

impl TypingCheck {
    pub fn new() -> Self {
        TypingCheck {
            m_phrase: String::new(),
            m_recent_typing: String::new(),
        }
    }

    pub fn with_phrase(the_phrase: &str) -> Self {
        let mut check = TypingCheck::new();
        check.set_phrase(the_phrase);
        check
    }

    pub fn set_phrase(&mut self, the_phrase: &str) {
        self.m_phrase.clear();
        for c in the_phrase.chars() {
            self.add_char(c);
        }
    }

    pub fn add_key_code(&mut self, the_key_code: u32) {
        self.m_phrase.push(unsafe { std::char::from_u32_unchecked(the_key_code) });
    }

    pub fn add_char(&mut self, the_char: char) {
        let lower = the_char.to_lowercase().next().unwrap_or(the_char);
        let a_char_string: String = lower.to_string();
        let key_code = get_key_code_from_name(&a_char_string);
        if key_code != KEYCODE_UNKNOWN {
            self.add_key_code(key_code);
        }
    }

    pub fn check(&mut self) -> bool {
        if self.m_recent_typing == self.m_phrase {
            self.m_recent_typing.clear();
            return true;
        }
        false
    }

    pub fn check_key(&mut self, the_key_code: u32) -> bool {
        self.m_recent_typing.push(unsafe { std::char::from_u32_unchecked(the_key_code) });
        let a_length = self.m_phrase.len();
        if a_length == 0 {
            return false;
        }

        if self.m_recent_typing.len() > a_length {
            self.m_recent_typing = self.m_recent_typing.chars().skip(1).take(a_length).collect();
        }

        self.check()
    }
}

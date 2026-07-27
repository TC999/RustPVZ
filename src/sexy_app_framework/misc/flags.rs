// [TRANSLATION_NOTE]: Flags.h -> Rust
// 标志位修饰工具：FlagsMod, ModalFlags, AutoModalFlags

#[derive(Clone, Copy, Debug)]
pub struct FlagsMod {
    pub m_add_flags: i32,
    pub m_remove_flags: i32,
}

impl FlagsMod {
    pub fn new() -> Self {
        FlagsMod {
            m_add_flags: 0,
            m_remove_flags: 0,
        }
    }
}

pub fn mod_flags(the_flags: &mut i32, the_flag_mod: &FlagsMod) {
    *the_flags = (*the_flags | the_flag_mod.m_add_flags) & !the_flag_mod.m_remove_flags;
}

pub fn get_mod_flags(the_flags: i32, the_flag_mod: &FlagsMod) -> i32 {
    (the_flags | the_flag_mod.m_add_flags) & !the_flag_mod.m_remove_flags
}

pub struct ModalFlags {
    pub m_over_flags: i32,
    pub m_under_flags: i32,
    pub m_is_over: bool,
}

impl ModalFlags {
    pub fn new() -> Self {
        ModalFlags {
            m_over_flags: 0,
            m_under_flags: 0,
            m_is_over: false,
        }
    }

    pub fn mod_flags_self(&mut self, the_flags_mod: &FlagsMod) {
        mod_flags(&mut self.m_over_flags, the_flags_mod);
        mod_flags(&mut self.m_under_flags, the_flags_mod);
    }

    pub fn get_flags(&self) -> i32 {
        if self.m_is_over { self.m_over_flags } else { self.m_under_flags }
    }
}

pub struct AutoModalFlags<'a> {
    m_modal_flags: &'a mut ModalFlags,
    m_old_over_flags: i32,
    m_old_under_flags: i32,
}

impl<'a> AutoModalFlags<'a> {
    pub fn new(the_modal_flags: &'a mut ModalFlags, the_flag_mod: &FlagsMod) -> Self {
        let old_over = the_modal_flags.m_over_flags;
        let old_under = the_modal_flags.m_under_flags;
        the_modal_flags.mod_flags_self(the_flag_mod);
        AutoModalFlags {
            m_modal_flags: the_modal_flags,
            m_old_over_flags: old_over,
            m_old_under_flags: old_under,
        }
    }
}

impl<'a> Drop for AutoModalFlags<'a> {
    fn drop(&mut self) {
        self.m_modal_flags.m_over_flags = self.m_old_over_flags;
        self.m_modal_flags.m_under_flags = self.m_old_under_flags;
    }
}

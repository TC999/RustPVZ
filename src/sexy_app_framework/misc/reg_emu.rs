// [TRANSLATION_NOTE]: RegEmu.h + RegEmu.cpp -> Rust
// 简易 Windows 注册表模拟器

use std::collections::HashMap;
use std::io::{Read, Write};
use std::sync::Mutex;

const REGEMU_VERSION: u16 = 1;

#[derive(Clone, Default)]
struct RegValue {
    m_type: u32,
    m_length: u32,
    m_value: Vec<u8>,
}

static REGISTRY: Mutex<Option<RegState>> = Mutex::new(None);

struct RegState {
    contents: HashMap<String, HashMap<String, RegValue>>,
    curr_file: String,
}

pub fn set_reg_file(file_name: &str) {
    let mut reg = REGISTRY.lock().unwrap();
    let mut contents = HashMap::new();

    // 尝试读取文件
    if let Ok(mut f) = std::fs::File::open(file_name) {
        let mut header = [0u8; 6];
        if f.read_exact(&mut header).is_ok() && &header == b"REGEMU" {
            let mut ver_buf = [0u8; 2];
            if f.read_exact(&mut ver_buf).is_ok() {
                let _version = u16::from_le_bytes(ver_buf);
                let mut num_keys_buf = [0u8; 4];
                if f.read_exact(&mut num_keys_buf).is_ok() {
                    let num_keys = u32::from_le_bytes(num_keys_buf);
                    for _ in 0..num_keys {
                        let mut key_len_buf = [0u8; 4];
                        if f.read_exact(&mut key_len_buf).is_err() { break; }
                        let key_len = u32::from_le_bytes(key_len_buf) as usize;
                        let mut key_name = vec![0u8; key_len];
                        if f.read_exact(&mut key_name).is_err() { break; }
                        let key_str = String::from_utf8_lossy(&key_name[..key_name.len().saturating_sub(1)]).to_string();
                        
                        let mut num_vals_buf = [0u8; 4];
                        if f.read_exact(&mut num_vals_buf).is_err() { break; }
                        let num_vals = u32::from_le_bytes(num_vals_buf);
                        
                        let mut values = HashMap::new();
                        for _ in 0..num_vals {
                            let mut val_name_len_buf = [0u8; 4];
                            if f.read_exact(&mut val_name_len_buf).is_err() { break; }
                            let val_name_len = u32::from_le_bytes(val_name_len_buf) as usize;
                            let mut val_name = vec![0u8; val_name_len];
                            if f.read_exact(&mut val_name).is_err() { break; }
                            let val_str = String::from_utf8_lossy(&val_name[..val_name.len().saturating_sub(1)]).to_string();
                            
                            let mut rv = RegValue::default();
                            let mut type_buf = [0u8; 4];
                            if f.read_exact(&mut type_buf).is_err() { break; }
                            rv.m_type = u32::from_le_bytes(type_buf);
                            let mut len_buf = [0u8; 4];
                            if f.read_exact(&mut len_buf).is_err() { break; }
                            rv.m_length = u32::from_le_bytes(len_buf);
                            rv.m_value.resize(rv.m_length as usize, 0);
                            if f.read_exact(&mut rv.m_value).is_err() { break; }
                            
                            values.insert(val_str, rv);
                        }
                        contents.insert(key_str, values);
                    }
                }
            }
        }
    }

    *reg = Some(RegState {
        contents,
        curr_file: file_name.to_string(),
    });
}

fn save_to_file(state: &RegState) -> bool {
    if state.curr_file.is_empty() {
        println!("RegEmu: Filename not specified, can't save");
        return false;
    }

    match std::fs::File::create(&state.curr_file) {
        Ok(mut f) => {
            let _ = f.write_all(b"REGEMU");
            let _ = f.write_all(&REGEMU_VERSION.to_le_bytes());
            let num_keys = state.contents.len() as u32;
            let _ = f.write_all(&num_keys.to_le_bytes());

            for (key_name, values) in &state.contents {
                let key_len = key_name.len() as u32 + 1;
                let _ = f.write_all(&key_len.to_le_bytes());
                let _ = f.write_all(key_name.as_bytes());
                let _ = f.write_all(&[0u8]);

                let num_vals = values.len() as u32;
                let _ = f.write_all(&num_vals.to_le_bytes());

                for (val_name, rv) in values {
                    let val_name_len = val_name.len() as u32 + 1;
                    let _ = f.write_all(&val_name_len.to_le_bytes());
                    let _ = f.write_all(val_name.as_bytes());
                    let _ = f.write_all(&[0u8]);
                    let _ = f.write_all(&rv.m_type.to_le_bytes());
                    let _ = f.write_all(&rv.m_length.to_le_bytes());
                    let _ = f.write_all(&rv.m_value);
                }
            }
            true
        }
        Err(_) => false,
    }
}

pub const REGEMU_NONE: u32 = 0;
pub const REGEMU_SZ: u32 = 1;
pub const REGEMU_EXPAND_SZ: u32 = 2;
pub const REGEMU_BINARY: u32 = 3;
pub const REGEMU_DWORD: u32 = 4;
pub const REGEMU_DWORD_LITTLE_ENDIAN: u32 = 4;
pub const REGEMU_DWORD_BIG_ENDIAN: u32 = 5;
pub const REGEMU_MULTI_SZ: u32 = 7;
pub const REGEMU_QWORD: u32 = 11;
pub const REGEMU_QWORD_LITTLE_ENDIAN: u32 = 11;

pub fn registry_read(key_name: &str, value_name: &str) -> Option<(u32, Vec<u8>)> {
    let reg = REGISTRY.lock().unwrap();
    if let Some(ref state) = *reg {
        if let Some(values) = state.contents.get(key_name) {
            if let Some(rv) = values.get(value_name) {
                return Some((rv.m_type, rv.m_value.clone()));
            }
        }
    }
    None
}

pub fn registry_write(key_name: &str, value_name: &str, typ: u32, value: &[u8]) -> bool {
    let mut reg = REGISTRY.lock().unwrap();
    if let Some(ref mut state) = *reg {
        let rv = RegValue {
            m_type: typ,
            m_length: value.len() as u32,
            m_value: value.to_vec(),
        };
        state.contents.entry(key_name.to_string())
            .or_insert_with(HashMap::new)
            .insert(value_name.to_string(), rv);
        save_to_file(state)
    } else {
        false
    }
}

pub fn registry_erase_key(key_name: &str) -> bool {
    let mut reg = REGISTRY.lock().unwrap();
    if let Some(ref mut state) = *reg {
        if state.contents.remove(key_name).is_some() {
            save_to_file(state)
        } else {
            false
        }
    } else {
        false
    }
}

pub fn registry_erase_value(key_name: &str, value_name: &str) -> bool {
    let mut reg = REGISTRY.lock().unwrap();
    if let Some(ref mut state) = *reg {
        if let Some(values) = state.contents.get_mut(key_name) {
            if values.remove(value_name).is_some() {
                save_to_file(state)
            } else {
                false
            }
        } else {
            false
        }
    } else {
        false
    }
}

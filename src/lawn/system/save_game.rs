// [TRANSLATION_NOTE]: SaveGame.h + SaveGame.cpp -> Rust 翻译
// 游戏存档加载与保存。SaveGame.cpp 有 128KB，此处为核心接口 + stub

#![allow(non_snake_case, dead_code)]

use crate::lawn::board::Board;

/// 从文件加载游戏存档
pub fn LawnLoadGame(theBoard: *mut Board, theFilePath: &str) -> bool {
    let _ = theBoard;
    let _ = theFilePath;
    // TODO: 实现完整的游戏状态反序列化
    false
}

/// 将游戏存档保存到文件
pub fn LawnSaveGame(theBoard: *mut Board, theFilePath: &str) -> bool {
    let _ = theBoard;
    let _ = theFilePath;
    // TODO: 实现完整的游戏状态序列化
    false
}

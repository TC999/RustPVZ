# RustPVZ

Rust 移植版 Plants vs. Zombies GOTY (PvZ-Portable)

## 项目状态

**C++ → Rust 翻译工程 — 100% 完成**

将 [PvZ-Portable](https://github.com/wszqkzqk/PvZ-Portable) 的 C++ 源码按 1:1 保真度翻译为 Rust。

| 编译状态 | Errors | Warnings |
|:--------:|:-----:|:--------:|
| ✅ 通过  | 0     | 0        |

### 已翻译模块

| 模块 | 状态 |
|:----|:----:|
| Plant 系统 | ✅ 数据表 + 战斗逻辑 + 绘制 |
| Zombie 系统 | ✅ 数据表 + AI + 绘制 |
| Board 游戏板 | ✅ 更新循环 + 波次生成 + 绘制 |
| LawnApp 控制器 | ✅ 47 接口方法 + 关卡流程 |
| Projectile 弹丸 | ✅ 飞行 + 碰撞 + 伤害 |
| Coin/LawnMower/GridItem | ✅ 完整实现 |
| Challenge/CutScene/ZenGarden | ✅ 完整框架 |
| Graphics/Reanimation/Foley | ✅ 底层渲染 + 动画 + 音效 |
| TodCommon/TodFoley | ✅ 工具函数 + 音效参数表 |
| UI 控件 | ✅ 20+ 对话框和按钮组件 |

### 构建

```bash
cargo build --release
cargo check    # 零错误零警告
```

# Sui Rust Wallet

轻量级的 Sui PC端钱包，用 Rust 编写，演示如何与 Sui Devnet、Testnet 和 Mainnet 交互。支持导入私钥、查看余额和刷新余额等功能。本钱包没进过全面的安全测试，请勿用来和Mainnet交互，仅用于研究学习。

## 功能

- 导入私钥（支持 Bech32 或 Base64 格式）
- 查看 SUI 代币余额
- 切换网络（Devnet、Testnet、Mainnet）
- 异步余额刷新
- 简单的 GUI 界面（使用 egui）
## 快速开始

### 先决条件
- Rust（推荐使用 rustup 安装的 stable toolchain）
- Cargo

### 克隆与构建
克隆后在项目根目录运行：

构建：
```sh
cargo build
```

运行：
```sh
cargo run
```

### 环境变量
可通过环境变量覆盖默认网络 endpoint：
- SUI_DEVNET_ENDPOINT — 指定 Devnet RPC/HTTP endpoint（默认：https://fullnode.devnet.sui.io:443）
- SUI_TESTNET_ENDPOINT — 指定 Testnet RPC/HTTP endpoint（默认：https://fullnode.testnet.sui.io:443）
- SUI_MAINNET_ENDPOINT — 指定 Mainnet RPC/HTTP endpoint（默认：https://fullnode.mainnet.sui.io:443）

## 项目结构

- [Cargo.toml](Cargo.toml) — 依赖与构建配置
- [src/main.rs](src/main.rs) — 程序入口（函数：[`main`](src/main.rs)）
- [src/controller.rs](src/controller.rs) — 控制器逻辑（包括 [`handle_import_key`](src/controller.rs)、[`handle_refresh_balance`](src/controller.rs) 等）
- [src/model.rs](src/model.rs) — 数据模型（包括 [`Network`](src/model.rs) 枚举和 [`Model`](src/model.rs) 结构体）
- [src/view.rs](src/view.rs) — GUI 视图逻辑

## 依赖

主要依赖：
- sui-sdk — Sui 区块链 SDK
- tokio — 异步运行时
- egui — GUI 框架

## 常见命令

运行测试：
```sh
cargo test
```

格式化代码：
```sh
cargo fmt
```

检查依赖安全：
```sh
cargo audit
```

## 程序打包

### 1制作图标
 下面是制作 macOS .icns 的常用方法（推荐使用系统自带的 iconutil + sips），以及可选的 ImageMagick 方案和注意事项。

步骤概览

准备一张方形高分辨率源图（推荐 1024×1024 PNG，带透明通道）。
生成 .iconset 目录并把不同尺寸的 PNG 放进去（按 macOS 要求命名）。
用 iconutil 将 .iconset 打包为 .icns。
终端脚本：icon.sh（macOS，假设源文件 icon_1024.png）
```sh
#!/bin/bash
# 生成 icns 的示例脚本
SRC="icon_1024.png"
ICONSET="AppIcon.iconset"
OUT_ICNS="MyApp.icns"

# 清理并创建 iconset 目录
rm -rf "$ICONSET"
mkdir -p "$ICONSET"

# 使用 sips 生成各尺寸（sips -z 高 宽）
sips -z 16 16    "$SRC" --out "$ICONSET/icon_16x16.png"
sips -z 32 32    "$SRC" --out "$ICONSET/icon_16x16@2x.png"
sips -z 32 32    "$SRC" --out "$ICONSET/icon_32x32.png"
sips -z 64 64    "$SRC" --out "$ICONSET/icon_32x32@2x.png"
sips -z 128 128  "$SRC" --out "$ICONSET/icon_128x128.png"
sips -z 256 256  "$SRC" --out "$ICONSET/icon_128x128@2x.png"
sips -z 256 256  "$SRC" --out "$ICONSET/icon_256x256.png"
sips -z 512 512  "$SRC" --out "$ICONSET/icon_256x256@2x.png"
sips -z 512 512  "$SRC" --out "$ICONSET/icon_512x512.png"
# 1024x1024 为 @2x 的 512
sips -z 1024 1024 "$SRC" --out "$ICONSET/icon_512x512@2x.png"

# 打包为 icns
iconutil -c icns "$ICONSET" -o "$OUT_ICNS"

# 可选：删除临时目录
rm -rf "$ICONSET"

echo "生成: $OUT_ICNS"
```
### 2安装工具（若未安装）：
```
cargo install cargo-bundle
```
从项目根运行（无需额外参数）：
```
cargo bundle --release
```
生成结果通常在：

target/release/bundle/osx/ 下包含 Sui Rust Wallet.app

## 使用说明

1. 启动应用后，输入私钥导入钱包。
2. 选择网络（默认 Devnet）。
3. 点击“Refresh Balance”查看 SUI 余额。
4. 使用“Logout”登出钱包。

## 贡献

欢迎提交 issue 或 PR。请保持代码风格一致并添加简要说明。



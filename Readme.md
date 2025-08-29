# Sui Rust Wallet

轻量级的 Sui PC钱包示例，用 Rust 编写，演示如何与 Sui Devnet、Testnet 和 Mainnet 交互。支持导入私钥、查看余额和刷新余额等功能。

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

## 使用说明

1. 启动应用后，输入私钥导入钱包。
2. 选择网络（默认 Devnet）。
3. 点击“Refresh Balance”查看 SUI 余额。
4. 使用“Logout”登出钱包。

## 贡献

欢迎提交 issue 或 PR。请保持代码风格一致并添加简要说明。



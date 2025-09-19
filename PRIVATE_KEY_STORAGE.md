# 私钥加密存储功能

## 功能概述

本功能实现了导入的私钥的自动加密保存，用户退出后再登录不需要重复导入私钥。

## 核心特性

### 🔐 安全性
- **AES-GCM 加密**: 使用 AES-GCM 算法对私钥进行对称加密
- **Argon2 密钥派生**: 使用 Argon2 算法从用户密码派生加密密钥
- **Base64 编码**: 加密后的数据使用 Base64 编码存储
- **会话密码管理**: 在用户会话期间临时存储密码用于加密操作
- **自动清理**: 用户登出时自动清理会话中的敏感数据

### 🔄 自动化流程
- **自动保存**: 私钥导入成功后，如果用户已认证，自动保存加密的私钥
- **自动加载**: 用户登录成功后，自动尝试加载并解密保存的私钥
- **无缝体验**: 整个过程对用户透明，无需额外操作

## 实现细节

### 文件结构
```
src/
├── model/
│   └── auth_model.rs          # 认证模型，包含加密存储逻辑
├── controller/
│   ├── auth_controller.rs     # 认证控制器，处理登录时自动加载
│   └── wallet_controller.rs   # 钱包控制器，处理导入时自动保存
```

### 核心方法

#### AuthState::save_encrypted_private_key()
```rust
pub fn save_encrypted_private_key(&self, private_key: &str, password: &str) -> Result<(), String>
```
- 使用用户密码派生加密密钥
- 对私钥进行 AES-GCM 加密
- 将加密结果保存到本地文件

#### AuthState::load_encrypted_private_key()
```rust
pub fn load_encrypted_private_key(&self, password: &str) -> Result<Option<String>, String>
```
- 从本地文件读取加密的私钥
- 使用用户密码派生解密密钥
- 解密并返回原始私钥

#### AuthState::set_session_password() & get_session_password()
```rust
pub fn set_session_password(&mut self, password: String)
pub fn get_session_password(&self) -> Option<&str>
```
- 在用户会话期间临时存储密码
- 用于私钥加密/解密操作
- 登出时自动清理

### 存储位置
- 加密私钥文件: `encrypted_private_key.dat`
- 存储位置: 与密码文件相同目录
- 格式: Base64 编码的加密数据

## 使用流程

### 首次使用
1. 用户设置认证密码
2. 用户导入私钥
3. 系统自动使用认证密码加密并保存私钥

### 再次使用
1. 用户输入认证密码登录
2. 系统自动加载并解密保存的私钥
3. 钱包状态恢复，无需重新导入

## 安全考虑

### 密码管理
- 会话密码仅在内存中临时存储
- 用户登出时立即清理会话密码
- 不在磁盘上明文存储任何密码

### 加密强度
- AES-GCM: 提供认证加密，防止篡改
- Argon2: 抗暴力破解的密钥派生函数
- 随机 nonce: 每次加密使用不同的随机数

### 错误处理
- 加密/解密失败不影响主要功能
- 详细的错误日志便于调试
- 优雅降级，保证用户体验

## 依赖库

```toml
[dependencies]
aes-gcm = "0.10"      # AES-GCM 加密
argon2 = "0.5"        # 密钥派生
base64 = "0.22"       # Base64 编码
```

## 国际化支持

新增的国际化键：
- `private_key_auto_loaded`: "私钥已自动加载"
- `private_key_auto_saved`: "私钥已自动保存"
- `private_key_load_failed`: "私钥加载失败"
- `private_key_save_failed`: "私钥保存失败"

## 测试建议

1. **基本流程测试**:
   - 设置密码 → 导入私钥 → 登出 → 登录 → 验证私钥自动加载

2. **错误处理测试**:
   - 错误密码尝试
   - 加密文件损坏
   - 权限问题

3. **安全性测试**:
   - 内存泄漏检查
   - 临时文件清理
   - 会话状态管理

## 注意事项

- 此功能需要用户首先设置认证密码
- 私钥的安全性依赖于用户设置的密码强度
- 建议用户设置强密码以确保私钥安全
- 如果忘记密码，需要重新导入私钥
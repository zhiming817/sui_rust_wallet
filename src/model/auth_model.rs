use std::{fs, path::PathBuf};
use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use argon2::password_hash::rand_core::OsRng;
use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng as AesOsRng},
    Aes256Gcm, Nonce, Key
};
use base64::{Engine as _, engine::general_purpose};

/// 认证状态
#[derive(Debug, Clone)]
pub struct AuthState {
    pub is_authenticated: bool,
    pub is_first_run: bool,
    pub password_input: String,
    pub password_confirm: String,
    pub password_hash: Option<String>,
    pub password_file: PathBuf,
    pub session_timeout: Option<std::time::Instant>,
    // 私钥加密存储相关
    pub encrypted_private_key_file: PathBuf,
    // 会话中的临时密码（仅用于私钥加密保存）
    session_password: Option<String>,
}

impl AuthState {
    /// 创建新的认证状态
    pub fn new() -> Self {
        let mut cfg_dir = dirs::config_dir().unwrap_or_else(|| std::env::current_dir().unwrap());
        cfg_dir.push("sui_rust_wallet");
        let mut password_file = cfg_dir.clone();
        password_file.push("password.hash");
        
        let mut encrypted_private_key_file = cfg_dir.clone();
        encrypted_private_key_file.push("private_key.enc");

        let (is_first_run, password_hash) = match fs::read_to_string(&password_file) {
            Ok(s) if !s.trim().is_empty() => (false, Some(s)),
            _ => (true, None),
        };

        Self {
            is_authenticated: false,
            is_first_run,
            password_input: String::new(),
            password_confirm: String::new(),
            password_hash,
            password_file,
            session_timeout: None,
            encrypted_private_key_file,
            session_password: None,
        }
    }

    /// 检查是否需要首次设置
    pub fn needs_setup(&self) -> bool {
        self.is_first_run
    }

    /// 检查是否已认证
    pub fn is_authenticated(&self) -> bool {
        self.is_authenticated && !self.is_session_expired()
    }

    /// 设置密码
    pub fn set_password(&mut self, i18n: &crate::i18n::I18nManager) -> Result<(), String> {
        let pw = self.password_input.trim();
        let pwc = self.password_confirm.trim();
        
        if pw.is_empty() {
            return Err(i18n.tr("password_empty_error"));
        }
        
        if pw != pwc {
            return Err(i18n.tr("password_mismatch_error"));
        }

        // 生成 salt 并计算 hash（argon2）
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = argon2
            .hash_password(pw.as_bytes(), &salt)
            .map_err(|e| i18n.tr("hash_error").replace("{}", &e.to_string()))?
            .to_string();

        // 确保存储目录存在并写入
        if let Some(parent) = self.password_file.parent() {
            if let Err(e) = fs::create_dir_all(parent) {
                return Err(i18n.tr("create_dir_error").replace("{}", &e.to_string()));
            }
        }
        
        fs::write(&self.password_file, &password_hash)
            .map_err(|e| i18n.tr("write_error").replace("{}", &e.to_string()))?;

        self.password_hash = Some(password_hash);
        self.is_first_run = false;
        self.is_authenticated = true;
        self.password_input.clear();
        self.password_confirm.clear();
        Ok(())
    }

    /// 验证密码
    pub fn verify_password(&mut self, attempt: &str, i18n: &crate::i18n::I18nManager) -> Result<bool, String> {
        let stored = match &self.password_hash {
            Some(h) => h.clone(),
            None => {
                // 尝试从文件读取（兜底）
                match fs::read_to_string(&self.password_file) {
                    Ok(s) if !s.trim().is_empty() => {
                        self.password_hash = Some(s.clone());
                        s
                    }
                    _ => return Err(i18n.tr("password_not_found_error")),
                }
            }
        };

        let parsed = PasswordHash::new(&stored)
            .map_err(|e| i18n.tr("parse_hash_error").replace("{}", &e.to_string()))?;
        let argon2 = Argon2::default();
        match argon2.verify_password(attempt.as_bytes(), &parsed) {
            Ok(()) => {
                self.is_authenticated = true;
                // 保存会话密码用于私钥加密
                self.session_password = Some(attempt.to_string());
                Ok(true)
            }
            Err(_) => Ok(false),
        }
    }

    /// 获取会话密码（用于私钥加密）
    pub fn get_session_password(&self) -> Option<&str> {
        self.session_password.as_deref()
    }

    /// 设置会话密码
    pub fn set_session_password(&mut self, password: String) {
        self.session_password = Some(password);
    }

    /// 清除会话密码
    pub fn clear_session_password(&mut self) {
        self.session_password = None;
    }

    /// 检查会话是否过期
    pub fn is_session_expired(&self) -> bool {
        if let Some(timeout) = self.session_timeout {
            std::time::Instant::now() > timeout
        } else {
            false
        }
    }

    /// 设置会话超时（分钟）
    pub fn set_session_timeout(&mut self, minutes: u64) {
        self.session_timeout = Some(
            std::time::Instant::now() + std::time::Duration::from_secs(minutes * 60)
        );
    }

    /// 清除会话超时
    pub fn clear_session_timeout(&mut self) {
        self.session_timeout = None;
    }

    /// 延长会话
    pub fn extend_session(&mut self, minutes: u64) {
        self.set_session_timeout(minutes);
    }

    /// 清除密码输入
    pub fn clear_password_inputs(&mut self) {
        self.password_input.clear();
        self.password_confirm.clear();
    }

    /// 登出
    pub fn logout(&mut self) {
        self.is_authenticated = false;
        self.clear_password_inputs();
        self.clear_session_timeout();
        self.clear_session_password();
    }

    /// 获取密码文件路径
    pub fn password_file_path(&self) -> &PathBuf {
        &self.password_file
    }

    /// 检查密码文件是否存在
    pub fn password_file_exists(&self) -> bool {
        self.password_file.exists()
    }

    /// 保存加密的私钥
    pub fn save_encrypted_private_key(&self, private_key: &str, password: &str) -> Result<(), String> {
        // 使用密码生成加密密钥
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        
        // 生成 32 字节的密钥用于 AES-256
        let mut key_bytes = [0u8; 32];
        argon2.hash_password_into(password.as_bytes(), salt.as_str().as_bytes(), &mut key_bytes)
            .map_err(|e| format!("Failed to derive key: {}", e))?;
        
        let key = Key::<Aes256Gcm>::from_slice(&key_bytes);
        let cipher = Aes256Gcm::new(key);
        
        // 生成随机 nonce
        let nonce = Aes256Gcm::generate_nonce(&mut AesOsRng);
        
        // 加密私钥
        let ciphertext = cipher.encrypt(&nonce, private_key.as_bytes())
            .map_err(|e| format!("Encryption failed: {}", e))?;
        
        // 组合数据：salt字符串 + "|" + nonce + ciphertext（base64编码）
        let salt_str = salt.as_str();
        let mut payload = Vec::new();
        payload.extend_from_slice(&nonce);
        payload.extend_from_slice(&ciphertext);
        let payload_b64 = general_purpose::STANDARD.encode(payload);
        
        let data = format!("{}|{}", salt_str, payload_b64);
        
        // 确保存储目录存在并写入
        if let Some(parent) = self.encrypted_private_key_file.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| format!("Failed to create directory: {}", e))?;
        }
        
        fs::write(&self.encrypted_private_key_file, data)
            .map_err(|e| format!("Failed to write encrypted private key: {}", e))?;
        
        Ok(())
    }

    /// 加载并解密私钥
    pub fn load_encrypted_private_key(&self, password: &str) -> Result<Option<String>, String> {
        // 检查文件是否存在
        if !self.encrypted_private_key_file.exists() {
            return Ok(None);
        }
        
        // 读取文件
        let file_data = fs::read_to_string(&self.encrypted_private_key_file)
            .map_err(|e| format!("Failed to read encrypted private key file: {}", e))?;
        
        // 解析格式：salt|payload_b64
        let parts: Vec<&str> = file_data.trim().split('|').collect();
        if parts.len() != 2 {
            return Err("Invalid encrypted data format".to_string());
        }
        
        let salt_str = parts[0];
        let payload_b64 = parts[1];
        
        // 重构 salt
        let salt = SaltString::from_b64(salt_str)
            .map_err(|e| format!("Invalid salt format: {}", e))?;
        
        // 解码 payload
        let payload = general_purpose::STANDARD.decode(payload_b64)
            .map_err(|e| format!("Failed to decode payload: {}", e))?;
        
        // payload 应该至少包含 nonce(12字节) + ciphertext(最少16字节)
        if payload.len() < 28 {
            return Err("Invalid payload format".to_string());
        }
        
        // 提取 nonce (前12字节)
        let nonce_bytes = &payload[0..12];
        let nonce = Nonce::from_slice(nonce_bytes);
        
        // 提取密文
        let ciphertext = &payload[12..];
        
        // 使用密码和 salt 重新生成密钥
        let argon2 = Argon2::default();
        let mut key_bytes = [0u8; 32];
        argon2.hash_password_into(password.as_bytes(), salt.as_str().as_bytes(), &mut key_bytes)
            .map_err(|e| format!("Failed to derive key: {}", e))?;
        
        let key = Key::<Aes256Gcm>::from_slice(&key_bytes);
        let cipher = Aes256Gcm::new(key);
        
        // 解密
        let plaintext = cipher.decrypt(nonce, ciphertext)
            .map_err(|_| "Failed to decrypt private key (wrong password?)".to_string())?;
        
        let private_key = String::from_utf8(plaintext)
            .map_err(|e| format!("Invalid UTF-8 in decrypted data: {}", e))?;
        
        Ok(Some(private_key))
    }

    /// 检查是否有保存的加密私钥
    pub fn has_encrypted_private_key(&self) -> bool {
        self.encrypted_private_key_file.exists()
    }

    /// 删除保存的加密私钥
    pub fn delete_encrypted_private_key(&self) -> Result<(), String> {
        if self.encrypted_private_key_file.exists() {
            fs::remove_file(&self.encrypted_private_key_file)
                .map_err(|e| format!("Failed to delete encrypted private key: {}", e))?;
        }
        Ok(())
    }
}

impl Default for AuthState {
    fn default() -> Self {
        Self::new()
    }
}

/// 认证管理器
pub struct AuthManager;

impl AuthManager {
    /// 设置密码
    pub fn set_password(auth_state: &mut AuthState, error_handler: impl Fn(&str) -> String) -> Result<(), String> {
        let pw = auth_state.password_input.trim();
        let pwc = auth_state.password_confirm.trim();
        
        if pw.is_empty() {
            return Err(error_handler("password_empty_error"));
        }
        
        if pw != pwc {
            return Err(error_handler("password_mismatch_error"));
        }

        // 检查密码强度
        if let Some(weakness) = Self::check_password_strength(pw) {
            return Err(format!("Password weakness: {}", weakness));
        }

        // 生成 salt 并计算 hash（argon2）
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = argon2
            .hash_password(pw.as_bytes(), &salt)
            .map_err(|e| error_handler("hash_error").replace("{}", &e.to_string()))?
            .to_string();

        // 确保存储目录存在并写入
        if let Some(parent) = auth_state.password_file.parent() {
            if let Err(e) = fs::create_dir_all(parent) {
                return Err(error_handler("create_dir_error").replace("{}", &e.to_string()));
            }
        }
        
        fs::write(&auth_state.password_file, &password_hash)
            .map_err(|e| error_handler("write_error").replace("{}", &e.to_string()))?;

        auth_state.password_hash = Some(password_hash);
        auth_state.is_first_run = false;
        auth_state.is_authenticated = true;
        auth_state.clear_password_inputs();
        
        // 设置默认会话超时（30分钟）
        auth_state.set_session_timeout(30);
        
        Ok(())
    }

    /// 验证密码
    pub fn verify_password(auth_state: &mut AuthState, attempt: &str, error_handler: impl Fn(&str) -> String) -> Result<bool, String> {
        let stored = match &auth_state.password_hash {
            Some(h) => h.clone(),
            None => {
                // 尝试从文件读取（兜底）
                match fs::read_to_string(&auth_state.password_file) {
                    Ok(s) if !s.trim().is_empty() => {
                        auth_state.password_hash = Some(s.clone());
                        s
                    }
                    _ => return Err(error_handler("password_not_found_error")),
                }
            }
        };

        let parsed = PasswordHash::new(&stored)
            .map_err(|e| error_handler("parse_hash_error").replace("{}", &e.to_string()))?;
        let argon2 = Argon2::default();
        
        match argon2.verify_password(attempt.as_bytes(), &parsed) {
            Ok(()) => {
                auth_state.is_authenticated = true;
                auth_state.set_session_timeout(30); // 30分钟会话
                Ok(true)
            }
            Err(_) => Ok(false),
        }
    }

    /// 检查密码强度
    pub fn check_password_strength(password: &str) -> Option<&'static str> {
        if password.len() < 8 {
            return Some("Password must be at least 8 characters long");
        }
        
        let has_lowercase = password.chars().any(|c| c.is_lowercase());
        let has_uppercase = password.chars().any(|c| c.is_uppercase());
        let has_digit = password.chars().any(|c| c.is_numeric());
        let has_special = password.chars().any(|c| !c.is_alphanumeric());
        
        if !has_lowercase {
            return Some("Password must contain at least one lowercase letter");
        }
        if !has_uppercase {
            return Some("Password must contain at least one uppercase letter");
        }
        if !has_digit {
            return Some("Password must contain at least one digit");
        }
        if !has_special {
            return Some("Password must contain at least one special character");
        }
        
        None
    }

    /// 计算密码强度分数 (0-5)
    pub fn calculate_password_score(password: &str) -> u8 {
        let mut score = 0;
        
        if password.len() >= 8 { score += 1; }
        if password.len() >= 12 { score += 1; }
        if password.chars().any(|c| c.is_lowercase()) { score += 1; }
        if password.chars().any(|c| c.is_uppercase()) { score += 1; }
        if password.chars().any(|c| c.is_numeric()) { score += 1; }
        if password.chars().any(|c| !c.is_alphanumeric()) { score += 1; }
        
        score.min(5)
    }

    /// 获取密码强度描述
    pub fn get_password_strength_description(score: u8) -> (&'static str, PasswordStrength) {
        match score {
            0..=1 => ("Very Weak", PasswordStrength::VeryWeak),
            2 => ("Weak", PasswordStrength::Weak),
            3 => ("Fair", PasswordStrength::Fair),
            4 => ("Good", PasswordStrength::Good),
            5 => ("Strong", PasswordStrength::Strong),
            _ => ("Strong", PasswordStrength::Strong),
        }
    }

    /// 重置密码
    pub fn reset_password(auth_state: &mut AuthState) -> Result<(), String> {
        // 删除密码文件
        if auth_state.password_file.exists() {
            fs::remove_file(&auth_state.password_file)
                .map_err(|e| format!("Failed to remove password file: {}", e))?;
        }

        // 重置状态
        auth_state.password_hash = None;
        auth_state.is_first_run = true;
        auth_state.is_authenticated = false;
        auth_state.clear_password_inputs();
        auth_state.clear_session_timeout();

        Ok(())
    }

    /// 更改密码
    pub fn change_password(
        auth_state: &mut AuthState, 
        old_password: &str, 
        new_password: &str, 
        confirm_password: &str,
        error_handler: impl Fn(&str) -> String
    ) -> Result<(), String> {
        // 验证旧密码
        if !Self::verify_password(auth_state, old_password, &error_handler)? {
            return Err("Current password is incorrect".to_string());
        }

        // 临时保存新密码到输入字段
        auth_state.password_input = new_password.to_string();
        auth_state.password_confirm = confirm_password.to_string();

        // 设置新密码
        Self::set_password(auth_state, error_handler)
    }
}

/// 密码强度枚举
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PasswordStrength {
    VeryWeak,
    Weak,
    Fair,
    Good,
    Strong,
}

impl PasswordStrength {
    /// 获取对应的颜色
    #[cfg(feature = "egui")]
    pub fn to_egui_color(&self) -> eframe::egui::Color32 {
        match self {
            PasswordStrength::VeryWeak => eframe::egui::Color32::RED,
            PasswordStrength::Weak => eframe::egui::Color32::from_rgb(255, 100, 100),
            PasswordStrength::Fair => eframe::egui::Color32::YELLOW,
            PasswordStrength::Good => eframe::egui::Color32::from_rgb(100, 200, 100),
            PasswordStrength::Strong => eframe::egui::Color32::GREEN,
        }
    }

    /// 获取 RGB 值
    pub fn to_rgb(&self) -> (u8, u8, u8) {
        match self {
            PasswordStrength::VeryWeak => (255, 0, 0),
            PasswordStrength::Weak => (255, 100, 100),
            PasswordStrength::Fair => (255, 255, 0),
            PasswordStrength::Good => (100, 200, 100),
            PasswordStrength::Strong => (0, 255, 0),
        }
    }
}

/// 认证配置
#[derive(Debug, Clone)]
pub struct AuthConfig {
    pub session_timeout_minutes: u64,
    pub require_password_change: bool,
    pub max_failed_attempts: u32,
    pub lockout_duration_minutes: u64,
    pub auto_logout_on_idle: bool,
}

impl AuthConfig {
    pub fn new() -> Self {
        Self {
            session_timeout_minutes: 30,
            require_password_change: false,
            max_failed_attempts: 5,
            lockout_duration_minutes: 15,
            auto_logout_on_idle: true,
        }
    }

    pub fn with_session_timeout(mut self, minutes: u64) -> Self {
        self.session_timeout_minutes = minutes;
        self
    }

    pub fn with_max_failed_attempts(mut self, attempts: u32) -> Self {
        self.max_failed_attempts = attempts;
        self
    }
}

impl Default for AuthConfig {
    fn default() -> Self {
        Self::new()
    }
}
use std::sync::mpsc::{self, Receiver, Sender};
use sui_sdk::types::{base_types::SuiAddress, crypto::SuiKeyPair};
use tokio::runtime::Runtime;
use std::{fs, path::PathBuf};

use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use argon2::password_hash::rand_core::OsRng;

/// 支持的网络
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Network {
    Devnet,
    Testnet,
    Mainnet,
}

impl Network {
    pub fn url(&self) -> &'static str {
        match self {
            Network::Devnet => "https://fullnode.devnet.sui.io:443",
            Network::Testnet => "https://fullnode.testnet.sui.io:443",
            Network::Mainnet => "https://fullnode.mainnet.sui.io:443",
        }
    }
}

/// 钱包状态
pub enum WalletState {
    // 未导入钱包，存储用户输入的私钥字符串
    NoWallet { private_key_input: String },
    // 已加载钱包
    Loaded {
        address: SuiAddress,
        keypair: SuiKeyPair,
    },
}

/// 应用的所有状态
pub struct Model {
    pub wallet: WalletState,
    pub network: Network,
    pub result_text: String,
    pub is_loading: bool,
    // 转账信息
    pub recipient_address: String,
    pub transfer_amount: String,

    // 新增密码相关字段
    pub is_authenticated: bool,
    pub is_first_run: bool,
    pub password_input: String,
    pub password_confirm: String,
    pub password_hash: Option<String>,
    pub password_file: PathBuf,

    // 异步处理
    pub rt: Runtime,
    pub sender: Sender<Result<String, String>>,
    pub receiver: Receiver<Result<String, String>>,
}

impl Default for Model {
    fn default() -> Self {
        let (sender, receiver) = mpsc::channel();
        // 存储路径：$XDG_CONFIG_HOME/sui_rust_wallet/password.hash 或 $HOME/.config/...
        let mut cfg_dir = dirs::config_dir().unwrap_or_else(|| std::env::current_dir().unwrap());
        cfg_dir.push("sui_rust_wallet");
        let mut password_file = cfg_dir.clone();
        password_file.push("password.hash");

        let (is_first_run, password_hash) = match fs::read_to_string(&password_file) {
            Ok(s) if !s.trim().is_empty() => (false, Some(s)),
            _ => (true, None),
        };

        Self {
            wallet: WalletState::NoWallet {
                private_key_input: "".to_string(),
            },
            network: Network::Devnet,
            result_text: "Please import a private key to begin.".to_string(),
            is_loading: false,
            recipient_address: "".to_string(),
            transfer_amount: "".to_string(),
            is_authenticated: false,
            is_first_run,
            password_input: String::new(),
            password_confirm: String::new(),
            password_hash,
            password_file,
            rt: Runtime::new().expect("Failed to create Tokio runtime"),
            sender,
            receiver,
        }
    }
}

impl Model {
    pub fn set_password(&mut self) -> Result<(), String> {
        let pw = self.password_input.trim();
        let pwc = self.password_confirm.trim();
        if pw.is_empty() {
            return Err("密码不能为空".into());
        }
        if pw != pwc {
            return Err("两次输入的密码不一致".into());
        }

        // 生成 salt 并计算 hash（argon2）
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = argon2
            .hash_password(pw.as_bytes(), &salt)
            .map_err(|e| format!("hash error: {}", e))?
            .to_string();

        // 确保存储目录存在并写入
        if let Some(parent) = self.password_file.parent() {
            if let Err(e) = fs::create_dir_all(parent) {
                return Err(format!("创建目录失败: {}", e));
            }
        }
        fs::write(&self.password_file, &password_hash).map_err(|e| format!("写入失败: {}", e))?;

        self.password_hash = Some(password_hash);
        self.is_first_run = false;
        self.is_authenticated = true;
        self.password_input.clear();
        self.password_confirm.clear();
        Ok(())
    }

    pub fn verify_password(&mut self, attempt: &str) -> Result<bool, String> {
        let stored = match &self.password_hash {
            Some(h) => h.clone(),
            None => {
                // 尝试从文件读取（兜底）
                match fs::read_to_string(&self.password_file) {
                    Ok(s) if !s.trim().is_empty() => {
                        self.password_hash = Some(s.clone());
                        s
                    }
                    _ => return Err("未找到已保存的密码".into()),
                }
            }
        };

        let parsed = PasswordHash::new(&stored).map_err(|e| format!("解析 hash 失败: {}", e))?;
        let argon2 = Argon2::default();
        match argon2.verify_password(attempt.as_bytes(), &parsed) {
            Ok(()) => {
                self.is_authenticated = true;
                Ok(true)
            }
            Err(_) => Ok(false),
        }
    }
}
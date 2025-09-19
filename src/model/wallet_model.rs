use sui_sdk::types::{base_types::SuiAddress, crypto::SuiKeyPair};

/// 钱包状态枚举
#[derive(Debug)]
pub enum WalletState {
    /// 未导入钱包，存储用户输入的私钥字符串
    NoWallet { 
        private_key_input: String 
    },
    /// 已加载钱包
    Loaded {
        address: SuiAddress,
        keypair: SuiKeyPair,
    },
}

impl Default for WalletState {
    fn default() -> Self {
        WalletState::NoWallet {
            private_key_input: String::new(),
        }
    }
}

impl WalletState {
    /// 创建一个新的未导入钱包状态
    pub fn new_no_wallet() -> Self {
        WalletState::NoWallet {
            private_key_input: String::new(),
        }
    }

    /// 创建一个已加载的钱包状态
    pub fn new_loaded(address: SuiAddress, keypair: SuiKeyPair) -> Self {
        WalletState::Loaded { address, keypair }
    }

    /// 检查钱包是否已加载
    pub fn is_loaded(&self) -> bool {
        matches!(self, WalletState::Loaded { .. })
    }

    /// 获取钱包地址（如果已加载）
    pub fn address(&self) -> Option<&SuiAddress> {
        match self {
            WalletState::Loaded { address, .. } => Some(address),
            WalletState::NoWallet { .. } => None,
        }
    }

    /// 获取私钥对（如果已加载）
    pub fn keypair(&self) -> Option<&SuiKeyPair> {
        match self {
            WalletState::Loaded { keypair, .. } => Some(keypair),
            WalletState::NoWallet { .. } => None,
        }
    }

    /// 获取私钥输入（如果未加载）
    pub fn private_key_input(&self) -> Option<&str> {
        match self {
            WalletState::NoWallet { private_key_input } => Some(private_key_input),
            WalletState::Loaded { .. } => None,
        }
    }

    /// 获取可变的私钥输入（如果未加载）
    pub fn private_key_input_mut(&mut self) -> Option<&mut String> {
        match self {
            WalletState::NoWallet { private_key_input } => Some(private_key_input),
            WalletState::Loaded { .. } => None,
        }
    }

    /// 清除私钥输入
    pub fn clear_private_key_input(&mut self) {
        if let WalletState::NoWallet { private_key_input } = self {
            private_key_input.clear();
        }
    }

    /// 设置私钥输入
    pub fn set_private_key_input(&mut self, input: String) {
        if let WalletState::NoWallet { private_key_input } = self {
            *private_key_input = input;
        }
    }

    /// 重置到未导入钱包状态
    pub fn reset(&mut self) {
        *self = WalletState::NoWallet {
            private_key_input: String::new(),
        };
    }

    /// 转换为已加载状态
    pub fn load_wallet(&mut self, address: SuiAddress, keypair: SuiKeyPair) {
        *self = WalletState::Loaded { address, keypair };
    }
}

/// 钱包相关工具函数
pub struct WalletUtils;

impl WalletUtils {
    /// 验证私钥格式
    pub fn validate_private_key_format(private_key: &str) -> bool {
        let trimmed = private_key.trim();
        !trimmed.is_empty() && (
            trimmed.starts_with("suiprivkey1") || // Bech32 format
            (trimmed.len() == 44 && trimmed.chars().all(|c| c.is_alphanumeric() || c == '+' || c == '/' || c == '=')) || // Base64
            (trimmed.len() == 64 && trimmed.chars().all(|c| c.is_ascii_hexdigit())) // Hex
        )
    }

    /// 获取私钥格式类型
    pub fn get_private_key_format(private_key: &str) -> Option<PrivateKeyFormat> {
        let trimmed = private_key.trim();
        if trimmed.starts_with("suiprivkey1") {
            Some(PrivateKeyFormat::Bech32)
        } else if trimmed.len() == 44 && trimmed.chars().all(|c| c.is_alphanumeric() || c == '+' || c == '/' || c == '=') {
            Some(PrivateKeyFormat::Base64)
        } else if trimmed.len() == 64 && trimmed.chars().all(|c| c.is_ascii_hexdigit()) {
            Some(PrivateKeyFormat::Hex)
        } else {
            None
        }
    }

    /// 截断地址用于显示
    pub fn truncate_address(address: &SuiAddress, start_len: usize, end_len: usize) -> String {
        let address_str = address.to_string();
        if address_str.len() <= start_len + end_len + 3 {
            address_str
        } else {
            format!("{}...{}", &address_str[..start_len], &address_str[address_str.len()-end_len..])
        }
    }

    /// 检查地址是否有效
    pub fn is_valid_address_format(address: &str) -> bool {
        // 基本格式检查
        address.len() >= 40 && address.starts_with("0x")
    }
}

/// 私钥格式枚举
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PrivateKeyFormat {
    Bech32,
    Base64,
    Hex,
}

impl PrivateKeyFormat {
    /// 获取格式描述
    pub fn description(&self) -> &'static str {
        match self {
            PrivateKeyFormat::Bech32 => "Bech32 format (suiprivkey1...)",
            PrivateKeyFormat::Base64 => "Base64 format (44 characters)",
            PrivateKeyFormat::Hex => "Hex format (64 characters)",
        }
    }

    /// 获取格式示例
    pub fn example(&self) -> &'static str {
        match self {
            PrivateKeyFormat::Bech32 => "suiprivkey1...",
            PrivateKeyFormat::Base64 => "Base64EncodedKey==",
            PrivateKeyFormat::Hex => "0123456789abcdef...",
        }
    }
}

/// 钱包操作结果
#[derive(Debug)]
pub enum WalletOperationResult {
    Success(String),
    Error(String),
}

impl WalletOperationResult {
    pub fn is_success(&self) -> bool {
        matches!(self, WalletOperationResult::Success(_))
    }

    pub fn is_error(&self) -> bool {
        matches!(self, WalletOperationResult::Error(_))
    }

    pub fn message(&self) -> &str {
        match self {
            WalletOperationResult::Success(msg) => msg,
            WalletOperationResult::Error(msg) => msg,
        }
    }
}
use crate::model::{Model, WalletState};
use crate::controller::BalanceController;
use sui_sdk::{
    types::{base_types::SuiAddress, crypto::SuiKeyPair},
};

/// 钱包控制器 - 处理私钥导入和钱包管理相关功能
pub struct WalletController;

impl WalletController {
    /// 处理私钥导入逻辑
    pub fn handle_import_key(model: &mut Model) {
        if let WalletState::NoWallet { private_key_input } = &model.wallet {
            let trimmed_input = private_key_input.trim();

            // 1. 尝试使用 `decode` 解析 Bech32 格式 (suiprivkey1...)
            // 2. 如果失败，则回退尝试使用 `decode_base64` 解析 Base64 格式
            let keypair_result = SuiKeyPair::decode(trimmed_input);

            match keypair_result {
                Ok(keypair) => {
                    let address: SuiAddress = (&keypair.public()).into();
                    model.wallet = WalletState::Loaded { address, keypair };
                    model.result_text = format!("{}: {}", model.i18n.tr("wallet_imported_success"), address);
                    // 导入成功后自动刷新余额
                    BalanceController::handle_refresh_balance(model);
                }
                Err(_) => {
                    model.result_text = model.i18n.tr("import_private_key_failed");
                }
            }
        }
    }

    /// 获取当前钱包地址
    pub fn get_wallet_address(model: &Model) -> Option<SuiAddress> {
        if let WalletState::Loaded { address, .. } = &model.wallet {
            Some(*address)
        } else {
            None
        }
    }

    /// 检查钱包是否已加载
    pub fn is_wallet_loaded(model: &Model) -> bool {
        matches!(model.wallet, WalletState::Loaded { .. })
    }

    /// 获取私钥输入
    pub fn get_private_key_input(model: &Model) -> String {
        if let WalletState::NoWallet { private_key_input } = &model.wallet {
            private_key_input.clone()
        } else {
            String::new()
        }
    }

    /// 设置私钥输入
    pub fn set_private_key_input(model: &mut Model, input: String) {
        if let WalletState::NoWallet { private_key_input } = &mut model.wallet {
            *private_key_input = input;
        }
    }

    /// 清除钱包数据
    pub fn clear_wallet(model: &mut Model) {
        model.wallet = WalletState::NoWallet {
            private_key_input: String::new(),
        };
    }

    /// 验证私钥格式
    pub fn validate_private_key(private_key: &str) -> bool {
        let trimmed = private_key.trim();
        !trimmed.is_empty() && (
            trimmed.starts_with("suiprivkey1") || // Bech32 format
            trimmed.len() == 44 || // Base64 format (typical length)
            trimmed.len() == 64    // Hex format
        )
    }
}
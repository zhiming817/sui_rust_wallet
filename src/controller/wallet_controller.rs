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
            let trimmed_input = private_key_input.trim().to_string();

            // 1. 尝试使用 `decode` 解析 Bech32 格式 (suiprivkey1...)
            // 2. 如果失败，则回退尝试使用 `decode_base64` 解析 Base64 格式
            let keypair_result = SuiKeyPair::decode(&trimmed_input);

            match keypair_result {
                Ok(keypair) => {
                    let address: SuiAddress = (&keypair.public()).into();
                    model.wallet = WalletState::Loaded { address, keypair };
                    model.result_text = format!("{}: {}", model.i18n.tr("wallet_imported_success"), address);
                    
                    // 如果用户已认证，自动保存加密的私钥
                    if model.auth_state.is_authenticated {
                        if let Some(password) = model.auth_state.get_session_password() {
                            if let Err(e) = model.auth_state.save_encrypted_private_key(&trimmed_input, password) {
                                eprintln!("Failed to save encrypted private key: {}", e);
                                // 不影响导入流程，只记录错误
                            } else {
                                println!("Private key saved successfully");
                            }
                        }
                    }
                    
                    // 导入成功后自动刷新余额
                    BalanceController::handle_refresh_balance(model);
                }
                Err(_) => {
                    model.result_text = model.i18n.tr("import_private_key_failed");
                }
            }
        }
    }

    /// 处理私钥导入并保存
    pub fn handle_import_and_save_key(model: &mut Model, password: &str) {
        if let WalletState::NoWallet { private_key_input } = &model.wallet {
            let trimmed_input = private_key_input.trim().to_string();

            // 1. 尝试使用 `decode` 解析 Bech32 格式 (suiprivkey1...)
            // 2. 如果失败，则回退尝试使用 `decode_base64` 解析 Base64 格式
            let keypair_result = SuiKeyPair::decode(&trimmed_input);

            match keypair_result {
                Ok(keypair) => {
                    let address: SuiAddress = (&keypair.public()).into();
                    model.wallet = WalletState::Loaded { address, keypair };
                    model.result_text = format!("{}: {}", model.i18n.tr("wallet_imported_success"), address);
                    
                    // 自动保存加密的私钥（如果用户已认证）
                    if model.auth_state.is_authenticated {
                        if let Err(e) = model.auth_state.save_encrypted_private_key(&trimmed_input, password) {
                            eprintln!("Failed to save encrypted private key: {}", e);
                            // 不影响导入流程，只记录错误
                        } else {
                            println!("Private key saved successfully");
                        }
                    }
                    
                    // 导入成功后自动刷新余额
                    BalanceController::handle_refresh_balance(model);
                }
                Err(_) => {
                    model.result_text = model.i18n.tr("import_private_key_failed");
                }
            }
        }
    }

    /// 尝试从加密存储加载私钥
    pub fn try_load_saved_key(model: &mut Model, password: &str) -> Result<bool, String> {
        match model.auth_state.load_encrypted_private_key(password)? {
            Some(private_key) => {
                // 解析私钥
                let keypair_result = SuiKeyPair::decode(&private_key);
                match keypair_result {
                    Ok(keypair) => {
                        let address: SuiAddress = (&keypair.public()).into();
                        model.wallet = WalletState::Loaded { address, keypair };
                        model.result_text = format!("{}: {}", model.i18n.tr("wallet_loaded_from_storage"), address);
                        
                        // 自动刷新余额
                        BalanceController::handle_refresh_balance(model);
                        Ok(true)
                    }
                    Err(_) => {
                        Err("Failed to parse saved private key".to_string())
                    }
                }
            }
            None => Ok(false), // 没有保存的私钥
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
use crate::model::{Model, WalletState};
use std::str::FromStr;
use sui_sdk::{
    // 确保导入了 KeypairTraits
    types::{base_types::SuiAddress, crypto::{SuiKeyPair, KeypairTraits}},
    SuiClientBuilder,
};

// --- 动作处理 ---

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
                model.result_text = format!("Wallet imported successfully for address: {}", address);
                // 导入成功后自动刷新余额
                handle_refresh_balance(model);
            }
            Err(_) => {
                model.result_text = "Failed to import private key. Please check the format (Bech32 or Base64).".to_string();
            }
        }
    }
}

/// 处理登出逻辑
pub fn handle_logout(model: &mut Model) {
    model.wallet = WalletState::NoWallet {
        private_key_input: "".to_string(),
    };
    model.result_text = "Wallet logged out. Import a key to begin.".to_string();
}

/// 处理刷新余额的请求
pub fn handle_refresh_balance(model: &mut Model) {
    if let WalletState::Loaded { address, .. } = &model.wallet {
        model.is_loading = true;
        model.result_text = "Refreshing balance...".to_string();
        let sender = model.sender.clone();
        let address = *address;
        let network_url = model.network.url();

        model.rt.spawn(async move {
            let result = fetch_balance(address, network_url).await;
            sender.send(result).expect("Failed to send message");
        });
    } else {
        model.result_text = "No wallet loaded. Please import a key first.".to_string();
    }
}

/// 处理从后台线程接收到的异步结果
pub fn handle_async_results(model: &mut Model) {
    if let Ok(result) = model.receiver.try_recv() {
        model.is_loading = false;
        match result {
            Ok(message) => model.result_text = message,
            Err(e) => model.result_text = format!("Error: {}", e),
        }
    }
}

// --- 异步逻辑 ---

/// 异步获取SUI代币余额
async fn fetch_balance(address: SuiAddress, network_url: &str) -> Result<String, String> {
    let sui_client = SuiClientBuilder::default()
        .build(network_url)
        .await
        .map_err(|e| e.to_string())?;
    let balances = sui_client
        .coin_read_api()
        .get_all_balances(address)
        .await
        .map_err(|e| e.to_string())?;

    // 只查找并显示 SUI 代币的余额
    let sui_balance = balances.iter().find(|b| b.coin_type == "0x2::sui::SUI");
    if let Some(balance) = sui_balance {
        let amount = balance.total_balance as f64 / 1_000_000_000.0;
        Ok(format!("{:.4} SUI", amount))
    } else {
        Ok("0.0000 SUI".to_string())
    }
}
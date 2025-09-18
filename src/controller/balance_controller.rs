use crate::model::{Model, WalletState};
use sui_sdk::{
    types::base_types::SuiAddress,
    SuiClientBuilder,
};

/// 余额控制器 - 处理余额查询和刷新相关功能
pub struct BalanceController;

impl BalanceController {
    /// 处理刷新余额的请求
    pub fn handle_refresh_balance(model: &mut Model) {
        if let WalletState::Loaded { address, .. } = &model.wallet {
            model.is_loading = true;
            model.result_text = model.i18n.tr("refreshing_balance");
            let sender = model.sender.clone();
            let address = *address;
            let network_url = model.network.url();

            model.rt.spawn(async move {
                let result = Self::fetch_balance(address, network_url).await;
                sender.send(result).expect("Failed to send message");
            });
        } else {
            model.result_text = model.i18n.tr("no_wallet_loaded");
        }
    }

    /// 处理从后台线程接收到的异步结果
    pub fn handle_async_results(model: &mut Model) {
        if let Ok(result) = model.receiver.try_recv() {
            model.is_loading = false;
            match result {
                Ok(message) => model.result_text = message,
                Err(e) => model.result_text = format!("{}: {}", model.i18n.tr("async_error"), e),
            }
        }
    }

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

    /// 检查是否正在加载余额
    pub fn is_loading(model: &Model) -> bool {
        model.is_loading
    }

    /// 设置加载状态
    pub fn set_loading(model: &mut Model, loading: bool) {
        model.is_loading = loading;
    }

    /// 获取余额显示格式化
    pub fn format_balance(balance: f64) -> String {
        format!("{:.4} SUI", balance)
    }

    /// 解析余额字符串
    pub fn parse_balance(balance_str: &str) -> Option<f64> {
        balance_str
            .trim_end_matches(" SUI")
            .parse::<f64>()
            .ok()
    }
}
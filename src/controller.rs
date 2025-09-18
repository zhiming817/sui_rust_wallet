// 主控制器 - 协调各个子控制器
use crate::model::Model;

// 导入子控制器
pub mod auth_controller;
pub mod wallet_controller;
pub mod balance_controller;

// 重新导出控制器以便外部使用
pub use auth_controller::AuthController;
pub use wallet_controller::WalletController;
pub use balance_controller::BalanceController;

/// 主控制器 - 提供统一的入口点来协调各个子控制器
pub struct MainController;

impl MainController {
    // --- 认证相关功能代理 ---
    
    /// 处理登出逻辑
    pub fn handle_logout(model: &mut Model) {
        AuthController::handle_logout(model);
    }

    /// 处理设置密码请求
    pub fn handle_set_password(model: &mut Model) -> Result<(), String> {
        AuthController::handle_set_password(model)
    }

    /// 处理验证密码请求
    pub fn handle_verify_password(model: &mut Model) -> Result<(), String> {
        AuthController::handle_verify_password(model)
    }

    // --- 钱包相关功能代理 ---
    
    /// 处理私钥导入逻辑
    pub fn handle_import_key(model: &mut Model) {
        WalletController::handle_import_key(model);
    }

    // --- 余额相关功能代理 ---
    
    /// 处理刷新余额的请求
    pub fn handle_refresh_balance(model: &mut Model) {
        BalanceController::handle_refresh_balance(model);
    }

    /// 处理从后台线程接收到的异步结果
    pub fn handle_async_results(model: &mut Model) {
        BalanceController::handle_async_results(model);
    }

    // --- 应用程序级别的协调功能 ---

    /// 处理应用程序初始化
    pub fn initialize_app(model: &mut Model) {
        // 如果是首次运行，可以进行一些初始化操作
        if model.is_first_run {
            model.result_text = model.i18n.tr("welcome_first_run");
        } else {
            model.result_text = model.i18n.tr("import_private_key_message");
        }
    }

    /// 处理语言切换
    pub fn handle_language_change(model: &mut Model) {
        // 更新结果文本以反映新语言
        if WalletController::is_wallet_loaded(model) {
            if let Some(address) = WalletController::get_wallet_address(model) {
                model.result_text = format!("{}: {}", model.i18n.tr("wallet_imported_success"), address);
            }
        } else {
            model.result_text = model.i18n.tr("import_private_key_message");
        }
    }

    /// 处理网络切换
    pub fn handle_network_change(model: &mut Model) {
        // 如果钱包已加载，切换网络后需要刷新余额
        if WalletController::is_wallet_loaded(model) {
            Self::handle_refresh_balance(model);
        }
    }

    /// 获取应用程序状态摘要
    pub fn get_app_status(model: &Model) -> AppStatus {
        AppStatus {
            is_authenticated: AuthController::is_authenticated(model),
            wallet_loaded: WalletController::is_wallet_loaded(model),
            is_loading: BalanceController::is_loading(model),
            wallet_address: WalletController::get_wallet_address(model),
        }
    }
}

/// 应用程序状态摘要
#[derive(Debug, Clone)]
pub struct AppStatus {
    pub is_authenticated: bool,
    pub wallet_loaded: bool,
    pub is_loading: bool,
    pub wallet_address: Option<sui_sdk::types::base_types::SuiAddress>,
}

// --- 向后兼容性函数 ---
// 为了不破坏现有代码，提供向后兼容的函数

/// 处理私钥导入逻辑（向后兼容）
pub fn handle_import_key(model: &mut Model) {
    MainController::handle_import_key(model);
}

/// 处理登出逻辑（向后兼容）
pub fn handle_logout(model: &mut Model) {
    MainController::handle_logout(model);
}

/// 处理刷新余额的请求（向后兼容）
pub fn handle_refresh_balance(model: &mut Model) {
    MainController::handle_refresh_balance(model);
}

/// 处理从后台线程接收到的异步结果（向后兼容）
pub fn handle_async_results(model: &mut Model) {
    MainController::handle_async_results(model);
}

/// 处理设置密码请求（向后兼容）
pub fn handle_set_password(model: &mut Model) -> Result<(), String> {
    MainController::handle_set_password(model)
}

/// 处理验证密码请求（向后兼容）
pub fn handle_verify_password(model: &mut Model) -> Result<(), String> {
    MainController::handle_verify_password(model)
}
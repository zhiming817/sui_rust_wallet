// 模型协调器 - 统一导入和重新导出所有模型子模块
// 为了向后兼容，主要结构和接口保持在此文件中

mod wallet_model;
mod network_model;
mod auth_model;
mod app_state;

// 重新导出子模块的公共类型
pub use wallet_model::*;
pub use network_model::*;
pub use auth_model::*;
pub use app_state::*;

use std::sync::mpsc::{self, Receiver, Sender};
use tokio::runtime::Runtime;
use crate::i18n::{I18nManager, Language};

/// 应用的所有状态 - 主模型结构
/// 整合了所有子模块的功能
pub struct Model {
    // 钱包相关状态
    pub wallet: WalletState,
    
    // 网络配置
    pub network: Network,
    
    // 认证相关状态
    pub auth_state: AuthState,
    
    // 应用程序状态
    pub app_state: AppState,
    
    // UI 状态
    pub result_text: String,
    pub is_loading: bool,
    
    // 转账信息
    pub recipient_address: String,
    pub transfer_amount: String,

    // 国际化相关
    pub i18n: I18nManager,

    // 异步处理
    pub rt: Runtime,
    pub sender: Sender<Result<String, String>>,
    pub receiver: Receiver<Result<String, String>>,
}

impl Default for Model {
    fn default() -> Self {
        let (sender, receiver) = mpsc::channel();
        let i18n_manager = I18nManager::new();
        let import_message = i18n_manager.tr("import_private_key_message");

        Self {
            wallet: WalletState::default(),
            network: Network::Testnet,
            auth_state: AuthState::default(),
            app_state: AppState::default(),
            result_text: import_message,
            is_loading: false,
            recipient_address: String::new(),
            transfer_amount: String::new(),
            i18n: i18n_manager,
            rt: Runtime::new().expect("Failed to create Tokio runtime"),
            sender,
            receiver,
        }
    }
}

impl Model {
    // 国际化方法
    pub fn set_language(&mut self, language: Language) {
        self.i18n.set_language(language);
    }

    pub fn current_language(&self) -> Language {
        self.i18n.current_language()
    }

    // 密码相关方法 - 委托给 AuthState
    pub fn set_password(&mut self) -> Result<(), String> {
        self.auth_state.set_password(&self.i18n)
    }

    pub fn verify_password(&mut self, attempt: &str) -> Result<bool, String> {
        self.auth_state.verify_password(attempt, &self.i18n)
    }

    // 向后兼容的字段访问器
    pub fn is_authenticated(&self) -> bool {
        self.auth_state.is_authenticated
    }

    pub fn is_first_run(&self) -> bool {
        self.auth_state.is_first_run
    }

    pub fn password_input(&self) -> &str {
        &self.auth_state.password_input
    }

    pub fn password_confirm(&self) -> &str {
        &self.auth_state.password_confirm
    }

    pub fn password_input_mut(&mut self) -> &mut String {
        &mut self.auth_state.password_input
    }

    pub fn password_confirm_mut(&mut self) -> &mut String {
        &mut self.auth_state.password_confirm
    }

    // 钱包相关方法委托
    pub fn get_wallet_state(&self) -> &WalletState {
        &self.wallet
    }

    pub fn get_wallet_state_mut(&mut self) -> &mut WalletState {
        &mut self.wallet
    }

    // 网络相关方法委托
    pub fn get_network(&self) -> Network {
        self.network
    }

    pub fn set_network(&mut self, network: Network) {
        self.network = network;
    }

    // 应用状态相关方法委托
    pub fn get_app_settings(&self) -> &AppSettings {
        &self.app_state.settings
    }

    pub fn get_app_settings_mut(&mut self) -> &mut AppSettings {
        &mut self.app_state.settings
    }
}
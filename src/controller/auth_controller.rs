use crate::model::{Model, WalletState};
use crate::controller::WalletController;

/// 认证控制器 - 处理登录、登出和密码验证相关功能
pub struct AuthController;

impl AuthController {
    /// 处理登出逻辑
    pub fn handle_logout(model: &mut Model) {
        // 清理认证状态和会话密码
        model.auth_state.logout();
        
        // 清理钱包状态
        model.wallet = WalletState::NoWallet {
            private_key_input: "".to_string(),
        };
        model.result_text = model.i18n.tr("wallet_logged_out_message");
    }

    /// 处理设置密码请求（由 UI 触发）
    pub fn handle_set_password(model: &mut Model) -> Result<(), String> {
        model.set_password()
    }

    /// 处理登录
    pub fn handle_login(model: &mut crate::model::Model) -> Result<(), String> {
        let attempt = model.auth_state.password_input.clone();
        
        match model.verify_password(&attempt) {
            Ok(true) => {
                model.auth_state.is_authenticated = true;
                
                // 设置会话密码，用于后续的加密操作
                model.auth_state.set_session_password(attempt.clone());
                
                // 认证成功后，尝试加载保存的私钥
                if let Err(e) = WalletController::try_load_saved_key(model, &attempt) {
                    eprintln!("Failed to load saved private key: {}", e);
                    // 不影响登录流程，用户可以手动导入
                }
                
                Ok(())
            }
            Ok(false) => Err(model.i18n.tr("password_incorrect_error")),
            Err(e) => Err(e),
        }
    }

    /// 处理密码验证
    pub fn handle_verify_password(model: &mut crate::model::Model) -> Result<(), String> {
        let result = Self::handle_login(model);
        // 登录方法已经包含了自动加载私钥的逻辑
        result
    }

    /// 检查是否已认证
    pub fn is_authenticated(model: &crate::model::Model) -> bool {
        model.auth_state.is_authenticated && !model.auth_state.is_session_expired()
    }

    /// 登出
    pub fn logout(model: &mut crate::model::Model) {
        model.auth_state.is_authenticated = false;
        model.auth_state.password_input.clear();
        model.auth_state.password_confirm.clear();
    }

    /// 清除密码输入字段
    pub fn clear_password_inputs(model: &mut crate::model::Model) {
        model.auth_state.password_input.clear();
        model.auth_state.password_confirm.clear();
    }
}
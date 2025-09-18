use crate::model::{Model, WalletState};

/// 认证控制器 - 处理登录、登出和密码验证相关功能
pub struct AuthController;

impl AuthController {
    /// 处理登出逻辑
    pub fn handle_logout(model: &mut Model) {
        model.wallet = WalletState::NoWallet {
            private_key_input: "".to_string(),
        };
        model.result_text = model.i18n.tr("wallet_logged_out_message");
    }

    /// 处理设置密码请求（由 UI 触发）
    pub fn handle_set_password(model: &mut Model) -> Result<(), String> {
        model.set_password()
    }

    /// 处理验证密码请求（由 UI 触发）
    pub fn handle_verify_password(model: &mut Model) -> Result<(), String> {
        let attempt = model.password_input.clone();
        match model.verify_password(&attempt) {
            Ok(true) => Ok(()),
            Ok(false) => Err(model.i18n.tr("password_error")),
            Err(e) => Err(e),
        }
    }

    /// 检查用户是否已认证
    pub fn is_authenticated(model: &Model) -> bool {
        model.is_authenticated
    }

    /// 设置认证状态
    pub fn set_authenticated(model: &mut Model, authenticated: bool) {
        model.is_authenticated = authenticated;
    }

    /// 清除密码输入字段
    pub fn clear_password_inputs(model: &mut Model) {
        model.password_input.clear();
        model.password_confirm.clear();
    }
}
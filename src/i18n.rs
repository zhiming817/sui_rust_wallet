use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use std::sync::OnceLock;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Language {
    English,
    Chinese,
}

impl Language {
    pub fn code(&self) -> &'static str {
        match self {
            Language::English => "en",
            Language::Chinese => "zh-CN",
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            Language::English => "English",
            Language::Chinese => "中文",
        }
    }

    pub fn all() -> Vec<Language> {
        vec![Language::English, Language::Chinese]
    }
}

impl Default for Language {
    fn default() -> Self {
        // 根据系统语言设置默认语言
        match std::env::var("LANG") {
            Ok(lang) if lang.starts_with("zh") => Language::Chinese,
            _ => Language::English,
        }
    }
}

// 使用静态HashMap存储翻译
static TRANSLATIONS: OnceLock<HashMap<&'static str, HashMap<&'static str, &'static str>>> = OnceLock::new();

fn init_translations() -> &'static HashMap<&'static str, HashMap<&'static str, &'static str>> {
    TRANSLATIONS.get_or_init(|| {
        let mut translations = HashMap::new();
        
        // English translations
        let mut en = HashMap::new();
        en.insert("app_title", "Simple Sui Wallet");
        en.insert("login_title", "Sui Rust Wallet - Login");
        en.insert("first_run_message", "First run: Please set a password (for local encryption)");
        en.insert("enter_password", "Enter password");
        en.insert("confirm_password", "Confirm password");
        en.insert("create_password_button", "Create Password and Enter");
        en.insert("login_message", "Please enter your password to log in");
        en.insert("login_button", "Login");
        en.insert("exit_button", "Exit");
        en.insert("password_info", "The password will be encrypted with the Argon2 algorithm and saved in the local configuration directory.");
        en.insert("network_label", "Network");
        en.insert("devnet", "Devnet");
        en.insert("testnet", "Testnet");
        en.insert("mainnet", "Mainnet");
        en.insert("import_wallet_message", "Import your wallet using a Base64 private key:");
        en.insert("import_wallet_button", "Import Wallet");
        en.insert("wallet_loaded", "Wallet Loaded");
        en.insert("address_label", "Address:");
        en.insert("balance_label", "Balance:");
        en.insert("refresh_balance_button", "Refresh Balance");
        en.insert("logout_button", "Logout");
        en.insert("language_label", "Language");
        // Error messages
        en.insert("password_empty_error", "Password cannot be empty");
        en.insert("password_mismatch_error", "The two passwords entered do not match");
        en.insert("hash_error", "Hash error: {}");
        en.insert("create_dir_error", "Failed to create directory: {}");
        en.insert("write_error", "Write failed: {}");
        en.insert("password_not_found_error", "No saved password found");
        en.insert("parse_hash_error", "Failed to parse hash: {}");
        en.insert("password_error", "Password error");
        // Default messages
        en.insert("import_private_key_message", "Please import a private key to begin.");
        // Wallet messages
        en.insert("wallet_imported_success", "Wallet imported successfully for address");
        en.insert("import_private_key_failed", "Failed to import private key. Please check the format (Bech32 or Base64).");
        en.insert("wallet_logged_out_message", "Wallet logged out. Import a key to begin.");
        // Balance messages
        en.insert("refreshing_balance", "Refreshing balance...");
        en.insert("no_wallet_loaded", "No wallet loaded. Please import a key first.");
        en.insert("async_error", "Error");
        // App messages
        en.insert("welcome_first_run", "Welcome! Please set up your password to get started.");
        // New view module translations
        en.insert("import_wallet_title", "Import Wallet");
        en.insert("private_key_hint", "Enter your private key here...");
        en.insert("supported_formats", "Supported formats:");
        en.insert("format_help_title", "Supported Key Formats");
        en.insert("format_help_bech32", "Bech32 format:");
        en.insert("format_help_base64", "Base64 format:");
        en.insert("format_help_hex", "Hex format:");
        en.insert("valid_format", "Valid format");
        en.insert("invalid_format", "Invalid format");
        en.insert("format_status", "Format:");
        en.insert("security_warning_title", "Security Warning");
        en.insert("security_warning_message", "Never share your private key with anyone!");
        en.insert("copy_address_button", "Copy Address");
        en.insert("balance_unknown", "Unknown");
        en.insert("loading", "Loading...");
        en.insert("tools_label", "Tools");
        en.insert("clear_cache", "Clear Cache");
        en.insert("export_logs", "Export Logs");
        en.insert("about", "About");
        en.insert("status_loading", "Loading...");
        en.insert("status_connected", "Connected");
        en.insert("status_no_wallet", "No Wallet");
        en.insert("current_language", "Current Language");
        en.insert("quick_actions", "Quick Actions");
        en.insert("settings", "Settings");
        en.insert("copy", "Copy");
        en.insert("paste", "Paste");
        en.insert("select_all", "Select All");
        en.insert("keyboard_shortcuts", "Keyboard Shortcuts");
        en.insert("refresh", "Refresh");
        en.insert("close", "Close");
        en.insert("balance_title", "Balance");
        en.insert("balance_error", "Balance Error");
        en.insert("retry_button", "Retry");
        en.insert("current_balance", "Current Balance");
        en.insert("estimated_value", "Estimated Value");
        en.insert("network_fee", "Network Fee");
        en.insert("send_button", "Send");
        en.insert("receive_button", "Receive");
        en.insert("transaction_history", "Transaction History");
        en.insert("no_transactions", "No transactions yet");
        en.insert("view_explorer", "View in Explorer");
        en.insert("balance_chart", "Balance Chart");
        en.insert("chart_placeholder", "Chart coming soon...");
        en.insert("low_balance_warning", "Low Balance Warning");
        en.insert("consider_adding_funds", "Consider adding more funds");
        translations.insert("en", en);

        // Chinese translations
        let mut zh = HashMap::new();
        zh.insert("app_title", "简单Sui钱包");
        zh.insert("login_title", "Sui Rust钱包 - 登录");
        zh.insert("first_run_message", "首次运行：请设置密码（用于本地加密）");
        zh.insert("enter_password", "输入密码");
        zh.insert("confirm_password", "确认密码");
        zh.insert("create_password_button", "创建密码并进入");
        zh.insert("login_message", "请输入您的密码以登录");
        zh.insert("login_button", "登录");
        zh.insert("exit_button", "退出");
        zh.insert("password_info", "密码将使用Argon2算法加密并保存在本地配置目录中。");
        zh.insert("network_label", "网络");
        zh.insert("devnet", "开发网");
        zh.insert("testnet", "测试网");
        zh.insert("mainnet", "主网");
        zh.insert("import_wallet_message", "使用Base64私钥导入您的钱包：");
        zh.insert("import_wallet_button", "导入钱包");
        zh.insert("wallet_loaded", "钱包已加载");
        zh.insert("address_label", "地址：");
        zh.insert("balance_label", "余额：");
        zh.insert("refresh_balance_button", "刷新余额");
        zh.insert("logout_button", "退出登录");
        zh.insert("language_label", "语言");
        // Error messages
        zh.insert("password_empty_error", "密码不能为空");
        zh.insert("password_mismatch_error", "两次输入的密码不一致");
        zh.insert("hash_error", "哈希错误: {}");
        zh.insert("create_dir_error", "创建目录失败: {}");
        zh.insert("write_error", "写入失败: {}");
        zh.insert("password_not_found_error", "未找到已保存的密码");
        zh.insert("parse_hash_error", "解析哈希失败: {}");
        zh.insert("password_error", "密码错误");
        // Default messages
        zh.insert("import_private_key_message", "请导入私钥以开始使用。");
        // Wallet messages
        zh.insert("wallet_imported_success", "钱包导入成功，地址为");
        zh.insert("import_private_key_failed", "导入私钥失败。请检查格式（Bech32 或 Base64）。");
        zh.insert("wallet_logged_out_message", "钱包已退出。请导入私钥以开始使用。");
        // Balance messages
        zh.insert("refreshing_balance", "正在刷新余额...");
        zh.insert("no_wallet_loaded", "未加载钱包。请先导入私钥。");
        zh.insert("async_error", "错误");
        // App messages
        zh.insert("welcome_first_run", "欢迎！请设置您的密码以开始使用。");
        // New view module translations
        zh.insert("import_wallet_title", "导入钱包");
        zh.insert("private_key_hint", "在此输入您的私钥...");
        zh.insert("supported_formats", "支持的格式:");
        zh.insert("format_help_title", "支持的密钥格式");
        zh.insert("format_help_bech32", "Bech32 格式:");
        zh.insert("format_help_base64", "Base64 格式:");
        zh.insert("format_help_hex", "十六进制格式:");
        zh.insert("valid_format", "有效格式");
        zh.insert("invalid_format", "无效格式");
        zh.insert("format_status", "格式:");
        zh.insert("security_warning_title", "安全警告");
        zh.insert("security_warning_message", "绝不要与任何人分享您的私钥！");
        zh.insert("copy_address_button", "复制地址");
        zh.insert("balance_unknown", "未知");
        zh.insert("loading", "加载中...");
        zh.insert("tools_label", "工具");
        zh.insert("clear_cache", "清除缓存");
        zh.insert("export_logs", "导出日志");
        zh.insert("about", "关于");
        zh.insert("status_loading", "加载中...");
        zh.insert("status_connected", "已连接");
        zh.insert("status_no_wallet", "无钱包");
        zh.insert("current_language", "当前语言");
        zh.insert("quick_actions", "快速操作");
        zh.insert("settings", "设置");
        zh.insert("copy", "复制");
        zh.insert("paste", "粘贴");
        zh.insert("select_all", "全选");
        zh.insert("keyboard_shortcuts", "键盘快捷键");
        zh.insert("refresh", "刷新");
        zh.insert("close", "关闭");
        zh.insert("balance_title", "余额");
        zh.insert("balance_error", "余额错误");
        zh.insert("retry_button", "重试");
        zh.insert("current_balance", "当前余额");
        zh.insert("estimated_value", "估算价值");
        zh.insert("network_fee", "网络费用");
        zh.insert("send_button", "发送");
        zh.insert("receive_button", "接收");
        zh.insert("transaction_history", "交易历史");
        zh.insert("no_transactions", "暂无交易记录");
        zh.insert("view_explorer", "在浏览器中查看");
        zh.insert("balance_chart", "余额图表");
        zh.insert("chart_placeholder", "图表即将推出...");
        zh.insert("low_balance_warning", "余额不足警告");
        zh.insert("consider_adding_funds", "建议添加更多资金");
        translations.insert("zh-CN", zh);

        translations
    })
}

/// 国际化管理器
#[derive(Debug)]
pub struct I18nManager {
    current_language: Language,
}

impl I18nManager {
    pub fn new() -> Self {
        let language = Language::default();
        Self {
            current_language: language,
        }
    }

    pub fn set_language(&mut self, language: Language) {
        self.current_language = language;
    }

    pub fn current_language(&self) -> Language {
        self.current_language
    }

    /// 获取翻译文本的便捷函数
    pub fn tr(&self, key: &str) -> String {
        let translations = init_translations();
        let lang_code = self.current_language.code();
        
        if let Some(lang_map) = translations.get(lang_code) {
            if let Some(text) = lang_map.get(key) {
                return text.to_string();
            }
        }
        
        // 回退到英文
        if let Some(en_map) = translations.get("en") {
            if let Some(text) = en_map.get(key) {
                return text.to_string();
            }
        }
        
        // 如果都找不到，返回key本身
        key.to_string()
    }
}

impl Default for I18nManager {
    fn default() -> Self {
        Self::new()
    }
}
use std::sync::mpsc::{self, Receiver, Sender};
use tokio::runtime::Runtime;
use crate::i18n::{I18nManager, Language};

/// 应用程序状态
#[derive(Debug)]
pub struct AppState {
    /// 结果文本显示
    pub result_text: String,
    /// 是否正在加载
    pub is_loading: bool,
    /// 转账相关信息（未来功能）
    pub recipient_address: String,
    pub transfer_amount: String,
    /// 国际化管理器
    pub i18n: I18nManager,
    /// 异步运行时
    pub rt: Runtime,
    /// 异步消息通道
    pub sender: Sender<Result<String, String>>,
    pub receiver: Receiver<Result<String, String>>,
    /// 应用程序设置
    pub settings: AppSettings,
    /// 用户界面状态
    pub ui_state: UiState,
}

impl AppState {
    /// 创建新的应用程序状态
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel();
        let i18n_manager = I18nManager::new();
        let import_message = i18n_manager.tr("import_private_key_message");

        Self {
            result_text: import_message,
            is_loading: false,
            recipient_address: String::new(),
            transfer_amount: String::new(),
            i18n: i18n_manager,
            rt: Runtime::new().expect("Failed to create Tokio runtime"),
            sender,
            receiver,
            settings: AppSettings::default(),
            ui_state: UiState::default(),
        }
    }

    /// 设置语言
    pub fn set_language(&mut self, language: Language) {
        self.i18n.set_language(language);
        self.settings.language = language;
    }

    /// 获取当前语言
    pub fn current_language(&self) -> Language {
        self.i18n.current_language()
    }

    /// 设置结果文本
    pub fn set_result_text(&mut self, text: String) {
        self.result_text = text;
    }

    /// 设置加载状态
    pub fn set_loading(&mut self, loading: bool) {
        self.is_loading = loading;
        if loading {
            self.result_text = self.i18n.tr("loading");
        }
    }

    /// 清除结果文本
    pub fn clear_result_text(&mut self) {
        self.result_text.clear();
    }

    /// 显示错误消息
    pub fn show_error(&mut self, error: &str) {
        self.result_text = format!("{}: {}", self.i18n.tr("error"), error);
        self.is_loading = false;
    }

    /// 显示成功消息
    pub fn show_success(&mut self, message: &str) {
        self.result_text = message.to_string();
        self.is_loading = false;
    }

    /// 处理异步结果
    pub fn handle_async_results(&mut self) {
        if let Ok(result) = self.receiver.try_recv() {
            self.is_loading = false;
            match result {
                Ok(message) => self.result_text = message,
                Err(error) => self.show_error(&error),
            }
        }
    }

    /// 发送异步任务结果
    pub fn send_result(&self, result: Result<String, String>) {
        if let Err(e) = self.sender.send(result) {
            eprintln!("Failed to send async result: {}", e);
        }
    }

    /// 重置转账信息
    pub fn reset_transfer_info(&mut self) {
        self.recipient_address.clear();
        self.transfer_amount.clear();
    }

    /// 验证转账信息
    pub fn validate_transfer_info(&self) -> Result<(), String> {
        if self.recipient_address.trim().is_empty() {
            return Err(self.i18n.tr("recipient_required"));
        }
        
        if self.transfer_amount.trim().is_empty() {
            return Err(self.i18n.tr("amount_required"));
        }

        // 验证金额格式
        if self.transfer_amount.parse::<f64>().is_err() {
            return Err(self.i18n.tr("invalid_amount"));
        }

        Ok(())
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

/// 应用程序设置
#[derive(Debug, Clone)]
pub struct AppSettings {
    /// 当前语言
    pub language: Language,
    /// 主题设置
    pub theme: AppTheme,
    /// 自动保存设置
    pub auto_save: bool,
    /// 会话超时时间（分钟）
    pub session_timeout_minutes: u64,
    /// 启用通知
    pub enable_notifications: bool,
    /// 启用音效
    pub enable_sounds: bool,
    /// 窗口设置
    pub window_settings: WindowSettings,
    /// 安全设置
    pub security_settings: SecuritySettings,
}

impl AppSettings {
    pub fn new() -> Self {
        Self {
            language: Language::English,
            theme: AppTheme::Light,
            auto_save: true,
            session_timeout_minutes: 30,
            enable_notifications: true,
            enable_sounds: false,
            window_settings: WindowSettings::default(),
            security_settings: SecuritySettings::default(),
        }
    }

    /// 加载设置（从文件或注册表）
    pub fn load() -> Self {
        // TODO: 实现从配置文件加载设置
        Self::new()
    }

    /// 保存设置
    pub fn save(&self) -> Result<(), String> {
        // TODO: 实现保存设置到配置文件
        Ok(())
    }

    /// 重置为默认设置
    pub fn reset_to_defaults(&mut self) {
        *self = Self::new();
    }
}

impl Default for AppSettings {
    fn default() -> Self {
        Self::new()
    }
}

/// 应用程序主题
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AppTheme {
    Light,
    Dark,
    Auto, // 跟随系统
}

impl AppTheme {
    /// 获取所有可用主题
    pub fn all() -> Vec<AppTheme> {
        vec![AppTheme::Light, AppTheme::Dark, AppTheme::Auto]
    }

    /// 获取主题名称
    pub fn name(&self) -> &'static str {
        match self {
            AppTheme::Light => "Light",
            AppTheme::Dark => "Dark",
            AppTheme::Auto => "Auto",
        }
    }

    /// 获取主题描述
    pub fn description(&self) -> &'static str {
        match self {
            AppTheme::Light => "Light theme",
            AppTheme::Dark => "Dark theme",
            AppTheme::Auto => "Follow system theme",
        }
    }
}

/// 窗口设置
#[derive(Debug, Clone)]
pub struct WindowSettings {
    /// 窗口大小
    pub size: (f32, f32),
    /// 窗口位置
    pub position: Option<(f32, f32)>,
    /// 是否最大化
    pub maximized: bool,
    /// 是否总是在最前
    pub always_on_top: bool,
    /// 窗口透明度 (0.0 - 1.0)
    pub opacity: f32,
}

impl WindowSettings {
    pub fn new() -> Self {
        Self {
            size: (800.0, 600.0),
            position: None,
            maximized: false,
            always_on_top: false,
            opacity: 1.0,
        }
    }
}

impl Default for WindowSettings {
    fn default() -> Self {
        Self::new()
    }
}

/// 安全设置
#[derive(Debug, Clone)]
pub struct SecuritySettings {
    /// 启用自动锁定
    pub auto_lock: bool,
    /// 自动锁定时间（分钟）
    pub auto_lock_minutes: u64,
    /// 启用剪贴板清理
    pub clear_clipboard: bool,
    /// 剪贴板清理时间（秒）
    pub clipboard_clear_seconds: u64,
    /// 启用屏幕截图保护
    pub screenshot_protection: bool,
    /// 启用内存保护
    pub memory_protection: bool,
}

impl SecuritySettings {
    pub fn new() -> Self {
        Self {
            auto_lock: true,
            auto_lock_minutes: 30,
            clear_clipboard: true,
            clipboard_clear_seconds: 60,
            screenshot_protection: false,
            memory_protection: true,
        }
    }

    /// 获取高安全性设置
    pub fn high_security() -> Self {
        Self {
            auto_lock: true,
            auto_lock_minutes: 15,
            clear_clipboard: true,
            clipboard_clear_seconds: 30,
            screenshot_protection: true,
            memory_protection: true,
        }
    }

    /// 获取低安全性设置
    pub fn low_security() -> Self {
        Self {
            auto_lock: false,
            auto_lock_minutes: 60,
            clear_clipboard: false,
            clipboard_clear_seconds: 120,
            screenshot_protection: false,
            memory_protection: false,
        }
    }
}

impl Default for SecuritySettings {
    fn default() -> Self {
        Self::new()
    }
}

/// 用户界面状态
#[derive(Debug, Clone)]
pub struct UiState {
    /// 当前显示的面板
    pub current_panel: Panel,
    /// 侧边栏是否展开
    pub sidebar_expanded: bool,
    /// 是否显示高级选项
    pub show_advanced_options: bool,
    /// 选中的标签页
    pub selected_tab: usize,
    /// 对话框状态
    pub dialog_state: DialogState,
}

impl UiState {
    pub fn new() -> Self {
        Self {
            current_panel: Panel::Wallet,
            sidebar_expanded: true,
            show_advanced_options: false,
            selected_tab: 0,
            dialog_state: DialogState::None,
        }
    }

    /// 切换面板
    pub fn switch_panel(&mut self, panel: Panel) {
        self.current_panel = panel;
    }

    /// 切换侧边栏
    pub fn toggle_sidebar(&mut self) {
        self.sidebar_expanded = !self.sidebar_expanded;
    }

    /// 显示对话框
    pub fn show_dialog(&mut self, dialog: DialogState) {
        self.dialog_state = dialog;
    }

    /// 关闭对话框
    pub fn close_dialog(&mut self) {
        self.dialog_state = DialogState::None;
    }
}

impl Default for UiState {
    fn default() -> Self {
        Self::new()
    }
}

/// 面板枚举
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Panel {
    Wallet,
    Balance,
    Transfer,
    History,
    Settings,
}

impl Panel {
    /// 获取面板名称
    pub fn name(&self) -> &'static str {
        match self {
            Panel::Wallet => "Wallet",
            Panel::Balance => "Balance",
            Panel::Transfer => "Transfer",
            Panel::History => "History",
            Panel::Settings => "Settings",
        }
    }

    /// 获取所有面板
    pub fn all() -> Vec<Panel> {
        vec![
            Panel::Wallet,
            Panel::Balance,
            Panel::Transfer,
            Panel::History,
            Panel::Settings,
        ]
    }
}

/// 对话框状态
#[derive(Debug, Clone, PartialEq)]
pub enum DialogState {
    None,
    About,
    Settings,
    ConfirmLogout,
    ConfirmReset,
    Error(String),
    Info(String),
    Warning(String),
}

impl DialogState {
    /// 检查是否有对话框显示
    pub fn is_showing(&self) -> bool {
        !matches!(self, DialogState::None)
    }

    /// 获取对话框标题
    pub fn title(&self) -> &'static str {
        match self {
            DialogState::None => "",
            DialogState::About => "About",
            DialogState::Settings => "Settings",
            DialogState::ConfirmLogout => "Confirm Logout",
            DialogState::ConfirmReset => "Confirm Reset",
            DialogState::Error(_) => "Error",
            DialogState::Info(_) => "Information",
            DialogState::Warning(_) => "Warning",
        }
    }

    /// 获取对话框消息
    pub fn message(&self) -> Option<&str> {
        match self {
            DialogState::Error(msg) | DialogState::Info(msg) | DialogState::Warning(msg) => Some(msg),
            _ => None,
        }
    }
}
// 主视图 - 协调各个子视图模块
use crate::model::Model;
use crate::i18n::Language;
use eframe::egui;

// 导入子视图模块
pub mod auth_view;
pub mod wallet_view;
pub mod menu_view;
pub mod balance_view;

// 重新导出视图组件以便外部使用
pub use auth_view::AuthView;
pub use wallet_view::WalletView;
pub use menu_view::MenuView;

/// 视图动作枚举 - 定义用户可以触发的动作
#[derive(Debug, Clone, PartialEq)]
pub enum ViewAction {
    ImportKey,
    RefreshBalance,
    Logout,
    LanguageChanged(Language),
    None,
}

/// 主视图协调器
pub struct MainView;

impl MainView {
    /// 显示主应用程序界面
    pub fn show(model: &mut Model, ctx: &egui::Context) -> ViewAction {
        let mut action = ViewAction::None;

        // 首先显示菜单栏（如果已认证）
        if model.is_authenticated {
            if let Some(menu_action) = Self::show_menu_bar(model, ctx) {
                action = menu_action;
            }
        }

        // 显示主要内容区域
        action = Self::merge_actions(action, Self::show_main_content(model, ctx));

        action
    }

    /// 显示菜单栏
    fn show_menu_bar(model: &mut Model, ctx: &egui::Context) -> Option<ViewAction> {
        let menu_action = MenuView::show_top_menu_bar(model, ctx);
        match menu_action {
            ViewAction::None => None,
            other => Some(other),
        }
    }

    /// 显示主要内容区域
    fn show_main_content(model: &mut Model, ctx: &egui::Context) -> ViewAction {
        let mut action = ViewAction::None;

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(&model.i18n.tr("app_title"));
            ui.add_space(10.0);

            // 根据钱包状态显示不同的视图
            action = WalletView::show_wallet_content(model, ui);

            ui.add_space(10.0);
            ui.separator();

            // 显示状态和加载信息
            Self::show_status_section(model, ui);
        });

        action
    }

    /// 显示状态区域
    fn show_status_section(model: &Model, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            // 加载指示器
            if model.is_loading {
                ui.add(egui::Spinner::new());
                ui.label(&model.i18n.tr("loading"));
            }

            // 状态文本
            if !model.result_text.is_empty() {
                ui.separator();
                ui.label(&model.result_text);
            }
        });
    }

    /// 合并多个动作，优先返回非 None 的动作
    fn merge_actions(action1: ViewAction, action2: ViewAction) -> ViewAction {
        match (action1, action2) {
            (ViewAction::None, other) => other,
            (other, ViewAction::None) => other,
            (first, _) => first, // 优先返回第一个非 None 动作
        }
    }
}

// --- 向后兼容性函数 ---
// 为了不破坏现有代码，提供向后兼容的函数

/// 显示密码面板（向后兼容）
pub fn show_password_panel(model: &mut Model, ctx: &egui::Context) {
    AuthView::show_password_panel(model, ctx);
}

/// 绘制主界面，并返回用户触发的动作（向后兼容）
pub fn show(model: &mut Model, ctx: &egui::Context) -> ViewAction {
    MainView::show(model, ctx)
}
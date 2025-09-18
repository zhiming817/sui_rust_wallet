use crate::model::{Model, Network};
use crate::i18n::Language;
use crate::view::ViewAction;
use eframe::egui;

/// 菜单视图 - 处理菜单栏和导航相关的UI组件
pub struct MenuView;

impl MenuView {
    /// 显示顶部菜单栏
    pub fn show_top_menu_bar(model: &mut Model, ctx: &egui::Context) -> ViewAction {
        let mut action = ViewAction::None;
        
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                // 语言菜单
                if let Some(lang_action) = Self::show_language_menu(model, ui) {
                    action = lang_action;
                }
                
                ui.separator();
                
                // 网络菜单
                Self::show_network_menu(model, ui);
                
                ui.separator();
                
                // 工具菜单
                Self::show_tools_menu(model, ui);
                
                // 右侧状态显示
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    Self::show_status_indicators(model, ui);
                });
            });
        });
        
        action
    }

    /// 显示语言选择菜单
    fn show_language_menu(model: &mut Model, ui: &mut egui::Ui) -> Option<ViewAction> {
        let mut action = None;
        
        ui.menu_button(&model.i18n.tr("language_label"), |ui| {
            ui.style_mut().wrap = Some(false);
            
            for lang in Language::all() {
                let current_lang = model.current_language();
                let is_selected = current_lang == lang;
                
                ui.horizontal(|ui| {
                    if is_selected {
                        ui.label("✓");
                    } else {
                        ui.label("  ");
                    }
                    
                    if ui.selectable_label(is_selected, lang.display_name()).clicked() {
                        if lang != current_lang {
                            action = Some(ViewAction::LanguageChanged(lang));
                        }
                    }
                });
            }
        });
        
        action
    }

    /// 显示网络选择菜单
    fn show_network_menu(model: &mut Model, ui: &mut egui::Ui) {
        ui.menu_button(&model.i18n.tr("network_label"), |ui| {
            ui.style_mut().wrap = Some(false);
            
            let networks = [
                (Network::Devnet, "devnet"),
                (Network::Testnet, "testnet"),
                (Network::Mainnet, "mainnet"),
            ];
            
            for (network, key) in networks {
                let is_selected = model.network == network;
                
                ui.horizontal(|ui| {
                    if is_selected {
                        ui.label("✓");
                    } else {
                        ui.label("  ");
                    }
                    
                    if ui.selectable_label(is_selected, &model.i18n.tr(key)).clicked() {
                        model.network = network;
                    }
                });
            }
        });
    }

    /// 显示工具菜单
    fn show_tools_menu(model: &Model, ui: &mut egui::Ui) {
        ui.menu_button(&model.i18n.tr("tools_label"), |ui| {
            if ui.button(&model.i18n.tr("clear_cache")).clicked() {
                // 清除缓存的逻辑
                ui.close_menu();
            }
            
            if ui.button(&model.i18n.tr("export_logs")).clicked() {
                // 导出日志的逻辑
                ui.close_menu();
            }
            
            ui.separator();
            
            if ui.button(&model.i18n.tr("about")).clicked() {
                // 显示关于信息的逻辑
                ui.close_menu();
            }
        });
    }

    /// 显示状态指示器
    fn show_status_indicators(model: &Model, ui: &mut egui::Ui) {
        // 连接状态指示器
        Self::show_connection_status(model, ui);
        
        ui.separator();
        
        // 当前网络显示
        Self::show_current_network(model, ui);
        
        ui.separator();
        
        // 语言指示器
        Self::show_current_language(model, ui);
    }

    /// 显示连接状态
    fn show_connection_status(model: &Model, ui: &mut egui::Ui) {
        let (color, icon, tooltip) = if model.is_loading {
            (egui::Color32::YELLOW, "🔄", model.i18n.tr("status_loading"))
        } else {
            // 根据是否有钱包和网络连接状态决定
            match &model.wallet {
                crate::model::WalletState::Loaded { .. } => {
                    (egui::Color32::GREEN, "🟢", model.i18n.tr("status_connected"))
                }
                crate::model::WalletState::NoWallet { .. } => {
                    (egui::Color32::GRAY, "⚪", model.i18n.tr("status_no_wallet"))
                }
            }
        };
        
        ui.colored_label(color, icon)
            .on_hover_text(tooltip);
    }

    /// 显示当前网络
    fn show_current_network(model: &Model, ui: &mut egui::Ui) {
        let network_text = match model.network {
            Network::Devnet => "DEV",
            Network::Testnet => "TEST",
            Network::Mainnet => "MAIN",
        };
        
        let color = match model.network {
            Network::Devnet => egui::Color32::BLACK,
            Network::Testnet => egui::Color32::BLACK,
            Network::Mainnet => egui::Color32::BLACK,
        };
        
        ui.colored_label(color, network_text)
            .on_hover_text(&model.i18n.tr(&format!("{:?}", model.network).to_lowercase()));
    }

    /// 显示当前语言
    fn show_current_language(model: &Model, ui: &mut egui::Ui) {
        let lang_code = match model.current_language() {
            Language::English => "EN",
            Language::Chinese => "中",
        };
        
        ui.small(lang_code)
            .on_hover_text(&model.i18n.tr("current_language"));
    }

    /// 显示侧边菜单（可选功能）
    pub fn show_side_menu(model: &Model, ui: &mut egui::Ui) {
        ui.group(|ui| {
            ui.vertical(|ui| {
                ui.heading(&model.i18n.tr("quick_actions"));
                ui.separator();
                
                if ui.button(&model.i18n.tr("refresh_balance_button")).clicked() {
                    // 刷新余额
                }
                
                if ui.button(&model.i18n.tr("copy_address_button")).clicked() {
                    // 复制地址
                }
                
                ui.separator();
                
                if ui.button(&model.i18n.tr("settings")).clicked() {
                    // 打开设置
                }
            });
        });
    }

    /// 显示上下文菜单（右键菜单）
    pub fn show_context_menu(model: &Model, ui: &mut egui::Ui, response: &egui::Response) {
        response.context_menu(|ui| {
            if ui.button(&model.i18n.tr("copy")).clicked() {
                ui.close_menu();
            }
            
            if ui.button(&model.i18n.tr("paste")).clicked() {
                ui.close_menu();
            }
            
            ui.separator();
            
            if ui.button(&model.i18n.tr("select_all")).clicked() {
                ui.close_menu();
            }
        });
    }

    /// 显示快捷键提示
    pub fn show_shortcuts_tooltip(model: &Model, ui: &mut egui::Ui) {
        ui.group(|ui| {
            ui.vertical(|ui| {
                ui.heading(&model.i18n.tr("keyboard_shortcuts"));
                ui.separator();
                
                ui.horizontal(|ui| {
                    ui.code("Ctrl+C");
                    ui.label(&model.i18n.tr("copy"));
                });
                
                ui.horizontal(|ui| {
                    ui.code("Ctrl+V");
                    ui.label(&model.i18n.tr("paste"));
                });
                
                ui.horizontal(|ui| {
                    ui.code("F5");
                    ui.label(&model.i18n.tr("refresh"));
                });
                
                ui.horizontal(|ui| {
                    ui.code("Esc");
                    ui.label(&model.i18n.tr("close"));
                });
            });
        });
    }
}
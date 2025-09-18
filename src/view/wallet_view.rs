use crate::model::{Model, WalletState};
use crate::view::ViewAction;
use eframe::egui;
use sui_sdk::types::base_types::SuiAddress;

/// 钱包视图 - 处理钱包管理相关的UI组件
pub struct WalletView;

impl WalletView {
    /// 显示钱包相关的UI内容
    pub fn show_wallet_content(model: &mut Model, ui: &mut egui::Ui) -> ViewAction {
        match &model.wallet {
            WalletState::NoWallet { .. } => {
                Self::show_import_wallet_form(model, ui)
            }
            WalletState::Loaded { address, .. } => {
                let addr = *address; // 复制地址以避免借用问题
                Self::show_loaded_wallet_info(model, &addr, ui)
            }
        }
    }

    /// 显示导入钱包表单
    fn show_import_wallet_form(
        model: &mut Model, 
        ui: &mut egui::Ui
    ) -> ViewAction {
        let mut action = ViewAction::None;
        
        ui.group(|ui| {
            ui.vertical(|ui| {
                ui.heading(&model.i18n.tr("import_wallet_title"));
                ui.add_space(8.0);
                
                ui.label(&model.i18n.tr("import_wallet_message"));
                ui.add_space(4.0);
                
                // 私钥输入框
                if let WalletState::NoWallet { private_key_input } = &mut model.wallet {
                    ui.add(
                        egui::TextEdit::multiline(private_key_input)
                            .password(true)
                            .hint_text(&model.i18n.tr("private_key_hint"))
                            .desired_rows(3)
                    );
                }
                
                ui.add_space(8.0);
                
                // 导入按钮和格式说明
                ui.horizontal(|ui| {
                    if ui.button(&model.i18n.tr("import_wallet_button")).clicked() {
                        action = ViewAction::ImportKey;
                    }
                    
                    ui.separator();
                    
                    ui.label(&model.i18n.tr("supported_formats"));
                });
                
                ui.add_space(4.0);
                Self::show_format_help(model, ui);
            });
        });
        
        action
    }

    /// 显示已加载钱包的信息
    fn show_loaded_wallet_info(
        model: &Model, 
        address: &SuiAddress, 
        ui: &mut egui::Ui
    ) -> ViewAction {
        let mut action = ViewAction::None;
        
        ui.group(|ui| {
            ui.vertical(|ui| {
                ui.heading(&model.i18n.tr("wallet_loaded"));
                ui.add_space(8.0);
                
                // 钱包信息网格
                egui::Grid::new("wallet_info")
                    .num_columns(2)
                    .spacing([40.0, 4.0])
                    .show(ui, |ui| {
                        Self::show_wallet_info_grid(model, address, ui);
                    });
                
                ui.add_space(12.0);
                
                // 操作按钮
                ui.horizontal(|ui| {
                    if ui.button(&model.i18n.tr("refresh_balance_button")).clicked() {
                        action = ViewAction::RefreshBalance;
                    }
                    
                    ui.separator();
                    
                    if ui.button(&model.i18n.tr("copy_address_button")).clicked() {
                        ui.ctx().copy_text(address.to_string());
                    }
                    
                    ui.separator();
                    
                    if ui.button(&model.i18n.tr("logout_button")).clicked() {
                        action = ViewAction::Logout;
                    }
                });
            });
        });
        
        action
    }

    /// 显示钱包信息网格
    fn show_wallet_info_grid(model: &Model, address: &SuiAddress, ui: &mut egui::Ui) {
        // 地址行
        ui.label(&model.i18n.tr("address_label"));
        ui.horizontal(|ui| {
            // 截断显示长地址
            let address_str = address.to_string();
            let display_address = if address_str.len() > 20 {
                format!("{}...{}", &address_str[..10], &address_str[address_str.len()-10..])
            } else {
                address_str.clone()
            };
            
            if ui.add(egui::Label::new(display_address).sense(egui::Sense::click())).clicked() {
                ui.ctx().copy_text(address_str);
            }
            
            // 复制图标或按钮
            if ui.small_button("📋").clicked() {
                ui.ctx().copy_text(address.to_string());
            }
        });
        ui.end_row();

        // 网络行
        ui.label(&model.i18n.tr("network_label"));
        ui.label(&model.i18n.tr(&format!("{:?}", model.network).to_lowercase()));
        ui.end_row();

        // 余额行 - 这里可以显示余额信息
        ui.label(&model.i18n.tr("balance_label"));
        ui.horizontal(|ui| {
            if model.is_loading {
                ui.add(egui::Spinner::new().size(16.0));
                ui.label(&model.i18n.tr("loading"));
            } else {
                // 余额信息显示在 result_text 中
                let balance_text = if model.result_text.contains("SUI") {
                    model.result_text.clone()
                } else {
                    model.i18n.tr("balance_unknown")
                };
                ui.label(balance_text);
            }
        });
        ui.end_row();
    }

    /// 显示支持的私钥格式帮助信息
    fn show_format_help(model: &Model, ui: &mut egui::Ui) {
        ui.collapsing(&model.i18n.tr("format_help_title"), |ui| {
            ui.label(&model.i18n.tr("format_help_bech32"));
            ui.code("suiprivkey1...");
            ui.add_space(4.0);
            
            ui.label(&model.i18n.tr("format_help_base64"));
            ui.code("Base64 encoded key (44 characters)");
            ui.add_space(4.0);
            
            ui.label(&model.i18n.tr("format_help_hex"));
            ui.code("Hex string (64 characters)");
        });
    }

    /// 显示私钥验证状态
    pub fn show_key_validation_status(private_key: &str, model: &Model, ui: &mut egui::Ui) {
        if !private_key.is_empty() {
            let is_valid = Self::validate_private_key_format(private_key);
            let (color, text) = if is_valid {
                (egui::Color32::GREEN, model.i18n.tr("valid_format"))
            } else {
                (egui::Color32::RED, model.i18n.tr("invalid_format"))
            };
            
            ui.horizontal(|ui| {
                ui.label(&model.i18n.tr("format_status"));
                ui.colored_label(color, text);
            });
        }
    }

    /// 验证私钥格式
    fn validate_private_key_format(private_key: &str) -> bool {
        let trimmed = private_key.trim();
        !trimmed.is_empty() && (
            trimmed.starts_with("suiprivkey1") || // Bech32 format
            (trimmed.len() == 44 && trimmed.chars().all(|c| c.is_alphanumeric() || c == '+' || c == '/' || c == '=')) || // Base64
            (trimmed.len() == 64 && trimmed.chars().all(|c| c.is_ascii_hexdigit())) // Hex
        )
    }

    /// 显示安全提示
    pub fn show_security_warning(model: &Model, ui: &mut egui::Ui) {
        ui.group(|ui| {
            ui.horizontal(|ui| {
                ui.label("⚠️");
                ui.vertical(|ui| {
                    ui.label(&model.i18n.tr("security_warning_title"));
                    ui.small(&model.i18n.tr("security_warning_message"));
                });
            });
        });
    }
}
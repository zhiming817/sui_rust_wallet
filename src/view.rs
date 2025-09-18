use crate::model::{Model, Network, WalletState};
use crate::i18n::Language;
use eframe::egui;
use crate::controller;

pub enum ViewAction {
    ImportKey,
    RefreshBalance,
    Logout,
    LanguageChanged(Language),
    None,
}

/// 显示密码面板（首次设置或登录）
pub fn show_password_panel(model: &mut Model, ctx: &egui::Context) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.vertical_centered(|ui| {
            ui.heading(&model.i18n.tr("login_title"));

            ui.add_space(8.0);

            if model.is_first_run {
                ui.label(&model.i18n.tr("first_run_message"));
                ui.add(egui::TextEdit::singleline(&mut model.password_input).password(true).hint_text(&model.i18n.tr("enter_password")));
                ui.add(egui::TextEdit::singleline(&mut model.password_confirm).password(true).hint_text(&model.i18n.tr("confirm_password")));
                ui.add_space(6.0);
                if ui.button(&model.i18n.tr("create_password_button")).clicked() {
                    // Call controller to set password and handle result
                    if let Err(err) = controller::handle_set_password(model) {
                        // Simple popup notification (can be improved)
                        model.password_input.clear();
                        model.password_confirm.clear();
                        // Use egui::popup or a centralized error display, here we just log to console
                        eprintln!("Failed to set password: {}", err);
                    }
                }
            } else {
                ui.label(&model.i18n.tr("login_message"));
                ui.add(egui::TextEdit::singleline(&mut model.password_input).password(true).hint_text(&model.i18n.tr("enter_password")));
                ui.add_space(6.0);
                ui.horizontal(|ui| {
                    if ui.button(&model.i18n.tr("login_button")).clicked() {
                        // Call controller to verify password
                        if let Err(err) = controller::handle_verify_password(model) {
                            eprintln!("Password verification failed: {}", err);
                        }
                        // Clear input to prevent leaks
                        model.password_input.clear();
                    }
                    if ui.button(&model.i18n.tr("exit_button")).clicked() {
                        // Close the application directly: the simple way is to exit the process
                        std::process::exit(0);
                    }
                });
            }

            ui.add_space(12.0);
            ui.label(&model.i18n.tr("password_info"));
        });
    });
}

/// 绘制主界面，并返回用户触发的动作
pub fn show(model: &mut Model, ctx: &egui::Context) -> ViewAction {
    let mut action = ViewAction::None;

    egui::CentralPanel::default().show(ctx, |ui| {
        ui.heading(&model.i18n.tr("app_title"));
        ui.add_space(10.0);

        // --- 语言切换器 ---
        ui.horizontal(|ui| {
            ui.label(&model.i18n.tr("language_label"));
            let mut current_lang = model.current_language();
            for lang in Language::all() {
                if ui.selectable_value(&mut current_lang, lang, lang.display_name()).clicked() {
                    if lang != model.current_language() {
                        action = ViewAction::LanguageChanged(lang);
                    }
                }
            }
        });
        ui.separator();

        // --- 网络切换器 ---
        ui.horizontal(|ui| {
            ui.label(&model.i18n.tr("network_label"));
            ui.selectable_value(&mut model.network, Network::Devnet, &model.i18n.tr("devnet"));
            ui.selectable_value(&mut model.network, Network::Testnet, &model.i18n.tr("testnet"));
            ui.selectable_value(&mut model.network, Network::Mainnet, &model.i18n.tr("mainnet"));
        });
        ui.separator();

        // 根据钱包状态显示不同视图
        match &mut model.wallet {
            WalletState::NoWallet { private_key_input } => {
                ui.label(&model.i18n.tr("import_wallet_message"));
                ui.add(egui::TextEdit::singleline(private_key_input).password(true));
                if ui.button(&model.i18n.tr("import_wallet_button")).clicked() {
                    action = ViewAction::ImportKey;
                }
            }
            WalletState::Loaded { address, .. } => {
                ui.heading(&model.i18n.tr("wallet_loaded"));
                egui::Grid::new("wallet_info").num_columns(2).show(ui, |ui| {
                    ui.label(&model.i18n.tr("address_label"));
                    // 允许点击复制地址
                    if ui.add(egui::Label::new(address.to_string()).sense(egui::Sense::click())).clicked() {
                        ctx.copy_text(address.to_string());
                    };
                    ui.end_row();

                    ui.label(&model.i18n.tr("balance_label"));
                    // 余额现在显示在 result_text 中，这里直接使用
                    // ui.label(&model.balance_text);
                    ui.end_row();
                });

                if ui.button(&model.i18n.tr("refresh_balance_button")).clicked() {
                    action = ViewAction::RefreshBalance;
                }
                if ui.button(&model.i18n.tr("logout_button")).clicked() {
                    action = ViewAction::Logout;
                }
            }
        }

        // --- 通用加载和状态显示 ---
        if model.is_loading {
            ui.add(egui::Spinner::new());
        }
        ui.add_space(10.0);
        ui.separator();
        // 将余额和状态信息统一显示在这里
        ui.label(&model.result_text);
    });

    action
}
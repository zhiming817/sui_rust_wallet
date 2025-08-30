use crate::model::{Model, Network, WalletState};
use eframe::egui;
use crate::controller;

pub enum ViewAction {
    ImportKey,
    RefreshBalance,
    Logout,
    None,
}

/// 显示密码面板（首次设置或登录）
pub fn show_password_panel(model: &mut Model, ctx: &egui::Context) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.vertical_centered(|ui| {
            ui.heading("Sui Rust Wallet - Login");

            ui.add_space(8.0);

            if model.is_first_run {
                ui.label("First run: Please set a password (for local encryption)");
                ui.add(egui::TextEdit::singleline(&mut model.password_input).password(true).hint_text("Enter password"));
                ui.add(egui::TextEdit::singleline(&mut model.password_confirm).password(true).hint_text("Confirm password"));
                ui.add_space(6.0);
                if ui.button("Create Password and Enter").clicked() {
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
                ui.label("Please enter your password to log in");
                ui.add(egui::TextEdit::singleline(&mut model.password_input).password(true).hint_text("Enter password"));
                ui.add_space(6.0);
                ui.horizontal(|ui| {
                    if ui.button("Login").clicked() {
                        // Call controller to verify password
                        if let Err(err) = controller::handle_verify_password(model) {
                            eprintln!("Password verification failed: {}", err);
                        }
                        // Clear input to prevent leaks
                        model.password_input.clear();
                    }
                    if ui.button("Exit").clicked() {
                        // Close the application directly: the simple way is to exit the process
                        std::process::exit(0);
                    }
                });
            }

            ui.add_space(12.0);
            ui.label("The password will be encrypted with the Argon2 algorithm and saved in the local configuration directory.");
        });
    });
}

/// 绘制主界面，并返回用户触发的动作
pub fn show(model: &mut Model, ctx: &egui::Context) -> ViewAction {
    let mut action = ViewAction::None;

    egui::CentralPanel::default().show(ctx, |ui| {
        ui.heading("Simple Sui Wallet");
        ui.add_space(10.0);

        // --- 网络切换器 ---
        ui.horizontal(|ui| {
            ui.label("Network:");
            ui.selectable_value(&mut model.network, Network::Devnet, "Devnet");
            ui.selectable_value(&mut model.network, Network::Testnet, "Testnet");
            ui.selectable_value(&mut model.network, Network::Mainnet, "Mainnet");
        });
        ui.separator();

        // 根据钱包状态显示不同视图
        match &mut model.wallet {
            WalletState::NoWallet { private_key_input } => {
                ui.label("Import your wallet using a Base64 private key:");
                ui.add(egui::TextEdit::singleline(private_key_input).password(true));
                if ui.button("Import Wallet").clicked() {
                    action = ViewAction::ImportKey;
                }
            }
            WalletState::Loaded { address, .. } => {
                ui.heading("Wallet Loaded");
                egui::Grid::new("wallet_info").num_columns(2).show(ui, |ui| {
                    ui.label("Address:");
                    // 允许点击复制地址
                    if ui.add(egui::Label::new(address.to_string()).sense(egui::Sense::click())).clicked() {
                        ui.output_mut(|o| o.copied_text = address.to_string());
                    };
                    ui.end_row();

                    ui.label("Balance:");
                    // 余额现在显示在 result_text 中，这里直接使用
                    // ui.label(&model.balance_text);
                    ui.end_row();
                });

                if ui.button("Refresh Balance").clicked() {
                    action = ViewAction::RefreshBalance;
                }
                if ui.button("Logout").clicked() {
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
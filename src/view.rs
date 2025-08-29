use crate::model::{Model, Network, WalletState};
use egui::{Button, CentralPanel, Context, Grid, Sense, Spinner, TextEdit};

/// 定义视图可以触发的动作
#[derive(PartialEq)]
pub enum ViewAction {
    None,
    ImportKey,
    RefreshBalance,
    Logout,
}

/// 绘制主界面，并返回用户触发的动作
pub fn show(model: &mut Model, ctx: &Context) -> ViewAction {
    let mut action = ViewAction::None;

    CentralPanel::default().show(ctx, |ui| {
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
                ui.add(TextEdit::singleline(private_key_input).password(true));
                if ui.button("Import Wallet").clicked() {
                    action = ViewAction::ImportKey;
                }
            }
            WalletState::Loaded { address, .. } => {
                ui.heading("Wallet Loaded");
                Grid::new("wallet_info").num_columns(2).show(ui, |ui| {
                    ui.label("Address:");
                    // 允许点击复制地址
                    if ui.add(egui::Label::new(address.to_string()).sense(Sense::click())).clicked() {
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
            ui.add(Spinner::new());
        }
        ui.add_space(10.0);
        ui.separator();
        // 将余额和状态信息统一显示在这里
        ui.label(&model.result_text);
    });

    action
}
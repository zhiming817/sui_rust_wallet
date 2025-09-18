use crate::model::Model;
use eframe::egui;

/// 余额视图 - 处理余额显示和操作相关的UI组件
pub struct BalanceView;

impl BalanceView {
    /// 显示余额信息面板
    pub fn show_balance_panel(model: &Model, ui: &mut egui::Ui) {
        ui.group(|ui| {
            ui.vertical(|ui| {
                ui.heading(&model.i18n.tr("balance_title"));
                ui.separator();
                
                if model.is_loading {
                    Self::show_loading_balance(model, ui);
                } else {
                    Self::show_balance_details(model, ui);
                }
            });
        });
    }

    /// 显示余额加载状态
    fn show_loading_balance(model: &Model, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.add(egui::Spinner::new());
            ui.label(&model.i18n.tr("refreshing_balance"));
        });
    }

    /// 显示余额详细信息
    fn show_balance_details(model: &Model, ui: &mut egui::Ui) {
        let balance_text = &model.result_text;
        
        if balance_text.contains("SUI") {
            Self::show_sui_balance(balance_text, model, ui);
        } else if balance_text.contains(&model.i18n.tr("async_error")) {
            Self::show_balance_error(balance_text, model, ui);
        } else {
            Self::show_general_status(balance_text, model, ui);
        }
    }

    /// 显示 SUI 余额
    fn show_sui_balance(balance_text: &str, model: &Model, ui: &mut egui::Ui) {
        // 解析余额数值
        if let Some(amount) = Self::parse_sui_balance(balance_text) {
            // 主余额显示
            ui.horizontal(|ui| {
                ui.heading("💰");
                ui.vertical(|ui| {
                    ui.heading(format!("{:.4}", amount));
                    ui.label("SUI");
                });
            });
            
            ui.add_space(8.0);
            
            // 余额统计
            Self::show_balance_stats(amount, model, ui);
            
            ui.add_space(8.0);
            
            // 余额操作按钮
            Self::show_balance_actions(model, ui);
        } else {
            ui.label(balance_text);
        }
    }

    /// 显示余额错误信息
    fn show_balance_error(error_text: &str, model: &Model, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("❌");
            ui.vertical(|ui| {
                ui.colored_label(egui::Color32::RED, &model.i18n.tr("balance_error"));
                ui.small(error_text);
            });
        });
        
        ui.add_space(8.0);
        
        if ui.button(&model.i18n.tr("retry_button")).clicked() {
            // 重试逻辑将在主视图中处理
        }
    }

    /// 显示一般状态信息
    fn show_general_status(status_text: &str, _model: &Model, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("ℹ️");
            ui.label(status_text);
        });
    }

    /// 显示余额统计信息
    fn show_balance_stats(amount: f64, model: &Model, ui: &mut egui::Ui) {
        egui::Grid::new("balance_stats")
            .num_columns(2)
            .spacing([20.0, 4.0])
            .show(ui, |ui| {
                // 当前余额
                ui.label(&model.i18n.tr("current_balance"));
                ui.label(format!("{:.4} SUI", amount));
                ui.end_row();
                
                // 估算USD价值（假设价格）
                let estimated_usd = amount * 2.5; // 假设的SUI价格
                ui.label(&model.i18n.tr("estimated_value"));
                ui.label(format!("≈ ${:.2} USD", estimated_usd));
                ui.end_row();
                
                // 网络费用估算
                ui.label(&model.i18n.tr("network_fee"));
                ui.label("~0.001 SUI");
                ui.end_row();
            });
    }

    /// 显示余额操作按钮
    fn show_balance_actions(model: &Model, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            if ui.button(&model.i18n.tr("refresh_balance_button")).clicked() {
                // 刷新操作将在主视图中处理
            }
            
            ui.separator();
            
            if ui.button(&model.i18n.tr("send_button")).clicked() {
                // 发送操作（未来功能）
            }
            
            if ui.button(&model.i18n.tr("receive_button")).clicked() {
                // 接收操作（显示地址二维码等）
            }
        });
    }

    /// 显示历史记录面板
    pub fn show_transaction_history(model: &Model, ui: &mut egui::Ui) {
        ui.group(|ui| {
            ui.vertical(|ui| {
                ui.heading(&model.i18n.tr("transaction_history"));
                ui.separator();
                
                // 这里显示交易历史记录
                // 目前显示占位符
                ui.label(&model.i18n.tr("no_transactions"));
                
                ui.add_space(8.0);
                
                if ui.button(&model.i18n.tr("view_explorer")).clicked() {
                    // 在区块链浏览器中查看地址
                    if let crate::model::WalletState::Loaded { address, .. } = &model.wallet {
                        let explorer_url = match model.network {
                            crate::model::Network::Mainnet => format!("https://suiexplorer.com/address/{}", address),
                            crate::model::Network::Testnet => format!("https://suiexplorer.com/address/{}?network=testnet", address),
                            crate::model::Network::Devnet => format!("https://suiexplorer.com/address/{}?network=devnet", address),
                        };
                        
                        // 这里可以打开浏览器（需要实现）
                        println!("Open URL: {}", explorer_url);
                    }
                }
            });
        });
    }

    /// 显示余额图表（可选功能）
    pub fn show_balance_chart(model: &Model, ui: &mut egui::Ui) {
        ui.group(|ui| {
            ui.vertical(|ui| {
                ui.heading(&model.i18n.tr("balance_chart"));
                ui.separator();
                
                // 占位符图表
                let available_rect = ui.available_rect_before_wrap();
                let chart_rect = egui::Rect::from_min_size(
                    available_rect.min,
                    egui::Vec2::new(available_rect.width(), 100.0)
                );
                
                ui.allocate_ui_at_rect(chart_rect, |ui| {
                    ui.painter().rect_filled(
                        chart_rect,
                        4.0,
                        egui::Color32::from_gray(240)
                    );
                    
                    ui.centered_and_justified(|ui| {
                        ui.label(&model.i18n.tr("chart_placeholder"));
                    });
                });
            });
        });
    }

    /// 解析 SUI 余额字符串
    fn parse_sui_balance(balance_text: &str) -> Option<f64> {
        balance_text
            .trim_end_matches(" SUI")
            .trim()
            .parse::<f64>()
            .ok()
    }

    /// 格式化余额显示
    pub fn format_balance(amount: f64) -> String {
        if amount >= 1_000_000.0 {
            format!("{:.2}M SUI", amount / 1_000_000.0)
        } else if amount >= 1_000.0 {
            format!("{:.2}K SUI", amount / 1_000.0)
        } else {
            format!("{:.4} SUI", amount)
        }
    }

    /// 获取余额颜色（根据数量）
    pub fn get_balance_color(amount: f64) -> egui::Color32 {
        if amount >= 100.0 {
            egui::Color32::GREEN
        } else if amount >= 10.0 {
            egui::Color32::YELLOW
        } else if amount > 0.0 {
            egui::Color32::ORANGE
        } else {
            egui::Color32::RED
        }
    }

    /// 显示余额警告（如果余额过低）
    pub fn show_low_balance_warning(amount: f64, model: &Model, ui: &mut egui::Ui) {
        if amount < 1.0 && amount > 0.0 {
            ui.group(|ui| {
                ui.horizontal(|ui| {
                    ui.label("⚠️");
                    ui.vertical(|ui| {
                        ui.colored_label(egui::Color32::YELLOW, &model.i18n.tr("low_balance_warning"));
                        ui.small(&model.i18n.tr("consider_adding_funds"));
                    });
                });
            });
        }
    }
}
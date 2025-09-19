use crate::model::Model;
use crate::controller;
use eframe::egui;

/// 认证视图 - 处理登录、密码设置相关的UI组件
pub struct AuthView;

impl AuthView {
    /// 显示密码面板（首次设置或登录）
    pub fn show_password_panel(model: &mut Model, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading(&model.i18n.tr("login_title"));
                ui.add_space(8.0);

                if model.auth_state.is_first_run {
                    Self::show_password_setup(model, ui);
                } else {
                    Self::show_login_form(model, ui);
                }

                ui.add_space(12.0);
                ui.label(&model.i18n.tr("password_info"));
            });
        });
    }

    /// 显示密码设置表单（首次运行）
    fn show_password_setup(model: &mut Model, ui: &mut egui::Ui) {
        ui.label(&model.i18n.tr("first_run_message"));
        
        ui.add(
            egui::TextEdit::singleline(&mut model.auth_state.password_input)
                .password(true)
                .hint_text(&model.i18n.tr("enter_password"))
        );
        
        ui.add(
            egui::TextEdit::singleline(&mut model.auth_state.password_confirm)
                .password(true)
                .hint_text(&model.i18n.tr("confirm_password"))
        );
        
        ui.add_space(6.0);
        
        if ui.button(&model.i18n.tr("create_password_button")).clicked() {
            if let Err(err) = controller::handle_set_password(model) {
                model.auth_state.password_input.clear();
                model.auth_state.password_confirm.clear();
                eprintln!("Failed to set password: {}", err);
            }
        }
    }

    /// 显示登录表单
    fn show_login_form(model: &mut Model, ui: &mut egui::Ui) {
        ui.label(&model.i18n.tr("login_message"));
        
        ui.add(
            egui::TextEdit::singleline(&mut model.auth_state.password_input)
                .password(true)
                .hint_text(&model.i18n.tr("enter_password"))
        );
        
        ui.add_space(6.0);
        
        // 居中对齐的按钮
        Self::show_centered_buttons(model, ui);
    }

    /// 显示居中对齐的登录和退出按钮
    fn show_centered_buttons(model: &mut Model, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            // 计算居中需要的空间
            let available_width = ui.available_width();
            let button_width = 60.0; // 按钮宽度估算
            let spacing = 10.0; // 按钮间距
            let total_buttons_width = button_width * 2.0 + spacing;
            let left_padding = (available_width - total_buttons_width) / 2.0;
            
            if left_padding > 0.0 {
                ui.add_space(left_padding);
            }
            
            if ui.button(&model.i18n.tr("login_button")).clicked() {
                if let Err(err) = controller::handle_verify_password(model) {
                    eprintln!("Password verification failed: {}", err);
                }
                model.auth_state.password_input.clear();
            }
            
            ui.add_space(spacing);
            
            if ui.button(&model.i18n.tr("exit_button")).clicked() {
                std::process::exit(0);
            }
        });
    }

    /// 显示密码强度指示器（可选功能）
    pub fn show_password_strength(password: &str, ui: &mut egui::Ui) {
        let strength = Self::calculate_password_strength(password);
        let (color, text) = match strength {
            0..=2 => (egui::Color32::RED, "弱"),
            3..=4 => (egui::Color32::YELLOW, "中等"),
            5..=6 => (egui::Color32::GREEN, "强"),
            _ => (egui::Color32::DARK_GREEN, "非常强"),
        };
        
        ui.horizontal(|ui| {
            ui.label("密码强度:");
            ui.colored_label(color, text);
        });
    }

    /// 计算密码强度分数
    fn calculate_password_strength(password: &str) -> u8 {
        let mut score = 0;
        
        if password.len() >= 8 { score += 1; }
        if password.len() >= 12 { score += 1; }
        if password.chars().any(|c| c.is_lowercase()) { score += 1; }
        if password.chars().any(|c| c.is_uppercase()) { score += 1; }
        if password.chars().any(|c| c.is_numeric()) { score += 1; }
        if password.chars().any(|c| !c.is_alphanumeric()) { score += 1; }
        
        score
    }

    /// 显示错误消息（可选功能）
    pub fn show_error_message(message: &str, ui: &mut egui::Ui) {
        ui.colored_label(egui::Color32::RED, message);
    }

    /// 显示成功消息（可选功能）
    pub fn show_success_message(message: &str, ui: &mut egui::Ui) {
        ui.colored_label(egui::Color32::GREEN, message);
    }
}
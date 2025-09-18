mod controller;
mod model;
mod view;
mod i18n;

use eframe::{egui, App, Frame};
use model::Model;
use view::ViewAction; // 导入 ViewAction 枚举

impl App for Model {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        // 新增：未认证时显示密码面板（首次设置或登录）
        if !self.is_authenticated {
            view::show_password_panel(self, ctx);
            ctx.request_repaint();
            return;
        }

        // Controller: 处理后台消息
        controller::handle_async_results(self);

        // View: 绘制 UI 并获取用户动作
        let action = view::show(self, ctx);

        // Controller: 根据用户动作执行相应逻辑
        if !self.is_loading {
            match action {
                ViewAction::ImportKey => controller::handle_import_key(self),
                ViewAction::RefreshBalance => controller::handle_refresh_balance(self),
                ViewAction::Logout => controller::handle_logout(self),
                ViewAction::LanguageChanged(lang) => self.set_language(lang),
                ViewAction::None => {}
            }
        }

        // 持续请求重绘
        ctx.request_repaint();
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([550.0, 450.0]),
        ..Default::default()
    };

    let model = Model::default();
    let window_title = model.i18n.tr("app_title");

    eframe::run_native(
        &window_title,
        options,
        Box::new(|cc| {
            // 字体设置 - 使用 egui 内置字体支持中文
            let ctx = &cc.egui_ctx;
            let mut fonts = egui::FontDefinitions::default();
            
            // 尝试加载字体文件，如果失败则使用系统字体
            if let Ok(font_data) = std::fs::read("assets/NotoSansSC-Regular.ttf") {
                fonts.font_data.insert(
                    "noto_sans_sc".to_owned(),
                    egui::FontData::from_owned(font_data).into(),
                );
                // 将中文字体设为首选
                fonts.families.entry(egui::FontFamily::Proportional).or_default().insert(0, "noto_sans_sc".to_owned());
                fonts.families.entry(egui::FontFamily::Monospace).or_default().insert(0, "noto_sans_sc".to_owned());
            }
            
            ctx.set_fonts(fonts);

            Ok(Box::new(Model::default()))
        }),
    )
}


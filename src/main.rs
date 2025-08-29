mod controller;
mod model;
mod view;

use eframe::{egui, App, Frame};
use model::Model;
use view::ViewAction; // 导入 ViewAction 枚举

impl App for Model {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
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
    eframe::run_native(
        "Simple Sui Wallet", // 更新窗口标题
        options,
        Box::new(|_cc| Ok(Box::<Model>::default())),
    )
}


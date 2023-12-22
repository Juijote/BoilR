use copypasta::ClipboardProvider;
use eframe::egui;
use egui::ScrollArea;

use super::{
    ui_colors::{BACKGROUND_COLOR, EXTRA_BACKGROUND_COLOR},
    MyEguiApp,
};
pub const SECTION_SPACING: f32 = 25.0;
const VERSION: &str = env!("CARGO_PKG_VERSION");

impl MyEguiApp {
    pub(crate) fn render_settings(&mut self, ui: &mut egui::Ui) {
        ui.heading("设置");

        // 获取当前工作目录
        if let Ok(current_dir) = env::current_dir() {
            // 构建相对路径
            let font_path = current_dir.join("font.ttf");

            // 创建一个默认的字体定义，指定自定义字体
            let font_definitions = FontDefinitions {
                font_data: vec![(
                    std::fs::read(&font_path).unwrap_or_else(|_| Vec::new()),
                    egui::FontFamily::Proportional,
                )],
                family_and_size: Some(TextStyle {
                    font: egui::FontFamily::Proportional,
                    size: 15.0,
                }),
                ..Default::default()
            };

            // 设置字体定义
            ui.ctx().set_fonts(font_definitions);

        let mut scroll_style = ui.style_mut();
        scroll_style.visuals.extreme_bg_color = BACKGROUND_COLOR;
        scroll_style.visuals.widgets.inactive.bg_fill = EXTRA_BACKGROUND_COLOR;
        scroll_style.visuals.widgets.active.bg_fill = EXTRA_BACKGROUND_COLOR;
        scroll_style.visuals.selection.bg_fill = EXTRA_BACKGROUND_COLOR;
        scroll_style.visuals.widgets.hovered.bg_fill = EXTRA_BACKGROUND_COLOR;

        ScrollArea::vertical()
            .stick_to_right(true)
            .auto_shrink([false, true])
            .show(ui, |ui| {
                ui.reset_style();

                self.render_steamgriddb_settings(ui);

                self.render_steam_settings(ui);

                for platform in &mut self.platforms {
                    platform.render_ui(ui);
                    ui.add_space(SECTION_SPACING);
                }
                ui.label(format!("版本: {VERSION}"));
            });
    }

    fn render_steam_settings(&mut self, ui: &mut egui::Ui) {
        ui.heading("Steam");
        ui.horizontal(|ui| {
            let mut empty_string = "".to_string();
            let steam_location = self
                .settings
                .steam
                .location
                .as_mut()
                .unwrap_or(&mut empty_string);
            ui.label("Steam 位置: ");
            if ui.text_edit_singleline(steam_location).changed() {
                if steam_location.trim().is_empty() {
                    self.settings.steam.location = None;
                } else {
                    self.settings.steam.location = Some(steam_location.to_string());
                }
            }
        });
        ui.checkbox(
            &mut self.settings.steam.create_collections,
            "创建收藏",
        )
        .on_hover_text("Tries to create a games collection for each platform");
        ui.checkbox(&mut self.settings.steam.optimize_for_big_picture, "Optimize for big picture").on_hover_text("Set icons to be larger horizontal images, this looks nice in steam big picture mode, but a bit off in desktop mode");
        ui.checkbox(
            &mut self.settings.steam.stop_steam,
            "导入前停止 Steam",
        )
        .on_hover_text("Stops Steam if it is running when import starts");
        ui.checkbox(
            &mut self.settings.steam.start_steam,
            "导入后启动 Steam",
        )
        .on_hover_text("Starts Steam is it is not running after the import");
        ui.add_space(SECTION_SPACING);
    }

    fn render_steamgriddb_settings(&mut self, ui: &mut egui::Ui) {
        ui.heading("SteamGridDB");
        ui.checkbox(&mut self.settings.steamgrid_db.enabled, "Download images");
        if self.settings.steamgrid_db.enabled {
            ui.horizontal(|ui| {
                let mut auth_key = self
                    .settings
                    .steamgrid_db
                    .auth_key
                    .clone()
                    .unwrap_or_default();
                ui.label("Authentication key: ");
                if ui.text_edit_singleline(&mut auth_key).changed() {
                    if auth_key.is_empty() {
                        self.settings.steamgrid_db.auth_key = None;
                    } else {
                        self.settings.steamgrid_db.auth_key = Some(auth_key.to_string());
                    }
                }
                if auth_key.is_empty() && ui.button("Paste from clipboard").clicked() {
                    if let Ok(mut clipboard_ctx) = copypasta::ClipboardContext::new() {
                        if let Ok(content) = clipboard_ctx.get_contents() {
                            self.settings.steamgrid_db.auth_key = Some(content);
                        }
                    }
                }
            });
            ui.horizontal(|ui| {
                ui.label(
                    "To download images you need an API Key from SteamGridDB, you can find yours",
                );
                ui.hyperlink_to(
                    "here",
                    "https://www.steamgriddb.com/profile/preferences/api",
                )
            });
            ui.checkbox(&mut self.settings.steamgrid_db.prefer_animated, "Prefer animated images").on_hover_text("Prefer downloading animated images over static images (this can slow Steam down but looks neat)");
            ui.checkbox(
                &mut self.settings.steamgrid_db.only_download_boilr_images,
                "Only download images for BoilR shortcuts",
            );
            ui.checkbox(&mut self.settings.steamgrid_db.allow_nsfw, "Allow NSFW images");
        }
        ui.add_space(SECTION_SPACING);
    }
}
}

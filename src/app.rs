// Copyright © 2025-2026 Matei Pralea <matei@pralea.me>
// SPDX-License-Identifier: MIT OR Apache-2.0

use eframe::egui;
use serde::{Deserialize, Serialize};

use crate::extra_impl::extra_ctx_impl::ExtraCtxImpl;
use crate::font_setup::setup_fonts;
use crate::language::{LanguageConfiguration, get_browser_language_preference};
use crate::windows::about_window::about_window;
use crate::windows::main_window::main_window;
use crate::windows::more_window::{MoreWindowTab, more_window};
use crate::windows::settings_window::settings_window;
use crate::windows::top_panel::top_panel;

#[derive(Deserialize, Serialize)]
#[serde(default)]
pub struct Application {
    pub window_configuration: WindowConfiguration,
    pub language_configuration: LanguageConfiguration,
    pub language_code: String,
}

#[derive(Default, Deserialize, Serialize)]
#[serde(default)]
pub struct WindowConfiguration {
    pub compact: bool,
    pub show_more_window: bool,
    pub show_about_window: bool,
    pub show_settings_window: bool,
    pub more_window_tab: MoreWindowTab,
}

impl Default for Application {
    fn default() -> Self {
        Self {
            window_configuration: WindowConfiguration::default(),
            language_configuration: LanguageConfiguration::default(),
            language_code: get_browser_language_preference().unwrap_or(String::from("en-US")),
        }
    }
}

impl Application {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        setup_fonts(cc);

        let native_ppp = cc.egui_ctx.native_pixels_per_point().unwrap();
        let new_ppp = native_ppp * 1.5;

        cc.egui_ctx.set_pixels_per_point(new_ppp);
        cc.egui_ctx.request_repaint();

        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for Application {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.toggle_compact(&mut self.window_configuration.compact);

        top_panel(self, ctx);
        main_window(self, ctx);

        if self.window_configuration.show_more_window {
            more_window(self, ctx);
        }

        if self.window_configuration.show_settings_window {
            settings_window(self, ctx);
        }

        if self.window_configuration.show_about_window {
            about_window(self, ctx);
        }
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}

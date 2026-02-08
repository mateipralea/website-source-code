// Copyright © 2025-2026 Matei Pralea <matei@pralea.me>
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::app::Application;
use crate::language::{LanguageConfiguration, LanguageKind};

use eframe::egui;

pub fn settings_window(app: &mut Application, ctx: &egui::Context) {
    let mut open = app.window_configuration.show_settings_window;
    let window = egui::Window::new(app.language_configuration.get_raw().settings())
        .id(egui::Id::new("settings_window"))
        .collapsible(false)
        .resizable(false)
        .open(&mut open)
        .fixed_size([400., 100.])
        .min_size([400., 100.]);
    window.show(ctx, |ui| {
        ui.horizontal(|ui| {
            theme_combo_box(ui, &app.language_configuration.get_raw());
            language_combo_box(ui, app);
        });
    });
    app.window_configuration.show_settings_window = open;
}

fn language_combo_box(ui: &mut egui::Ui, app: &mut Application) {
    ui.vertical(|ui| {
        ui.label(app.language_configuration.get_raw().language());

        let current_language = app.language_configuration.get_raw();

        egui::ComboBox::from_id_salt("language_combo_box")
            .selected_text(match &app.language_configuration {
                LanguageConfiguration::System => current_language.system(),
                LanguageConfiguration::Specified(kind) => kind.language_name(),
            })
            .show_ui(ui, |ui| {
                ui.selectable_value(
                    &mut app.language_configuration,
                    LanguageConfiguration::System,
                    current_language.system(),
                )
                .on_hover_ui(|ui| {
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing.x = 0.0;

                        ui.label(format!(
                            "{}: ",
                            app.language_configuration
                                .get_raw()
                                .detected_system_locale(),
                        ));

                        ui.label(egui::RichText::new(format!("{}", app.language_code)).strong());
                    });
                });

                ui.selectable_value(
                    &mut app.language_configuration,
                    LanguageConfiguration::Specified(LanguageKind::English),
                    LanguageKind::English.language_name(),
                );

                ui.selectable_value(
                    &mut app.language_configuration,
                    LanguageConfiguration::Specified(LanguageKind::German),
                    LanguageKind::German.language_name(),
                );

                ui.selectable_value(
                    &mut app.language_configuration,
                    LanguageConfiguration::Specified(LanguageKind::Romanian),
                    LanguageKind::Romanian.language_name(),
                );
            });
    });
}

fn theme_combo_box(ui: &mut egui::Ui, language: &LanguageKind) {
    let mut ui_theme_preference = ui.ctx().options(|opt| opt.theme_preference);

    ui.vertical(|ui| {
        ui.label(language.theme());
        egui::ComboBox::from_id_salt("theme_combo_box")
            .selected_text(match ui_theme_preference {
                egui::ThemePreference::System => language.system(),
                egui::ThemePreference::Dark => language.dark(),
                egui::ThemePreference::Light => language.light(),
            })
            .show_ui(ui, |ui| {
                ui.selectable_value(
                    &mut ui_theme_preference,
                    egui::ThemePreference::System,
                    language.system(),
                );
                ui.selectable_value(
                    &mut ui_theme_preference,
                    egui::ThemePreference::Dark,
                    language.dark(),
                );
                ui.selectable_value(
                    &mut ui_theme_preference,
                    egui::ThemePreference::Light,
                    language.light(),
                );
            });
    });

    ui.ctx().set_theme(ui_theme_preference);
}

// Copyright © 2025-2026 Matei Pralea <matei@pralea.me>
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::app::Application;
use crate::extra_impl::bool_ext::BoolExt;
use crate::extra_impl::extra_ui_impl::ExtraUiImpl;

use eframe::egui;

pub fn main_window(app: &mut Application, ctx: &egui::Context) {
    if app.window_configuration.compact {
        egui::CentralPanel::default().show(ctx, |ui| main_window_ui(app, ui));
    } else {
        egui::Window::new(app.language_configuration.get_raw().my_website())
            .id(egui::Id::new("main_window"))
            .collapsible(false)
            .resizable(false)
            .fixed_size([300., 105.])
            .min_size([300., 105.])
            .show(ctx, |ui| main_window_ui(app, ui));
    }
}

fn main_window_ui(app: &mut Application, ui: &mut egui::Ui) {
    ui.custom_heading(app.language_configuration.get_raw().main_heading());

    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label(format!(
            "{} ",
            app.language_configuration.get_raw().my_name_is()
        ));
        ui.label(egui::RichText::new("Matei Pralea.").strong())
            .on_hover_ui(|ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    ui.label(app.language_configuration.get_raw().my_pronouns_are());
                    ui.label(
                        egui::RichText::new(format!(
                            " {}.",
                            app.language_configuration.get_raw().pronoun()
                        ))
                        .strong(),
                    );
                });
            });

        ui.add_space(5.);

        ui.label(
            egui::RichText::new(format!(
                "({})",
                app.language_configuration.get_raw().pronoun()
            ))
            .small_raised(),
        );
    });

    ui.add_space(2.);
    ui.horizontal(|ui| {
        ui.hyperlink_to(
            format!("{} GitHub", egui::special_emojis::GITHUB),
            "https://github.com/mateipralea",
        );

        ui.label(egui::RichText::new("•").strong());

        ui.hyperlink_to("Instagram", "https://instagram.com/mateipralea");
    });

    if app.window_configuration.compact {
        ui.add_space(5.);
        let text = if app.window_configuration.show_more_window {
            app.language_configuration.get_raw().show_less_about_me()
        } else {
            app.language_configuration.get_raw().show_more_about_me()
        };

        if ui.add_sized([185., 25.], egui::Button::new(text)).clicked() {
            app.window_configuration.show_more_window.toggle();
        }
    } else {
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
            ui.with_layout(egui::Layout::bottom_up(egui::Align::Max), |ui| {
                let text = if app.window_configuration.show_more_window {
                    app.language_configuration.get_raw().show_less_about_me()
                } else {
                    app.language_configuration.get_raw().show_more_about_me()
                };

                if ui.add_sized([185., 25.], egui::Button::new(text)).clicked() {
                    app.window_configuration.show_more_window.toggle();
                }
            });
        });
    }
}

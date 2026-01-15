// Copyright © 2025-2026 Matei Pralea <matei@pralea.me>
// SPDX-License-Identifier: MIT OR Apache-2.0

use eframe::egui;

pub trait ExtraUiImpl {
    fn custom_heading(&mut self, text: impl ToString);
}

impl ExtraUiImpl for egui::Ui {
    fn custom_heading(&mut self, text: impl ToString) {
        self.add_space(3.0);

        self.heading(
            egui::RichText::new(text.to_string())
                .font(egui::FontId {
                    size: 24.0,
                    family: egui::FontFamily::Name("Bold".into()),
                })
                .strong(),
        );
    }
}

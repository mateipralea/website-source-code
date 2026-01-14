// Copyright © 2025-2026 Matei Pralea <matei@pralea.me>
// SPDX-License-Identifier: MIT OR Apache-2.0

use eframe::egui;
use std::collections::BTreeMap;

pub fn setup_fonts(cc: &eframe::CreationContext<'_>) {
    let mut fonts = egui::FontDefinitions::default();

    let tweak = egui::FontTweak {
        scale: 0.9,
        ..Default::default()
    };

    let mut regular_font =
        egui::FontData::from_static(include_bytes!("../assets/DINishExpanded-Regular.ttf"));

    regular_font.tweak = tweak;

    fonts
        .font_data
        .insert("Regular".to_owned(), regular_font.into());

    let mut bold_font =
        egui::FontData::from_static(include_bytes!("../assets/DINishExpanded-Black.ttf"));

    bold_font.tweak = tweak;

    fonts
        .font_data
        .insert("Bold".to_owned(), bold_font.into());

    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "Regular".to_owned());

    let mut new_family = BTreeMap::new();

    new_family.insert(
        egui::FontFamily::Name("Bold".into()),
        vec!["Bold".to_owned()],
    );

    fonts.families.append(&mut new_family);

    cc.egui_ctx.set_fonts(fonts);

    let mut style = (*cc.egui_ctx.style()).clone();

    style.text_styles.insert(
        egui::TextStyle::Heading,
        egui::FontId::new(16.0, egui::FontFamily::Proportional),
    );

    cc.egui_ctx.set_style(style);
}

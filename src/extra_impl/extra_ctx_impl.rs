// Copyright © 2025-2026 Matei Pralea <matei@pralea.me>
// SPDX-License-Identifier: MIT OR Apache-2.0

use eframe::egui;

pub const COMPACT_THRESHOLD: f32 = 496.;

pub trait ExtraCtxImpl {
    fn toggle_compact(&self, compact: &mut bool);
}

impl ExtraCtxImpl for egui::Context {
    fn toggle_compact(&self, compact: &mut bool) {
        let screen_size = self.input(|i| i.content_rect());
        *compact = screen_size.width() <= COMPACT_THRESHOLD;
    }
}

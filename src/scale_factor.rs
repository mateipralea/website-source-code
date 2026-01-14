// Copyright © 2025-2026 Matei Pralea <matei@pralea.me>
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::extra_impl::extra_ctx_impl::COMPACT_THRESHOLD;
use eframe::egui;

pub fn get_custom_scale_factor(ctx: &egui::Context) -> f32 {
    if ctx.input(|i| i.content_rect()).width() >= COMPACT_THRESHOLD {
        1.5
    } else {
        1.25
    }
}

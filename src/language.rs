// Copyright © 2025-2026 Matei Pralea <matei@pralea.me>
// SPDX-License-Identifier: MIT OR Apache-2.0

use serde::{Deserialize, Serialize};

pub fn get_browser_language_preference() -> Option<String> {
    let window = eframe::web_sys::window()?;
    let navigator = window.navigator();

    let languages = navigator.languages();
    if languages.length() > 0 {
        let first_lang_js = languages.get(0);
        return first_lang_js.as_string();
    }

    navigator.language().and_then(|js_value| Some(js_value))
}

#[derive(Deserialize, Serialize, PartialEq, Clone)]
pub enum LanguageConfiguration {
    System,
    Specified(LanguageKind),
}

impl Default for LanguageConfiguration {
    fn default() -> Self {
        Self::System
    }
}

impl LanguageConfiguration {
    pub fn get_raw(&self) -> LanguageKind {
        match self {
            LanguageConfiguration::System => LanguageKind::from_language_code(
                get_browser_language_preference().unwrap_or(String::from("en-US")),
            ),
            LanguageConfiguration::Specified(kind) => kind.clone(),
        }
    }
}

#[derive(Deserialize, Serialize, PartialEq, Clone)]
pub enum LanguageKind {
    English,
    Romanian,
}

impl LanguageKind {
    pub fn from_language_code(code: impl ToString) -> Self {
        let code = code.to_string();
        if code.starts_with("en") {
            Self::English
        } else if code.starts_with("ro") {
            Self::Romanian
        } else {
            Self::English
        }
    }

    pub fn main_heading(&self) -> &'static str {
        match self {
            LanguageKind::English => "Hello there!",
            LanguageKind::Romanian => "Salutare!",
        }
    }

    pub fn language(&self) -> &'static str {
        match self {
            LanguageKind::English => "Language",
            LanguageKind::Romanian => "Limbă",
        }
    }

    pub fn language_name(&self) -> &'static str {
        match self {
            LanguageKind::English => "English",
            LanguageKind::Romanian => "Română",
        }
    }

    pub fn my_website(&self) -> &'static str {
        match self {
            LanguageKind::English => "My Website",
            LanguageKind::Romanian => "Website-ul meu",
        }
    }

    pub fn my_name_is(&self) -> &'static str {
        match self {
            LanguageKind::English => "My name is",
            LanguageKind::Romanian => "Numele meu este",
        }
    }

    pub fn show_more_about_me(&self) -> &'static str {
        match self {
            LanguageKind::English => "Show More About Me",
            LanguageKind::Romanian => "Arată mai multe despre mine",
        }
    }

    pub fn show_less_about_me(&self) -> &'static str {
        match self {
            LanguageKind::English => "Show Less About Me",
            LanguageKind::Romanian => "Arată mai puține despre mine",
        }
    }

    pub fn more_about_me(&self) -> &'static str {
        match self {
            LanguageKind::English => "More About Me",
            LanguageKind::Romanian => "Mai multe despre mine",
        }
    }

    pub fn website_source_code(&self) -> &'static str {
        match self {
            LanguageKind::English => "Website Source Code",
            LanguageKind::Romanian => "Codul sursă al website-ului",
        }
    }

    pub fn about(&self) -> &'static str {
        match self {
            LanguageKind::English => "About",
            LanguageKind::Romanian => "Despre",
        }
    }

    pub fn theme(&self) -> &'static str {
        match self {
            LanguageKind::English => "Theme",
            LanguageKind::Romanian => "Temă",
        }
    }

    pub fn light(&self) -> &'static str {
        match self {
            LanguageKind::English => "Light",
            LanguageKind::Romanian => "Luminoasă",
        }
    }

    pub fn dark(&self) -> &'static str {
        match self {
            LanguageKind::English => "Dark",
            LanguageKind::Romanian => "Întunecată",
        }
    }

    pub fn system(&self) -> &'static str {
        match self {
            LanguageKind::English => "System",
            LanguageKind::Romanian => "De sistem",
        }
    }

    pub fn more_window_programming_text(&self) -> &'static str {
        match self {
            LanguageKind::English => {
                "I started my programming journey in 2020 with C# and .NET, building WinForms applications as a hobby. Since then, I've expanded my skills to languages like JavaScript, Python, Swift, C, and Rust, developing everything from Discord bots and iOS apps to cross-platform desktop software."
            }
            LanguageKind::Romanian => {
                "Am început să programez în 2020 cu C# și .NET, creând aplicații WinForms inițial ca pasiune. De atunci, mi-am extins competențele către limbaje precum JavaScript, Python, Swift, C și Rust, dezvoltând de la boți pentru Discord și aplicații iOS până la software de desktop cross-platform."
            }
        }
    }
    pub fn more_window_programming_tab(&self) -> &'static str {
        match self {
            LanguageKind::English => "Programming",
            LanguageKind::Romanian => "Programare",
        }
    }

    pub fn more_window_other_interests_text(&self) -> &'static str {
        match self {
            LanguageKind::English => {
                "Beyond the world of programming, I'm fascinated by thought systems and I enjoy exploring philosophical concepts and engaging in debates on religion from a secular perspective. I am also actively learning the German language and immersing myself in its remarkably rich culture."
            }
            LanguageKind::Romanian => {
                "Dincolo de lumea programării, sunt fascinat de sisteme de gândire și îmi place să explorez concepte filosofice și să particip la dezbateri pe teme religioase dintr-o perspectivă laică. De asemenea, învăț activ limba germană și mă cufund în cultura sa remarcabil de bogată."
            }
        }
    }

    pub fn more_window_other_interests_tab(&self) -> &'static str {
        match self {
            LanguageKind::English => "Other Interests",
            LanguageKind::Romanian => "Alte interese",
        }
    }

    pub fn license_information(&self) -> &'static str {
        match self {
            LanguageKind::English => "License Information",
            LanguageKind::Romanian => "Informații de licență",
        }
    }

    pub fn settings(&self) -> &'static str {
        match self {
            LanguageKind::English => "Settings",
            LanguageKind::Romanian => "Setări",
        }
    }
}

#![allow(clippy::unreadable_literal)]

//! Cyberpunk Futurista theme
//!
//! A futuristic neon theme inspired by cyberpunk aesthetics:
//! deep space backgrounds, electric cyan, magenta glow and amber accents.
use iced::color;

use crate::gui::styles::types::palette::Palette;
use crate::gui::styles::types::palette_extension::PaletteExtension;

pub static CYBERPUNK_DARK_PALETTE: std::sync::LazyLock<Palette> =
    std::sync::LazyLock::new(|| Palette {
        primary: color!(0x0a0e17),   // Deep space background
        secondary: color!(0x00f0ff), // Electric cyan (incoming, headers)
        outgoing: color!(0xff2bd6),  // Neon magenta (outgoing)
        starred: color!(0xffb800),   // Amber (favorites)
        text_headers: color!(0x0a0e17), // Deep space (contrast w/ cyan)
        text_body: color!(0xd7e5ff), // Ice white (body text)
    });

pub static CYBERPUNK_DARK_PALETTE_EXTENSION: std::sync::LazyLock<PaletteExtension> =
    std::sync::LazyLock::new(|| CYBERPUNK_DARK_PALETTE.generate_palette_extension());

// Cyberpunk Light variant
pub static CYBERPUNK_LIGHT_PALETTE: std::sync::LazyLock<Palette> =
    std::sync::LazyLock::new(|| Palette {
        primary: color!(0xe8f2ff),   // Ice white background
        secondary: color!(0x0044ff), // Electric blue (incoming, headers)
        outgoing: color!(0xc4008a),  // Deep magenta (outgoing)
        starred: color!(0xff9800),   // Amber (favorites)
        text_headers: color!(0xe8f2ff), // Ice white (contrast w/ blue)
        text_body: color!(0x0a0e17), // Deep space (body text)
    });

pub static CYBERPUNK_LIGHT_PALETTE_EXTENSION: std::sync::LazyLock<PaletteExtension> =
    std::sync::LazyLock::new(|| CYBERPUNK_LIGHT_PALETTE.generate_palette_extension());

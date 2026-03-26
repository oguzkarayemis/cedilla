// SPDX-License-Identifier: GPL-3.0

use crate::config::CedillaConfig;
use cosmic::cosmic_config;

/// Flags given to our COSMIC application to use in it's "init" function.
#[derive(Clone, Debug)]
pub struct Flags {
    pub config_handler: Option<cosmic_config::Config>,
    pub config: CedillaConfig,
    pub system_fonts: Vec<String>,
}

pub fn flags() -> Flags {
    let (config_handler, config) = (CedillaConfig::config_handler(), CedillaConfig::config());
    let system_fonts = load_system_fonts();

    Flags {
        config_handler,
        config,
        system_fonts,
    }
}

fn load_system_fonts() -> Vec<String> {
    let source = font_kit::source::SystemSource::new();
    let mut names = source.all_families().unwrap_or_default();
    names.sort();
    names.dedup();
    names
}

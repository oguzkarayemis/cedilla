// SPDX-License-Identifier: GPL-3.0

use crate::app::{AppModel, Message};
use crate::config::{AppTheme, ConfigInput};
use cosmic::prelude::*;

impl AppModel {
    pub fn handle_config_input(&mut self, input: ConfigInput) -> Task<cosmic::Action<Message>> {
        #[allow(clippy::collapsible_if)]
        match input {
            ConfigInput::UpdateTheme(index) => {
                let app_theme = match index {
                    1 => AppTheme::Dark,
                    2 => AppTheme::Light,
                    _ => AppTheme::System,
                };

                if let Some(handler) = &self.config_handler {
                    if let Err(err) = self.config.set_app_theme(handler, app_theme) {
                        eprintln!("{err}");
                        let mut old_config = self.config.clone();
                        old_config.app_theme = app_theme;
                        self.config = old_config;
                    }

                    return cosmic::command::set_theme(self.config.app_theme.theme());
                }
                Task::none()
            }
            ConfigInput::HelperHeaderBarShowState(show_state) => {
                if let Some(handler) = &self.config_handler {
                    if let Err(err) = self.config.set_show_helper_header_bar(handler, show_state) {
                        eprintln!("{err}");
                        let mut old_config = self.config.clone();
                        old_config.show_helper_header_bar = show_state;
                        self.config = old_config;
                    }
                }
                Task::none()
            }
            ConfigInput::StatusBarShowState(show_state) => {
                if let Some(handler) = &self.config_handler {
                    if let Err(err) = self.config.set_show_status_bar(handler, show_state) {
                        eprintln!("{err}");
                        let mut old_config = self.config.clone();
                        old_config.show_status_bar = show_state;
                        self.config = old_config;
                    }
                }
                Task::none()
            }
            ConfigInput::OpenLastFile(state) => {
                if let Some(handler) = &self.config_handler {
                    if let Err(err) = self.config.set_open_last_file(handler, state) {
                        eprintln!("{err}");
                        let mut old_config = self.config.clone();
                        old_config.open_last_file = state;
                        self.config = old_config;
                    }
                }
                Task::none()
            }
            ConfigInput::ScrollbarSync(state) => {
                if let Some(handler) = &self.config_handler {
                    if let Err(err) = self.config.set_scrollbar_sync(handler, state) {
                        eprintln!("{err}");
                        let mut old_config = self.config.clone();
                        old_config.scrollbar_sync = state;
                        self.config = old_config;
                    }
                }
                Task::none()
            }
            ConfigInput::GotenbergUrlInput(state) => {
                if let Some(handler) = &self.config_handler {
                    if let Err(err) = self.config.set_gotenberg_url(handler, state) {
                        eprintln!("{err}");
                    }
                }
                Task::none()
            }
            ConfigInput::GotenbergUrlSave => {
                self.gotenberg_client = gotenberg_pdf::Client::new(&self.config.gotenberg_url);
                Task::none()
            }
            ConfigInput::UpdateTextSize(new_size) => {
                let size = new_size as i32;
                if let Some(handler) = &self.config_handler {
                    if let Err(err) = self.config.set_text_size(handler, size) {
                        eprintln!("{err}");
                        let mut old_config = self.config.clone();
                        old_config.text_size = size;
                        self.config = old_config;
                    }
                }
                Task::none()
            }
        }
    }
}

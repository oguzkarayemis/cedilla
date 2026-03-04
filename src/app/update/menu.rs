// SPDX-License-Identifier: GPL-3.0

use crate::app::app_menu::MenuAction;
use crate::app::context_page::ContextPage;
use crate::app::core::utils;
use crate::app::{AppModel, Message, PreviewState};
use crate::app::{State, dialogs};
use cosmic::{Application, prelude::*};

impl AppModel {
    pub fn handle_menu_action(&mut self, action: MenuAction) -> Task<cosmic::Action<Message>> {
        let State::Ready { preview_state, .. } = &mut self.state else {
            return Task::none();
        };

        match action {
            MenuAction::About => self.update(Message::ToggleContextPage(ContextPage::About)),
            MenuAction::Settings => self.update(Message::ToggleContextPage(ContextPage::Settings)),
            MenuAction::OpenFile => Task::perform(
                async move {
                    match utils::files::open_markdown_file_picker().await {
                        Some(path) => Some(utils::files::load_file(path.into()).await),
                        None => None,
                    }
                },
                |res| match res {
                    Some(result) => cosmic::action::app(Message::OpenFile(result)),
                    None => cosmic::action::none(),
                },
            ),
            MenuAction::NewFile => self.update(Message::NewFile),
            MenuAction::NewVaultFile => self.update(Message::DialogAction(
                dialogs::DialogAction::OpenNewVaultFileDialog,
            )),
            MenuAction::NewVaultFolder => self.update(Message::DialogAction(
                dialogs::DialogAction::OpenNewVaultFolderDialog,
            )),
            MenuAction::SaveFile => self.update(Message::SaveFile),
            MenuAction::TogglePreview => {
                match preview_state {
                    PreviewState::Hidden => *preview_state = PreviewState::Shown,
                    PreviewState::Shown => *preview_state = PreviewState::Hidden,
                }
                Task::none()
            }
            MenuAction::Undo => self.update(Message::Undo),
            MenuAction::Redo => self.update(Message::Redo),
        }
    }
}

use tui::prelude::*;
use tui::widgets::{Block, BorderType, Borders, Clear, Paragraph};

use crate::tui::app::KaguFormatting;
use crate::tui::popups::popup_traits::PopupTraits;

#[derive(Debug)]
pub struct RemoveChannelPopup {
    pub title: String,
    pub message: String,
    pub current_ui_element: RemoveChannelPopupUiElement,
}

#[derive(Debug)]
pub enum RemoveChannelPopupUiElement {
    Yes,
    No,
}

#[derive(Debug)]
pub enum RemoveChannelPopupResult {
    Yes,
    No,
}

impl PopupTraits for RemoveChannelPopup {
    fn reset(&mut self) {
        self.title = String::new();
        self.message = String::new();
        self.current_ui_element = RemoveChannelPopupUiElement::No;
    }

    fn setup(&mut self, title: Option<String>, message: Option<String>) {
        self.reset();
        self.title = title.unwrap_or(String::from("Remove Channel"));
        self.message = message.unwrap_or(String::from("Remove channel?"));
    }
}

impl Default for RemoveChannelPopup {
    fn default() -> Self {
        RemoveChannelPopup {
            title: String::new(),
            message: String::new(),
            current_ui_element: RemoveChannelPopupUiElement::No,
        }
    }
}

impl RemoveChannelPopup {
    pub fn render<B: Backend>(&self, frame: &mut Frame<'_, B>) {
        // Clear out our space to draw in
        let cleared_area = self.fixed_size_middle_popup(28, 10, frame.size());

        let back_block = Block::default()
            .title(self.title.clone().with_pre_post_spaces())
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);
        let inner_content_area = back_block.inner(cleared_area);

        let [message_area, _filler, yes_area, no_area] = *Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Max(1),
                Constraint::Max(1),
                Constraint::Max(1),
                Constraint::Max(1),
            ])
            .margin(1)
            .split(inner_content_area)
        else {
            return;
        };

        let yes = match self.current_ui_element {
            RemoveChannelPopupUiElement::Yes => vec![Line::from(vec![Span::styled(
                String::from("Yes").with_focus(),
                Style::default(),
            )])],
            RemoveChannelPopupUiElement::No => vec![Line::from(vec![Span::styled(
                String::from("Yes"),
                Style::default(),
            )])],
        };

        let no = match self.current_ui_element {
            RemoveChannelPopupUiElement::Yes => vec![Line::from(vec![Span::styled(
                String::from("No"),
                Style::default(),
            )])],
            RemoveChannelPopupUiElement::No => vec![Line::from(vec![Span::styled(
                String::from("No").with_focus(),
                Style::default(),
            )])],
        };

        let message = Paragraph::new(self.message.clone());
        let yes_paragraph = Paragraph::new(yes);
        let no_paragraph = Paragraph::new(no);

        frame.render_widget(Clear, cleared_area);
        frame.render_widget(back_block, cleared_area);
        frame.render_widget(message, message_area);
        frame.render_widget(yes_paragraph, yes_area);
        frame.render_widget(no_paragraph, no_area);
    }
}

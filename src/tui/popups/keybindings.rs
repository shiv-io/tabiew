use crossterm::event::{KeyCode, KeyModifiers};
use unicode_width::UnicodeWidthStr;

use crate::{
    handler::message::Message,
    tui::{component::Component, icons, pickers::search_picker::SearchPicker},
};

#[derive(Debug)]
pub struct Keybindings {
    picker: SearchPicker<String>,
}

impl Default for Keybindings {
    fn default() -> Self {
        Self {
            picker: SearchPicker::new(entries()).with_title(icons::KEYBOARD.title("Keybindings")),
        }
    }
}

impl Component for Keybindings {
    fn render(
        &mut self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        focus_state: crate::tui::component::FocusState,
    ) {
        self.picker.render(area, buf, focus_state);
    }

    fn handle(&mut self, event: crossterm::event::KeyEvent) -> bool {
        self.picker.handle(event)
            || match (event.code, event.modifiers) {
                (KeyCode::Esc, KeyModifiers::NONE) | (KeyCode::Enter, KeyModifiers::NONE) => {
                    Message::AppDismissOverlay.enqueue();
                    true
                }
                _ => false,
            }
    }
}

fn entries() -> Vec<String> {
    let context_width = KEYBINDINGS
        .iter()
        .map(|(context, _, _)| context.width())
        .max()
        .unwrap_or(0);
    let keys_width = KEYBINDINGS
        .iter()
        .map(|(_, keys, _)| keys.width())
        .max()
        .unwrap_or(0);
    KEYBINDINGS
        .iter()
        .map(|(context, keys, action)| {
            format!(
                " {:context_width$}  {:keys_width$}  {}",
                context, keys, action
            )
        })
        .collect()
}

const KEYBINDINGS: &[(&str, &str, &str)] = &[
    ("Global", ":", "Open the command palette"),
    ("Global", "Q", "Quit Tabiew"),
    (
        "Global",
        "q",
        "Close the current view, or quit on the last tab",
    ),
    ("Global", "t", "Open the tab switcher"),
    (
        "Global",
        "Shift + H / Shift + Left",
        "Go to the previous tab",
    ),
    ("Global", "Shift + L / Shift + Right", "Go to the next tab"),
    ("Table", "j / Down / Ctrl + n", "Move one row down"),
    ("Table", "k / Up / Ctrl + p", "Move one row up"),
    ("Table", "h / Left", "Scroll one column left"),
    ("Table", "l / Right", "Scroll one column right"),
    ("Table", "w", "Go to the next column"),
    ("Table", "b", "Go to the previous column"),
    ("Table", "_", "Go to the first column"),
    ("Table", "$", "Go to the last column"),
    ("Table", "g / Home", "Go to the first row"),
    ("Table", "G / End", "Go to the last row"),
    ("Table", "Ctrl + u", "Move half a page up"),
    ("Table", "Ctrl + d", "Move half a page down"),
    ("Table", "Ctrl + b / PageUp", "Move a full page up"),
    ("Table", "Ctrl + f / PageDown", "Move a full page down"),
    (
        "Table",
        "1 - 9",
        "Go to a row number, starting with this digit",
    ),
    ("Table", "e", "Toggle auto-fit column widths"),
    ("Table", "Enter", "Open the selected row in the sheet"),
    ("Table", "i", "Show table info"),
    ("Table", "/", "Fuzzy search the table"),
    ("Table", "?", "Exact search the table"),
    ("Table", "Shift + R", "Select a random row"),
    ("Sheet", "Shift + J / Shift + Down", "Scroll down"),
    ("Sheet", "Shift + K / Shift + Up", "Scroll up"),
    ("Sheet", "f", "Switch between the plain and JSON format"),
    ("Sheet", "c", "Copy the row to the clipboard"),
    ("Sheet", "q / Esc", "Close the sheet"),
    ("Tab switcher", "j / Down / Ctrl + n", "Select the next tab"),
    (
        "Tab switcher",
        "k / Up / Ctrl + p",
        "Select the previous tab",
    ),
    ("Tab switcher", "g / Home", "Select the first tab"),
    ("Tab switcher", "G / End", "Select the last tab"),
    (
        "Tab switcher",
        "Backspace / Delete",
        "Close the selected tab",
    ),
    ("Tab switcher", "Enter", "Go to the selected tab"),
    ("Tab switcher", "q / Esc", "Close the tab switcher"),
    ("Schema", "j / Down / Ctrl + n", "Select the next table"),
    ("Schema", "k / Up / Ctrl + p", "Select the previous table"),
    ("Schema", "g / Home", "Select the first table"),
    ("Schema", "G / End", "Select the last table"),
    ("Schema", "Backspace", "Unregister the selected table"),
    ("Schema", "Enter", "Open the selected table in a new tab"),
    ("Schema", "q / Esc", "Close the schema view"),
    ("Palette", "Down / Ctrl + n", "Select the next item"),
    ("Palette", "Up / Ctrl + p", "Select the previous item"),
    ("Palette", "Enter", "Run the selected command"),
    ("Palette", "Esc", "Close the command palette"),
    ("Palette", "q + Space", "Open the SQL query editor"),
    ("Palette", "s + Space", "Open the inline select editor"),
    ("Palette", "o + Space", "Open the inline order editor"),
    ("Palette", "f + Space", "Open the inline filter editor"),
];

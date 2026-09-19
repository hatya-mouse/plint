use ratatui::{style::Style, widgets::Widget};

const CHECKMARK: &str = "\u{2713}";

pub(crate) struct Check {
    checked: bool,
}

impl Widget for Check {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        if self.checked {
            buf.set_string(area.left(), area.top(), CHECKMARK, Style::new().green());
        }
    }
}

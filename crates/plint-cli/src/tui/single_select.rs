use ratatui::{
    Frame,
    layout::Rect,
    style::Style,
    widgets::{Row, Table, TableState},
};
use std::fmt::Display;

pub(crate) struct SingleSelect<T: Display> {
    /// The header text of the single select.
    header: String,
    /// The items to select from.
    items: Vec<T>,
}

impl<T: Display> SingleSelect<T> {
    fn render(self, frame: &mut Frame, area: Rect, table_state: &mut TableState) {
        let header = Row::new([self.header]).style(Style::new().bold());

        let rows: Vec<_> = self
            .items
            .iter()
            .map(|item| Row::new([item.to_string()]))
            .collect();
        let widths = [ratatui::layout::Constraint::Fill(1)];

        let table = Table::new(rows, widths)
            .header(header)
            .row_highlight_style(Style::new().reversed().on_yellow());

        frame.render_stateful_widget(table, area, table_state);
    }
}

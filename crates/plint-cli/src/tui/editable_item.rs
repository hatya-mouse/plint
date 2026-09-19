use ratatui::{layout::Rect, text::Line, widgets::Widget};

pub(crate) struct EditableItem<'a, I: Widget> {
    /// The label of the editable item.
    label: Option<Line<'a>>,
    /// The editable widget.
    editable: I,
    /// Whether the editable item is currently selected.
    selected: bool,
}

impl<'a, I: Widget> Widget for EditableItem<'a, I> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let mut col = area.left();
        if let Some(label) = self.label.as_ref() {
            // Draw the label if it exists
            (col, _) = buf.set_line(area.left(), area.top(), label, area.width);
        }

        // Calculate the left position for the editable widget
        let start = col + 1;
        if start >= area.right() {
            return;
        }

        let editable_area = Rect::new(start, area.top(), area.right() - start, area.height);
        self.editable.render(editable_area, buf);
    }
}

impl<'a, I: Widget> EditableItem<'a, I> {
    fn new(editable: I) -> Self {
        Self {
            label: None,
            editable,
            selected: false,
        }
    }

    fn label(mut self, label: impl Into<Line<'a>>) -> Self {
        self.label = Some(label.into());
        self
    }

    fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }
}

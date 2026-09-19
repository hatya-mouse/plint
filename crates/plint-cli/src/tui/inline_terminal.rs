use ratatui::{
    CompletedFrame, Frame, Terminal, TerminalOptions, Viewport,
    backend::{Backend, CrosstermBackend},
};
use std::io::Stdout;

pub struct InlineTerminal(Terminal<CrosstermBackend<Stdout>>);

impl Default for InlineTerminal {
    fn default() -> Self {
        let terminal = ratatui::init_with_options(TerminalOptions {
            viewport: Viewport::Inline(8),
        });
        InlineTerminal(terminal)
    }
}

impl InlineTerminal {
    pub fn draw<F>(
        &mut self,
        render_callback: F,
    ) -> Result<CompletedFrame<'_>, <CrosstermBackend<Stdout> as Backend>::Error>
    where
        F: FnOnce(&mut Frame),
    {
        self.0.draw(render_callback)
    }
}

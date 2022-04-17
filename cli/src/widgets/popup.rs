use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    text::{Span, Spans},
    widgets::{Clear, Paragraph, Widget},
    Frame,
};

#[derive(Clone)]
pub struct PopupState<T> {
    show: bool,
    content: T,
}

/// let popup_state = PopupState::new();
/// popup_state.show(f,block);
/// popup_state.hide();
impl<T> PopupState<T>
where
    T: Widget + Clone,
{
    pub fn new(widget: T) -> Self {
        Self {
            show: false,
            content: widget,
        }
    }

    pub fn render<B: Backend>(&mut self, f: &mut Frame<B>) {
        if self.show {
            let area = render_popup(70, 50, f.size());
            f.render_widget(Clear, area);
            f.render_widget(self.content.clone(), area);
        }
    }

    pub fn set_content(&mut self, content: T) {
        self.content = content;
    }

    pub fn toggle(&mut self) {
        self.show = !self.show;
    }
}

fn render_popup(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}

use std::sync::Arc;

use tokio::sync::Mutex;
use tui::{
    backend::Backend,
    layout::{self, Constraint, Direction, Layout, Rect},
    style::{Color::Yellow, Style},
    widgets::{Block, Borders, Clear, Paragraph, Widget, Wrap},
    Frame,
};

use super::input::Input;

#[derive(Clone)]
pub struct PopupInput {
    show: bool,
    forms: Arc<Mutex<[Input; 3]>>,
    focus: usize,
    chunks: Option<Vec<Rect>>,
}

/// let popup_state = PopupState::new();
/// popup_state.show(f,block);
/// popup_state.hide();
impl PopupInput {
    pub fn new() -> Self {
        let name_input = Input::new();
        let mail_input = Input::new();
        let body_input = Input::new().multiline(true);
        let forms = Arc::new(Mutex::new([name_input, mail_input, body_input]));
        Self {
            show: false,
            forms,
            focus: 0,
            chunks: None,
        }
    }

    pub async fn render<B: Backend>(&mut self, f: &mut Frame<'_, B>) {
        if self.show {
            let area = render_popup(70, 50, f.size());

            let block = Block::default()
                .borders(Borders::ALL)
                .title("書き込み")
                .style(Style::default());

            let wrap = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(100)].as_ref())
                .margin(1)
                .split(area);

            let constraints = vec![
                Constraint::Length(3),       // name
                Constraint::Length(3),       // mail
                Constraint::Percentage(100), // body
            ];

            let layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints(constraints.as_ref())
                .margin(2)
                .split(area);

            f.render_widget(Clear, area);
            f.render_widget(block, wrap[0]);

            self.chunks = Some(layout.clone());
            let titles = ["name", "mail", "body"];

            for (i, form) in self.forms.lock().await.iter_mut().enumerate() {
                let area = layout[i];
                let para = Paragraph::new(form.text())
                    .block(Block::default().borders(Borders::ALL).title(titles[i]))
                    .style(if i == self.focus {
                        Style::default().fg(Yellow)
                    } else {
                        Style::default()
                    })
                    .wrap(Wrap { trim: false });
                f.render_widget(para, area);
            }
        }
    }

    pub fn toggle(&mut self) {
        self.show = !self.show;
    }

    pub async fn next_form(&mut self) {
        self.focus = (self.focus + 1) % self.forms.lock().await.len();
    }

    pub async fn prev_form(&mut self) {
        let len = self.forms.lock().await.len();
        self.focus = (self.focus + len - 1) % len;
    }

    pub fn current_chunk(&self) -> Option<Rect> {
        self.chunks.clone().map(|chunks| chunks[self.focus])
    }

    // input methods
    pub async fn text(&self) -> String {
        self.forms.lock().await[self.focus].text().to_string()
    }

    pub async fn backspace(&mut self) {
        self.forms.lock().await[self.focus].backspace();
    }

    pub async fn char(&mut self, c: &str) {
        self.forms.lock().await[self.focus].char(c);
    }

    pub async fn clear(&mut self) {
        self.forms.lock().await[self.focus].clear();
    }

    pub async fn enter(&mut self) {
        self.forms.lock().await[self.focus].enter();
    }

    pub async fn left(&mut self) {
        self.forms.lock().await[self.focus].cursor.back();
    }

    pub async fn right(&mut self) {
        self.forms.lock().await[self.focus].cursor.next();
    }

    pub async fn up(&mut self) {
        self.forms.lock().await[self.focus].up();
    }

    pub async fn down(&mut self) {
        self.forms.lock().await[self.focus].down();
    }

    pub async fn height(&self) -> usize {
        self.forms.lock().await[self.focus].cursor.y
    }

    pub async fn width(&self) -> usize {
        self.forms.lock().await[self.focus].cursor.x
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

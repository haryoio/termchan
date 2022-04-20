use std::sync::Arc;

use tokio::sync::Mutex;
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{
        Color::{Black, Blue, LightBlue, White, Yellow},
        Modifier, Style,
    },
    widgets::{Block, Borders, Clear, Paragraph, Widget, Wrap},
    Frame,
};

use super::input::Input;

#[derive(Clone, Debug)]
pub struct ReplyForm {
    show: bool,
    forms: Arc<Mutex<[Input; 4]>>,
    focused_idx: usize,
    chunks: Option<Vec<Rect>>,
}

/// let popup_state = PopupState::new();
/// popup_state.show(f,block);
/// popup_state.hide();
impl ReplyForm {
    pub fn new() -> Self {
        let name_input = Input::new();
        let mail_input = Input::new();
        let body_input = Input::new().multiline(true);
        let submit_input = Input::new();
        let forms = Arc::new(Mutex::new([
            name_input,
            mail_input,
            body_input,
            submit_input,
        ]));
        Self {
            show: false,
            forms,
            focused_idx: 0,
            chunks: None,
        }
    }

    pub async fn render<B: Backend>(&mut self, f: &mut Frame<'_, B>) {
        if self.show {
            let area = layout_popup(70, 80, f.size());

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
                Constraint::Length(3), // name
                Constraint::Length(3), // mail
                Constraint::Min(10),   // body
                Constraint::Length(3), // submit
            ];

            let layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints(constraints.as_ref())
                .margin(2)
                .split(area);

            f.render_widget(Clear, area);
            f.render_widget(block, wrap[0]);

            self.chunks = Some(layout.clone());
            let titles = ["name", "mail", "body", "submit"];
            for (i, form) in self.forms.lock().await.iter_mut().enumerate() {
                let area = layout[i];
                if i == 3 {
                    let button = Paragraph::new(titles[i])
                        .block(Block::default().borders(Borders::ALL).style(
                            if i == self.focused_idx {
                                Style::default().fg(LightBlue).add_modifier(Modifier::BOLD)
                            } else {
                                Style::default()
                            },
                        ))
                        .style(if i == self.focused_idx {
                            Style::default().fg(LightBlue).add_modifier(Modifier::BOLD)
                        } else {
                            Style::default()
                        })
                        .alignment(Alignment::Center);
                    f.render_widget(button, area);
                    continue;
                }
                let para = Paragraph::new(form.text())
                    .block(Block::default().borders(Borders::ALL).title(titles[i]))
                    .style(if i == self.focused_idx {
                        Style::default().fg(Yellow)
                    } else {
                        Style::default()
                    })
                    .wrap(Wrap { trim: false });
                f.render_widget(para, area);
            }
        }
    }
    pub async fn mail(&self) -> String {
        self.forms.lock().await[1].text()
    }
    pub async fn message(&mut self) -> String {
        self.forms.lock().await[2].text()
    }
    pub async fn name(&self) -> String {
        self.forms.lock().await[0].text()
    }

    pub fn focused(&self) -> usize {
        self.focused_idx
    }

    pub fn toggle(&mut self) {
        self.show = !self.show;
    }

    pub async fn next_form(&mut self) {
        self.focused_idx = (self.focused_idx + 1) % self.forms.lock().await.len();
    }

    pub async fn prev_form(&mut self) {
        let len = self.forms.lock().await.len();
        self.focused_idx = (self.focused_idx + len - 1) % len;
    }

    pub fn current_chunk(&self) -> Option<Rect> {
        self.chunks.clone().map(|chunks| chunks[self.focused_idx])
    }

    // input methods
    pub async fn text(&self) -> String {
        self.forms.lock().await[self.focused_idx].text().to_string()
    }

    pub async fn backspace(&mut self) {
        self.forms.lock().await[self.focused_idx].backspace();
    }

    pub async fn char(&mut self, c: char) {
        self.forms.lock().await[self.focused_idx].char(c);
    }

    pub async fn clear(&mut self) {
        self.forms.lock().await[self.focused_idx].clear();
    }

    pub async fn enter(&mut self) {
        self.forms.lock().await[self.focused_idx].enter();
    }

    pub async fn left(&mut self) {
        self.forms.lock().await[self.focused_idx].back();
    }

    pub async fn right(&mut self) {
        self.forms.lock().await[self.focused_idx].next();
    }

    pub async fn up(&mut self) {
        self.forms.lock().await[self.focused_idx].up();
    }

    pub async fn down(&mut self) {
        self.forms.lock().await[self.focused_idx].down();
    }

    pub async fn height(&self) -> usize {
        self.forms.lock().await[self.focused_idx].cursor.y
    }

    pub async fn width(&self) -> usize {
        self.forms.lock().await[self.focused_idx].cursor.x
    }
}

fn layout_popup(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
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

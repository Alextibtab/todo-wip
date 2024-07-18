use ratatui::{
    layout::Alignment,
    style::{Color, Style},
    widgets::{Block, BorderType, Paragraph},
    Frame,
};

use crate::app::TodoApp;

pub fn render(_app: &mut TodoApp, frame: &mut Frame) {
    frame.render_widget(
        Paragraph::new(format!("Template"))
            .block(
                Block::bordered()
                    .title("Template")
                    .title_alignment(Alignment::Center)
                    .border_type(BorderType::Rounded),
            )
            .style(Style::default().fg(Color::Cyan).bg(Color::Black))
            .centered(),
        frame.size(),
    )
}

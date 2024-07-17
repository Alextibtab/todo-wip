use std::io;

use ratatui::{buffer::Buffer, crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind}, layout::{Alignment, Rect}, style::Stylize, symbols::border, text::Line, widgets::{block::{Position, Title}, Block, Paragraph, Widget}, Frame};

use crate::tui;

#[derive(Debug, Default)]
pub struct TodoApp {
    // TODO: decide on what state needs to be tracked throughout the TUI app
    // for things such as viewing current todos and editing/creating new ones
    // users should be able to press enter on the main list of todos to see a specific
    // todo/task details
    exit: bool,
}

impl TodoApp {
    // TODO: extract run and rendering and put in relevant main loop
    pub fn run(&mut self, terminal: &mut tui::Tui) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.render_frame(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    // TODO: ^ same as above
    fn render_frame(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.size());
    }

    // TODO: move handle events to respective handler or event code
    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    // TODO: ^ same as above function
    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Char('n') => self.create_task(),
            KeyCode::Char('x') => self.delete_task(),
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn create_task(&mut self) {
        todo!()
    }

    fn delete_task(&mut self) {
        todo!()
    }
}

// TODO: split this code into the ui file use ratatui template as example
impl Widget for &TodoApp {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Title::from(" Todo App ".bold());
        let instructions = Title::from(Line::from(vec![
            " New Task ".into(),
            "<n>".white().bold(),
            " Delete Task ".into(),
            "<x>".white().bold(),
            " Quit ".into(),
            "<q> ".white().bold(),
        ]));
        let block = Block::bordered()
            .title(title.alignment(Alignment::Center))
            .title(
                instructions
                    .alignment(Alignment::Center)
                    .position(Position::Bottom),
            )
            .border_set(border::THICK);

        Paragraph::new("TODO!")
            .centered()
            .block(block)
            .render(area, buf);
    }
}



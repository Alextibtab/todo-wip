/*
    List of planned features for this TUI Todo Application

    Use the ratatui example projects/documentation for inspiration and ideas.
    incorporate widgets such as; calendar, list, block, scrollbar, tabs, table etc...

    demo apps have useful features such as popups either use a popup or separate tab
    for creating/editing initial tasks.

    initial layout ideas: user should first be greeted with their list of ideas perhaps
    on initial boot without any tasks have some kind of welcome dashboard with a clock
    and motivational quote or something.

    want to have some kind of profile/stats page for showing guage of tasks completed
    vs in progress.

    when a user has tasks they should be presented as a list that the user can navigate 
    through

    key-mappings:
        - q: quit/close current focus
        - <Enter>: progress/confirm or toggle task completion state.
        - n: create new todo/task.
        - x: remove todo/task (confirmation prompt?)
        - <left>/h: switch tab left
        - <right>/l: switch tab right

    TODO: create structs for containing
    create app state such as selected tab/screen
    as well as how tasks should be structured

    Example task structure:
        title
        date_created
        date_started
        date_completed
        desc
        type??
        desc

    TODO: come up with name for application
*/
mod tui;

use std::io;

use ratatui::{
    buffer::Buffer, crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind}, layout::{Alignment, Rect}, style::Stylize, symbols::border, text::Line, widgets::{
        block::{Position, Title},
        Block, Paragraph, Widget,
    }, Frame
};

#[derive(Debug, Default)]
pub struct TodoApp {
    // TODO: decide on what state needs to be tracked throughout the TUI app
    // for things such as viewing current todos and editing/creating new ones
    // users should be able to press enter on the main list of todos to see a specific
    // todo/task details
    exit: bool,
}

impl TodoApp {
    pub fn run(&mut self, terminal: &mut tui::Tui) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.render_frame(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn render_frame(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.size());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

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

// TODO: TUI todo application store state in file/json
//       write utility cli program for quick edits/interaction
//       this tui for graphical editing/visualising
fn main() -> io::Result<()> {
    let mut terminal = tui::init()?;
    let todo_result = TodoApp::default().run(&mut terminal);
    tui::restore()?;
    todo_result
}

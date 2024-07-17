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

    TODO: serialize data to either json or a compressed format so that state
    can persist between runs

    TODO: write utility cli program for quick edits/interaction

    TODO: read through: https://github.com/ratatui-org/templates/tree/main/simple
    to understand how ratatui devs structure an example app and best practices etc...

    FIX: read through application patterns: https://ratatui.rs/concepts/application-patterns/
    decide which one fits this kind of application the best and try to adhere to it

    WARN: if stuck look at other todo apps built with ratatui/tui-realm (ratatui framework)
    https://github.com/newfla/todotui
*/
use tui_todo::{app::TodoApp, tui};

use std::io;

fn main() -> io::Result<()> {
    let mut terminal = tui::init()?;
    let todo_result = TodoApp::default().run(&mut terminal);
    tui::restore()?;
    todo_result
}

use std::error::Error;

use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    crossterm::{
        execute,
        terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
    },
};

mod app;
mod grid;
mod patterns;
mod rules;
mod settings;
mod theme;
mod ui;

use crate::grid::Grid;
use crate::{app::App, settings::Settings};

fn main() -> Result<(), Box<dyn Error>> {
    // Setup terminal
    enable_raw_mode()?;

    execute!(&mut std::io::stdout(), EnterAlternateScreen)?;
    // Initialize the
    let backend = CrosstermBackend::new(std::io::stdout());
    let mut terminal = Terminal::new(backend)?;

    let settings = Settings::default();

    // TODO: move this logic internally, and resize automatically, not after creating the grid
    let mut grid = Grid::new(settings.get_rule());
    grid.set_pattern(&settings.get_pattern());
    let (height, width) = ui::grid_dimensions_for_terminal(terminal.size()?.into());
    grid.resize(height, width);

    let mut app = App::new(settings.clone());

    app.run(&mut terminal)?;

    // restore terminal
    disable_raw_mode()?;
    terminal.show_cursor()?;

    execute!(&mut std::io::stdout(), LeaveAlternateScreen)?;

    Ok(())
}

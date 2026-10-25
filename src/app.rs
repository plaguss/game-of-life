use std::io;
use std::time::{Duration, Instant};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use rand::RngExt;
use ratatui::layout::Rect;
use ratatui::{DefaultTerminal, Frame};

use crate::rules::RuleName;
use crate::{
    grid::Grid,
    patterns::{Pattern, PatternName, random_pattern},
    rules::Rule,
    settings::Settings,
    theme::ThemeName,
    ui,
};

/// Helper for the wraparound in the menu
#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
}

/// Determines which overlay is open.
/// The main screen is always visible
#[derive(Debug, Clone, Copy)]
pub enum CurrentMenu {
    // The main overlay, there's no Menu
    None,
    // Help message menu
    Help,
    /// Settings menu state (the settings are managed on its own).
    /// The selected corresponds to the element in the list
    Settings(SettingsPage),
}

/// The subpages for the settings menu
#[derive(Debug, Clone, Copy)]
pub enum SettingsPage {
    Main { selected: usize },
    Fps,
    Theme { selected: usize },
    Pattern { selected: usize },
    Rule { selected: usize },
}

pub struct App {
    grid: Grid,
    settings: Settings,
    current_menu: CurrentMenu,
    exit: bool,
    /// Stored here to simplify resizing
    area: Rect,
    on_startup: bool,
}

impl App {
    pub fn new(settings: Settings) -> Self {
        let mut grid = Grid::new(settings.get_rule());
        grid.set_pattern(&settings.get_pattern());
        Self {
            grid,
            settings,
            current_menu: CurrentMenu::None,
            exit: false,
            area: Rect::default(), // A dummy value until run is used
            on_startup: true,
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        // Get the actual terminal size on startup
        self.area = terminal.size()?.into();
        self.reset_grid();

        // When to do the following update (calling Grid::evolve)
        let mut next_tick = Instant::now() + self.settings.tick_interval();

        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            let poll_timeout = next_tick.saturating_duration_since(Instant::now());
            self.handle_events(poll_timeout)?;

            // Don't need to evolve anything if we just want to exit
            if self.exit {
                break;
            }

            // Only evolve if enough time has elapsed, it can happen that handle_events
            // returns early, in that case we just wait.
            // And also if there's enough space, otherwise the simulation won't make much sense.
            if Instant::now() >= next_tick
                && ui::is_terminal_big_enough(self.area)
                && !self.on_startup
            {
                self.grid.evolve();
                next_tick = Instant::now() + self.settings.tick_interval();
                if self.settings.auto
                    && (self.grid.is_stagnant(Grid::HISTORY_WINDOW)
                        || self.grid.density() < self.settings.min_density)
                {
                    self.add_random_patterns();
                }
            }
        }
        Ok(())
    }

    fn add_random_patterns(&mut self) {
        let mut rng = rand::rng();
        for _ in 0..rng.random_range(1..4) {
            self.grid.add_pattern(&random_pattern());
        }
    }

    /// Updates a single frame, recomputes the UI
    fn draw(&self, frame: &mut Frame) {
        let area = frame.area();

        if !ui::is_terminal_big_enough(area) {
            return ui::terminal_too_small(frame, area, &self.settings);
        }

        ui::render(frame, &self.grid, &self.settings.get_theme());

        if self.on_startup {
            ui::startup_banner(frame, area, &self.settings.get_theme());
            return;
        }

        match &self.current_menu {
            CurrentMenu::None => {}
            CurrentMenu::Help => {
                ui::help_message(frame, area, &self.settings.get_theme());
            }
            CurrentMenu::Settings(page) => match page {
                SettingsPage::Main { selected } => {
                    ui::settings_menu(frame, area, &self.settings, *selected);
                }
                SettingsPage::Fps => {
                    ui::fps_menu(frame, area, &self.settings);
                }
                SettingsPage::Theme { selected } => {
                    ui::theme_menu(frame, area, &self.settings, *selected);
                }
                SettingsPage::Pattern { selected } => {
                    ui::patterns_menu(frame, area, &self.settings, *selected);
                }
                SettingsPage::Rule { selected } => {
                    ui::rule_menu(frame, area, &self.settings, *selected);
                }
            },
        }
    }

    /// Listens and checks for updates on a given timeout (that is given by the FPS).
    /// When a given event occurs, it's passed to the corresponding method
    fn handle_events(&mut self, timeout: Duration) -> io::Result<()> {
        if event::poll(timeout)? {
            match event::read()? {
                Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                    self.handle_key_event(key_event);
                }
                Event::Resize(width, height) => {
                    self.area = Rect::new(0, 0, width, height);
                    self.fit_grid();
                }
                // TODO: Add handler for mouse just in case
                _ => {}
            }
        }

        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        if self.on_startup {
            match key_event.code {
                KeyCode::Enter => self.on_startup = false,
                KeyCode::Char('q') | KeyCode::Esc => self.exit(),
                _ => {}
            }
        }
        match self.current_menu {
            CurrentMenu::None => match key_event.code {
                KeyCode::Char('q') | KeyCode::Esc => self.exit(),
                KeyCode::Char('h') => self.open_help(),
                KeyCode::Char('s') => self.open_settings(),
                _ => {}
            },
            CurrentMenu::Help => match key_event.code {
                KeyCode::Char('q') | KeyCode::Esc => self.current_menu = CurrentMenu::None,
                _ => {}
            },
            CurrentMenu::Settings(page) => self.handle_settings_key(page, key_event),
        }
    }

    fn handle_settings_key(&mut self, page: SettingsPage, key_event: KeyEvent) {
        match page {
            SettingsPage::Main { selected } => self.on_main_page(key_event, selected),
            SettingsPage::Fps => self.on_fps_page(key_event),
            SettingsPage::Theme { selected } => self.on_theme_page(key_event, selected),
            SettingsPage::Pattern { selected } => self.on_patterns_page(key_event, selected),
            SettingsPage::Rule { selected } => self.on_rules_page(key_event, selected),
        }
    }

    fn on_main_page(&mut self, key_event: KeyEvent, selected: usize) {
        match key_event.code {
            KeyCode::Down => {
                self.current_menu = CurrentMenu::Settings(SettingsPage::Main {
                    selected: wraparound_menu(selected, 5, Direction::Down),
                })
            }
            KeyCode::Up => {
                self.current_menu = CurrentMenu::Settings(SettingsPage::Main {
                    selected: wraparound_menu(selected, 5, Direction::Up),
                })
            }
            KeyCode::Enter => match selected {
                0 => self.current_menu = CurrentMenu::Settings(SettingsPage::Fps),
                1 => self.current_menu = CurrentMenu::Settings(SettingsPage::Theme { selected: 0 }),
                2 => {
                    self.current_menu = CurrentMenu::Settings(SettingsPage::Pattern { selected: 0 })
                }
                3 => self.current_menu = CurrentMenu::Settings(SettingsPage::Rule { selected: 0 }),
                4 => self.settings.auto = !self.settings.auto,
                _ => {}
            },
            KeyCode::Char('q') | KeyCode::Esc | KeyCode::Backspace => {
                self.current_menu = CurrentMenu::None
            }
            _ => {}
        }
    }

    fn on_fps_page(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Down => self.settings.fps = self.settings.fps.saturating_sub(1).clamp(1, 60),
            KeyCode::Up => self.settings.fps = self.settings.fps.saturating_add(1).clamp(1, 60),
            KeyCode::Left => self.settings.fps = self.settings.fps.saturating_sub(5).clamp(1, 60),
            KeyCode::Right => self.settings.fps = self.settings.fps.saturating_add(5).clamp(1, 60),
            KeyCode::Char('q') | KeyCode::Esc | KeyCode::Backspace => {
                self.current_menu = CurrentMenu::Settings(SettingsPage::Main { selected: 0 })
            }
            _ => {}
        }
    }

    fn on_theme_page(&mut self, key_event: KeyEvent, selected: usize) {
        match key_event.code {
            KeyCode::Down => {
                self.current_menu = CurrentMenu::Settings(SettingsPage::Theme {
                    selected: wraparound_menu(selected, ThemeName::count(), Direction::Down),
                })
            }
            KeyCode::Up => {
                self.current_menu = CurrentMenu::Settings(SettingsPage::Theme {
                    selected: wraparound_menu(selected, ThemeName::count(), Direction::Up),
                })
            }
            KeyCode::Enter => self.settings.theme = ThemeName::by_index(selected),
            KeyCode::Char('q') | KeyCode::Esc | KeyCode::Backspace => {
                self.current_menu = CurrentMenu::Settings(SettingsPage::Main { selected: 1 })
            }
            _ => {}
        }
    }

    fn on_patterns_page(&mut self, key_event: KeyEvent, selected: usize) {
        match key_event.code {
            KeyCode::Down => {
                self.current_menu = CurrentMenu::Settings(SettingsPage::Pattern {
                    selected: wraparound_menu(selected, PatternName::count(), Direction::Down),
                })
            }
            KeyCode::Up => {
                self.current_menu = CurrentMenu::Settings(SettingsPage::Pattern {
                    selected: wraparound_menu(selected, PatternName::count(), Direction::Up),
                })
            }
            KeyCode::Enter => {
                self.settings.pattern = PatternName::by_index(selected);
                self.reset_grid();
            }
            KeyCode::Char('q') | KeyCode::Esc | KeyCode::Backspace => {
                self.current_menu = CurrentMenu::Settings(SettingsPage::Main { selected: 2 })
            }
            _ => {}
        }
    }

    fn on_rules_page(&mut self, key_event: KeyEvent, selected: usize) {
        match key_event.code {
            KeyCode::Down => {
                self.current_menu = CurrentMenu::Settings(SettingsPage::Rule {
                    selected: wraparound_menu(selected, RuleName::count(), Direction::Down),
                })
            }
            KeyCode::Up => {
                self.current_menu = CurrentMenu::Settings(SettingsPage::Rule {
                    selected: wraparound_menu(selected, RuleName::count(), Direction::Up),
                })
            }
            KeyCode::Enter => {
                self.settings.rule = RuleName::by_index(selected);
                // Don't reset the grid by default, just reuse the new rule
                // on what its in the board
            }
            KeyCode::Char('q') | KeyCode::Esc | KeyCode::Backspace => {
                self.current_menu = CurrentMenu::Settings(SettingsPage::Main { selected: 3 })
            }
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn open_help(&mut self) {
        self.current_menu = CurrentMenu::Help;
    }

    fn open_settings(&mut self) {
        // When the settings menu is opened, it always starts from the first item in the list
        self.current_menu = CurrentMenu::Settings(SettingsPage::Main { selected: 0 })
    }

    /// This method should be called on startup, and whenever a change to the grid is applied
    fn reset_grid(&mut self) {
        let pattern = &self.settings.get_pattern();
        self.grid = Grid::new(Rule::game_of_life());
        // Call fit grid first so that we have the actual size of the grid
        // if we are creating a random soup
        self.fit_grid();
        self.grid.set_pattern(pattern);
        match pattern {
            Pattern::Fixed { .. } => {
                self.grid.center_alive_cells();
            }
            // There's no need to recenter on a RandomSoup
            _ => {}
        }
    }

    /// The only difference with reset_grid is that this method only calls resize and
    /// is only run on Resize events, to avoid wiping the board when the window size
    /// changes.
    ///
    /// Note: The grid won't be fitted if there isn't enough space, otherwise the shrinking
    /// would clamp the grid and lose cells off the visible area.
    fn fit_grid(&mut self) {
        if !ui::is_terminal_big_enough(self.area) {
            return;
        }

        let (height, width) = ui::grid_dimensions_for_terminal(self.area);
        self.grid.resize(height, width);
    }
}

/// Helper function to allow the navigation selector to wraparound when overflowing
fn wraparound_menu(selected_index: usize, menu_count: usize, direction: Direction) -> usize {
    return match direction {
        Direction::Down => {
            if selected_index == menu_count - 1 {
                0
            } else {
                selected_index.saturating_add(1)
            }
        }
        Direction::Up => {
            if selected_index == 0 {
                menu_count - 1
            } else {
                selected_index.saturating_sub(1)
            }
        }
    };
}

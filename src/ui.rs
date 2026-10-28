use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Clear, List, ListState, Paragraph, Wrap},
};
use strum::IntoEnumIterator;

use crate::settings::Settings;
use crate::theme::{Theme, ThemeName};
use crate::{grid::Grid, patterns::Cell};
use crate::{patterns::PatternName, rules::RuleName};

/// Explicit state to discriminate not just dead/alive, but the shadow in case
/// we are plotting it.
enum CellState {
    Alive,
    Shadow,
    Dead,
}

pub fn render(frame: &mut Frame<'_>, grid: &Grid, theme: &Theme, shadow: bool) {
    let areas = layout_areas(frame.area());

    game(
        frame,
        areas.game,
        areas.status,
        areas.board,
        grid,
        theme,
        shadow,
    );
    footer(frame, areas.footer, theme);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UiAreas {
    pub game: Rect,
    pub status: Rect,
    pub board: Rect,
    pub footer: Rect,
}

pub fn startup_banner(frame: &mut Frame<'_>, area: Rect, theme: &Theme) {
    let startup_area = get_popup_area(area, 12, 50);

    let startup_block = get_submenu_block("".to_string(), theme);

    let text = vec![
        Line::from(vec![Span::styled(
            "Welcome to Lifeterm",
            Style::default().add_modifier(Modifier::BOLD),
        )])
        .centered(),
        Line::from(vec![Span::raw("")]),
        Line::from(vec![Span::raw(
            "This simple TUI runs the game of life in your terminal.",
        )]),
        Line::from(vec![
            Span::raw("Once you press <Enter>, the simulation will start, and you"),
            Span::raw(" can interact ONLY with your keyboard,"),
            Span::raw(" using the keys listed in the lower left corner, "),
            Span::styled(
                "this message will dissapear",
                Style::default().add_modifier(Modifier::UNDERLINED),
            ),
            Span::raw("."),
        ]),
        Line::from(vec![Span::raw("")]),
        Line::from(vec![
            Span::raw("- "),
            Span::styled("<Enter>", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(": Starts the game"),
        ]),
        Line::from(vec![
            Span::raw("- "),
            Span::styled("<s>", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(": Show the settings, where you can navigate and modify different options."),
        ]),
        Line::from(vec![
            Span::raw("- "),
            Span::styled("<h>", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(": Shows the help menu."),
        ]),
        Line::from(vec![
            Span::raw("- "),
            Span::styled("<q>", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" or "),
            Span::styled("<Esc>", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(": Stops and closes the app."),
        ]),
    ];

    let startup_message = Paragraph::new(text)
        .wrap(Wrap { trim: true })
        .style(theme.text)
        .block(startup_block);

    frame.render_widget(Clear, startup_area);
    frame.render_widget(startup_message, startup_area);
}

/// Calculate the game, status, board, and footer areas for a terminal size.
fn layout_areas(area: Rect) -> UiAreas {
    let [game_area, footer_area] = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),    // The game area
            Constraint::Length(1), // The footer line
        ])
        .areas(area);

    let game_inner = Block::bordered().inner(game_area);
    let [status_area, board_area] =
        Layout::vertical([Constraint::Length(1), Constraint::Min(0)]).areas(game_inner);

    UiAreas {
        game: game_area,
        status: status_area,
        board: board_area,
        footer: footer_area,
    }
}

/// Choose logical grid dimensions that fit the board viewport, with a 40-cell minimum.
/// The tuple is (height, width), matching `Grid::resize`.
pub fn grid_dimensions_for_terminal(area: Rect) -> Cell {
    let board = layout_areas(area).board;
    // We can unwrap the try_from even if it doesn't fit because
    // the board (the terminal grid) cannot be biger than the i16 size
    (
        (i16::try_from(board.height).unwrap() * 2).max(Grid::MIN_DIMENSION),
        i16::try_from(board.width).unwrap().max(Grid::MIN_DIMENSION),
    )
}

/// Draw the game border and title
fn game(
    frame: &mut Frame<'_>,
    area: Rect,
    status_area: Rect,
    board_area: Rect,
    grid: &Grid,
    theme: &Theme,
    shadow: bool,
) {
    let title_block = Block::bordered()
        .style(theme.background)
        .border_style(theme.accent)
        .title(" Lifeterm ");
    frame.render_widget(title_block, area);

    let counter = format!(
        " Generation: {} | Population: {} ",
        grid.generation(),
        grid.population()
    );
    let counter_block = Paragraph::new(counter).style(theme.text).right_aligned();

    frame.render_widget(counter_block, status_area);

    board(frame, board_area, grid, theme, shadow);
}

fn footer(frame: &mut Frame<'_>, area: Rect, theme: &Theme) {
    let footer_line = Line::from(vec![
        Span::raw("Keys: "),
        Span::styled("[S]", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(" Settings | "),
        Span::styled("[H]", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(" Help | "),
        Span::styled("[Q]", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(" Quit"),
    ]);
    let footer_paragraph = Paragraph::new(footer_line).style(theme.text).left_aligned();

    frame.render_widget(footer_paragraph, area);
}

/// Render one logical column per terminal column and pack two logical rows into
/// each terminal row. Each pair's glyph and colors represent its top and bottom
/// cell states. Missing or out-of-bounds cells are dead.
fn board(frame: &mut Frame<'_>, area: Rect, grid: &Grid, theme: &Theme, shadow: bool) {
    let buffer = frame.buffer_mut();
    let alive_cells = grid.get_alive_cells();
    let previous_cells = grid.get_previous_cells();
    let visible_width = u32::from(area.width).min(grid.width() as u32);
    let visible_height = (u32::from(area.height) * 2).min(grid.height() as u32);
    let cell_state = |row: i16, col: i16| {
        if (row as u32) >= visible_height || (col as u32) >= visible_width {
            CellState::Dead
        } else if alive_cells.contains(&(row, col)) {
            CellState::Alive
        } else if shadow && previous_cells.contains(&(row, col)) {
            CellState::Shadow
        } else {
            CellState::Dead
        }
    };

    for display_row in 0..area.height {
        let top_row = i16::try_from(display_row).unwrap() * 2;
        let bottom_row = top_row + 1;

        for col in 0..area.width {
            // TODO: Can this logic be moved to the grid itself to avoid having it split here?
            let logical_col = i16::try_from(col).unwrap();
            let top_state = cell_state(top_row, logical_col);
            let bottom_state = cell_state(bottom_row, logical_col);
            let (glyph, style) = vertical_pair_appearance(top_state, bottom_state, theme);

            buffer[(area.x + col, area.y + display_row)]
                .set_symbol(glyph)
                .set_style(style);
        }
    }
}

/// Helper function to return at once the glyph and style. It gets
/// messier when we need to add the shadow because the character is
/// directly related to the style.
fn vertical_pair_appearance(
    top: CellState,
    bottom: CellState,
    theme: &Theme,
) -> (&'static str, Style) {
    let alive = theme.alive_cell.fg.unwrap_or(Color::Reset);
    let shadow = theme.shadow_cell.fg.unwrap_or(Color::Reset);
    let dead = theme.dead_cell.bg.unwrap_or(Color::Reset);

    match (top, bottom) {
        (CellState::Alive, CellState::Alive) => ("█", theme.alive_cell),
        (CellState::Shadow, CellState::Shadow) => ("█", theme.shadow_cell),
        (CellState::Dead, CellState::Dead) => (" ", theme.dead_cell),
        (CellState::Alive, CellState::Shadow) => ("▀", theme.alive_cell.bg(shadow)),
        (CellState::Shadow, CellState::Alive) => ("▀", theme.shadow_cell.bg(alive)),
        (CellState::Alive, CellState::Dead) => ("▀", theme.alive_cell.bg(dead)),
        (CellState::Shadow, CellState::Dead) => ("▀", theme.shadow_cell.bg(dead)),
        (CellState::Dead, CellState::Alive) => ("▀", theme.dead_cell.fg(dead).bg(alive)),
        (CellState::Dead, CellState::Shadow) => ("▀", theme.dead_cell.fg(dead).bg(shadow)),
    }
}

/// Explain the simulation and its keyboard controls after startup.
pub fn help_message(frame: &mut Frame<'_>, area: Rect, theme: &Theme) {
    let heading_style = theme.accent.add_modifier(Modifier::BOLD);
    let text = vec![
        Line::from("Watch cells live, die and form patterns across generations."),
        Line::from("Track generation and population above the board."),
        Line::from(""),
        Line::styled("Keyboard", heading_style),
        Line::from("h: Help   s: Settings"),
        Line::from("q / Esc: Quit from the board"),
        Line::from("Esc / q: Close help or go back"),
        Line::from(""),
        Line::styled("Settings", heading_style),
        Line::from("Up/Down: Navigate or adjust FPS"),
        Line::from("Enter: Select or toggle an option"),
        Line::from("FPS: Speed   Theme: Colors"),
        Line::from("Pattern: Restart with a new seed"),
        Line::from("Rule: Birth and survival rules"),
        Line::from("Auto: Add life if sparse or stale"),
        Line::from("Shadow: Show previously live cells"),
        Line::from(""),
        Line::from("Simulation keeps running in menus."),
    ];
    let popup_area = get_popup_area(area, text.len() as u16, 30);

    let help_block = Block::bordered()
        .title(" Help ")
        .style(theme.background)
        .border_style(theme.accent);
    let help_message = Paragraph::new(text)
        .wrap(Wrap { trim: true })
        .style(theme.text)
        .block(help_block);
    frame.render_widget(Clear, popup_area);
    frame.render_widget(help_message, popup_area);
}

pub fn settings_menu(frame: &mut Frame<'_>, area: Rect, settings: &Settings, selected: usize) {
    let theme = &settings.get_theme();
    let items = [
        format!("FPS: {}", settings.fps),
        format!("Theme: {:?}", settings.theme),
        format!("Pattern: {}", settings.pattern),
        format!("Rule: {:?}", settings.rule),
        format!(
            "Auto: {}",
            if settings.auto {
                "● ON ○ OFF"
            } else {
                "○ ON ● OFF"
            }
        ),
        format!(
            "Add shadow: {}",
            if settings.shadow {
                "● ON ○ OFF"
            } else {
                "○ ON ● OFF"
            }
        ),
        format!("Min Density: {:.3}", settings.min_density),
    ];

    let width = items.iter().map(|item| item.chars().count()).max().unwrap() as u16 + 4;
    let popup_area = get_list_popup_area(area, items.len() as u16, width);

    let settings_block = Block::bordered()
        .title(" Settings ")
        .style(theme.background)
        .border_style(theme.accent);

    let mut list_state = ListState::default().with_selected(Some(selected));
    frame.render_widget(Clear, popup_area);
    list(
        frame,
        popup_area,
        theme,
        settings_block,
        items,
        &mut list_state,
    );
}

fn list(
    frame: &mut Frame<'_>,
    area: Rect,
    theme: &Theme,
    settings_block: Block,
    items: impl IntoIterator<Item = String>,
    list_state: &mut ListState,
) {
    let list = List::new(items)
        .block(settings_block)
        .style(theme.text)
        .highlight_style(theme.accent.add_modifier(Modifier::REVERSED))
        .highlight_symbol("> ");

    frame.render_stateful_widget(list, area, list_state);
}

/// FPS submenu
pub fn fps_menu(frame: &mut Frame<'_>, area: Rect, settings: &Settings) {
    let popup_area = get_popup_area(area, 1, 20);

    let theme = settings.get_theme();

    let fps_block = get_submenu_block(" Settings > FPS ".to_string(), &theme);

    let block_area = fps_block.inner(popup_area);

    let fps_line = Paragraph::new(Line::from(vec![
        Span::styled("▲", theme.accent),
        Span::styled(
            format!(" {} ", settings.fps),
            theme.accent.add_modifier(Modifier::BOLD),
        ),
        Span::styled("▼", theme.accent),
    ]))
    .centered();

    frame.render_widget(Clear, popup_area);
    frame.render_widget(fps_block, popup_area);
    frame.render_widget(fps_line, block_area);
}

/// Theme selection submenu
pub fn theme_menu(frame: &mut Frame<'_>, area: Rect, settings: &Settings, selected: usize) {
    let popup_area = get_popup_area(area, ThemeName::count() as u16, 20);

    let theme = settings.get_theme();

    let theme_block = get_submenu_block(" Settings > Theme ".to_string(), &theme);

    frame.render_widget(Clear, popup_area);

    let mut list_state = ListState::default().with_selected(Some(selected));

    list(
        frame,
        popup_area,
        &theme,
        theme_block,
        ThemeName::iter().map(|t| t.to_string()),
        &mut list_state,
    );
}

/// Pattern selection submenu
pub fn patterns_menu(frame: &mut Frame<'_>, area: Rect, settings: &Settings, selected: usize) {
    let width = PatternName::iter()
        .map(|name| name.to_string().chars().count())
        .max()
        .unwrap() as u16
        + 4;
    let popup_area = get_list_popup_area(area, PatternName::count() as u16, width);

    let theme = settings.get_theme();

    let pattern_block = get_submenu_block(" Settings > Pattern ".to_string(), &theme);

    frame.render_widget(Clear, popup_area);

    let mut list_state = ListState::default().with_selected(Some(selected));

    list(
        frame,
        popup_area,
        &theme,
        pattern_block,
        PatternName::iter().map(|t| t.to_string()),
        &mut list_state,
    );
}

/// Rule selection submenu
pub fn rule_menu(frame: &mut Frame<'_>, area: Rect, settings: &Settings, selected: usize) {
    let popup_area = get_popup_area(area, RuleName::count() as u16, 20);

    let theme = settings.get_theme();

    let rule_block = get_submenu_block(" Settings > Rule ".to_string(), &theme);

    frame.render_widget(Clear, popup_area);

    let mut list_state = ListState::default().with_selected(Some(selected));

    list(
        frame,
        popup_area,
        &theme,
        rule_block,
        RuleName::iter().map(|t| t.to_string()),
        &mut list_state,
    );
}

/// Min density submenu
pub fn density_menu(frame: &mut Frame<'_>, area: Rect, settings: &Settings) {
    let popup_area = get_popup_area(area, 1, 20);

    let theme = settings.get_theme();

    let density_block = get_submenu_block(" Settings > Min Density ".to_string(), &theme);

    let block_area = density_block.inner(popup_area);

    let min_density_line = Paragraph::new(Line::from(vec![
        Span::styled("▲", theme.accent),
        Span::styled(
            format!(" {:.3} ", settings.min_density),
            theme.accent.add_modifier(Modifier::BOLD),
        ),
        Span::styled("▼", theme.accent),
    ]))
    .centered();

    frame.render_widget(Clear, popup_area);
    frame.render_widget(density_block, popup_area);
    frame.render_widget(min_density_line, block_area);
}

/// Common popop area for the settings options
fn get_popup_area(area: Rect, item_count: u16, content_space: u16) -> Rect {
    let [_, popup_area, _] = Layout::vertical([
        Constraint::Min(0),
        Constraint::Length(item_count + 2), // As much items + 2 spaces
        Constraint::Min(0),
    ])
    .areas(area);

    let margin_per_side = (100 - content_space).strict_div(2);
    // Leave some space on either side
    let [_, popup_area, _] = Layout::horizontal([
        Constraint::Percentage(margin_per_side),
        Constraint::Percentage(content_space),
        Constraint::Percentage(margin_per_side),
    ])
    .areas(popup_area);
    popup_area
}

fn get_list_popup_area(area: Rect, item_count: u16, width: u16) -> Rect {
    let mut popup_area = get_popup_area(area, item_count, 100);
    popup_area.width = width.min(area.width);
    popup_area.x = area.x + (area.width - popup_area.width) / 2;
    popup_area
}

/// Common block for the submenu blocks
fn get_submenu_block(title: String, theme: &Theme) -> Block<'static> {
    Block::bordered()
        .title(title)
        .style(theme.background)
        .border_style(theme.accent)
}

// Helper functions to avoid having too little space in the terminal to represent the patterns

pub const MIN_TERMINAL_WIDTH: u16 = 42; // Grid::MIN_DIMENSION + 2 border columns
pub const MIN_TERMINAL_HEIGHT: u16 = 24; // Grid::MIN_DIMENSION / 2 + 6 rows of chrome

pub fn is_terminal_big_enough(area: Rect) -> bool {
    area.width >= MIN_TERMINAL_WIDTH && area.height >= MIN_TERMINAL_HEIGHT
}

// TODO: Add a test here for:
// layout_areas(Rect::new(0, 0, MIN_TERMINAL_WIDTH, MIN_TERMINAL_HEIGHT)) gives a board of at
// least Grid::MIN_DIMENSION in both directions will keep the numbers honest.
pub fn terminal_too_small(frame: &mut Frame<'_>, area: Rect, settings: &Settings) {
    let [_, popup_area, _] = Layout::vertical([
        Constraint::Min(0),
        Constraint::Length(4),
        Constraint::Min(0),
    ])
    .areas(area);

    let theme = settings.get_theme();
    let block = Block::bordered()
        .style(theme.background)
        .border_style(theme.accent);

    let message = Paragraph::new(format!(
        "Terminal too small\nNeeds at least {MIN_TERMINAL_WIDTH}x{MIN_TERMINAL_HEIGHT}"
    ))
    .centered()
    .style(theme.text)
    .block(block);

    frame.render_widget(Clear, popup_area);
    frame.render_widget(message, popup_area);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pattern_names_fit_and_scroll_on_minimum_terminal() {
        let width = MIN_TERMINAL_WIDTH;
        let height = MIN_TERMINAL_HEIGHT;
        let backend = ratatui::backend::TestBackend::new(width, height);
        let mut terminal = ratatui::Terminal::new(backend).unwrap();
        let settings = Settings::default();

        for (selected, name) in PatternName::iter().enumerate() {
            let frame = terminal
                .draw(|frame| patterns_menu(frame, frame.area(), &settings, selected))
                .unwrap();
            let rows: Vec<String> = (0..height)
                .map(|row| {
                    (0..width)
                        .map(|col| frame.buffer[(col, row)].symbol())
                        .collect()
                })
                .collect();
            let expected = format!("> {name}");
            assert!(
                rows.iter().any(|row| row.contains(&expected)),
                "{expected} not visible"
            );
        }
    }

    #[test]
    fn settings_displays_full_pattern_name_on_minimum_terminal() {
        let backend = ratatui::backend::TestBackend::new(MIN_TERMINAL_WIDTH, MIN_TERMINAL_HEIGHT);
        let mut terminal = ratatui::Terminal::new(backend).unwrap();
        let settings = Settings {
            pattern: PatternName::MiddleweightSpaceship,
            ..Settings::default()
        };
        let frame = terminal
            .draw(|frame| settings_menu(frame, frame.area(), &settings, 2))
            .unwrap();
        let rows: Vec<String> = (0..MIN_TERMINAL_HEIGHT)
            .map(|row| {
                (0..MIN_TERMINAL_WIDTH)
                    .map(|col| frame.buffer[(col, row)].symbol())
                    .collect()
            })
            .collect();
        assert!(
            rows.iter()
                .any(|row| row.contains("> Pattern: Middleweight spaceship"))
        );
    }

    #[test]
    fn board_area_matches_terminal_layout() {
        let areas = layout_areas(Rect::new(0, 0, 80, 24));

        assert_eq!(areas.board, Rect::new(1, 2, 78, 20));
        assert_eq!(areas.footer, Rect::new(0, 23, 80, 1));
    }

    #[test]
    fn grid_dimensions_keep_minimum_on_small_terminal() {
        assert_eq!(
            grid_dimensions_for_terminal(Rect::new(0, 0, 80, 24)),
            (40, 78)
        );
        assert_eq!(
            grid_dimensions_for_terminal(Rect::new(0, 0, 20, 10)),
            (40, 40)
        );
    }

    #[test]
    fn grid_dimensions_grow_with_terminal_area() {
        assert_eq!(
            grid_dimensions_for_terminal(Rect::new(0, 0, 160, 60)),
            (112, 158)
        );
    }

    #[test]
    fn board_renders_all_live_dead_pairs_with_their_colors() {
        let theme = Theme::matrix();
        let mut grid = Grid::new(crate::rules::Rule::game_of_life());
        grid.set_pattern(&crate::patterns::Pattern::Fixed {
            alive_cells: [(0, 0), (1, 0), (0, 1), (1, 2)].into_iter().collect(),
        });
        let backend = ratatui::backend::TestBackend::new(6, 3);
        let mut terminal = ratatui::Terminal::new(backend).unwrap();
        let frame = terminal
            .draw(|frame| board(frame, Rect::new(1, 1, 4, 1), &grid, &theme, false))
            .unwrap();
        let alive = theme.alive_cell.fg.unwrap();
        let dead = theme.dead_cell.bg.unwrap();
        let expected = [
            ("█", alive, dead),
            ("▀", alive, dead),
            ("▀", dead, alive),
            (" ", Color::Reset, dead),
        ];

        for (col, (glyph, fg, bg)) in (1..=4).zip(expected) {
            let cell = &frame.buffer[(col, 1)];
            assert_eq!(cell.symbol(), glyph, "column {col}");
            assert_eq!(cell.fg, fg, "column {col}");
            assert_eq!(cell.bg, bg, "column {col}");
        }
    }

    #[test]
    fn vertical_pair_appearance_handles_all_state_pairs_in_every_theme() {
        for name in ThemeName::iter() {
            let theme = Theme::from(name);
            let alive = theme.alive_cell.fg.unwrap();
            let shadow = theme.shadow_cell.fg.unwrap();
            let dead = theme.dead_cell.bg.unwrap();
            let cases = [
                (CellState::Alive, CellState::Alive, "█", theme.alive_cell),
                (CellState::Shadow, CellState::Shadow, "█", theme.shadow_cell),
                (CellState::Dead, CellState::Dead, " ", theme.dead_cell),
                (
                    CellState::Alive,
                    CellState::Shadow,
                    "▀",
                    theme.alive_cell.bg(shadow),
                ),
                (
                    CellState::Shadow,
                    CellState::Alive,
                    "▀",
                    theme.shadow_cell.bg(alive),
                ),
                (
                    CellState::Alive,
                    CellState::Dead,
                    "▀",
                    theme.alive_cell.bg(dead),
                ),
                (
                    CellState::Shadow,
                    CellState::Dead,
                    "▀",
                    theme.shadow_cell.bg(dead),
                ),
                (
                    CellState::Dead,
                    CellState::Alive,
                    "▀",
                    theme.dead_cell.fg(dead).bg(alive),
                ),
                (
                    CellState::Dead,
                    CellState::Shadow,
                    "▀",
                    theme.dead_cell.fg(dead).bg(shadow),
                ),
            ];

            for (index, (top, bottom, glyph, style)) in cases.into_iter().enumerate() {
                assert_eq!(
                    vertical_pair_appearance(top, bottom, &theme),
                    (glyph, style),
                    "theme {name}, pair {index}"
                );
            }
        }
    }

    #[test]
    fn board_toggles_shadows_without_overriding_live_cells() {
        let theme = Theme::matrix();
        let mut grid = Grid::new(crate::rules::Rule::game_of_life());
        grid.set_pattern(&crate::patterns::Pattern::Fixed {
            alive_cells: [(1, 0), (1, 1), (1, 2)].into_iter().collect(),
        });
        grid.evolve();
        let backend = ratatui::backend::TestBackend::new(3, 2);
        let mut terminal = ratatui::Terminal::new(backend).unwrap();
        let alive = theme.alive_cell.fg.unwrap();
        let dead = theme.dead_cell.bg.unwrap();

        for shadow in [true, false, true] {
            let frame = terminal
                .draw(|frame| board(frame, Rect::new(0, 0, 3, 2), &grid, &theme, shadow))
                .unwrap();
            for col in [0, 2] {
                let cell = &frame.buffer[(col, 0)];
                assert_eq!(cell.symbol(), if shadow { "▀" } else { " " });
                assert_eq!(cell.fg, if shadow { dead } else { Color::Reset });
                assert_eq!(
                    cell.bg,
                    if shadow {
                        theme.shadow_cell.fg.unwrap()
                    } else {
                        dead
                    }
                );
            }
            let surviving_pair = &frame.buffer[(1, 0)];
            assert_eq!(surviving_pair.symbol(), "█");
            assert_eq!(surviving_pair.fg, alive);
            assert_eq!(surviving_pair.bg, dead);
            let new_cell = &frame.buffer[(1, 1)];
            assert_eq!(new_cell.symbol(), "▀");
            assert_eq!(new_cell.fg, alive);
            assert_eq!(new_cell.bg, dead);
        }
    }

    #[test]
    fn board_does_not_render_shadows_outside_grid_bounds() {
        let theme = Theme::matrix();
        let mut grid = Grid::new(crate::rules::Rule::game_of_life());
        let height = grid.height();
        let width = grid.width();
        grid.set_pattern(&crate::patterns::Pattern::Fixed {
            alive_cells: [(height, 0), (0, width)].into_iter().collect(),
        });
        grid.evolve();
        assert_eq!(grid.get_previous_cells().len(), 2);
        let area = Rect::new(0, 0, width as u16 + 1, height as u16 / 2 + 1);
        let backend = ratatui::backend::TestBackend::new(area.width, area.height);
        let mut terminal = ratatui::Terminal::new(backend).unwrap();
        let frame = terminal
            .draw(|frame| board(frame, area, &grid, &theme, true))
            .unwrap();

        for position in [(0, height as u16 / 2), (width as u16, 0)] {
            let cell = &frame.buffer[position];
            assert_eq!(cell.symbol(), " ");
            assert_eq!(cell.bg, theme.dead_cell.bg.unwrap());
        }
    }

    #[test]
    fn vertical_pair_appearance_resets_unspecified_mixed_pair_colors() {
        let theme = Theme::default();
        let expected = ("▀", Style::default().fg(Color::Reset).bg(Color::Reset));

        assert_eq!(
            vertical_pair_appearance(CellState::Dead, CellState::Alive, &theme),
            expected
        );
        assert_eq!(
            vertical_pair_appearance(CellState::Dead, CellState::Shadow, &theme),
            expected
        );
        assert_eq!(
            vertical_pair_appearance(CellState::Alive, CellState::Shadow, &theme),
            ("▀", Style::default().bg(Color::Reset))
        );
    }
}

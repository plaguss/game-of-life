use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Clear, List, ListState, Paragraph},
};
use strum::IntoEnumIterator;

use crate::grid::Grid;
use crate::settings::Settings;
use crate::theme::{Theme, ThemeName};

pub fn render(frame: &mut Frame<'_>, grid: &Grid, theme: &Theme) {
    let areas = layout_areas(frame.area());

    render_game(frame, areas.game, areas.status, areas.board, grid, theme);
    render_footer(frame, areas.footer, theme);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UiAreas {
    pub game: Rect,
    pub status: Rect,
    pub board: Rect,
    pub footer: Rect,
}

/// Calculate the game, status, board, and footer areas for a terminal size.
pub fn layout_areas(area: Rect) -> UiAreas {
    let [game_area, footer_area] = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0), // The game area
            // Constraint::Length(1), // Blank space
            Constraint::Length(3), // The footer panels
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
pub fn grid_dimensions_for_terminal(area: Rect) -> (i32, i32) {
    let board = layout_areas(area).board;
    (
        (i32::from(board.height) * 2).max(Grid::MIN_DIMENSION),
        i32::from(board.width).max(Grid::MIN_DIMENSION),
    )
}

/// Draw the game border and title
fn render_game(
    frame: &mut Frame<'_>,
    area: Rect,
    status_area: Rect,
    board_area: Rect,
    grid: &Grid,
    theme: &Theme,
) {
    let title_block = Block::bordered()
        .style(theme.background)
        .border_style(theme.accent)
        .title(" Game of Life ");
    frame.render_widget(title_block, area);

    let counter = format!(
        " Generation: {} | Population: {} ",
        grid.generation(),
        grid.population()
    );
    let counter_block = Paragraph::new(counter).style(theme.text).right_aligned();

    frame.render_widget(counter_block, status_area);

    render_board(frame, board_area, grid, theme);
}

fn render_footer(frame: &mut Frame<'_>, area: Rect, theme: &Theme) {
    let [settings_area, help_area, quit_area] =
        Layout::horizontal([Constraint::Ratio(1, 3); 3]).areas(area);

    let section = Block::bordered()
        .style(theme.background)
        .border_style(theme.accent);

    let settings_line = Line::from(vec![
        Span::styled("S", Style::default().add_modifier(Modifier::UNDERLINED)),
        Span::raw("ettings"),
    ]);
    let settings_block = Paragraph::new(settings_line)
        .style(theme.text)
        .centered()
        .block(section.clone());

    let help_line = Line::from(vec![
        Span::styled("H", Style::default().add_modifier(Modifier::UNDERLINED)),
        Span::raw("elp"),
    ]);
    let help_block = Paragraph::new(help_line)
        .style(theme.text)
        .centered()
        .block(section.clone());

    let quit_line = Line::from(vec![
        Span::styled("Q", Style::default().add_modifier(Modifier::UNDERLINED)),
        Span::raw("uit"),
    ]);
    let quit_block = Paragraph::new(quit_line)
        .style(theme.text)
        .centered()
        .block(section);

    frame.render_widget(settings_block, settings_area);
    frame.render_widget(help_block, help_area);
    frame.render_widget(quit_block, quit_area);
}

/// Render one logical column per terminal column and pack two logical rows into
/// each terminal row. Full blocks represent two live cells; upper/lower half
/// blocks represent a live top/bottom cell, respectively. Missing or out-of-
/// bounds cells are dead, and half blocks use the dead-cell background so their
/// unfilled half matches the board.
fn render_board(frame: &mut Frame<'_>, area: Rect, grid: &Grid, theme: &Theme) {
    let buffer = frame.buffer_mut();
    let alive_cells = grid.get_alive_cells();
    let visible_width = u32::from(area.width).min(grid.width() as u32);
    let visible_height = (u32::from(area.height) * 2).min(grid.height() as u32);
    let half_block_style = theme
        .alive_cell
        .bg(theme.dead_cell.bg.unwrap_or(Color::Reset));

    for display_row in 0..area.height {
        let top_row = i32::from(display_row) * 2;
        let bottom_row = top_row + 1;

        for col in 0..area.width {
            // TODO: Can this logic be moved to the grid itself to avoid having it split here?
            let logical_col = i32::from(col);
            let in_grid_width = u32::from(col) < visible_width;
            let top_alive = in_grid_width
                && (top_row as u32) < visible_height
                && alive_cells.contains(&(top_row, logical_col));
            let bottom_alive = in_grid_width
                && (bottom_row as u32) < visible_height
                && alive_cells.contains(&(bottom_row, logical_col));

            let glyph = vertical_pair_glyph(top_alive, bottom_alive);
            let style = match (top_alive, bottom_alive) {
                (true, true) => theme.alive_cell,
                (true, false) | (false, true) => half_block_style,
                (false, false) => theme.dead_cell,
            };

            buffer[(area.x + col, area.y + display_row)]
                .set_symbol(glyph)
                .set_style(style);
        }
    }
}

fn vertical_pair_glyph(top_alive: bool, bottom_alive: bool) -> &'static str {
    match (top_alive, bottom_alive) {
        (true, true) => "█",
        (true, false) => "▀",
        (false, true) => "▄",
        (false, false) => " ",
    }
}

/// TODO: Draws the help message, here we should explain what each piece does,
/// how it works, and useful links
pub fn render_help_message(frame: &mut Frame<'_>, area: Rect, theme: &Theme) {
    // Two text lines plus the top and bottom borders
    let [_, popup_area, _] = Layout::vertical([
        Constraint::Min(0),
        Constraint::Length(6),
        Constraint::Min(0),
    ])
    .areas(area);

    // Leave some space on either side
    let [_, popup_area, _] = Layout::horizontal([
        Constraint::Percentage(20),
        Constraint::Percentage(60),
        Constraint::Percentage(20),
    ])
    .areas(popup_area);

    let help_block = Block::bordered()
        .style(theme.background)
        .border_style(theme.accent);
    let help_message = Paragraph::new("h: Show help\nEsc: Close this menu")
        .style(theme.text)
        .block(help_block);
    frame.render_widget(Clear, popup_area);
    frame.render_widget(help_message, popup_area);
}

pub fn render_settings_menu(
    frame: &mut Frame<'_>,
    area: Rect,
    settings: &Settings,
    selected: usize,
) {
    let [_, popup_area, _] = Layout::vertical([
        Constraint::Min(0),
        Constraint::Length(5), // 3 items + top and bottom borders
        Constraint::Min(0),
    ])
    .areas(area);

    // Leave some space on either side
    let [_, popup_area, _] = Layout::horizontal([
        Constraint::Percentage(40),
        Constraint::Percentage(20),
        Constraint::Percentage(40),
    ])
    .areas(popup_area);

    let theme = &settings.get_theme();
    let settings_block = Block::bordered()
        .title(" Settings ")
        .style(theme.background)
        .border_style(theme.accent);

    let items = [
        format!("FPS: {}", settings.fps),
        format!("Theme: {:?}", settings.theme),
        format!("Pattern: {:?}", settings.pattern),
    ];

    let mut list_state = ListState::default().with_selected(Some(selected));
    frame.render_widget(Clear, popup_area);
    render_list(
        frame,
        popup_area,
        theme,
        settings_block,
        items,
        &mut list_state,
    );
}

fn render_list(
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

/// Settings submenu functions:

pub fn render_fps_menu(frame: &mut Frame<'_>, area: Rect, settings: &Settings) {
    let [_, popup_area, _] = Layout::vertical([
        Constraint::Min(0),
        Constraint::Length(3),
        Constraint::Min(0),
    ])
    .areas(area);

    // Leave some space on either side
    let [_, popup_area, _] = Layout::horizontal([
        Constraint::Percentage(40),
        Constraint::Percentage(20),
        Constraint::Percentage(40),
    ])
    .areas(popup_area);

    let theme = settings.get_theme();

    let fps_block = Block::bordered()
        .title(" Settings > FPS ")
        .style(theme.background)
        .border_style(theme.accent);

    let block_area = fps_block.inner(popup_area);

    let fps = settings.fps;
    let fps_line = Paragraph::new(Line::from(vec![
        Span::styled("▲", theme.accent),
        Span::styled(
            format!(" {fps} "),
            theme.accent.add_modifier(Modifier::BOLD),
        ),
        Span::styled("▼", theme.accent),
    ]))
    .centered();

    frame.render_widget(Clear, popup_area);
    frame.render_widget(fps_block, popup_area);
    frame.render_widget(fps_line, block_area);
}

pub fn render_theme_menu(frame: &mut Frame<'_>, area: Rect, settings: &Settings, selected: usize) {
    let [_, popup_area, _] = Layout::vertical([
        Constraint::Min(0),
        Constraint::Length(ThemeName::count() as u16 + 2), // As much items as themes available + 2 spaces
        Constraint::Min(0),
    ])
    .areas(area);

    // Leave some space on either side
    let [_, popup_area, _] = Layout::horizontal([
        Constraint::Percentage(40),
        Constraint::Percentage(20),
        Constraint::Percentage(40),
    ])
    .areas(popup_area);

    let theme = settings.get_theme();

    let theme_block = Block::bordered()
        .title(" Settings > Theme ")
        .style(theme.background)
        .border_style(theme.accent);

    frame.render_widget(Clear, popup_area);

    let mut list_state = ListState::default().with_selected(Some(selected));

    render_list(
        frame,
        popup_area,
        &theme,
        theme_block,
        ThemeName::iter().map(|t| t.to_string()),
        &mut list_state,
    );
}

// Helper functions to avoid having too little space in the terminal to represent the patterns

pub const MIN_TERMINAL_WIDTH: u16 = 42; // Grid::MIN_DIMENSION + 2 border columns
pub const MIN_TERMINAL_HEIGHT: u16 = 26; // Grid::MIN_DIMENSION / 2 + 6 rows of chrome

pub fn is_terminal_big_enough(area: Rect) -> bool {
    area.width >= MIN_TERMINAL_WIDTH && area.height >= MIN_TERMINAL_HEIGHT
}

// TODO: Add a test here for:
// layout_areas(Rect::new(0, 0, MIN_TERMINAL_WIDTH, MIN_TERMINAL_HEIGHT)) gives a board of at
// least Grid::MIN_DIMENSION in both directions will keep the numbers honest.
pub fn render_terminal_too_small(frame: &mut Frame<'_>, area: Rect, settings: &Settings) {
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
    fn board_area_matches_terminal_layout() {
        let areas = layout_areas(Rect::new(0, 0, 80, 24));

        assert_eq!(areas.board, Rect::new(1, 2, 78, 18));
        assert_eq!(areas.footer, Rect::new(0, 21, 80, 3));
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
            (108, 158)
        );
    }

    #[test]
    fn vertical_pair_glyphs_represent_both_cell_states() {
        assert_eq!(vertical_pair_glyph(true, true), "█");
        assert_eq!(vertical_pair_glyph(true, false), "▀");
        assert_eq!(vertical_pair_glyph(false, true), "▄");
        assert_eq!(vertical_pair_glyph(false, false), " ");
    }
}

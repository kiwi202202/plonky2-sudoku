use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io;
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Cell, Row, Table},
    Frame, Terminal,
};

use crate::model::Sudoku;

pub fn run_tui(sudoku: &mut Sudoku) -> Result<(), io::Error> {
    let mut selected_row: usize = 0;
    let mut selected_col: usize = 0;
    let mut is_valid: Option<bool> = None;

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    loop {
        terminal.draw(|f| draw_ui(f, sudoku, is_valid, selected_row, selected_col))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => break,
                KeyCode::Char('c') => {
                    is_valid = Some(sudoku.is_valid());
                }
                KeyCode::Left => {
                    if selected_col > 0 {
                        selected_col -= 1;
                    }
                }
                KeyCode::Right => {
                    if selected_col < 8 {
                        selected_col += 1;
                    }
                }
                KeyCode::Up => {
                    if selected_row > 0 {
                        selected_row -= 1;
                    }
                }
                KeyCode::Down => {
                    if selected_row < 8 {
                        selected_row += 1;
                    }
                }
                KeyCode::Char(c) if c.is_digit(10) => {
                    let value = c.to_digit(10).unwrap() as u8;
                    sudoku.set_cell(selected_row, selected_col, value);
                }
                _ => {}
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}

fn draw_ui<B: Backend>(
    f: &mut Frame<B>,
    sudoku: &Sudoku,
    is_valid: Option<bool>,
    selected_row: usize,
    selected_col: usize,
) {
    let size = f.size();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([Constraint::Percentage(80), Constraint::Percentage(20)])
        .split(size);

    let selected_style = tui::style::Style::default().add_modifier(tui::style::Modifier::REVERSED);
    let normal_style = tui::style::Style::default();

    let check_chunk = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(10), Constraint::Min(0)].as_ref())
        .split(chunks[0])[1];

    let rows: Vec<Row> = sudoku
        .grid
        .iter()
        .enumerate()
        .map(|(row_idx, row)| {
            let cells: Vec<Cell> = row
                .iter()
                .enumerate()
                .map(|(col_idx, cell)| {
                    let content = if cell.value == 0 {
                        " ".into()
                    } else {
                        cell.value.to_string()
                    };
                    let cell_style = if row_idx == selected_row && col_idx == selected_col {
                        selected_style
                    } else {
                        normal_style
                    };
                    tui::widgets::Cell::from(content).style(cell_style)
                })
                .collect();
            Row::new(cells)
        })
        .collect();

    let table = Table::new(rows)
        .block(Block::default().borders(Borders::ALL).title("Sudoku"))
        .widths(&[Constraint::Length(3); 9]);
    f.render_widget(table, chunks[0]);

    let check_result = match is_valid {
        Some(true) => "Valid Sudoku!",
        Some(false) => "Invalid Sudoku!",
        None => "Press 'c' to check",
    };
    let text = tui::widgets::Paragraph::new(check_result)
        .block(Block::default().borders(Borders::ALL).title("Check Result"));
    f.render_widget(text, chunks[1]);
}

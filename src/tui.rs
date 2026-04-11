use std::io::{Stdout, Write};

use color_eyre::{Result, eyre::Context};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use ratatui::{
    DefaultTerminal, Frame, Terminal, TerminalOptions, Viewport,
    layout::Constraint,
    prelude::CrosstermBackend,
    style::{Color, Style, Stylize},
    widgets::{Block, BorderType, Paragraph, Row, Table},
};

use crate::{app::App, disks::BlockDevice};

pub fn run(terminal: &mut DefaultTerminal, app: &mut App) -> Result<()> {
    terminal.draw(|frame| render(frame, &app.disks))?;

    teardown_terminal(terminal)?;
    Ok(())
}

fn render(frame: &mut Frame, disks: &Vec<BlockDevice>) {
    let header = Row::new(["Name", "Path", "Label"])
        .style(Style::new().bold())
        .bottom_margin(1);

    let mut rows: Vec<Row> = Vec::new();
    for disk in disks {
        rows.push(make_row(disk, ""));
        if let Some(children) = &disk.children {
            let mut iter = children.iter().peekable();

            while let Some(child) = iter.next() {
                let is_last_child = iter.peek().is_none();

                let prefix = if is_last_child {
                    " └─ "
                } else {
                    " ├─ "
                };

                rows.push(make_row(child, prefix));
            }
        }
    }

    let widths = [
        Constraint::Length(20),
        Constraint::Length(25),
        Constraint::Length(10),
    ];
    let table = Table::new(rows, widths)
        .header(header)
        .column_spacing(1)
        .block(Block::bordered().border_type(BorderType::Rounded))
        .style(Color::White)
        .row_highlight_style(Style::new().on_black().bold())
        .column_highlight_style(Color::Gray)
        .cell_highlight_style(Style::new().reversed().yellow())
        .highlight_symbol("🍴 ");

    frame.render_widget(table, frame.area());
}

fn make_row(disk: &BlockDevice, prefix: &str) -> Row<'static> {
    let name = format!("{}{}", prefix, disk.name.as_deref().unwrap_or_default());
    let path = disk
        .path
        .as_ref()
        .and_then(|p| p.to_str())
        .unwrap_or_default();
    let label = disk.label.as_deref().unwrap_or_default();

    Row::new([name.to_string(), path.to_string(), label.to_string()])
}

pub fn setup_terminal(app: &mut App) -> Result<Terminal<CrosstermBackend<Stdout>>> {
    let stdout = std::io::stdout();
    enable_raw_mode()?;
    let total_devices: u16 = app
        .disks
        .iter()
        .map(|disk| 1 + disk.children.as_ref().map_or(0, |child| child.len()))
        .sum::<usize>() as u16;

    let needed_lines: u16 = total_devices + 4; // 4 = borders(2) + header(2)

    let backend = CrosstermBackend::new(stdout);
    Terminal::with_options(
        backend,
        TerminalOptions {
            viewport: Viewport::Inline(needed_lines),
        },
    )
    .context("Failed to setup terminal")
}

fn teardown_terminal(_terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
    let mut stdout = std::io::stdout();
    disable_raw_mode()?;

    // Clean line so percentage sign dont show.
    println!();
    stdout.flush()?;
    Ok(())
}

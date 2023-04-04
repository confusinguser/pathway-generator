use std::io::stdout;

use crossterm::event::{Event, KeyCode, KeyEventKind, KeyModifiers, MouseEventKind};
use crossterm::style::Color;
use terminal_pixel_renderer::TerminalDisplay;

use crate::pathing::{CellType, Configuration};

mod pathing;

struct PathwayUI {
    terminal_display: TerminalDisplay,
}

async fn wait_until_mouse_drag() -> crossterm::event::MouseEvent {
    loop {
        let event = crossterm::event::read();
        match event {
            Ok(event) => {
                if let crossterm::event::Event::Mouse(mouse_event) = event {
                    if mouse_event.kind == crossterm::event::MouseEventKind::Moved {
                        return mouse_event;
                    }
                }
            }
            Err(e) => println!("Error: {:?}\n", e),
        }
    }
}

fn main() -> crossterm::Result<()> {
    let mut config = Configuration::new(crossterm::terminal::size()?);
    let mut pathway_ui = PathwayUI {
        terminal_display: TerminalDisplay::default(),
    };

    crossterm::terminal::enable_raw_mode()?;

    let mut stdout = stdout();
    crossterm::execute!(stdout, crossterm::event::EnableMouseCapture)?;

    pathway_ui
        .terminal_display
        .update_display(TerminalDisplay::render_full_block_color(
            &map_to_colors(&config.map),
            config.size.0,
        ));

    loop {
        let event = crossterm::event::read()?;
        let action = handle_event(event, config.size);
        match action {
            Action::AddPathNode(col, row) => {
                config.add_node_with_paths((col as f32, row as f32));
                pathway_ui.terminal_display.update_display(
                    TerminalDisplay::render_full_block_color(
                        &map_to_colors(&config.map),
                        config.size.0,
                    ),
                );
            }
            Action::Start => {
                config.clean_map();
                crossterm::terminal::disable_raw_mode()?;
                config.add_path_cells();
                crossterm::terminal::enable_raw_mode()?;
                pathway_ui.terminal_display.update_display(
                    TerminalDisplay::render_full_block_color(
                        &map_to_colors(&config.map),
                        config.size.0,
                    ),
                );
            }
            Action::Exit => std::process::exit(1),
            Action::Nothing => {}
        }
    }
}

fn map_to_colors(map: &[CellType]) -> Vec<Option<Color>> {
    map.iter()
        .map(|cell| match cell {
            CellType::None => Option::None,
            CellType::Path => Some(Color::White),
            CellType::Node => Some(Color::Blue),

            CellType::Color(r, g, b) => Some(Color::from((*r, *g, *b))),
        })
        .collect()
}

fn create_configuration(nodes: &Vec<(f32, f32)>, size: (u16, u16)) -> Configuration {
    let mut config = Configuration::new(size);
    for &node in nodes {
        config.add_node_with_paths(node);
    }
    config
}

enum Action {
    AddPathNode(u16, u16),
    Start,
    Exit,
    Nothing,
}

fn handle_event(event: Event, bounds: (u16, u16)) -> Action {
    match event {
        Event::Mouse(mouse_event) => match mouse_event.kind {
            MouseEventKind::Down(_btn) => {
                let terminal_size = crossterm::terminal::size().unwrap();
                if mouse_event.column < bounds.0
                    && (mouse_event.row + bounds.1 + 1)
                        .checked_sub(terminal_size.1)
                        .map_or(false, |v| v < bounds.1)
                {
                    Action::AddPathNode(
                        mouse_event.column,
                        mouse_event.row + bounds.1 - terminal_size.1,
                    )
                } else {
                    Action::Nothing
                }
            }
            _ => Action::Nothing,
        },
        Event::Key(key_event) => {
            if KeyEventKind::Press == key_event.kind {
                if key_event.code == KeyCode::Char('s') {
                    return Action::Start;
                }
                if key_event.code == KeyCode::Char('c')
                    && key_event.modifiers.contains(KeyModifiers::CONTROL)
                {
                    return Action::Exit;
                }
            }
            Action::Nothing
        }
        _ => Action::Nothing,
    }
}

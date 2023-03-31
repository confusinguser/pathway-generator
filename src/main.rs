mod pathing;

use std::{io::stdout};
use crossterm::event::{Event, KeyCode, KeyEventKind, MouseEventKind};
use crossterm::style::Color;

use terminal_pixel_renderer::TerminalDisplay;
use crate::pathing::{CellType, Configuration};

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
    let size = (crossterm::terminal::size()?.0, 60u16);
    let mut nodes = Vec::new();
    let mut pixels: Vec<Option<Color>> = vec![None; (size.0 * size.1) as usize];
    let mut pathway_ui = PathwayUI {
        terminal_display: TerminalDisplay::default(),
    };

    crossterm::terminal::enable_raw_mode()?;

    let mut stdout = stdout();
    crossterm::execute!(stdout, crossterm::event::EnableMouseCapture)?;

    pathway_ui.terminal_display.update_display(TerminalDisplay::render_full_block_color(&pixels, size.0));

    loop {
        let event = crossterm::event::read()?;
        let action = handle_event(event, size);
        match action {
            Action::AddPathNode(col, row) => {
                pixels[(row * size.1 + col) as usize] = Some(Color::Blue);
                nodes.push((col as f32, row as f32));
                pathway_ui.terminal_display.update_display(
                    TerminalDisplay::render_full_block_color(&pixels, size.0));
            }
            Action::Start => {
                let mut config = create_configuration(&nodes, size);
                config.add_path_cells();
                for (i, cell) in config.map.iter().enumerate() {
                    pixels[i] = match cell {
                        CellType::None => {Option::None}
                        CellType::Path => {Some(Color::White)}
                        CellType::Node => {Some(Color::Blue)}
                    }
                }
                pathway_ui.terminal_display.update_display(
                    TerminalDisplay::render_full_block_color(&pixels, size.0));
            }
            Action::Nothing => {}
        }
    }
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
    Nothing,
}

fn handle_event(event: Event, bounds: (u16, u16)) -> Action {
    match event {
        Event::Mouse(mouse_event) => {
            match mouse_event.kind {
                MouseEventKind::Down(_btn) => {
                    if mouse_event.column < bounds.0 && mouse_event.row < bounds.1 {
                        Action::AddPathNode(mouse_event.column, mouse_event.row)
                    } else {
                        Action::Nothing
                    }
                }
                _ => Action::Nothing
            }
        }
        Event::Key(key_event) => {
            if KeyEventKind::Press == key_event.kind && key_event.code == KeyCode::Char('s') {
                return Action::Start;
            }
            Action::Nothing
        }
        _ => Action::Nothing
    }
}
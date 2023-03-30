mod pathing;

use std::{io::stdout};
use crossterm::event::{Event, KeyCode, KeyEventKind, MouseEventKind};
use crossterm::style::Color;

use terminal_pixel_renderer::TerminalDisplay;
use crate::Action::{AddPathNode, Nothing};

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
    let bounds = (crossterm::terminal::size()?.0, 60u16);
    let mut pixels: Vec<Vec<Option<Color>>> = vec![vec![None; bounds.0 as usize]; bounds.1 as usize];
    let mut pathway_ui = PathwayUI {
        terminal_display: TerminalDisplay::default(),
    };

    crossterm::terminal::enable_raw_mode()?;

    let mut stdout = stdout();
    crossterm::execute!(stdout, crossterm::event::EnableMouseCapture)?;

    pathway_ui.terminal_display.update_display(TerminalDisplay::render_full_block_color(&pixels));

    loop {
        let event = crossterm::event::read()?;
        let action = handle_event(event, bounds);
        match action {
            AddPathNode(col, row) => {
                pixels[row as usize][col as usize] = Some(Color::Blue);
                pathway_ui.terminal_display.update_display(TerminalDisplay::render_full_block_color(&pixels));
            }
            Start =>
            _ => {}
        }
    }
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
                        AddPathNode(mouse_event.column, mouse_event.row)
                    } else {
                        Nothing
                    }
                }
                _ => Nothing
            }
        }
        Event::Key(key_event) => {
            if KeyEventKind::Press == key_event.kind && key_event.code == KeyCode::Char('s') {
                return Action::Start
            }
            Nothing
        }
        _ => Nothing
    }
}
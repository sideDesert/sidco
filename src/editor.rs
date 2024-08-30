
use crossterm::event::{KeyCode::{self, Up, Down, Left, Right, Char}, KeyModifiers,read, Event::Key};
use crate::terminal::{Coords, Terminal};

pub mod view;
pub mod buffer;

use crate::editor::view::View;

pub struct Editor {
    view: View,
    terminal: Terminal
}

impl Editor {
    
    pub fn default() -> Self {
        Self {
            view: View::default(),
            terminal: Terminal::initialize().unwrap(),
        }
    }
    pub fn run(&mut self) {
        self.view.init();
        self.view.render().unwrap();
        self.key_press_listener();
        self.terminal.move_cursor(Coords{x: 0, y: 0});        
        Terminal::terminate().unwrap();
    }
    fn key_press_listener(&mut self){
        loop {
            match read() {
                Ok(Key(event)) => {
                    Terminal::execute().unwrap();
                    if event.code == KeyCode::Char('c') && event.modifiers == KeyModifiers::CONTROL {
                        break;
                    }
                    match event.code {
                        Left => self.terminal.move_cursor_left(),
                        Right => self.terminal.move_cursor_right(),
                        Up => self.terminal.move_cursor_up(),
                        Down => self.terminal.move_cursor_down(),
                        Char(c) => {
                            Terminal::print(&c.to_string()).unwrap();
                            self.terminal.move_cursor_right();
                        },
                        _ => ()
                    }
                }
                Err(err) => println!("Error: {err}"),
                _ => (),
            }
        }
    }
}

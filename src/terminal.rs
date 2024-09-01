use crossterm::cursor::{Hide, MoveTo, MoveToNextLine, Show};
use crossterm::terminal::{enable_raw_mode, disable_raw_mode, Clear, size};
use crossterm::{queue, Command};
use crossterm::style::Print;
use std::io::{stdout, Write, Error};


pub struct Coords {
    pub x: u16,
    pub y: u16,
}
pub struct Terminal {
    cursor_position: Coords,
    window_size: Size,
}
pub struct Size {
    pub height: u16,
    pub width: u16,
}

// Methods for Cursor
#[allow(dead_code)]
impl Terminal {
    pub fn update_cursor_position(&self){
        Self::move_to(&self.cursor_position).unwrap();
    }
    pub fn get_cursor_position(&self) -> &Coords{
        &self.cursor_position
    }
    pub fn move_cursor(&mut self, coords: Coords) {
        self.cursor_position = coords;
        self.update_cursor_position();
    }
    pub fn move_cursor_left(&mut self){
        if self.cursor_position.x == 0 {
            return
        }
        self.cursor_position = Coords {
            x: self.cursor_position.x -1,
            ..self.cursor_position
        };
        self.update_cursor_position();
    }
    pub fn move_cursor_right(&mut self){
        if self.cursor_position.y == self.window_size.width - 1 {
            println!("Hey, you reached the end of this");
            return;
        }
        self.cursor_position = Coords {
            x: self.cursor_position.x +1,
            ..self.cursor_position
        };
        self.update_cursor_position();
    }
    pub fn move_cursor_up(&mut self){
        if self.cursor_position.y == 0 {
            return
        }
        self.cursor_position = Coords {
            y: self.cursor_position.y-1,
            ..self.cursor_position
        };
        self.update_cursor_position();
    }
    pub fn move_cursor_down(&mut self){
if self.cursor_position.y == self.window_size.height - 1 {
            return;
        }
        self.cursor_position = Coords {
            y: self.cursor_position.y+1,
            ..self.cursor_position
        };
        self.update_cursor_position();
    }

}
impl Terminal {
    pub fn initialize() -> Result<Terminal, Error> {
        enable_raw_mode().unwrap();
        Self::hide_cursor().unwrap();
        Self::clear_screen().unwrap();
        Self::move_to(&Coords { x: 0, y: 0 }).unwrap();
        Self::show_cursor().unwrap();
        Self::execute().unwrap();

        let size = Self::get_size().unwrap();
        Ok(Self{
            cursor_position: Coords{ x: 0, y: 0}, 
            window_size: size
        })
    }

    pub fn terminate()->Result<(), Error>{
        disable_raw_mode().unwrap();
        Self::move_to(&Coords { x: 0, y: 0 }).unwrap();
        Self::clear_screen().unwrap();
        Self::execute().unwrap();
        Ok(())
    }

    pub fn move_to(Coords { x, y }: &Coords)-> Result<(), Error>{
        Self::queue_command(MoveTo(*x,*y)).unwrap();
        Ok(())
    }
    pub fn move_to_next_line() -> Result<(), Error>{
        Self::queue_command(MoveToNextLine(1)).unwrap();
        Ok(())
    }

    pub fn clear_screen() -> Result<(), Error> {
        Self::queue_command( Clear(crossterm::terminal::ClearType::All)).unwrap();
        Ok(())
    }
    pub fn get_size() -> Result<Size, Error>{
        let size = size()?;
        let size = Size{height: size.1, width: size.0};
        Ok(size)
    }
    pub fn hide_cursor() -> Result<(), Error> {
        Self::queue_command( Show)?;
        Ok(())
    }
    pub fn show_cursor() -> Result<(), Error> {
        Self::queue_command( Hide)?;
        Ok(())
    }
    pub fn execute() -> Result<(), Error> {
        stdout().flush()?;
        Ok(())
    }
    pub fn clear_line() -> Result<(), Error> {
        Self::queue_command(Clear(crossterm::terminal::ClearType::CurrentLine))?;
        Ok(())
    }
    pub fn print(string: &str) -> Result<(), Error> {
        Self::queue_command(Print(string))?;
        Self::execute()?;
        
        Ok(())
    }
    pub fn queue_command<T: Command> (command:T) -> Result<(), Error> {
        queue!(stdout(), command)?;
        Ok(())
    }
}

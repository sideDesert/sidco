
use crate::terminal::{Terminal, Coords, Size};
use crate::editor::buffer::Buffer;

#[derive(Default)]
pub struct View {
    pub buffer: Buffer
}

impl View {
    pub fn init(&mut self) {
        self.buffer.load_file();
    }

    pub fn render(&self) -> Result<(), std::io::Error>{
        self.draw_rows()?;
        if self.buffer.is_empty() {
            self.show_welcome()?;
            Terminal::execute()?;
            return Ok(())
        } 
        let size: Size = Terminal::get_size()?;
        let height = size.height;
        for row_num in 0..height {
            Terminal::clear_line()?;
            if let Some(row_content) = self.buffer.content.get(row_num as usize) {
                self.print_row_content(row_content);
            }
            Terminal::move_to_next_line()?;
        }    
        Terminal::execute()?;
        Ok(())
    }

    fn print_row_content(&self, row_content: &str){
        let size = Terminal::get_size().unwrap();
        let width = size.width as usize;
        let mut lptr = 0;
        let mut rptr = width;
        
        if row_content.len() < width {
            Terminal::clear_line().unwrap();
            Terminal::print(row_content).unwrap();
            return;
        }
        while rptr < width {
            Terminal::clear_line().unwrap();
            Terminal::print(&row_content[lptr..rptr]).unwrap();
            Terminal::move_to_next_line().unwrap();
            lptr = rptr;
            rptr += width;
        }
        if lptr < width {
            Terminal::clear_line().unwrap();
            Terminal::print(&row_content[lptr..]).unwrap();
        }
    }
    fn draw_rows(&self) -> Result<(), std::io::Error>{
        Terminal::move_to(&Coords{x: 0, y: 0})?;
        let Size{height,..} = Terminal::get_size()?;

        for i in 0..= height {
            Terminal::move_to(&Coords{x: 0, y: i})?;
            Terminal::print("~")?;
        }

        Terminal::move_to(&Coords{x: 0, y: 0})?;
        Terminal::execute().unwrap();

        Ok(())
    }
    fn show_welcome(&self) -> Result<(), std::io::Error>{
        let text_editor = "Sidco".to_string();
        let version = "v0.1-alpha".to_string();
        let welcome_msg_len = text_editor.len() as u16;

        let Size{width, height} = Terminal::get_size().unwrap();
        let msg_coords = &Coords{x: (width/2)-welcome_msg_len/2, y: height-3};
        let version_coords = &Coords{x: (width/2)-(version.len() as u16)/2, y: height-2};
        Terminal::move_to(msg_coords).unwrap();
        Terminal::print(&text_editor).unwrap();
        Terminal::move_to(version_coords).unwrap();
        Terminal::print(&version).unwrap();

        Terminal::move_to(&Coords{x: 0,y: 0}).unwrap();
        Terminal::execute().unwrap();
        Ok(())
    }
}

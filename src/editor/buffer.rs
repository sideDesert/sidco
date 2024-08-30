use std::env::args;

#[derive(Default)]
pub struct Buffer {
    pub content: Vec<String>
}

impl Buffer {
    pub fn is_empty(&self) -> bool{
        if self.content.len() == 0 {
            return true;
        }
        false
    }
    pub fn load_file(&mut self) {
        let args : Vec<String> = args().collect();
        if let Some(file_path) = args.get(1) {
            let file_contents = std::fs::read_to_string(file_path).expect("No such file");
            for line in file_contents.lines() {
                self.content.push(line.to_string());
            }
        }
    }
}

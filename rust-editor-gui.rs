use std::fs;
use std::io::{self, Write};
use std::path::Path;

struct Editor {
    lines: Vec<String>,
    filename: Option<String>,
    cursor_row: usize,
    cursor_col: usize,
}

impl Editor {
    fn new() -> Self {
        Editor {
            lines: vec![String::new()],
            filename: None,
            cursor_row: 0,
            cursor_col: 0,
        }
    }

    fn load_file(&mut self, filename: &str) -> io::Result<()> {
        let content = fs::read_to_string(filename)?;
        self.lines = content.lines().map(String::from).collect();
        if self.lines.is_empty() {
            self.lines.push(String::new());
        }
        self.filename = Some(filename.to_string());
        Ok(())
    }

    fn save_file(&self) -> io::Result<()> {
        if let Some(filename) = &self.filename {
            let content = self.lines.join("\n");
            fs::write(filename, content)?;
            println!("Saved to {}", filename);
        } else {
            println!("No filename specified. Use 'save <filename>' to save.");
        }
        Ok(())
    }

    fn save_as(&mut self, filename: &str) -> io::Result<()> {
        self.filename = Some(filename.to_string());
        self.save_file()
    }

    fn insert_char(&mut self, c: char) {
        if self.cursor_row >= self.lines.len() {
            self.lines.push(String::new());
        }
        self.lines[self.cursor_row].insert(self.cursor_col, c);
        self.cursor_col += 1;
    }

    fn insert_newline(&mut self) {
        let current_line = &self.lines[self.cursor_row];
        let new_line = current_line[self.cursor_col..].to_string();
        self.lines[self.cursor_row].truncate(self.cursor_col);
        self.cursor_row += 1;
        self.lines.insert(self.cursor_row, new_line);
        self.cursor_col = 0;
    }

    fn delete_char(&mut self) {
        if self.cursor_col > 0 {
            self.lines[self.cursor_row].remove(self.cursor_col - 1);
            self.cursor_col -= 1;
        } else if self.cursor_row > 0 {
            let current_line = self.lines.remove(self.cursor_row);
            self.cursor_row -= 1;
            self.cursor_col = self.lines[self.cursor_row].len();
            self.lines[self.cursor_row].push_str(&current_line);
        }
    }

    fn display(&self) {
        print!("\x1B[2J\x1B[H"); // Clear screen and move cursor to top

        println!("=== Simple Text Editor ===");
        if let Some(filename) = &self.filename {
            println!("File: {}", filename);
        } else {
            println!("File: [New File]");
        }
        println!("Commands: :q (quit), :w (save), :wq (save & quit)");
        println!("---");

        for (i, line) in self.lines.iter().enumerate() {
            if i == self.cursor_row {
                print!("{:3} > {}", i + 1, line);
                if self.cursor_col == line.len() {
                    print!("█");
                }
                println!();
            } else {
                println!("{:3}   {}", i + 1, line);
            }
        }

        io::stdout().flush().unwrap();
    }

    fn run(&mut self) {
        println!("Simple Text Editor - Type :help for commands");

        loop {
            self.display();

            print!("\n> ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let input = input.trim();

            if input.starts_with(':') {
                match input {
                    ":q" => break,
                    ":w" => {
                        if let Err(e) = self.save_file() {
                            println!("Error saving: {}", e);
                        }
                    }
                    ":wq" => {
                        if let Err(e) = self.save_file() {
                            println!("Error saving: {}", e);
                        } else {
                            break;
                        }
                    }
                    cmd if cmd.starts_with(":w ") => {
                        let filename = &cmd[3..];
                        if let Err(e) = self.save_as(filename) {
                            println!("Error saving: {}", e);
                        }
                    }
                    ":help" => {
                        println!("Commands:");
                        println!("  :q       - Quit");
                        println!("  :w       - Save");
                        println!("  :wq      - Save and quit");
                        println!("  :w <file> - Save as");
                        println!("  i <text> - Insert text at cursor");
                        println!("  d        - Delete character before cursor");
                        println!("  n        - New line");
                        std::thread::sleep(std::time::Duration::from_secs(3));
                    }
                    _ => println!("Unknown command: {}", input),
                }
            } else if input.starts_with("i ") {
                let text = &input[2..];
                for c in text.chars() {
                    self.insert_char(c);
                }
            } else if input == "d" {
                self.delete_char();
            } else if input == "n" {
                self.insert_newline();
            }
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut editor = Editor::new();

    if args.len() > 1 {
        let filename = &args[1];
        if Path::new(filename).exists() {
            if let Err(e) = editor.load_file(filename) {
                eprintln!("Error loading file: {}", e);
            }
        } else {
            editor.filename = Some(filename.to_string());
            println!("Creating new file: {}", filename);
        }
    }

    editor.run();
}

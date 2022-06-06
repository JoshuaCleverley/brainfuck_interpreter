use std::{
    fs::File,
    io::prelude::*,
    path::Path,
};

const MEMORY_SIZE: usize = 30000;

fn main() {
    interpret_brainfuck("test.txt");
}

fn open_file(path: &str) -> Vec<char> {
    let path = Path::new(path);
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(e) => panic!("Couldn't open {}: {}", display, e),
        Ok(file) => file,
    };

    let mut file_string = String::new();
    match file.read_to_string(&mut file_string) {
        Err(e) => panic!("Couldn't read {}: {}", display, e),
        _ => (),
    }

    file_string.chars().collect()
}

fn interpret_brainfuck(path: &str) {
    let mut mem = vec![0u8; MEMORY_SIZE];
    let mut addr: isize = 0;

    let code = open_file(path);
    let code_length = code.len();
    let mut code_index = 0;

    while code_index < code_length {
        let c = code[code_index];
        match c {
            '>' => {
                addr += 1;
                if addr == MEMORY_SIZE as isize { addr = 0; }
            },
            '<' => {
                addr -= 1;
                if addr < 0 { addr = MEMORY_SIZE as isize - 1; }
            },
            '+' => mem[addr as usize] += 1,
            '-' => mem[addr as usize] -= 1,
            '.' => print!("{}", mem[addr as usize] as char),
            ',' => {
                let input = 
                    std::io::stdin()
                    .bytes()
                    .next()
                    .and_then(|result| result.ok())
                    .unwrap();
                mem[addr as usize] = input;
            },
            '[' => {
                if mem[addr as usize] == 0 {
                    let mut depth = 1;
                    while depth != 0 {
                        code_index += 1;
                        if code_index == code_length { panic!("[ command without corresponding ]"); }
                        if code[code_index] == '[' { depth += 1; }
                        if code[code_index] == ']' { depth -= 1; }
                    }
                }
            },
            ']' => {
                if mem[addr as usize] != 0 {
                    let mut depth = 1;
                    while depth != 0 {
                        code_index -= 1;
                        if code_index == code_length { panic!("] command without corresponding ["); }
                        if code[code_index] == ']' { depth += 1; }
                        if code[code_index] == '[' { depth -= 1; }
                    }
                }
            },
            _ => (),
        }
        code_index += 1;
    }
}
use std::{path::Path, fs::{File, read_to_string}, io::Read};

mod cpu;

pub struct Emulator {
    cpu: cpu::Cpu,
}

impl Emulator {
    pub fn new(flag: &String, path: &Path) -> Emulator {
        let program;
        if flag == "-b" {
            program = Emulator::read_program_bin(path);
        } else if flag == "-t" {
            program = Emulator::read_program_text(path);
        } else {
            panic!("Invalid flag");
        }

        Emulator {
            cpu: cpu::Cpu::new(program),
        }
    }

    fn read_program_text(path: &Path) -> Vec<u8> {
        let file_string = read_to_string(&path).expect("Failed to read file.");
        file_string
            .split(char::is_whitespace)
            .filter(|item| !item.is_empty())
            .enumerate()
            .map(|(index, item)| {
                u8::from_str_radix(item, 16)
                    .expect(format!("Failed to parse opcode at: {} '{}'", index + 1, item).as_str())
            })
            .collect()
    }
    
    fn read_program_bin(path: &Path) -> Vec<u8> {
        // TODO: Better error handling
        let mut buffer: Vec<u8> = Vec::new();
        File::open(path).unwrap().read_to_end(&mut buffer).unwrap();
        buffer
    }

    pub fn start(&mut self) {
        self.cpu.run();
    }
}
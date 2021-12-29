use std::{env, fs::{read_to_string, File}, path::Path, io::Read};
mod cpu;

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
    let mut buffer: Vec<u8> = Vec::new();
    File::open(path).unwrap().read_to_end(&mut buffer).unwrap();
    buffer
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        panic!("Missing file path.");
    }

    let path = Path::new(&args[2]);
    let program;
    if args[1] == "-b" {
        program = read_program_bin(&path);
    } else if args[1] == "-t" {
        program = read_program_text(&path);
    } else {
        panic!("Invalid flag");
    }

    let mut cpu: cpu::Cpu = cpu::Cpu::new(program);
    cpu.run();
    // cpu.print_registers();
}

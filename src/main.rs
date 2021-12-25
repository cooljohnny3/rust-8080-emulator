use std::{env, fs::read_to_string, path::Path};
mod cpu;

fn read_program(path: &Path) -> Vec<u8> {
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

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Missing file path.");
    }

    // read program
    let path = Path::new(&args[1]);
    let program = read_program(&path);

    let mut cpu: cpu::Cpu = cpu::Cpu::new(program);
    cpu.run();
    cpu.print_registers();
}

use std::{env, path::Path};

mod emulator;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        panic!("Missing file path.");
    }

    let mut emu: emulator::Emulator = emulator::Emulator::new(&args[1], &Path::new(&args[2]));
    emu.start();
}

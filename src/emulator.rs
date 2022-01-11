use std::{path::Path, fs::{File, read_to_string}, io::Read, time::Duration};

use sdl2::{pixels::Color, event::Event, keyboard::Keycode, video::Window, render::Canvas, Sdl, rect::Point};

mod cpu;

// Actual window dimensions
const SCREEN_WIDTH: usize = 448;
const SCREEN_HEIGHT: usize = 512;

// Logical window dimmensions
const LOGICAL_SCREEN_WIDTH: usize = 224;
const LOGICAL_SCREEN_HEIGHT: usize = 256;

pub struct Emulator {
    breakpoints: Vec<u16>,
    cpu: cpu::Cpu,
    sdl_context: Sdl,
    canvas: Canvas<Window>,
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

        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem.window("Space Invaders", SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32)
            .position_centered()
            .build()
            .unwrap();
        let mut canvas = window.into_canvas().build().unwrap();
        canvas.set_logical_size(LOGICAL_SCREEN_WIDTH as u32, LOGICAL_SCREEN_HEIGHT as u32).unwrap();

        let breakpoints: Vec<u16> = vec![
            // Add any breakpoints here
            0x18DF,
        ];

        Emulator {
            breakpoints,
            cpu: cpu::Cpu::new(program),
            sdl_context,
            canvas,
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
        let mut event_pump = self.sdl_context.event_pump().unwrap();

        'running: loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit {..} |
                    Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        break 'running
                    },
                    Event::KeyDown { keycode: Some(Keycode::P), .. } => {
                        if self.cpu.enable != 0 {
                            println!("Stopping");
                            self.cpu.enable = 0;
                        } else {
                            println!("Running");
                            self.cpu.enable = 1;
                        }
                    },
                    Event::KeyDown { keycode: Some(Keycode::Space), .. } => {
                        self.clear_screen();

                        self.cpu.enable = 1;
                        self.cpu.cycle();
                        self.cpu.enable = 0;

                        self.update_screen();
                        self.cpu.print_registers();
                    },
                    _ => {}
                }
            }

            // The rest of the game loop goes here...
            if self.cpu.enable != 0 {
                self.clear_screen();
                self.cpu.cycle();
                self.update_screen();
                self.check_breakpoint();
            }

            std::thread::sleep(2 * Duration::from_micros(1)); // Should be 2Mhz
        }
    }

    fn check_breakpoint(&mut self) {
        for breakpoint in &self.breakpoints {
            if self.cpu.pc == *breakpoint {
                self.cpu.enable = 0;
                println!("BREAK {:04X}", self.cpu.pc);
                self.cpu.print_registers();
                return;
            }
        }
    }

    fn clear_screen(&mut self) {
        self.canvas.set_draw_color(Color::BLACK);
        self.canvas.clear();
    }

    fn update_screen(&mut self) {
        self.canvas.set_draw_color(Color::WHITE);
        for byte_index in 0x2400..0x3FFF {
            let byte = self.cpu.memory[byte_index];
            if byte != 0 {
                let points = Emulator::byte_to_points(byte, byte_index);
                self.canvas.draw_points(points.as_slice()).unwrap();
            }
        }
        self.canvas.present();
    }

    fn byte_to_points(byte: u8, byte_index: usize) -> Vec<Point> {
        let mut points: Vec<Point> = Vec::new();
        let mask: u8 = 0b10000000;
        for i in 0..8 {
            if byte & (mask >> i) != 0 {
                let index = (byte_index - 0x2400) * 8;
                let x: i32 = ((index / LOGICAL_SCREEN_HEIGHT)) as i32;
                let y: i32 = (LOGICAL_SCREEN_HEIGHT - ((index % LOGICAL_SCREEN_HEIGHT) + i)) as i32;
                points.push(Point::new(x, y));
            }
        }
        points
    }
}

#[cfg(test)]
mod byte_to_xy_tests {
    use super::Emulator;

    #[test]
    fn it_works() {
        let mut result;

        for byte in 0..0xff {
            for byte_index in 0x2400..0x3fff {
                result = Emulator::byte_to_points(byte, byte_index);
                assert_eq!(result.len(), byte.count_ones() as usize);
            }
        }
    }
}
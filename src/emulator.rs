use std::{path::Path, fs::{File, read_to_string}, io::Read, time::Duration};

use sdl2::{pixels::Color, event::Event, keyboard::Keycode, rect::Point};

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
        const SCREEN_SIZE: usize = 0x4000 - 0x2400;
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
    
        let window = video_subsystem.window("Space Invaders", 256, 224)
            .position_centered()
            .build()
            .unwrap();
    
        let mut canvas = window.into_canvas().build().unwrap();
        let mut event_pump = sdl_context.event_pump().unwrap();

        'running: loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit {..} |
                    Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        break 'running
                    },
                    Event::KeyDown { keycode: Some(Keycode::P), .. } => {
                        if self.cpu.enable != 0 {
                            self.cpu.enable = 0;
                        } else {
                            self.cpu.enable = 1;
                        }
                    }
                    _ => {}
                }
            }
            // The rest of the game loop goes here...
            if self.cpu.enable != 0 {
                // Clear screen
                canvas.set_draw_color(Color::RGB(0, 0, 0));
                canvas.clear();

                self.cpu.cycle();

                // Draw screen
                canvas.set_draw_color(Color::RGB(255, 255, 255));
                // let points: [Point; SCREEN_SIZE] = [Point::new(0, 0); SCREEN_SIZE];
                for index in 0..SCREEN_SIZE {
                    // point.x = index as i32;
                    // point.y = (index % 224) as i32;
                    // println!("{:X}", self.cpu.memory[(index % 224) + 0x2400]);
                    let byte = self.cpu.memory[index + 0x2400];
                    if byte != 0 {
                        let mut mask: u8 = 1;
                        for i in 1..8 {
                            if byte & mask != 0 {
                                let x: i32 = (index as i32 / 256) + i;
                                let y: i32 = 256 - (index as i32 % 256) + i;
                                canvas.draw_point((y, x)).unwrap();
                            }
                            mask = mask << 1;
                        }
                    }
                    
                }
                // canvas.draw_points(&points[..]).unwrap();
                canvas.present();
            }

            // std::thread::sleep(Duration::from_millis(10));
        }
    }
}
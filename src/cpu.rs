use core::panic;
use std::u8;

#[allow(dead_code)]
#[derive(Debug)]
struct ConditionCodes {
    z: bool,
    s: bool,
    p: bool,
    cy: bool,
    ac: bool,
}

#[allow(dead_code)]
pub struct Cpu {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    sp: u16,
    pc: u16,
    memory: [u8; 0xffff],
    condition_codes: ConditionCodes,
    enable: u8,
}

impl Cpu {
    pub fn new(program: Vec<u8>) -> Cpu {
        let mut memory: [u8; 0xffff] = [0; 0xffff];
        for (index, opcode) in program.into_iter().enumerate() {
            memory[index] = opcode;
        }
        Cpu {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            sp: 0, // TODO: init to correct value
            pc: 0,
            memory: memory,
            condition_codes: ConditionCodes {
                z: true,
                s: false,
                p: false,
                cy: false,
                ac: false,
            },
            enable: 0,
        }
    }

    fn unimplimented(&self) {
        panic!("Unimplimented instruction:  {:#04X}", self.fetch(0));
    }

    pub fn run(&mut self) {
        self.enable = 1;
        while self.enable == 1 {
            println!("{:02X}", self.fetch(0));
            match self.fetch(0) {
                0x00 => {
                    // NOP
                }
                0x01 => {
                    // LXI B,D16
                    self.c = self.fetch(1);
                    self.b = self.fetch(2);
                    self.pc += 2;
                }
                0x02 => {
                    // STAX B
                    self.memory[self.get_bc() as usize] = self.a;
                }
                0x03 => {
                    // INX B
                    let answer: u16 = self.get_bc() + 1;
                    self.set_bc(answer);
                }
                0x04 => {
                    // INR B
                    let answer: u16 = (self.b + 1) as u16;
                    self.update_condition_codes(answer, true, true, true, false, true);
                    self.b = answer as u8;
                }
                0x05 => {
                    // DCR B
                    let answer: u16 = (self.b - 1) as u16;
                    self.update_condition_codes(answer, true, true, true, false, true);
                    self.b = answer as u8;
                }
                0x06 => {
                    // MVI B, D8
                    self.pc += 1;
                    self.b = self.fetch(0);
                }
                0x07 => {
                    // RLC
                    let x = self.a;
                    self.a = x << 1 | x >> 7;
                    self.condition_codes.cy = (x & 1) == 1;
                }
                0x08 => {
                    // -
                }
                0x09 => {
                    // DAD B
                    let answer = self.get_hl() + self.get_bc();
                    self.set_hl(answer);
                }
                0x0a => {
                    // LDAX B
                    self.a = self.memory[self.get_bc() as usize];
                }
                0x0b => {
                    // DCX B
                    let answer = self.get_bc() - 1;
                    self.set_bc(answer);
                }
                0x0c => {
                    // INR C
                    let answer: u16 = (self.c + 1) as u16;
                    self.update_condition_codes(answer, true, true, true, false, true);
                    self.c = answer as u8;
                }
                0x0d => {
                    // DCR C
                    let answer: u16 = (self.c - 1) as u16;
                    self.update_condition_codes(answer, true, true, true, false, true);
                    self.c = answer as u8;
                }
                0x0e => {
                    // MVI C, D8
                    self.pc += 1;
                    self.c = self.fetch(0);
                }
                0x0f => {
                    // RRC
                    let x = self.a;
                    self.a = x << 7 | x >> 1;
                    self.condition_codes.cy = (x & 1) == 1;
                }
                0x10 => {
                    // -
                }
                0x11 => {
                    // LXI D,D16
                    self.e = self.fetch(1);
                    self.d = self.fetch(2);
                    self.pc += 2;
                }
                0x12 => {
                    // STAX D
                    self.memory[self.get_de() as usize] = self.a;
                }
                0x13 => {
                    // INX D
                    let answer: u16 = self.get_de() + 1;
                    self.set_de(answer);
                }
                0x14 => {
                    // INR D
                    let answer: u16 = (self.d + 1) as u16;
                    self.update_condition_codes(answer, true, true, true, false, true);
                    self.d = answer as u8;
                }
                0x15 => {
                    // DCR D
                    let answer: u16 = (self.d - 1) as u16;
                    self.update_condition_codes(answer, true, true, true, false, true);
                    self.d = answer as u8;
                }
                0x16 => {
                    // MVI D, D8
                    self.pc += 1;
                    self.d = self.fetch(0);
                }
                0x17 => {
                    // RAL
                    let x = self.a;
                    self.a = x << 1 | self.condition_codes.cy as u8;
                    self.condition_codes.cy = (x & 1) == 1;
                }
                0x18 => {
                    // -
                }
                0x19 => {
                    // DAD D
                    let answer = self.get_hl() + self.get_de();
                    self.set_hl(answer);
                }
                0x1c => {
                    // INR E
                    let answer: u16 = (self.e + 1) as u16;
                    self.update_condition_codes(answer, true, true, true, false, true);
                    self.e = answer as u8;
                }
                0x1d => {
                    // DCR E
                    let answer: u16 = (self.e - 1) as u16;
                    self.update_condition_codes(answer, true, true, true, false, true);
                    self.e = answer as u8;
                }
                0x1e => {
                    // MVI E, D8
                    self.pc += 1;
                    self.e = self.fetch(0);
                }
                0x1f => {
                    // RAR
                    let x = self.a;
                    self.a = (self.condition_codes.cy as u8) << 7 | x >> 1;
                    self.condition_codes.cy = (x & 1) == 1;
                }
                0x20 => {
                    // -
                }
                0x21 => {
                    // LXI H,D,D16
                    self.l = self.fetch(1);
                    self.h = self.fetch(2);
                    self.pc += 2;
                }
                0x22 => {
                    // SHLD adr
                    let addr: u16 = (self.fetch(2) as u16) << 8 | self.fetch(1) as u16;
                    self.memory[addr as usize] = self.h;
                    self.memory[(addr + 1) as usize] = self.l;
                    self.pc += 2;
                }
                0x23 => {
                    // INX H
                    let answer: u16 = self.get_hl() + 1;
                    self.set_hl(answer);
                }
                0x24 => {
                    // INR H
                    let answer: u16 = (self.h + 1) as u16;
                    self.update_condition_codes(answer, true, true, true, false, true);
                    self.h = answer as u8;
                }
                0x25 => {
                    // DCR H
                    let answer: u16 = (self.h - 1) as u16;
                    self.update_condition_codes(answer, true, true, true, false, true);
                    self.h = answer as u8;
                }
                0x26 => {
                    // MVI H, D8
                    self.pc += 1;
                    self.h = self.fetch(0);
                }
                0x27 => {
                    // DAA
                }
                0x28 => {
                    // -
                }
                0x29 => {
                    // DAD H
                    let answer = self.get_hl() + self.get_hl();
                    self.set_hl(answer);
                    // TODO: set CY
                }
                0x2a => {
                    // JHJD adr
                    let addr: u16 = (self.fetch(2) as u16) << 8 | self.fetch(1) as u16;
                    self.memory[addr as usize] = self.h;
                    self.memory[(addr + 1) as usize] = self.l;
                    self.pc += 2;
                }
                0x2b => {
                    // DCX H
                    let answer: u16 = self.get_hl() - 1;
                    self.set_hl(answer);
                }
                0x2c => {
                    // INR L
                    let answer: u16 = (self.l + 1) as u16;
                    self.update_condition_codes(answer, true, true, true, false, true);
                    self.l = answer as u8;
                }
                0x2d => {
                    // DCR L
                    let answer: u16 = (self.l - 1) as u16;
                    self.update_condition_codes(answer, true, true, true, false, true);
                    self.l = answer as u8;
                }
                0x2e => {
                    // MVI L, D8
                    self.pc += 1;
                    self.l = self.fetch(0);
                }
                0x2f => {
                    // CMA
                    self.a = !self.a;
                }
                0x30 => {
                    // -
                }
                0x31 => {
                    // LXI SP,D16
                    self.sp = (self.fetch(2) as u16) << 8 | self.fetch(1) as u16;
                    self.pc += 2;
                }
                0x32 => {
                    // STA adr
                    let adr = (self.fetch(2) as u16) << 8 | self.fetch(1) as u16;
                    self.memory[adr as usize] = self.a;
                    self.pc += 2;
                }
                0x33 => {
                    //INX SP
                    self.sp += 1;
                }
                0x34 => {
                    // INR M
                    self.memory[self.get_hl() as usize] = self.memory[self.get_hl() as usize] + 1;
                }
                0x37 => {
                    // STC
                    self.condition_codes.cy = true;
                }
                0x38 => {
                    // -
                }
                0x39 => {
                    // DAD SP
                    let answer = self.get_hl() + self.sp;
                    self.set_hl(answer);
                    // TODO: set CY
                }
                0x3a => {
                    // LDA adr
                    let adr = (self.fetch(2) as u16) << 8 | self.fetch(1) as u16;
                    self.a = self.memory[adr as usize];
                }
                0x3b => self.sp += 1, // DCX SP
                0x3c => {
                    // INR A
                    let answer: u16 = (self.a + 1) as u16;
                    self.update_condition_codes(answer, true, true, true, false, true);
                    self.a = answer as u8;
                }
                0x3d => {
                    // DCR A
                    let answer: u16 = (self.a - 1) as u16;
                    self.update_condition_codes(answer, true, true, true, false, true);
                    self.a = answer as u8;
                }
                0x3e => {
                    // MVI A, D8
                    self.pc += 1;
                    self.a = self.fetch(0);
                }
                0x3f => {
                    // CMC
                    self.condition_codes.cy = !self.condition_codes.cy;
                }
                0x40 => self.b = self.b, // MOV B,B
                0x41 => self.b = self.c, // MOV B,C
                0x42 => self.b = self.d, // MOV B,D
                0x43 => self.b = self.e, // MOV B,E
                0x44 => self.b = self.h, // MOV B,H
                0x45 => self.b = self.l, // MOV B,L
                0x46 => self.b = self.memory[self.get_hl() as usize], // MOV B,M
                0x47 => self.b = self.a, // MOV B,A
                0x48 => self.c = self.b, // MOV C,B
                0x49 => self.c = self.c, // MOV C,C
                0x4a => self.c = self.d, // MOV C,D
                0x4b => self.c = self.e, // MOV C,E
                0x4c => self.c = self.h, // MOV C,H
                0x4d => self.c = self.l, // MOV C,L
                0x4e => self.c = self.memory[self.get_hl() as usize], // MOV C,M
                0x4f => self.c = self.a, // MOV C,A
                0x50 => self.d = self.b, // MOV D,B
                0x51 => self.d = self.c, // MOV D,C
                0x52 => self.d = self.d, // MOV D,D
                0x53 => self.d = self.e, // MOV D,E
                0x54 => self.d = self.h, // MOV D,H
                0x55 => self.d = self.l, // MOV D,L
                0x56 => self.d = self.memory[self.get_hl() as usize], // MOV D,M
                0x57 => self.d = self.a, // MOV D,A
                0x58 => self.e = self.b, // MOV E,B
                0x59 => self.e = self.c, // MOV E,C
                0x5a => self.e = self.d, // MOV E,D
                0x5b => self.e = self.e, // MOV E,E
                0x5c => self.e = self.h, // MOV E,H
                0x5d => self.e = self.l, // MOV E,L
                0x5e => self.e = self.memory[self.get_hl() as usize], // MOV E,M
                0x5f => self.e = self.a, // MOV E,A
                0x60 => self.h = self.b, // MOV H,B
                0x61 => self.h = self.c, // MOV H,C
                0x62 => self.h = self.d, // MOV H,D
                0x63 => self.h = self.e, // MOV H,E
                0x64 => self.h = self.h, // MOV H,H
                0x65 => self.h = self.l, // MOV H,L
                0x66 => self.h = self.memory[self.get_hl() as usize], // MOV H,M
                0x67 => self.h = self.a, // MOV H,A
                0x68 => self.l = self.b, // MOV L,B
                0x69 => self.l = self.c, // MOV L,C
                0x6a => self.l = self.d, // MOV L,D
                0x6b => self.l = self.e, // MOV L,E
                0x6c => self.l = self.h, // MOV L,H
                0x6d => self.l = self.l, // MOV L,L
                0x6e => self.l = self.memory[self.get_hl() as usize], // MOV L,M
                0x6f => self.l = self.a, // MOV L,A
                0x70 => self.memory[self.get_hl() as usize] = self.b, // MOV M,B
                0x71 => self.memory[self.get_hl() as usize] = self.c, // MOV M,C
                0x72 => self.memory[self.get_hl() as usize] = self.d, // MOV M,D
                0x73 => self.memory[self.get_hl() as usize] = self.e, // MOV M,E
                0x74 => self.memory[self.get_hl() as usize] = self.h, // MOV M,H
                0x75 => self.memory[self.get_hl() as usize] = self.l, // MOV M,L
                0x76 => {
                    // HLT
                    println!("Halting");
                    self.enable = 0;
                }
                0x77 => self.memory[self.get_hl() as usize] = self.a, // MOV M,A
                0x78 => self.a = self.b,                              // MOV A,B
                0x79 => self.a = self.c,                              // MOV A,C
                0x7a => self.a = self.d,                              // MOV A,D
                0x7b => self.a = self.e,                              // MOV A,E
                0x7c => self.a = self.h,                              // MOV A,H
                0x7d => self.a = self.l,                              // MOV A,L
                0x7e => self.a = self.memory[self.get_hl() as usize], // MOV A,M
                0x7f => self.a = self.a,                              // MOV A,A
                0x80 => {
                    // ADD B
                    let answer: u16 = (self.a + self.b) as u16;
                    self.update_condition_codes(answer, true, true, true, true, true);
                    self.a = answer as u8;
                }
                0x81 => {
                    // ADD C
                    let answer: u16 = (self.a + self.c) as u16;
                    self.update_condition_codes(answer, true, true, true, true, true);
                    self.a = answer as u8;
                }
                0x82 => {
                    // ADD D
                    let answer: u16 = (self.a + self.d) as u16;
                    self.update_condition_codes(answer, true, true, true, true, true);
                    self.a = answer as u8;
                }
                0x83 => {
                    // ADD E
                    let answer: u16 = (self.a + self.e) as u16;
                    self.update_condition_codes(answer, true, true, true, true, true);
                    self.a = answer as u8;
                }
                0x84 => {
                    // ADD H
                    let answer: u16 = (self.a + self.h) as u16;
                    self.update_condition_codes(answer, true, true, true, true, true);
                    self.a = answer as u8;
                }
                0x85 => {
                    // ADD L
                    let answer: u16 = (self.a + self.l) as u16;
                    self.update_condition_codes(answer, true, true, true, true, true);
                    self.a = answer as u8;
                }
                0x86 => {
                    // ADD M
                    let answer: u16 = (self.a + self.memory[self.get_hl() as usize]) as u16;
                    self.update_condition_codes(answer, true, true, true, true, true);
                    self.a = answer as u8;
                }
                0x87 => {
                    // ADD A
                    let answer: u16 = (self.a + self.a) as u16;
                    self.update_condition_codes(answer, true, true, true, true, true);
                    self.a = answer as u8;
                }
                0x88 => {
                    // ADC B
                    let answer: u16 = (self.a + self.b + self.condition_codes.cy as u8) as u16;
                    self.update_condition_codes(answer, true, true, true, true, true);
                    self.a = answer as u8;
                }
                0x89 => {
                    // ADC C
                    let answer: u16 = (self.a + self.c + self.condition_codes.cy as u8) as u16;
                    self.update_condition_codes(answer, true, true, true, true, true);
                    self.a = answer as u8;
                }
                0x8a => {
                    // ADC D
                    let answer: u16 = (self.a + self.d + self.condition_codes.cy as u8) as u16;
                    self.update_condition_codes(answer, true, true, true, true, true);
                    self.a = answer as u8;
                }
                0x8b => {
                    // ADC E
                    let answer: u16 = (self.a + self.e + self.condition_codes.cy as u8) as u16;
                    self.update_condition_codes(answer, true, true, true, true, true);
                    self.a = answer as u8;
                }
                0x8c => {
                    // ADC H
                    let answer: u16 = (self.a + self.h + self.condition_codes.cy as u8) as u16;
                    self.update_condition_codes(answer, true, true, true, true, true);
                    self.a = answer as u8;
                }
                0x8d => {
                    // ADC L
                    let answer: u16 = (self.a + self.l + self.condition_codes.cy as u8) as u16;
                    self.update_condition_codes(answer, true, true, true, true, true);
                    self.a = answer as u8;
                }
                0x8e => {
                    // ADC M
                    let answer: u16 = (self.a
                        + self.memory[self.get_hl() as usize]
                        + self.condition_codes.cy as u8)
                        as u16;
                    self.update_condition_codes(answer, true, true, true, true, true);
                    self.a = answer as u8;
                }
                0x8f => {
                    // ADC A
                    let answer: u16 = (self.a + self.a + self.condition_codes.cy as u8) as u16;
                    self.update_condition_codes(answer, true, true, true, true, true);
                    self.a = answer as u8;
                }
                0x90 => {
                    // SUB B
                    let answer: u16 = (self.a - self.b) as u16;
                    self.update_condition_codes(answer, true, true, true, true, true);
                    self.a = answer as u8;
                }
                0x91 => {
                    // SUB C
                    let answer: u16 = (self.a - self.c) as u16;
                    self.update_condition_codes(answer, true, true, true, true, true);
                    self.a = answer as u8;
                }
                0x92 => {
                    // SUB D
                    let answer: u16 = (self.a - self.d) as u16;
                    self.update_condition_codes(answer, true, true, true, true, true);
                    self.a = answer as u8;
                }
                0x93 => {
                    // SUB E
                    let answer: u16 = (self.a - self.e) as u16;
                    self.update_condition_codes(answer, true, true, true, true, true);
                    self.a = answer as u8;
                }
                0x94 => {
                    // SUB H
                    let answer: u16 = (self.a - self.h) as u16;
                    self.update_condition_codes(answer, true, true, true, true, true);
                    self.a = answer as u8;
                }
                0x95 => {
                    // SUB L
                    let answer: u16 = (self.a - self.l) as u16;
                    self.update_condition_codes(answer, true, true, true, true, true);
                    self.a = answer as u8;
                }
                0x96 => {
                    // SUB M
                    let answer: u16 = (self.a - self.memory[self.get_hl() as usize]) as u16;
                    self.update_condition_codes(answer, true, true, true, true, true);
                    self.a = answer as u8;
                }
                0x97 => {
                    // SUB A
                    let answer: u16 = (self.a - self.a) as u16;
                    self.update_condition_codes(answer, true, true, true, true, true);
                    self.a = answer as u8;
                }
                0x98 => {
                    // SBB B
                    let answer: u16 = (self.a - self.b - self.condition_codes.cy as u8) as u16;
                    self.update_condition_codes(answer, true, true, true, true, true);
                    self.a = answer as u8;
                }
                0x99 => {
                    // SBB C
                    let answer: u16 = (self.a - self.c - self.condition_codes.cy as u8) as u16;
                    self.update_condition_codes(answer, true, true, true, true, true);
                    self.a = answer as u8;
                }
                0x9a => {
                    // SBB D
                    let answer: u16 = (self.a - self.d - self.condition_codes.cy as u8) as u16;
                    self.update_condition_codes(answer, true, true, true, true, true);
                    self.a = answer as u8;
                }
                0x9b => {
                    // SBB E
                    let answer: u16 = (self.a - self.e - self.condition_codes.cy as u8) as u16;
                    self.update_condition_codes(answer, true, true, true, true, true);
                    self.a = answer as u8;
                }
                0x9c => {
                    // SBB H
                    let answer: u16 = (self.a - self.h - self.condition_codes.cy as u8) as u16;
                    self.update_condition_codes(answer, true, true, true, true, true);
                    self.a = answer as u8;
                }
                0x9d => {
                    // SBB L
                    let answer: u16 = (self.a - self.l - self.condition_codes.cy as u8) as u16;
                    self.update_condition_codes(answer, true, true, true, true, true);
                    self.a = answer as u8;
                }
                0x9e => {
                    // SUB M
                    let answer: u16 = (self.a
                        - self.memory[self.get_hl() as usize]
                        - self.condition_codes.cy as u8)
                        as u16;
                    self.update_condition_codes(answer, true, true, true, true, true);
                    self.a = answer as u8;
                }
                0x9f => {
                    // SBB A
                    let answer: u16 = (self.a + self.a + self.condition_codes.cy as u8) as u16;
                    self.update_condition_codes(answer, true, true, true, true, true);
                    self.a = answer as u8;
                }
                0xa0 => {
                    // ANA B
                    let answer: u16 = (self.a & self.b) as u16;
                    self.update_condition_codes(answer, true, true, true, true, true);
                    self.a = answer as u8;
                }
                0xa1 => {
                    // ANA C
                    let answer: u16 = (self.a & self.c) as u16;
                    self.update_condition_codes(answer, true, true, true, true, true);
                    self.a = answer as u8;
                }
                0xa2 => {
                    // ANA D
                    let answer: u16 = (self.a & self.d) as u16;
                    self.update_condition_codes(answer, true, true, true, true, true);
                    self.a = answer as u8;
                }
                0xa3 => {
                    // ANA E
                    let answer: u16 = (self.a & self.e) as u16;
                    self.update_condition_codes(answer, true, true, true, true, true);
                    self.a = answer as u8;
                }
                0xa4 => {
                    // ANA H
                    let answer: u16 = (self.a & self.h) as u16;
                    self.update_condition_codes(answer, true, true, true, true, true);
                    self.a = answer as u8;
                }
                0xa5 => {
                    // ANA L
                    let answer: u16 = (self.a & self.l) as u16;
                    self.update_condition_codes(answer, true, true, true, true, true);
                    self.a = answer as u8;
                }
                0xa6 => {
                    // ANA M
                    let answer: u16 = (self.a & self.memory[self.get_hl() as usize]) as u16;
                    self.update_condition_codes(answer, true, true, true, true, true);
                    self.a = answer as u8;
                }
                0xa7 => {
                    // ANA A
                    let answer: u16 = (self.a & self.a) as u16;
                    self.update_condition_codes(answer, true, true, true, true, true);
                    self.a = answer as u8;
                }
                0xa8 => {
                    // XRA B
                    let answer: u16 = (self.a ^ self.b) as u16;
                    self.update_condition_codes(answer, true, true, true, true, true);
                    self.a = answer as u8;
                }
                0xa9 => {
                    // XRA C
                    let answer: u16 = (self.a ^ self.c) as u16;
                    self.update_condition_codes(answer, true, true, true, true, true);
                    self.a = answer as u8;
                }
                0xaa => {
                    // XRA D
                    let answer: u16 = (self.a ^ self.d) as u16;
                    self.update_condition_codes(answer, true, true, true, true, true);
                    self.a = answer as u8;
                }
                0xab => {
                    // XRA E
                    let answer: u16 = (self.a ^ self.e) as u16;
                    self.update_condition_codes(answer, true, true, true, true, true);
                    self.a = answer as u8;
                }
                0xac => {
                    // XRA H
                    let answer: u16 = (self.a ^ self.h) as u16;
                    self.update_condition_codes(answer, true, true, true, true, true);
                    self.a = answer as u8;
                }
                0xad => {
                    // XRA L
                    let answer: u16 = (self.a ^ self.l) as u16;
                    self.update_condition_codes(answer, true, true, true, true, true);
                    self.a = answer as u8;
                }
                0xae => {
                    // XRA M
                    let answer: u16 = (self.a ^ self.memory[self.get_hl() as usize]) as u16;
                    self.update_condition_codes(answer, true, true, true, true, true);
                    self.a = answer as u8;
                }
                0xaf => {
                    // ORA A
                    let answer: u16 = (self.a | self.a) as u16;
                    self.update_condition_codes(answer, true, true, true, true, true);
                    self.a = answer as u8;
                }
                0xb0 => {
                    // ORA B
                    let answer: u16 = (self.a | self.b) as u16;
                    self.update_condition_codes(answer, true, true, true, true, true);
                    self.a = answer as u8;
                }
                0xb1 => {
                    // ORA C
                    let answer: u16 = (self.a | self.c) as u16;
                    self.update_condition_codes(answer, true, true, true, true, true);
                    self.a = answer as u8;
                }
                0xb2 => {
                    // ORA D
                    let answer: u16 = (self.a | self.d) as u16;
                    self.update_condition_codes(answer, true, true, true, true, true);
                    self.a = answer as u8;
                }
                0xb3 => {
                    // ORA E
                    let answer: u16 = (self.a | self.e) as u16;
                    self.update_condition_codes(answer, true, true, true, true, true);
                    self.a = answer as u8;
                }
                0xb4 => {
                    // ORA H
                    let answer: u16 = (self.a | self.h) as u16;
                    self.update_condition_codes(answer, true, true, true, true, true);
                    self.a = answer as u8;
                }
                0xb5 => {
                    // ORA L
                    let answer: u16 = (self.a | self.l) as u16;
                    self.update_condition_codes(answer, true, true, true, true, true);
                    self.a = answer as u8;
                }
                0xb6 => {
                    // ORA M
                    let answer: u16 = (self.a | self.memory[self.get_hl() as usize]) as u16;
                    self.update_condition_codes(answer, true, true, true, true, true);
                    self.a = answer as u8;
                }
                0xb7 => {
                    // ORA A
                    let answer: u16 = (self.a ^ self.a) as u16;
                    self.update_condition_codes(answer, true, true, true, true, true);
                    self.a = answer as u8;
                }
                0xc1 => {
                    // POP B
                    self.c = self.memory[self.sp as usize];
                    self.b = self.memory[(self.sp+1) as usize];
                    self.sp += 2;
                }

                0xc5 => {
                    // PUSH B
                    self.memory[(self.sp-1) as usize] = self.b;
                    self.memory[(self.sp-2) as usize] = self.c;
                    self.sp -= 2;
                }

                0xcb => {
                    // -
                }

                0xd1 => {
                    // POP D
                    self.e = self.memory[self.sp as usize];
                    self.d = self.memory[(self.sp+1) as usize];
                    self.sp += 2;
                }

                0xd5 => {
                    // PUSH D
                    self.memory[(self.sp-1) as usize] = self.d;
                    self.memory[(self.sp-2) as usize] = self.e;
                    self.sp -= 2;
                }

                0xd9 => {
                    // -
                }

                0xdd => {
                    // -
                }

                0xed => {
                    // -
                }

                0xfd => {
                    // -
                }

                _ => self.unimplimented(),
            }
            self.pc += 1;
        }
    }

    fn get_bc(&self) -> u16 {
        ((self.b as u16) << 8) | self.c as u16
    }

    fn get_de(&self) -> u16 {
        ((self.d as u16) << 8) | self.e as u16
    }

    fn get_hl(&self) -> u16 {
        ((self.h as u16) << 8) | self.l as u16
    }

    fn set_bc(&mut self, answer: u16) {
        self.b = (answer >> 8) as u8;
        self.c = answer as u8;
    }

    fn set_de(&mut self, answer: u16) {
        self.d = (answer >> 8) as u8;
        self.e = answer as u8;
    }

    fn set_hl(&mut self, answer: u16) {
        self.h = (answer >> 8) as u8;
        self.l = answer as u8;
    }

    fn update_condition_codes(
        &mut self,
        value: u16,
        z: bool,
        s: bool,
        p: bool,
        cy: bool,
        ac: bool,
    ) {
        let codes: &mut ConditionCodes = &mut self.condition_codes;
        if z {
            codes.z = (value & 0xff) == 0;
        }
        if s {
            codes.s = (value & 0x80) != 0;
        }
        if p {
            codes.p = false; // TODO
        }
        if cy {
            codes.cy = value > 0xff;
        }
        if ac {
            codes.ac = false;
        }
    }

    fn fetch(&self, offset: usize) -> u8 {
        self.memory[self.pc as usize + offset]
    }

    #[allow(dead_code)]
    pub fn print_registers(&self) {
        println!(
            "a={}\nb={}\nc={}\nd={}\ne={}\nh={}\nl={}\nsp={}\npc={}\n{:?}",
            self.a,
            self.b,
            self.c,
            self.d,
            self.e,
            self.h,
            self.l,
            self.sp,
            self.pc,
            self.condition_codes
        );
    }

    #[allow(dead_code)]
    pub fn print_memory(&self) {
        self.print_memory_width(32);
    }

    #[allow(dead_code)]
    pub fn print_memory_width(&self, width: usize) {
        for (index, code) in (&self.memory).into_iter().enumerate() {
            if index % width == 0 {
                println!("{:02X}", code);
            } else {
                print!("{:02X} ", code);
            }
        }
    }
}

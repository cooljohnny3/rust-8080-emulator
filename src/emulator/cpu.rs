use core::panic;
use std::u8;

#[derive(Debug)]
struct ConditionCodes {
    z: bool,
    s: bool,
    p: bool,
    cy: bool,
    ac: bool,
}

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
    pub memory: [u8; 0xffff],
    condition_codes: ConditionCodes,
    pub enable: u8,
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
            sp: 0xffff - 1,
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
        panic!("Unimplimented instruction: {:#04X}", self.memory[self.pc as usize]);
    }

    pub fn cycle(&mut self) {
        // self.print_registers();
        // if self.pc == 0x1A65 {
        //     self.enable = 0;
        //     return;
        // }
        match self.memory[self.pc as usize] {
            0x00 => {
                // NOP
                self.pc += 1; // instruction
            }
            0x01 => {
                // LXI B,D16
                self.pc += 1; // instruction
                self.c = self.memory[self.pc as usize];
                self.b = self.memory[(self.pc+1) as usize];
                self.pc += 2;
            }
            0x02 => {
                // STAX B
                self.pc += 1; // instruction
                self.memory[self.get_bc() as usize] = self.a;
            }
            0x03 => {
                // INX B
                self.pc += 1; // instruction
                let answer: u16 = self.get_bc() + 1;
                self.set_bc(answer);
            }
            0x04 => {
                // INR B
                self.pc += 1; // instruction
                let answer: u16 = (self.b + 1) as u16;
                self.update_condition_codes(answer, true, true, true, false, true);
                self.b = answer as u8;
            }
            0x05 => {
                // DCR B
                self.pc += 1; // instruction
                let answer: u16 = (self.b - 1) as u16;
                self.update_condition_codes(answer, true, true, true, false, true);
                self.b = answer as u8;
            }
            0x06 => {
                // MVI B, D8
                self.pc += 1; // instruction
                self.b = self.memory[self.pc as usize];
                self.pc += 1;
            }
            0x07 => {
                // RLC
                self.pc += 1; // instruction
                let x = self.a;
                self.a = x << 1 | x >> 7;
                self.condition_codes.cy = (x & 1) == 1;
            }
            0x08 => {
                // -
                self.pc += 1; // instruction
            }
            0x09 => {
                // DAD B
                self.pc += 1; // instruction
                let answer = self.get_hl() + self.get_bc();
                self.set_hl(answer);
            }
            0x0a => {
                // LDAX B
                self.pc += 1; // instruction
                self.a = self.memory[self.get_bc() as usize];
            }
            0x0b => {
                // DCX B
                self.pc += 1; // instruction
                let answer = self.get_bc() - 1;
                self.set_bc(answer);
            }
            0x0c => {
                // INR C
                self.pc += 1; // instruction
                let answer: u16 = (self.c + 1) as u16;
                self.update_condition_codes(answer, true, true, true, false, true);
                self.c = answer as u8;
            }
            0x0d => {
                // DCR C
                self.pc += 1; // instruction
                let answer: u16 = (self.c - 1) as u16;
                self.update_condition_codes(answer, true, true, true, false, true);
                self.c = answer as u8;
            }
            0x0e => {
                // MVI C, D8
                self.pc += 1; // instruction
                self.c = self.memory[self.pc as usize];
                self.pc += 1;
            }
            0x0f => {
                // RRC
                self.pc += 1; // instruction
                let x = self.a;
                self.a = x << 7 | x >> 1;
                self.condition_codes.cy = (x & 1) == 1;
            }
            0x10 => {
                // -
                self.pc += 1; // instruction
            }
            0x11 => {
                // LXI D,D16
                self.pc += 1; // instruction
                self.e = self.memory[self.pc as usize];
                self.d = self.memory[self.pc as usize];
                self.pc += 2;
            }
            0x12 => {
                // STAX D
                self.pc += 1; // instruction
                self.memory[self.get_de() as usize] = self.a;
            }
            0x13 => {
                // INX D
                self.pc += 1; // instruction
                let answer: u16 = self.get_de() + 1;
                self.set_de(answer);
            }
            0x14 => {
                // INR D
                self.pc += 1; // instruction
                let answer: u16 = (self.d + 1) as u16;
                self.update_condition_codes(answer, true, true, true, false, true);
                self.d = answer as u8;
            }
            0x15 => {
                // DCR D
                self.pc += 1; // instruction
                let answer: u16 = (self.d - 1) as u16;
                self.update_condition_codes(answer, true, true, true, false, true);
                self.d = answer as u8;
            }
            0x16 => {
                // MVI D, D8
                self.pc += 1; // instruction
                self.d = self.memory[self.pc as usize];
                self.pc += 1;
            }
            0x17 => {
                // RAL
                self.pc += 1; // instruction
                let x = self.a;
                self.a = x << 1 | self.condition_codes.cy as u8;
                self.condition_codes.cy = (x & 1) == 1;
            }
            0x18 => {
                // -
                self.pc += 1; // instruction
            }
            0x19 => {
                // DAD D
                self.pc += 1; // instruction
                let answer = self.get_hl() + self.get_de();
                self.set_hl(answer);
            }
            0x1a => {
                // LDAX D
                self.pc += 1; // instruction
                self.a = self.memory[self.get_de() as usize];
            }
            0x1b => {
                // DCX D
                self.pc += 1; // instruction
                let answer = self.get_de() - 1;
                self.set_de(answer);
            }
            0x1c => {
                // INR E
                self.pc += 1; // instruction
                let answer: u16 = (self.e + 1) as u16;
                self.update_condition_codes(answer, true, true, true, false, true);
                self.e = answer as u8;
            }
            0x1d => {
                // DCR E
                self.pc += 1; // instruction
                let answer: u16 = (self.e - 1) as u16;
                self.update_condition_codes(answer, true, true, true, false, true);
                self.e = answer as u8;
            }
            0x1e => {
                // MVI E, D8
                self.pc += 1; // instruction
                self.e = self.memory[self.pc as usize];
                self.pc += 1;
            }
            0x1f => {
                // RAR
                self.pc += 1; // instruction
                let x = self.a;
                self.a = (self.condition_codes.cy as u8) << 7 | x >> 1;
                self.condition_codes.cy = (x & 1) == 1;
            }
            0x20 => {
                // -
                self.pc += 1; // instruction
            }
            0x21 => {
                // LXI H,D,D16
                self.pc += 1; // instruction
                self.l = self.memory[self.pc as usize];
                self.h = self.memory[(self.pc+1) as usize];
                self.pc += 2;
            }
            0x22 => {
                // SHLD adr
                self.pc += 1; // instruction
                let addr: u16 = (self.memory[(self.pc+1) as usize] as u16) << 8 | self.memory[self.pc as usize] as u16;
                self.pc += 2;
                self.memory[addr as usize] = self.h;
                self.memory[(addr + 1) as usize] = self.l;
            }
            0x23 => {
                // INX H
                self.pc += 1; // instruction
                let answer: u16 = self.get_hl() + 1;
                self.set_hl(answer);
            }
            0x24 => {
                // INR H
                self.pc += 1; // instruction
                let answer: u16 = (self.h + 1) as u16;
                self.update_condition_codes(answer, true, true, true, false, true);
                self.h = answer as u8;
            }
            0x25 => {
                // DCR H
                self.pc += 1; // instruction
                let answer: u16 = (self.h - 1) as u16;
                self.update_condition_codes(answer, true, true, true, false, true);
                self.h = answer as u8;
            }
            0x26 => {
                // MVI H, D8
                self.pc += 1; // instruction
                self.h = self.memory[self.pc as usize];
                self.pc += 1;
            }
            0x27 => {
                // DAA
                self.pc += 1; // instruction
            }
            0x28 => {
                // -
                self.pc += 1; // instruction
            }
            0x29 => {
                // DAD H
                self.pc += 1; // instruction
                let answer = self.get_hl() + self.get_hl();
                self.set_hl(answer);
                // TODO: set CY
            }
            0x2a => {
                // JHJD adr
                self.pc += 1; // instruction
                let addr: u16 = (self.memory[(self.pc+1) as usize] as u16) << 8 | self.memory[self.pc as usize] as u16;
                self.pc += 2;
                self.memory[addr as usize] = self.h;
                self.memory[(addr + 1) as usize] = self.l;
            }
            0x2b => {
                // DCX H
                self.pc += 1; // instruction
                let answer: u16 = self.get_hl() - 1;
                self.set_hl(answer);
            }
            0x2c => {
                // INR L
                self.pc += 1; // instruction
                let answer: u16 = (self.l + 1) as u16;
                self.update_condition_codes(answer, true, true, true, false, true);
                self.l = answer as u8;
            }
            0x2d => {
                // DCR L
                self.pc += 1; // instruction
                let answer: u16 = (self.l - 1) as u16;
                self.update_condition_codes(answer, true, true, true, false, true);
                self.l = answer as u8;
            }
            0x2e => {
                // MVI L, D8
                self.pc += 1; // instruction
                self.l = self.memory[self.pc as usize];
                self.pc += 1;
            }
            0x2f => {
                // CMA
                self.pc += 1; // instruction
                self.a = !self.a;
            }
            0x30 => {
                // -
                self.pc += 1; // instruction
            }
            0x31 => {
                // LXI SP,D16
                self.pc += 1; // instruction
                self.sp = (self.memory[(self.pc+1) as usize] as u16) << 8 | self.memory[self.pc as usize] as u16;
                self.pc += 2;
            }
            0x32 => {
                // STA adr
                self.pc += 1; // instruction
                let adr = (self.memory[(self.pc+1) as usize] as u16) << 8 | self.memory[self.pc as usize] as u16;
                self.pc += 2;
                self.memory[adr as usize] = self.a;
            }
            0x33 => {
                //INX SP
                self.pc += 1; // instruction
                self.sp += 1;
            }
            0x34 => {
                // INR M
                self.pc += 1; // instruction
                self.memory[self.get_hl() as usize] = self.memory[self.get_hl() as usize] + 1;
            }
            0x35 => {
                // DCR M
                self.pc += 1; // instruction
                let answer: u16 = self.get_hl() - 1;
                self.update_condition_codes(answer, true, true, true, false, true);
                self.set_hl(answer);
            }
            0x36 => {
                // 	MVI M,D8
                self.pc += 1; // instruction
                self.memory[self.get_hl() as usize] = self.memory[self.pc as usize];
                self.pc += 1;
            }
            0x37 => {
                // STC
                self.pc += 1; // instruction
                self.condition_codes.cy = true;
            }
            0x38 => {
                // -
                self.pc += 1; // instruction
            }
            0x39 => {
                // DAD SP
                let answer = self.get_hl() + self.sp;
                self.set_hl(answer);
                // TODO: set CY
            }
            0x3a => {
                // LDA adr
                self.pc += 1; // instruction
                let adr = (self.memory[self.pc as usize] as u16) << 8 | self.memory[self.pc as usize] as u16;
                self.a = self.memory[adr as usize];
            }
            0x3b => {
                // DCX SP
                self.pc += 1; // instruction
                self.sp += 1;
            }
            0x3c => {
                // INR A
                self.pc += 1; // instruction
                let answer: u16 = (self.a + 1) as u16;
                self.update_condition_codes(answer, true, true, true, false, true);
                self.a = answer as u8;
            }
            0x3d => {
                // DCR A
                self.pc += 1; // instruction
                let answer: u16 = (self.a - 1) as u16;
                self.update_condition_codes(answer, true, true, true, false, true);
                self.a = answer as u8;
            }
            0x3e => {
                // MVI A, D8
                self.pc += 1; // instruction
                self.a = self.memory[self.pc as usize];
                self.pc += 1;
            }
            0x3f => {
                // CMC
                self.pc += 1; // instruction
                self.condition_codes.cy = !self.condition_codes.cy;
            }
            0x40 => {
                // MOV B,B
                self.pc += 1; // instruction
                self.b = self.b;
            }
            0x41 => {
                // MOV B,C
                self.pc += 1; // instruction
                self.b = self.c;
            }
            0x42 => {
                // MOV B,D
                self.pc += 1; // instruction
                self.b = self.d; 
            }
            0x43 => {
                // MOV B,E
                self.pc += 1; // instruction
                self.b = self.e;
            }
            0x44 => {
                // MOV B,H
                self.pc += 1; // instruction
                self.b = self.h;
            }
            0x45 => {
                // MOV B,L
                self.pc += 1; // instruction
                self.b = self.l;
            }
            0x46 => {
                // MOV B,M
                self.pc += 1; // instruction
                self.b = self.memory[self.get_hl() as usize];
            }
            0x47 => {
                // MOV B,A
                self.pc += 1; // instruction
                self.b = self.a;
            }
            0x48 => {
                // MOV C,B
                self.pc += 1; // instruction
                self.c = self.b;
            }
            0x49 => {
                // MOV C,C
                self.pc += 1; // instruction
                self.c = self.c;
            }
            0x4a => {
                // MOV C,D
                self.pc += 1; // instruction
                self.c = self.d;
            }
            0x4b => {
                // MOV C,E
                self.pc += 1; // instruction
                self.c = self.e;
            }
            0x4c => {
                // MOV C,H
                self.pc += 1; // instruction
                self.c = self.h;
            }
            0x4d => {
                // MOV C,L
                self.pc += 1; // instruction
                self.c = self.l;
            }
            0x4e => {
                // MOV C,M
                self.pc += 1; // instruction
                self.c = self.memory[self.get_hl() as usize];
            }
            0x4f => {
                // MOV C,A
                self.pc += 1; // instruction
                self.c = self.a;
            }
            0x50 => {

                self.pc += 1; // instruction
                self.d = self.b; // MOV D,B
            }
            0x51 => {
                // MOV D,C
                self.pc += 1; // instruction
                self.d = self.c;
            }
            0x52 => {
                // MOV D,D
                self.pc += 1; // instruction
                self.d = self.d;
            }
            0x53 => {
                // MOV D,E
                self.pc += 1; // instruction
                self.d = self.e;
            }
            0x54 => {
                // MOV D,H
                self.pc += 1; // instruction
                self.d = self.h;
            }
            0x55 => {
                // MOV D,L
                self.pc += 1; // instruction
                self.d = self.l;
            }
            0x56 => {
                // MOV D,M
                self.pc += 1; // instruction
                self.d = self.memory[self.get_hl() as usize];
            }
            0x57 => {
                // MOV D,A
                self.pc += 1; // instruction
                self.d = self.a;
            }
            0x58 => {
                // MOV E,B
                self.pc += 1; // instruction
                self.e = self.b;
            }
            0x59 => {
                // MOV E,C
                self.pc += 1; // instruction
                self.e = self.c;
            }
            0x5a => {
                // MOV E,D
                self.pc += 1; // instruction
                self.e = self.d;
            }
            0x5b => {
                // MOV E,E
                self.pc += 1; // instruction
                self.e = self.e;
            }
            0x5c => {
                // MOV E,H
                self.pc += 1; // instruction
                self.e = self.h;
            }
            0x5d => {
                // MOV E,L
                self.pc += 1; // instruction
                self.e = self.l;
            }
            0x5e => {
                // MOV E,M
                self.pc += 1; // instruction
                self.e = self.memory[self.get_hl() as usize];
            }
            0x5f => {
                // MOV E,A
                self.pc += 1; // instruction
                self.e = self.a;
            }
            0x60 => {
                // MOV H,B
                self.pc += 1; // instruction
                self.h = self.b;
            }
            0x61 => {
                // MOV H,C
                self.pc += 1; // instruction
                self.h = self.c;
            }
            0x62 => {
                // MOV H,D
                self.pc += 1; // instruction
                self.h = self.d;
            }
            0x63 => {
                // MOV H,E
                self.pc += 1; // instruction
                self.h = self.e;
            }
            0x64 => {
                // MOV H,H
                self.pc += 1; // instruction
                self.h = self.h;
            }
            0x65 => {
                // MOV H,L
                self.pc += 1; // instruction
                self.h = self.l;
            }
            0x66 => {
                // MOV H,M
                self.pc += 1; // instruction
                self.h = self.memory[self.get_hl() as usize];
            }
            0x67 => {
                // MOV H,A
                self.pc += 1; // instruction
                self.h = self.a;
            }
            0x68 => {
                // MOV L,B
                self.pc += 1; // instruction
                self.l = self.b;
            }
            0x69 => {
                // MOV L,C
                self.pc += 1; // instruction
                self.l = self.c;
            }
            0x6a => {
                // MOV L,D
                self.pc += 1; // instruction
                self.l = self.d;
            }
            0x6b => {
                // MOV L,E
                self.pc += 1; // instruction
                self.l = self.e;
            }
            0x6c => {
                // MOV L,H
                self.pc += 1; // instruction
                self.l = self.h;
            }
            0x6d => {
                // MOV L,L
                self.pc += 1; // instruction
                self.l = self.l;
            }
            0x6e => {
                // MOV L,M
                self.pc += 1; // instruction
                self.l = self.memory[self.get_hl() as usize];
            }
            0x6f => {
                // MOV L,A
                self.pc += 1; // instruction
                self.l = self.a;
            }
            0x70 => {
                // MOV M,B
                self.pc += 1; // instruction
                self.memory[self.get_hl() as usize] = self.b;
            }
            0x71 => {
                // MOV M,C
                self.pc += 1; // instruction
                self.memory[self.get_hl() as usize] = self.c;
            }
            0x72 => {
                // MOV M,D
                self.pc += 1; // instruction
                self.memory[self.get_hl() as usize] = self.d;
            }
            0x73 => {
                // MOV M,E
                self.pc += 1; // instruction
                self.memory[self.get_hl() as usize] = self.e;
            }
            0x74 => {
                // MOV M,H
                self.pc += 1; // instruction
                self.memory[self.get_hl() as usize] = self.h;
            }
            0x75 => {
                // MOV M,L
                self.pc += 1; // instruction
                self.memory[self.get_hl() as usize] = self.l;
            }
            0x76 => {
                // HLT
                self.pc += 1; // instruction
                println!("Halting");
                self.enable = 0;
            }
            0x77 => { 
                // MOV M,A
                self.pc += 1; // instruction
                self.memory[self.get_hl() as usize] = self.a;
            }
            0x78 => { 
                // MOV A,B
                self.pc += 1; // instruction
                self.a = self.b;
            }                       
            0x79 => { 
                // MOV A,C
                self.pc += 1; // instruction
                self.a = self.c;
            }        
            0x7a => { 
                // MOV A,D
                self.pc += 1; // instruction
                self.a = self.d;
            }                          
            0x7b => { 
                // MOV A,E
                self.pc += 1; // instruction
                self.a = self.e;
            }                          
            0x7c => { 
                // MOV A,H
                self.pc += 1; // instruction
                self.a = self.h;
            }                          
            0x7d => { 
                // MOV A,L
                self.pc += 1; // instruction
                self.a = self.l;
            }                          
            0x7e => { 
                // MOV A,M
                self.pc += 1; // instruction
                self.a = self.memory[self.get_hl() as usize];
            }
            0x7f => { 
                // MOV A,A
                self.pc += 1; // instruction
                self.a = self.a;
            }                  
            0x80 => {
                // ADD B
                self.pc += 1; // instruction
                let answer: u16 = (self.a + self.b) as u16;
                self.update_condition_codes(answer, true, true, true, true, true);
                self.a = answer as u8;
            }
            0x81 => {
                // ADD C
                self.pc += 1; // instruction
                let answer: u16 = (self.a + self.c) as u16;
                self.update_condition_codes(answer, true, true, true, true, true);
                self.a = answer as u8;
            }
            0x82 => {
                // ADD D
                self.pc += 1; // instruction
                let answer: u16 = (self.a + self.d) as u16;
                self.update_condition_codes(answer, true, true, true, true, true);
                self.a = answer as u8;
            }
            0x83 => {
                // ADD E
                self.pc += 1; // instruction
                let answer: u16 = (self.a + self.e) as u16;
                self.update_condition_codes(answer, true, true, true, true, true);
                self.a = answer as u8;
            }
            0x84 => {
                // ADD H
                self.pc += 1; // instruction
                let answer: u16 = (self.a + self.h) as u16;
                self.update_condition_codes(answer, true, true, true, true, true);
                self.a = answer as u8;
            }
            0x85 => {
                // ADD L
                self.pc += 1; // instruction
                let answer: u16 = (self.a + self.l) as u16;
                self.update_condition_codes(answer, true, true, true, true, true);
                self.a = answer as u8;
            }
            0x86 => {
                // ADD M
                self.pc += 1; // instruction
                let answer: u16 = (self.a + self.memory[self.get_hl() as usize]) as u16;
                self.update_condition_codes(answer, true, true, true, true, true);
                self.a = answer as u8;
            }
            0x87 => {
                // ADD A
                self.pc += 1; // instruction
                let answer: u16 = (self.a + self.a) as u16;
                self.update_condition_codes(answer, true, true, true, true, true);
                self.a = answer as u8;
            }
            0x88 => {
                // ADC B
                self.pc += 1; // instruction
                let answer: u16 = (self.a + self.b + self.condition_codes.cy as u8) as u16;
                self.update_condition_codes(answer, true, true, true, true, true);
                self.a = answer as u8;
            }
            0x89 => {
                // ADC C
                self.pc += 1; // instruction
                let answer: u16 = (self.a + self.c + self.condition_codes.cy as u8) as u16;
                self.update_condition_codes(answer, true, true, true, true, true);
                self.a = answer as u8;
            }
            0x8a => {
                // ADC D
                self.pc += 1; // instruction
                let answer: u16 = (self.a + self.d + self.condition_codes.cy as u8) as u16;
                self.update_condition_codes(answer, true, true, true, true, true);
                self.a = answer as u8;
            }
            0x8b => {
                // ADC E
                self.pc += 1; // instruction
                let answer: u16 = (self.a + self.e + self.condition_codes.cy as u8) as u16;
                self.update_condition_codes(answer, true, true, true, true, true);
                self.a = answer as u8;
            }
            0x8c => {
                // ADC H
                self.pc += 1; // instruction
                let answer: u16 = (self.a + self.h + self.condition_codes.cy as u8) as u16;
                self.update_condition_codes(answer, true, true, true, true, true);
                self.a = answer as u8;
            }
            0x8d => {
                // ADC L
                self.pc += 1; // instruction
                let answer: u16 = (self.a + self.l + self.condition_codes.cy as u8) as u16;
                self.update_condition_codes(answer, true, true, true, true, true);
                self.a = answer as u8;
            }
            0x8e => {
                // ADC M
                self.pc += 1; // instruction
                let answer: u16 = (self.a
                    + self.memory[self.get_hl() as usize]
                    + self.condition_codes.cy as u8)
                    as u16;
                self.update_condition_codes(answer, true, true, true, true, true);
                self.a = answer as u8;
            }
            0x8f => {
                // ADC A
                self.pc += 1; // instruction
                let answer: u16 = (self.a + self.a + self.condition_codes.cy as u8) as u16;
                self.update_condition_codes(answer, true, true, true, true, true);
                self.a = answer as u8;
            }
            0x90 => {
                // SUB B
                self.pc += 1; // instruction
                let answer: u16 = (self.a - self.b) as u16;
                self.update_condition_codes(answer, true, true, true, true, true);
                self.a = answer as u8;
            }
            0x91 => {
                // SUB C
                self.pc += 1; // instruction
                let answer: u16 = (self.a - self.c) as u16;
                self.update_condition_codes(answer, true, true, true, true, true);
                self.a = answer as u8;
            }
            0x92 => {
                // SUB D
                self.pc += 1; // instruction
                let answer: u16 = (self.a - self.d) as u16;
                self.update_condition_codes(answer, true, true, true, true, true);
                self.a = answer as u8;
            }
            0x93 => {
                // SUB E
                self.pc += 1; // instruction
                let answer: u16 = (self.a - self.e) as u16;
                self.update_condition_codes(answer, true, true, true, true, true);
                self.a = answer as u8;
            }
            0x94 => {
                // SUB H
                self.pc += 1; // instruction
                let answer: u16 = (self.a - self.h) as u16;
                self.update_condition_codes(answer, true, true, true, true, true);
                self.a = answer as u8;
            }
            0x95 => {
                // SUB L
                self.pc += 1; // instruction
                let answer: u16 = (self.a - self.l) as u16;
                self.update_condition_codes(answer, true, true, true, true, true);
                self.a = answer as u8;
            }
            0x96 => {
                // SUB M
                self.pc += 1; // instruction
                let answer: u16 = (self.a - self.memory[self.get_hl() as usize]) as u16;
                self.update_condition_codes(answer, true, true, true, true, true);
                self.a = answer as u8;
            }
            0x97 => {
                // SUB A
                self.pc += 1; // instruction
                let answer: u16 = (self.a - self.a) as u16;
                self.update_condition_codes(answer, true, true, true, true, true);
                self.a = answer as u8;
            }
            0x98 => {
                // SBB B
                self.pc += 1; // instruction
                let answer: u16 = (self.a - self.b - self.condition_codes.cy as u8) as u16;
                self.update_condition_codes(answer, true, true, true, true, true);
                self.a = answer as u8;
            }
            0x99 => {
                // SBB C
                self.pc += 1; // instruction
                let answer: u16 = (self.a - self.c - self.condition_codes.cy as u8) as u16;
                self.update_condition_codes(answer, true, true, true, true, true);
                self.a = answer as u8;
            }
            0x9a => {
                // SBB D
                self.pc += 1; // instruction
                let answer: u16 = (self.a - self.d - self.condition_codes.cy as u8) as u16;
                self.update_condition_codes(answer, true, true, true, true, true);
                self.a = answer as u8;
            }
            0x9b => {
                // SBB E
                self.pc += 1; // instruction
                let answer: u16 = (self.a - self.e - self.condition_codes.cy as u8) as u16;
                self.update_condition_codes(answer, true, true, true, true, true);
                self.a = answer as u8;
            }
            0x9c => {
                self.pc += 1; // instruction
                // SBB H
                let answer: u16 = (self.a - self.h - self.condition_codes.cy as u8) as u16;
                self.update_condition_codes(answer, true, true, true, true, true);
                self.a = answer as u8;
            }
            0x9d => {
                self.pc += 1; // instruction
                // SBB L
                let answer: u16 = (self.a - self.l - self.condition_codes.cy as u8) as u16;
                self.update_condition_codes(answer, true, true, true, true, true);
                self.a = answer as u8;
            }
            0x9e => {
                // SUB M
                self.pc += 1; // instruction
                let answer: u16 = (self.a
                    - self.memory[self.get_hl() as usize]
                    - self.condition_codes.cy as u8)
                    as u16;
                self.update_condition_codes(answer, true, true, true, true, true);
                self.a = answer as u8;
            }
            0x9f => {
                // SBB A
                self.pc += 1; // instruction
                let answer: u16 = (self.a + self.a + self.condition_codes.cy as u8) as u16;
                self.update_condition_codes(answer, true, true, true, true, true);
                self.a = answer as u8;
            }
            0xa0 => {
                // ANA B
                self.pc += 1; // instruction
                let answer: u16 = (self.a & self.b) as u16;
                self.update_condition_codes(answer, true, true, true, true, true);
                self.a = answer as u8;
            }
            0xa1 => {
                // ANA C
                self.pc += 1; // instruction
                let answer: u16 = (self.a & self.c) as u16;
                self.update_condition_codes(answer, true, true, true, true, true);
                self.a = answer as u8;
            }
            0xa2 => {
                // ANA D
                self.pc += 1; // instruction
                let answer: u16 = (self.a & self.d) as u16;
                self.update_condition_codes(answer, true, true, true, true, true);
                self.a = answer as u8;
            }
            0xa3 => {
                // ANA E
                self.pc += 1; // instruction
                let answer: u16 = (self.a & self.e) as u16;
                self.update_condition_codes(answer, true, true, true, true, true);
                self.a = answer as u8;
            }
            0xa4 => {
                // ANA H
                self.pc += 1; // instruction
                let answer: u16 = (self.a & self.h) as u16;
                self.update_condition_codes(answer, true, true, true, true, true);
                self.a = answer as u8;
            }
            0xa5 => {
                // ANA L
                self.pc += 1; // instruction
                let answer: u16 = (self.a & self.l) as u16;
                self.update_condition_codes(answer, true, true, true, true, true);
                self.a = answer as u8;
            }
            0xa6 => {
                // ANA M
                self.pc += 1; // instruction
                let answer: u16 = (self.a & self.memory[self.get_hl() as usize]) as u16;
                self.update_condition_codes(answer, true, true, true, true, true);
                self.a = answer as u8;
            }
            0xa7 => {
                // ANA A
                self.pc += 1; // instruction
                let answer: u16 = (self.a & self.a) as u16;
                self.update_condition_codes(answer, true, true, true, true, true);
                self.a = answer as u8;
            }
            0xa8 => {
                // XRA B
                self.pc += 1; // instruction
                let answer: u16 = (self.a ^ self.b) as u16;
                self.update_condition_codes(answer, true, true, true, true, true);
                self.a = answer as u8;
            }
            0xa9 => {
                // XRA C
                self.pc += 1; // instruction
                let answer: u16 = (self.a ^ self.c) as u16;
                self.update_condition_codes(answer, true, true, true, true, true);
                self.a = answer as u8;
            }
            0xaa => {
                // XRA D
                self.pc += 1; // instruction
                let answer: u16 = (self.a ^ self.d) as u16;
                self.update_condition_codes(answer, true, true, true, true, true);
                self.a = answer as u8;
            }
            0xab => {
                // XRA E
                self.pc += 1; // instruction
                let answer: u16 = (self.a ^ self.e) as u16;
                self.update_condition_codes(answer, true, true, true, true, true);
                self.a = answer as u8;
            }
            0xac => {
                // XRA H
                self.pc += 1; // instruction
                let answer: u16 = (self.a ^ self.h) as u16;
                self.update_condition_codes(answer, true, true, true, true, true);
                self.a = answer as u8;
            }
            0xad => {
                // XRA L
                self.pc += 1; // instruction
                let answer: u16 = (self.a ^ self.l) as u16;
                self.update_condition_codes(answer, true, true, true, true, true);
                self.a = answer as u8;
            }
            0xae => {
                // XRA M
                self.pc += 1; // instruction
                let answer: u16 = (self.a ^ self.memory[self.get_hl() as usize]) as u16;
                self.update_condition_codes(answer, true, true, true, true, true);
                self.a = answer as u8;
            }
            0xaf => {
                // ORA A
                self.pc += 1; // instruction
                let answer: u16 = (self.a | self.a) as u16;
                self.update_condition_codes(answer, true, true, true, true, true);
                self.a = answer as u8;
            }
            0xb0 => {
                // ORA B
                self.pc += 1; // instruction
                let answer: u16 = (self.a | self.b) as u16;
                self.update_condition_codes(answer, true, true, true, true, true);
                self.a = answer as u8;
            }
            0xb1 => {
                // ORA C
                self.pc += 1; // instruction
                let answer: u16 = (self.a | self.c) as u16;
                self.update_condition_codes(answer, true, true, true, true, true);
                self.a = answer as u8;
            }
            0xb2 => {
                // ORA D
                self.pc += 1; // instruction
                let answer: u16 = (self.a | self.d) as u16;
                self.update_condition_codes(answer, true, true, true, true, true);
                self.a = answer as u8;
            }
            0xb3 => {
                // ORA E
                self.pc += 1; // instruction
                let answer: u16 = (self.a | self.e) as u16;
                self.update_condition_codes(answer, true, true, true, true, true);
                self.a = answer as u8;
            }
            0xb4 => {
                // ORA H
                self.pc += 1; // instruction
                let answer: u16 = (self.a | self.h) as u16;
                self.update_condition_codes(answer, true, true, true, true, true);
                self.a = answer as u8;
            }
            0xb5 => {
                // ORA L
                self.pc += 1; // instruction
                let answer: u16 = (self.a | self.l) as u16;
                self.update_condition_codes(answer, true, true, true, true, true);
                self.a = answer as u8;
            }
            0xb6 => {
                // ORA M
                self.pc += 1; // instruction
                let answer: u16 = (self.a | self.memory[self.get_hl() as usize]) as u16;
                self.update_condition_codes(answer, true, true, true, true, true);
                self.a = answer as u8;
            }
            0xb7 => {
                // ORA A
                self.pc += 1; // instruction
                let answer: u16 = (self.a ^ self.a) as u16;
                self.update_condition_codes(answer, true, true, true, true, true);
                self.a = answer as u8;
            }

            0xc0 => {
                // RNZ
                self.pc += 1; // instruction
                if !self.condition_codes.z {
                    self.ret();
                }
            }
            0xc1 => {
                // POP B
                self.pc += 1; // instruction
                self.c = self.memory[self.sp as usize];
                self.b = self.memory[(self.sp + 1) as usize];
                self.sp += 2;
            }
            0xc2 => {
                // JNZ adr
                self.pc += 1; // instruction
                let adr = (self.memory[(self.pc+1) as usize] as u16) << 8 | self.memory[self.pc as usize] as u16;
                self.pc += 2;
                if !self.condition_codes.z {
                    self.pc = adr;
                }
            }
            0xc3 => {
                // JMP adr
                self.pc += 1; // instruction
                let adr = (self.memory[(self.pc+1) as usize] as u16) << 8 | self.memory[self.pc as usize] as u16;
                self.pc += 2;
                self.pc = adr;
            }
            0xc4 => {
                // CNZ adr
                self.pc += 1; // instruction
                let adr = (self.memory[(self.pc+1) as usize] as u16) << 8 | self.memory[self.pc as usize] as u16;
                self.pc += 2;
                if !self.condition_codes.z {
                    self.call(adr);
                }
            }
            0xc5 => {
                // PUSH B
                self.pc += 1; // instruction
                self.memory[(self.sp - 1) as usize] = self.b;
                self.memory[(self.sp - 2) as usize] = self.c;
                self.sp -= 2;
            }
            0xc6 => {
                // ADI D8
                self.pc += 1; // instruction
                let answer: u16 = self.a as u16 + self.memory[self.pc as usize] as u16;
                self.pc += 1;
                self.update_condition_codes(answer, true, true, true, true, true);
                self.a = answer as u8;
            }
            0xc7 => {
                // RST 0
                // CALL $0
                self.pc += 1; // instruction
            }
            0xc8 => {
                // RZ
                self.pc += 1; // instruction
                if self.condition_codes.z {
                    self.ret();
                }
            }
            0xc9 => {
                self.ret();
            }
            0xca => {
                // JZ adr
                self.pc += 1; // instruction
                let adr = (self.memory[(self.pc+1) as usize] as u16) << 8 | self.memory[self.pc as usize] as u16;
                self.pc += 2;
                if self.condition_codes.z {
                    self.pc = adr;
                }
            }
            0xcb => {
                // -
                self.pc += 1; // instruction
            }
            0xcc => {
                // CZ adr
                self.pc += 1; // instruction
                let adr = (self.memory[(self.pc+1) as usize] as u16) << 8 | self.memory[self.pc as usize] as u16;
                self.pc += 2;
                if self.condition_codes.z {
                    self.call(adr);
                }
            }
            0xcd => {
                // CALL adr
                self.pc += 1; // instruction
                let adr = (self.memory[(self.pc+1) as usize] as u16) << 8 | self.memory[self.pc as usize] as u16;
                self.pc += 2;
                self.call(adr);
            }
            0xce => {
                // ACI D8
                self.pc += 1; // instruction
                let answer: u16 =
                    self.a as u16 + self.memory[self.pc as usize] as u16 + self.condition_codes.cy as u16;
                self.pc += 1;
                self.update_condition_codes(answer, true, true, true, true, true);
                self.a = answer as u8;
            }
            0xcf => {
                // RST 1
                // CALL $8
                self.pc += 1; // instruction
            }
            0xd0 => {
                self.pc += 1; // instruction
                if !self.condition_codes.cy {
                    self.ret();
                }
            }
            0xd1 => {
                // POP D
                self.pc += 1; // instruction
                self.e = self.memory[self.sp as usize];
                self.d = self.memory[(self.sp + 1) as usize];
                self.sp += 2;
            }
            0xd2 => {
                // JNC adr
                self.pc += 1; // instruction
                let adr = (self.memory[(self.pc+1) as usize] as u16) << 8 | self.memory[self.pc as usize] as u16;
                self.pc += 2;
                if !self.condition_codes.cy {
                    self.pc = adr;
                }
            }
            0xd3 => {
                // OUT D8
                self.pc += 1; // instruction
                self.pc += 1;
            }
            0xd4 => {
                // CNC adr
                self.pc += 1; // instruction
                let adr = (self.memory[(self.pc+1) as usize] as u16) << 8 | self.memory[self.pc as usize] as u16;
                self.pc += 2;
                if !self.condition_codes.cy {
                    self.call(adr);
                }
            }
            0xd5 => {
                // PUSH D
                self.pc += 1; // instruction
                self.memory[(self.sp - 1) as usize] = self.d;
                self.memory[(self.sp - 2) as usize] = self.e;
                self.sp -= 2;
            }
            0xd6 => {
                // SUI D8
                self.pc += 1; // instruction
                let answer: u16 = self.a as u16 - self.memory[self.pc as usize] as u16;
                self.pc += 1;
                self.update_condition_codes(answer, true, true, true, true, true);
                self.a = answer as u8;
            }
            0xd7 => {
                // RST 2
                // CALL $10
                self.pc += 1; // instruction
            }
            0xd8 => {
                // RC
                self.pc += 1; // instruction
                if self.condition_codes.cy {
                    self.ret();
                }
            }
            0xd9 => {
                // -
                self.pc += 1; // instruction
            }
            0xda => {
                // JC adr
                self.pc += 1; // instruction
                let adr = (self.memory[(self.pc+1) as usize] as u16) << 8 | self.memory[self.pc as usize] as u16;
                self.pc += 2;
                if self.condition_codes.cy {
                    self.pc = adr;
                }
            }
            0xdb => {
                // IN D8
                self.pc += 1; // instruction
                self.pc += 1;
            }
            0xdc => {
                // CC adr
                self.pc += 1; // instruction
                let adr = (self.memory[(self.pc+1) as usize] as u16) << 8 | self.memory[self.pc as usize] as u16;
                self.pc += 2;
                if self.condition_codes.cy {
                    self.call(adr);
                }
            }
            0xdd => {
                // -
                self.pc += 1; // instruction
            }
            0xde => {
                // SBI D8
                self.pc += 1; // instruction
                let answer: u16 =
                    self.a as u16 - self.memory[self.pc as usize] as u16 - self.condition_codes.cy as u16;
                self.pc += 1;
                self.update_condition_codes(answer, true, true, true, true, true);
                self.a = answer as u8;
            }
            0xdf => {
                // RST 3
                // CALL $18
                self.pc += 1; // instruction
            }
            0xe0 => {
                // RPO
                self.pc += 1; // instruction
                if !self.condition_codes.p {
                    self.ret();
                }
            }
            0xe1 => {
                // POP H
                self.pc += 1; // instruction
                self.l = self.memory[self.sp as usize];
                self.h = self.memory[(self.sp + 1) as usize];
                self.sp += 2;
            }
            0xe2 => {
                // JPO adr
                self.pc += 1; // instruction
                let adr = (self.memory[(self.pc+1) as usize] as u16) << 8 | self.memory[self.pc as usize] as u16;
                self.pc += 2;
                if !self.condition_codes.p {
                    self.pc = adr;
                }
            }
            0xe3 => {
                // XTHL
                self.pc += 1; // instruction
                let temp = self.l;
                self.l = self.memory[self.sp as usize];
                self.memory[self.sp as usize] = temp;
                let temp = self.h;
                self.h = self.memory[(self.sp+1) as usize];
                self.memory[(self.sp+1) as usize] = temp;
            }
            0xe4 => {
                // CPO adr
                self.pc += 1; // instruction
                let adr = (self.memory[(self.pc+1) as usize] as u16) << 8 | self.memory[self.pc as usize] as u16;
                self.pc += 2;
                if !self.condition_codes.p {
                    self.call(adr);
                }
            }
            0xe5 => {
                // PUSH H
                self.pc += 1; // instruction
                self.memory[(self.sp - 1) as usize] = self.h;
                self.memory[(self.sp - 2) as usize] = self.l;
                self.sp -= 2;
            }
            0xe6 => {
                // ANI D8
                self.pc += 1; // instruction
                let answer: u8 = self.a & self.memory[self.pc as usize];
                self.pc += 1;
                self.update_condition_codes(answer as u16, true, true, true, true, true);
                self.a = answer;
            }
            0xe7 => {
                // RST 4
                // CALL $20
                self.pc += 1; // instruction
            }
            0xe8 => {
                // RPE
                self.pc += 1; // instruction
                if self.condition_codes.p {
                    self.ret();
                }
            }
            0xe9 => {
                // PCHL
                self.pc += 1; // instruction
                self.pc = ((self.h as u16) << 8) | self.l as u16;
            }
            0xea => {
                // JPE adr
                self.pc += 1; // instruction
                let adr = (self.memory[(self.pc+1) as usize] as u16) << 8 | self.memory[self.pc as usize] as u16;
                self.pc += 2;
                if self.condition_codes.p {
                    self.pc = adr;
                }
            }
            0xeb => {
                // XCHG
                self.pc += 1; // instruction
                let temp = self.d;
                self.d = self.h;
                self.h = temp;
                let temp = self.e;
                self.e = self.l;
                self.l = temp;
            }
            0xec => {
                // CPE adr
                self.pc += 1; // instruction
                let adr = (self.memory[(self.pc+1) as usize] as u16) << 8 | self.memory[self.pc as usize] as u16;
                self.pc += 2;
                if self.condition_codes.p {
                    self.call(adr);
                }
            }
            0xed => {
                // -
                self.pc += 1; // instruction
            }
            0xee => {
                // XRI D8
                self.pc += 1; // instruction
                let answer: u8 = self.a ^ self.memory[self.pc as usize];
                self.pc += 1;
                self.update_condition_codes(answer as u16, true, true, true, true, true);
                self.a = answer;
            }

            0xf1 => {
                // POP PSW
                self.pc += 1; // instruction
                self.a = self.memory[(self.sp+1) as usize];  
                let psw: u8 = self.memory[self.sp as usize];    
                self.condition_codes.z  = 0x01 == (psw & 0x01);  
                self.condition_codes.s  = 0x02 == (psw & 0x02); 
                self.condition_codes.p  = 0x04 == (psw & 0x04);
                self.condition_codes.cy = 0x05 == (psw & 0x08);
                self.condition_codes.ac = 0x10 == (psw & 0x10);
                self.sp += 2;
            }

            0xf3 => {
                // DI
                self.pc += 1; // instruction
            }

            0xf5 => {
                // PUSH PSW
                self.pc += 1; // instruction
                let psw = 
                    self.condition_codes.z as u8 |    
                    (self.condition_codes.s as u8) << 1 |    
                    (self.condition_codes.p as u8) << 2 |    
                    (self.condition_codes.cy as u8) << 3 |    
                    (self.condition_codes.ac as u8) << 4; 
                self.memory[(self.sp-2) as usize] = psw;
                self.memory[(self.sp-1) as usize] = self.a;
                self.sp -= 2;
            }
            0xf6 => {
                // ORI D8
                self.pc += 1; // instruction
                let answer: u8 = self.a | self.memory[self.pc as usize];
                self.pc += 1;
                self.update_condition_codes(answer as u16, true, true, true, true, true);
                self.a = answer;
            }
            0xf7 => {
                // RES 6
                // CALL $30
                self.pc += 1; // instruction
            }

            0xf9 => {
                // SPHL
                self.pc += 1; // instruction
                self.sp = ((self.h as u16) << 8) | self.l as u16;
            }

            0xfb => {
                // EL
                self.pc += 1; // instruction
            }

            0xfd => {
                // -
                self.pc += 1; // instruction
            }
            0xfe => {
                // CPI D8
                self.pc += 1; // instruction
                let answer: u8 = self.a - self.memory[self.pc as usize];
                self.pc += 1;
                self.update_condition_codes(answer as u16, true, true, true, true, true);
            }
            0xff => {
                // RST 7
                // CALL $38
                self.pc += 1; // instruction
            }
            _ => self.unimplimented(),
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

    fn call(&mut self, adr: u16) {
        self.memory[(self.sp - 1) as usize] = (self.pc >> 8) as u8;
        self.memory[(self.sp - 2) as usize] = self.pc as u8;
        self.sp -= 2;
        self.pc = adr;
    }

    fn ret(&mut self) {
        self.pc =
            (self.memory[(self.sp+1) as usize] as u16) << 8 | self.memory[self.sp as usize] as u16;
        self.sp += 2;
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
            /*
            Parity IS determined by counting the number of one bits set in the result in the accumulator.
            Instructions that affect the parity flag set the flag to one for even parity and reset the
            flag to zero to indicate odd parity.
            */
            codes.p = value.count_ones() % 2 == 0;
        }
        if cy {
            codes.cy = value > 0xff;
        }
        if ac {
            codes.ac = false;
        }
    }

    #[allow(dead_code)]
    pub fn print_registers(&self) {
        println!(
            "a={}\nb={}\nc={}\nd={}\ne={}\nh={:02X}\nl={:02X}\nsp={:02X}\npc={:02X}\n{:?}",
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

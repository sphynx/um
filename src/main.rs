use byteorder::BigEndian;
use byteorder::ReadBytesExt;
use byteorder::WriteBytesExt;
use std::env;
use std::fs;
use std::io;
use std::io::Write;
use um::mem::Mem;

struct Machine {
    reg: [u32; 8],
    mem: Mem,
    ip: u32,
}

impl Machine {
    fn load(bytes: &[u8]) -> Self {
        // Here we convert from bytes (u8) to our VM words (u32).
        assert_eq!(
            bytes.len() % 4,
            0,
            "Program must have whole number of u32's"
        );

        let size = bytes.len() / 4;
        let mut reader = io::Cursor::new(bytes);
        let mut program = Vec::with_capacity(size);

        for _ in 0..size {
            let word32 = reader.read_u32::<BigEndian>().unwrap();
            program.push(word32);
        }

        Machine {
            reg: [0; 8],
            mem: Mem::init(program),
            ip: 0,
        }
    }

    fn run(&mut self) {
        let mut input = String::new();
        loop {
            let word = self.mem.read(0, self.ip);
            let op = Op::parse(*word);

            match op {
                Op::CondMov(a, b, c) => {
                    if self.reg[c] != 0 {
                        self.reg[a] = self.reg[b]
                    }
                }

                Op::MemRead(a, b, c) => {
                    self.reg[a] = *self.mem.read(self.reg[b], self.reg[c]);
                }

                Op::MemWrite(a, b, c) => {
                    self.mem.write(self.reg[a], self.reg[b], self.reg[c]);
                }

                Op::Add(a, b, c) => {
                    self.reg[a] = self.reg[b].wrapping_add(self.reg[c]);
                }

                Op::Mul(a, b, c) => {
                    self.reg[a] = self.reg[b].wrapping_mul(self.reg[c]);
                }

                Op::Div(a, b, c) => {
                    if self.reg[c] == 0 {
                        panic!("vm: division by zero!");
                    }
                    self.reg[a] = self.reg[b].wrapping_div(self.reg[c]);
                }

                Op::Nand(a, b, c) => {
                    self.reg[a] = !(self.reg[b] & self.reg[c]);
                }

                Op::Halt => {
                    break;
                }

                Op::Alloc(b, c) => {
                    self.reg[b] = self.mem.alloc(self.reg[c]);
                }

                Op::Free(c) => {
                    self.mem.free(self.reg[c]);
                }

                Op::Output(c) => {
                    let chr = self.reg[c];

                    if chr > 255 {
                        panic!("vm: character for output > 255: {}", chr);
                    }

                    io::stdout()
                        .write_u8(chr as u8)
                        .and_then(|()| io::stdout().flush())
                        .expect("vm: writing character failed");
                }

                Op::Input(c) => {
                    if input.len() == 0 {
                        // Read a new line of input.
                        io::stdin().read_line(&mut input).unwrap();
                    }

                    if input.len() == 0 {
                        // If it's still empty, this means we've got EOF,
                        // treat it as terminate.
                        println!("");
                        break;
                    } else {
                        self.reg[c] = input.remove(0) as u32;
                    }
                }

                Op::LoadProgram(b, c) => {
                    self.mem.copy_to_zero(self.reg[b]);
                    self.ip = self.reg[c];
                    continue; // to skip 'ip += 1'
                }

                Op::Mov(a, val) => {
                    self.reg[a] = val;
                }
            }

            self.ip += 1;
        }
    }
}

type Reg = usize;

#[derive(Debug)]
enum Op {
    CondMov(Reg, Reg, Reg),
    MemRead(Reg, Reg, Reg),
    MemWrite(Reg, Reg, Reg),
    Add(Reg, Reg, Reg),
    Mul(Reg, Reg, Reg),
    Div(Reg, Reg, Reg),
    Nand(Reg, Reg, Reg),
    Halt,
    Alloc(Reg, Reg),
    Free(Reg),
    Output(Reg),
    Input(Reg),
    LoadProgram(Reg, Reg),
    Mov(Reg, u32),
}

impl Op {
    fn parse(v: u32) -> Op {
        let code = v >> 28;
        let a = ((v & 0b111000000_u32) >> 6) as usize;
        let b = ((v & 0b111000_u32) >> 3) as usize;
        let c = (v & 0b111_u32) as usize;
        match code {
            0 => Op::CondMov(a, b, c),
            1 => Op::MemRead(a, b, c),
            2 => Op::MemWrite(a, b, c),
            3 => Op::Add(a, b, c),
            4 => Op::Mul(a, b, c),
            5 => Op::Div(a, b, c),
            6 => Op::Nand(a, b, c),
            7 => Op::Halt,
            8 => Op::Alloc(b, c),
            9 => Op::Free(c),
            10 => Op::Output(c),
            11 => Op::Input(c),
            12 => Op::LoadProgram(b, c),
            13 => {
                let a = ((v >> 25) & 0b111_u32) as usize;
                let val = v & 0x01FFFFFF_u32;
                Op::Mov(a, val)
            }
            _ => panic!("vm: unexpected op code: {}", code),
        }
    }
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let prog = fs::read(&args[1])?;
    let mut um = Machine::load(&prog);
    um.run();
    Ok(())
}

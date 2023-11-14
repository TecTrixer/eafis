use std::hint::unreachable_unchecked;

fn main() {
    println!("Hello, world!");
}

macro_rules! opc {
    ($($num:expr => $op:ident),*) => {
        #[repr(u8)]
        enum OpCode {
            $($op = $num,)*
            NOIMPL = 0xff,
        }

        impl From<u8> for OpCode {
            fn from(value: u8) -> Self {
                match value {
                    $($num => OpCode::$op,)*
                    _ => OpCode::NOIMPL,
                }
            }
        }
    }
}

opc!(
     0x00 => HLT,
     0x01 => NOP,
     0x02 => SYS,
     0x03 => RET,
     0x10 => JMP,
     0x11 => JEQ,
     0x12 => JNE,
     0x13 => JLT,
     0x14 => JLE,
     0x15 => JGT,
     0x16 => JGE,
     0x20 => CALL,
     0x21 => PUSH,
     0x22 => POP,
     0x23 => INC,
     0x24 => DEC,
     0x30 => CMP,
     0x31 => ST,
     0x32 => LD,
     0x40 => NOT,
     0x41 => XOR,
     0x42 => AND,
     0x43 => OR,
     0x44 => ADD,
     0x45 => SUB,
     0x46 => MUL,
     0x47 => DIV
);

struct Processor {
    a: u32,
    b: u32,
    c: u32,
    d: u32,
    r: u32,
    o: u32,
    sp: u32,
    ip: u32,
    mem: Memory,
}

impl Processor {
    fn new() -> Self {
        let mem = Memory::new();
        Self {
            mem,
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            r: 0,
            o: 0,
            sp: 0,
            ip: 0,
        }
    }
    fn load(code: &Vec<u8>) -> Self {
        let mem = Memory::load(code);
        Self {
            mem,
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            r: 0,
            o: 0,
            sp: 0,
            ip: 0,
        }
    }
}

struct Memory {
    mem: Vec<u8>,
}

const MEMBITS: usize = 24;
const MEMSIZE: usize = (1 << MEMBITS) + 5;

impl Memory {
    fn new() -> Self {
        let mem = vec![0; MEMSIZE];
        Self { mem }
    }
    fn load(code: &Vec<u8>) -> Self {
        assert!(code.len() < MEMSIZE);
        let mut mem = vec![0; MEMSIZE];
        for i in 0..code.len() {
            mem[i] = code[i];
        }
        Self { mem }
    }
    fn get_ptr(idx: u32) -> usize {
        (idx & ((1 << MEMBITS) - 1)) as usize
    }
    fn read_byte(&self, idx: u32) -> u8 {
        let ptr = Self::get_ptr(idx);
        unsafe { *self.mem.get_unchecked(ptr) }
    }
    fn read_u32(&self, idx: u32) -> u32 {
        let ptr = Self::get_ptr(idx);
        let b0 = unsafe { *self.mem.get_unchecked(ptr) };
        let b1 = unsafe { *self.mem.get_unchecked(ptr + 1) };
        let b2 = unsafe { *self.mem.get_unchecked(ptr + 2) };
        let b3 = unsafe { *self.mem.get_unchecked(ptr + 3) };
        u32::from_le_bytes([b0, b1, b2, b3])
    }
    fn write_u32(&mut self, idx: u32, val: u32) {
        let ptr = Self::get_ptr(idx);
        let [b0, b1, b2, b3] = val.to_le_bytes();
        unsafe {
            *self.mem.get_unchecked_mut(ptr) = b0;
            *self.mem.get_unchecked_mut(ptr + 1) = b1;
            *self.mem.get_unchecked_mut(ptr + 2) = b2;
            *self.mem.get_unchecked_mut(ptr + 3) = b3;
        }
    }
}
trait Argument {
    fn write(&self, cpu: &mut Processor, val: u32);
    fn read(&self, cpu: &Processor) -> u32;
}

struct Constant {
    val: u32,
}

impl Constant {
    fn new(val: u32) -> Self {
        Constant { val }
    }
}

impl Argument for Constant {
    fn write(&self, cpu: &mut Processor, _val: u32) {
        println!("Attemted to write to constant at instruction {:x}", cpu.ip);
    }

    fn read(&self, _cpu: &Processor) -> u32 {
        self.val
    }
}

struct DirectAddress {
    addr: u32,
}

impl DirectAddress {
    fn new(addr: u32) -> Self {
        Self { addr }
    }
}

impl Argument for DirectAddress {
    fn write(&self, cpu: &mut Processor, val: u32) {
        let ptr = cpu.o + self.addr;
        cpu.mem.write_u32(ptr, val)
    }

    fn read(&self, cpu: &Processor) -> u32 {
        let ptr = cpu.o + self.addr;
        cpu.mem.read_u32(ptr)
    }
}

struct RegisterAddress {
    reg: Register,
}

impl RegisterAddress {
    fn new(reg: Register) -> Self {
        Self { reg }
    }
}

impl Argument for RegisterAddress {
    fn write(&self, cpu: &mut Processor, val: u32) {
        let ptr = cpu.o + self.reg.read(cpu);
        cpu.mem.write_u32(ptr, val)
    }

    fn read(&self, cpu: &Processor) -> u32 {
        let ptr = cpu.o + self.reg.read(cpu);
        cpu.mem.read_u32(ptr)
    }
}

#[repr(u8)]
enum Register {
    A = 0x00,
    B = 0x01,
    C = 0x02,
    D = 0x03,
    R = 0x04,
    O = 0x05,
    SP = 0x06,
    IP = 0x07,
}

impl Argument for Register {
    fn write(&self, cpu: &mut Processor, val: u32) {
        match &self {
            Register::A => cpu.a = val,
            Register::B => cpu.b = val,
            Register::C => cpu.c = val,
            Register::D => cpu.d = val,
            Register::R => cpu.r = val,
            Register::O => cpu.o = val,
            Register::SP => cpu.sp = val,
            Register::IP => cpu.ip = val,
        }
    }

    fn read(&self, cpu: &Processor) -> u32 {
        match &self {
            Register::A => cpu.a,
            Register::B => cpu.b,
            Register::C => cpu.c,
            Register::D => cpu.d,
            Register::R => cpu.r,
            Register::O => cpu.o,
            Register::SP => cpu.sp,
            Register::IP => cpu.ip,
        }
    }
}

impl From<u8> for Register {
    fn from(value: u8) -> Self {
        match value & 0x07 {
            0x00 => Register::A,
            0x01 => Register::B,
            0x02 => Register::C,
            0x03 => Register::D,
            0x04 => Register::R,
            0x05 => Register::O,
            0x06 => Register::SP,
            0x07 => Register::IP,
            _ => unsafe { unreachable_unchecked() },
        }
    }
}

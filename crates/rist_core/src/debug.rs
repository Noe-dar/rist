use std::{
    fmt::{self, Display, Write},
    io,
};

use itertools::Itertools;
use owo_colors::{AnsiColors, OwoColorize};

use crate::registers::XRegisters;

const ABI_NAMES: [&'static str; 32] = [
    "zero", "ra", "sp", "gp", "tp", "t0", "t1", "t2", "s0", "s1", "a0", "a1", "a2", "a3", "a4",
    "a5", "a6", "a7", "s2", "s3", "s4", "s5", "s6", "s7", "s8", "s9", "s10", "s11", "t3", "t4",
    "t5", "t6",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operand {
    Register(usize, u64),
    Immediate(u64),
    SignedImmediate(i64),
    None,
}

impl Operand {
    pub fn is_none(&self) -> bool {
        matches!(self, Operand::None)
    }
}

impl Display for Operand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Operand::Register(register, value) => {
                write!(f, "{}({value})", ABI_NAMES[*register])
            }
            Operand::Immediate(imm) => write!(f, "{imm}"),
            Operand::SignedImmediate(imm) => write!(f, "{imm}"),
            Operand::None => Ok(()),
        }
    }
}

pub trait Debugger {
    fn debug(&self, name: &'static str, operands: [Operand; 3], pc: usize);
    fn dump(&self, pc: u64, machine: &XRegisters) -> Result<(), io::Error>;
}

pub struct BlankDebugger;

impl Debugger for BlankDebugger {
    fn debug(&self, _: &'static str, _: [Operand; 3], _: usize) {}
    fn dump(&self, _: u64, _: &XRegisters) -> Result<(), io::Error> {
        Ok(())
    }
}

pub struct DefaultDebugger;

impl Debugger for DefaultDebugger {
    fn debug(&self, name: &'static str, operands: [Operand; 3], pc: usize) {
        const OPERAND_COLORS: [AnsiColors; 3] =
            [AnsiColors::Green, AnsiColors::Yellow, AnsiColors::Blue];

        let operands = operands
            .iter()
            .filter(|opernad| !opernad.is_none())
            .enumerate()
            .map(|(index, operand)| operand.color(OPERAND_COLORS[index]))
            .format(", ");

        println!("{}: {} {operands}", pc.bright_black(), name.red());
    }

    fn dump(&self, pc: u64, xregisters: &XRegisters) -> Result<(), io::Error> {
        println!("{:=^1$}", " DUMP ".bright_black(), 56);
        println!("{}: {}", "pc".red(), pc);
        println!();

        const MAX1: usize = 8;
        const MAX2: usize = 16;

        for mut chunk in &xregisters.iter().enumerate().chunks(2) {
            let (register1, value1) = chunk.next().unwrap();
            let (register2, value2) = chunk.next().unwrap();

            for (register, color, value) in
                [(register1, AnsiColors::Red, value1), (register2, AnsiColors::Blue, value2)]
            {
                let mut temp = String::default();

                write!(&mut temp, "{}:", ABI_NAMES[register]).unwrap();
                print!("{: <0MAX1$} ", temp.color(color));

                temp.clear();
                write!(&mut temp, "{value:#x}({value})").unwrap();
                print!("{: <0MAX2$} ", temp.black());
            }
            println!();
        }

        Ok(())
    }
}

#[macro_export]
macro_rules! debug {
    ($machine: ident: $name: ident $($type: ident($value: expr)),*) => {
        let mut operands = [$crate::debug::Operand::None; 3];

        for (index, operand) in [$(debug!(VALUE: $machine, $type($value))),*].iter().enumerate() {
            operands[index] = *operand;
        }

        if let Some(debugger) = $machine.debugger {
            debugger.debug(stringify!($name), operands, $machine.pc as usize)
        }

    };

    (VALUE: $machine: ident, reg($value: expr)) => {
        $crate::debug::Operand::Register($value, $machine.xregisters[$value])
    };

    (VALUE: $machine: ident, simm($value: expr)) => {
        $crate::debug::Operand::SignedImmediate($value)
    };

    (VALUE: $machine: ident, imm($value: expr)) => {
        $crate::debug::Operand::Immediate($value)
    };
}

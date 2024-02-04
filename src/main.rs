use instructions::{
    addi::ADDI, andi::ANDI, lb::LB, lbu::LBU, lh::LH, lhu::LHU, lui::LUI, lw::LW, ori::ORI, slli::SLLI, slti::SLTI, srli::SRLI, xori::XORI, ADD, SLTIU, SUB
};
use machine::Machine;

use crate::exception::Exception;

pub mod bus;
pub mod decoder;
pub mod exception;
pub mod instructions;
pub mod machine;
pub mod memory;
pub mod registers;

const ROM: &[u8] = include_bytes!("../test.bin");

fn execute(machine: &mut Machine) -> Result<(), Exception> {
    let word = machine.fetch();
    let instruction = machine.decode(word)?;

    (instruction.execute)(machine, word);

    Ok(())
}

fn main() {
    let mut machine = Machine::new(&[
        ADDI, SLTI, SLTIU, XORI, ORI, ANDI, LUI, SLLI, SRLI, LB, LBU, LH, LHU, LW, ADD, SUB
    ]);
    machine.flash(ROM);

    loop {
        if let Err(exception) = execute(&mut machine) {
            println!("! EXCEPTION !: {:?}", exception);
            break;
        } else {
            println!("{}", machine.pc);
        }
    }

    machine.xregisters.dump()
}

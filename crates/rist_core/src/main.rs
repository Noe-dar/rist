use std::time::Instant;

use constcat::concat_slices;
use instructions::{
    branch::BRANCH_INSTRUCTIONS, jal::JAL, lui::LUI, Instruction, IMM_INSTRUCTIONS,
};
use machine::Machine;


use crate::{
    debug::BlankDebugger, exception::Exception, instructions::{
        auipc::AUIPC, ecall::ECALL, jalr::JALR, store::sd::SD, IMM_32_INSTRUCTIONS, LOAD_INSTRUCTIONS, OP_INSTRUCTIONS
    }
};

pub mod bus;
pub mod debug;
pub mod decoder;
pub mod exception;
pub mod instructions;
pub mod machine;
pub mod memory;
pub mod registers;

const ISA: &[Instruction] = concat_slices!(
    [Instruction::EMPTY; Instruction]:
    IMM_INSTRUCTIONS, IMM_32_INSTRUCTIONS, OP_INSTRUCTIONS, BRANCH_INSTRUCTIONS, LOAD_INSTRUCTIONS,
    &[JAL, JALR, LUI, AUIPC, SD, ECALL]
);

const ROM: &[u8] = include_bytes!("../../../test.bin");

fn execute(machine: &mut Machine) -> Result<(), Exception> {
    let word = machine.fetch();
    let instruction = machine.decode(word)?;

    machine.pc += 4;
    (instruction.execute)(machine, word);

    Ok(())
}

fn main() {
    let mut machine = Machine::new(ISA);
    machine.attach_debugger(&BlankDebugger);
    machine.flash(ROM);

    machine.xregisters[2] = 0x9000;

    let start = Instant::now();
    let mut executed = 0;
    loop {
        if let Err(exception) = execute(&mut machine) {
            println!("! EXCEPTION !: {:?}", exception);
            break;
        } else {
            executed += 1;
        }
    }

    println!("executed {} instructions in {:?}", executed, start.elapsed());

    machine.dump();
}

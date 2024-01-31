use machine::Machine;
use rvemu::{bus::DRAM_BASE, emulator::Emulator};

pub mod machine;
pub mod bus;
pub mod memory;
pub mod registers;

const ROM: &[u8] = include_bytes!("../test.bin");

fn main() {
    let mut machine = Machine::new();
    let mut emulator = Emulator::new();
    emulator.initialize_dram(ROM.into());
    emulator.initialize_pc(DRAM_BASE);

    machine.flash(ROM);

    for _ in 0..(ROM.len() / 4) {
        let instruction = machine.fetch();
        machine.increment_pc();

        machine.execute(instruction);
    }

    emulator.start();

    println!("rist {}", machine.xregisters[5] as i64);
    println!("{}", emulator.cpu.xregs.read(5));

    
}
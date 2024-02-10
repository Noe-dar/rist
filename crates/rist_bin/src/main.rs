use std::{fs, path::PathBuf};

use clap::Parser;
use rist_core::{debug::DefaultDebugger, Machine, RV64_I};

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Arguments {
    /// Path to the ROM file
    #[arg(short, long)]
    rom: PathBuf
}

fn main() -> anyhow::Result<()> {
    let arguments = Arguments::parse();
    let rom = fs::read(arguments.rom)?;

    let mut machine = Machine::new(RV64_I);
    machine.attach_debugger(&DefaultDebugger);
    machine.flash(&rom);

    
    loop {
        let word = machine.fetch();
        let instruction = machine.decode(word)?;
        
        (instruction.execute)(&mut machine, word);
        machine.increment_pc();
    }
}

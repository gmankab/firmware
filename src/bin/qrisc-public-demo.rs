// SPDX-License-Identifier: GPL-3.0-only
//! Public-ISA-only, synthetic QRISC program to independently compare our
//! label-resolving Rust assembler with Mesa's `qrisc-asm`.
use nabu_firmware_research::{qrisc_codec::{firmware_header, QriscWord}, qrisc_program::{assemble, Item}};
use std::{env, fs};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = env::args().nth(1).ok_or("usage: qrisc-public-demo OUTPUT")?;
    let program = [
        Item::Instr(firmware_header(0x6ee, 1)?),
        Item::Label("loop"),
        Item::BranchTo { opcode: 0x32, src: 0, condition: 0, label: "end" },
        Item::Instr(QriscWord::MovImmediate { repeat: false, dst: 2, shift: 0, imm: 42 }),
        Item::Label("end"),
        Item::BranchTo { opcode: 0x30, src: 2, condition: 0, label: "loop" },
        Item::Instr(QriscWord::Return { ignored_bits: 0 }),
    ];
    let bytes = assemble(&program)?;
    fs::write(path, &bytes)?;
    println!("Emitted {} bytes of independently authored QRISC demonstration code", bytes.len());
    Ok(())
}

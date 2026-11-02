// SPDX-License-Identifier: GPL-3.0-only
//! Venus runtime allocator/initialization logic. Uses typed A32 encodings and
//! platform-owned data addresses, not embedded original instruction words.
use arm32::{
    data_immediate, mov_register, push_registers, transfer_multiple, transfer_word_postindexed,
    BlockMode,
    Condition as C, DataOp as Op, Register as R,
};
use super::venus_runtime_tail::{put, call, jump, mov, add, cmp, load, store};

struct Code<'a> { bytes: &'a mut Vec<u8>, at: usize }
impl Code<'_> {
    fn emit(&mut self, word: u32) -> Result<(), String> {
        put(self.bytes, self.at, word)?;
        self.at += 4;
        Ok(())
    }
    fn target(&mut self, target: usize) -> Result<(), String> {
        self.emit(call(self.at, target)?)
    }
    fn branch(&mut self, cond: C, target: usize) -> Result<(), String> {
        self.emit(jump(cond, self.at, target)?)
    }
    fn check(&self, expected: usize) -> Result<(), String> {
        if self.at != expected { return Err(format!("Venus resource initialization layout 0x{:x} != 0x{expected:x}", self.at)); }
        Ok(())
    }
    fn literal(&mut self, reg: R, location: usize) -> Result<(), String> {
        self.emit(load(reg, R::PC, (location as isize - self.at as isize - 8) as i32)?)
    }
}

/// Allocate a firmware-owned object, store its pointer in the system-state
/// record, and emit the diagnostic registration command. A zero allocation
/// branches to the shared unwind handler.
fn allocate_and_register(c: &mut Code<'_>, size: u32, offset: i32,
                         literal: usize, service_id: u32) -> Result<(), String> {
    c.emit(mov(R::R0, size)?)?;
    c.target(0x2efc)?;
    c.emit(cmp(R::R0, 0)?)?;
    c.emit(store(R::R0, R::new(5)?, offset)?)?;
    c.branch(C::Eq, 0x1d3c)?;
    c.literal(R::R2, literal)?;
    c.emit(mov(R::R1, service_id)?)?;
    c.target(0x21b8)?;
    Ok(())
}

/// Allocate an auxiliary buffer, place it in a nested object slot, and
/// stop the startup path when the allocator returns a null pointer.
fn allocate_nested(c: &mut Code<'_>, size: u32, field: i32) -> Result<(), String> {
    c.emit(mov(R::R0, size)?)?;
    c.target(0x2efc)?;
    c.emit(load(R::R1, R::new(5)?, 0x18)?)?;
    c.emit(cmp(R::R0, 0)?)?;
    c.emit(store(R::R0, R::R1, field)?)?;
    c.branch(C::Eq, 0x1d3c)?;
    Ok(())
}

/// Attach an allocated firmware service to the corresponding virtual
/// memory mapping descriptor; `flags` is the public memory-attribute field.
fn map_allocation(c: &mut Code<'_>, object_offset: i32,
                  mapping_offset: u32, flags: u32) -> Result<(), String> {
    let r5 = R::new(5)?;
    c.emit(load(R::R0, r5, object_offset)?)?;
    c.target(0x21b0)?;
    c.emit(mov_register(C::Al, R::R1, R::R0))?;
    c.emit(add(R::R0, r5, mapping_offset)?)?;
    c.emit(mov(R::R2, flags)?)?;
    c.target(0x2924)?;
    Ok(())
}

/// Register a nested resource's mapping. Descriptor pointers belong to the
/// parent runtime object; the leaf address is stored within that descriptor.
fn map_nested(c: &mut Code<'_>, nested_field: i32,
              output_offset: u32, flags: u32) -> Result<(), String> {
    c.emit(load(R::R0, R::new(5)?, 0x18)?)?;
    c.emit(mov(R::R2, flags)?)?;
    c.emit(load(R::R1, R::R0, nested_field)?)?;
    c.emit(add(R::R0, R::R0, output_offset)?)?;
    c.emit(load(R::R1, R::R1, 4)?)?;
    c.target(0x2924)?;
    Ok(())
}

/// Release the outer runtime object's ownership of an allocated mapping.
fn release_mapping(c: &mut Code<'_>, offset: i32) -> Result<(), String> {
    let r5 = R::new(5)?;
    c.emit(load(R::R0, r5, offset)?)?;
    c.emit(add(R::R0, R::R0, 0x4c)?)?;
    c.target(0x2ce0)?;
    c.emit(load(R::R0, r5, offset)?)?;
    c.emit(load(R::R0, R::R0, 0)?)?;
    c.target(0x2bcc)?;
    c.emit(load(R::R0, r5, offset)?)?;
    c.target(0x2f1c)?;
    Ok(())
}

fn release_nested(c: &mut Code<'_>, field: i32) -> Result<(), String> {
    c.emit(load(R::R0, R::new(5)?, 0x18)?)?;
    c.emit(load(R::R0, R::R0, field)?)?;
    c.emit(add(R::R0, R::R0, 4)?)?;
    c.target(0x2ce0)?;
    c.emit(load(R::R0, R::new(5)?, 0x18)?)?;
    c.emit(load(R::R0, R::R0, field)?)?;
    c.target(0x2f1c)?;
    Ok(())
}

/// Reconstruct the ARM32 startup allocation sequence through its first eight
/// subsidiary firmware-service registrations. The next block will continue
/// this same sequence; no output is emitted when code regions are incomplete.
pub(super) fn extend(bytes: &mut Vec<u8>) -> Result<(), String> {
    let mut c = Code { bytes, at: 0x1994 };
    let r4 = R::new(4)?;
    let r5 = R::new(5)?;
    c.emit(push_registers(C::Al, r4.bit() | r5.bit() | R::new(6)?.bit()
         | R::new(7)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    c.emit(add(R::R11, R::SP, 16)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Sub, R::SP, R::SP, 8)?)?;
    for target in [0xd5fc, 0x31d80, 0x25d14] { c.target(target)?; }

    // First resource: the object-table pointer is loaded into r5 after the
    // allocation, which differs from the subsequent resource records.
    c.emit(mov(R::R0, 80)?)?;
    c.target(0x2efc)?;
    c.literal(r5, 0x1e80)?;
    c.emit(cmp(R::R0, 0)?)?;
    c.emit(store(R::R0, r5, 0x10)?)?;
    c.branch(C::Eq, 0x1d3c)?;
    c.literal(R::R2, 0x1e84)?;
    c.emit(mov(R::R1, 126)?)?;
    c.target(0x21b8)?;
    c.check(0x19d0)?;

    for (start, size, field, literal, service) in [
        (0x19d0, 80, 0x14, 0x1e88, 127),
        (0x19f0, 80, 0x08, 0x1e8c, 155),
        (0x1a10, 80, 0x0c, 0x1e90, 156),
    ] {
        c.check(start)?;
        allocate_and_register(&mut c, size, field, literal, service)?;
    }
    c.check(0x1a30)?;
    c.emit(mov(R::R0, 436)?)?;
    c.target(0x2efc)?;
    c.emit(cmp(R::R0, 0)?)?;
    c.emit(store(R::R0, r5, 0x18)?)?;
    c.branch(C::Eq, 0x1d3c)?;
    c.emit(mov(R::R1, 0)?)?;
    c.emit(mov(R::R2, 436)?)?;
    c.emit(mov(r4, 0)?)?;
    c.target(0x31ec)?;
    c.emit(load(R::R0, r5, 0x18)?)?;
    c.emit(add(R::R1, R::R0, 64)?)?;
    c.check(0x1a5c)?;
    c.literal(R::R2, 0x1e94)?;
    c.emit(add(r4, r4, 1)?)?;
    c.emit(cmp(r4, 16)?)?;
    c.emit(transfer_word_postindexed(C::Al, false, R::R2, R::R1, 4)?)?;
    c.branch(C::Ne, 0x1a5c)?;
    c.emit(mov(R::R1, 1)?)?;
    c.emit(store(R::R1, R::R0, 0x84)?)?;
    c.emit(mov(R::R0, 36)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, R::R0, R::R0, 1024)?)?;
    c.target(0x2efc)?;
    c.emit(load(R::R1, r5, 0x18)?)?;
    c.emit(cmp(R::R0, 0)?)?;
    c.emit(store(R::R0, R::R1, 0x90)?)?;
    c.branch(C::Eq, 0x1d3c)?;
    c.emit(mov(R::R2, 36)?)?;
    c.emit(mov(R::R1, 0)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, R::R2, R::R2, 1024)?)?;
    c.target(0x31ec)?;
    c.emit(load(R::R0, r5, 0x18)?)?;
    c.literal(R::R2, 0x1e98)?;
    c.emit(mov(R::R1, 128)?)?;
    c.target(0x21888)?;
    c.check(0x1ab4)?;

    // Nested runtime objects: each block must be allocated, initialized,
    // and registered with a service identifier. These descriptors express
    // the ordered resources without copying any binary opcode bytes.
    for (at, size, field, literal, id, destination) in [
        (0x1ab4, 32, 0x98, 0x1e9c, 158, 0x1bc34),
        (0x1ae8, 32, 0x8c, 0x1ea0, 157, 0xe15c),
        (0x1b1c, 24, 0x94, 0x1ea4, 129, 0x1afe0),
        (0x1b50, 8, 0x04, 0x1ea8, 161, 0x2f5b0),
    ] {
        c.check(at)?;
        if at == 0x1b50 {
            c.emit(mov(R::R0, size)?)?;
            c.target(0x2efc)?;
            c.emit(cmp(R::R0, 0)?)?;
            c.emit(store(R::R0, r5, field)?)?;
            c.branch(C::Eq, 0x1d3c)?;
        } else {
            allocate_nested(&mut c, size, field)?;
            c.emit(mov(R::R1, 0)?)?;
            c.emit(mov(R::R2, size)?)?;
            c.target(0x31ec)?;
        }
        if at != 0x1b50 {
            c.emit(load(R::R0, r5, 0x18)?)?;
        }
        if at == 0x1b50 {
            // The final allocation registers against the newly allocated
            // root object only after retrieving the platform label.
            c.check(0x1b64)?;
        }
        c.literal(R::R2, literal)?;
        c.emit(mov(R::R1, id)?)?;
        c.target(destination)?;
    }
    c.check(0x1b70)?;
    c.emit(mov(R::R0, 200)?)?;
    c.target(0x2efc)?;
    c.emit(cmp(R::R0, 0)?)?;
    c.emit(store(R::R0, r5, 0)?)?;
    c.branch(C::Eq, 0x1d3c)?;
    c.literal(R::R2, 0x1eac)?;
    c.emit(mov(R::R1, 139)?)?;
    c.target(0x29944)?;
    c.check(0x1b90)?;

    c.emit(load(R::R0, r5, 0)?)?;
    c.emit(load(R::R1, r5, 0x18)?)?;
    c.emit(store(R::R1, R::R0, 0x70)?)?;
    for (at, source, destination, flags) in [
        (0x1b9c, 0x08, 0x2c, 0x0800_0000),
        (0x1bb4, 0x0c, 0x34, 0x0800_0000),
        (0x1bcc, 0x0c, 0x44, 0x2000_0000),
        (0x1be4, 0x0c, 0x3c, 0x1000_0000),
        (0x1bfc, 0x10, 0x1c, 0x0800_0000),
        (0x1c14, 0x14, 0x24, 0x0800_0000),
    ] {
        c.check(at)?;
        map_allocation(&mut c, source, destination, flags)?;
    }
    c.check(0x1c2c)?;
    for (at, field, destination, flags) in [
        (0x1c2c, 0x98, 0x9c, 0x0800_0000),
        (0x1c44, 0x8c, 0xa4, 0x1000_0000),
        (0x1c5c, 0x8c, 0xac, 0x0800_0000),
    ] {
        c.check(at)?;
        map_nested(&mut c, field, destination, flags)?;
    }
    c.check(0x1c74)?;
    c.emit(load(R::R0, r5, 4)?)?;
    c.target(0x2f7c0)?;
    c.emit(mov_register(C::Al, R::R1, R::R0))?;
    c.emit(add(R::R0, r5, 0x4c)?)?;
    c.emit(mov(R::R2, 0x0300_0000)?)?;
    c.target(0x2924)?;
    c.check(0x1c8c)?;

    let r6 = R::new(6)?;
    c.literal(r6, 0x1eb0)?;
    c.emit(load(R::new(3)?, r5, 0)?)?;
    c.literal(R::R2, 0x1eb4)?;
    c.emit(mov(r4, 0)?)?;
    c.emit(mov(R::R1, 15)?)?;
    c.emit(store(r4, R::SP, 0)?)?;
    c.emit(load(R::R0, r6, 0)?)?;
    c.target(0xd654)?;
    c.emit(load(R::R0, r6, 0)?)?;
    c.emit(mov(R::R1, 14)?)?;
    c.target(0xd708)?;
    c.emit(load(R::new(3)?, r5, 0)?)?;
    c.emit(load(R::R0, r6, 0)?)?;
    c.literal(R::R2, 0x1eb8)?;
    c.emit(mov(R::R1, 14)?)?;
    c.emit(store(r4, R::SP, 0)?)?;
    c.target(0xd654)?;
    c.literal(R::R0, 0x1ebc)?;
    c.emit(mov(R::R1, 1)?)?;
    c.emit(store(R::R1, R::R0, 0x110)?)?;
    c.emit(store(R::R1, R::R0, 0)?)?;
    c.emit(mov(R::R1, 3)?)?;
    c.emit(store(R::R1, R::R0, 0xf0)?)?;
    c.check(0x1ce8)?;
    for (at, slot) in [(0x1ce8,0x10),(0x1cf0,0x14),(0x1cf8,0x08),(0x1d00,0x0c)] {
        c.check(at)?;
        c.emit(load(R::R0, r5, slot)?)?;
        c.target(0x2898)?;
    }
    for (at, target) in [(0x1d08,0xe3ec),(0x1d10,0x1bebc),
                         (0x1d18,0x1b158),(0x1d20,0x21a98)] {
        c.check(at)?;
        c.emit(load(R::R0, r5, 0x18)?)?;
        c.target(target)?;
    }
    c.emit(load(R::R0, r5, 4)?)?;
    c.target(0x2f7c8)?;
    c.emit(load(R::R0, r5, 0)?)?;
    c.target(0x2d38c)?;
    c.branch(C::Al, 0x1e74)?;
    c.check(0x1d3c)?;

    // All partial-init error paths unwind through this same cleanup block.
    c.emit(load(R::R0, r5, 4)?)?;
    c.target(0x2f1c)?;
    c.emit(load(R::R0, r5, 0)?)?;
    c.emit(cmp(R::R0, 0)?)?;
    c.branch(C::Eq, 0x1d7c)?;
    c.emit(add(R::R0, R::R0, 4)?)?;
    c.target(0x2ce0)?;
    c.emit(load(R::R0, r5, 0)?)?;
    c.emit(add(R::R0, R::R0, 8)?)?;
    c.target(0x2ce0)?;
    c.emit(load(R::R0, r5, 0)?)?;
    c.emit(load(R::R0, R::R0, 0x28)?)?;
    c.target(0x2bcc)?;
    c.emit(load(R::R0, r5, 0)?)?;
    c.target(0x2f1c)?;
    c.check(0x1d78)?;
    c.emit(load(R::R0, r5, 0)?)?;
    c.target(0x2f1c)?;
    for (at, source) in [(0x1d80,0x10),(0x1da0,0x14),
                         (0x1dc0,0x08),(0x1de0,0x0c)] {
        c.check(at)?;
        release_mapping(&mut c, source)?;
    }
    for (at, field) in [(0x1e00,0x90),(0x1e1c,0x8c),
                        (0x1e38,0x98),(0x1e54,0x94)] {
        c.check(at)?;
        release_nested(&mut c, field)?;
    }
    c.check(0x1e70)?;
    c.emit(data_immediate(C::Al, Op::Mvn, r4, R::R0, 0)?)?;
    c.emit(mov_register(C::Al, R::R0, r4))?;
    c.emit(data_immediate(C::Al, Op::Sub, R::SP, R::R11, 16)?)?;
    c.emit(transfer_multiple(C::Al, true, R::SP,
        r4.bit() | r5.bit() | r6.bit() | R::new(7)?.bit() | R::R11.bit() | R::PC.bit(),
        BlockMode::IncrementAfter, true)?)?;
    c.check(0x1e80)?;
    // Runtime state pointers, memory attributes, diagnostics labels and
    // a conventional sentinel; this table contains no instruction encodings.
    const INITIALIZATION_CONSTANTS: [u32; 16] = [
        0x0010_ceac, 0x000e_2511, 0x000e_253b, 0x000e_2534,
        0x000e_252d, 0xdead_beef, 0x000e_255e, 0x000e_2565,
        0x000e_2573, 0x000e_256c, 0x000e_2550, 0x000e_2549,
        0x0010_cf04, 0x0002_b13c, 0x0002_b178, 0xe00d_2010,
    ];
    for value in INITIALIZATION_CONSTANTS { c.emit(value)?; }
    c.check(0x1ec0)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rejects_partial_prefix() { assert!(extend(&mut vec![]).is_err()); }
}

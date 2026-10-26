// SPDX-License-Identifier: GPL-3.0-only
//! Venus MMIO/controller service diagnostics, allocation and status helpers.
//! ARM A32 instructions are independently encoded from operands. Literal
//! address/diagnostic tables below are modeled as data, not OEM opcodes.
use super::Code;
use arm32::{Condition as C, DataOp as Op, Register as R, BlockMode};
const START:usize=0x31d80;
const END:usize=0x32190;

/// ARM32 subroutine/handler at 0x31d80; expected next boundary 0x31db0.
fn log_control_status(c:&mut Code)->Result<(),String>{
    c.check(0x31d80)?;
    // 0x31d80: push {r11, lr}
    c.emit(arm32::push_registers(C::Al,R::R11.bit()|R::LR.bit())?)?;
    // 0x31d84: mov r11, sp
    c.emit(arm32::mov_register(C::Al,R::R11,R::SP))?;
    // 0x31d88: ldr r0, [pc, #0x20]         @ 0x30 <entry+0x30>
    c.literal(R::R0,0x31db0)?;
    // 0x31d8c: bl 0x2b8c <entry+0xfffffffffffd0e0c> @ imm = #-0x2f208
    c.bl(0x2b8c)?;
    // 0x31d90: ldr r2, [pc, #0x1c]         @ 0x34 <entry+0x34>
    c.literal(R::R2,0x31db4)?;
    // 0x31d94: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x31d98: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x31d9c: moveq r1, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x31da0: mov r0, r1
    c.emit(arm32::mov_register(C::Al,R::R0,R::R1))?;
    // 0x31da4: mov r1, #196
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,196)?)?;
    // 0x31da8: pop {r11, lr}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::R11.bit()|R::LR.bit(),BlockMode::IncrementAfter,true)?)?;
    // 0x31dac: b 0x17b0 <entry+0xfffffffffffcfa30> @ imm = #-0x30604
    c.b(C::Al,0x17b0)?;
    c.check(0x31db0)
}

/// ARM32 subroutine/handler at 0x31db8; expected next boundary 0x31dc4.
fn read_control_status(c:&mut Code)->Result<(),String>{
    c.check(0x31db8)?;
    // 0x31db8: ldr r0, [pc, #0x4]          @ 0x44 <entry+0x44>
    c.literal(R::R0,0x31dc4)?;
    // 0x31dbc: ldr r0, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    // 0x31dc0: b 0x2bcc <entry+0xfffffffffffd0e4c> @ imm = #-0x2f1fc
    c.b(C::Al,0x2bcc)?;
    c.check(0x31dc4)
}

/// ARM32 subroutine/handler at 0x31dc8; expected next boundary 0x31dcc.
fn return_control(c:&mut Code)->Result<(),String>{
    c.check(0x31dc8)?;
    // 0x31dc8: bx lr
    c.emit(arm32::bx(C::Al,R::LR))?;
    c.check(0x31dcc)
}

/// ARM32 subroutine/handler at 0x31dcc; expected next boundary 0x31e80.
fn allocate_service_buffers(c:&mut Code)->Result<(),String>{
    c.check(0x31dcc)?;
    // 0x31dcc: push {r4, r5, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // 0x31dd0: add r11, sp, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,8)?)?;
    // 0x31dd4: ldr r1, [pc, #0xa4]         @ 0x100 <entry+0x100>
    c.literal(R::R1,0x31e80)?;
    // 0x31dd8: ldr r2, [pc, #0xa4]         @ 0x104 <entry+0x104>
    c.literal(R::R2,0x31e84)?;
    // 0x31ddc: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x31de0: mov r3, #20
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,20)?)?;
    // 0x31de4: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x3158
    c.bl(0x2ec94)?;
    // 0x31de8: ldr r1, [pc, #0x98]         @ 0x108 <entry+0x108>
    c.literal(R::R1,0x31e88)?;
    // 0x31dec: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x31df0: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x3164
    c.bl(0x2ec94)?;
    // 0x31df4: mov r0, #40
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,40)?)?;
    // 0x31df8: bl 0x2efc <entry+0xfffffffffffd117c> @ imm = #-0x2ef04
    c.bl(0x2efc)?;
    // 0x31dfc: mov r4, r0
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // 0x31e00: cmp r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,0)?)?;
    // 0x31e04: beq 0x31e48 <entry+0xc8>    @ imm = #0x3c
    c.b(C::Eq,0x31e48)?;
    // 0x31e08: mov r0, #20
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,20)?)?;
    // 0x31e0c: bl 0x2efc <entry+0xfffffffffffd117c> @ imm = #-0x2ef18
    c.bl(0x2efc)?;
    // 0x31e10: mov r5, r0
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R0))?;
    // 0x31e14: cmp r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,0)?)?;
    // 0x31e18: beq 0x31e5c <entry+0xdc>    @ imm = #0x3c
    c.b(C::Eq,0x31e5c)?;
    // 0x31e1c: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x31e20: mov r1, #153
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,153)?)?;
    // 0x31e24: mov r2, #90
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,90)?)?;
    // 0x31e28: bl 0x31ec <entry+0xfffffffffffd146c> @ imm = #-0x2ec44
    c.bl(0x31ec)?;
    // 0x31e2c: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // 0x31e30: bl 0x2f1c <entry+0xfffffffffffd119c> @ imm = #-0x2ef1c
    c.bl(0x2f1c)?;
    // 0x31e34: ldr r1, [pc, #0x50]         @ 0x10c <entry+0x10c>
    c.literal(R::R1,0x31e8c)?;
    // 0x31e38: ldr r2, [pc, #0x44]         @ 0x104 <entry+0x104>
    c.literal(R::R2,0x31e84)?;
    // 0x31e3c: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x31e40: mov r3, #30
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,30)?)?;
    // 0x31e44: b 0x31e6c <entry+0xec>    @ imm = #0x20
    c.b(C::Al,0x31e6c)?;
    // 0x31e48: ldr r1, [pc, #0x40]         @ 0x110 <entry+0x110>
    c.literal(R::R1,0x31e90)?;
    // 0x31e4c: ldr r2, [pc, #0x30]         @ 0x104 <entry+0x104>
    c.literal(R::R2,0x31e84)?;
    // 0x31e50: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x31e54: mov r3, #39
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,39)?)?;
    // 0x31e58: b 0x31e6c <entry+0xec>    @ imm = #0xc
    c.b(C::Al,0x31e6c)?;
    // 0x31e5c: ldr r1, [pc, #0x2c]         @ 0x110 <entry+0x110>
    c.literal(R::R1,0x31e90)?;
    // 0x31e60: ldr r2, [pc, #0x1c]         @ 0x104 <entry+0x104>
    c.literal(R::R2,0x31e84)?;
    // 0x31e64: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x31e68: mov r3, #34
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,34)?)?;
    // 0x31e6c: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x31e0
    c.bl(0x2ec94)?;
    // 0x31e70: ldr r1, [pc, #0x10]         @ 0x108 <entry+0x108>
    c.literal(R::R1,0x31e88)?;
    // 0x31e74: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x31e78: pop {r4, r5, r11, lr}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::R11.bit()|R::LR.bit(),BlockMode::IncrementAfter,true)?)?;
    // 0x31e7c: b 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x31f0
    c.b(C::Al,0x2ec94)?;
    c.check(0x31e80)
}

/// ARM32 subroutine/handler at 0x31e94; expected next boundary 0x31f60.
fn probe_control_allocation(c:&mut Code)->Result<(),String>{
    c.check(0x31e94)?;
    // 0x31e94: push {r11, lr}
    c.emit(arm32::push_registers(C::Al,R::R11.bit()|R::LR.bit())?)?;
    // 0x31e98: mov r11, sp
    c.emit(arm32::mov_register(C::Al,R::R11,R::SP))?;
    // 0x31e9c: ldr r1, [pc, #0xbc]         @ 0x1e0 <entry+0x1e0>
    c.literal(R::R1,0x31f60)?;
    // 0x31ea0: ldr r2, [pc, #0xbc]         @ 0x1e4 <entry+0x1e4>
    c.literal(R::R2,0x31f64)?;
    // 0x31ea4: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x31ea8: mov r3, #48
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,48)?)?;
    // 0x31eac: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x3220
    c.bl(0x2ec94)?;
    // 0x31eb0: ldr r1, [pc, #0xb0]         @ 0x1e8 <entry+0x1e8>
    c.literal(R::R1,0x31f68)?;
    // 0x31eb4: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x31eb8: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x322c
    c.bl(0x2ec94)?;
    // 0x31ebc: mov r0, #40
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,40)?)?;
    // 0x31ec0: bl 0x2efc <entry+0xfffffffffffd117c> @ imm = #-0x2efcc
    c.bl(0x2efc)?;
    // 0x31ec4: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x31ec8: beq 0x31edc <entry+0x15c>   @ imm = #0xc
    c.b(C::Eq,0x31edc)?;
    // 0x31ecc: mov r1, #153
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,153)?)?;
    // 0x31ed0: mov r2, #90
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,90)?)?;
    // 0x31ed4: bl 0x31ec <entry+0xfffffffffffd146c> @ imm = #-0x2ecf0
    c.bl(0x31ec)?;
    // 0x31ed8: b 0x31efc <entry+0x17c>   @ imm = #0x1c
    c.b(C::Al,0x31efc)?;
    // 0x31edc: ldr r1, [pc, #0x88]         @ 0x1ec <entry+0x1ec>
    c.literal(R::R1,0x31f6c)?;
    // 0x31ee0: ldr r2, [pc, #0x7c]         @ 0x1e4 <entry+0x1e4>
    c.literal(R::R2,0x31f64)?;
    // 0x31ee4: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x31ee8: mov r3, #57
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,57)?)?;
    // 0x31eec: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x3260
    c.bl(0x2ec94)?;
    // 0x31ef0: ldr r1, [pc, #0x70]         @ 0x1e8 <entry+0x1e8>
    c.literal(R::R1,0x31f68)?;
    // 0x31ef4: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x31ef8: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x326c
    c.bl(0x2ec94)?;
    // 0x31efc: mov r0, #20
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,20)?)?;
    // 0x31f00: bl 0x2efc <entry+0xfffffffffffd117c> @ imm = #-0x2f00c
    c.bl(0x2efc)?;
    // 0x31f04: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x31f08: beq 0x31f1c <entry+0x19c>   @ imm = #0xc
    c.b(C::Eq,0x31f1c)?;
    // 0x31f0c: mov r1, #221
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,221)?)?;
    // 0x31f10: mov r2, #20
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,20)?)?;
    // 0x31f14: bl 0x31ec <entry+0xfffffffffffd146c> @ imm = #-0x2ed30
    c.bl(0x31ec)?;
    // 0x31f18: b 0x31f3c <entry+0x1bc>   @ imm = #0x1c
    c.b(C::Al,0x31f3c)?;
    // 0x31f1c: ldr r1, [pc, #0x48]         @ 0x1ec <entry+0x1ec>
    c.literal(R::R1,0x31f6c)?;
    // 0x31f20: ldr r2, [pc, #0x3c]         @ 0x1e4 <entry+0x1e4>
    c.literal(R::R2,0x31f64)?;
    // 0x31f24: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x31f28: mov r3, #67
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,67)?)?;
    // 0x31f2c: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x32a0
    c.bl(0x2ec94)?;
    // 0x31f30: ldr r1, [pc, #0x30]         @ 0x1e8 <entry+0x1e8>
    c.literal(R::R1,0x31f68)?;
    // 0x31f34: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x31f38: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x32ac
    c.bl(0x2ec94)?;
    // 0x31f3c: ldr r1, [pc, #0x2c]         @ 0x1f0 <entry+0x1f0>
    c.literal(R::R1,0x31f70)?;
    // 0x31f40: ldr r2, [pc, #0x1c]         @ 0x1e4 <entry+0x1e4>
    c.literal(R::R2,0x31f64)?;
    // 0x31f44: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x31f48: mov r3, #70
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,70)?)?;
    // 0x31f4c: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x32c0
    c.bl(0x2ec94)?;
    // 0x31f50: ldr r1, [pc, #0x10]         @ 0x1e8 <entry+0x1e8>
    c.literal(R::R1,0x31f68)?;
    // 0x31f54: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x31f58: pop {r11, lr}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::R11.bit()|R::LR.bit(),BlockMode::IncrementAfter,true)?)?;
    // 0x31f5c: b 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x32d0
    c.b(C::Al,0x2ec94)?;
    c.check(0x31f60)
}

/// ARM32 subroutine/handler at 0x31f74; expected next boundary 0x31fe0.
fn record_controller_result(c:&mut Code)->Result<(),String>{
    c.check(0x31f74)?;
    // 0x31f74: push {r4, r5, r6, r7, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // 0x31f78: add r11, sp, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,16)?)?;
    // 0x31f7c: ldr r4, [pc, #0x60]         @ 0x264 <entry+0x264>
    c.literal(R::new(4)?,0x31fe4)?;
    // 0x31f80: ldr r1, [pc, #0x58]         @ 0x260 <entry+0x260>
    c.literal(R::R1,0x31fe0)?;
    // 0x31f84: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x31f88: mov r3, #77
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,77)?)?;
    // 0x31f8c: mov r2, r4
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(4)?))?;
    // 0x31f90: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x3304
    c.bl(0x2ec94)?;
    // 0x31f94: ldr r5, [pc, #0x4c]         @ 0x268 <entry+0x268>
    c.literal(R::new(5)?,0x31fe8)?;
    // 0x31f98: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x31f9c: mov r1, r5
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(5)?))?;
    // 0x31fa0: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x3314
    c.bl(0x2ec94)?;
    // 0x31fa4: mov r0, #40
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,40)?)?;
    // 0x31fa8: bl 0x2efc <entry+0xfffffffffffd117c> @ imm = #-0x2f0b4
    c.bl(0x2efc)?;
    // 0x31fac: mov r6, r0
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::R0))?;
    // 0x31fb0: bl 0x2f1c <entry+0xfffffffffffd119c> @ imm = #-0x2f09c
    c.bl(0x2f1c)?;
    // 0x31fb4: mov r0, r6
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
    // 0x31fb8: bl 0x2f1c <entry+0xfffffffffffd119c> @ imm = #-0x2f0a4
    c.bl(0x2f1c)?;
    // 0x31fbc: ldr r1, [pc, #0x28]         @ 0x26c <entry+0x26c>
    c.literal(R::R1,0x31fec)?;
    // 0x31fc0: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x31fc4: mov r2, r4
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(4)?))?;
    // 0x31fc8: mov r3, #82
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,82)?)?;
    // 0x31fcc: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x3340
    c.bl(0x2ec94)?;
    // 0x31fd0: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x31fd4: mov r1, r5
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(5)?))?;
    // 0x31fd8: pop {r4, r5, r6, r7, r11, lr}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit(),BlockMode::IncrementAfter,true)?)?;
    // 0x31fdc: b 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x3350
    c.b(C::Al,0x2ec94)?;
    c.check(0x31fe0)
}

/// ARM32 subroutine/handler at 0x31ff0; expected next boundary 0x32054.
fn check_controller_message(c:&mut Code)->Result<(),String>{
    c.check(0x31ff0)?;
    // 0x31ff0: push {r4, r5, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // 0x31ff4: add r11, sp, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,8)?)?;
    // 0x31ff8: ldr r4, [pc, #0x58]         @ 0x2d8 <entry+0x2d8>
    c.literal(R::new(4)?,0x32058)?;
    // 0x31ffc: ldr r1, [pc, #0x50]         @ 0x2d4 <entry+0x2d4>
    c.literal(R::R1,0x32054)?;
    // 0x32000: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x32004: mov r3, #88
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,88)?)?;
    // 0x32008: mov r2, r4
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(4)?))?;
    // 0x3200c: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x3380
    c.bl(0x2ec94)?;
    // 0x32010: ldr r5, [pc, #0x44]         @ 0x2dc <entry+0x2dc>
    c.literal(R::new(5)?,0x3205c)?;
    // 0x32014: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x32018: mov r1, r5
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(5)?))?;
    // 0x3201c: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x3390
    c.bl(0x2ec94)?;
    // 0x32020: mov r0, #40
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,40)?)?;
    // 0x32024: bl 0x2efc <entry+0xfffffffffffd117c> @ imm = #-0x2f130
    c.bl(0x2efc)?;
    // 0x32028: add r0, r0, #20
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,20)?)?;
    // 0x3202c: bl 0x2f1c <entry+0xfffffffffffd119c> @ imm = #-0x2f118
    c.bl(0x2f1c)?;
    // 0x32030: ldr r1, [pc, #0x28]         @ 0x2e0 <entry+0x2e0>
    c.literal(R::R1,0x32060)?;
    // 0x32034: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x32038: mov r2, r4
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(4)?))?;
    // 0x3203c: mov r3, #91
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,91)?)?;
    // 0x32040: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x33b4
    c.bl(0x2ec94)?;
    // 0x32044: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x32048: mov r1, r5
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(5)?))?;
    // 0x3204c: pop {r4, r5, r11, lr}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::R11.bit()|R::LR.bit(),BlockMode::IncrementAfter,true)?)?;
    // 0x32050: b 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x33c4
    c.b(C::Al,0x2ec94)?;
    c.check(0x32054)
}

/// ARM32 subroutine/handler at 0x32064; expected next boundary 0x32170.
fn trace_controller_initial_values(c:&mut Code)->Result<(),String>{
    c.check(0x32064)?;
    // 0x32064: push {r4, r5, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // 0x32068: add r11, sp, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,8)?)?;
    // 0x3206c: sub sp, sp, #16
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,16)?)?;
    // 0x32070: ldr r4, [pc, #0xfc]         @ 0x3f4 <entry+0x3f4>
    c.literal(R::new(4)?,0x32174)?;
    // 0x32074: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x32078: ldr r1, [pc, #0xf0]         @ 0x3f0 <entry+0x3f0>
    c.literal(R::R1,0x32170)?;
    // 0x3207c: mov r3, #110
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,110)?)?;
    // 0x32080: str r0, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,12)?)?;
    // 0x32084: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // 0x32088: strb r0, [sp, #0xb]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::SP,11)?)?;
    // 0x3208c: mov r0, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3)?)?;
    // 0x32090: strb r0, [sp, #0xa]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::SP,10)?)?;
    // 0x32094: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x32098: mov r2, r4
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(4)?))?;
    // 0x3209c: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x3410
    c.bl(0x2ec94)?;
    // 0x320a0: ldr r5, [pc, #0xd0]         @ 0x3f8 <entry+0x3f8>
    c.literal(R::new(5)?,0x32178)?;
    // 0x320a4: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x320a8: mov r1, r5
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(5)?))?;
    // 0x320ac: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x3420
    c.bl(0x2ec94)?;
    // 0x320b0: ldr r1, [pc, #0xc4]         @ 0x3fc <entry+0x3fc>
    c.literal(R::R1,0x3217c)?;
    // 0x320b4: add r0, sp, #10
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::SP,10)?)?;
    // 0x320b8: mov r2, r4
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(4)?))?;
    // 0x320bc: mov r3, #111
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,111)?)?;
    // 0x320c0: str r0, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // 0x320c4: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x320c8: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x343c
    c.bl(0x2ec94)?;
    // 0x320cc: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x320d0: mov r1, r5
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(5)?))?;
    // 0x320d4: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x3448
    c.bl(0x2ec94)?;
    // 0x320d8: ldr r1, [pc, #0xa0]         @ 0x400 <entry+0x400>
    c.literal(R::R1,0x32180)?;
    // 0x320dc: add r0, sp, #11
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::SP,11)?)?;
    // 0x320e0: mov r2, r4
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(4)?))?;
    // 0x320e4: mov r3, #112
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,112)?)?;
    // 0x320e8: str r0, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // 0x320ec: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x320f0: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x3464
    c.bl(0x2ec94)?;
    // 0x320f4: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x320f8: mov r1, r5
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(5)?))?;
    // 0x320fc: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x3470
    c.bl(0x2ec94)?;
    // 0x32100: ldr r1, [pc, #0x7c]         @ 0x404 <entry+0x404>
    c.literal(R::R1,0x32184)?;
    // 0x32104: add r0, sp, #12
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::SP,12)?)?;
    // 0x32108: mov r2, r4
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(4)?))?;
    // 0x3210c: mov r3, #113
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,113)?)?;
    // 0x32110: str r0, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // 0x32114: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x32118: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x348c
    c.bl(0x2ec94)?;
    // 0x3211c: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x32120: mov r1, r5
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(5)?))?;
    // 0x32124: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x3498
    c.bl(0x2ec94)?;
    // 0x32128: ldr r1, [pc, #0x58]         @ 0x408 <entry+0x408>
    c.literal(R::R1,0x32188)?;
    // 0x3212c: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x32130: mov r2, r4
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(4)?))?;
    // 0x32134: mov r3, #114
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,114)?)?;
    // 0x32138: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x34ac
    c.bl(0x2ec94)?;
    // 0x3213c: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x32140: mov r1, r5
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(5)?))?;
    // 0x32144: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x34b8
    c.bl(0x2ec94)?;
    // 0x32148: ldr r1, [pc, #0x3c]         @ 0x40c <entry+0x40c>
    c.literal(R::R1,0x3218c)?;
    // 0x3214c: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x32150: mov r2, r4
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(4)?))?;
    // 0x32154: mov r3, #120
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,120)?)?;
    // 0x32158: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x34cc
    c.bl(0x2ec94)?;
    // 0x3215c: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x32160: mov r1, r5
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(5)?))?;
    // 0x32164: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x34d8
    c.bl(0x2ec94)?;
    // 0x32168: sub sp, r11, #8
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,8)?)?;
    // 0x3216c: pop {r4, r5, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::R11.bit()|R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    c.check(0x32170)
}

// Original data pointers and constants at [0x31db0,0x31db8).
const STATUS_POINTERS:[u32;2]=[0x0010e080,0x000eb1f8];

// Original data pointers and constants at [0x31dc4,0x31dc8).
const STATUS_WORD:[u32;1]=[0x0010e080];

// Original data pointers and constants at [0x31e80,0x31e94).
const BUFFER_ALLOCATION_MESSAGES:[u32;5]=[0x000eb22d,0x000eb253,0x000e2788,0x000eb26c,0x000eb2b2];

// Original data pointers and constants at [0x31f60,0x31f74).
const BUFFER_STATUS_MESSAGES:[u32;5]=[0x000eb22d,0x000eb2e2,0x000e2788,0x000eb2f5,0x000eb26c];

// Original data pointers and constants at [0x31fe0,0x31ff0).
const CONTROLLER_MESSAGE_POINTERS:[u32;4]=[0x000eb33e,0x000eb360,0x000e2788,0x000eb26c];

// Original data pointers and constants at [0x32054,0x32064).
const CONTROLLER_ERRORS:[u32;4]=[0x000eb375,0x000eb39b,0x000e2788,0x000eb26c];

// Original data pointers and constants at [0x32170,0x32190).
const STREAM_INITIAL_STATE_STRINGS:[u32;8]=[0x000eb3b4,0x000eb3d0,0x000e2788,0x000eb3dc,0x000eb3ed,0x000eb3fe,0x000eb40f,0x000eb43b];

pub(super) fn emit(c:&mut Code)->Result<(),String>{
    c.check(START)?;
    log_control_status(c)?;
    c.check(0x31db0)?;
    for word in STATUS_POINTERS{c.emit(word)?;}
    c.check(0x31db8)?;
    read_control_status(c)?;
    c.check(0x31dc4)?;
    for word in STATUS_WORD{c.emit(word)?;}
    c.check(0x31dc8)?;
    return_control(c)?;
    allocate_service_buffers(c)?;
    c.check(0x31e80)?;
    for word in BUFFER_ALLOCATION_MESSAGES{c.emit(word)?;}
    c.check(0x31e94)?;
    probe_control_allocation(c)?;
    c.check(0x31f60)?;
    for word in BUFFER_STATUS_MESSAGES{c.emit(word)?;}
    c.check(0x31f74)?;
    record_controller_result(c)?;
    c.check(0x31fe0)?;
    for word in CONTROLLER_MESSAGE_POINTERS{c.emit(word)?;}
    c.check(0x31ff0)?;
    check_controller_message(c)?;
    c.check(0x32054)?;
    for word in CONTROLLER_ERRORS{c.emit(word)?;}
    c.check(0x32064)?;
    trace_controller_initial_values(c)?;
    c.check(0x32170)?;
    for word in STREAM_INITIAL_STATE_STRINGS{c.emit(word)?;}
    c.check(0x32190)?;
    c.check(END)
}

#[cfg(test)]mod tests {
    use super::*;
    #[test]fn structured_mmio_helpers_cover_contiguous_region(){
        let mut c=Code::new(START);emit(&mut c).unwrap();
        assert_eq!(c.bytes.len(),END-START);
        let at=|addr:usize|u32::from_le_bytes(c.bytes[addr-START..addr-START+4].try_into().unwrap());
        assert_eq!(at(0x31db0),STATUS_POINTERS[0]);
        assert_eq!(at(0x31e80),BUFFER_ALLOCATION_MESSAGES[0]);
        assert_eq!(at(0x31f60),BUFFER_STATUS_MESSAGES[0]);
        assert_eq!(at(0x32054),CONTROLLER_ERRORS[0]);
        assert_eq!(at(0x3218c),STREAM_INITIAL_STATE_STRINGS[7]);
    }
}

// SPDX-License-Identifier: GPL-3.0-only
//! Agent 20-03: independent, typed ARM A32 reconstruction of Venus runtime
//! routines AFTER the prior Agent 03 interval. Does not touch earlier VMA
//! `[0x183e0,0x21c78)` or the other agent's range `>=0x40000`.
//!
//! This is instruction-level re-encoding with separately classified literals,
//! not a high-level source recovery or an OEM-authenticated firmware image.
use arm32::{Condition as C, DataOp as Op, Register as R, Shift, BlockMode};

pub const START: usize = 0x21c78;
pub const END: usize = 0x3701c;
pub const FILE_BASE:usize = 0x4000;
pub const FILE_OFFSET:usize=FILE_BASE+START;
pub const RESERVATION_END:usize=0x40000;

struct Code { start:usize, bytes:Vec<u8> }
impl Code {
    fn new(start:usize)->Self {Self{start,bytes:Vec::new()}}
    fn at(&self)->usize {self.start+self.bytes.len()}
    fn check(&self,expected:usize)->Result<(),String>{
        if self.at()!=expected {return Err(format!("agent20-03 ARM location 0x{:x} != 0x{:x}",self.at(),expected));}
        Ok(())
    }
    fn emit(&mut self,opcode:u32)->Result<(),String>{
        if self.at()<START||self.at()>=RESERVATION_END||self.at()%4!=0 {
            return Err(format!("agent20-03 outside reserved runtime range at 0x{:x}",self.at()));
        }
        self.bytes.extend_from_slice(&opcode.to_le_bytes());Ok(())
    }
    fn b(&mut self,c:C,target:usize)->Result<(),String>{
        self.emit(arm32::branch(c,self.at() as u32,target as u32,false)?)
    }
    fn bl(&mut self,target:usize)->Result<(),String>{
        self.emit(arm32::branch(C::Al,self.at() as u32,target as u32,true)?)
    }
    fn literal(&mut self,rd:R,target:usize)->Result<(),String>{
        self.literal_cond(C::Al,rd,target)
    }
    fn literal_cond(&mut self,condition:C,rd:R,target:usize)->Result<(),String>{
        let delta=(target as i64)-(self.at() as i64)-8;
        let offset=i32::try_from(delta).map_err(|_|"literal offset overflow")?;
        self.emit(arm32::transfer_word(condition,true,rd,R::PC,offset)?)
    }
}

/// Checked generic A32 SMULL/UMULL with correct RdLo/RdHi input fields.
/// Validates public ISA constraints; no vendor-specific opcode words.
fn encode_long_multiply(condition:C,signed:bool,rd_lo:R,rd_hi:R,rm:R,rs:R)->Result<u32,String>{
    if rd_lo==rd_hi || [rd_lo,rd_hi,rm,rs].contains(&R::PC){
        return Err("A32 long multiply requires distinct non-PC destinations and operands".into());
    }
    Ok(((condition as u32)<<28) | if signed {0x00c0_0090} else {0x0080_0090}
       |(rd_hi.number()<<16)|(rd_lo.number()<<12)|(rs.number()<<8)|rm.number())
}

/// ARMv7 DSP SMULBB using the signed lower halfwords from each operand.
/// Encodes fields independently of any proprietary instruction word.
fn encode_smulbb(cond:C,rd:R,rm:R,rs:R)->Result<u32,String>{
    if [rd,rm,rs].iter().any(|r|r.number()>=13) {
        return Err("SMULBB requires general-purpose registers r0..r12".into());
    }
    Ok(((cond as u32)<<28)|0x0160_0080|(rd.number()<<16)|(rs.number()<<8)|rm.number())
}

/// A32 ADC immediate #0; add the current carry to a 32-bit register.
/// Minimal checked form required to propagate a 64-bit counter increment.
fn encode_adc_zero(cond:C,rd:R,rn:R)->Result<u32,String>{
    if [rd,rn].iter().any(|r|r.number()>=13) {
        return Err("A32 ADC counter increment requires ordinary GPR registers".into());
    }
    Ok(((cond as u32)<<28)|0x02a0_0000|(rn.number()<<16)|(rd.number()<<12))
}

#[path="venus_runtime_mid_initialize.rs"]
mod initialize;
#[path="venus_runtime_mid_input.rs"]
mod input;
#[path="venus_runtime_mid_commands.rs"]
mod commands;
#[path="venus_runtime_mid_state.rs"]
mod state;
#[path="venus_runtime_mid_logging.rs"]
mod logging;
#[path="venus_runtime_mid_dispatch.rs"]
mod dispatch;
#[path="venus_runtime_mid_modes.rs"]
mod modes;
#[path="venus_runtime_mid_copy.rs"]
mod copy;
#[path="venus_runtime_mid_helpers.rs"]
mod helpers;
#[path="venus_runtime_mid_capture.rs"]
mod capture;
#[path="venus_runtime_mid_registers.rs"]
mod registers;
#[path="venus_runtime_mid_irq.rs"]
mod irq;
#[path="venus_runtime_mid_irq_init.rs"]
mod irq_init;
#[path="venus_runtime_mid_events.rs"]
mod events;
#[path="venus_runtime_mid_irq_shutdown.rs"]
mod irq_shutdown;
#[path="venus_runtime_mid_irq_channels.rs"]
mod irq_channels;
#[path="venus_runtime_mid_handshake.rs"]
mod handshake;
#[path="venus_runtime_mid_controller.rs"]
mod controller;
#[path="venus_runtime_mid_status.rs"]
mod status;
#[path="venus_runtime_mid_queue_control.rs"]
mod queue_control;
#[path="venus_runtime_mid_queue_extension.rs"]
mod queue_extension;
#[path="venus_runtime_mid_controller_frame.rs"]
mod controller_frame;
#[path="venus_runtime_mid_frame_policy.rs"]
mod frame_policy;
#[path="venus_runtime_mid_plane_dma.rs"]
mod plane_dma;
#[path="venus_runtime_mid_transfer_sequence.rs"]
mod transfer_sequence;
#[path="venus_runtime_mid_active_port.rs"]
mod active_port;
#[path="venus_runtime_mid_frame_release.rs"]
mod frame_release;
#[path="venus_runtime_mid_callbacks_release.rs"]
mod callbacks_release;
#[path="venus_runtime_mid_register_snapshot.rs"]
mod register_snapshot;
#[path="venus_runtime_mid_irq_readback.rs"]
mod irq_readback;
#[path="venus_runtime_mid_priority_irq.rs"]
mod priority_irq;
#[path="venus_runtime_mid_dma_completion.rs"]
mod dma_completion;
#[path="venus_runtime_mid_capture_setup.rs"]
mod capture_setup;
#[path="venus_runtime_mid_format_policy.rs"]
mod format_policy;
#[path="venus_runtime_mid_submit_release.rs"]
mod submit_release;
#[path="venus_runtime_mid_control_dispatch.rs"]
mod control_dispatch;
#[path="venus_runtime_mid_control_io.rs"]
mod control_io;
#[path="venus_runtime_mid_controller_start.rs"]
mod controller_start;
#[path="venus_runtime_mid_event_router.rs"]
mod event_router;
#[path="venus_runtime_mid_codec_routes.rs"]
mod codec_routes;
#[path="venus_runtime_mid_frame_messages.rs"]
mod frame_messages;
#[path="venus_runtime_mid_buffer_cleanup.rs"]
mod buffer_cleanup;
#[path="venus_runtime_mid_class_handlers.rs"]
mod class_handlers;
#[path="venus_runtime_mid_codec_categories.rs"]
mod codec_categories;
#[path="venus_runtime_mid_request_dispatch.rs"]
mod request_dispatch;
#[path="venus_runtime_mid_class_continuation.rs"]
mod class_continuation;
#[path="venus_runtime_mid_resource_access.rs"]
mod resource_access;
#[path="venus_runtime_mid_command_registers.rs"]
mod command_registers;
#[path="venus_runtime_mid_encoder_init.rs"]
mod encoder_init;
#[path="venus_runtime_mid_packed_bits.rs"]
mod packed_bits;
#[path="venus_runtime_mid_event_publish.rs"]
mod event_publish;
#[path="venus_runtime_mid_event_monitor.rs"]
mod event_monitor;
#[path="venus_runtime_mid_ring_helpers.rs"]
mod ring_helpers;
#[path="venus_runtime_mid_ring_slot_access.rs"]
mod ring_slot_access;
#[path="venus_runtime_mid_dma_initialize.rs"]
mod dma_initialize;
#[path="venus_runtime_mid_mmio_poll.rs"]
mod mmio_poll;
#[path="venus_runtime_mid_irq_banks.rs"]
mod irq_banks;
#[path="venus_runtime_mid_irq_router.rs"]
mod irq_router;
#[path="venus_runtime_mid_image_planes.rs"]
mod image_planes;
#[path="venus_runtime_mid_codec_register_serializer.rs"]
mod codec_register_serializer;
#[path="venus_runtime_mid_buffer_ctrl_commands.rs"]
mod buffer_ctrl_commands;
#[path="venus_runtime_mid_post_reset_banks.rs"]
mod post_reset_banks;
#[path="venus_runtime_mid_input_capabilities.rs"]
mod input_capabilities;
#[path="venus_runtime_mid_mmio_diagnostics.rs"]
mod mmio_diagnostics;
#[path="venus_runtime_mid_codec_table_setup.rs"]
mod codec_table_setup;
#[path="venus_runtime_mid_codec_resources.rs"]
mod codec_resources;
#[path="venus_runtime_mid_codec_class_prefix.rs"]
mod codec_class_prefix;
#[path="venus_runtime_mid_codec_class_cases.rs"]
mod codec_class_cases;
#[path="venus_runtime_mid_event_classification.rs"]
mod event_classification;
#[path="venus_runtime_mid_callback_helpers.rs"]
mod callback_helpers;
#[path="venus_runtime_mid_event_type_resolve.rs"]
mod event_type_resolve;
#[path="venus_runtime_mid_codec_irq_frames.rs"]
mod codec_irq_frames;
#[path="venus_runtime_mid_codec_frame_update.rs"]
mod codec_frame_update;
#[path="venus_runtime_mid_controller_stream_ops.rs"]
mod controller_stream_ops;
#[path="venus_runtime_mid_controller_commit.rs"]
mod controller_commit;
#[path="venus_runtime_mid_codec_stream_init.rs"]
mod codec_stream_init;
#[path="venus_runtime_mid_exponent_and_alloc.rs"]
mod exponent_and_alloc;
#[path="venus_runtime_mid_ring_queue.rs"]
mod ring_queue;
#[path="venus_runtime_mid_frame_dispatch_init.rs"]
mod frame_dispatch_init;
#[path="venus_runtime_mid_ring_descriptor_ops.rs"]
mod ring_descriptor_ops;
#[path="venus_runtime_mid_ring_queue_update.rs"]
mod ring_queue_update;
#[path="venus_runtime_mid_controller_defaults.rs"]
mod controller_defaults;

fn frame_allocation(c:&mut Code)->Result<(),String>{
    c.check(START)?;
    // 0x21c78
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // 0x21c7c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,16)?)?;
    // 0x21c80
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,16)?)?;
    // 0x21c84
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,11)?)?;
    // 0x21c88
    c.b(C::Hi,0x21d70)?;
    // 0x21c8c
    c.emit(arm32::mov_shifted_register(C::Al,R::R2,R::R2,Shift::Lsl,2)?)?;
    // 0x21c90
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::PC,0)?)?;
    // 0x21c94
    c.emit(arm32::transfer_word_register(C::Al,true,R::PC,R::R2,R::new(3)?,Shift::Lsl,0)?)?;
    c.check(0x21c98)?;
    // Branch targets for 12 known image-space formats (non-instruction data).
    const FORMAT_BRANCHES:[u32;12]=[
        0x21d4c,0x21cc8,0x21d30,0x21d70,0x21d4c,0x21d88,
        0x21d70,0x21d4c,0x21cc8,0x21cc8,0x21cc8,0x21dac,
    ];
    for destination in FORMAT_BRANCHES { c.emit(destination)?; }
    c.check(0x21cc8)?;
    // 0x21cc8
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,56)?)?;
    // 0x21ccc
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R1,60)?)?;
    // 0x21cd0
    c.emit(arm32::multiply_register(C::Al,R::new(6)?,R::new(3)?,R::R2)?)?;
    // 0x21cd4
    c.emit(arm32::data_immediate_with_flags(C::Al,Op::Add,R::R2,R::new(6)?,1,true)?)?;
    // 0x21cd8
    c.b(C::Eq,0x21e18)?;
    // 0x21cdc
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,12)?)?;
    // 0x21ce0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // 0x21ce4
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::R0,R::R1,Shift::Lsl,2)?)?;
    // 0x21ce8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x21cec
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,676)?)?;
    // 0x21cf0
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,4096)?)?;
    // 0x21cf4
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R0,0)?)?;
    // 0x21cf8
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::R0,40)?)?;
    // 0x21cfc
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,1)?)?;
    // 0x21d00
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,4)?)?;
    // 0x21d04
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::R2,Shift::Lsl,0,false)?)?;
    // 0x21d08
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(3)?,15)?)?;
    // 0x21d0c
    c.emit(arm32::mov_shifted_register(C::Al,R::new(4)?,R::new(3)?,Shift::Lsr,4)?)?;
    // 0x21d10
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(6)?,15)?)?;
    // 0x21d14
    c.emit(arm32::mov_shifted_register(C::Al,R::new(6)?,R::new(3)?,Shift::Lsr,4)?)?;
    // 0x21d18
    c.emit(arm32::multiply_accumulate(C::Al,R::new(3)?,R::new(6)?,R::new(4)?,R::new(5)?)?)?;
    // 0x21d1c
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::new(3)?))?;
    // 0x21d20
    c.b(C::Ne,0x21cf4)?;
    // 0x21d24
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,210)?)?;
    // 0x21d28
    c.emit(arm32::multiply_register(C::Al,R::new(5)?,R::new(3)?,R::R0)?)?;
    // 0x21d2c
    c.b(C::Al,0x21eb0)?;
    // 0x21d30
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,12)?)?;
    // 0x21d34
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::R0,R::R2,Shift::Lsl,2)?)?;
    // 0x21d38
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,496)?)?;
    // 0x21d3c
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x21d40
    c.emit(arm32::transfer_word_register(C::Al,true,R::R2,R::R0,R::R2,Shift::Lsl,0)?)?;
    // 0x21d44
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,2)?)?;
    // 0x21d48
    c.b(C::Ne,0x21e20)?;
    // 0x21d4c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R1,88)?)?;
    // 0x21d50
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,92)?)?;
    // 0x21d54
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,15)?)?;
    // 0x21d58
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,15)?)?;
    // 0x21d5c
    c.emit(arm32::mov_shifted_register(C::Al,R::R1,R::R1,Shift::Lsr,4)?)?;
    // 0x21d60
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsr,4)?)?;
    // 0x21d64
    c.emit(arm32::multiply_register(C::Al,R::R2,R::R0,R::R1)?)?;
    // 0x21d68
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,230)?)?;
    // 0x21d6c
    c.b(C::Al,0x21eac)?;
    // 0x21d70
    c.literal(R::R2,0x21eb8)?;
    // 0x21d74
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x21d78
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,195)?)?;
    // 0x21d7c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // 0x21d80
    c.bl(0x17b0)?;
    // 0x21d84
    c.b(C::Al,0x21eb0)?;
    // 0x21d88
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R1,88)?)?;
    // 0x21d8c
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,92)?)?;
    // 0x21d90
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,15)?)?;
    // 0x21d94
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,15)?)?;
    // 0x21d98
    c.emit(arm32::mov_shifted_register(C::Al,R::R1,R::R1,Shift::Lsr,4)?)?;
    // 0x21d9c
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsr,4)?)?;
    // 0x21da0
    c.emit(arm32::multiply_register(C::Al,R::R2,R::R0,R::R1)?)?;
    // 0x21da4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,216)?)?;
    // 0x21da8
    c.b(C::Al,0x21eac)?;
    // 0x21dac
    c.emit(arm32::transfer_word(C::Al,true,R::R12,R::R1,56)?)?;
    // 0x21db0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R12,0)?)?;
    // 0x21db4
    c.b(C::Eq,0x21e18)?;
    // 0x21db8
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,12)?)?;
    // 0x21dbc
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,230)?)?;
    // 0x21dc0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0)?)?;
    // 0x21dc4
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::R0,R::R2,Shift::Lsl,2)?)?;
    // 0x21dc8
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,60)?)?;
    // 0x21dcc
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1456)?)?;
    // 0x21dd0
    c.emit(arm32::multiply_register(C::Al,R::LR,R::R2,R::new(3)?)?)?;
    // 0x21dd4
    c.emit(arm32::mov_shifted_register(C::Al,R::new(3)?,R::R2,Shift::Lsl,2)?)?;
    // 0x21dd8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x21ddc
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,4096)?)?;
    // 0x21de0
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::R0,-120)?)?;
    // 0x21de4
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,0)?)?;
    // 0x21de8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,1)?)?;
    // 0x21dec
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x21df0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::R12,Shift::Lsl,0,false)?)?;
    // 0x21df4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(5)?,15)?)?;
    // 0x21df8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,15)?)?;
    // 0x21dfc
    c.emit(arm32::mov_shifted_register(C::Al,R::new(5)?,R::new(5)?,Shift::Asr,4)?)?;
    // 0x21e00
    c.emit(arm32::mov_shifted_register(C::Al,R::R1,R::R1,Shift::Asr,4)?)?;
    // 0x21e04
    c.emit(arm32::multiply_register(C::Al,R::new(6)?,R::LR,R::new(5)?)?)?;
    // 0x21e08
    c.emit(arm32::multiply_accumulate(C::Al,R::new(5)?,R::new(6)?,R::R1,R::new(4)?)?)?;
    // 0x21e0c
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::new(5)?))?;
    // 0x21e10
    c.b(C::Cc,0x21de0)?;
    // 0x21e14
    c.b(C::Al,0x21eb0)?;
    // 0x21e18
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // 0x21e1c
    c.b(C::Al,0x21eb0)?;
    // 0x21e20
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,328)?)?;
    // 0x21e24
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,340)?)?;
    // 0x21e28
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,4096)?)?;
    // 0x21e2c
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x21e30
    c.emit(arm32::transfer_word_register(C::Al,true,R::R1,R::R0,R::R1,Shift::Lsl,0)?)?;
    // 0x21e34
    c.emit(arm32::transfer_word_register(C::Al,true,R::R2,R::R0,R::R2,Shift::Lsl,0)?)?;
    // 0x21e38
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,15)?)?;
    // 0x21e3c
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,15)?)?;
    // 0x21e40
    c.emit(arm32::mov_shifted_register(C::Al,R::R1,R::R1,Shift::Lsr,4)?)?;
    // 0x21e44
    c.emit(arm32::mov_shifted_register(C::Al,R::R2,R::R2,Shift::Lsr,4)?)?;
    // 0x21e48
    c.emit(arm32::multiply_register(C::Al,R::new(3)?,R::R2,R::R1)?)?;
    // 0x21e4c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,332)?)?;
    // 0x21e50
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,344)?)?;
    // 0x21e54
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,4096)?)?;
    // 0x21e58
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x21e5c
    c.emit(arm32::transfer_word_register(C::Al,true,R::R1,R::R0,R::R1,Shift::Lsl,0)?)?;
    // 0x21e60
    c.emit(arm32::transfer_word_register(C::Al,true,R::R2,R::R0,R::R2,Shift::Lsl,0)?)?;
    // 0x21e64
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,15)?)?;
    // 0x21e68
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,15)?)?;
    // 0x21e6c
    c.emit(arm32::mov_shifted_register(C::Al,R::R1,R::R1,Shift::Lsr,4)?)?;
    // 0x21e70
    c.emit(arm32::mov_shifted_register(C::Al,R::R2,R::R2,Shift::Lsr,4)?)?;
    // 0x21e74
    c.emit(arm32::multiply_accumulate(C::Al,R::new(5)?,R::R2,R::R1,R::new(3)?)?)?;
    // 0x21e78
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,336)?)?;
    // 0x21e7c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,348)?)?;
    // 0x21e80
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,4096)?)?;
    // 0x21e84
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x21e88
    c.emit(arm32::transfer_word_register(C::Al,true,R::R1,R::R0,R::R1,Shift::Lsl,0)?)?;
    // 0x21e8c
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::R0,R::R2,Shift::Lsl,0)?)?;
    // 0x21e90
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,15)?)?;
    // 0x21e94
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,15)?)?;
    // 0x21e98
    c.emit(arm32::mov_shifted_register(C::Al,R::R1,R::R1,Shift::Lsr,4)?)?;
    // 0x21e9c
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsr,4)?)?;
    // 0x21ea0
    c.emit(arm32::multiply_accumulate(C::Al,R::R2,R::R0,R::R1,R::new(5)?)?)?;
    // 0x21ea4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,452)?)?;
    // 0x21ea8
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,2048)?)?;
    // 0x21eac
    c.emit(arm32::multiply_register(C::Al,R::new(5)?,R::R2,R::R0)?)?;
    // 0x21eb0
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::new(5)?,Shift::Lsl,1)?)?;
    // 0x21eb4
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    c.check(0x21eb8)?;
    c.emit(0x000e_6a43)?; // diagnostic text address, not instruction
    c.check(0x21ebc)
}

/// Compute accumulated packet/plane size from three descriptor fields.
fn descriptor_words(c:&mut Code)->Result<(),String>{
    c.check(0x21ebc)?;
    // 0x21ebc
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R0,12)?)?;
    // 0x21ec0
    c.emit(arm32::transfer_multiple(C::Al,true,R::new(3)?,R::R1.bit() | R::R2.bit() | R::new(3)?.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x21ec4
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,24)?)?;
    // 0x21ec8
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // 0x21ecc
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R1,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x21ed0
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // 0x21ed4
    c.emit(arm32::bx(C::Al,R::LR))?;
    c.check(0x21ed8)
}

/// Compare primary/secondary packet offsets and log a mismatched relation.
fn validate_descriptor_address(c:&mut Code)->Result<(),String>{
    c.check(0x21ed8)?;
    // 0x21ed8
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // 0x21edc
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,16)?)?;
    // 0x21ee0
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R1))?;
    // 0x21ee4
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R0))?;
    // 0x21ee8
    c.literal(R::R1,0x21f7c)?;
    // 0x21eec
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x21ef0
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(5)?,40)?)?;
    // 0x21ef4
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,40)?)?;
    // 0x21ef8
    c.bl(0x2ec94)?;
    // 0x21efc
    c.literal(R::R1,0x21f80)?;
    // 0x21f00
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x21f04
    c.bl(0x2ec94)?;
    // 0x21f08
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,40)?)?;
    // 0x21f0c
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,40)?)?;
    // 0x21f10
    c.literal(R::R2,0x21f84)?;
    // 0x21f14
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0)?)?;
    // 0x21f18
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // 0x21f1c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x21f20
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,210)?)?;
    // 0x21f24
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x21f28
    c.bl(0x17b0)?;
    // 0x21f2c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,4)?)?;
    // 0x21f30
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x21f34
    c.emit(arm32::transfer_multiple(C::Eq,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    // 0x21f38
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(5)?,48)?)?;
    // 0x21f3c
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,48)?)?;
    // 0x21f40
    c.literal(R::R1,0x21f88)?;
    // 0x21f44
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x21f48
    c.bl(0x2ec94)?;
    // 0x21f4c
    c.literal(R::R1,0x21f80)?;
    // 0x21f50
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x21f54
    c.bl(0x2ec94)?;
    // 0x21f58
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,48)?)?;
    // 0x21f5c
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,48)?)?;
    // 0x21f60
    c.literal(R::R2,0x21f84)?;
    // 0x21f64
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // 0x21f68
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,216)?)?;
    // 0x21f6c
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(6)?,R::R0,1)?)?;
    // 0x21f70
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
    // 0x21f74
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::LR.bit(),BlockMode::IncrementAfter,true)?)?;
    // 0x21f78
    c.b(C::Al,0x17b0)?;
    c.check(0x21f7c)?;
    const ADDRESS_DIAGNOSTICS:[u32;4]=[
        0x000e_6a6e,0x000e_2788,0x000e_6a43,0x000e_6ad1,
    ];
    for value in ADDRESS_DIAGNOSTICS {c.emit(value)?;}
    c.check(0x21f8c)
}

/// Validate record count and length fields on each side of an update.
fn validate_descriptor_count(c:&mut Code)->Result<(),String>{
    c.check(0x21f8c)?;
    // 0x21f8c
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // 0x21f90
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,16)?)?;
    // 0x21f94
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R1))?;
    // 0x21f98
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R0))?;
    // 0x21f9c
    c.literal(R::R1,0x22030)?;
    // 0x21fa0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x21fa4
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(5)?,12)?)?;
    // 0x21fa8
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,12)?)?;
    // 0x21fac
    c.bl(0x2ec94)?;
    // 0x21fb0
    c.literal(R::R1,0x22034)?;
    // 0x21fb4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x21fb8
    c.bl(0x2ec94)?;
    // 0x21fbc
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,12)?)?;
    // 0x21fc0
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,12)?)?;
    // 0x21fc4
    c.literal(R::R2,0x22038)?;
    // 0x21fc8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0)?)?;
    // 0x21fcc
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // 0x21fd0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x21fd4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,224)?)?;
    // 0x21fd8
    c.emit(arm32::data_immediate(C::Ls,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x21fdc
    c.bl(0x17b0)?;
    // 0x21fe0
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,4)?)?;
    // 0x21fe4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x21fe8
    c.emit(arm32::transfer_multiple(C::Eq,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    // 0x21fec
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(5)?,20)?)?;
    // 0x21ff0
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,20)?)?;
    // 0x21ff4
    c.literal(R::R1,0x2203c)?;
    // 0x21ff8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x21ffc
    c.bl(0x2ec94)?;
    // 0x22000
    c.literal(R::R1,0x22034)?;
    // 0x22004
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x22008
    c.bl(0x2ec94)?;
    // 0x2200c
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,20)?)?;
    // 0x22010
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,20)?)?;
    // 0x22014
    c.literal(R::R2,0x22038)?;
    // 0x22018
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // 0x2201c
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,230)?)?;
    // 0x22020
    c.emit(arm32::data_immediate(C::Ls,Op::Mov,R::new(6)?,R::R0,1)?)?;
    // 0x22024
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
    // 0x22028
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::LR.bit(),BlockMode::IncrementAfter,true)?)?;
    // 0x2202c
    c.b(C::Al,0x17b0)?;
    c.check(0x22030)?;
    const COUNT_DIAGNOSTICS:[u32;4]=[
        0x000e_6b44,0x000e_2788,0x000e_6a43,0x000e_6b97,
    ];
    for value in COUNT_DIAGNOSTICS {c.emit(value)?;}
    c.check(0x22040)
}

pub fn fragment()->Result<Vec<u8>,String>{
    let mut code=Code::new(START);
    frame_allocation(&mut code)?;
    descriptor_words(&mut code)?;
    validate_descriptor_address(&mut code)?;
    validate_descriptor_count(&mut code)?;
    initialize::emit_next(&mut code)?;
    input::emit_next(&mut code)?;
    commands::emit_next(&mut code)?;
    state::emit_next(&mut code)?;
    logging::emit_next(&mut code)?;
    dispatch::emit(&mut code)?;
    modes::emit(&mut code)?;
    copy::emit(&mut code)?;
    helpers::emit(&mut code)?;
    capture::emit(&mut code)?;
    registers::emit(&mut code)?;
    irq::emit(&mut code)?;
    irq_init::emit(&mut code)?;
    events::emit(&mut code)?;
    irq_shutdown::emit(&mut code)?;
    irq_channels::emit(&mut code)?;
    handshake::emit(&mut code)?;
    controller::emit(&mut code)?;
    status::emit(&mut code)?;
    queue_control::emit(&mut code)?;
    queue_extension::emit(&mut code)?;
    controller_frame::emit(&mut code)?;
    frame_policy::emit(&mut code)?;
    plane_dma::emit(&mut code)?;
    transfer_sequence::emit(&mut code)?;
    active_port::emit(&mut code)?;
    frame_release::emit(&mut code)?;
    callbacks_release::emit(&mut code)?;
    register_snapshot::emit(&mut code)?;
    irq_readback::emit(&mut code)?;
    priority_irq::emit(&mut code)?;
    dma_completion::emit(&mut code)?;
    capture_setup::emit(&mut code)?;
    format_policy::emit(&mut code)?;
    submit_release::emit(&mut code)?;
    control_dispatch::emit(&mut code)?;
    control_io::emit(&mut code)?;
    controller_start::emit(&mut code)?;
    event_router::emit(&mut code)?;
    codec_routes::emit(&mut code)?;
    frame_messages::emit(&mut code)?;
    buffer_cleanup::emit(&mut code)?;
    class_handlers::emit(&mut code)?;
    codec_categories::emit(&mut code)?;
    request_dispatch::emit(&mut code)?;
    class_continuation::emit(&mut code)?;
    resource_access::emit(&mut code)?;
    command_registers::emit(&mut code)?;
    encoder_init::emit(&mut code)?;
    packed_bits::emit(&mut code)?;
    event_publish::emit(&mut code)?;
    event_monitor::emit(&mut code)?;
    ring_helpers::emit(&mut code)?;
    ring_slot_access::emit(&mut code)?;
    dma_initialize::emit(&mut code)?;
    mmio_poll::emit(&mut code)?;
    irq_banks::emit(&mut code)?;
    irq_router::emit(&mut code)?;
    image_planes::emit(&mut code)?;
    codec_register_serializer::emit(&mut code)?;
    buffer_ctrl_commands::emit(&mut code)?;
    post_reset_banks::emit(&mut code)?;
    input_capabilities::emit(&mut code)?;
    mmio_diagnostics::emit(&mut code)?;
    codec_table_setup::emit(&mut code)?;
    codec_resources::emit(&mut code)?;
    codec_class_prefix::emit(&mut code)?;
    codec_class_cases::emit(&mut code)?;
    event_classification::emit(&mut code)?;
    callback_helpers::emit(&mut code)?;
    event_type_resolve::emit(&mut code)?;
    codec_irq_frames::emit(&mut code)?;
    codec_frame_update::emit(&mut code)?;
    controller_stream_ops::emit(&mut code)?;
    controller_commit::emit(&mut code)?;
    codec_stream_init::emit(&mut code)?;
    exponent_and_alloc::emit(&mut code)?;
    ring_queue::emit(&mut code)?;
    frame_dispatch_init::emit(&mut code)?;
    ring_descriptor_ops::emit(&mut code)?;
    ring_queue_update::emit(&mut code)?;
    controller_defaults::emit(&mut code)?;
    Ok(code.bytes)
}

pub fn verify_reference(reference:&[u8])->Result<usize,String>{
    let actual=fragment()?;
    let expected=reference.get(FILE_OFFSET..FILE_OFFSET+actual.len())
        .ok_or_else(||"reference Venus image shorter than needed".to_string())?;
    if actual!=expected {
        let off=actual.iter().zip(expected).position(|(a,b)|a!=b).unwrap();
        return Err(format!("Venus Agent 20-03 mismatch at file offset 0x{:x}, runtime 0x{:x}",FILE_OFFSET+off,START+off));
    }
    Ok(actual.len())
}

#[cfg(test)] mod tests {
    use super::*;
    #[test] fn generated_region_exact_layout(){
        assert_eq!(END-START,0x153a4);
        assert_eq!(fragment().unwrap().len(),END-START);
        assert_eq!(FILE_OFFSET,0x25c78);
    }
    #[test]fn adc_zero_and_carry_are_checked(){
        assert_eq!(encode_adc_zero(C::Al,R::new(3).unwrap(),R::new(3).unwrap()).unwrap(),0xe2a3_3000);
        assert!(encode_adc_zero(C::Al,R::PC,R::R0).is_err());
    }
    #[test] fn smulbb_has_checked_operands_and_architecture_encoding(){
        assert_eq!(encode_smulbb(C::Al,R::R0,R::R1,R::R0).unwrap(),0xe160_0081);
        assert!(encode_smulbb(C::Eq,R::PC,R::R0,R::R1).is_err());
    }
    #[test] fn long_multiply_encodings_are_checked(){
        assert_eq!(encode_long_multiply(C::Al,false,R::new(3).unwrap(),R::new(7).unwrap(),R::R1,R::LR).unwrap(),0xe087_3e91);
        assert!(encode_long_multiply(C::Al,false,R::R1,R::R1,R::new(7).unwrap(),R::LR).is_err());
        assert!(encode_long_multiply(C::Al,true,R::R2,R::new(7).unwrap(),R::PC,R::R1).is_err());
    }
    #[test] fn named_jump_table_is_data(){
        let blob=fragment().unwrap();
        let from=0x21c98-START;
        let words=(from..from+48).step_by(4).map(|i|u32::from_le_bytes(blob[i..i+4].try_into().unwrap())).collect::<Vec<_>>();
        assert_eq!(words,[0x21d4c,0x21cc8,0x21d30,0x21d70,0x21d4c,0x21d88,
                          0x21d70,0x21d4c,0x21cc8,0x21cc8,0x21cc8,0x21dac]);
    }
}

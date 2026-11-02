// SPDX-License-Identifier: GPL-3.0-only
// Hexagon V60 session validation and controller dispatch, ELF file
// [0x446ee8,0x447028), VMA [0x1e400ee8,0x1e401028).
// LLVM 6 does not support the public `r3 = framekey` mnemonic; four bytes
// are reserved only for layout while the checked ISA field encoder supplies
// this instruction separately. No OEM opcode is inserted into source.
.text
.globl modem_late_aux_session_validation
modem_late_aux_session_validation:
.set ext_1e401028, modem_late_aux_session_validation + (320)
.set ext_1e4007a0, modem_late_aux_session_validation + (-1864)
.set ext_1e403c60, modem_late_aux_session_validation + (11640)
.set ext_1e404148, modem_late_aux_session_validation + (12896)
.set ext_1e408e40, modem_late_aux_session_validation + (32600)
.set ext_1e403be8, modem_late_aux_session_validation + (11520)
.set ext_1e401078, modem_late_aux_session_validation + (400)
// A temporary 4-byte zero slot: replaced by `encode_v60_framekey(3)`
// only AFTER checked ISA encoding. Not included as generated zero padding.
.space 4
{
  r2 = memw(r30+#4)
}
{
  call ext_1e401028
  r0 = xor(r2,r3)
}
{
  r18 = r0
  if (!cmp.eq(r18.new,#0)) jump:t .Lvalid
}
{
  r4 = memw(##507617104)
  if (cmp.eq(r4.new,#0)) jump:nt .Linvalid_log_done
}
{
  r1:0 = combine(##507576347,#3)
  r2 = #653
}
{
  r3 = ##507576599
  callr r4
}
.Linvalid_log_done:
{
  call ext_1e4007a0
  r0 = ##507576599
}
{
  r0 = #0
  jump .Ldone
}
.Lvalid:
{
  p0 = bitsset(r23,r21)
  if (p0.new) jump:nt .Lresource_type
}
{
  p0 = cmp.eq(r23,#0)
  if (!p0.new) jump:nt .Lerror_or_special
}
{
  if (p0.new) r1:0 = combine(r17,r16)
  p0 = cmp.eq(r22,#0)
  if (!p0.new) jump:nt .Lerror_or_special
}
{
  call ext_1e403c60
  r3:2 = combine(#0,r18)
}
{
  jump .Lhandle_result
  memw(r29+#12) = r18
}
.Lresource_type:
{
  p0 = cmpb.eq(r19,#253)
  if (p0.new) jump:nt .Ldirect_lookup
}
{
  p0 = cmp.eq(r22,#254)
  if (p0.new) r1:0 = combine(r17,r16)
  if (p0.new) jump:nt .Lnext_check
}
{
  p0 = cmp.eq(r22,#255)
  if (!p0.new) jump:nt .Lerror_or_special
}
.Lnext_check_before:
{
  r18 = memw(r18+#0)
  if (cmp.eq(r18.new,#0)) jump:nt .Llog_final
}
.Ldirect_lookup:
{
  call ext_1e403c60
  r1:0 = combine(r17,r16)
  r3:2 = combine(#0,r18)
}
{
  p0 = cmp.eq(r0,#0)
  if (p0.new) jump:nt .Lnext_check_before
  if (!p0.new) memw(r29+#12) = r18
}
{
  jump .Lcall_after_handle
}
.Lnext_check:
{
  r3 = add(r29,#12)
}
{
  r4 = #0
  r2 = r18
}
{
  call ext_1e404148
}
{
  jump .Lhandle_result
}
.Lerror_or_special:
{
  call ext_1e408e40
}
{
  r1:0 = combine(r17,r16)
  r2 = memub(r18+#136)
}
{
  p0 = tstbit(r2,#0)
  if (!p0.new) jump:t .Llog_error
}
{
  r3 = add(r29,#12)
}
{
  r2 = ##507588612
  r4 = #0
}
{
  call ext_1e403be8
}
{
  jump .Lhandle_result
}
.Llog_error:
{
  call ext_1e403c60
  r3:2 = combine(#0,r18)
}
{
  memw(r29+#12) = r18
}
.Lhandle_result:
{
  p0 = cmp.eq(r0,#0)
  if (p0.new) jump:nt .Llog_final
  if (!p0.new) r18 = memw(r29+#12)
}
.Lcall_after_handle:
{
  call ext_1e401078
  r0 = r18
  r1 = memw(r0+#4)
}
{
  jump .Ldone
}
.Llog_final:
{
  r4 = memw(##507617104)
  if (cmp.eq(r4.new,#0)) jump:nt .Lfinal_log_done
}
{
  r1:0 = combine(##507576347,#3)
  r2 = #708
}
{
  callr r4
}
.Lfinal_log_done:
{
  call ext_1e4007a0
  r3 = ##507576639
  memw(r29+#0) = r16
}
{
  r0 = ##507576639
  memw(r29+#0) = r16
}
.Ldone:
{
  r0 = #0
}
{
  r17:16 = memd(r29+#40)
  r19:18 = memd(r29+#32)
}
{
  r21:20 = memd(r29+#24)
  r23:22 = memd(r29+#16)
}
{
  dealloc_return
}

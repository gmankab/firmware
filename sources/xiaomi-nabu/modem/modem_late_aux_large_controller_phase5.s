// SPDX-License-Identifier: GPL-3.0-only
// Session arithmetic and error-branch prelude, file [0x4479a8,0x4479d0).
.text
.globl modem_late_aux_large_controller_phase5
modem_late_aux_large_controller_phase5:
.set original_session_start, modem_late_aux_large_controller_phase5 - 0x284
.set resource_callback, original_session_start + 0x7e8c
.set log_to_fallback, original_session_start + 0x2c8
{call resource_callback}
{
  r2 = sub(r24,r18)
  r0 = add(r23,r18)
  r1 = #0
}
{
  r2 = r24
  r3 = memw(r29+#84)
}
{ r2 += add(r23,#-1) }
{
  p0 = r3
  if (!p0.new) jump:t log_to_fallback
  if (!p0.new) memw(r29+#8) = r24
}

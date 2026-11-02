// SPDX-License-Identifier: GPL-3.0-only
// Hexagon V60 codec record scanner, file [0x447ae4,0x447b60).
// The original controller session function begins at file 0x447724.
.text
.globl modem_late_aux_large_controller_phase9
modem_late_aux_large_controller_phase9:
.set session_base, modem_late_aux_large_controller_phase9 - 0x3c0
.set registry_unavailable, session_base + 0x5fc
.set no_codec_records, session_base + 0x660
.set begin_loop, session_base + 0x3f4
.set skip_invalid_entry, session_base + 0x430
.set callback_fallback, session_base + 0x43c
.set callback_finished, session_base + 0x454
.set log_callback, session_base + 0x136c
{r2 = memw(r16+#72)}
{
  p0 = cmp.eq(r2,#0)
  if (p0.new) jump:nt registry_unavailable
  r2 = memuh(r21+#44)
}
{
  if (!p0.new) r6 = add(r21,#20)
  if (!p0.new) r25 = #0
  p0 = cmp.eq(r2,#0)
  if (!p0.new) jump:t .Lhas_entries
}
{ r23 = #0; jump no_codec_records }
.Lhas_entries:
{r3 = #1; r22 = #-1}
{
  r5 = sub(r2,r25)
  r25 = r3
  r4 = memw(r21+#28)
}
{
  loop0(begin_loop,r5)
  r3 = add(r6,r4)
}
begin_loop:
{
  r4 = memw(r3+#-20)
  if (!cmp.eq(r4.new,#4)) jump:t skip_invalid_entry
}
{
  r18 = r27
  r2 = memw(r29+#84)
  memw(r29+#64) = r6
}
{
  p0 = r2
  memw(r29+#80) = r26
  r23 = memw(r3+#-16)
}
{
  if (!p0) jump:nt callback_fallback
  r27 = memw(r3+#-4)
  r26 = memw(r3+#0)
}
{
  call log_callback
  r2 = #316
  r3 = ##507577202
}
{ jump callback_finished }
skip_invalid_entry:
{r3 = add(r3,#32)
 nop }:endloop0

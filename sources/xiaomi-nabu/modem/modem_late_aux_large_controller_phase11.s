// SPDX-License-Identifier: GPL-3.0-only
// Source-assembled V60 session resource/diagnostic callbacks [0x447bc4,0x447c74).
.text
.globl modem_late_aux_large_controller_phase11
modem_late_aux_large_controller_phase11:
.set session_base, modem_late_aux_large_controller_phase11 - 0x4a0
.set log_callback, session_base + 0x136c
.set fallback_log, session_base + 0x4654
.set diagnostic_console, session_base - 0xf84
.set cleanup, session_base + 0x37c
.set next_state, session_base + 0x568
.set missing_object, session_base + 0x4d8
.set post_log, session_base + 0x4f0
.set no_controller_log, session_base + 0x53c
{r2 += add(r4,#-1)}
{
  r24 = and(r2,r3)
  r2 = memw(r29+#84)
}
{
  p0 = r2
  if (!p0.new) jump:t missing_object
  if (!p0.new) memw(r29+#8) = r27
  if (!p0.new) memw(r29+#0) = r23
}
{
  r2 = #323
  r3 = ##507577248
  memw(r29+#8) = r27
}
{
  call log_callback
  memw(r29+#4) = r22
  memw(r29+#0) = r23
}
{jump post_log}
missing_object:
{
  r1:0 = combine(##507576841,#2)
  r2 = #323
}
{
  r3 = ##507577248
  memw(r29+#4) = r22
}
{call fallback_log}
post_log:
{
  r1:0 = combine(r24,#0)
  r5:4 = combine(r23,r20)
  r3:2 = combine(#1,#1)
  r6 = memw(r16+#48)
}
{
  r27 = r18
  callr r6
  r24 = r26
  memw(r29+#0) = r20
}
{
  r22 = r0
  if (!cmp.eq(r22.new,#-1)) jump:t next_state
}
{r18 = memw(r29+#60)}
{
  r4 = memw(r19+#0)
  if (cmp.eq(r4.new,#0)) jump:nt no_controller_log
}
{
  r1:0 = combine(##507576841,#3)
  r2 = #327
}
{
  r3 = ##507577286
  memw(r29+#4) = r23
  memw(r29+#0) = r18
}
{callr r4}
no_controller_log:
{
  r0 = ##507577286
  memw(r29+#4) = r23
  memw(r29+#0) = r18
}
{call diagnostic_console}
{jump cleanup}

// SPDX-License-Identifier: GPL-3.0-only
// Hexagon V60 resource result checks and callback setup, file [0x447c74,0x447d20).
.text
.globl modem_late_aux_large_controller_phase12
modem_late_aux_large_controller_phase12:
.set session_base, modem_late_aux_large_controller_phase12 - 0x550
.set fallback_log, session_base + 0x4654
.set controller_log, session_base + 0x136c
.set resource_start, session_base + 0x7aec
.set callback_fallback, session_base + 0x590
.set callback_join, session_base + 0x5ac
.set callback_failed, session_base + 0x5e8
.set skip_warning, session_base + 0x614
.set scan_loop, session_base + 0x3e0
{
  r1:0 = combine(##507576841,#2)
  r2 = #318
  r22 = add(r21,r23)
}
{call fallback_log}
{r27 = r18}
{
  r23 = add(r22,#12)
  r2 = memw(r29+#84)
}
{
  p0 = r2
  if (!p0.new) jump:t callback_fallback
}
{
  r2 = #329
  r3 = ##507577320
  memw(r29+#0) = r23
}
{call controller_log}
{
  jump callback_join
  r26 = memw(r29+#80)
}
callback_fallback:
{
  r1:0 = combine(##507576841,#2)
  r2 = #329
}
{
  call fallback_log
  r3 = ##507577320
  memw(r29+#0) = r23
}
{r26 = memw(r29+#80)}
callback_join:
{
  call resource_start
  r1:0 = combine(##507577334,r23)
  r2 = #11
}
{
  p0 = cmp.eq(r0,#0)
  if (!p0.new) jump:t callback_failed
}
{memw(r17+#8) = #1}
{r2 = memw(r29+#84)}
{
  p0 = r2
  if (!p0.new) r3 = ##507577346
  if (!p0.new) jump:t skip_warning
}
{
  call controller_log
  r2 = #332
  r3 = ##507577346
}
callback_failed:
{
  r3 = add(r25,#1)
  r2 = memuh(r21+#44)
  r6 = memw(r29+#64)
}
{
  p0 = cmp.gtu(r2,r25)
  if (p0.new) jump:t scan_loop
}

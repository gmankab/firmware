// SPDX-License-Identifier: GPL-3.0-only
// Controller registry-mode diagnostic selection, file [0x447aac,0x447ae4).
.text
.globl modem_late_aux_large_controller_phase8
modem_late_aux_large_controller_phase8:
.set session_base, modem_late_aux_large_controller_phase8 - 0x388
.set controller_log, session_base + 0x136c
.set fallback_log, session_base + 0x4654
.set diagnostic_else, session_base + 0x3b0
.set controller_next, session_base + 0x3c0
{r2 = memw(r29+#84)}
{
  p0 = r2
  if (!p0.new) r3 = ##507577156
  if (!p0.new) jump:t diagnostic_else
}
{
  call controller_log
  r2 = #302
  r3 = ##507577156
}
{ jump controller_next }
diagnostic_else:
{
  r1:0 = combine(##507576841,#2)
  r2 = #302
}
{call fallback_log}

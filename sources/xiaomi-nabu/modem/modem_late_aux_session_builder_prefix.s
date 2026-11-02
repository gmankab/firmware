// SPDX-License-Identifier: GPL-3.0-only
// The next controller-session function: save callee state, invoke the
// allocator callback when present, and follow the original invalid-state
// diagnostic path. The successful path and final return follow this fragment.
.text
.globl modem_late_aux_session_builder_prefix
modem_late_aux_session_builder_prefix:
.set allocator_available,modem_late_aux_session_builder_prefix+0x70
.set skip_logger,modem_late_aux_session_builder_prefix+0x5c
.set shared_error_exit,modem_late_aux_session_builder_prefix+0x964
.set error_report,modem_late_aux_session_builder_prefix-0x2690
{
  memd(r29+#-16) = r17:16
  allocframe(#104)
}
{
  r21:20 = combine(r0,r2)
  r19 = r1
  memd(r29+#80) = r21:20
  memd(r29+#88) = r19:18
}
{
  p0 = cmp.eq(r20,#-1)
  if (!p0.new) r0 = #208
  memd(r29+#64) = r25:24
  memd(r29+#56) = r27:26
}
{
  r26 = ##507617104
  r25 = r3
  memd(r29+#72) = r23:22
}
{
  if (!p0) jump:nt allocator_available
  if (!p0) r2 = memw(r26+#36)
  memw(r29+#44) = #0
}
{
  r4 = memw(r26+#0)
  if (cmp.eq(r4.new,#0)) jump:nt skip_logger
}
{
  r1:0 = combine(##507576841,#3)
  r2 = #621
}
{
  r3 = ##507576854
  callr r4
}
skip_logger:
{
  call error_report
  r0 = ##507576854
  r17:16 = combine(#-1,#0)
}
{jump shared_error_exit}

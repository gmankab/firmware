// SPDX-License-Identifier: GPL-3.0-only
// Run the installed session allocator and report a null allocation. This
// fragment is contiguous with the session prologue; the successful path
// continues at file offset 0x448edc.
.text
.globl modem_late_aux_session_builder_allocation
modem_late_aux_session_builder_allocation:
.set allocated_state,modem_late_aux_session_builder_allocation+0x3c
.set skip_logger,modem_late_aux_session_builder_allocation+0x2c
.set shared_error_exit,modem_late_aux_session_builder_allocation+0x8f4
.set error_report,modem_late_aux_session_builder_allocation-0x2700
{callr r2}
{
  r16 = r0
  if (!cmp.eq(r16.new,#0)) jump:t allocated_state
}
{
  r4 = memw(r26+#0)
  if (cmp.eq(r4.new,#0)) jump:nt skip_logger
}
{
  r1:0 = combine(##507576841,#3)
  r2 = #622
}
{
  r3 = ##507576878
  callr r4
}
skip_logger:
{
  call error_report
  r0 = ##507576878
  r17 = #-1
}
{jump shared_error_exit}

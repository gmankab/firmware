// SPDX-License-Identifier: GPL-3.0-only
// Initialize an allocated session's descriptor head, count and callback
// fields, then validate the second runtime allocator before returning an
// error. All calls are symbolic; the normal path continues at 0x448f64.
.text
.globl modem_late_aux_session_builder_init
modem_late_aux_session_builder_init:
.set state_ready,modem_late_aux_session_builder_init+0x88
.set skip_logger,modem_late_aux_session_builder_init+0x68
.set report_error,modem_late_aux_session_builder_init-0x273c
.set prepare_state,modem_late_aux_session_builder_init+0x66d4
.set destroy_state,modem_late_aux_session_builder_init-0x1a2c
.set shared_error_exit,modem_late_aux_session_builder_init+0x8c4
{
  call prepare_state
  r1:0 = combine(#0,r16)
  r2 = #200
}
{
  r2 = add(r16,#168)
  r1:0 = combine(#0,#0)
  memw(r16+#172) = r2.new
}
{
  r2 = add(r16,#176)
  memw(r16+#176) = #0
  memd(r16+#200) = r1:0
}
{memw(r16+#180) = r2}
{
  r2 = memw(r26+#36)
  r0 = memw(r18=##507617080)
}
{callr r2}
{
  r17 = r0
  if (!cmp.eq(r17.new,#0)) jump:t state_ready
}
{
  r4 = memw(r26+#0)
  if (cmp.eq(r4.new,#0)) jump:nt skip_logger
}
{
  r1:0 = combine(##507576841,#3)
  r2 = memw(r18+#0)
  memw(r29+#0) = r2.new
}
{
  r3 = ##507578157
  callr r4
  r2 = #628
}
skip_logger:
{
  r0 = ##507578157
  r2 = memw(r18+#0)
  memw(r29+#0) = r2.new
}
{call report_error}
{
  call destroy_state
  r16 = #0
  r0 = r16
}
{jump shared_error_exit}

// SPDX-License-Identifier: GPL-3.0-only
// Validate the packed controller-record signature against the kernel's
// public device type, invoke the installed error callback if necessary, and
// return an error code. The valid-input branch continues in the next source
// fragment (at file offset 0x448b34), not inside this partial function.
.text
.globl modem_late_aux_record_guard
modem_late_aux_record_guard:
.set valid_header,modem_late_aux_record_guard+0x64
.set no_logger,modem_late_aux_record_guard+0x50
.set error_report,modem_late_aux_record_guard-0x2330
{
  r16 = r0
  memd(r29+#-16) = r17:16
  allocframe(#16)
}
{
  r2 = memub(r16+#2)
  r3 = memub(r16+#3)
}
{
  r2 |= asl(r3,#8)
  r4 = memub(r16+#1)
  r5 = memub(r16+#0)
}
{r5 |= asl(r4,#8)}
{
  r5 |= asl(r2,#16)
  r2 = ##1179403647
}
{
  r2 = ##507617104
  p0 = cmp.eq(r5,r2)
  if (p0.new) jump:nt valid_header
}
{
  r4 = memw(r2+#0)
  if (cmp.eq(r4.new,#0)) jump:nt no_logger
}
{
  r1:0 = combine(##507576841,#3)
  r2 = #138
}
{
  r3 = ##507578583
  callr r4
}
no_logger:
{
  call error_report
  r0 = ##507578583
}
{
  r0 = #-1
  r17:16 = memd(r29+#8)
  dealloc_return
}

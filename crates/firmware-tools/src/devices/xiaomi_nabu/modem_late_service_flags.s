// SPDX-License-Identifier: GPL-3.0-only
// Hexagon V60 service flag/query helpers, modem file [0x7c3a8,0x7c40c).
// Three distinct callable blocks, including one deliberate guard trap.
.text
.globl modem_late_service_flag_helpers
modem_late_service_flag_helpers:
{
  r2 = memub(##3229939714)
}
{ p0 = tstbit(r2,#1); if (p0.new) jump:t .Lflag_return }
{ r3:2 = combine(##-351256572,#6) }
{ jumpr r31; memw(r3+#0) &= r2 }
.Lflag_return:
{ jumpr r31 }
{ brkpt }
.globl modem_late_flag_set
modem_late_flag_set:
{ r2 = ##-351255552; r3 = #16000 }
{ jumpr r31; r4 = memw(r2+#-1012); memw(r2+#-1012) = r3 }
.globl modem_late_flag_get
modem_late_flag_get:
{
  p0 = cmp.eq(r0,#0)
  if (!p0.new) r2 = ##-351255560
  if (p0.new) jump:nt .Lget_return
}
{ r2 = memw(r2+#-1012) }
{ r2 = sub(#-1,r2) }
{ r2 = and(r2,#1); jumpr r31; memw(r0+#0)=r2.new }
.Lget_return:
{ jumpr r31 }

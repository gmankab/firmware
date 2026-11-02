// SPDX-License-Identifier: GPL-3.0-only
// Hexagon V60 late-baseband service MMIO configuration from original logic.
// Source controls C2 setbit, current-status checks and platform notification.
// File [0x7c324,0x7c3a8). The last 4 bytes are an explicit brkpt guard.
.text
.globl modem_late_service_mmio
modem_late_service_mmio:
.set resource_probe, modem_late_service_mmio + (0xd6c8c - 0x324)
.set resource_emit, modem_late_service_mmio + (0xd6cac - 0x324)
.set late_cleanup, modem_late_service_mmio - (0x32da8 + 0x324)
{
 r1:0 = combine(#0,##3249533728)
 r5:4 = combine(#0,#0)
 allocframe(#8)
}
{ call resource_probe; r2=add(r29,#0); memd(r29+#0)=r5:4 }
{ call resource_emit }
{ r0 = ##-1045433568 }
{ call late_cleanup }
{ r0 = ##-1032742170 }
{ r2 = memub(##3229939714) }
{ p0 = tstbit(r2,#0); if (p0.new) jump:t .Lexit }
{
 r2 = ##-351255560
 r3 = #22400
 r4 = #25600
}
{ r5 = memw(r2+#-996); memw(r2+#-996) = r3 }
{ r3 = memw(r2+#-1012) }
{ r3 = and(r3,#3) }
{ r3 = setbit(r3,#2); memw(r2+#-1012) = r3.new }
{ r3 = memw(r2+#-1000); memw(r2+#-1000) = r4 }
{ r3 = memw(r2+#-1012) }
{ r3 = and(r3,#6) }
{ r3 = setbit(r3,#0); jumpr r31; memw(r2+#-1012) = r3.new }
.Lexit:
{ jumpr r31 }
{ brkpt }

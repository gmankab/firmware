// SPDX-License-Identifier: GPL-3.0-only
// Hexagon V60 service state reset and controller notification helper in
// the late modem executable. File [0x7c2ec,0x7c324), 56 source bytes.
.text
.globl modem_late_service_probe
modem_late_service_probe:
.set service_load, modem_late_service_probe + (0xd6c8c - 0x2ec)
.set service_flush, modem_late_service_probe + (0xd6cac - 0x2ec)
.set service_finalize, modem_late_service_probe + (0x9e74 - 0x2ec)
{
  r1:0 = combine(#0,##3249533728)
  r5:4 = combine(#0,#0)
  allocframe(#8)
}
{
  call service_load
  r2 = add(r29,#0)
  memd(r29+#0) = r5:4
}
{ call service_flush }
{ r0 = ##-1045433568 }
{ r0 = ##-1062350004 }
{ r1 = ##-1064762644 }
{ call service_finalize }
{ dealloc_return }

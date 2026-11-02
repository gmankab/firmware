// SPDX-License-Identifier: GPL-3.0-only
// Independent Hexagon V60 reconstruction of modem late-region controller
// callback, file [0x7c040, 0x7c0a0), ELF VMA [0xc0890040,0xc08900a0).
// All absolute values are named controller addresses / assembly immediates.
.text
.globl modem_late_controller_callback
modem_late_controller_callback:
.set platform_log_callback, modem_late_controller_callback + (0x14a994 - 0x40)
.set conditional_cleanup_callback, modem_late_controller_callback - (0x32da8 + 0x40)
{
  r0 = ##33554436
  allocframe(#0)
}
{ call platform_log_callback
  r1 = ##-1045433584 }
{
  r0 = ##-1032742230
  p0 = cmp.eq(r0,#0)
  if (!p0.new) jump:nt .Lcleanup
}
{ r1 = memw(##3249533712)
  if (cmp.eq(r1.new,#0)) jump:nt .Lcleanup }
{ r3 = ##-1064972288 }
{ r0 = #3; r2 = memw(r1+#4) }
{ r5:4 = combine(##260,#0) }
{ r2 = #288
  r6 = memw(r2+#36) }
{ callr r6 }
{
  if (!p0.new) r0 = ##-1032742220
  p0 = cmp.eq(r0,#0); if (p0.new) jump:t .Lreturn
}
.Lcleanup:
{ call conditional_cleanup_callback }
.Lreturn:
{ dealloc_return }

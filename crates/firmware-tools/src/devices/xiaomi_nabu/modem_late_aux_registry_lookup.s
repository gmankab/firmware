// SPDX-License-Identifier: GPL-3.0-only
// Hexagon V60 auxiliary controller registry lookup and notification,
// modem file [0x446420,0x446488); ELF vaddr [0x1e400420,0x1e400488).
// Walk a bounded linked list of runtime objects, compare object identity,
// invoke registered notification callback and return a descriptor handle.
.text
.globl modem_late_aux_registry_lookup
modem_late_aux_registry_lookup:
.set lookup_notify, modem_late_aux_registry_lookup + (0x4a8 - 0x128)
{
 r2 = memw(##507617068)
 allocframe(#16)
}
{
 p0 = cmp.eq(r2,#0)
 if (p0.new) jump:nt .Lempty
 r16 = r0
 memd(r29+#8) = r17:16
}
.Lwalk:
{ p0 = cmp.eq(r2,r16); if (p0.new) jump:nt .Lmatch }
{ r2 = memw(r2+#0); if (cmp.eq(r2.new,#0)) jump:nt .Lempty }
{ jump .Lwalk }
.Lmatch:
{ r2 = memw(r16+#24); if (!cmp.eq(r2.new,#0)) jump:t .Lfound }
.Lempty:
{ r4 = memw(##507617104); if (cmp.eq(r4.new,#0)) jump:nt .Lnotify }
{ r1:0 = combine(##507576347,#3); r2 = #153 }
{ callr r4; r3 = ##507576758; memw(r29+#0) = r16 }
.Lnotify:
{ call lookup_notify; r0 = ##507576758; memw(r29+#0) = r16 }
{ r16 = #0 }
.Lfound:
{ r0 = r16 }
{ r17:16 = memd(r29+#8); dealloc_return }

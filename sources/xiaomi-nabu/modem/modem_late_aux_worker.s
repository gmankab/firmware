// SPDX-License-Identifier: GPL-3.0-only
// Hexagon V60 later modem auxiliary controller worker, file [0x4463b4,0x446420).
// Checks global worker state, allocates a service object, sends an MMIO flag,
// releases the global service lock, then returns the object or -1.
.text
.globl modem_late_aux_worker
modem_late_aux_worker:
.set worker_enter, modem_late_aux_worker + (0x62d8 - 0xbc)
.set worker_register, modem_late_aux_worker + (0x128 - 0xbc)
.set controller_lock, modem_late_aux_worker + (0x190 - 0xbc)
.set controller_update, modem_late_aux_worker + (0x198 - 0xbc)
.set worker_leave, modem_late_aux_worker + (0x62e0 - 0xbc)
{ r16 = r0; memd(r29+#-16) = r17:16; allocframe(#8) }
{ call worker_enter; r0 = ##507612016 }
{ call worker_register; r0 = r16 }
{ r16 = r0; if (!cmp.eq(r16.new,#0)) jump:t .Lready }
{ r16 = #-1 }
{ jump .Ldone }
.Lready:
{ call controller_lock; r2 = #2; memb(##507617064) = r2.new }
{ call controller_update }
{ r1:0 = combine(#1,r16)
  r16 = #0
  memw(r16+#24) -= #1
}
{ call controller_lock; memb(##507617064) = r16 }
{ call worker_leave; r0 = ##507612016 }
.Ldone:
{ r0 = r16; r17:16 = memd(r29+#0); dealloc_return }

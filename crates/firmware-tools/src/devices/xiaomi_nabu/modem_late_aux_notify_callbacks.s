// SPDX-License-Identifier: GPL-3.0-only
// Auxiliary encoded service callbacks for modem.mbn file [0x447448,0x4474b0).
// Program vaddr [0x1e401448,0x1e4014b0) in auxiliary executable PT_LOAD.
// Typed Hexagon V60 instruction groups; no OEM words are embedded.
.text
.globl modem_late_aux_notify_callbacks
modem_late_aux_notify_callbacks:
.set notify_initial_handler, modem_late_aux_notify_callbacks + 0x2fd0
.set notify_relay_handler, modem_late_aux_notify_callbacks + 0x2ff8
{ r17:16 = combine(r1,r0)
  memd(r29+#-16) = r17:16; allocframe(#16) }
{ memd(r29+#0) = r19:18
  r2 = memub(r17+#136) }
{ p0 = tstbit(r2,#6)
  if (!p0.new) r18 = add(r17,##168)
  if (p0.new) jump:t .Lscan_list }
{ call notify_initial_handler
  r1:0 = combine(r16,r18) }
{ if (p0.new) r1:0 = combine(r16,r18)
  p0 = cmp.eq(r0,#0); if (!p0.new) jump:t .Lexit }
{ call notify_relay_handler }
{ call notify_relay_handler
  r0 = add(r16,#176)
  r1 = r17 }
.Lscan_list:
{ r17 = memw(r17+#124)
  if (cmp.eq(r17.new,#0)) jump:nt .Lexit }
.Literate:
{ r1 = memw(r17+#4) }
{ p0 = cmp.eq(r1,#0)
  if (!p0.new) r0 = add(r16,#0) }
{ if (!p0) call modem_late_aux_notify_callbacks }
{ r17 = memw(r17+#0)
  if (!cmp.eq(r17.new,#0)) jump:t .Literate }
.Lexit:
{ r17:16 = memd(r29+#8); r19:18 = memd(r29+#0) }
{ dealloc_return }

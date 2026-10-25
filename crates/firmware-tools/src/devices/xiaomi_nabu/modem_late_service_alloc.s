// SPDX-License-Identifier: GPL-3.0-only
// Hexagon V60 late modem service descriptor allocator, file [0x7c40c,0x7c4b0).
// Creates a controller allocation descriptor and attaches it to the global
// list under a checked lock; no raw OEM opcodes are embedded here.
.text
.globl modem_late_service_alloc
modem_late_service_alloc:
.set platform_alloc_probe, modem_late_service_alloc + (0x149788 - 0x64)
.set alloc_fail_log, modem_late_service_alloc - (0x33150 + 0x64)
.set alloc_create, modem_late_service_alloc + (0x27bd0 - 0x64)
.set alloc_lock, modem_late_service_alloc - (0x768a8 + 0x64)
.set alloc_release, modem_late_service_alloc - (0x76768 + 0x64)
{
  r17 = r2
  memd(r29+#-16) = r17:16
  allocframe(#16)
}
{
  r19:18 = combine(r0,r1)
  r0 = ##3262225136
  memd(r29+#0) = r19:18
}
{ p0 = cmp.eq(r19,#0); if (p0.new) jump:nt .Lalloc_fail }
{ p0 = cmp.eq(r18,#0); if (p0.new) jump:nt .Lalloc_fail }
{ p0 = cmp.eq(r17,#0); if (p0.new) jump:nt .Lalloc_fail }
{ call platform_alloc_probe
  r1:0 = combine(#36,#1)
}
{ r16 = r0
  if (!cmp.eq(r16.new,#0)) jump:t .Lallocated
}
{ r0 = ##-1032742150 }
.Lalloc_fail:
{ call alloc_fail_log }
.Lallocated:
{ r1:0 = combine(r19,r16)
  r19 = ##3261494288
  r2 = #16
}
{ call alloc_create }
{
  r2 = memw(r19+#16)
  memw(r16+#16) = #2
}
{
  p0 = cmp.eq(r2,#0)
  if (!p0.new) r0 = add(r19,#0)
  memw(r16+#20) = r18
  memw(r16+#24) = r17
}
{ if (!p0) jump:nt .Lpush_list
  memw(r16+#28) = #0
}
{ r2 = memw(r19+#20)
  memw(r16+#32) = r2.new
}
{ jump .Lreturn_descriptor
  memw(r19+#20) = r16
}
.Lpush_list:
{ call alloc_lock }
{
  r2 = memw(r19+#16)
  r0 = memw(r19+#20)
}
{
  if (!p0.new) r0 = ##-1033473008
  p0 = cmp.eq(r2,#0)
  memw(r16+#32) = r0
}
{
  if (!p0) call alloc_release
  memw(r19+#20) = r16
}
.Lreturn_descriptor:
{ r0 = r16 }
{ r17:16 = memd(r29+#8); r19:18 = memd(r29+#0) }
{ dealloc_return }

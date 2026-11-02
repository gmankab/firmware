// SPDX-License-Identifier: GPL-3.0-only
// Hexagon V60 controller registry service and auxiliary status updater,
// modem file [0x446488,0x446650), VMA [0x1e400488,0x1e400650).
// First two instructions are an accessor/return veneer; function body
// from [0x446490,0x446648). The final two NOPs are explicit linker fill.
// This source uses named control registers and symbolic absolute call offsets.
.text
.globl modem_late_aux_registry_core
modem_late_aux_registry_core:
.set ext_1e4007a0, modem_late_aux_registry_core + (792)
.set ext_1e401410, modem_late_aux_registry_core + (3976)
.set ext_1e4043ac, modem_late_aux_registry_core + (16164)
.set ext_1e400e20, modem_late_aux_registry_core + (2456)
.set ext_1e405d78, modem_late_aux_registry_core + (22768)
.set ext_1e4043a8, modem_late_aux_registry_core + (16160)
.set ext_1e4014b0, modem_late_aux_registry_core + (4136)
{
  jumpr r31
}
{
  nop
}
.Lmain:
{
  r17:16 = combine(r1,r0)
  memd(r29+#-16) = r17:16
  allocframe(#40)
}
{
  if (!p0.new) r0 = add(r16,#0)
  p0 = cmp.eq(r16,#0)
  if (!p0.new) jump:t .Lready
  memd(r29+#24) = r19:18
}
{
  r4 = memw(##507617104)
  if (cmp.eq(r4.new,#0)) jump:nt .Llog_start
}
{
  r1:0 = combine(##507576347,#3)
  r2 = #226
}
{
  r3 = ##507576790
  callr r4
}
.Llog_start:
{
  call ext_1e4007a0
  r0 = ##507576790
}
{
  r17:16 = memd(r29+#32)
  r19:18 = memd(r29+#24)
}
{
  dealloc_return
}
.Lready:
{
  call ext_1e401410
}
{
  r2 = memw(r16+#20)
  if (!cmp.eq(r2.new,#0)) jump:t .Ldone
}
{
  p0 = cmp.eq(r17,#0)
  if (p0.new) jump:nt .Lscan_user
}
{
  r2 = memw(##507617068)
}
.Lscan_list:
{
  r18 = memw(r2+#0)
  if (cmp.eq(r18.new,#0)) jump:nt .Lscan_user
}
.Llist_next:
{
  r2 = memw(r18+#20)
  if (cmp.eq(r2.new,#0)) jump:nt .Lsearch_callback
}
{
  r18 = memw(r18+#0)
  if (!cmp.eq(r18.new,#0)) jump:t .Llist_next
}
.Lscan_user:
{
  r18 = memw(r16+#176)
  if (cmp.eq(r18.new,#0)) jump:nt .Lchecking_flags
}
.Lcallback_loop:
{
  r1 = r16
  r2 = memw(r18+#4)
}
{
  call ext_1e4043ac
  r0 = add(r2,#168)
}
{
  r18 = memw(r18+#0)
  if (!cmp.eq(r18.new,#0)) jump:t .Lcallback_loop
}
.Lchecking_flags:
{
  r2 = memb(r16+#136)
  if (cmp.gt(r2.new,#-1)) jump:t .Lupdate_list
}
{
  r1:0 = combine(r16,##507588620)
  r2 = and(r2,#127)
  memb(r16+#136) = r2.new
}
{
  call ext_1e4043ac
}
.Lupdate_list:
{
  r17 = memw(##507617068)
}
{
  r2 = memw(r17+#0)
  if (!cmp.eq(r2.new,#0)) jump:t .Lvalidate_entry
}
{
  r16 = r17
  jump .Lcommit
}
.Lsearch_callback:
{
  r2 = memw(r18+#132)
  if (cmp.eq(r2.new,#0)) jump:nt .Llist_next
}
{
  callr r2
}
{
  jump .Llist_next
}
.Lvalidate_entry:
{
  r3 = ##507534712
}
{
  p0 = cmp.eq(r3,#0)
}
{
  r3 = p0
  memw(r29+#12) = r3.new
}
.Lreset_r16:
{
  r16 = r2
}
.Lvalidate_next:
{
  r2 = memw(r16+#20)
  if (!cmp.eq(r2.new,#0)) jump:nt .Llast_branch
}
{
  r2 = memw(r16+#4)
  r3 = memw(r29+#12)
}
{
  p0 = r3
  if (!p0.new) r3 = ##507576827
  if (!p0.new) jump:t .Lnotify_attached
}
{
  r2 = #262
  r3 = ##507576827
  memw(r29+#0) = r2
}
{
  call ext_1e400e20
}
{
  jump .Lchange_counter
}
.Lnotify_attached:
{
  r1:0 = combine(##507576347,#2)
  r2 = #262
  memw(r29+#0) = r2
}
{
  call ext_1e405d78
}
.Lchange_counter:
{
  call ext_1e4043ac
}
{
  r1:0 = combine(r16,##507588620)
}
{
  r0 = #0
  r2 = memw(r16+#28)
  r1 = memw(r16+#4)
}
{
  call ext_1e4043a8
  r2 = memw(r2+#4)
}
{
  r3 = memw(r16+#160)
  r2 = memw(r16+#156)
}
{
  p0 = cmp.eq(r3,#0)
  if (!p0.new) jump:t .Lbranch_else
  if (p0.new) memw(##507617056) = r2
}
{
  p0 = cmp.eq(r2,#0)
  if (!p0.new) memw(r2+#20) = #0
}
{
  jump .Lupdate_status
}
.Lbranch_else:
{
  p0 = cmp.eq(r2,#0)
  memw(r3+#16) = r2
  if (!p0.new) r2 = memw(r16+#156)
}
{
  if (!p0) r3 = memw(r16+#160)
  if (!p0) memw(r2+#20) = r3.new
}
.Lupdate_status:
{
  call ext_1e4014b0
}
{
  r0 = r16
  r2 = memw(r16+#0)
  memw(r17+#0) = r2.new
}
.Lloop:
{
  r16 = memw(r17+#0)
  if (!cmp.eq(r16.new,#0)) jump:t .Lvalidate_next
}
{
  r16 = r17
  jump .Lcommit
}
.Llast_branch:
{
  r17 = r16
  r2 = memw(r16+#0)
}
{
  p0 = cmp.eq(r2,#0)
  if (!p0.new) jump:t .Lreset_r16
}
.Lcommit:
{
  memw(##507617076) = r16
}
.Ldone:
{
  r17:16 = memd(r29+#32)
  r19:18 = memd(r29+#24)
}
{
  dealloc_return
}
{
  nop
}
{
  nop
}

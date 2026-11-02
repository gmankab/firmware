// SPDX-License-Identifier: GPL-3.0-only
// A 64-byte Hexagon V60 local buffer-object factory: preserve frame, build
// stack descriptors, initialize the instance and return the initialized slot.
.text
.globl modem_late_aux_buffer_factory
modem_late_aux_buffer_factory:
.set make_array,modem_late_aux_buffer_factory+0x7b38
.set register_array,modem_late_aux_buffer_factory+0x3340
{r16 = r2}
{r2 = r3}
{
  memd(r29+#-16) = r17:16
  allocframe(#272)
}
{
  r0 = add(r29,#8)
  r1 = #256
  r3 = add(r29,#280)
  r4 = add(r29,#280)
}
{
  call make_array
  memw(r29+#4) = r4
}
{
  r3:2 = combine(r16,##507576841)
  r1 = #2
  r0 = add(r29,#8)
}
{call register_array}
{
  r17:16 = memd(r29+#264)
  dealloc_return
}

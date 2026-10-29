// SPDX-License-Identifier: GPL-3.0-only
// Decode controller record format and select type-specific processing paths.
// Branch addresses derive from the relocated handler symbols; no OEM opcode
// bytes are included in this Hexagon V60 source.
.text
.globl modem_late_aux_large_controller_phase30
modem_late_aux_large_controller_phase30:
.set unsupported_format, modem_late_aux_large_controller_phase30 + 0x50
.set primary_format, modem_late_aux_large_controller_phase30 + 0xa4
.set format_three, modem_late_aux_large_controller_phase30 + 0x3a4
.set format_unknown, modem_late_aux_large_controller_phase30 + 0x51c
.set update_config, modem_late_aux_large_controller_phase30 + 0x618
.set lookup_handle, modem_late_aux_large_controller_phase30 - 0x12a8
{
  memw(r29+#48) = r27
  memw(r29+#76) = r24
}
{
  r2 = memw(r18+#-16)
  if (cmp.gt(r2.new,#2)) jump:t unsupported_format
}
{
  p0 = cmp.eq(r2,#1)
  if (p0.new) jump:nt primary_format
  if (p0.new) r2 = memw(r17+#28)
  if (p0.new) r7 = memw(r29+#60)
}
{
  if (p0.new) r0 = add(r17,#0)
  p0 = cmp.eq(r2,#2)
  if (!p0.new) jump:nt format_unknown
}
{
  call lookup_handle
  r1 = memw(r18+#-8)
}
{
  r3 = memub(r17+#136)
  memw(r17+#44) = r0
}
{r2 = memh(r21+#16)}
{p0 = cmp.eq(r2,#3)}
{r2 = mux(p0,#32,#0)}
{
  r3 = or(r2,and(r3,#223))
  jump update_config
  memb(r17+#136) = r3.new
}
unsupported_format:
{
  if (p0.new) r0 = add(r17,#0)
  p0 = cmp.eq(r2,#3)
  if (p0.new) jump:nt format_three
}
{
  p0 = cmp.eq(r2,#4)
  if (!p0.new) jump:nt format_unknown
  if (p0.new) r1 = memw(r18+##-8)
}
{
  call lookup_handle
  r0 = r17
  r25 = r24
}

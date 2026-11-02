// Source-authored Hexagon controller callback initializer.
// OEM file [0x39060,0x390d8) in the f0036000 RX text segment.
.text
.globl adsp_late_controller_dispatch
adsp_late_controller_dispatch:
{
  immext(#0xf02cb680)
  memw(##0xf02cb68c) = r3
}
{
  immext(#0x5880)
  call begin_controller
  r29 = add(r29,#-8)
  memd(r29+#-8) = r1:0
}
{
  immext(#0x1740)
  call first_controller
  r1:0 = memd(r29+#0)
}
{
  immext(#0x2880)
  call second_controller
  r1:0 = memd(r29+#0)
}
{
  immext(#0xf01f1000)
  r18 = ##0xf01f1000
}
{
  immext(#0x140)
  call third_controller
}
{ memw(r18+#0) = r0 }
{
  immext(#0x200)
  call fourth_controller
}
{
  immext(#0x5980)
  call fifth_controller
  r1:0 = memd(r29+#0)
}
{
  immext(#0x2840)
  call sixth_controller
  r1:0 = memd(r29+#0)
}
{
  immext(#0xf0480)
  call seventh_controller
  r1:0 = memd(r29++#8)
}
trap:
{ jump trap }
{ jumpr lr }
.set begin_controller, adsp_late_controller_dispatch+0x58b0
.set first_controller, adsp_late_controller_dispatch+0x1758
.set second_controller, adsp_late_controller_dispatch+0x28cc
.set third_controller, adsp_late_controller_dispatch+0x180
.set fourth_controller, adsp_late_controller_dispatch+0x268
.set fifth_controller, adsp_late_controller_dispatch+0x59f0
.set sixth_controller, adsp_late_controller_dispatch+0x28b8
.set seventh_controller, adsp_late_controller_dispatch+0xf04e4

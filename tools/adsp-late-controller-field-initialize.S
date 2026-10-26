// SPDX-License-Identifier: GPL-3.0-only
// ADSP [file 0x3ed1c,0x3edc8): initialize and publish five controller
// fields from the platform status register bank, with the original packet
// ordering and final branch into the independently verified descriptor setup.
.text
.globl adsp_late_controller_field_initialize
adsp_late_controller_field_initialize:
{ immext(#0xf02cbd00); r0 = ##-265506816; memd(r29+#-16) = r17:16; allocframe(#8) }
{ immext(#0xf004ef00); r1 = ##-268190120; immext(#0xf01f1500); r3 = memh(r16=##4028568856) }
{ call initialize_field; r2 = #28; r4 = #0 }
{ immext(#0xf02cbd00); r0 = ##4029460528; r3 = memh(r16+#2) }
{ immext(#0xf004ef00); r1 = ##-268190092; r2 = #52; r4 = #0 }
{ call initialize_field }
{ immext(#0xf02cbd40); r0 = ##4029460576; r3 = memh(r16+#4) }
{ immext(#0xf004ef40); r1 = ##-268190064; r2 = #56; r4 = #0 }
{ call initialize_field }
{ immext(#0xf02cbd80); r0 = ##4029460624; r3 = memh(r16+#12) }
{ immext(#0xf004ef40); r1 = ##-268190036; r2 = #48; r4 = #0 }
{ call initialize_field }
{ immext(#0xf02cbdc0); r0 = ##4029460672; r3 = memh(r16+#10) }
{ r2 = #112; immext(#0xf004ef80); r1 = ##4026777288; r4 = #0 }
{ call initialize_field }
{ immext(#0xf0300900); r0 = ##-264959712; immext(#0xf01f1500); r3 = memw(##4028568872) }
{ r2 = #112; immext(#0xf004ef80); r1 = ##4026777288; r17:16 = memd(r29+#0) }
{ r4 = #1; immext(#0); jump initialize_field; deallocframe }
.set initialize_field, adsp_late_controller_field_initialize+0xb4

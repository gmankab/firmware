// ADSP second late RX segment entry [file 0x364000,0x3640a8).
// Source-derived controller initialization, runtime relocation and bootstrap
// handoff; all addresses are typed Hexagon PC-relative constants/call labels.
.text
.globl adsp_late_rx_startup
adsp_late_rx_startup:
{ r29 = add(pc,##10309632); r30 = #0; r0 = #0 }
{ framelimit = r0 }
{ r0 = add(pc,##21217260) }
{ r1 = add(pc,##4633588) }
{ gp = r0; callr r1 }
{ r0 = add(pc,##10305492) }
{ r1 = add(pc,##21216204) }
{ call init_platform }
{ r0 = add(pc,##21221184) }
{ r1 = add(pc,##21221176) }
{ call init_platform }
{ r0 = add(pc,##21218352) }
{ memw(r0+#0) = r16 }
{ r2 = add(pc,##4633144) }
{ callr r2 }
{ r0 = add(pc,##140) }
{ r1 = add(pc,##136) }
{ r0 = memw(r0+#0) }
{ r1 = memw(r1+#0) }
{ call finalize_init }
{ r2 = add(pc,##4631840) }
{ r0 = add(pc,##10305392) }
{ r1 = ##4096; callr r2 }
stop_forever:
{ jump stop_forever }
.set init_platform, adsp_late_rx_startup+0xa8
.set finalize_init, adsp_late_rx_startup+0x46b1c4

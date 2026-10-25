// SPDX-License-Identifier: GPL-3.0-only
// Hexagon v66 WLAN packetized reporting wrappers, an import trampoline,
// and four short tail thunks. Runtime [0xb0002494,0xb00025a0).
// The calls and jumps use PC-relative displacements normalized to the
// independently assembled routine base.
.text
.globl wlan_runtime_status_report
wlan_runtime_status_report:
{ call 0x2a32c0; allocframe(#48) }
{ r17 = r0; r0 = ##-0x7dfdffff; r6 = memuh(gp+#0x10e) }
{ r16 = r1; r0 = ##48; memw(r29+#0) = r0 }
{ r1 = #0x803; r18 = or(r6,##0xff0000); memw(r29+#8) = r16 }
{ r21:20 = combine(r3,r4); r19 = r5; memw(r29+#4) = r18 }
{ call 0x218ce4; r23:22 = combine(##-0x4fc9fef2,r2) }
{ r0 = ##-0x4fd59e4c; r2 = memuh(gp+#0x10e) }
{ r18 = insert(r2,#16,#0); r1 = ##-0x7dfdffff }
{ call report_import_trampoline; r3:2 = combine(r16,r18) }
{ r1:0 = combine(r16,r17); r3:2 = combine(r21,r22); r5:4 = combine(r19,r20); memh(r23+#0) += #1 }
{ call 0x2a32b4 }
{ jump 0x1d5b7c }
report_import_trampoline:
{ jump ##-0x1464 }
.globl wlan_runtime_guard_report
wlan_runtime_guard_report:
{ call 0x2a325c; allocframe(#32) }
{ r16 = r1; r1 = ##-0x7dfdfffe; r2 = memuh(gp+#0x10e) }
{ r19:18 = combine(##-0x4fc9fef2,r0); memw(r29+#0) = r1 }
{ r0 = ##56; r1 = #0x803 }
{ r17 = or(r2,##0xff0000); memw(r29+#8) = r16 }
{ call 0x218c5c; memw(r29+#4) = r17 }
{ r0 = ##-0x4fd59e40; r2 = memuh(gp+#0x10e) }
{ r17 = insert(r2,#16,#0); r1 = ##-0x7dfdfffe }
{ call report_import_trampoline; r3:2 = combine(r16,r17) }
{ call 0x2a326c; r1:0 = combine(r16,r18); memh(r19+#0) += #1 }
{ jump 0x1d5b04 }
// Loader table exports as explicit executable Hexagon branch packets.
.rept 4
{ jump 0x1d5b04 }
.endr

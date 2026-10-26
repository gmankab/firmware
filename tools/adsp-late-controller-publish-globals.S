// Controller-global table setup for the ADSP runtime at file [0x3f31c,0x3f37c).
// Source emits fixed-address platform pointers, installs callbacks, and
// records the platform init result; no vendor code words or metadata copied.
.text
.globl adsp_late_controller_publish_globals
adsp_late_controller_publish_globals:
{ immext(#4026777472); r3 = ##-268189784; memd(r29+#-16) = r17:16; allocframe(#8) }
{ immext(#5373888); r4 = ##5373951; r16 = r0 }
{ immext(#4026779392); r5 = ##-268187876; immext(#4029460672); memw(r3=##4029460720) = r3 }
{ immext(#4030007616); memw(##4030007632) = r5; immext(#4030007616); memw(##4030007636) = r4 }
{ memw(r3+#4) = r4; memw(r16+#0) = r3 }
{ immext(#4029460672); memw(r16+#4) = ##-265506576; immext(#4029460672); memw(r16+#8) = ##-265506576 }
{ call platform_register_device; immext(#4030007616); memw(r16+#16) = ##-264959664 }
{ memw(r16+#12) = r0; r17:16 = memd(r29+#0) }:mem_noshuf
{ dealloc_return }
.set platform_register_device, adsp_late_controller_publish_globals+0xec508

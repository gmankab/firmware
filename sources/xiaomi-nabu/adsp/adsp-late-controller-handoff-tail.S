// ADSP Hexagon V66 [file 0x3a7b8,0x3a7e0): platform handoff
// continuation, calling four controller helpers and a shared event notifier.
// The branch exits into a shared older-function epilogue; this is a
// byte-verified *fragment*, not a self-contained callable function.
.text
.globl adsp_late_controller_handoff_tail
adsp_late_controller_handoff_tail:
{ call platform_begin; allocframe(#0) }
{ call platform_prepare }
{ call platform_commit }
{ call platform_trigger; immext(#0xf0299000); r0 = ##-268208144 }
{ call platform_notify }
{ jump shared_restore; deallocframe }
.set platform_begin, adsp_late_controller_handoff_tail+0x1298
.set platform_prepare, adsp_late_controller_handoff_tail+0x2818
.set platform_commit, adsp_late_controller_handoff_tail-0x158
.set platform_trigger, adsp_late_controller_handoff_tail+0x12e580
.set platform_notify, adsp_late_controller_handoff_tail-0x11b8
.set shared_restore, adsp_late_controller_handoff_tail-0x38

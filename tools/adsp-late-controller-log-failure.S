// Small QuRT controller failure logger. ADSP file [0x3f4b0,0x3f4c8).
// Writes a platform failure descriptor on the stack and returns -1.
.text
.globl adsp_late_controller_log_failure
adsp_late_controller_log_failure:
{ immext(#4028152192); r0 = ##-266815051; memw(r29+#-16) = r0; allocframe(#8) }
{ call platform_failure_log }
{ r0 = #-1; dealloc_return }
.set platform_failure_log, adsp_late_controller_log_failure+0xe797c

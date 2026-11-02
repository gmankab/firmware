// SPDX-License-Identifier: GPL-3.0-only
// Hexagon V66 [file 0x3e8f4,0x3e910): marshal diagnostic report and tail-dispatch.
.text
.globl adsp_late_controller_report_default
adsp_late_controller_report_default:
{ memw(r29+#-12) = r1; allocframe(#8) }
{ call create_report; immext(#4028152064); r0 = ##4028152064; memw(r29+#0) = r0 }
{ jump report_platform; deallocframe }
.set create_report, adsp_late_controller_report_default+0xe8538
.set report_platform, adsp_late_controller_report_default+0xe82f4

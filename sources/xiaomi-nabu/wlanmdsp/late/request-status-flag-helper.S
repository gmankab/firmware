// SPDX-License-Identifier: GPL-3.0-only
// Agent 14 file [0x102cd0,0x102ce4): status callback helper.
.text
.globl agent14_late_request_status_flag_helper
agent14_late_request_status_flag_helper:
// +0x0000: 2 encoded words
{ r5:4 = combine(r4,#-0x1); allocframe(#0x8) }
// +0x0008: 2 encoded words
{ call -739148; memw(r29+#0x0) = #0x2 }
// +0x0010: 1 encoded words
{ dealloc_return }

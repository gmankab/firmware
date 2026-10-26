// SPDX-License-Identifier: GPL-3.0-only
// File offsets [0x105668,0x10567c): create a session-start diagnostic and tail jump.
.text
.globl agent14_late_session_start_thunk
agent14_late_session_start_thunk:
{ call -782840; r1 = #0x10; allocframe(#0x0) }
{ r0 = ##0xb0345784; deallocframe }
{ jump 0xfd248 }

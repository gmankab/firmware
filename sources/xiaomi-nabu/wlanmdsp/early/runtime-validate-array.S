// SPDX-License-Identifier: GPL-3.0-only
// Agent13: visit each frame in the bounded wireless record array.
// File [0x42bcc,0x42c04), VMA [0xb0002bcc,0xb0002c04).
.text
.globl agent13_runtime_validate_array
agent13_runtime_validate_array:
{ call 0x2a2bac; allocframe(#16) }
{ r16 = r0 }
{ r17 = add(r16,#808); r2 = memb(r16+#1); if (cmp.eq(r2.new,#0)) jump:nt done }
{ r18 = #0 }
array_loop:
{ call -0x4e0; r1 = #255; r18 = add(r18,#1); r0 = memw(r17++#4) }
{ r2 = memub(r16+#1); if (cmp.gt(r2.new,r18)) jump:t array_loop }
done:
{ jump 0x2a2c54 }

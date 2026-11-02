# SPDX-License-Identifier: GPL-3.0-only
# Independent GNU NDS32 assembly cross-check; no opaque instruction words.
# File [0x10016,0x101b6), runtime [0xfe16,0xffb6).
.section .text
.flag verbatim
.global agent18_upper_start
agent18_upper_start:
# Save the caller context and select an input mode; EX9 entries remain explicit dependencies.
L_10016: push25 $r10, #8
L_10018: mov55 $r6, $r0
L_1001a: lwi.gp	$r0, [ + #-82992]
L_1001e: swi37.sp $r0, [ + #4]
L_10020: ex9.it #205
L_10022: ex9.it #203
L_10024: mov55 $r7, $r1
L_10026: mov55 $r9, $r2
L_10028: beq	$r5, $r0, . + (386)
L_1002c: bnez38 $r6, . + (8)
L_1002e: sbi333 $r6, [$r2 + #0]
L_10030: movi55 $r7, #1
L_10032: j8 . + (30)
L_10034: bnec	$r6, #4, . + (18)
L_10038: mov55 $r0, $r1
L_1003a: jal	. + (-62)
L_1003e: sbi	$r7, [$r9 + #0]
L_10042: movi55 $r7, #3
L_10044: j8 . + (12)
L_10046: addri36.sp $r0, #4
L_10048: lb	$r6, [$r0 + ($r1 << #0)]
L_1004c: sbi333 $r1, [$r2 + #0]
L_1004e: movi55 $r7, #3
# Initialize all 32 bytes of the channel-code workspace at SRAM 0x2dc58.
L_10050: movi55 $r0, #0
L_10052: movi	$r1, #0x2dc58
L_10056: sb	$r6, [$r1 + ($r0 << #0)]
L_1005a: addi45 $r0, #1
L_1005c: movi	$r9, #0x2dc58
L_10060: bnec	$r0, #0x20, . + (-10)
L_10064: jal	. + (-248)
L_10068: lwi.gp	$r6, [ + #-5340]
# Obtain controller pointers, normalize status bytes, and write descriptor fields.
L_1006c: beqz38 $r0, . + (32)
L_1006e: lwi	$r0, [$r6 + #0x50]
L_10072: movi	$r1, #0x203c0
L_10076: srli45 $r0, #0x10
L_10078: ex9.it #68
L_1007a: ex9.it #77
L_1007c: lwi.gp	$r1, [ + #0xe68]
L_10080: addi45 $r1, #5
L_10082: zeb33 $r1, $r1
L_10084: sbi333 $r1, [$r0 + #4]
L_10086: sbi333 $r1, [$r0 + #5]
L_10088: sbi333 $r1, [$r0 + #6]
L_1008a: sbi333 $r1, [$r0 + #7]
L_1008c: jal	. + (-324)
L_10090: beqz38 $r0, . + (204)
L_10092: addi	$r8, $r6, #0x50
L_10096: lwi450 $r0, [$r8]
L_10098: movi	$r1, #0x20420
L_1009c: srli45 $r0, #0x10
L_1009e: ex9.it #68
L_100a0: ex9.it #77
L_100a2: ex9.it #458
L_100a4: lwi.gp	$r2, [ + #-5376]
L_100a8: addi45 $r1, #5
L_100aa: zeb33 $r1, $r1
L_100ac: sbi333 $r1, [$r0 + #4]
L_100ae: lwi.gp	$r1, [ + #0xe70]
L_100b2: movi	$r3, #0x39780
L_100b6: addi45 $r1, #5
L_100b8: zeb33 $r1, $r1
L_100ba: sbi333 $r1, [$r0 + #6]
L_100bc: lwi450 $r0, [$r8]
L_100be: movi	$r1, #0x20360
L_100c2: srli45 $r0, #0x10
L_100c4: ex9.it #68
L_100c6: ex9.it #77
L_100c8: lwi.gp	$r1, [ + #0x2b8]
L_100cc: movi55 $r4, #8
L_100ce: addi45 $r1, #5
L_100d0: zeb33 $r1, $r1
L_100d2: sbi333 $r1, [$r0 + #4]
L_100d4: lwi.gp	$r1, [ + #0x2c0]
L_100d8: movi55 $r5, #2
L_100da: addi45 $r1, #5
L_100dc: zeb33 $r1, $r1
L_100de: sbi333 $r1, [$r0 + #5]
L_100e0: lwi.gp	$r1, [ + #0x2bc]
L_100e4: movi55 $r16, #9
L_100e6: addi45 $r1, #5
L_100e8: zeb33 $r1, $r1
L_100ea: sbi333 $r1, [$r0 + #6]
L_100ec: lwi.gp	$r1, [ + #0x2c4]
L_100f0: addi45 $r1, #5
L_100f2: zeb33 $r1, $r1
L_100f4: sbi333 $r1, [$r0 + #7]
L_100f6: lwi450 $r0, [$r8]
L_100f8: movi	$r1, #0x203c0
L_100fc: srli45 $r0, #0x10
L_100fe: ex9.it #68
L_10100: ex9.it #77
L_10102: lwi.gp	$r1, [ + #0xe68]
L_10106: movi55 $r6, #0xb
L_10108: addi45 $r1, #5
L_1010a: zeb33 $r1, $r1
L_1010c: sbi333 $r1, [$r0 + #4]
L_1010e: sbi333 $r1, [$r0 + #5]
L_10110: sbi333 $r1, [$r0 + #6]
L_10112: sbi333 $r1, [$r0 + #7]
# Map each bounded input channel code to the selected output value.
L_10114: movi55 $r0, #0
L_10116: lwi	$r1, [$r2 + #0x20]
L_1011a: fexti33 $r1, #5
L_1011c: slts45 $r0, $r1
L_1011e: beqzs8 . + (130)
L_10120: add333 $r1, $r0, $r3
L_10122: lbi333 $r1, [$r1 + #0]
L_10124: zeb33 $r1, $r1
L_10126: slti45 $r1, #0x14
L_10128: beqzs8 . + (18)
L_1012a: slti45 $r1, #0xc
L_1012c: beqzs8 . + (32)
L_1012e: beqz38 $r1, . + (36)
L_10130: bnec	$r1, #0xa, . + (38)
L_10134: sb	$r16, [$r9 + ($r0 << #0)]
L_10138: j8 . + (30)
L_1013a: beqc	$r1, #0x14, . + (12)
L_1013e: addi	$r15, $r1, #-32
L_10142: slt45 $r5, $r15
L_10144: bnezs8 . + (18)
L_10146: sb	$r4, [$r9 + ($r0 << #0)]
L_1014a: j8 . + (12)
L_1014c: sb	$r6, [$r9 + ($r0 << #0)]
L_10150: j8 . + (6)
L_10152: sb	$r7, [$r9 + ($r0 << #0)]
L_10156: addi45 $r0, #1
L_10158: zeb33 $r0, $r0
L_1015a: j8 . + (-68)
# Handle the alternate channel selections and update GP-relative status bytes.
L_1015c: ex9.it #109
L_1015e: beq	$r6, $r0, . + (10)
L_10162: ex9.it #152
L_10164: bne	$r6, $r0, . + (60)
L_10168: movi55 $r1, #8
L_1016a: sbi.gp	$r1, [ + #0x278]
L_1016e: movi55 $r1, #0xa
L_10170: sbi.gp	$r1, [ + #0x27a]
L_10174: sbi.gp	$r1, [ + #0x284]
L_10178: lbi.gp	$r1, [ + #0x13d2]
L_1017c: movi	$r2, #0x2e1f4
L_10180: lb	$r2, [$r2 + ($r1 << #0)]
L_10184: movi	$r0, #0x2dc58
L_10188: bnec	$r2, #9, . + (12)
L_1018c: movi55 $r2, #2
L_1018e: sb	$r2, [$r0 + ($r1 << #0)]
L_10192: j8 . + (14)
L_10194: sb	$r7, [$r0 + ($r1 << #0)]
L_10198: lbi.gp	$r1, [ + #0x13d3]
L_1019c: sb	$r7, [$r0 + ($r1 << #0)]
# Publish the workspace through the lower-ILM helper, then restore context and return.
L_101a0: ex9.it #80
L_101a2: movi	$r1, #0x2dc58
L_101a6: jal	. + (-952)
L_101aa: pop25 $r10, #8
# Separate leaf helper: store two supplied byte values and return through LP.
L_101ac: movi	$r1, #0x38d40
L_101b0: sbi333 $r0, [$r1 + #0]
L_101b2: sbi333 $r2, [$r3 + #0]
L_101b4: ret5 $lp

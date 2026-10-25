# SPDX-License-Identifier: GPL-3.0-only
# Width-preserving independent GNU assembly; FILE PCs label the runtime-biased stream.
# Four embedded tables use target-label differences, not opaque byte copies.
.section .text
.flag verbatim
.global agent18_upper_start
agent18_upper_start:
# Input mode / bounded channel-code mapping; context saved here.
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
# Initialize the 32-byte SRAM channel-code workspace.
L_10050: movi55 $r0, #0
L_10052: movi	$r1, #0x2dc58
L_10056: sb	$r6, [$r1 + ($r0 << #0)]
L_1005a: addi45 $r0, #1
L_1005c: movi	$r9, #0x2dc58
L_10060: bnec	$r0, #0x20, . + (-10)
L_10064: jal	. + (-248)
L_10068: lwi.gp	$r6, [ + #-5340]
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
# Map each bounded input channel code to an output code.
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
L_101a0: ex9.it #80
L_101a2: movi	$r1, #0x2dc58
L_101a6: jal	. + (-952)
L_101aa: pop25 $r10, #8
# Leaf helper: store two caller-supplied byte values.
L_101ac: movi	$r1, #0x38d40
L_101b0: sbi333 $r0, [$r1 + #0]
L_101b2: sbi333 $r2, [$r3 + #0]
L_101b4: ret5 $lp
L_101b6: push25 $r6, #8
L_101b8: mov55 $r6, $r0
L_101ba: ex9.it #11
L_101bc: swi450 $r2, [$sp]
L_101be: swi37.sp $r3, [ + #4]
L_101c0: sbi333 $r6, [$r0 + #0]
L_101c2: mov55 $r0, $r1
L_101c4: ex9.it #64
L_101c6: mov55 $r0, $r6
L_101c8: lwi450 $r1, [$sp]
L_101ca: lwi37.sp $r2, [ + #4]
L_101cc: jal	. + (-438)
L_101d0: pop25 $r6, #8
L_101d2: push25 $r14, #0
L_101d4: movi55 $r0, #0
L_101d6: sbi.gp	$r0, [ + #-5710]
L_101da: ex9.it #0
L_101dc: movi55 $r1, #3
L_101de: lbi	$r10, [$r0 + #1]
L_101e2: movi	$r7, #0x38d41
L_101e6: sbi333 $r1, [$r0 + #1]
L_101e8: lbi	$r9, [$r7 + #0]
L_101ec: jal	. + (-2316)
L_101f0: movi55 $r0, #0
L_101f2: mov55 $r2, $r0
L_101f4: mov55 $r3, $r7
L_101f6: ex9.it #18
L_101f8: movi	$r6, #0x38d40
L_101fc: lbi	$r11, [$r6 + #0]
L_10200: ex9.it #392
L_10202: ex9.it #80
L_10204: movi	$r1, #0x2dd0c
L_10208: jal	. + (-1700)
L_1020c: movi55 $r0, #3
L_1020e: ex9.it #18
L_10210: mov55 $r2, $r9
L_10212: mov55 $r3, $r7
L_10214: ex9.it #392
L_10216: slti45 $r11, #2
L_10218: movi55 $r0, #3
L_1021a: beqzs8 . + (4)
L_1021c: movi55 $r0, #0
L_1021e: sbi333 $r0, [$r6 + #0]
L_10220: movi	$r1, #0x2dd0c
L_10224: ex9.it #80
L_10226: jal	. + (-1730)
L_1022a: lbi333 $r0, [$r6 + #0]
L_1022c: ex9.it #18
L_1022e: mov55 $r2, $r9
L_10230: ex9.it #67
L_10232: ex9.it #392
L_10234: lbi333 $r0, [$r6 + #0]
L_10236: sbi.gp	$r0, [ + #-5637]
L_1023a: ex9.it #0
L_1023c: sbi	$r10, [$r0 + #1]
L_10240: pop25 $r14, #0
L_10242: push25 $r8, #0
L_10244: mov55 $fp, $r0
L_10246: lwi37 $r2, [$fp + #0x48]
L_10248: lwi37 $r6, [$fp + #0x4c]
L_1024a: movi	$r4, #0x20320
L_1024e: andi	$r2, $r2, #0x7fff
L_10252: add_slli	$r2, $r4, $r2, #2
L_10256: srli45 $r6, #0x10
L_10258: andi	$r0, $r6, #0x7fff
L_1025c: lbi333 $r6, [$r2 + #5]
L_1025e: movi55 $r4, #0xc
L_10260: mul33 $r6, $r4
L_10262: add_slli	$r6, $r6, $r0, #2
L_10266: ex9.it #161
L_10268: add45 $r6, $r0
L_1026a: beqz38 $r3, . + (86)
L_1026c: srli45 $r1, #1
L_1026e: subri	$r0, $r1, #0x640
L_10272: zeh33 $r1, $r0
L_10274: ex9.it #180
L_10276: divr	$r0, $r0, $r1, $r0
L_1027a: lwi333 $r1, [$r2 + #0xc]
L_1027c: zeb33 $r0, $r0
L_1027e: bitci	$r1, $r1, #0x300
L_10282: ori	$r1, $r1, #0x200
L_10286: swi333 $r1, [$r2 + #0xc]
L_10288: subi333 $r1, $r0, #5
L_1028a: slti45 $r1, #0x17
L_1028c: beqzs8 . + (42)
L_1028e: movi55 $r4, #1
L_10290: sll	$r7, $r4, $r0
L_10294: addi	$r16, $r0, #-1
L_10298: movi55 $r1, #0
L_1029a: add45 $r0, $r4
L_1029c: sub	$r3, $r16, $r1
L_102a0: sll	$r5, $r4, $r3
L_102a4: add333 $r3, $r0, $r1
L_102a6: sll	$r3, $r4, $r3
L_102aa: or33 $r3, $r5
L_102ac: addi45 $r1, #1
L_102ae: or33 $r7, $r3
L_102b0: bnec	$r1, #3, . + (-20)
L_102b4: j8 . + (20)
L_102b6: slti45 $r0, #5
L_102b8: bnezs8 . + (12)
L_102ba: sethi	$r7, #0xff000
L_102be: j8 . + (10)
L_102c0: ifcall9 . + (28)
L_102c2: j8 . + (6)
L_102c4: movi	$r7, #0xff
L_102c8: lbi.gp	$r0, [ + #-5633]
L_102cc: beqz38 $r0, . + (28)
L_102ce: lhi.gp	$r0, [ + #-5538]
L_102d2: lhi.gp	$r1, [ + #-5532]
L_102d6: srli45 $r0, #1
L_102d8: slt45 $r1, $r0
L_102da: beqzs8 . + (14)
L_102dc: lwi333 $r0, [$r2 + #0xc]
L_102de: movi55 $r7, #-1
L_102e0: bitci	$r0, $r0, #0x300
L_102e4: swi333 $r0, [$r2 + #0xc]
L_102e6: ifret16
L_102e8: ex9.it #2
L_102ea: beqz38 $r0, . + (4)
L_102ec: srli45 $r7, #0x10
L_102ee: btst	$r0, $r7, #0xf
L_102f2: ifcall9 . + (132)
L_102f4: lwi333 $r0, [$r6 + #4]
L_102f6: sethi	$r4, #0xffc7f
L_102fa: ifcall9 . + (88)
L_102fc: swi333 $r0, [$r6 + #4]
L_102fe: btst	$r0, $r7, #0xe
L_10302: ifcall9 . + (116)
L_10304: lwi333 $r0, [$r6 + #4]
L_10306: addi	$r4, $r4, #0xfff
L_1030a: ifcall9 . + (92)
L_1030c: swi333 $r0, [$r6 + #4]
L_1030e: btst	$r0, $r7, #0xd
L_10312: ifcall9 . + (100)
L_10314: lwi333 $r0, [$r6 + #4]
L_10316: sethi	$r3, #0xfe3ff
L_1031a: bitci	$r0, $r0, #7
L_1031e: or33 $r0, $r1
L_10320: swi333 $r0, [$r6 + #4]
L_10322: btst	$r0, $r7, #0xc
L_10326: ifcall9 . + (80)
L_10328: lwi450 $r0, [$r6]
L_1032a: addi	$r3, $r3, #0xfff
L_1032e: bitci	$r0, $r0, #0x7000
L_10332: or_slli	$r0, $r0, $r1, #0xc
L_10336: swi450 $r0, [$r6]
L_10338: btst	$r0, $r7, #0xb
L_1033c: ifcall9 . + (58)
L_1033e: lwi450 $r0, [$r6]
L_10340: bitci	$r0, $r0, #0xe00
L_10344: or_slli	$r0, $r0, $r1, #9
L_10348: swi450 $r0, [$r6]
L_1034a: btst	$r0, $r7, #0xa
L_1034e: ifcall9 . + (40)
L_10350: lwi450 $r0, [$r6]
L_10352: bitci	$r0, $r0, #0x1c0
L_10356: or_slli	$r0, $r0, $r1, #6
L_1035a: ifret16
L_1035c: swi450 $r0, [$r6]
L_1035e: btst	$r0, $r7, #9
L_10362: ifcall9 . + (20)
L_10364: lwi450 $r0, [$r6]
L_10366: bitci	$r0, $r0, #0x38
L_1036a: or_slli	$r0, $r0, $r1, #3
L_1036e: ifret16
L_10370: swi450 $r0, [$r6]
L_10372: btst	$r0, $r7, #8
L_10376: slli333 $r0, $r0, #1
L_10378: addi45 $r0, #4
L_1037a: andi	$r1, $r0, #6
L_1037e: ifret16
L_10380: lwi450 $r0, [$r6]
L_10382: bitci	$r0, $r0, #7
L_10386: or33 $r0, $r1
L_10388: btst	$r1, $r7, #7
L_1038c: slli333 $r1, $r1, #1
L_1038e: swi450 $r0, [$r6]
L_10390: ifcall9 . + (34)
L_10392: movi	$r0, #-458753
L_10396: and33 $r1, $r0
L_10398: or_slli	$r1, $r1, $r2, #0x10
L_1039c: swi450 $r1, [$r6]
L_1039e: btst	$r1, $r7, #6
L_103a2: ifcall9 . + (14)
L_103a4: and33 $r1, $r4
L_103a6: or_slli	$r1, $r1, $r2, #0x13
L_103aa: swi450 $r1, [$r6]
L_103ac: btst	$r1, $r7, #5
L_103b0: slli333 $r1, $r1, #1
L_103b2: addi45 $r1, #4
L_103b4: andi	$r2, $r1, #6
L_103b8: lwi450 $r1, [$r6]
L_103ba: ifret16
L_103bc: and33 $r1, $r3
L_103be: or_slli	$r1, $r1, $r2, #0x16
L_103c2: swi450 $r1, [$r6]
L_103c4: btst	$r1, $r7, #4
L_103c8: ifcall9 . + (20)
L_103ca: sethi	$r1, #0xf1fff
L_103ce: ex9.it #252
L_103d0: and33 $r1, $r5
L_103d2: or_slli	$r1, $r1, $r2, #0x19
L_103d6: swi450 $r1, [$r6]
L_103d8: btst	$r1, $r7, #3
L_103dc: slli333 $r1, $r1, #1
L_103de: addi45 $r1, #4
L_103e0: lwi450 $r5, [$r6]
L_103e2: andi	$r2, $r1, #6
L_103e6: ifret16
L_103e8: sethi	$r1, #0x8ffff
L_103ec: ex9.it #252
L_103ee: and33 $r1, $r5
L_103f0: or_slli	$r1, $r1, $r2, #0x1c
L_103f4: swi450 $r1, [$r6]
L_103f6: btst	$r1, $r7, #2
L_103fa: lwi333 $r2, [$r6 + #4]
L_103fc: slli333 $r1, $r1, #1
L_103fe: addi45 $r1, #4
L_10400: andi	$r1, $r1, #6
L_10404: and33 $r0, $r2
L_10406: or_slli	$r0, $r0, $r1, #0x10
L_1040a: swi333 $r0, [$r6 + #4]
L_1040c: andi	$r0, $r7, #2
L_10410: ifcall	. + (-152)
L_10414: lwi333 $r0, [$r6 + #4]
L_10416: xlsb33 $r7, $r7
L_10418: and33 $r0, $r4
L_1041a: or_slli	$r0, $r0, $r1, #0x13
L_1041e: slli333 $r7, $r7, #1
L_10420: swi333 $r0, [$r6 + #4]
L_10422: addi45 $r7, #4
L_10424: andi	$r0, $r7, #6
L_10428: lwi333 $r7, [$r6 + #4]
L_1042a: and33 $r7, $r3
L_1042c: or_slli	$r7, $r7, $r0, #0x16
L_10430: swi333 $r7, [$r6 + #4]
L_10432: pop25 $r8, #0
L_10434: movi55 $r0, #0
L_10436: ex9.it #311
L_10438: sbi333 $r0, [$r1 + #0]
L_1043a: ex9.it #179
L_1043c: shi.gp	$r1, [ + #-5466]
L_10440: movi	$r1, #0x2c9f8
L_10444: sbi.gp	$r0, [ + #-5637]
L_10448: sbi333 $r0, [$r1 + #0]
L_1044a: sbi333 $r0, [$r1 + #1]
L_1044c: sbi333 $r0, [$r1 + #2]
L_1044e: sbi333 $r0, [$r1 + #3]
L_10450: ret5 $lp
L_10452: push25 $r6, #0
L_10454: lbi.gp	$r2, [ + #-4066]
L_10458: movi55 $r6, #3
L_1045a: divr	$r1, $r2, $r2, $r6
L_1045e: movi	$r3, #0x320
L_10462: zeb33 $r2, $r2
L_10464: movi	$r1, #0x28178
L_10468: maddr32	$r1, $r2, $r3
L_1046c: ex9.it #29
L_1046e: ex9.it #460
L_10470: lbi.gp	$r0, [ + #-4066]
L_10474: addi45 $r0, #1
L_10476: zeb33 $r0, $r0
L_10478: divr	$r6, $r1, $r0, $r6
L_1047c: zeb33 $r1, $r1
L_1047e: cmovn	$r1, $r0, $r1
L_10482: sbi.gp	$r1, [ + #-4066]
L_10486: pop25 $r6, #0
L_10488: push25 $r6, #0
L_1048a: lbi.gp	$r2, [ + #-4067]
L_1048e: movi55 $r6, #3
L_10490: divr	$r1, $r2, $r2, $r6
L_10494: movi	$r3, #0x320
L_10498: zeb33 $r2, $r2
L_1049a: movi	$r1, #0x28178
L_1049e: maddr32	$r1, $r2, $r3
L_104a2: ex9.it #29
L_104a4: ex9.it #460
L_104a6: lbi.gp	$r0, [ + #-4067]
L_104aa: addi45 $r0, #1
L_104ac: zeb33 $r0, $r0
L_104ae: divr	$r6, $r1, $r0, $r6
L_104b2: zeb33 $r1, $r1
L_104b4: cmovn	$r1, $r0, $r1
L_104b8: sbi.gp	$r1, [ + #-4067]
L_104bc: pop25 $r6, #0
L_104be: movi55 $r0, #0
L_104c0: shi.gp	$r0, [ + #-5512]
L_104c4: shi.gp	$r0, [ + #-5554]
L_104c8: ex9.it #11
L_104ca: lbi333 $r0, [$r0 + #0]
L_104cc: beqz38 $r0, . + (8)
L_104ce: subi333 $r1, $r0, #3
L_104d0: slti45 $r1, #2
L_104d2: beqzs8 . + (36)
L_104d4: movi	$r0, #0x37320
L_104d8: lbi333 $r0, [$r0 + #0]
L_104da: shi.gp	$r0, [ + #-5554]
L_104de: movi	$r1, #0x37322
L_104e2: movi	$r0, #0x37321
L_104e6: movi	$r2, #0x37323
L_104ea: lbi333 $r0, [$r0 + #0]
L_104ec: lbi333 $r1, [$r1 + #0]
L_104ee: lbi333 $r2, [$r2 + #0]
L_104f0: shi.gp	$r2, [ + #-5512]
L_104f4: j8 . + (76)
L_104f6: subi333 $r1, $r0, #1
L_104f8: slti45 $r1, #2
L_104fa: bnezs8 . + (6)
L_104fc: bnec	$r0, #5, . + (64)
L_10500: movi55 $r0, #0
L_10502: movi	$r5, #0x37320
L_10506: mov55 $r1, $r0
L_10508: movi	$r3, #0x37334
L_1050c: lbi333 $r4, [$r5 + #0]
L_1050e: lhi.gp	$r2, [ + #-5554]
L_10512: addi45 $r5, #4
L_10514: add45 $r2, $r4
L_10516: shi.gp	$r2, [ + #-5554]
L_1051a: lbi	$r2, [$r5 + #-3]
L_1051e: add45 $r0, $r2
L_10520: lbi	$r2, [$r5 + #-2]
L_10524: lbi	$r4, [$r5 + #-1]
L_10528: add45 $r1, $r2
L_1052a: lhi.gp	$r2, [ + #-5512]
L_1052e: zeh33 $r0, $r0
L_10530: add45 $r2, $r4
L_10532: zeh33 $r1, $r1
L_10534: shi.gp	$r2, [ + #-5512]
L_10538: bnes38 $r3, . + (-44)
L_1053a: j8 . + (6)
L_1053c: movi55 $r0, #0
L_1053e: mov55 $r1, $r0
L_10540: lhi.gp	$r2, [ + #-5512]
L_10544: add45 $r1, $r2
L_10546: lhi.gp	$r2, [ + #-5554]
L_1054a: zeh33 $r1, $r1
L_1054c: add45 $r0, $r2
L_1054e: zeh33 $r0, $r0
L_10550: shi.gp	$r1, [ + #-5512]
L_10554: add45 $r1, $r0
L_10556: shi.gp	$r0, [ + #-5554]
L_1055a: shi.gp	$r1, [ + #-5516]
L_1055e: ret5 $lp
L_10560: push25 $r6, #0
L_10562: movi55 $r3, #0
L_10564: mov55 $r5, $r3
L_10566: movi55 $r6, #-1
L_10568: zeb33 $r4, $r3
L_1056a: slt45 $r4, $r2
L_1056c: beqzs8 . + (14)
L_1056e: sh	$r5, [$r0 + ($r3 << #1)]
L_10572: sh	$r6, [$r1 + ($r3 << #1)]
L_10576: addi45 $r3, #1
L_10578: j8 . + (-16)
L_1057a: pop25 $r6, #0
L_1057c: push25 $r6, #0
L_1057e: movi55 $r5, #0
L_10580: add_slli	$r3, $r1, $r3, #1
L_10584: beq	$r1, $r3, . + (48)
L_10588: lhi333 $r4, [$r0 + #0]
L_1058a: lhi333 $r6, [$r1 + #0]
L_1058c: slt45 $r6, $r4
L_1058e: beqzs8 . + (6)
L_10590: lhi333 $r4, [$r0 + #0]
L_10592: shi333 $r4, [$r1 + #0]
L_10594: lhi333 $r6, [$r0 + #0]
L_10596: lhi333 $r4, [$r2 + #0]
L_10598: slt45 $r6, $r4
L_1059a: beqzs8 . + (6)
L_1059c: lhi333 $r4, [$r0 + #0]
L_1059e: shi333 $r4, [$r2 + #0]
L_105a0: lhi.bi	$r4, [$r1], #2
L_105a4: lhi.bi	$r6, [$r2], #2
L_105a8: sub45 $r4, $r6
L_105aa: slts45 $r5, $r4
L_105ac: beqzs8 . + (4)
L_105ae: zeh33 $r5, $r4
L_105b0: addi45 $r0, #2
L_105b2: j8 . + (-46)
L_105b4: mov55 $r0, $r5
L_105b6: pop25 $r6, #0
L_105b8: push25 $r8, #0
L_105ba: movi55 $r5, #0
L_105bc: mov55 $r19, $r0
L_105be: srli	$r18, $r2, #0x10
L_105c2: mov55 $r4, $r2
L_105c4: mov55 $r0, $r5
L_105c6: zeh33 $r7, $r5
L_105c8: slt45 $r7, $r3
L_105ca: beqzs8 . + (124)
L_105cc: slti	$r15, $r7, #0x64
L_105d0: bnezs8 . + (12)
L_105d2: addi	$r15, $r7, #-200
L_105d6: slti	$r15, $r15, #0x64
L_105da: beqzs8 . + (100)
L_105dc: add_slli	$r6, $r19, $r5, #1
L_105e0: lhi	$r15, [$r6 + #0]
L_105e4: lhi	$r17, [$r1 + #0]
L_105e8: slt45 $r17, $r15
L_105ea: beqzs8 . + (10)
L_105ec: lhi	$r16, [$r6 + #0]
L_105f0: shi	$r16, [$r1 + #0]
L_105f4: lhi	$r17, [$r6 + #0]
L_105f8: lhi	$r15, [$r4 + #0]
L_105fc: slt45 $r17, $r15
L_105fe: beqzs8 . + (10)
L_10600: lhi	$r16, [$r6 + #0]
L_10604: shi	$r16, [$r4 + #0]
L_10608: lhi	$r17, [$r1 + #0]
L_1060c: lhi	$r16, [$r4 + #0]
L_10610: sub	$r16, $r17, $r16
L_10614: slts45 $r0, $r16
L_10616: beqzs8 . + (40)
L_10618: shi.gp	$r17, [ + #0x18b4]
L_1061c: zeh	$r0, $r16
L_10620: lhi	$r16, [$r4 + #0]
L_10624: shi.gp	$r16, [ + #0x18b6]
L_10628: lhi333 $r6, [$r6 + #0]
L_1062a: shi.gp	$r6, [ + #0x18b8]
L_1062e: shi.gp	$r7, [ + #0x18ba]
L_10632: shi.gp	$r0, [ + #0x18bc]
L_10636: shi.gp	$r2, [ + #0x18be]
L_1063a: shi.gp	$r18, [ + #0x18c0]
L_1063e: addi45 $r5, #1
L_10640: addi45 $r1, #2
L_10642: addi45 $r4, #2
L_10644: j8 . + (-126)
L_10646: pop25 $r8, #0
L_10648: push25 $r6, #0
L_1064a: movi	$r4, #0x320
L_1064e: mov55 $r6, $r1
L_10650: movi	$r1, #0x30000
L_10654: maddr32	$r1, $r0, $r4
L_10658: mov55 $r0, $r1
L_1065a: mov55 $r1, $r2
L_1065c: mov55 $r2, $r3
L_1065e: movi	$r3, #0x12c
L_10662: jal	. + (-170)
L_10666: shi333 $r0, [$r6 + #0]
L_10668: pop25 $r6, #0
L_1066a: add_slli	$r1, $r0, $r1, #1
L_1066e: movi	$r3, #0xffff
L_10672: movi55 $r2, #0
L_10674: beq	$r0, $r1, . + (28)
L_10678: lhi333 $r4, [$r0 + #0]
L_1067a: slt45 $r2, $r4
L_1067c: beqzs8 . + (6)
L_1067e: lhi333 $r2, [$r0 + #0]
L_10680: zeh33 $r2, $r2
L_10682: lhi333 $r4, [$r0 + #0]
L_10684: slt45 $r4, $r3
L_10686: beqzs8 . + (6)
L_10688: lhi333 $r3, [$r0 + #0]
L_1068a: zeh33 $r3, $r3
L_1068c: addi45 $r0, #2
L_1068e: j8 . + (-26)
L_10690: ex9.it #427
L_10692: min	$r0, $r3, $r0
L_10696: sub333 $r0, $r2, $r0
L_10698: zeh33 $r0, $r0
L_1069a: ret5 $lp
L_1069c: push25 $r14, #0
L_1069e: movi	$r6, #0x320
L_106a2: mul33 $r0, $r6
L_106a4: movi	$r8, #0x30190
L_106a8: movi	$r6, #0x30000
L_106ac: addi	$r11, $r3, #0x190
L_106b0: addi	$r10, $r4, #0x190
L_106b4: mov55 $r7, $r1
L_106b6: mov55 $r9, $r2
L_106b8: add45 $r8, $r0
L_106ba: add45 $r6, $r0
L_106bc: ex9.it #179
L_106be: mov55 $r0, $r6
L_106c0: ex9.it #349
L_106c2: lhi	$r1, [$r9 + #0]
L_106c6: mov55 $r13, $r9
L_106c8: slt45 $r1, $r0
L_106ca: beqzs8 . + (6)
L_106cc: shi	$r0, [$r9 + #0]
L_106d0: addi	$r12, $r6, #0x190
L_106d4: ex9.it #179
L_106d6: mov55 $r0, $r12
L_106d8: ex9.it #349
L_106da: lhi	$r1, [$r13 + #0]
L_106de: addi45 $r9, #2
L_106e0: slt45 $r1, $r0
L_106e2: beqzs8 . + (6)
L_106e4: shi	$r0, [$r9 + #-2]
L_106e8: addi	$r1, $r11, #-400
L_106ec: mov55 $r0, $r6
L_106ee: addi	$r2, $r10, #-400
L_106f2: ifcall9 . + (18)
L_106f4: lhi333 $r1, [$r7 + #0]
L_106f6: mov55 $r13, $r7
L_106f8: slt45 $r1, $r0
L_106fa: beqzs8 . + (4)
L_106fc: shi333 $r0, [$r7 + #0]
L_106fe: mov55 $r1, $r11
L_10700: mov55 $r0, $r12
L_10702: mov55 $r2, $r10
L_10704: ex9.it #352
L_10706: ex9.it #267
L_10708: lhi	$r1, [$r13 + #0]
L_1070c: addi45 $r7, #2
L_1070e: slt45 $r1, $r0
L_10710: beqzs8 . + (6)
L_10712: shi	$r0, [$r7 + #-2]
L_10716: addi	$r6, $r6, #0x64
L_1071a: addi	$r11, $r11, #0x64
L_1071e: addi	$r10, $r10, #0x64
L_10722: bne	$r6, $r8, . + (-102)
L_10726: pop25 $r14, #0
L_10728: push25 $r6, #0
L_1072a: lbi.gp	$r0, [ + #-4825]
L_1072e: lbi.gp	$r1, [ + #-4754]
L_10732: beqz38 $r0, . + (64)
L_10734: lwi.gp	$r0, [ + #-54776]
L_10738: bnec	$r1, #1, . + (10)
L_1073c: srli45 $r0, #1
L_1073e: zeh33 $r0, $r0
L_10740: j8 . + (6)
L_10742: srli45 $r0, #2
L_10744: zeh33 $r0, $r0
L_10746: lwi.gp	$r2, [ + #-4808]
L_1074a: slt45 $r2, $r0
L_1074c: lbi.gp	$r0, [ + #-4790]
L_10750: beqzs8 . + (12)
L_10752: slti45 $r0, #0x1e
L_10754: beqzs8 . + (30)
L_10756: addi45 $r0, #1
L_10758: ex9.it #428
L_1075a: j8 . + (24)
L_1075c: beqz38 $r0, . + (22)
L_1075e: slti	$r15, $r2, #0xe10
L_10762: bnezs8 . + (12)
L_10764: slti45 $r0, #0xf
L_10766: bnezs8 . + (8)
L_10768: subi45 $r0, #5
L_1076a: ex9.it #428
L_1076c: j8 . + (6)
L_1076e: subi45 $r0, #1
L_10770: ex9.it #428
L_10772: bnec	$r1, #1, . + (34)
L_10776: lbi.gp	$r0, [ + #-4790]
L_1077a: bnec	$r0, #0x1e, . + (10)
L_1077e: movi55 $r0, #5
L_10780: ex9.it #468
L_10782: j8 . + (24)
L_10784: slti45 $r0, #0x14
L_10786: bnezs8 . + (8)
L_10788: movi55 $r0, #6
L_1078a: ex9.it #468
L_1078c: j8 . + (14)
L_1078e: movi55 $r0, #7
L_10790: ex9.it #468
L_10792: j8 . + (8)
L_10794: jal	. + (-6204)
L_10798: ex9.it #468
L_1079a: pop25 $r6, #0
L_1079c: lhi.gp	$r3, [ + #-4078]
L_107a0: slti	$r15, $r3, #0x33
L_107a4: bnezs8 . + (38)
L_107a6: push25 $r6, #0
L_107a8: movi55 $r4, #0
L_107aa: movi55 $r6, #3
L_107ac: slt45 $r4, $r2
L_107ae: beqzs8 . + (26)
L_107b0: add_slli	$r3, $r0, $r4, #1
L_107b4: lhi333 $r5, [$r3 + #0]
L_107b6: lhi333 $r3, [$r1 + #0]
L_107b8: addi45 $r4, #1
L_107ba: mul33 $r3, $r6
L_107bc: srai45 $r3, #2
L_107be: add_srli	$r3, $r3, $r5, #2
L_107c2: shi.bi	$r3, [$r1], #2
L_107c6: j8 . + (-26)
L_107c8: pop25 $r6, #0
L_107ca: ret5 $lp
L_107cc: movi55 $r0, #0
L_107ce: sbi.gp	$r0, [ + #-4791]
L_107d2: movi55 $r0, #0
L_107d4: ex9.it #225
L_107d6: ret5 $lp
L_107d8: push25 $r6, #0
L_107da: lhi.gp	$r0, [ + #-4078]
L_107de: ex9.it #433
L_107e0: bnezs8 . + (10)
L_107e2: ex9.it #212
L_107e4: movi55 $r0, #0
L_107e6: shi.gp	$r0, [ + #-5468]
L_107ea: ex9.it #346
L_107ec: ex9.it #433
L_107ee: bnezs8 . + (6)
L_107f0: movi55 $r0, #0
L_107f2: ex9.it #199
L_107f4: pop25 $r6, #0
L_107f6: push25 $r6, #0
L_107f8: mov55 $r6, $r0
L_107fa: movi55 $r0, #0
L_107fc: shi.gp	$r0, [ + #-5516]
L_10800: jal	. + (-834)
L_10804: lbi.gp	$r0, [ + #-5684]
L_10808: divsr	$r6, $r6, $r6, $r0
L_1080c: ex9.it #6
L_1080e: lhi.gp	$r2, [ + #-55488]
L_10812: beqc	$r0, #2, . + (16)
L_10816: movi55 $r1, #3
L_10818: mul33 $r1, $r2
L_1081a: zeh33 $r1, $r1
L_1081c: slt45 $r6, $r1
L_1081e: bnezs8 . + (22)
L_10820: j8 . + (28)
L_10822: ex9.it #407
L_10824: slt45 $r6, $r2
L_10826: lbi	$r1, [$r0 + #0x2b]
L_1082a: beqzs8 . + (18)
L_1082c: lhi.gp	$r0, [ + #-5516]
L_10830: slt45 $r1, $r0
L_10832: bnezs8 . + (10)
L_10834: movi55 $r0, #0
L_10836: ex9.it #199
L_10838: movi55 $r0, #0
L_1083a: j8 . + (4)
L_1083c: movi55 $r0, #1
L_1083e: pop25 $r6, #0
L_10840: lwi450 $r3, [$r1]
L_10842: sbi333 $r0, [$r3 + #0]
L_10844: bnec	$r2, #2, . + (8)
L_10848: srli45 $r0, #8
L_1084a: sbi333 $r0, [$r3 + #1]
L_1084c: lwi450 $r0, [$r1]
L_1084e: add45 $r2, $r0
L_10850: swi450 $r2, [$r1]
L_10852: ret5 $lp
L_10854: push25 $r10, #32
L_10856: mov55 $r10, $r0
L_10858: swi450 $r0, [$sp]
L_1085a: mov55 $r9, $r1
L_1085c: movi55 $r0, #0
L_1085e: addri36.sp $r7, #4
L_10860: lb	$r1, [$r10 + ($r0 << #0)]
L_10864: sb	$r1, [$r0 + ($r7 << #0)]
L_10868: addi45 $r0, #1
L_1086a: bnec	$r0, #0x1a, . + (-10)
L_1086e: movi	$r0, #0x1900
L_10872: ifcall9 . + (136)
L_10874: ex9.it #152
L_10876: lwi450 $r1, [$sp]
L_10878: lbi333 $r2, [$r0 + #0]
L_1087a: sbi333 $r2, [$r1 + #0]
L_1087c: lwi	$r0, [$r0 + #0x78]
L_10880: lwi450 $r1, [$sp]
L_10882: srli45 $r0, #0x10
L_10884: addi333 $r2, $r1, #1
L_10886: fexti33 $r0, #5
L_10888: swi450 $r2, [$sp]
L_1088a: sbi333 $r0, [$r1 + #1]
L_1088c: ifcall9 . + (86)
L_1088e: movi55 $r2, #2
L_10890: movi	$r0, #0x280
L_10894: ifcall9 . + (106)
L_10896: ex9.it #109
L_10898: mov55 $r1, $sp
L_1089a: lwi	$r6, [$r0 + #0x50]
L_1089e: ex9.it #161
L_108a0: srli45 $r6, #0x10
L_108a2: ex9.it #102
L_108a4: add_slli	$r6, $r0, $r6, #2
L_108a8: mov55 $fp, $r6
L_108aa: lwi37 $r0, [$fp + #0x38]
L_108ac: movi55 $r2, #2
L_108ae: ifcall9 . + (66)
L_108b0: ifcall9 . + (70)
L_108b2: lwi37 $r0, [$fp + #0x2c]
L_108b4: ifcall9 . + (82)
L_108b6: ifcall9 . + (70)
L_108b8: lwi450 $r0, [$sp]
L_108ba: lbi	$r1, [$r6 + #0x28]
L_108be: sbi333 $r1, [$r0 + #0]
L_108c0: lwi450 $r0, [$sp]
L_108c2: movi55 $r2, #2
L_108c4: addi333 $r1, $r0, #1
L_108c6: swi450 $r1, [$sp]
L_108c8: lwi.gp	$r1, [ + #-392]
L_108cc: lwi	$r6, [$r1 + #0x50]
L_108d0: movi	$r1, #0x20180
L_108d4: srli45 $r6, #0x10
L_108d6: ex9.it #102
L_108d8: add_slli	$r6, $r1, $r6, #2
L_108dc: lbi	$r1, [$r6 + #0x28]
L_108e0: sbi333 $r1, [$r0 + #1]
L_108e2: lwi450 $r0, [$sp]
L_108e4: mov55 $r1, $sp
L_108e6: addi45 $r0, #1
L_108e8: swi450 $r0, [$sp]
L_108ea: ifret16
L_108ec: lwi	$r0, [$r6 + #0x38]
L_108f0: srli45 $r0, #0x10
L_108f2: x11b33 $r0, $r0
L_108f4: ifcall9 . + (10)
L_108f6: lhi	$r0, [$r6 + #0x2c]
L_108fa: mov55 $r1, $sp
L_108fc: movi55 $r2, #2
L_108fe: jal	. + (-190)
L_10902: lwi	$r0, [$r6 + #0x2c]
L_10906: mov55 $r1, $sp
L_10908: srli45 $r0, #0x10
L_1090a: andi	$r0, $r0, #0xfff
L_1090e: ifret16
L_10910: ifcall	. + (-20)
L_10914: movi	$r0, #-86
L_10918: sbi	$r0, [$r9 + #0x51]
L_1091c: ex9.it #83
L_1091e: ex9.it #60
L_10920: lbi	$r0, [$r9 + #0x51]
L_10924: beqc	$r0, #0xcc, . + (8)
L_10928: sbi333 $r2, [$r1 + #0]
L_1092a: j8 . + (-10)
L_1092c: ex9.it #2
L_1092e: beqz38 $r0, . + (14)
L_10930: movi	$r1, #0x2fe51
L_10934: movi55 $r0, #2
L_10936: mov55 $r2, $r1
L_10938: movi55 $r3, #1
L_1093a: ex9.it #5
L_1093c: movi55 $r0, #0
L_1093e: lb	$r1, [$r7 + ($r0 << #0)]
L_10942: sb	$r1, [$r10 + ($r0 << #0)]
L_10946: addi45 $r0, #1
L_10948: bnec	$r0, #0x1a, . + (-10)
L_1094c: movi55 $r0, #0
L_1094e: sbi	$r0, [$r9 + #0x51]
L_10952: pop25 $r10, #32
L_10954: push25 $r6, #0
L_10956: movi	$r0, #0x30c80
L_1095a: lwi.gp	$r1, [ + #0x774]
L_1095e: movi	$r2, #0xc8
L_10962: ex9.it #460
L_10964: pop25 $r6, #0
L_10966: push25 $r6, #0
L_10968: lbi.gp	$r0, [ + #-5669]
L_1096c: subi45 $r0, #1
L_1096e: slti45 $r0, #2
L_10970: beqzs8 . + (16)
L_10972: ex9.it #29
L_10974: movi	$r0, #0x30c80
L_10978: ex9.it #121
L_1097a: swi.gp	$r2, [ + #-4936]
L_1097e: ex9.it #460
L_10980: lbi.gp	$r0, [ + #-5669]
L_10984: ex9.it #298
L_10986: bnec	$r0, #1, . + (44)
L_1098a: movi	$r5, #0x30c80
L_1098e: movi	$r2, #-35592
L_10992: movi	$r3, #0x960
L_10996: movi	$r4, #0x30fa0
L_1099a: movi55 $r0, #0
L_1099c: add333 $r1, $r5, $r2
L_1099e: lhi333 $r6, [$r5 + #0]
L_109a0: sh	$r6, [$r1 + ($r0 << #0)]
L_109a4: addi	$r0, $r0, #0x320
L_109a8: bne	$r0, $r3, . + (-10)
L_109ac: addi45 $r5, #2
L_109ae: bnes38 $r4, . + (-20)
L_109b0: pop25 $r6, #0
L_109b2: pop25 $r6, #0
L_109b4: push25 $r6, #0
L_109b6: jal	. + (-478)
L_109ba: jal	. + (-102)
L_109be: lbi.gp	$r0, [ + #-5669]
L_109c2: beqz38 $r0, . + (16)
L_109c4: ex9.it #212
L_109c6: ex9.it #11
L_109c8: lbi333 $r0, [$r0 + #0]
L_109ca: jal	. + (-100)
L_109ce: movi55 $r0, #0
L_109d0: ex9.it #464
L_109d2: pop25 $r6, #0
L_109d4: push25 $r6, #0
L_109d6: lwi	$r0, [$r0 + #0x50]
L_109da: movi	$r1, #0x20060
L_109de: srli45 $r0, #0x10
L_109e0: ex9.it #68
L_109e2: ex9.it #77
L_109e4: lbi.gp	$r1, [ + #-55480]
L_109e8: sbi	$r1, [$r0 + #0x18]
L_109ec: lbi.gp	$r1, [ + #-55479]
L_109f0: sbi	$r1, [$r0 + #0x19]
L_109f4: lbi.gp	$r1, [ + #-55482]
L_109f8: sbi	$r1, [$r0 + #0x1b]
L_109fc: lbi.gp	$r1, [ + #-55481]
L_10a00: sbi	$r1, [$r0 + #0x1a]
L_10a04: lwi333 $r1, [$r0 + #0x14]
L_10a06: bset	$r1, $r1, #0x1c
L_10a0a: swi333 $r1, [$r0 + #0x14]
L_10a0c: lwi333 $r1, [$r0 + #0x14]
L_10a0e: bset	$r1, $r1, #0x1d
L_10a12: swi333 $r1, [$r0 + #0x14]
L_10a14: lwi333 $r1, [$r0 + #0x14]
L_10a16: bset	$r1, $r1, #0x1f
L_10a1a: swi333 $r1, [$r0 + #0x14]
L_10a1c: lwi333 $r1, [$r0 + #0x14]
L_10a1e: bset	$r1, $r1, #0x1e
L_10a22: swi333 $r1, [$r0 + #0x14]
L_10a24: ex9.it #2
L_10a26: beqz38 $r0, . + (10)
L_10a28: movi	$r0, #0x37320
L_10a2c: ifcall9 . + (8)
L_10a2e: pop25 $r6, #0
L_10a30: movi	$r0, #0x37384
L_10a34: jal	. + (-9920)
L_10a38: pop25 $r6, #0
L_10a3a: swi.gp	$r0, [ + #-4060]
L_10a3e: ret5 $lp
L_10a40: lbi.gp	$r3, [ + #-4754]
L_10a44: beqc	$r3, #1, . + (12)
L_10a48: lbi.gp	$r3, [ + #-5561]
L_10a4c: bnec	$r3, #1, . + (56)
L_10a50: movi55 $r3, #0
L_10a52: sbi.gp	$r3, [ + #-5561]
L_10a56: lbi	$r3, [$r0 + #0x10]
L_10a5a: sbi.gp	$r3, [ + #-54799]
L_10a5e: lwi333 $r0, [$r0 + #0x14]
L_10a60: swi.gp	$r0, [ + #-54796]
L_10a64: lbi333 $r0, [$r1 + #2]
L_10a66: sbi.gp	$r0, [ + #-54724]
L_10a6a: lbi333 $r0, [$r1 + #3]
L_10a6c: sbi.gp	$r0, [ + #-54723]
L_10a70: lhi333 $r0, [$r2 + #4]
L_10a72: shi.gp	$r0, [ + #-54682]
L_10a76: lhi333 $r0, [$r2 + #6]
L_10a78: shi.gp	$r0, [ + #-54680]
L_10a7c: movpi45 $r0, #0x14
L_10a7e: sbi.gp	$r0, [ + #-54678]
L_10a82: ret5 $lp
L_10a84: lbi333 $r3, [$r0 + #0]
L_10a86: sbi.gp	$r3, [ + #-54799]
L_10a8a: lwi333 $r0, [$r0 + #0xc]
L_10a8c: swi.gp	$r0, [ + #-54796]
L_10a90: lbi333 $r0, [$r1 + #0]
L_10a92: sbi.gp	$r0, [ + #-54724]
L_10a96: lbi333 $r0, [$r1 + #1]
L_10a98: sbi.gp	$r0, [ + #-54723]
L_10a9c: lhi333 $r0, [$r2 + #0]
L_10a9e: shi.gp	$r0, [ + #-54682]
L_10aa2: lhi333 $r0, [$r2 + #2]
L_10aa4: shi.gp	$r0, [ + #-54680]
L_10aa8: movi55 $r0, #4
L_10aaa: sbi.gp	$r0, [ + #-54678]
L_10aae: ret5 $lp
L_10ab0: beqc	$r0, #1, . + (10)
L_10ab4: movi	$r1, #0x28178
L_10ab8: j8 . + (4)
L_10aba: ex9.it #121
L_10abc: addi	$r4, $r1, #0x320
L_10ac0: mov55 $r5, $r1
L_10ac2: movi55 $r0, #0
L_10ac4: shi.bi	$r0, [$r5], #2
L_10ac8: bnes38 $r4, . + (-4)
L_10aca: push25 $r8, #0
L_10acc: movi	$r5, #0x30000
L_10ad0: movi	$r6, #0x30c80
L_10ad4: mov55 $r0, $r1
L_10ad6: mov55 $r3, $r5
L_10ad8: lhi.bi	$r2, [$r3], #2
L_10adc: lhi333 $r7, [$r0 + #0]
L_10ade: add_srli	$r2, $r7, $r2, #2
L_10ae2: shi.bi	$r2, [$r0], #2
L_10ae6: bne	$r0, $r4, . + (-14)
L_10aea: addi	$r5, $r5, #0x320
L_10aee: bnes38 $r6, . + (-26)
L_10af0: movi55 $r0, #0
L_10af2: movi	$r3, #0x28178
L_10af6: movi	$r4, #0x960
L_10afa: movi55 $r5, #0
L_10afc: add333 $r2, $r0, $r3
L_10afe: lh	$r6, [$r1 + ($r0 << #0)]
L_10b02: sh	$r6, [$r2 + ($r5 << #0)]
L_10b06: addi	$r5, $r5, #0x320
L_10b0a: bnes38 $r4, . + (-12)
L_10b0c: addi45 $r0, #2
L_10b0e: bnec	$r0, #0x320, . + (-20)
L_10b12: pop25 $r8, #0
L_10b14: push25 $r14, #0
L_10b16: movi55 $r4, #0
L_10b18: movi55 $r7, #-1
L_10b1a: movi55 $r6, #1
L_10b1c: mov55 $r19, $r7
L_10b1e: mov55 $r5, $r4
L_10b20: mov55 $r17, $r4
L_10b22: movi	$r9, #0x2c9f8
L_10b26: mov55 $r11, $r6
L_10b28: mov55 $r8, $r4
L_10b2a: lhi	$r16, [$r0 + #0]
L_10b2e: andi	$r18, $r4, #0xff
L_10b32: slt45 $r16, $r2
L_10b34: mov55 $r10, $r0
L_10b36: beqzs8 . + (28)
L_10b38: lhi	$r16, [$r1 + #0]
L_10b3c: slt45 $r16, $r3
L_10b3e: beqzs8 . + (20)
L_10b40: sb	$r8, [$r9 + ($r4 << #0)]
L_10b44: lhi333 $r6, [$r0 + #0]
L_10b46: slt45 $r6, $r19
L_10b48: beqzs8 . + (16)
L_10b4a: mov55 $r19, $r6
L_10b4c: mov55 $r17, $r18
L_10b4e: movi55 $r6, #0
L_10b50: j8 . + (10)
L_10b52: sb	$r11, [$r9 + ($r4 << #0)]
L_10b56: j8 . + (4)
L_10b58: movi55 $r6, #0
L_10b5a: lhi.bi	$r16, [$r1], #2
L_10b5e: lhi	$r10, [$r10 + #0]
L_10b62: addi45 $r4, #1
L_10b64: add45 $r16, $r10
L_10b66: slt45 $r16, $r7
L_10b68: addi45 $r0, #2
L_10b6a: cmovn	$r7, $r16, $r15
L_10b6e: cmovn	$r5, $r18, $r15
L_10b72: bnec	$r4, #4, . + (-72)
L_10b76: beqc	$r6, #1, . + (6)
L_10b7a: mov55 $r5, $r17
L_10b7c: mov55 $r0, $r5
L_10b7e: pop25 $r14, #0
L_10b80: push25 $r6, #0
L_10b82: lbi.gp	$r1, [ + #-5614]
L_10b86: lwi.gp	$r6, [ + #-444]
L_10b8a: beqz38 $r1, . + (22)
L_10b8c: movi55 $r0, #0
L_10b8e: sbi.gp	$r0, [ + #-5614]
L_10b92: movi	$r1, #0xe6
L_10b96: movi55 $r0, #6
L_10b98: lwi.gp	$r6, [ + #-352]
L_10b9c: ex9.it #7
L_10b9e: j8 . + (104)
L_10ba0: bnez38 $r0, . + (20)
L_10ba2: ex9.it #27
L_10ba4: beqc	$r0, #1, . + (98)
L_10ba8: ex9.it #27
L_10baa: bnec	$r0, #2, . + (92)
L_10bae: lwi.gp	$r6, [ + #-384]
L_10bb2: j8 . + (84)
L_10bb4: bnec	$r0, #3, . + (28)
L_10bb8: ex9.it #27
L_10bba: bnec	$r0, #1, . + (10)
L_10bbe: lwi.gp	$r6, [ + #-440]
L_10bc2: j8 . + (68)
L_10bc4: ex9.it #27
L_10bc6: bnec	$r0, #2, . + (64)
L_10bca: lwi.gp	$r6, [ + #-380]
L_10bce: j8 . + (56)
L_10bd0: bnec	$r0, #1, . + (28)
L_10bd4: ex9.it #27
L_10bd6: bnec	$r0, #1, . + (10)
L_10bda: lwi.gp	$r6, [ + #-412]
L_10bde: j8 . + (40)
L_10be0: ex9.it #27
L_10be2: bnec	$r0, #2, . + (36)
L_10be6: lwi.gp	$r6, [ + #-376]
L_10bea: j8 . + (28)
L_10bec: bnec	$r0, #2, . + (26)
L_10bf0: ex9.it #27
L_10bf2: bnec	$r0, #1, . + (10)
L_10bf6: lwi.gp	$r6, [ + #-408]
L_10bfa: j8 . + (12)
L_10bfc: ex9.it #27
L_10bfe: bnec	$r0, #2, . + (8)
L_10c02: lwi.gp	$r6, [ + #-372]
L_10c06: mov55 $r0, $r6
L_10c08: pop25 $r6, #0
L_10c0a: push25 $r6, #0
L_10c0c: lbi.gp	$r0, [ + #-5641]
L_10c10: bnez38 $r0, . + (86)
L_10c12: lwi.gp	$r0, [ + #-4768]
L_10c16: slti	$r15, $r0, #0x32
L_10c1a: bnezs8 . + (8)
L_10c1c: lbi.gp	$r0, [ + #-4812]
L_10c20: beqz38 $r0, . + (12)
L_10c22: lbi.gp	$r0, [ + #-4813]
L_10c26: slti	$r15, $r0, #0x32
L_10c2a: bnezs8 . + (60)
L_10c2c: movi55 $r1, #0
L_10c2e: shi.gp	$r1, [ + #-5528]
L_10c32: movi55 $r0, #0
L_10c34: movi	$r3, #0x26eb8
L_10c38: ex9.it #314
L_10c3a: slt45 $r0, $r2
L_10c3c: beqzs8 . + (12)
L_10c3e: add333 $r2, $r0, $r3
L_10c40: addi45 $r0, #1
L_10c42: sbi333 $r1, [$r2 + #0]
L_10c44: zeh33 $r0, $r0
L_10c46: j8 . + (-14)
L_10c48: lwi.gp	$r0, [ + #0x764]
L_10c4c: lwi.gp	$r1, [ + #0x76c]
L_10c50: ex9.it #69
L_10c52: movi	$r0, #0x33
L_10c56: ex9.it #225
L_10c58: ex9.it #121
L_10c5a: ex9.it #365
L_10c5c: ex9.it #29
L_10c5e: jal	. + (-1218)
L_10c62: movi55 $r0, #1
L_10c64: ex9.it #174
L_10c66: pop25 $r6, #0
L_10c68: add_slli	$r3, $r0, $r3, #1
L_10c6c: beq	$r0, $r3, . + (10)
L_10c70: shi.bi	$r2, [$r0], #2
L_10c74: j8 . + (-8)
L_10c76: ret5 $lp
L_10c78: push25 $r10, #0
L_10c7a: addi	$sp, $sp, #-1608
L_10c7e: swi37.sp $r0, [ + #4]
L_10c80: movi55 $r0, #0
L_10c82: ex9.it #310
L_10c84: ex9.it #121
L_10c86: addri36.sp $r0, #8
L_10c88: movi55 $r2, #0
L_10c8a: ex9.it #34
L_10c8c: ex9.it #441
L_10c8e: addi	$r0, $sp, #0x328
L_10c92: ex9.it #121
L_10c94: ex9.it #292
L_10c96: ex9.it #34
L_10c98: ex9.it #441
L_10c9a: movi55 $r6, #0
L_10c9c: movi	$r10, #0x320
L_10ca0: movi	$r9, #0x28178
L_10ca4: movi	$r7, #0x2f27c
L_10ca8: mov55 $r0, $r9
L_10caa: maddr32	$r0, $r6, $r10
L_10cae: addri36.sp $r1, #8
L_10cb0: addi	$r2, $sp, #0x328
L_10cb4: ex9.it #34
L_10cb6: jal	. + (-1790)
L_10cba: sh	$r0, [$r7 + ($r6 << #1)]
L_10cbe: addi45 $r6, #1
L_10cc0: ex9.it #310
L_10cc2: bnec	$r6, #3, . + (-26)
L_10cc6: addri36.sp $r1, #8
L_10cc8: addi	$r2, $sp, #0x328
L_10ccc: ex9.it #34
L_10cce: lwi37.sp $r0, [ + #4]
L_10cd0: jal	. + (-1816)
L_10cd4: ex9.it #310
L_10cd6: shi.gp	$r0, [ + #0x18a4]
L_10cda: ex9.it #121
L_10cdc: lwi37.sp $r0, [ + #4]
L_10cde: jal	. + (-2188)
L_10ce2: ex9.it #11
L_10ce4: lbi333 $r0, [$r0 + #0]
L_10ce6: subi45 $r0, #3
L_10ce8: slti45 $r0, #2
L_10cea: beqzs8 . + (34)
L_10cec: lhi.gp	$r0, [ + #-5466]
L_10cf0: movi	$r6, #0x32
L_10cf4: ex9.it #433
L_10cf6: bnezs8 . + (16)
L_10cf8: subi333 $r1, $r0, #1
L_10cfa: addi	$r0, $r0, #-50
L_10cfe: divsr	$r6, $r6, $r0, $r6
L_10d02: sub333 $r6, $r1, $r6
L_10d04: zeh33 $r6, $r6
L_10d06: shi.gp	$r6, [ + #-5466]
L_10d0a: j8 . + (8)
L_10d0c: ex9.it #407
L_10d0e: lhi	$r6, [$r0 + #0x1c]
L_10d12: ex9.it #6
L_10d14: beqc	$r0, #2, . + (10)
L_10d18: movi55 $r0, #3
L_10d1a: mul33 $r6, $r0
L_10d1c: zeh33 $r6, $r6
L_10d1e: lhi.gp	$r0, [ + #-4804]
L_10d22: lhi.gp	$r1, [ + #0x18a8]
L_10d26: shi.gp	$r0, [ + #0x18a6]
L_10d2a: slt45 $r1, $r0
L_10d2c: beqzs8 . + (6)
L_10d2e: shi.gp	$r0, [ + #0x18a8]
L_10d32: slt45 $r6, $r0
L_10d34: shi.gp	$r6, [ + #0x18aa]
L_10d38: shi.gp	$r0, [ + #0x16e0]
L_10d3c: shi.gp	$r6, [ + #0x16e2]
L_10d40: beqzs8 . + (10)
L_10d42: movi55 $r0, #2
L_10d44: sbi.gp	$r0, [ + #-4791]
L_10d48: j8 . + (44)
L_10d4a: lbi.gp	$r1, [ + #-4791]
L_10d4e: beqz38 $r1, . + (8)
L_10d50: subi45 $r1, #1
L_10d52: sbi.gp	$r1, [ + #-4791]
L_10d56: lhi.gp	$r1, [ + #-5482]
L_10d5a: slt45 $r1, $r0
L_10d5c: bnezs8 . + (20)
L_10d5e: lhi.gp	$r0, [ + #-4078]
L_10d62: movi	$r1, #0xea5f
L_10d66: slt45 $r1, $r0
L_10d68: bnezs8 . + (12)
L_10d6a: addi45 $r0, #1
L_10d6c: ex9.it #225
L_10d6e: j8 . + (6)
L_10d70: movi55 $r0, #0
L_10d72: ex9.it #225
L_10d74: ex9.it #370
L_10d76: slti45 $r0, #2
L_10d78: bnezs8 . + (14)
L_10d7a: movi55 $r0, #1
L_10d7c: ex9.it #137
L_10d7e: movi55 $r0, #0
L_10d80: sbi.gp	$r0, [ + #-4092]
L_10d84: j8 . + (18)
L_10d86: movi55 $r0, #0
L_10d88: ex9.it #137
L_10d8a: lbi.gp	$r0, [ + #-4092]
L_10d8e: beqz38 $r0, . + (8)
L_10d90: subi45 $r0, #1
L_10d92: sbi.gp	$r0, [ + #-4092]
L_10d96: addi	$sp, $sp, #0x648
L_10d9a: pop25 $r10, #0
L_10d9c: movi	$r0, #0x38d40
L_10da0: lbi333 $r1, [$r0 + #0]
L_10da2: lbi.gp	$r0, [ + #-5622]
L_10da6: beqz38 $r1, . + (12)
L_10da8: lbi.gp	$r1, [ + #-5623]
L_10dac: bnec	$r1, #1, . + (6)
L_10db0: addi45 $r0, #1
L_10db2: sbi.gp	$r0, [ + #-55576]
L_10db6: ret5 $lp
L_10db8: movi	$r1, #0x55
L_10dbc: ex9.it #3
L_10dbe: sbi333 $r1, [$r0 + #0]
L_10dc0: ex9.it #134
L_10dc2: lbi333 $r0, [$r1 + #0]
L_10dc4: bnez38 $r0, . + (-2)
L_10dc6: ret5 $lp
L_10dc8: ex9.it #496
L_10dca: movi	$r2, #0x200c0
L_10dce: lwi	$r0, [$r1 + #0x50]
L_10dd2: srli45 $r0, #0x10
L_10dd4: ex9.it #68
L_10dd6: add_slli	$r0, $r2, $r0, #2
L_10dda: lwi450 $r0, [$r0]
L_10ddc: srli45 $r0, #0xa
L_10dde: fexti33 $r0, #6
L_10de0: sbi.gp	$r0, [ + #-5655]
L_10de4: lwi	$r0, [$r1 + #0x50]
L_10de8: movi	$r1, #0x20000
L_10dec: srli45 $r0, #0x10
L_10dee: ex9.it #68
L_10df0: ex9.it #77
L_10df2: lwi450 $r1, [$r0]
L_10df4: lbi	$r0, [$r0 + #0x28]
L_10df8: srli45 $r1, #0xa
L_10dfa: fexti33 $r1, #6
L_10dfc: sbi.gp	$r1, [ + #-5684]
L_10e00: sbi.gp	$r0, [ + #-5647]
L_10e04: ret5 $lp
L_10e06: push25 $r6, #0
L_10e08: movi55 $r0, #0
L_10e0a: ex9.it #367
L_10e0c: ex9.it #12
L_10e0e: movi55 $r1, #1
L_10e10: sbi333 $r1, [$r0 + #0]
L_10e12: movi55 $r0, #1
L_10e14: mov55 $r1, $r0
L_10e16: ex9.it #398
L_10e18: ex9.it #3
L_10e1a: ex9.it #4
L_10e1c: sbi333 $r1, [$r0 + #0]
L_10e1e: lwi.gp	$r2, [ + #-5356]
L_10e22: lbi333 $r1, [$r2 + #0]
L_10e24: beqc	$r1, #1, . + (-2)
L_10e28: ex9.it #4
L_10e2a: sbi333 $r1, [$r0 + #0]
L_10e2c: movi55 $r0, #0xc
L_10e2e: ex9.it #33
L_10e30: ex9.it #12
L_10e32: movi55 $r1, #1
L_10e34: sbi333 $r1, [$r0 + #0]
L_10e36: movi55 $r0, #0
L_10e38: ex9.it #321
L_10e3a: sbi.gp	$r0, [ + #-4813]
L_10e3e: pop25 $r6, #0
L_10e40: push25 $r6, #0
L_10e42: movi55 $r0, #3
L_10e44: ex9.it #464
L_10e46: ex9.it #164
L_10e48: movi55 $r0, #0
L_10e4a: mov55 $r2, $r0
L_10e4c: ex9.it #18
L_10e4e: ex9.it #67
L_10e50: ex9.it #143
L_10e52: movi55 $r0, #0
L_10e54: ex9.it #174
L_10e56: sbi.gp	$r0, [ + #-4813]
L_10e5a: movi55 $r0, #1
L_10e5c: sbi.gp	$r0, [ + #-4812]
L_10e60: ex9.it #278
L_10e62: ex9.it #11
L_10e64: lbi333 $r0, [$r0 + #0]
L_10e66: jal	. + (-33598)
L_10e6a: ex9.it #212
L_10e6c: pop25 $r6, #0
L_10e6e: push25 $r8, #0
L_10e70: movi	$r4, #0x20000
L_10e74: lbi.gp	$r6, [ + #-55372]
L_10e78: movi55 $r1, #0
L_10e7a: movi	$r3, #0x38d40
L_10e7e: mov55 $r7, $r4
L_10e80: zeb33 $r2, $r1
L_10e82: slt45 $r2, $r6
L_10e84: beqzs8 . + (50)
L_10e86: lbi333 $r5, [$r3 + #0]
L_10e88: add333 $r2, $r0, $r1
L_10e8a: beqz38 $r5, . + (8)
L_10e8c: lbi	$r5, [$r2 + #0x20]
L_10e90: j8 . + (6)
L_10e92: lb	$r5, [$r0 + ($r1 << #0)]
L_10e96: add	$r16, $r4, $r1
L_10e9a: sbi	$r5, [$r16 + #0x184]
L_10e9e: lbi333 $r5, [$r3 + #0]
L_10ea0: beqz38 $r5, . + (8)
L_10ea2: lbi	$r2, [$r2 + #0x28]
L_10ea6: j8 . + (6)
L_10ea8: lbi	$r2, [$r2 + #8]
L_10eac: add333 $r5, $r7, $r1
L_10eae: sbi	$r2, [$r5 + #0x18c]
L_10eb2: addi45 $r1, #1
L_10eb4: j8 . + (-52)
L_10eb6: pop25 $r8, #0
L_10eb8: movi	$r2, #0x2d824
L_10ebc: lwi450 $r3, [$r2]
L_10ebe: movi	$r4, #0x20000
L_10ec2: ex9.it #395
L_10ec4: lwi333 $r2, [$r2 + #4]
L_10ec6: srli45 $r1, #0x10
L_10ec8: ex9.it #157
L_10eca: add_slli	$r1, $r4, $r1, #2
L_10ece: lwi	$r1, [$r1 + #0x28]
L_10ed2: srli45 $r1, #0x12
L_10ed4: andi	$r1, $r1, #0xfff
L_10ed8: seh33 $r1, $r1
L_10eda: shi	$r1, [$r0 + #0x102]
L_10ede: shi.gp	$r1, [ + #-5484]
L_10ee2: ex9.it #395
L_10ee4: lwi	$r2, [$r2 + #0x54]
L_10ee8: srli45 $r1, #0x10
L_10eea: movi	$r3, #0x20180
L_10eee: ex9.it #157
L_10ef0: ex9.it #361
L_10ef2: andi	$r2, $r2, #0x7fff
L_10ef6: movi	$r3, #0x2000c
L_10efa: add_slli	$r2, $r3, $r2, #2
L_10efe: lhi	$r3, [$r1 + #0x3e]
L_10f02: sbi	$r3, [$r0 + #0x104]
L_10f06: lhi	$r3, [$r1 + #0x3e]
L_10f0a: srli45 $r3, #8
L_10f0c: sbi	$r3, [$r0 + #0x105]
L_10f10: lhi	$r3, [$r1 + #0x3c]
L_10f14: sbi	$r3, [$r0 + #0x106]
L_10f18: lhi	$r3, [$r1 + #0x3c]
L_10f1c: srli45 $r3, #8
L_10f1e: sbi	$r3, [$r0 + #0x107]
L_10f22: lhi333 $r3, [$r2 + #6]
L_10f24: sbi	$r3, [$r0 + #0x108]
L_10f28: lhi333 $r3, [$r2 + #6]
L_10f2a: srli45 $r3, #8
L_10f2c: sbi	$r3, [$r0 + #0x109]
L_10f30: lhi333 $r3, [$r2 + #2]
L_10f32: sbi	$r3, [$r0 + #0x10a]
L_10f36: lhi333 $r3, [$r2 + #2]
L_10f38: srli45 $r3, #8
L_10f3a: sbi	$r3, [$r0 + #0x10b]
L_10f3e: lwi333 $r4, [$r2 + #8]
L_10f40: movi55 $r3, #1
L_10f42: srli45 $r4, #0x18
L_10f44: andi	$r5, $r4, #0x3f
L_10f48: lwi	$r4, [$r1 + #0x44]
L_10f4c: and_srli	$r4, $r3, $r4, #1
L_10f50: or_slli	$r4, $r5, $r4, #7
L_10f54: sbi	$r4, [$r0 + #0x10c]
L_10f58: lwi333 $r2, [$r2 + #8]
L_10f5a: lwi	$r1, [$r1 + #0x44]
L_10f5e: srli45 $r2, #8
L_10f60: fexti33 $r2, #5
L_10f62: and33 $r3, $r1
L_10f64: or_slli	$r3, $r2, $r3, #7
L_10f68: sbi	$r3, [$r0 + #0x10d]
L_10f6c: ret5 $lp
L_10f6e: push25 $r8, #0
L_10f70: mov55 $r7, $r0
L_10f72: mov55 $r6, $r1
L_10f74: ex9.it #362
L_10f76: ex9.it #2
L_10f78: beqz38 $r0, . + (24)
L_10f7a: addi	$r1, $r6, #0x104
L_10f7e: mov55 $r2, $r1
L_10f80: movi55 $r3, #0xa
L_10f82: movi55 $r0, #2
L_10f84: ex9.it #5
L_10f86: ex9.it #91
L_10f88: movi55 $r0, #2
L_10f8a: jal	. + (-53038)
L_10f8e: j8 . + (8)
L_10f90: movi55 $r0, #2
L_10f92: jal	. + (-53046)
L_10f96: ex9.it #152
L_10f98: movi	$r2, #0x20180
L_10f9c: mov55 $fp, $r0
L_10f9e: lwi37 $r1, [$fp + #0x50]
L_10fa0: lwi37 $r3, [$fp + #0x54]
L_10fa2: srli45 $r1, #0x10
L_10fa4: movi	$r0, #0x2000c
L_10fa8: andi	$r3, $r3, #0x7fff
L_10fac: add_slli	$r3, $r0, $r3, #2
L_10fb0: ex9.it #157
L_10fb2: lbi	$r0, [$r6 + #0x105]
L_10fb6: add_slli	$r1, $r2, $r1, #2
L_10fba: lbi	$r2, [$r6 + #0x104]
L_10fbe: add_slli	$r0, $r2, $r0, #8
L_10fc2: zeh33 $r0, $r0
L_10fc4: shi	$r0, [$r1 + #0x3e]
L_10fc8: lbi	$r0, [$r6 + #0x107]
L_10fcc: lbi	$r2, [$r6 + #0x106]
L_10fd0: add_slli	$r0, $r2, $r0, #8
L_10fd4: zeh33 $r0, $r0
L_10fd6: shi	$r0, [$r1 + #0x3c]
L_10fda: lbi	$r0, [$r6 + #0x10c]
L_10fde: srli	$r2, $r0, #7
L_10fe2: addi	$r4, $r1, #0x44
L_10fe6: lwi450 $r0, [$r4]
L_10fe8: ex9.it #185
L_10fea: or_slli	$r0, $r0, $r2, #1
L_10fee: swi450 $r0, [$r4]
L_10ff0: lwi450 $r0, [$r4]
L_10ff2: lbi	$r2, [$r6 + #0x10d]
L_10ff6: bitci	$r0, $r0, #1
L_10ffa: or_srli	$r0, $r0, $r2, #7
L_10ffe: swi450 $r0, [$r4]
L_11000: lbi	$r0, [$r6 + #0x109]
L_11004: lbi	$r1, [$r6 + #0x108]
L_11008: ex9.it #507
L_1100a: zeh33 $r0, $r0
L_1100c: shi333 $r0, [$r3 + #6]
L_1100e: lbi	$r0, [$r6 + #0x10b]
L_11012: lbi	$r1, [$r6 + #0x10a]
L_11016: ex9.it #507
L_11018: zeh33 $r0, $r0
L_1101a: shi333 $r0, [$r3 + #2]
L_1101c: lbi	$r0, [$r6 + #0x10c]
L_11020: lwi333 $r2, [$r3 + #8]
L_11022: andi	$r1, $r0, #0x3f
L_11026: sethi	$r0, #0xc0fff
L_1102a: addi	$r0, $r0, #0xfff
L_1102e: and33 $r0, $r2
L_11030: or_slli	$r0, $r0, $r1, #0x18
L_11034: swi333 $r0, [$r3 + #8]
L_11036: lbi	$r0, [$r6 + #0x10d]
L_1103a: andi	$r1, $r0, #0x3f
L_1103e: lwi333 $r0, [$r3 + #8]
L_11040: bitci	$r0, $r0, #0x3f00
L_11044: or_slli	$r0, $r0, $r1, #8
L_11048: swi333 $r0, [$r3 + #8]
L_1104a: beqz38 $r7, . + (6)
L_1104c: subi45 $r7, #1
L_1104e: zeb33 $r7, $r7
L_11050: movi55 $r0, #3
L_11052: ex9.it #18
L_11054: mov55 $r2, $r7
L_11056: ex9.it #67
L_11058: ex9.it #392
L_1105a: pop25 $r8, #0
L_1105c: push25 $r14, #0
L_1105e: mov55 $r9, $r1
L_11060: mov55 $r11, $r2
L_11062: mov55 $r10, $r3
L_11064: mov55 $r6, $r4
L_11066: ex9.it #21
L_11068: lbi333 $r0, [$r1 + #0]
L_1106a: zeb33 $r0, $r0
L_1106c: bnec	$r0, #1, . + (-4)
L_11070: ex9.it #26
L_11072: sbi333 $r0, [$r1 + #0]
L_11074: mov55 $r1, $r0
L_11076: ex9.it #398
L_11078: ex9.it #25
L_1107a: lbi333 $r1, [$r0 + #0]
L_1107c: bnez38 $r1, . + (-2)
L_1107e: lbi333 $r1, [$r0 + #0]
L_11080: bnec	$r1, #1, . + (-2)
L_11084: movi55 $r0, #0
L_11086: sbi.gp	$r0, [ + #-5639]
L_1108a: ex9.it #2
L_1108c: beqz38 $r0, . + (64)
L_1108e: ex9.it #37
L_11090: bnec	$r0, #9, . + (-2)
L_11094: ex9.it #183
L_11096: ex9.it #188
L_11098: ex9.it #69
L_1109a: ex9.it #12
L_1109c: movi55 $r1, #1
L_1109e: sbi333 $r1, [$r0 + #0]
L_110a0: lbi	$r7, [$r9 + #0x12e]
L_110a4: bnez38 $r7, . + (10)
L_110a6: ifcall9 . + (16)
L_110a8: sbi.gp	$r7, [ + #-5639]
L_110ac: j8 . + (32)
L_110ae: lbi	$r7, [$r6 + #0x51]
L_110b2: xlsb33 $r7, $r7
L_110b4: bnez38 $r7, . + (12)
L_110b6: jal	. + (-34030)
L_110ba: sbi	$r7, [$r9 + #0x9e]
L_110be: j8 . + (10)
L_110c0: movi55 $r0, #0
L_110c2: ex9.it #426
L_110c4: jal	. + (-34422)
L_110c8: movi55 $r0, #1
L_110ca: ex9.it #227
L_110cc: movi55 $r0, #0xa
L_110ce: ex9.it #173
L_110d0: movi55 $r0, #0
L_110d2: sbi	$r0, [$r6 + #0x51]
L_110d6: sbi	$r0, [$r6 + #0x52]
L_110da: sbi	$r0, [$r6 + #0x53]
L_110de: pop25 $r14, #0
L_110e0: ex9.it #496
L_110e2: movi	$r2, #0x20000
L_110e6: lwi	$r1, [$r1 + #0x50]
L_110ea: sethi	$r3, #0xc003f
L_110ee: srli45 $r1, #0x10
L_110f0: ex9.it #157
L_110f2: add_slli	$r1, $r2, $r1, #2
L_110f6: lwi	$r2, [$r1 + #0x28]
L_110fa: addi	$r3, $r3, #0xfff
L_110fe: andi	$r0, $r0, #0xfff
L_11102: and33 $r2, $r3
L_11104: or_slli	$r2, $r2, $r0, #0x12
L_11108: swi	$r2, [$r1 + #0x28]
L_1110c: ret5 $lp
L_1110e: push25 $r10, #16
L_11110: mov55 $r6, $r2
L_11112: mov55 $r9, $r0
L_11114: mov55 $r7, $r1
L_11116: movpi45 $r2, #0x10
L_11118: movi55 $r1, #0
L_1111a: mov55 $r0, $sp
L_1111c: ex9.it #75
L_1111e: add333 $r1, $r7, $r6
L_11120: lbi.gp	$r2, [ + #-4076]
L_11124: movi	$r3, #0x2c69c
L_11128: ex9.it #236
L_1112a: zeh33 $r1, $r1
L_1112c: sb	$r0, [$r3 + ($r2 << #0)]
L_11130: mov55 $r4, $r3
L_11132: movi	$r6, #0x2f26c
L_11136: beqz38 $r1, . + (8)
L_11138: sh	$r1, [$r6 + ($r2 << #1)]
L_1113c: j8 . + (8)
L_1113e: movi55 $r0, #1
L_11140: sh	$r0, [$r6 + ($r2 << #1)]
L_11144: movi55 $r1, #0
L_11146: addi333 $r0, $r2, #1
L_11148: movi	$r16, #0x196d4
L_1114c: movi	$r17, #0xbb8
L_11150: add333 $r5, $r0, $r1
L_11152: fexti33 $r5, #2
L_11154: lb	$r7, [$r4 + ($r5 << #0)]
L_11158: beqc	$r7, #0xff, . + (30)
L_1115c: lb	$r3, [$r16 + ($r1 << #0)]
L_11160: lh	$r5, [$r6 + ($r5 << #1)]
L_11164: mul	$r3, $r3, $r17
L_11168: divsr	$r3, $r3, $r3, $r5
L_1116c: lw	$r5, [$sp + ($r7 << #2)]
L_11170: add45 $r3, $r5
L_11172: sw	$r3, [$sp + ($r7 << #2)]
L_11176: addi45 $r1, #1
L_11178: bnec	$r1, #8, . + (-40)
L_1117c: movi55 $r1, #0
L_1117e: mov55 $r3, $r1
L_11180: mov55 $r4, $r1
L_11182: lw	$r5, [$sp + ($r1 << #2)]
L_11186: zeb33 $r6, $r1
L_11188: slt45 $r4, $r5
L_1118a: addi45 $r1, #1
L_1118c: cmovn	$r3, $r6, $r15
L_11190: cmovn	$r4, $r5, $r15
L_11194: bnec	$r1, #4, . + (-18)
L_11198: slti45 $r2, #7
L_1119a: bnezs8 . + (4)
L_1119c: movi55 $r0, #0
L_1119e: sbi.gp	$r0, [ + #-4076]
L_111a2: sbi	$r3, [$r9 + #0]
L_111a6: pop25 $r10, #16
L_111a8: push25 $r14, #0
L_111aa: addi10.sp #-456
L_111ac: addi	$r9, $sp, #0x28
L_111b0: swi37.sp $r0, [ + #8]
L_111b2: swi37.sp $r1, [ + #0xc]
L_111b4: movi55 $r2, #8
L_111b6: movi55 $r1, #0
L_111b8: mov55 $r0, $r9
L_111ba: addri36.sp $r7, #0x30
L_111bc: ex9.it #75
L_111be: mov55 $r0, $r7
L_111c0: movi55 $r1, #0
L_111c2: movi55 $r2, #8
L_111c4: ex9.it #75
L_111c6: movi55 $r0, #0
L_111c8: ex9.it #310
L_111ca: lwi37.sp $r0, [ + #0xc]
L_111cc: movi	$r6, #0xc8
L_111d0: divsr	$r6, $r6, $r6, $r0
L_111d4: andi	$r13, $r6, #0xff
L_111d8: slli	$r0, $r13, #1
L_111dc: zeb33 $r0, $r0
L_111de: swi37.sp $r0, [ + #0x10]
L_111e0: movi55 $r12, #0
L_111e2: slli333 $r0, $r6, #1
L_111e4: swi37.sp $r0, [ + #4]
L_111e6: lwi	$r8, [$sp + #8]
L_111ea: mov55 $r11, $r9
L_111ec: mov55 $r10, $r12
L_111ee: zeh33 $r6, $r6
L_111f0: lwi37.sp $r0, [ + #0xc]
L_111f2: beq	$r10, $r0, . + (190)
L_111f6: addri36.sp $r0, #0x38
L_111f8: addi	$r1, $sp, #0x100
L_111fc: lwi37.sp $r2, [ + #0x10]
L_111fe: jal	. + (-3230)
L_11202: movi	$r5, #0x28178
L_11206: movi	$r4, #0x28ad8
L_1120a: add	$r14, $r12, $r5
L_1120e: add45 $r4, $r12
L_11210: swi	$r11, [$sp + #0x14]
L_11214: addri36.sp $r1, #0x38
L_11216: addi	$r2, $sp, #0x100
L_1121a: mov55 $r3, $r6
L_1121c: mov55 $r0, $r14
L_1121e: swi37.sp $r4, [ + #0x1c]
L_11220: ex9.it #267
L_11222: swi37.sp $r0, [ + #0x18]
L_11224: addri36.sp $r1, #0x9c
L_11226: addi	$r0, $r14, #0x190
L_1122a: addi	$r2, $sp, #0x164
L_1122e: mov55 $r3, $r6
L_11230: ex9.it #267
L_11232: lwi37.sp $r5, [ + #0x18]
L_11234: lhi	$r1, [$r11 + #0]
L_11238: slt45 $r0, $r5
L_1123a: cmovz	$r5, $r0, $r15
L_1123e: zeh33 $r0, $r5
L_11240: slt45 $r1, $r0
L_11242: lwi37.sp $r4, [ + #0x1c]
L_11244: beqzs8 . + (6)
L_11246: shi	$r5, [$r11 + #0]
L_1124a: addi	$r14, $r14, #0x320
L_1124e: bne	$r14, $r4, . + (-58)
L_11252: addri36.sp $r1, #0x38
L_11254: addi	$r2, $sp, #0x100
L_11258: mov55 $r3, $r6
L_1125a: mov55 $r0, $r8
L_1125c: ex9.it #267
L_1125e: addi	$r14, $r8, #0x190
L_11262: swi37.sp $r0, [ + #0x18]
L_11264: addri36.sp $r1, #0x9c
L_11266: mov55 $r0, $r14
L_11268: addi	$r2, $sp, #0x164
L_1126c: mov55 $r3, $r6
L_1126e: ex9.it #267
L_11270: lwi37.sp $r4, [ + #0x18]
L_11272: lwi37.sp $r1, [ + #0x14]
L_11274: slt45 $r0, $r4
L_11276: cmovz	$r4, $r0, $r15
L_1127a: lhi333 $r1, [$r1 + #0]
L_1127c: zeh33 $r0, $r4
L_1127e: slt45 $r1, $r0
L_11280: addi45 $r11, #2
L_11282: beqzs8 . + (6)
L_11284: shi	$r4, [$r11 + #-2]
L_11288: mov55 $r1, $r13
L_1128a: mov55 $r0, $r8
L_1128c: ex9.it #349
L_1128e: sh	$r0, [$r7 + ($r10 << #1)]
L_11292: mov55 $r1, $r13
L_11294: mov55 $r0, $r14
L_11296: ex9.it #349
L_11298: lh	$r1, [$r7 + ($r10 << #1)]
L_1129c: slt45 $r1, $r0
L_1129e: beqzs8 . + (6)
L_112a0: sh	$r0, [$r7 + ($r10 << #1)]
L_112a4: lwi37.sp $r0, [ + #4]
L_112a6: addi45 $r10, #1
L_112a8: add45 $r8, $r0
L_112aa: add	$r12, $r12, $r0
L_112ae: j8 . + (-190)
L_112b0: lwi37.sp $r0, [ + #8]
L_112b2: jal	. + (-3626)
L_112b6: ex9.it #6
L_112b8: lwi.gp	$r1, [ + #-4060]
L_112bc: lhi	$r2, [$r1 + #0x1e]
L_112c0: beqc	$r0, #2, . + (10)
L_112c4: movi55 $r0, #3
L_112c6: mul33 $r2, $r0
L_112c8: zeh33 $r2, $r2
L_112ca: mov55 $r3, $r2
L_112cc: mov55 $r1, $r9
L_112ce: mov55 $r0, $r7
L_112d0: ifcall9 . + (762)
L_112d2: lh	$r1, [$r7 + ($r0 << #1)]
L_112d6: sbi	$r0, [$sp + #0x27]
L_112da: lh	$r2, [$r9 + ($r0 << #1)]
L_112de: addi	$r0, $sp, #0x27
L_112e2: jal	. + (-468)
L_112e6: movi	$r1, #0x38d41
L_112ea: lbi333 $r0, [$r1 + #0]
L_112ec: lbi	$r4, [$sp + #0x27]
L_112f0: lh	$r0, [$r7 + ($r0 << #1)]
L_112f4: ex9.it #310
L_112f6: lbi333 $r3, [$r1 + #0]
L_112f8: lh	$r5, [$r7 + ($r4 << #1)]
L_112fc: lh	$r2, [$r7 + ($r3 << #1)]
L_11300: lh	$r0, [$r9 + ($r3 << #1)]
L_11304: add45 $r0, $r2
L_11306: lh	$r2, [$r9 + ($r4 << #1)]
L_1130a: add45 $r2, $r5
L_1130c: slts45 $r2, $r0
L_1130e: beqzs8 . + (12)
L_11310: sub333 $r2, $r0, $r2
L_11312: slti	$r15, $r2, #0xc8
L_11316: beqzs8 . + (10)
L_11318: j8 . + (4)
L_1131a: movi55 $r2, #0
L_1131c: sbi	$r3, [$sp + #0x27]
L_11320: lbi333 $r0, [$r1 + #0]
L_11322: movi	$r3, #0x1f4
L_11326: slli333 $r1, $r2, #1
L_11328: divr	$r1, $r1, $r1, $r3
L_1132c: slti45 $r1, #3
L_1132e: beqzs8 . + (14)
L_11330: bnez38 $r1, . + (14)
L_11332: movi	$r1, #0xc7
L_11336: slt	$r1, $r1, $r2
L_1133a: j8 . + (4)
L_1133c: movi55 $r1, #3
L_1133e: lbi	$r2, [$sp + #0x27]
L_11342: lbi.gp	$r5, [ + #-4068]
L_11346: bnes38 $r2, . + (14)
L_11348: lbi.gp	$r3, [ + #-4075]
L_1134c: add45 $r3, $r1
L_1134e: sbi.gp	$r3, [ + #-4075]
L_11352: j8 . + (6)
L_11354: sbi.gp	$r1, [ + #-4075]
L_11358: lbi.gp	$r3, [ + #-4075]
L_1135c: slti45 $r3, #3
L_1135e: beqzs8 . + (6)
L_11360: bnec	$r1, #3, . + (12)
L_11364: movi55 $r0, #0
L_11366: sbi.gp	$r0, [ + #-4075]
L_1136a: mov55 $r0, $r2
L_1136c: sbi.gp	$r2, [ + #-4068]
L_11370: addi10.sp #0x1c8
L_11372: pop25 $r14, #0
L_11374: movi55 $r0, #1
L_11376: ret5 $lp
L_11378: movi55 $r0, #0
L_1137a: movi	$r1, #0x2c69c
L_1137e: movi55 $r2, #-1
L_11380: ex9.it #260
L_11382: addi45 $r0, #1
L_11384: bnec	$r0, #8, . + (-4)
L_11388: movi55 $r0, #0
L_1138a: sbi.gp	$r0, [ + #-4076]
L_1138e: ret5 $lp
L_11390: bnec	$r0, #1, . + (14)
L_11394: movi55 $r0, #1
L_11396: sbi333 $r0, [$r1 + #0]
L_11398: movi55 $r0, #0
L_1139a: shi333 $r0, [$r2 + #0]
L_1139c: ret5 $lp
L_1139e: ex9.it #407
L_113a0: lhi	$r0, [$r0 + #0x1e]
L_113a4: min	$r4, $r4, $r0
L_113a8: slt45 $r4, $r3
L_113aa: bnezs8 . + (-22)
L_113ac: lhi333 $r0, [$r2 + #0]
L_113ae: slti45 $r0, #0x1e
L_113b0: beqzs8 . + (8)
L_113b2: addi45 $r0, #1
L_113b4: shi333 $r0, [$r2 + #0]
L_113b6: ret5 $lp
L_113b8: movi55 $r0, #0
L_113ba: sbi333 $r0, [$r1 + #0]
L_113bc: ret5 $lp
L_113be: push25 $r10, #0
L_113c0: mov55 $r9, $r1
L_113c2: movi55 $r1, #0
L_113c4: shi.gp	$r1, [ + #-4804]
L_113c8: sbi	$r9, [$r2 + #0]
L_113cc: movi55 $r1, #4
L_113ce: mov55 $r6, $r2
L_113d0: mov55 $r10, $r3
L_113d2: jal	. + (-554)
L_113d6: mov55 $r7, $r0
L_113d8: movi	$r0, #0x2c9f8
L_113dc: lb	$r1, [$r0 + ($r9 << #0)]
L_113e0: mov55 $r9, $r0
L_113e2: bnec	$r1, #1, . + (30)
L_113e6: ex9.it #370
L_113e8: slti45 $r0, #2
L_113ea: beqzs8 . + (8)
L_113ec: addi45 $r0, #1
L_113ee: sbi.gp	$r0, [ + #-4791]
L_113f2: movi55 $r0, #1
L_113f4: sbi.gp	$r0, [ + #-4754]
L_113f8: movi55 $r0, #0
L_113fa: shi.gp	$r0, [ + #-5470]
L_113fe: j8 . + (92)
L_11400: movi	$r1, #0x2c74e
L_11404: movi55 $r0, #0
L_11406: movi	$r2, #0x2c482
L_1140a: lhi.gp	$r3, [ + #-4804]
L_1140e: lhi.gp	$r4, [ + #-5482]
L_11412: ifcall9 . + (556)
L_11414: lbi.gp	$r1, [ + #-4791]
L_11418: beqz38 $r1, . + (8)
L_1141a: subi45 $r1, #1
L_1141c: sbi.gp	$r1, [ + #-4791]
L_11420: bnec	$r10, #1, . + (36)
L_11424: movi55 $r3, #0
L_11426: mov55 $r1, $r3
L_11428: lb	$r0, [$r9 + ($r3 << #0)]
L_1142c: bnez38 $r0, . + (6)
L_1142e: addi45 $r1, #1
L_11430: zeh33 $r1, $r1
L_11432: addi45 $r3, #1
L_11434: bnec	$r3, #4, . + (-12)
L_11438: bnec	$r1, #4, . + (12)
L_1143c: ex9.it #346
L_1143e: addi45 $r0, #1
L_11440: ex9.it #199
L_11442: j8 . + (6)
L_11444: movi55 $r0, #0
L_11446: ex9.it #199
L_11448: lhi.gp	$r1, [ + #-4078]
L_1144c: movi	$r0, #0xea5f
L_11450: slt45 $r0, $r1
L_11452: bnezs8 . + (8)
L_11454: addi45 $r1, #1
L_11456: shi.gp	$r1, [ + #-4078]
L_1145a: movi55 $r0, #0
L_1145c: ex9.it #137
L_1145e: ex9.it #370
L_11460: beqz38 $r0, . + (8)
L_11462: movi55 $r0, #1
L_11464: ex9.it #137
L_11466: sbi333 $r7, [$r6 + #0]
L_11468: pop25 $r10, #0
L_1146a: push25 $r6, #0
L_1146c: xlsb33 $r6, $r0
L_1146e: add_slli	$r6, $r1, $r6, #1
L_11472: movi	$r1, #0x640
L_11476: mul33 $r0, $r1
L_11478: movi	$r1, #0x30000
L_1147c: add_srli	$r0, $r1, $r0, #1
L_11480: mov55 $r1, $r2
L_11482: mov55 $r2, $r3
L_11484: ex9.it #34
L_11486: jal	. + (-3790)
L_1148a: shi333 $r0, [$r6 + #0]
L_1148c: pop25 $r6, #0
L_1148e: push25 $r14, #0
L_11490: addi	$sp, $sp, #-1608
L_11494: swi450 $r3, [$sp]
L_11496: mov55 $r11, $r0
L_11498: ex9.it #34
L_1149a: addri36.sp $r0, #8
L_1149c: mov55 $r12, $r1
L_1149e: mov55 $r7, $r2
L_114a0: ex9.it #121
L_114a2: movi55 $r2, #0
L_114a4: swi37.sp $r4, [ + #4]
L_114a6: ex9.it #441
L_114a8: ex9.it #292
L_114aa: ex9.it #34
L_114ac: addi	$r0, $sp, #0x328
L_114b0: ex9.it #121
L_114b2: ex9.it #441
L_114b4: ex9.it #12
L_114b6: movi55 $r1, #1
L_114b8: sbi333 $r1, [$r0 + #0]
L_114ba: movi55 $r6, #0
L_114bc: movi55 $r0, #1
L_114be: swi.gp	$r6, [ + #-5080]
L_114c2: movi	$r9, #0x3f04a
L_114c6: ex9.it #113
L_114c8: movi	$r10, #0x55
L_114cc: ex9.it #212
L_114ce: movi	$r13, #0x3f423
L_114d2: beq	$r6, $r11, . + (142)
L_114d6: movi55 $r0, #0
L_114d8: sbi.gp	$r0, [ + #-5606]
L_114dc: movi55 $r0, #0
L_114de: ex9.it #375
L_114e0: movi55 $r0, #1
L_114e2: mov55 $r1, $r0
L_114e4: ex9.it #398
L_114e6: movi55 $r0, #0xc
L_114e8: sbi	$r10, [$r9 + #0]
L_114ec: ex9.it #33
L_114ee: lbi	$r0, [$r13 + #0]
L_114f2: beqc	$r0, #4, . + (10)
L_114f6: sbi	$r10, [$r9 + #0]
L_114fa: j8 . + (-12)
L_114fc: movi55 $r8, #0xc
L_114fe: movi55 $r14, #0
L_11500: lbi.gp	$r0, [ + #-5606]
L_11504: slt45 $r0, $r12
L_11506: beqzs8 . + (74)
L_11508: ex9.it #37
L_1150a: bnec	$r0, #9, . + (-2)
L_1150e: swi.gp	$r8, [ + #-5292]
L_11512: swi.gp	$r14, [ + #-5080]
L_11516: lbi.gp	$r0, [ + #-5606]
L_1151a: lwi450 $r1, [$sp]
L_1151c: bnec	$r7, #2, . + (18)
L_11520: lwi37.sp $r2, [ + #4]
L_11522: addri36.sp $r3, #8
L_11524: addi	$r4, $sp, #0x328
L_11528: jal	. + (-3724)
L_1152c: j8 . + (22)
L_1152e: addri36.sp $r2, #8
L_11530: addi	$r3, $sp, #0x328
L_11534: bnec	$r7, #1, . + (10)
L_11538: jal	. + (-3824)
L_1153c: j8 . + (6)
L_1153e: jal	. + (-212)
L_11542: lbi.gp	$r0, [ + #-5606]
L_11546: addi45 $r0, #1
L_11548: zeb33 $r0, $r0
L_1154a: sbi.gp	$r0, [ + #-5606]
L_1154e: j8 . + (-78)
L_11550: ex9.it #12
L_11552: movi55 $r1, #1
L_11554: sbi333 $r1, [$r0 + #0]
L_11556: addi45 $r6, #1
L_11558: ex9.it #61
L_1155a: ex9.it #149
L_1155c: zeb33 $r6, $r6
L_1155e: j8 . + (-140)
L_11560: ex9.it #164
L_11562: addi	$sp, $sp, #0x648
L_11566: pop25 $r14, #0
L_11568: push25 $r10, #24
L_1156a: movi55 $r1, #0
L_1156c: movi55 $r2, #8
L_1156e: addri36.sp $r0, #8
L_11570: ex9.it #75
L_11572: movi55 $r1, #0
L_11574: movi55 $r2, #8
L_11576: addri36.sp $r0, #0x10
L_11578: ex9.it #75
L_1157a: ex9.it #164
L_1157c: movi55 $r1, #2
L_1157e: ex9.it #11
L_11580: sbi333 $r1, [$r0 + #0]
L_11582: movi55 $r2, #2
L_11584: movi55 $r0, #1
L_11586: movi55 $r1, #5
L_11588: ifcall9 . + (24)
L_1158a: movi	$r6, #0x38d42
L_1158e: ex9.it #115
L_11590: ex9.it #120
L_11592: lbi333 $r0, [$r6 + #0]
L_11594: bnec	$r0, #0x8f, . + (20)
L_11598: ex9.it #82
L_1159a: movi55 $r0, #1
L_1159c: movi55 $r1, #5
L_1159e: movi55 $r2, #2
L_115a0: addri36.sp $r3, #8
L_115a2: addri36.sp $r4, #0x10
L_115a4: ifcall9 . + (82)
L_115a6: j8 . + (-20)
L_115a8: addri36.sp $r1, #8
L_115aa: movi55 $r2, #4
L_115ac: movi55 $r0, #2
L_115ae: ifcall9 . + (178)
L_115b0: ex9.it #6
L_115b2: lwi.gp	$r1, [ + #-4060]
L_115b6: lhi	$r2, [$r1 + #0x1e]
L_115ba: beqc	$r0, #2, . + (10)
L_115be: movi55 $r0, #3
L_115c0: mul33 $r2, $r0
L_115c2: zeh33 $r2, $r2
L_115c4: addri36.sp $r1, #8
L_115c6: mov55 $r3, $r2
L_115c8: addri36.sp $r0, #0x10
L_115ca: jal	. + (-2742)
L_115ce: swi37.sp $r0, [ + #4]
L_115d0: jal	. + (-600)
L_115d4: movi55 $r0, #2
L_115d6: ifcall9 . + (42)
L_115d8: ex9.it #164
L_115da: lwi37.sp $r0, [ + #4]
L_115dc: pop25 $r10, #24
L_115de: push25 $r6, #8
L_115e0: mov55 $r6, $r0
L_115e2: ex9.it #164
L_115e4: movi55 $r1, #1
L_115e6: ex9.it #11
L_115e8: sbi333 $r1, [$r0 + #0]
L_115ea: movi55 $r0, #1
L_115ec: movi55 $r1, #5
L_115ee: mov55 $r2, $r0
L_115f0: addri36.sp $r3, #4
L_115f2: addi	$r4, $sp, #6
L_115f6: jal	. + (-360)
L_115fa: bnec	$r6, #1, . + (10)
L_115fe: mov55 $r0, $r6
L_11600: jal	. + (-2896)
L_11604: ex9.it #164
L_11606: ex9.it #376
L_11608: jal	. + (-3602)
L_1160c: pop25 $r6, #8
L_1160e: push25 $r10, #8
L_11610: movi55 $r0, #0
L_11612: ex9.it #44
L_11614: jal	. + (-3168)
L_11618: ex9.it #11
L_1161a: lbi333 $r0, [$r0 + #0]
L_1161c: beqz38 $r0, . + (40)
L_1161e: beqc	$r0, #4, . + (8)
L_11622: j	. + (196)
L_11626: ex9.it #133
L_11628: shi.gp	$r0, [ + #-5482]
L_1162c: movi	$r1, #0x2c74e
L_11630: movi55 $r0, #0
L_11632: movi	$r2, #0x2c482
L_11636: lhi.gp	$r3, [ + #-4804]
L_1163a: movi	$r4, #0x50
L_1163e: jal	. + (-686)
L_11642: j8 . + (12)
L_11644: sbi.gp	$r0, [ + #-4754]
L_11648: ex9.it #133
L_1164a: shi.gp	$r0, [ + #-5482]
L_1164e: ex9.it #365
L_11650: movi55 $r1, #0
L_11652: jal	. + (-2522)
L_11656: ex9.it #11
L_11658: lbi333 $r0, [$r0 + #0]
L_1165a: movi	$r1, #0x2c71c
L_1165e: movi55 $r2, #1
L_11660: jal	. + (-32702)
L_11664: ex9.it #335
L_11666: lbi333 $r0, [$r0 + #0]
L_11668: seb33 $r1, $r0
L_1166a: bgez	$r1, . + (30)
L_1166e: bnec	$r0, #0x88, . + (18)
L_11672: movi55 $r0, #0
L_11674: ex9.it #137
L_11676: lhi.gp	$r0, [ + #-4078]
L_1167a: addi45 $r0, #1
L_1167c: ex9.it #225
L_1167e: j8 . + (10)
L_11680: movi55 $r0, #1
L_11682: ex9.it #137
L_11684: movi55 $r0, #0
L_11686: ex9.it #225
L_11688: ex9.it #365
L_1168a: ex9.it #121
L_1168c: ex9.it #29
L_1168e: jal	. + (-3826)
L_11692: ex9.it #343
L_11694: beqc	$r0, #1, . + (6)
L_11698: ifcall9 . + (196)
L_1169a: lbi.gp	$r0, [ + #-4802]
L_1169e: beqc	$r0, #1, . + (6)
L_116a2: ifcall9 . + (186)
L_116a4: movi55 $r0, #0
L_116a6: ex9.it #268
L_116a8: movi	$r0, #0x43
L_116ac: ex9.it #1
L_116ae: jal	. + (-326)
L_116b2: ex9.it #44
L_116b4: ex9.it #335
L_116b6: lbi333 $r0, [$r0 + #0]
L_116b8: seb33 $r1, $r0
L_116ba: bgez	$r1, . + (8)
L_116be: fexti33 $r0, #1
L_116c0: ex9.it #44
L_116c2: ex9.it #18
L_116c4: lbi	$r2, [$sp + #7]
L_116c8: ex9.it #67
L_116ca: movi55 $r0, #3
L_116cc: ex9.it #143
L_116ce: movi55 $r6, #1
L_116d0: movi55 $r0, #0
L_116d2: shi.gp	$r0, [ + #-5470]
L_116d6: ex9.it #174
L_116d8: sbi.gp	$r6, [ + #-4754]
L_116dc: ex9.it #278
L_116de: ex9.it #212
L_116e0: sbi.gp	$r6, [ + #-4753]
L_116e4: ifcall9 . + (120)
L_116e6: ex9.it #427
L_116e8: shi.gp	$r0, [ + #-5482]
L_116ec: ex9.it #313
L_116ee: bnec	$r0, #1, . + (18)
L_116f2: lwi.gp	$r3, [ + #-4768]
L_116f6: movi	$r0, #0x31
L_116fa: slt	$r3, $r0, $r3
L_116fe: j8 . + (14)
L_11700: movi55 $r3, #0
L_11702: bnez38 $r0, . + (10)
L_11704: lbi.gp	$r3, [ + #-5650]
L_11708: slt	$r3, $r0, $r3
L_1170c: movi	$r1, #0x38d41
L_11710: ex9.it #365
L_11712: lbi333 $r1, [$r1 + #0]
L_11714: ex9.it #181
L_11716: jal	. + (-856)
L_1171a: movi	$r1, #0x38d42
L_1171e: lbsi	$r0, [$r1 + #0]
L_11722: bgez	$r0, . + (52)
L_11726: movi55 $r0, #0
L_11728: ex9.it #137
L_1172a: lbi333 $r0, [$r1 + #0]
L_1172c: bmski33 $r0, #6
L_1172e: beqz38 $r0, . + (30)
L_11730: movi55 $r0, #0
L_11732: ex9.it #199
L_11734: lbi333 $r0, [$r1 + #0]
L_11736: andi	$r2, $r0, #0x20
L_1173a: beqz38 $r2, . + (28)
L_1173c: bitci	$r2, $r0, #0x20
L_11740: fexti33 $r0, #1
L_11742: sbi333 $r2, [$r1 + #0]
L_11744: ex9.it #44
L_11746: movi55 $r0, #1
L_11748: ex9.it #137
L_1174a: j8 . + (12)
L_1174c: ex9.it #346
L_1174e: slti45 $r0, #0x1e
L_11750: beqzs8 . + (6)
L_11752: movpi45 $r0, #0x1e
L_11754: ex9.it #199
L_11756: ex9.it #343
L_11758: beqc	$r0, #1, . + (8)
L_1175c: j	. + (224)
L_11760: lbi.gp	$r0, [ + #-4802]
L_11764: bnec	$r0, #1, . + (60)
L_11768: lbi.gp	$r7, [ + #-5661]
L_1176c: bnec	$r7, #1, . + (52)
L_11770: movi	$r9, #0x38d41
L_11774: lbi	$r5, [$r9 + #0]
L_11778: ex9.it #329
L_1177a: beqs38 $r0, . + (194)
L_1177c: movi55 $r6, #0
L_1177e: sbi.gp	$r6, [ + #-4753]
L_11782: ex9.it #164
L_11784: movi55 $r0, #3
L_11786: ex9.it #18
L_11788: lbi	$r2, [$sp + #7]
L_1178c: mov55 $r3, $r9
L_1178e: ex9.it #143
L_11790: sbi.gp	$r6, [ + #-5641]
L_11794: sbi.gp	$r6, [ + #-4813]
L_11798: sbi.gp	$r7, [ + #-4753]
L_1179c: ex9.it #278
L_1179e: j8 . + (158)
L_117a0: ex9.it #346
L_117a2: slti45 $r0, #0x1e
L_117a4: bnezs8 . + (152)
L_117a6: movi	$r0, #0x42
L_117aa: ex9.it #1
L_117ac: movi55 $r0, #0
L_117ae: ex9.it #199
L_117b0: ex9.it #164
L_117b2: ex9.it #2
L_117b4: beqz38 $r0, . + (16)
L_117b6: ex9.it #37
L_117b8: bnec	$r0, #9, . + (-2)
L_117bc: ex9.it #11
L_117be: lbi333 $r0, [$r0 + #0]
L_117c0: jal	. + (-35992)
L_117c4: movi55 $r0, #1
L_117c6: jal	. + (-488)
L_117ca: movi	$r2, #0x38d42
L_117ce: lbi333 $r1, [$r2 + #0]
L_117d0: mov55 $r3, $r2
L_117d2: seb33 $r4, $r1
L_117d4: bgez	$r4, . + (16)
L_117d8: bmski33 $r1, #4
L_117da: beqz38 $r1, . + (12)
L_117dc: movi	$r0, #-64
L_117e0: sbi333 $r0, [$r2 + #0]
L_117e2: j8 . + (66)
L_117e4: bnez38 $r0, . + (64)
L_117e6: movi55 $r7, #0
L_117e8: sbi333 $r7, [$r3 + #0]
L_117ea: lbi.gp	$r6, [ + #-4798]
L_117ee: ex9.it #67
L_117f0: bnez38 $r6, . + (28)
L_117f2: mov55 $r0, $r6
L_117f4: ex9.it #18
L_117f6: mov55 $r2, $r6
L_117f8: ex9.it #143
L_117fa: sbi.gp	$r6, [ + #-5641]
L_117fe: swi.gp	$r6, [ + #-4768]
L_11802: sbi.gp	$r7, [ + #-4813]
L_11806: ex9.it #278
L_11808: ex9.it #212
L_1180a: j8 . + (50)
L_1180c: movi55 $r0, #3
L_1180e: ex9.it #18
L_11810: lbi333 $r2, [$r3 + #0]
L_11812: ex9.it #143
L_11814: lwi.gp	$r0, [ + #-4768]
L_11818: ex9.it #433
L_1181a: bnezs8 . + (34)
L_1181c: movi55 $r0, #1
L_1181e: sbi.gp	$r0, [ + #-5625]
L_11822: j8 . + (26)
L_11824: ex9.it #67
L_11826: movi55 $r0, #3
L_11828: ex9.it #18
L_1182a: lbi333 $r2, [$r3 + #0]
L_1182c: ex9.it #143
L_1182e: movi55 $r0, #0
L_11830: ex9.it #174
L_11832: sbi.gp	$r0, [ + #-4813]
L_11836: movi55 $r0, #1
L_11838: ex9.it #268
L_1183a: ex9.it #278
L_1183c: ex9.it #407
L_1183e: lbi	$r0, [$r0 + #0x2e]
L_11842: beqz38 $r0, . + (22)
L_11844: lhi.gp	$r1, [ + #-4804]
L_11848: movi	$r0, #0x2fe54
L_1184c: shi333 $r1, [$r0 + #0]
L_1184e: lhi.gp	$r1, [ + #-5504]
L_11852: movi	$r0, #0x2fe56
L_11856: shi333 $r1, [$r0 + #0]
L_11858: pop25 $r10, #8
L_1185a: push25 $r6, #0
L_1185c: ex9.it #6
L_1185e: bnec	$r0, #2, . + (88)
L_11862: lwi.gp	$r1, [ + #-5072]
L_11866: ex9.it #150
L_11868: slt45 $r1, $r0
L_1186a: beqzs8 . + (6)
L_1186c: ex9.it #165
L_1186e: beqz38 $r0, . + (24)
L_11870: movi55 $r0, #0
L_11872: shi.gp	$r0, [ + #-5520]
L_11876: jal	. + (-8238)
L_1187a: bnez38 $r0, . + (60)
L_1187c: jal	. + (-5054)
L_11880: jal	. + (-626)
L_11884: j8 . + (50)
L_11886: ex9.it #111
L_11888: bnec	$r0, #1, . + (12)
L_1188c: movi	$r0, #0x2c450
L_11890: jal	. + (-36802)
L_11894: movi55 $r0, #1
L_11896: movi55 $r2, #0
L_11898: mov55 $r4, $r0
L_1189a: movi55 $r1, #0xa
L_1189c: mov55 $r3, $r2
L_1189e: mov55 $r5, $r2
L_118a0: ex9.it #23
L_118a2: ex9.it #27
L_118a4: bnec	$r0, #2, . + (10)
L_118a8: jal	. + (-27494)
L_118ac: j8 . + (-74)
L_118ae: lbi.gp	$r0, [ + #-5707]
L_118b2: bnec	$r0, #1, . + (-80)
L_118b6: pop25 $r6, #0
L_118b8: push25 $r8, #0
L_118ba: ex9.it #11
L_118bc: lbi333 $r2, [$r0 + #0]
L_118be: lbi.gp	$r1, [ + #-5684]
L_118c2: lbi.gp	$r0, [ + #-5647]
L_118c6: lbi.gp	$r3, [ + #-5701]
L_118ca: lbi.gp	$r5, [ + #0x13df]
L_118ce: bnez38 $r2, . + (10)
L_118d0: mov55 $r2, $r0
L_118d2: mov55 $r16, $r1
L_118d4: mov55 $r0, $r3
L_118d6: j8 . + (8)
L_118d8: mov55 $r16, $r5
L_118da: mov55 $r2, $r3
L_118dc: mov55 $r5, $r1
L_118de: mul	$r7, $r2, $r16
L_118e2: lwi.gp	$r17, [ + #0x76c]
L_118e6: mul	$r2, $r0, $r5
L_118ea: movi55 $r4, #0
L_118ec: movi	$r18, #0x33310
L_118f0: lhi.gp	$r0, [ + #-55724]
L_118f4: slt45 $r4, $r0
L_118f6: beqzs8 . + (42)
L_118f8: slli333 $r6, $r4, #1
L_118fa: add	$r0, $r6, $r18
L_118fe: lhi333 $r3, [$r0 + #0]
L_11900: lhi333 $r1, [$r0 + #0]
L_11902: add45 $r6, $r17
L_11904: lhi333 $r0, [$r6 + #0]
L_11906: mul33 $r1, $r5
L_11908: mul	$r3, $r3, $r16
L_1190c: sub333 $r0, $r1, $r0
L_1190e: mul33 $r0, $r7
L_11910: addi45 $r4, #1
L_11912: divsr	$r0, $r0, $r0, $r2
L_11916: sub333 $r0, $r3, $r0
L_11918: zeh33 $r0, $r0
L_1191a: shi333 $r0, [$r6 + #0]
L_1191c: zeh33 $r4, $r4
L_1191e: j8 . + (-46)
L_11920: lwi.gp	$r0, [ + #0x76c]
L_11924: lwi.gp	$r1, [ + #0x764]
L_11928: ex9.it #69
L_1192a: pop25 $r8, #0
L_1192c: movi	$r5, #0x31c20
L_11930: movi	$r2, #0x3e8
L_11934: movi	$r3, #0x31db0
L_11938: bnec	$r0, #5, . + (22)
L_1193c: lhi333 $r4, [$r5 + #0]
L_1193e: lhi	$r1, [$r5 + #0x2370]
L_11942: mul33 $r1, $r4
L_11944: divr	$r1, $r1, $r1, $r2
L_11948: zeh33 $r1, $r1
L_1194a: shi333 $r1, [$r5 + #0]
L_1194c: j8 . + (22)
L_1194e: bnec	$r0, #1, . + (20)
L_11952: lhi333 $r1, [$r5 + #0]
L_11954: lhi	$r4, [$r5 + #0x2370]
L_11958: mul33 $r1, $r2
L_1195a: divr	$r1, $r1, $r1, $r4
L_1195e: zeh33 $r1, $r1
L_11960: shi333 $r1, [$r5 + #0]
L_11962: addi45 $r5, #2
L_11964: bnes38 $r3, . + (-44)
L_11966: ret5 $lp
L_11968: push25 $r14, #0
L_1196a: mov55 $r7, $r0
L_1196c: ex9.it #11
L_1196e: mov55 $r6, $r1
L_11970: mov55 $r12, $r2
L_11972: mov55 $r11, $r3
L_11974: mov55 $r10, $r4
L_11976: lbi	$r9, [$r0 + #0]
L_1197a: bnec	$r7, #1, . + (40)
L_1197e: beqz38 $r1, . + (6)
L_11980: subi45 $r6, #1
L_11982: zeb33 $r6, $r6
L_11984: ex9.it #430
L_11986: lbi333 $r5, [$r0 + #0]
L_11988: beqs38 $r6, . + (14)
L_1198a: ex9.it #313
L_1198c: bnec	$r0, #1, . + (10)
L_11990: mov55 $r0, $r6
L_11992: jal	. + (-18680)
L_11996: movi55 $r0, #3
L_11998: ex9.it #18
L_1199a: mov55 $r2, $r6
L_1199c: ex9.it #67
L_1199e: ex9.it #143
L_119a0: j8 . + (26)
L_119a2: bnec	$r7, #2, . + (24)
L_119a6: bnec	$r9, #4, . + (10)
L_119aa: movi55 $r0, #1
L_119ac: jal	. + (-128)
L_119b0: movi55 $r0, #0
L_119b2: ex9.it #18
L_119b4: mov55 $r2, $r6
L_119b6: ex9.it #67
L_119b8: ex9.it #143
L_119ba: movi55 $r0, #1
L_119bc: jal	. + (-11998)
L_119c0: ex9.it #21
L_119c2: lbi333 $r0, [$r1 + #0]
L_119c4: zeb33 $r0, $r0
L_119c6: bnec	$r0, #1, . + (-4)
L_119ca: ex9.it #26
L_119cc: sbi333 $r0, [$r1 + #0]
L_119ce: mov55 $r1, $r0
L_119d0: ex9.it #398
L_119d2: ex9.it #25
L_119d4: lbi333 $r1, [$r0 + #0]
L_119d6: bnez38 $r1, . + (-2)
L_119d8: lbi333 $r1, [$r0 + #0]
L_119da: bnec	$r1, #1, . + (-2)
L_119de: ex9.it #2
L_119e0: beqz38 $r0, . + (44)
L_119e2: ex9.it #37
L_119e4: bnec	$r0, #9, . + (-2)
L_119e8: ex9.it #183
L_119ea: ex9.it #188
L_119ec: ex9.it #69
L_119ee: ex9.it #11
L_119f0: lbi333 $r0, [$r0 + #0]
L_119f2: bnez38 $r0, . + (8)
L_119f4: bnec	$r9, #4, . + (18)
L_119f8: j8 . + (10)
L_119fa: bnec	$r0, #4, . + (12)
L_119fe: bnez	$r9, . + (8)
L_11a02: jal	. + (-330)
L_11a06: ex9.it #12
L_11a08: movi55 $r1, #1
L_11a0a: sbi333 $r1, [$r0 + #0]
L_11a0c: bnec	$r7, #1, . + (18)
L_11a10: movi55 $r0, #0
L_11a12: sbi	$r7, [$r12 + #0]
L_11a16: sbi	$r7, [$r11 + #0]
L_11a1a: ex9.it #173
L_11a1c: j8 . + (18)
L_11a1e: bnec	$r7, #2, . + (16)
L_11a22: movi55 $r0, #0
L_11a24: sbi	$r0, [$r12 + #0]
L_11a28: ex9.it #227
L_11a2a: movi55 $r0, #1
L_11a2c: ex9.it #173
L_11a2e: pop25 $r14, #0
L_11a30: push25 $r8, #0
L_11a32: lbi.gp	$r6, [ + #-5626]
L_11a36: lhi.gp	$r7, [ + #-5546]
L_11a3a: lbi333 $r0, [$r0 + #1]
L_11a3c: jal	. + (-17958)
L_11a40: subi45 $r6, #1
L_11a42: slt45 $r7, $r0
L_11a44: slti	$r6, $r6, #1
L_11a48: bnezs8 . + (8)
L_11a4a: ex9.it #328
L_11a4c: beqz38 $r0, . + (4)
L_11a4e: movi55 $r6, #1
L_11a50: mov55 $r0, $r6
L_11a52: pop25 $r8, #0
L_11a54: push25 $r6, #0
L_11a56: ex9.it #0
L_11a58: lbi333 $r1, [$r0 + #1]
L_11a5a: beqc	$r1, #4, . + (10)
L_11a5e: movi55 $r1, #4
L_11a60: sbi333 $r1, [$r0 + #1]
L_11a62: ex9.it #64
L_11a64: movi55 $r0, #0xa
L_11a66: movi	$r1, #0xb5
L_11a6a: ex9.it #7
L_11a6c: ex9.it #328
L_11a6e: bnez38 $r0, . + (6)
L_11a70: ifcall9 . + (20)
L_11a72: beqzs8 . + (6)
L_11a74: jal	. + (-16658)
L_11a78: movi55 $r0, #0xa
L_11a7a: movi	$r1, #0xb6
L_11a7e: ex9.it #7
L_11a80: ex9.it #2
L_11a82: beqz38 $r0, . + (72)
L_11a84: ex9.it #24
L_11a86: subi45 $r0, #4
L_11a88: slti45 $r0, #2
L_11a8a: ifret16
L_11a8c: beqzs8 . + (46)
L_11a8e: lbi.gp	$r0, [ + #-6248]
L_11a92: bnec	$r0, #1, . + (20)
L_11a96: movi	$r1, #0x3f
L_11a9a: movi55 $r0, #8
L_11a9c: mov55 $r2, $r1
L_11a9e: mov55 $r3, $r1
L_11aa0: jal	. + (-13754)
L_11aa4: j8 . + (42)
L_11aa6: bnec	$r0, #2, . + (40)
L_11aaa: movi	$r0, #0x3f
L_11aae: movi55 $r1, #6
L_11ab0: mov55 $r2, $r0
L_11ab2: mov55 $r3, $r0
L_11ab4: jal	. + (-13774)
L_11ab8: j8 . + (22)
L_11aba: movi	$r2, #0x3f
L_11abe: movi55 $r0, #8
L_11ac0: movi55 $r1, #6
L_11ac2: mov55 $r3, $r2
L_11ac4: jal	. + (-13790)
L_11ac8: j8 . + (6)
L_11aca: jal	. + (-13760)
L_11ace: movi	$r1, #0xb7
L_11ad2: movi55 $r0, #0xa
L_11ad4: ex9.it #7
L_11ad6: movi55 $r0, #5
L_11ad8: ex9.it #10
L_11ada: ex9.it #91
L_11adc: jal	. + (-62686)
L_11ae0: movi55 $r0, #6
L_11ae2: ex9.it #10
L_11ae4: movi55 $r0, #0xa
L_11ae6: movi	$r1, #0xb8
L_11aea: ex9.it #7
L_11aec: ex9.it #12
L_11aee: movi55 $r1, #1
L_11af0: sbi333 $r1, [$r0 + #0]
L_11af2: movi55 $r1, #0
L_11af4: movi55 $r0, #2
L_11af6: mov55 $r2, $r1
L_11af8: movi55 $r3, #1
L_11afa: mov55 $r4, $r1
L_11afc: mov55 $r5, $r1
L_11afe: ex9.it #23
L_11b00: pop25 $r6, #0
L_11b02: push25 $r10, #8
L_11b04: mov55 $r9, $r0
L_11b06: ex9.it #61
L_11b08: ex9.it #149
L_11b0a: movi55 $r0, #6
L_11b0c: lwi.gp	$r7, [ + #-5440]
L_11b10: ex9.it #99
L_11b12: movi55 $r6, #1
L_11b14: ex9.it #0
L_11b16: ex9.it #493
L_11b18: ex9.it #64
L_11b1a: ex9.it #11
L_11b1c: lbi333 $r0, [$r0 + #0]
L_11b1e: movi55 $r1, #6
L_11b20: jal	. + (-7402)
L_11b24: movi55 $r0, #0
L_11b26: ex9.it #146
L_11b28: ex9.it #25
L_11b2a: zeb33 $r7, $r7
L_11b2c: swi.gp	$r7, [ + #-5440]
L_11b30: sbi.gp	$r6, [ + #-5594]
L_11b34: sbi333 $r6, [$r0 + #0]
L_11b36: ex9.it #12
L_11b38: sbi333 $r6, [$r0 + #0]
L_11b3a: sbi.gp	$r6, [ + #-5649]
L_11b3e: lbi.gp	$r6, [ + #-5689]
L_11b42: zeb33 $r6, $r6
L_11b44: bnec	$r6, #1, . + (32)
L_11b48: mov55 $r1, $r6
L_11b4a: ex9.it #0
L_11b4c: ex9.it #20
L_11b4e: mov55 $r0, $r9
L_11b50: jal	. + (-18298)
L_11b54: swi37.sp $r0, [ + #4]
L_11b56: movi55 $r2, #0
L_11b58: movi55 $r0, #2
L_11b5a: lwi37.sp $r1, [ + #4]
L_11b5c: mov55 $r3, $r6
L_11b5e: mov55 $r4, $r2
L_11b60: mov55 $r5, $r2
L_11b62: ex9.it #23
L_11b64: pop25 $r10, #8
L_11b66: push25 $r8, #0
L_11b68: mov55 $r6, $r0
L_11b6a: ex9.it #4
L_11b6c: ex9.it #3
L_11b6e: sbi333 $r1, [$r0 + #0]
L_11b70: movi	$r7, #0x2fef2
L_11b74: lbi.gp	$r0, [ + #-4899]
L_11b78: bnez38 $r0, . + (50)
L_11b7a: movi55 $r0, #4
L_11b7c: movi	$r1, #0xf3
L_11b80: ex9.it #7
L_11b82: ex9.it #286
L_11b84: movi55 $r0, #1
L_11b86: ex9.it #371
L_11b88: bnec	$r0, #1, . + (6)
L_11b8c: ifcall9 . + (68)
L_11b8e: lbi333 $r2, [$r7 + #0]
L_11b90: zeb33 $r2, $r2
L_11b92: beqz38 $r2, . + (10)
L_11b94: ex9.it #0
L_11b96: jal	. + (-55590)
L_11b9a: j8 . + (-38)
L_11b9c: movi55 $r0, #1
L_11b9e: mov55 $r1, $r0
L_11ba0: mov55 $r3, $r2
L_11ba2: mov55 $r4, $r0
L_11ba4: mov55 $r5, $r2
L_11ba6: ex9.it #23
L_11ba8: j8 . + (-52)
L_11baa: movi55 $r0, #1
L_11bac: ex9.it #88
L_11bae: movi55 $r0, #0xa
L_11bb0: ex9.it #10
L_11bb2: lbi.gp	$r0, [ + #-4899]
L_11bb6: ex9.it #10
L_11bb8: lbi.gp	$r2, [ + #-4899]
L_11bbc: bnec	$r2, #3, . + (24)
L_11bc0: movi55 $r0, #1
L_11bc2: ex9.it #59
L_11bc4: jal	. + (-18098)
L_11bc8: movi55 $r0, #0
L_11bca: ex9.it #88
L_11bcc: movi55 $r0, #0
L_11bce: ex9.it #145
L_11bd0: j	. + (218)
L_11bd4: bnec	$r2, #4, . + (18)
L_11bd8: ex9.it #0
L_11bda: movi55 $r1, #1
L_11bdc: ex9.it #20
L_11bde: ex9.it #254
L_11be0: movi55 $r0, #0
L_11be2: ex9.it #145
L_11be4: j8 . + (198)
L_11be6: bnec	$r2, #6, . + (26)
L_11bea: mov55 $r0, $r6
L_11bec: jal	. + (-234)
L_11bf0: movi55 $r0, #1
L_11bf2: sbi.gp	$r0, [ + #-4905]
L_11bf6: movi55 $r0, #0
L_11bf8: ex9.it #88
L_11bfa: movi55 $r0, #0
L_11bfc: ex9.it #145
L_11bfe: j8 . + (172)
L_11c00: bnec	$r2, #0xa, . + (38)
L_11c04: lbi333 $r0, [$r6 + #1]
L_11c06: movi55 $r6, #1
L_11c08: bnec	$r0, #0xa, . + (14)
L_11c0c: movi55 $r0, #0
L_11c0e: sbi.gp	$r6, [ + #-4756]
L_11c12: ex9.it #145
L_11c14: j8 . + (150)
L_11c16: ex9.it #209
L_11c18: ex9.it #12
L_11c1a: sbi.gp	$r6, [ + #-6328]
L_11c1e: sbi333 $r6, [$r0 + #0]
L_11c20: movi55 $r0, #0
L_11c22: ex9.it #145
L_11c24: j8 . + (134)
L_11c26: bnec	$r2, #9, . + (90)
L_11c2a: lbi333 $r0, [$r6 + #1]
L_11c2c: movi55 $r7, #0
L_11c2e: bnec	$r0, #0xa, . + (18)
L_11c32: movi55 $r0, #2
L_11c34: sbi.gp	$r7, [ + #-4756]
L_11c38: sbi.gp	$r7, [ + #-4899]
L_11c3c: sbi333 $r0, [$r6 + #0]
L_11c3e: j8 . + (108)
L_11c40: bnec	$r0, #0xb, . + (40)
L_11c44: sbi333 $r2, [$r6 + #1]
L_11c46: ex9.it #0
L_11c48: sbi.gp	$r7, [ + #-4899]
L_11c4c: ex9.it #64
L_11c4e: ex9.it #0
L_11c50: movi55 $r1, #1
L_11c52: ex9.it #20
L_11c54: movi55 $r0, #0
L_11c56: ex9.it #46
L_11c58: sbi.gp	$r7, [ + #-4756]
L_11c5c: jal	. + (-34270)
L_11c60: movi55 $r0, #1
L_11c62: sbi.gp	$r0, [ + #-4839]
L_11c66: j8 . + (68)
L_11c68: movi55 $r0, #5
L_11c6a: ex9.it #10
L_11c6c: ex9.it #91
L_11c6e: jal	. + (-63088)
L_11c72: movi55 $r0, #6
L_11c74: ex9.it #10
L_11c76: sbi.gp	$r7, [ + #-4899]
L_11c7a: movi55 $r0, #2
L_11c7c: movi55 $r1, #0
L_11c7e: j8 . + (34)
L_11c80: bnec	$r2, #0xb, . + (42)
L_11c84: movi55 $r1, #0
L_11c86: swi.gp	$r1, [ + #-4772]
L_11c8a: sbi333 $r2, [$r6 + #1]
L_11c8c: lwi.gp	$r2, [ + #-5356]
L_11c90: movi55 $r0, #1
L_11c92: ex9.it #195
L_11c94: shi.gp	$r1, [ + #-5546]
L_11c98: sbi.gp	$r1, [ + #-4899]
L_11c9c: sbi333 $r0, [$r2 + #0]
L_11c9e: movi55 $r0, #2
L_11ca0: mov55 $r2, $r1
L_11ca2: movi55 $r3, #1
L_11ca4: mov55 $r4, $r1
L_11ca6: mov55 $r5, $r1
L_11ca8: ex9.it #23
L_11caa: pop25 $r8, #0
L_11cac: push25 $r14, #8
L_11cae: mov55 $r6, $r0
L_11cb0: lbi333 $r0, [$r0 + #1]
L_11cb2: swi450 $r2, [$sp]
L_11cb4: swi37.sp $r3, [ + #4]
L_11cb6: bnec	$r0, #4, . + (16)
L_11cba: lbi333 $r0, [$r6 + #0]
L_11cbc: bnec	$r0, #1, . + (10)
L_11cc0: movi55 $r0, #0
L_11cc2: jal	. + (-18712)
L_11cc6: lbi.gp	$r0, [ + #-5599]
L_11cca: bnec	$r0, #1, . + (24)
L_11cce: movi	$r0, #0x2c401
L_11cd2: lwi450 $r1, [$sp]
L_11cd4: lwi37.sp $r2, [ + #4]
L_11cd6: lwi.gp	$r3, [ + #-5380]
L_11cda: lwi.gp	$r4, [ + #-5388]
L_11cde: jal	. + (-53760)
L_11ce2: ex9.it #61
L_11ce4: ex9.it #269
L_11ce6: ex9.it #2
L_11ce8: bnez38 $r0, . + (10)
L_11cea: ex9.it #36
L_11cec: ex9.it #58
L_11cee: ex9.it #54
L_11cf0: j8 . + (204)
L_11cf2: movi55 $r9, #0
L_11cf4: movi	$r10, #0x2fe50
L_11cf8: movi	$r12, #0x2c56c
L_11cfc: movi	$r13, #-196176
L_11d00: movi	$r11, #0x2fe56
L_11d04: ex9.it #30
L_11d06: beqc	$r0, #1, . + (66)
L_11d0a: lbi333 $r0, [$r6 + #1]
L_11d0c: bnec	$r0, #4, . + (60)
L_11d10: ex9.it #141
L_11d12: movi	$r7, #0x2fe50
L_11d16: ex9.it #289
L_11d18: beqc	$r0, #0x10, . + (48)
L_11d1c: movi55 $r0, #1
L_11d1e: ex9.it #256
L_11d20: lbi333 $r0, [$r7 + #0]
L_11d22: movi55 $r10, #0
L_11d24: ex9.it #123
L_11d26: mov55 $r1, $r7
L_11d28: mov55 $r2, $r7
L_11d2a: movi55 $r3, #6
L_11d2c: sbi	$r10, [$r7 + #0]
L_11d30: movi55 $r0, #2
L_11d32: ex9.it #5
L_11d34: movi55 $r0, #2
L_11d36: movi	$r1, #0x2c704
L_11d3a: mov55 $r2, $r1
L_11d3c: mov55 $r3, $r0
L_11d3e: ex9.it #5
L_11d40: ex9.it #82
L_11d42: sbi.gp	$r10, [ + #-5236]
L_11d46: j8 . + (88)
L_11d48: lmw.bi $r16, [$r10], $r16, #0x0
L_11d4c: smw.bi $r16, [$r12], $r16, #0x0
L_11d50: movi	$r0, #0x2fe54
L_11d54: lbi333 $r1, [$r0 + #0]
L_11d56: sbi.gp	$r1, [ + #-5232]
L_11d5a: lbi333 $r0, [$r0 + #1]
L_11d5c: sbi.gp	$r0, [ + #-5231]
L_11d60: ex9.it #175
L_11d62: movi55 $r0, #2
L_11d64: movi	$r2, #0x2c56c
L_11d68: movi55 $r3, #6
L_11d6a: ex9.it #5
L_11d6c: movi	$r5, #0x2fe50
L_11d70: movi	$r1, #0x2c56c
L_11d74: mov55 $r0, $r5
L_11d76: lb.bi	$r2, [$r0], $r13 << #0
L_11d7a: lb	$r0, [$r1 + ($r0 << #0)]
L_11d7e: beq	$r0, $r2, . + (26)
L_11d82: addi	$r7, $r9, #1
L_11d86: slti45 $r9, #5
L_11d88: zeb33 $r7, $r7
L_11d8a: beqzs8 . + (12)
L_11d8c: movi	$r0, #0x89
L_11d90: ex9.it #1
L_11d92: mov55 $r9, $r7
L_11d94: j8 . + (-144)
L_11d96: mov55 $r9, $r7
L_11d98: addi45 $r5, #1
L_11d9a: bne	$r5, $r11, . + (-38)
L_11d9e: ex9.it #36
L_11da0: movpi45 $r0, #0x22
L_11da2: sbi333 $r0, [$r1 + #0]
L_11da4: mov55 $r2, $r1
L_11da6: movi55 $r0, #2
L_11da8: movi55 $r3, #1
L_11daa: ex9.it #5
L_11dac: lbi.gp	$r0, [ + #-5236]
L_11db0: bnec	$r0, #0x13, . + (30)
L_11db4: movi	$r0, #0x85
L_11db8: ex9.it #1
L_11dba: j8 . + (20)
L_11dbc: lbi333 $r0, [$r1 + #0]
L_11dbe: beqc	$r0, #0x22, . + (8)
L_11dc2: sbi333 $r3, [$r2 + #0]
L_11dc4: j8 . + (-8)
L_11dc6: ex9.it #132
L_11dc8: lbi333 $r0, [$r0 + #0]
L_11dca: beqc	$r0, #0x13, . + (-22)
L_11dce: ex9.it #18
L_11dd0: movi	$r2, #0x2c3b4
L_11dd4: lwi450 $r3, [$sp]
L_11dd6: lwi37.sp $r4, [ + #4]
L_11dd8: ex9.it #499
L_11dda: jal	. + (-53106)
L_11dde: zeb33 $r0, $r0
L_11de0: ex9.it #235
L_11de2: jal	. + (-56068)
L_11de6: ex9.it #24
L_11de8: beqc	$r0, #1, . + (18)
L_11dec: movi55 $r7, #1
L_11dee: swi.gp	$r7, [ + #-5216]
L_11df2: ex9.it #2
L_11df4: bnez38 $r0, . + (6)
L_11df6: swi.gp	$r7, [ + #-5128]
L_11dfa: lbi333 $r0, [$r6 + #1]
L_11dfc: bnec	$r0, #8, . + (36)
L_11e00: lbi333 $r0, [$r6 + #0]
L_11e02: bnec	$r0, #2, . + (158)
L_11e06: ex9.it #63
L_11e08: beqc	$r0, #6, . + (124)
L_11e0c: beqc	$r0, #9, . + (12)
L_11e10: ex9.it #63
L_11e12: bnec	$r0, #0x11, . + (124)
L_11e16: j8 . + (40)
L_11e18: ex9.it #6
L_11e1a: bnec	$r0, #2, . + (-10)
L_11e1e: j8 . + (102)
L_11e20: andi	$r1, $r0, #0xfd
L_11e24: bnec	$r1, #9, . + (32)
L_11e28: lbi333 $r1, [$r6 + #0]
L_11e2a: bnec	$r1, #2, . + (26)
L_11e2e: movi55 $r0, #1
L_11e30: ex9.it #113
L_11e32: ex9.it #63
L_11e34: subi333 $r1, $r0, #6
L_11e36: slti45 $r1, #2
L_11e38: bnezs8 . + (76)
L_11e3a: bnec	$r0, #0x11, . + (84)
L_11e3e: ex9.it #2
L_11e40: beqz38 $r0, . + (68)
L_11e42: j8 . + (76)
L_11e44: bnec	$r0, #0xa, . + (14)
L_11e48: lbi333 $r0, [$r6 + #0]
L_11e4a: subi45 $r0, #2
L_11e4c: slti45 $r0, #2
L_11e4e: beqzs8 . + (82)
L_11e50: j8 . + (52)
L_11e52: bnec	$r0, #3, . + (24)
L_11e56: lbi333 $r6, [$r6 + #0]
L_11e58: bnec	$r6, #1, . + (72)
L_11e5c: mov55 $r0, $r6
L_11e5e: jal	. + (-13184)
L_11e62: mov55 $r0, $r6
L_11e64: ex9.it #113
L_11e66: movi55 $r0, #8
L_11e68: j8 . + (58)
L_11e6a: bnec	$r0, #4, . + (30)
L_11e6e: lbi333 $r6, [$r6 + #0]
L_11e70: bnec	$r6, #1, . + (48)
L_11e74: mov55 $r0, $r6
L_11e76: jal	. + (-13208)
L_11e7a: mov55 $r0, $r6
L_11e7c: ex9.it #113
L_11e7e: ex9.it #385
L_11e80: sbi.gp	$r0, [ + #-5702]
L_11e84: movi55 $r0, #8
L_11e86: j8 . + (28)
L_11e88: beqc	$r0, #6, . + (-50)
L_11e8c: j8 . + (20)
L_11e8e: movi55 $r2, #0
L_11e90: movi55 $r3, #1
L_11e92: movi55 $r0, #2
L_11e94: movpi45 $r1, #0x15
L_11e96: mov55 $r4, $r3
L_11e98: mov55 $r5, $r2
L_11e9a: ex9.it #23
L_11e9c: movi55 $r0, #8
L_11e9e: j8 . + (4)
L_11ea0: movi55 $r0, #1
L_11ea2: pop25 $r14, #8
L_11ea4: push25 $r6, #0
L_11ea6: movi55 $r0, #0
L_11ea8: swi.gp	$r0, [ + #-5092]
L_11eac: ex9.it #4
L_11eae: ex9.it #3
L_11eb0: sbi333 $r1, [$r0 + #0]
L_11eb2: lwi.gp	$r2, [ + #-5092]
L_11eb6: bnez38 $r2, . + (16)
L_11eb8: movi55 $r0, #1
L_11eba: mov55 $r1, $r0
L_11ebc: mov55 $r3, $r2
L_11ebe: mov55 $r4, $r0
L_11ec0: mov55 $r5, $r2
L_11ec2: ex9.it #23
L_11ec4: j8 . + (-18)
L_11ec6: pop25 $r6, #0
L_11ec8: push25 $r6, #0
L_11eca: ex9.it #21
L_11ecc: movi55 $r0, #0
L_11ece: sbi333 $r0, [$r1 + #0]
L_11ed0: ex9.it #26
L_11ed2: sbi333 $r0, [$r1 + #0]
L_11ed4: movi55 $r0, #1
L_11ed6: ex9.it #40
L_11ed8: movi55 $r0, #0xb
L_11eda: ex9.it #49
L_11edc: lbi.gp	$r0, [ + #-4909]
L_11ee0: ex9.it #92
L_11ee2: ex9.it #4
L_11ee4: ex9.it #3
L_11ee6: sbi333 $r1, [$r0 + #0]
L_11ee8: ex9.it #134
L_11eea: lbi333 $r0, [$r1 + #0]
L_11eec: beqc	$r0, #1, . + (-2)
L_11ef0: ex9.it #2
L_11ef2: beqz38 $r0, . + (20)
L_11ef4: ex9.it #36
L_11ef6: lbi333 $r0, [$r1 + #0]
L_11ef8: bnec	$r0, #0x32, . + (-2)
L_11efc: movi55 $r0, #9
L_11efe: ex9.it #33
L_11f00: movi55 $r0, #1
L_11f02: ex9.it #10
L_11f04: j8 . + (24)
L_11f06: jal	. + (-58012)
L_11f0a: ex9.it #36
L_11f0c: ex9.it #180
L_11f0e: sbi333 $r0, [$r1 + #0]
L_11f10: mov55 $r2, $r1
L_11f12: movi55 $r0, #2
L_11f14: movi55 $r3, #1
L_11f16: ex9.it #5
L_11f18: movi55 $r0, #2
L_11f1a: ex9.it #10
L_11f1c: pop25 $r6, #0
L_11f1e: nop16
L_11f20: push25 $r14, #8
L_11f22: mov55 $r6, $r0
L_11f24: mov55 $r11, $r1
L_11f26: movi55 $r0, #6
L_11f28: lbi333 $r1, [$r6 + #1]
L_11f2a: mov55 $r7, $r2
L_11f2c: movi55 $r9, #0
L_11f2e: ex9.it #7
L_11f30: movi55 $r10, #9
L_11f32: movi	$r12, #0x3f04a
L_11f36: ex9.it #401
# Selector 3..11: range-check, load unsigned16 relative target, jump via r15.
L_11f38: lbi333 $r0, [$r6 + #1]
L_11f3a: sbi.gp	$r9, [ + #-4839]
L_11f3e: subi45 $r0, #3
L_11f40: slti45 $r0, #9
L_11f42: beqz	$r15, . + (1534)
L_11f46: movi	$r15, #0x11d50
L_11f4a: ex9.it #423
L_11f4c: ex9.it #52
L_11f4e: jr5 $r15
# Loads byte [r6+1], subtracts 3, tests unsigned index <9, loads u16 at r15 + index*2 through EX9 slot423, adds r15 through slot52, jumps to r15.
L_11f50:
    .short L_11f62 - L_11f50
    .short L_120a2 - L_11f50
    .short L_12540 - L_11f50
    .short L_121a0 - L_11f50
    .short L_12540 - L_11f50
    .short L_122bc - L_11f50
    .short L_1231a - L_11f50
    .short L_123f4 - L_11f50
    .short L_1245c - L_11f50
L_11f62: ex9.it #2
L_11f64: lbi.gp	$r0, [ + #-55622]
L_11f68: bnez38 $r0, . + (18)
L_11f6a: ex9.it #4
L_11f6c: ex9.it #3
L_11f6e: sbi333 $r1, [$r0 + #0]
L_11f70: movi55 $r0, #9
L_11f72: movi	$r1, #0x91
L_11f76: ex9.it #7
L_11f78: j8 . + (82)
L_11f7a: bnec	$r0, #1, . + (126)
L_11f7e: lwi	$r9, [$r7 + #0xc]
L_11f82: ifcall9 . + (144)
L_11f84: slt45 $r9, $r0
L_11f86: bnezs8 . + (-28)
L_11f88: j8 . + (112)
L_11f8a: ex9.it #25
L_11f8c: lbi333 $r0, [$r0 + #0]
L_11f8e: bnez38 $r0, . + (72)
L_11f90: ex9.it #12
L_11f92: lbi333 $r0, [$r0 + #0]
L_11f94: bnec	$r0, #1, . + (66)
L_11f98: ex9.it #27
L_11f9a: bnec	$r0, #2, . + (54)
L_11f9e: ex9.it #111
L_11fa0: bnec	$r0, #1, . + (12)
L_11fa4: movi	$r0, #0x2c450
L_11fa8: jal	. + (-38618)
L_11fac: lbi.gp	$r0, [ + #-55729]
L_11fb0: lwi.gp	$r1, [ + #-5072]
L_11fb4: addi45 $r0, #1
L_11fb6: slli333 $r0, $r0, #1
L_11fb8: slt45 $r1, $r0
L_11fba: beqzs8 . + (16)
L_11fbc: movi55 $r2, #0
L_11fbe: movi55 $r0, #1
L_11fc0: movi55 $r1, #0xa
L_11fc2: mov55 $r3, $r2
L_11fc4: mov55 $r4, $r0
L_11fc6: mov55 $r5, $r2
L_11fc8: ex9.it #23
L_11fca: ex9.it #2
L_11fcc: beqz38 $r0, . + (10)
L_11fce: j8 . + (-68)
L_11fd0: ex9.it #27
L_11fd2: beqc	$r0, #1, . + (-52)
L_11fd6: ex9.it #2
L_11fd8: bnez38 $r0, . + (24)
L_11fda: ex9.it #25
L_11fdc: lbi333 $r0, [$r0 + #0]
L_11fde: bnez38 $r0, . + (18)
L_11fe0: lbi.gp	$r0, [ + #-5705]
L_11fe4: beqc	$r0, #1, . + (12)
L_11fe8: lbi.gp	$r0, [ + #-5649]
L_11fec: beqc	$r0, #1, . + (-78)
L_11ff0: movi55 $r0, #9
L_11ff2: movi	$r1, #0x92
L_11ff6: ex9.it #7
L_11ff8: lbi.gp	$r0, [ + #-55622]
L_11ffc: beqc	$r0, #1, . + (6)
L_12000: ex9.it #32
L_12002: ex9.it #2
L_12004: beqz38 $r0, . + (54)
L_12006: ex9.it #335
L_12008: lbi333 $r0, [$r0 + #0]
L_1200a: beqz38 $r0, . + (6)
L_1200c: movi55 $r0, #0
L_1200e: ex9.it #46
L_12010: lwi333 $r7, [$r7 + #0xc]
L_12012: jal	. + (-19434)
L_12016: slt45 $r0, $r7
L_12018: bnezs8 . + (8)
L_1201a: ex9.it #148
L_1201c: beqz38 $r0, . + (12)
L_1201e: j8 . + (18)
L_12020: jal	. + (14154)
L_12024: bnez38 $r0, . + (-10)
L_12026: j8 . + (32)
L_12028: ex9.it #24
L_1202a: subi45 $r0, #4
L_1202c: slti45 $r0, #2
L_1202e: bnezs8 . + (24)
L_12030: lbi.gp	$r0, [ + #-5626]
L_12034: beqc	$r0, #1, . + (18)
L_12038: ex9.it #32
L_1203a: lbi.gp	$r1, [ + #-4899]
L_1203e: beqc	$r1, #4, . + (6)
L_12042: ex9.it #32
L_12044: ex9.it #145
L_12046: ex9.it #4
L_12048: ex9.it #3
L_1204a: sbi333 $r1, [$r0 + #0]
L_1204c: ex9.it #21
L_1204e: lbi333 $r0, [$r1 + #0]
L_12050: beqz38 $r0, . + (-2)
L_12052: ex9.it #2
L_12054: bnez38 $r0, . + (16)
L_12056: ex9.it #11
L_12058: lbi333 $r0, [$r0 + #0]
L_1205a: bnec	$r0, #3, . + (18)
L_1205e: jal	. + (-4638)
L_12062: j8 . + (10)
L_12064: jal	. + (-61706)
L_12068: beqz38 $r0, . + (-4)
L_1206a: j8 . + (-20)
L_1206c: movi55 $r0, #4
L_1206e: sbi333 $r0, [$r6 + #1]
L_12070: movi55 $r0, #0
L_12072: sbi.gp	$r0, [ + #-4898]
L_12076: ex9.it #0
L_12078: ex9.it #64
L_1207a: jal	. + (-18688)
L_1207e: ex9.it #2
L_12080: beqz38 $r0, . + (16)
L_12082: movi55 $r0, #6
L_12084: movi	$r1, #0xd5
L_12088: ex9.it #7
L_1208a: movi55 $r0, #4
L_1208c: ex9.it #62
L_1208e: j8 . + (6)
L_12090: movi55 $r0, #1
L_12092: ex9.it #88
L_12094: movi55 $r0, #0xe
L_12096: ex9.it #10
L_12098: movi55 $r0, #2
L_1209a: sbi.gp	$r0, [ + #-5673]
L_1209e: ex9.it #254
L_120a0: ex9.it #32
L_120a2: movi55 $r0, #0xb
L_120a4: movi	$r1, #0xba
L_120a8: ex9.it #7
L_120aa: ex9.it #2
L_120ac: bnez38 $r0, . + (36)
L_120ae: movi55 $r0, #0xb
L_120b0: movi	$r1, #0xbb
L_120b4: ex9.it #7
L_120b6: mov55 $r0, $r6
L_120b8: ex9.it #308
L_120ba: ifcall9 . + (696)
L_120bc: movi55 $r0, #5
L_120be: ex9.it #10
L_120c0: movi55 $r0, #4
L_120c2: movi	$r1, #0xf1
L_120c6: ex9.it #7
L_120c8: ex9.it #286
L_120ca: mov55 $r0, $r6
L_120cc: ex9.it #489
L_120ce: ex9.it #32
L_120d0: movi55 $r0, #0xb
L_120d2: movi	$r1, #0xbc
L_120d6: ex9.it #7
L_120d8: ex9.it #30
L_120da: bnez38 $r0, . + (112)
L_120dc: movi55 $r0, #1
L_120de: ex9.it #10
L_120e0: ex9.it #24
L_120e2: subi333 $r1, $r0, #4
L_120e4: slti45 $r1, #4
L_120e6: bnezs8 . + (28)
L_120e8: beqc	$r0, #0xc, . + (26)
L_120ec: ex9.it #30
L_120ee: beqc	$r0, #4, . + (20)
L_120f2: lhi.gp	$r0, [ + #-4828]
L_120f6: bnec	$r0, #1, . + (8)
L_120fa: mov55 $r0, $r6
L_120fc: ex9.it #425
L_120fe: movi55 $r0, #0
L_12100: ex9.it #256
L_12102: ex9.it #385
L_12104: lbi.gp	$r5, [ + #-5702]
L_12108: bnes38 $r0, . + (10)
L_1210a: ex9.it #108
L_1210c: ex9.it #484
L_1210e: ex9.it #380
L_12110: ifcall9 . + (562)
L_12112: ex9.it #61
L_12114: ex9.it #149
L_12116: ex9.it #24
L_12118: subi45 $r0, #4
L_1211a: slti45 $r0, #2
L_1211c: beqzs8 . + (18)
L_1211e: ex9.it #119
L_12120: ex9.it #70
L_12122: ex9.it #96
L_12124: movi55 $r0, #0
L_12126: ex9.it #71
L_12128: movi55 $r0, #0
L_1212a: ex9.it #146
L_1212c: ex9.it #246
L_1212e: ex9.it #30
L_12130: beqc	$r0, #4, . + (38)
L_12134: movi	$r1, #0xd2
L_12138: movi55 $r0, #6
L_1213a: ex9.it #7
L_1213c: movi55 $r0, #4
L_1213e: ex9.it #62
L_12140: ex9.it #0
L_12142: movi55 $r1, #1
L_12144: ex9.it #20
L_12146: ex9.it #254
L_12148: j8 . + (14)
L_1214a: bnec	$r0, #4, . + (12)
L_1214e: movi55 $r0, #1
L_12150: ex9.it #72
L_12152: movi55 $r0, #0
L_12154: ex9.it #256
L_12156: movi55 $r0, #0
L_12158: ex9.it #195
L_1215a: ex9.it #308
L_1215c: mov55 $r0, $r6
L_1215e: ifcall9 . + (532)
L_12160: movi55 $r0, #1
L_12162: ex9.it #59
L_12164: ex9.it #328
L_12166: bnez38 $r0, . + (22)
L_12168: jal	. + (-34006)
L_1216c: ex9.it #177
L_1216e: bnec	$r0, #3, . + (10)
L_12172: movi55 $r0, #2
L_12174: ex9.it #118
L_12176: j8 . + (6)
L_12178: movi55 $r0, #1
L_1217a: ex9.it #118
L_1217c: ex9.it #211
L_1217e: bnec	$r0, #1, . + (6)
L_12182: ifcall9 . + (8)
L_12184: ex9.it #30
L_12186: bnec	$r0, #4, . + (8)
L_1218a: j	. + (938)
L_1218e: ex9.it #27
L_12190: beqc	$r0, #2, . + (6)
L_12194: ifcall9 . + (8)
L_12196: ex9.it #328
L_12198: bnez	$r0, . + (924)
L_1219c: j	. + (852)
L_121a0: lbi.gp	$r13, [ + #-5594]
L_121a4: bnec	$r13, #1, . + (38)
L_121a8: jal	. + (-19442)
L_121ac: movi55 $r0, #2
L_121ae: ex9.it #195
L_121b0: ex9.it #12
L_121b2: ex9.it #405
L_121b4: movi55 $r0, #5
L_121b6: sbi.gp	$r9, [ + #-5586]
L_121ba: ex9.it #10
L_121bc: ex9.it #2
L_121be: beqz	$r0, . + (-646)
L_121c2: ex9.it #37
L_121c4: bnec	$r0, #9, . + (-2)
L_121c8: ex9.it #32
L_121ca: beqc	$r13, #2, . + (6)
L_121ce: ex9.it #32
L_121d0: movi55 $r0, #0
L_121d2: ex9.it #118
L_121d4: ex9.it #2
L_121d6: beqz38 $r0, . + (80)
L_121d8: ex9.it #150
L_121da: addi45 $r0, #2
L_121dc: lwi.gp	$r1, [ + #-5072]
L_121e0: addi45 $r1, #2
L_121e2: slt45 $r1, $r0
L_121e4: beqzs8 . + (10)
L_121e6: lwi.gp	$r1, [ + #-5192]
L_121ea: bnec	$r1, #1, . + (-14)
L_121ee: ex9.it #108
L_121f0: ex9.it #484
L_121f2: ex9.it #380
L_121f4: ex9.it #0
L_121f6: movi	$r1, #0x2e534
L_121fa: jal	. + (-18320)
L_121fe: ex9.it #211
L_12200: bnez38 $r0, . + (10)
L_12202: ex9.it #0
L_12204: lbi333 $r0, [$r0 + #1]
L_12206: ex9.it #251
L_12208: ifcall9 . + (752)
L_1220a: movi55 $r0, #6
L_1220c: ex9.it #10
L_1220e: ex9.it #3
L_12210: ex9.it #4
L_12212: sbi333 $r1, [$r0 + #0]
L_12214: mov55 $r6, $r0
L_12216: jal	. + (-62166)
L_1221a: beqz38 $r0, . + (-4)
L_1221c: ex9.it #48
L_1221e: sbi333 $r0, [$r6 + #0]
L_12220: ex9.it #27
L_12222: beqz38 $r0, . + (-2)
L_12224: j8 . + (68)
L_12226: jal	. + (-34196)
L_1222a: ex9.it #177
L_1222c: bnec	$r0, #3, . + (10)
L_12230: swi.gp	$r13, [ + #-5440]
L_12234: j8 . + (6)
L_12236: movi55 $r0, #1
L_12238: ex9.it #118
L_1223a: ex9.it #4
L_1223c: ex9.it #3
L_1223e: sbi333 $r1, [$r0 + #0]
L_12240: movi	$r9, #0x3f7f1
L_12244: ex9.it #165
L_12246: mov55 $r3, $r0
L_12248: bnec	$r0, #1, . + (-4)
L_1224c: lbi	$r7, [$r9 + #0]
L_12250: xlsb33 $r7, $r7
L_12252: bnez38 $r7, . + (-14)
L_12254: movi	$r1, #0x2c4a0
L_12258: mov55 $r2, $r1
L_1225a: movi55 $r0, #2
L_1225c: ex9.it #5
L_1225e: mov55 $r0, $r6
L_12260: ex9.it #489
L_12262: sbi.gp	$r7, [ + #-4905]
L_12266: ex9.it #32
L_12268: movi55 $r0, #0
L_1226a: sbi.gp	$r0, [ + #-4905]
L_1226e: ex9.it #409
L_12270: beqc	$r0, #1, . + (8)
L_12274: ex9.it #218
L_12276: j8 . + (4)
L_12278: movi55 $r0, #0
L_1227a: lbi.gp	$r6, [ + #-5626]
L_1227e: bnec	$r6, #1, . + (16)
L_12282: ex9.it #0
L_12284: movi55 $r1, #0
L_12286: ex9.it #20
L_12288: sbi.gp	$r6, [ + #-5594]
L_1228c: ex9.it #32
L_1228e: lbi	$r1, [$r11 + #0]
L_12292: bnez	$r1, . + (678)
L_12296: lbi.gp	$r1, [ + #-5665]
L_1229a: bnez	$r1, . + (670)
L_1229e: bnez	$r0, . + (666)
L_122a2: ex9.it #27
L_122a4: beqc	$r0, #1, . + (8)
L_122a8: j	. + (656)
L_122ac: movi	$r1, #0xd4
L_122b0: movi55 $r0, #6
L_122b2: ex9.it #7
L_122b4: movi55 $r0, #4
L_122b6: ex9.it #62
L_122b8: ex9.it #254
L_122ba: ex9.it #32
L_122bc: ex9.it #63
L_122be: bnec	$r0, #6, . + (32)
L_122c2: movi55 $r0, #0
L_122c4: ex9.it #72
L_122c6: ex9.it #195
L_122c8: ex9.it #484
L_122ca: ex9.it #108
L_122cc: ex9.it #380
L_122ce: ex9.it #6
L_122d0: beqz38 $r0, . + (24)
L_122d2: ex9.it #117
L_122d4: lhi	$r0, [$r0 + #0x20]
L_122d8: jal	. + (-58844)
L_122dc: j8 . + (12)
L_122de: bnec	$r0, #0x11, . + (32)
L_122e2: ex9.it #2
L_122e4: beqz38 $r0, . + (-34)
L_122e6: j8 . + (24)
L_122e8: movi55 $r0, #3
L_122ea: ex9.it #1
L_122ec: ex9.it #209
L_122ee: ex9.it #26
L_122f0: movi55 $r0, #1
L_122f2: ex9.it #59
L_122f4: ex9.it #128
L_122f6: sbi333 $r0, [$r1 + #0]
L_122f8: movi55 $r0, #0xf
L_122fa: ex9.it #10
L_122fc: ex9.it #32
L_122fe: ex9.it #63
L_12300: beqc	$r0, #9, . + (6)
L_12304: ex9.it #32
L_12306: ex9.it #6
L_12308: beqc	$r0, #2, . + (6)
L_1230c: ex9.it #32
L_1230e: ex9.it #0
L_12310: movi55 $r1, #0
L_12312: ex9.it #20
L_12314: movi55 $r0, #3
L_12316: sbi333 $r0, [$r6 + #1]
L_12318: ex9.it #32
L_1231a: movi55 $r0, #1
L_1231c: ex9.it #10
L_1231e: ex9.it #2
L_12320: bnez38 $r0, . + (18)
L_12322: mov55 $r0, $r6
L_12324: ex9.it #308
L_12326: ifcall9 . + (76)
L_12328: movi55 $r0, #5
L_1232a: ex9.it #10
L_1232c: mov55 $r0, $r6
L_1232e: ex9.it #489
L_12330: ex9.it #32
L_12332: ex9.it #30
L_12334: bnez38 $r0, . + (46)
L_12336: movi55 $r0, #2
L_12338: ex9.it #10
L_1233a: jal	. + (-36066)
L_1233e: mov55 $r0, $r6
L_12340: ex9.it #425
L_12342: jal	. + (-19092)
L_12346: ex9.it #12
L_12348: movi55 $r1, #1
L_1234a: sbi333 $r1, [$r0 + #0]
L_1234c: ex9.it #30
L_1234e: beqc	$r0, #4, . + (28)
L_12352: movi55 $r0, #9
L_12354: ex9.it #62
L_12356: movi55 $r0, #3
L_12358: ex9.it #10
L_1235a: ex9.it #2
L_1235c: beqz38 $r0, . + (14)
L_1235e: ifcall9 . + (106)
L_12360: j8 . + (10)
L_12362: bnec	$r0, #4, . + (8)
L_12366: movi55 $r0, #1
L_12368: ex9.it #72
L_1236a: movi55 $r0, #4
L_1236c: ex9.it #10
L_1236e: mov55 $r0, $r6
L_12370: ex9.it #308
L_12372: jal	. + (-18308)
L_12376: ex9.it #211
L_12378: movi55 $r9, #1
L_1237a: sbi.gp	$r9, [ + #-5586]
L_1237e: beqc	$r0, #1, . + (10)
L_12382: ex9.it #30
L_12384: bnec	$r0, #4, . + (20)
L_12388: movi55 $r0, #0xa
L_1238a: ex9.it #62
L_1238c: ex9.it #209
L_1238e: ex9.it #26
L_12390: movi55 $r0, #1
L_12392: ex9.it #128
L_12394: sbi333 $r0, [$r1 + #0]
L_12396: ex9.it #32
L_12398: lbi333 $r0, [$r6 + #1]
L_1239a: ex9.it #251
L_1239c: ifcall9 . + (348)
L_1239e: lhi.gp	$r10, [ + #-5546]
L_123a2: lbi333 $r0, [$r6 + #1]
L_123a4: jal	. + (-20366)
L_123a8: slt45 $r10, $r0
L_123aa: bnezs8 . + (36)
L_123ac: movi55 $r0, #0xb
L_123ae: ex9.it #62
L_123b0: movi55 $r0, #0
L_123b2: movi55 $r1, #0xb
L_123b4: swi333 $r0, [$r7 + #0xc]
L_123b6: sbi333 $r1, [$r6 + #1]
L_123b8: ex9.it #146
L_123ba: ex9.it #12
L_123bc: sbi.gp	$r9, [ + #-5594]
L_123c0: ex9.it #405
L_123c2: ex9.it #2
L_123c4: beqz	$r0, . + (380)
L_123c8: jal	. + (-36202)
L_123cc: ex9.it #32
L_123ce: movi55 $r0, #9
L_123d0: ex9.it #62
L_123d2: movi55 $r0, #5
L_123d4: ex9.it #10
L_123d6: ex9.it #91
L_123d8: jal	. + (-64986)
L_123dc: movi55 $r0, #6
L_123de: ex9.it #10
L_123e0: ex9.it #2
L_123e2: mov55 $r1, $r0
L_123e4: bnez38 $r0, . + (-28)
L_123e6: movi55 $r0, #2
L_123e8: mov55 $r2, $r1
L_123ea: movi55 $r3, #1
L_123ec: mov55 $r4, $r1
L_123ee: mov55 $r5, $r1
L_123f0: ex9.it #23
L_123f2: ex9.it #32
L_123f4: ex9.it #2
L_123f6: bnez38 $r0, . + (4)
L_123f8: ex9.it #128
L_123fa: lbi333 $r0, [$r6 + #0]
L_123fc: bnec	$r0, #2, . + (62)
L_12400: sbi	$r8, [$r12 + #0]
L_12404: ex9.it #21
L_12406: lbi333 $r0, [$r1 + #0]
L_12408: beqz38 $r0, . + (-2)
L_1240a: lbi333 $r0, [$r6 + #1]
L_1240c: ex9.it #251
L_1240e: swi37.sp $r0, [ + #4]
L_12410: movi55 $r13, #0
L_12412: movi	$r0, #0x2c404
L_12416: lwi37.sp $r1, [ + #4]
L_12418: movi	$r2, #0x2c46c
L_1241c: jal	. + (-19702)
L_12420: shi.gp	$r13, [ + #-5546]
L_12424: sbi	$r10, [$r6 + #1]
L_12428: ex9.it #0
L_1242a: ex9.it #64
L_1242c: movi55 $r0, #0
L_1242e: sbi	$r13, [$r7 + #0x1c]
L_12432: swi333 $r0, [$r7 + #0xc]
L_12434: sbi.gp	$r13, [ + #-5594]
L_12438: ifcall9 . + (166)
L_1243a: ex9.it #63
L_1243c: bnec	$r0, #7, . + (24)
L_12440: movi55 $r0, #9
L_12442: sbi333 $r0, [$r6 + #1]
L_12444: ex9.it #209
L_12446: ex9.it #26
L_12448: movi55 $r0, #1
L_1244a: ex9.it #128
L_1244c: sbi333 $r0, [$r1 + #0]
L_1244e: movpi45 $r0, #0x18
L_12450: swi.gp	$r0, [ + #-5328]
L_12454: movi55 $r0, #0
L_12456: sbi	$r0, [$r7 + #0x1c]
L_1245a: ex9.it #32
L_1245c: lbi.gp	$r0, [ + #-5594]
L_12460: bnec	$r0, #1, . + (50)
L_12464: ex9.it #0
L_12466: ex9.it #64
L_12468: jal	. + (-36336)
L_1246c: ex9.it #0
L_1246e: movi55 $r1, #9
L_12470: sbi333 $r1, [$r0 + #1]
L_12472: movi55 $r1, #0
L_12474: ex9.it #20
L_12476: ex9.it #0
L_12478: movi55 $r1, #0xb
L_1247a: sbi333 $r1, [$r0 + #1]
L_1247c: jal	. + (-1460)
L_12480: movi55 $r0, #2
L_12482: ex9.it #26
L_12484: ex9.it #195
L_12486: movi55 $r0, #0
L_12488: sbi333 $r0, [$r1 + #0]
L_1248a: ex9.it #59
L_1248c: movi55 $r0, #3
L_1248e: ex9.it #10
L_12490: j8 . + (176)
L_12492: bnec	$r0, #2, . + (174)
L_12496: movi55 $r0, #4
L_12498: ex9.it #10
L_1249a: sbi.gp	$r9, [ + #-5594]
L_1249e: ex9.it #2
L_124a0: bnez38 $r0, . + (16)
L_124a2: mov55 $r0, $r6
L_124a4: ex9.it #489
L_124a6: lbi.gp	$r0, [ + #-4839]
L_124aa: bnez	$r0, . + (-1394)
L_124ae: j8 . + (146)
L_124b0: lbi	$r0, [$r11 + #0]
L_124b4: bnez38 $r0, . + (6)
L_124b6: ex9.it #211
L_124b8: beqz38 $r0, . + (8)
L_124ba: lbi.gp	$r0, [ + #-6328]
L_124be: bnez38 $r0, . + (36)
L_124c0: movi55 $r0, #9
L_124c2: ex9.it #62
L_124c4: sbi	$r10, [$r6 + #1]
L_124c8: ex9.it #0
L_124ca: ex9.it #64
L_124cc: ex9.it #0
L_124ce: movi55 $r1, #1
L_124d0: ex9.it #20
L_124d2: movi55 $r0, #0
L_124d4: sbi	$r9, [$r7 + #0x1c]
L_124d8: swi333 $r0, [$r7 + #0xc]
L_124da: jal	. + (-36444)
L_124de: j	. + (-1446)
L_124e2: movi55 $r0, #0xa
L_124e4: ex9.it #62
L_124e6: ex9.it #209
L_124e8: ex9.it #12
L_124ea: movi55 $r1, #1
L_124ec: sbi333 $r1, [$r0 + #0]
L_124ee: j8 . + (82)
L_124f0: movi55 $r0, #3
L_124f2: ex9.it #10
L_124f4: lbi333 $r0, [$r6 + #1]
L_124f6: ex9.it #251
L_124f8: jal	. + (-20934)
L_124fc: ex9.it #0
L_124fe: jal	. + (-2766)
L_12502: beqz38 $r0, . + (28)
L_12504: movi55 $r0, #4
L_12506: ex9.it #10
L_12508: movi55 $r0, #1
L_1250a: sbi.gp	$r0, [ + #-4905]
L_1250e: movi55 $r0, #6
L_12510: ex9.it #62
L_12512: mov55 $r0, $r6
L_12514: jal	. + (-2578)
L_12518: movi55 $r0, #5
L_1251a: ex9.it #10
L_1251c: j8 . + (36)
L_1251e: movi	$r1, #0xd3
L_12522: movi55 $r0, #6
L_12524: ex9.it #7
L_12526: movi55 $r0, #4
L_12528: ex9.it #62
L_1252a: ex9.it #0
L_1252c: movi55 $r1, #1
L_1252e: ex9.it #20
L_12530: ex9.it #254
L_12532: j8 . + (14)
L_12534: movi55 $r0, #2
L_12536: ex9.it #10
L_12538: movi55 $r0, #3
L_1253a: ex9.it #62
L_1253c: jal	. + (-20522)
L_12540: pop25 $r14, #8
L_12542: push25 $r10, #0
L_12544: beqz38 $r0, . + (4)
L_12546: ifcall9 . + (652)
L_12548: movi	$r6, #0x65
L_1254c: ex9.it #115
L_1254e: ex9.it #120
L_12550: ex9.it #6
L_12552: beqz38 $r0, . + (28)
L_12554: subi45 $r6, #1
L_12556: beqz38 $r6, . + (24)
L_12558: movi55 $r1, #1
L_1255a: mov55 $r2, $r1
L_1255c: mov55 $r3, $r1
L_1255e: movi55 $r4, #0
L_12560: movi	$r0, #0x4f00
L_12564: ex9.it #191
L_12566: ex9.it #287
L_12568: ex9.it #57
L_1256a: ex9.it #82
L_1256c: j8 . + (-28)
L_1256e: movi55 $r0, #0
L_12570: ex9.it #415
L_12572: pop25 $r10, #0
L_12574: movi	$r1, #0x55
L_12578: ex9.it #3
L_1257a: sbi333 $r1, [$r0 + #0]
L_1257c: ex9.it #134
L_1257e: lbi333 $r0, [$r1 + #0]
L_12580: beqc	$r0, #1, . + (-2)
L_12584: ret5 $lp
L_12586: nop16
L_12588: push25 $r10, #0
L_1258a: mov55 $r7, $r0
L_1258c: mov55 $r9, $r2
L_1258e: ex9.it #369
L_12590: mov55 $r6, $r0
L_12592: jal	. + (-30)
# Selector 1..15: checked unsigned16 relative-target dispatch.
L_12596: subi333 $r1, $r7, #1
L_12598: slti45 $r1, #0xf
L_1259a: beqz	$r15, . + (824)
L_1259e: movi	$r15, #0x123ac
L_125a2: lh	$r1, [$r15 + ($r1 << #1)]
L_125a6: ex9.it #158
L_125a8: jr	$r15
# Computes index r7-1 and tests unsigned index <15; loads u16 at r15+index*2 explicitly, adds r15 through EX9 slot158, jumps to r15.
L_125ac:
    .short L_125ca - L_125ac
    .short L_12610 - L_125ac
    .short L_126de - L_125ac
    .short L_1280a - L_125ac
    .short L_12650 - L_125ac
    .short L_128d2 - L_125ac
    .short L_128d2 - L_125ac
    .short L_128d2 - L_125ac
    .short L_128d2 - L_125ac
    .short L_1282c - L_125ac
    .short L_12860 - L_125ac
    .short L_128b4 - L_125ac
    .short L_128d2 - L_125ac
    .short L_128d2 - L_125ac
    .short L_128d6 - L_125ac
L_125ca: movi55 $r0, #4
L_125cc: movpi45 $r1, #0x11
L_125ce: ex9.it #7
L_125d0: movi	$r0, #0x30
L_125d4: ex9.it #1
L_125d6: movi55 $r0, #0xe
L_125d8: ex9.it #99
L_125da: movi55 $r0, #5
L_125dc: ex9.it #138
L_125de: movi55 $r0, #0
L_125e0: ex9.it #136
L_125e2: ex9.it #193
L_125e4: ex9.it #151
L_125e6: ex9.it #6
L_125e8: bnec	$r0, #2, . + (8)
L_125ec: sbi333 $r7, [$r6 + #0]
L_125ee: j8 . + (-8)
L_125f0: ex9.it #6
L_125f2: beqz38 $r0, . + (22)
L_125f4: movi55 $r1, #1
L_125f6: movi	$r0, #0x5af0
L_125fa: mov55 $r2, $r1
L_125fc: mov55 $r3, $r1
L_125fe: mov55 $r4, $r1
L_12600: ex9.it #191
L_12602: ex9.it #287
L_12604: jal	. + (-194)
L_12608: mov55 $r0, $r9
L_1260a: movi55 $r1, #1
L_1260c: ex9.it #20
L_1260e: j8 . + (44)
L_12610: movi55 $r0, #4
L_12612: movpi45 $r1, #0x12
L_12614: ex9.it #7
L_12616: movi55 $r0, #0xe
L_12618: ex9.it #99
L_1261a: movi55 $r0, #6
L_1261c: ex9.it #138
L_1261e: movi55 $r0, #0
L_12620: ex9.it #136
L_12622: mov55 $r5, $r6
L_12624: addi	$r0, $r6, #0x41
L_12628: movi55 $r1, #-1
L_1262a: ex9.it #239
L_1262c: bnes38 $r0, . + (-2)
L_1262e: mov55 $r0, $r9
L_12630: movi55 $r1, #1
L_12632: ex9.it #20
L_12634: ex9.it #6
L_12636: beqc	$r0, #2, . + (10)
L_1263a: movi55 $r0, #1
L_1263c: sbi.gp	$r0, [ + #-4908]
L_12640: movi55 $r1, #0
L_12642: movi55 $r0, #2
L_12644: mov55 $r2, $r1
L_12646: movi55 $r3, #1
L_12648: mov55 $r4, $r1
L_1264a: mov55 $r5, $r1
L_1264c: ex9.it #23
L_1264e: ifcall9 . + (440)
L_12650: movpi45 $r1, #0x15
L_12652: movi55 $r0, #4
L_12654: ex9.it #7
L_12656: movi	$r0, #0x31
L_1265a: ex9.it #1
L_1265c: movi55 $r7, #0
L_1265e: movi	$r0, #0x3f429
L_12662: sbi333 $r7, [$r0 + #0]
L_12664: ifcall9 . + (224)
L_12666: movi55 $r0, #1
L_12668: sbi.gp	$r7, [ + #-5710]
L_1266c: ex9.it #138
L_1266e: sbi	$r7, [$r6 + #0x50]
L_12672: ex9.it #273
L_12674: ex9.it #341
L_12676: lbi.gp	$r2, [ + #-5689]
L_1267a: movi55 $r4, #1
L_1267c: lbi.gp	$r3, [ + #-5656]
L_12680: movi	$r5, #0x2c504
L_12684: jal	. + (-13376)
L_12688: ex9.it #11
L_1268a: lbi333 $r0, [$r0 + #0]
L_1268c: bnez38 $r0, . + (12)
L_1268e: ex9.it #80
L_12690: movi	$r1, #0x2ed60
L_12694: ex9.it #114
L_12696: j8 . + (14)
L_12698: bnec	$r0, #3, . + (12)
L_1269c: ex9.it #80
L_1269e: movi	$r1, #0x2e548
L_126a2: ex9.it #114
L_126a4: movi55 $r0, #0
L_126a6: sbi.gp	$r0, [ + #-4756]
L_126aa: lbi.gp	$r1, [ + #-4753]
L_126ae: movi55 $r0, #0
L_126b0: ex9.it #46
L_126b2: movi55 $r0, #1
L_126b4: sbi.gp	$r0, [ + #-5688]
L_126b8: beqc	$r1, #1, . + (6)
L_126bc: ex9.it #464
L_126be: movi55 $r0, #0
L_126c0: ex9.it #113
L_126c2: ex9.it #21
L_126c4: movi55 $r0, #2
L_126c6: sbi333 $r0, [$r1 + #0]
L_126c8: ex9.it #26
L_126ca: sbi333 $r0, [$r1 + #0]
L_126cc: ex9.it #139
L_126ce: subi45 $r0, #8
L_126d0: slti45 $r0, #4
L_126d2: beqz	$r15, . + (516)
L_126d6: mov55 $r0, $r9
L_126d8: jal	. + (-40530)
L_126dc: ifcall9 . + (298)
L_126de: movi55 $r0, #4
L_126e0: movpi45 $r1, #0x13
L_126e2: ex9.it #7
L_126e4: ex9.it #180
L_126e6: ex9.it #1
L_126e8: ex9.it #12
L_126ea: movi55 $r2, #2
L_126ec: sbi333 $r2, [$r0 + #0]
L_126ee: ex9.it #236
L_126f0: subi45 $r0, #2
L_126f2: slti45 $r0, #3
L_126f4: bnezs8 . + (12)
L_126f6: ex9.it #341
L_126f8: andi	$r0, $r1, #0xfd
L_126fc: bnec	$r0, #8, . + (14)
L_12700: movi55 $r0, #0
L_12702: ex9.it #136
L_12704: ex9.it #200
L_12706: ex9.it #1
L_12708: ifcall9 . + (254)
L_1270a: sbi	$r2, [$r9 + #0]
L_1270e: lbi.gp	$r0, [ + #-4787]
L_12712: bnez38 $r0, . + (18)
L_12714: subi333 $r0, $r1, #3
L_12716: slti45 $r0, #4
L_12718: bnezs8 . + (24)
L_1271a: beqc	$r1, #0xd, . + (22)
L_1271e: bnec	$r1, #0xe, . + (24)
L_12722: j8 . + (6)
L_12724: bnec	$r1, #0xe, . + (12)
L_12728: ex9.it #472
L_1272a: movi55 $r0, #0
L_1272c: jal	. + (-40318)
L_12730: movi55 $r0, #8
L_12732: ex9.it #99
L_12734: j8 . + (6)
L_12736: movi55 $r0, #9
L_12738: ex9.it #99
L_1273a: movi55 $r7, #0
L_1273c: sbi	$r7, [$r6 + #0x50]
L_12740: sbi.gp	$r7, [ + #-5660]
L_12744: jal	. + (-9586)
L_12748: ex9.it #0
L_1274a: ex9.it #64
L_1274c: movi55 $r0, #4
L_1274e: sbi.gp	$r7, [ + #-5570]
L_12752: ex9.it #151
L_12754: ex9.it #10
L_12756: ex9.it #193
L_12758: ex9.it #6
L_1275a: bnec	$r0, #2, . + (10)
L_1275e: ex9.it #286
L_12760: sbi333 $r7, [$r6 + #0]
L_12762: j8 . + (-10)
L_12764: movi	$r1, #0x3f3d1
L_12768: lbi333 $r0, [$r1 + #0]
L_1276a: ex9.it #167
L_1276c: sbi333 $r0, [$r1 + #0]
L_1276e: movi	$r1, #-61
L_12772: movi	$r0, #0x3f429
L_12776: sbi333 $r1, [$r0 + #0]
L_12778: ex9.it #6
L_1277a: beqz38 $r0, . + (16)
L_1277c: movi55 $r1, #1
L_1277e: movi	$r0, #0x5af0
L_12782: mov55 $r2, $r1
L_12784: mov55 $r3, $r1
L_12786: mov55 $r4, $r1
L_12788: ex9.it #191
L_1278a: movi55 $r0, #0
L_1278c: sbi.gp	$r0, [ + #-4756]
L_12790: lbi.gp	$r1, [ + #-4753]
L_12794: movi55 $r0, #0
L_12796: ex9.it #46
L_12798: movi55 $r0, #1
L_1279a: sbi.gp	$r0, [ + #-5688]
L_1279e: beqc	$r1, #1, . + (6)
L_127a2: ex9.it #464
L_127a4: movi55 $r0, #0
L_127a6: ex9.it #113
L_127a8: movi55 $r0, #0
L_127aa: jal	. + (-15624)
L_127ae: movi55 $r1, #0
L_127b0: mov55 $r0, $r9
L_127b2: ex9.it #20
L_127b4: movi55 $r0, #5
L_127b6: ex9.it #10
L_127b8: movi	$r0, #0x33
L_127bc: ex9.it #1
L_127be: ex9.it #63
L_127c0: beqc	$r0, #4, . + (8)
L_127c4: bnec	$r0, #7, . + (12)
L_127c8: movi	$r0, #0x1f4
L_127cc: ex9.it #57
L_127ce: j8 . + (8)
L_127d0: ex9.it #133
L_127d2: jal	. + (-26676)
L_127d6: ex9.it #6
L_127d8: beqz38 $r0, . + (8)
L_127da: movi55 $r0, #0
L_127dc: jal	. + (-60128)
L_127e0: ex9.it #472
L_127e2: movi	$r0, #0x34
L_127e6: ex9.it #1
L_127e8: ex9.it #2
L_127ea: mov55 $r1, $r0
L_127ec: movi55 $r4, #0xf
L_127ee: movi55 $r0, #2
L_127f0: beqz38 $r1, . + (14)
L_127f2: movi55 $r2, #0
L_127f4: mov55 $r1, $r0
L_127f6: mov55 $r3, $r2
L_127f8: mov55 $r5, $r2
L_127fa: ex9.it #23
L_127fc: ifcall9 . + (10)
L_127fe: mov55 $r2, $r1
L_12800: mov55 $r3, $r1
L_12802: mov55 $r5, $r1
L_12804: ex9.it #23
L_12806: j	. + (208)
L_1280a: movi55 $r0, #4
L_1280c: movpi45 $r1, #0x14
L_1280e: ex9.it #7
L_12810: movi55 $r1, #0
L_12812: ex9.it #0
L_12814: ex9.it #20
L_12816: movi55 $r0, #0xa
L_12818: ex9.it #99
L_1281a: movi55 $r0, #3
L_1281c: ex9.it #138
L_1281e: movi55 $r0, #0
L_12820: ex9.it #136
L_12822: ex9.it #0
L_12824: ex9.it #64
L_12826: movi55 $r0, #1
L_12828: ex9.it #363
L_1282a: j8 . + (172)
L_1282c: movi55 $r0, #4
L_1282e: movpi45 $r1, #0x1a
L_12830: ex9.it #7
L_12832: movi55 $r0, #0xe
L_12834: ex9.it #99
L_12836: movi55 $r0, #6
L_12838: ex9.it #138
L_1283a: mov55 $r5, $r6
L_1283c: addi	$r0, $r6, #0x41
L_12840: movi55 $r1, #-1
L_12842: ex9.it #239
L_12844: bnes38 $r0, . + (-2)
L_12846: mov55 $r0, $r9
L_12848: movi55 $r1, #1
L_1284a: ex9.it #20
L_1284c: movi55 $r1, #0
L_1284e: movi55 $r0, #0
L_12850: movi55 $r3, #1
L_12852: ex9.it #136
L_12854: mov55 $r2, $r1
L_12856: movi55 $r0, #2
L_12858: mov55 $r4, $r3
L_1285a: mov55 $r5, $r1
L_1285c: ex9.it #23
L_1285e: j8 . + (120)
L_12860: movpi45 $r1, #0x1b
L_12862: movi55 $r0, #4
L_12864: ex9.it #7
L_12866: ex9.it #6
L_12868: ex9.it #18
L_1286a: bnec	$r0, #2, . + (26)
L_1286e: lbi333 $r0, [$r1 + #1]
L_12870: beqc	$r0, #3, . + (38)
L_12874: mov55 $r0, $r1
L_12876: movi55 $r1, #0
L_12878: ex9.it #20
L_1287a: jal	. + (-21352)
L_1287e: movi55 $r0, #1
L_12880: ex9.it #59
L_12882: j8 . + (20)
L_12884: lbi333 $r0, [$r1 + #1]
L_12886: beqc	$r0, #0xa, . + (16)
L_1288a: mov55 $r0, $r1
L_1288c: movi55 $r1, #0
L_1288e: ex9.it #20
L_12890: ex9.it #209
L_12892: movi55 $r0, #1
L_12894: ex9.it #59
L_12896: lbi	$r0, [$r6 + #0x51]
L_1289a: lbi	$r1, [$r6 + #0x52]
L_1289e: movi	$r2, #0x2c3d0
L_128a2: movi	$r3, #0x2c39f
L_128a6: movi	$r4, #0x2c3e3
L_128aa: jal	. + (-3906)
L_128ae: movi55 $r0, #0
L_128b0: ex9.it #136
L_128b2: j8 . + (36)
L_128b4: movi55 $r0, #4
L_128b6: movpi45 $r1, #0x1c
L_128b8: ex9.it #7
L_128ba: lbi	$r0, [$r6 + #0x51]
L_128be: beqc	$r0, #1, . + (10)
L_128c2: bnec	$r0, #2, . + (10)
L_128c6: movi55 $r0, #0
L_128c8: sbi.gp	$r0, [ + #-5626]
L_128cc: movi55 $r0, #0
L_128ce: ex9.it #136
L_128d0: j8 . + (6)
L_128d2: movi55 $r0, #-5
L_128d4: ex9.it #136
L_128d6: pop25 $r10, #0
L_128d8: swi.gp	$r0, [ + #-4056]
L_128dc: ret5 $lp
L_128de: push25 $r14, #0
L_128e0: lwi	$r14, [$sp + #0x34]
L_128e4: bnez38 $r0, . + (20)
L_128e6: lwi450 $r3, [$r1]
L_128e8: lwi333 $r1, [$r1 + #4]
L_128ea: slli333 $r0, $r3, #1
L_128ec: slli333 $r1, $r1, #1
L_128ee: or_srli	$r1, $r1, $r3, #0x1f
L_128f2: ex9.it #85
L_128f4: ex9.it #42
L_128f6: j8 . + (140)
L_128f8: slli333 $r4, $r4, #1
L_128fa: add333 $r0, $r3, $r4
L_128fc: subi333 $r1, $r4, #2
L_128fe: addi45 $r4, #2
L_12900: lhi333 $r0, [$r0 + #0]
L_12902: add45 $r1, $r3
L_12904: add45 $r3, $r4
L_12906: lhi333 $r7, [$r1 + #0]
L_12908: lhi333 $r6, [$r3 + #0]
L_1290a: seh33 $r0, $r0
L_1290c: mov55 $r10, $r5
L_1290e: seh33 $r7, $r7
L_12910: seh33 $r6, $r6
L_12912: blez	$r0, . + (12)
L_12916: ex9.it #262
L_12918: ifcall9 . + (36)
L_1291a: movd44 $r12, $r0
L_1291c: j8 . + (6)
L_1291e: movi55 $r12, #0
L_12920: mov55 $r13, $r12
L_12922: blez	$r7, . + (14)
L_12926: mov55 $r0, $r7
L_12928: ex9.it #262
L_1292a: ifcall9 . + (18)
L_1292c: movd44 $r8, $r0
L_1292e: j8 . + (6)
L_12930: movi55 $r8, #0
L_12932: mov55 $r9, $r8
L_12934: blez	$r6, . + (16)
L_12938: mov55 $r0, $r6
L_1293a: ex9.it #262
L_1293c: jal	. + (18208)
L_12940: movd44 $r6, $r0
L_12942: j8 . + (6)
L_12944: movi55 $r6, #0
L_12946: mov55 $r7, $r6
L_12948: slli	$r0, $r10, #1
L_1294c: ex9.it #262
L_1294e: movd44 $r10, $r0
L_12950: movd44 $r2, $r6
L_12952: movd44 $r0, $r8
L_12954: ex9.it #105
L_12956: movd44 $r2, $r0
L_12958: movd44 $r0, $r10
L_1295a: ex9.it #38
L_1295c: movd44 $r10, $r0
L_1295e: movd44 $r2, $r6
L_12960: movd44 $r0, $r8
L_12962: ex9.it #39
L_12964: movd44 $r2, $r12
L_12966: ex9.it #105
L_12968: movd44 $r2, $r12
L_1296a: ex9.it #105
L_1296c: movd44 $r2, $r0
L_1296e: movd44 $r0, $r10
L_12970: jal	. + (20428)
L_12974: jal	. + (21140)
L_12978: lhi	$r1, [$sp + #0x30]
L_1297c: srli45 $r0, #1
L_1297e: add_slli	$r0, $r0, $r1, #1
L_12982: shi	$r0, [$r14 + #0]
L_12986: pop25 $r14, #0
L_12988: push25 $r14, #24
L_1298a: mov55 $r7, $r0
L_1298c: lhi.gp	$r0, [ + #-9268]
L_12990: ex9.it #130
L_12992: ex9.it #147
L_12994: lbi333 $r0, [$r7 + #4]
L_12996: mov55 $r9, $r1
L_12998: mov55 $r6, $r2
L_1299a: mov55 $r10, $r3
L_1299c: beqz38 $r0, . + (18)
L_1299e: lbi333 $r0, [$r7 + #5]
L_129a0: beqc	$r0, #0x1f, . + (14)
L_129a4: lbi333 $r0, [$r7 + #6]
L_129a6: beqz38 $r0, . + (8)
L_129a8: lbi333 $r0, [$r7 + #7]
L_129aa: bnec	$r0, #0x31, . + (18)
L_129ae: lbi	$r0, [$r6 + #0x45]
L_129b2: bnec	$r0, #1, . + (10)
L_129b6: ex9.it #427
L_129b8: ex9.it #130
L_129ba: ex9.it #147
L_129bc: lbi	$r0, [$r7 + #9]
L_129c0: beqz38 $r0, . + (16)
L_129c2: beqc	$r0, #0x1f, . + (14)
L_129c6: lbi	$r0, [$r7 + #8]
L_129ca: beqz38 $r0, . + (6)
L_129cc: bnec	$r0, #0x31, . + (30)
L_129d0: lbi	$r0, [$r6 + #0x42]
L_129d4: beqz38 $r0, . + (14)
L_129d6: lwi450 $r0, [$r6]
L_129d8: bnec	$r0, #5, . + (18)
L_129dc: lbi	$r0, [$r6 + #0x46]
L_129e0: bnez38 $r0, . + (10)
L_129e2: movi	$r0, #0x1f4
L_129e6: ex9.it #130
L_129e8: ex9.it #147
L_129ea: lhi333 $r5, [$r7 + #8]
L_129ec: beqz38 $r5, . + (48)
L_129ee: movi	$r0, #0x1f00
L_129f2: beqs38 $r0, . + (42)
L_129f4: bnec	$r5, #0x31, . + (34)
L_129f8: lbi	$r0, [$r6 + #0x42]
L_129fc: beqz38 $r0, . + (14)
L_129fe: lwi450 $r0, [$r6]
L_12a00: bnec	$r0, #5, . + (54)
L_12a04: lbi	$r0, [$r6 + #0x46]
L_12a08: bnez38 $r0, . + (46)
L_12a0a: movi	$r0, #0x7d0
L_12a0e: ex9.it #130
L_12a10: movi	$r0, #0x708
L_12a14: j8 . + (32)
L_12a16: movi	$r0, #0x1f31
L_12a1a: bnes38 $r0, . + (28)
L_12a1c: lbi	$r0, [$r6 + #0x42]
L_12a20: beqz38 $r0, . + (14)
L_12a22: lwi450 $r0, [$r6]
L_12a24: bnec	$r0, #5, . + (18)
L_12a28: lbi	$r0, [$r6 + #0x46]
L_12a2c: bnez38 $r0, . + (10)
L_12a2e: movi	$r0, #0x3e8
L_12a32: ex9.it #130
L_12a34: ex9.it #147
L_12a36: lwi.gp	$r0, [ + #0x684]
L_12a3a: bgtz	$r0, . + (382)
L_12a3e: bnez38 $r0, . + (16)
L_12a40: lwi.gp	$r0, [ + #0x680]
L_12a44: movi	$r1, #0x752f
L_12a48: slt45 $r1, $r0
L_12a4a: bnez	$r15, . + (366)
L_12a4e: lwi.gp	$r0, [ + #0x68c]
L_12a52: bgtz	$r0, . + (358)
L_12a56: bnez38 $r0, . + (16)
L_12a58: lwi.gp	$r0, [ + #0x688]
L_12a5c: movi	$r1, #0x752f
L_12a60: slt45 $r1, $r0
L_12a62: bnez	$r15, . + (342)
L_12a66: lbi.gp	$r0, [ + #-5613]
L_12a6a: bnez	$r0, . + (334)
L_12a6e: lhi333 $r0, [$r7 + #0]
L_12a70: beqz	$r0, . + (328)
L_12a74: lhi333 $r0, [$r7 + #2]
L_12a76: beqz	$r0, . + (322)
L_12a7a: lbi.gp	$r0, [ + #-4794]
L_12a7e: bnez	$r0, . + (314)
L_12a82: lhi	$r1, [$sp + #0x14]
L_12a86: ex9.it #490
L_12a88: slt45 $r1, $r0
L_12a8a: beqz	$r15, . + (302)
L_12a8e: lhi	$r1, [$sp + #0x16]
L_12a92: ex9.it #124
L_12a94: slt45 $r1, $r0
L_12a96: beqz	$r15, . + (290)
L_12a9a: ex9.it #409
L_12a9c: bnez	$r0, . + (284)
L_12aa0: movi55 $r1, #1
L_12aa2: sbi	$r1, [$r10 + #0]
L_12aa6: ex9.it #272
L_12aa8: lhi.gp	$r4, [ + #0x67c]
L_12aac: bnez38 $r1, . + (14)
L_12aae: beqz38 $r4, . + (10)
L_12ab0: addi	$r0, $r4, #-31
L_12ab4: ex9.it #325
L_12ab6: j8 . + (4)
L_12ab8: mov55 $r0, $r4
L_12aba: movi	$r1, #0x194b8
L_12abe: lh	$r5, [$r1 + ($r4 << #1)]
L_12ac2: movi	$r1, #0x19478
L_12ac6: lh	$r1, [$r1 + ($r4 << #1)]
L_12aca: swi450 $r1, [$sp]
L_12acc: addi333 $r1, $r6, #4
L_12ace: swi37.sp $r1, [ + #4]
L_12ad0: ex9.it #140
L_12ad2: swi37.sp $r1, [ + #8]
L_12ad4: addi	$r2, $r7, #0x28
L_12ad8: addi	$r1, $r7, #0x10
L_12adc: lwi.gp	$r3, [ + #0x670]
L_12ae0: ifcall9 . + (68)
L_12ae2: ex9.it #55
L_12ae4: lhi.gp	$r4, [ + #0x67e]
L_12ae8: bnez38 $r0, . + (14)
L_12aea: beqz38 $r4, . + (16)
L_12aec: addi	$r1, $r4, #-49
L_12af0: slt	$r0, $r0, $r1
L_12af4: j8 . + (8)
L_12af6: movi55 $r0, #0
L_12af8: j8 . + (4)
L_12afa: mov55 $r0, $r4
L_12afc: movi	$r1, #0x19414
L_12b00: lh	$r5, [$r1 + ($r4 << #1)]
L_12b04: movi	$r1, #0x193b0
L_12b08: lh	$r1, [$r1 + ($r4 << #1)]
L_12b0c: swi450 $r1, [$sp]
L_12b0e: addi333 $r1, $r6, #6
L_12b10: swi37.sp $r1, [ + #4]
L_12b12: movi	$r1, #0x68
L_12b16: swi37.sp $r1, [ + #8]
L_12b18: addi	$r2, $r7, #0x30
L_12b1c: lwi.gp	$r3, [ + #0x674]
L_12b20: addi	$r1, $r7, #0x18
L_12b24: jal	. + (-582)
L_12b28: lwi	$r3, [$r9 + #0x2c]
L_12b2c: lwi	$r2, [$r9 + #0x28]
L_12b30: bgtz	$r3, . + (12)
L_12b34: bnez38 $r3, . + (124)
L_12b36: slti	$r15, $r2, #0x12d
L_12b3a: bnezs8 . + (118)
L_12b3c: lwi	$r10, [$r9 + #0x34]
L_12b40: lwi	$r11, [$r9 + #0x30]
L_12b44: bgtz	$r10, . + (14)
L_12b48: bnez	$r10, . + (104)
L_12b4c: slti	$r15, $r11, #0x12d
L_12b50: bnezs8 . + (96)
L_12b52: lwi.gp	$r0, [ + #0xe7c]
L_12b56: subi45 $r0, #2
L_12b58: slti45 $r0, #5
L_12b5a: beqzs8 . + (86)
L_12b5c: mov55 $r7, $r9
L_12b5e: lwi333 $r0, [$r7 + #0x14]
L_12b60: lwi333 $r4, [$r7 + #0x10]
L_12b62: slli333 $r1, $r0, #1
L_12b64: or_srli	$r1, $r1, $r4, #0x1f
L_12b68: slli333 $r0, $r4, #1
L_12b6a: ex9.it #42
L_12b6c: lwi333 $r1, [$r7 + #0x1c]
L_12b6e: lwi333 $r2, [$r7 + #0x18]
L_12b70: seh33 $r7, $r0
L_12b72: slli333 $r1, $r1, #1
L_12b74: shi	$r7, [$r6 + #0x18]
L_12b78: slli333 $r0, $r2, #1
L_12b7a: or_srli	$r1, $r1, $r2, #0x1f
L_12b7e: mov55 $r3, $r10
L_12b80: mov55 $r2, $r11
L_12b82: ex9.it #42
L_12b84: lhsi	$r15, [$r6 + #4]
L_12b88: seh33 $r1, $r0
L_12b8a: sub45 $r7, $r15
L_12b8c: abs	$r15, $r7
L_12b90: movi55 $r0, #1
L_12b92: sltsi	$r15, $r15, #0xf1
L_12b96: sbi	$r0, [$r6 + #0x7a]
L_12b9a: shi	$r1, [$r6 + #0x1a]
L_12b9e: lhsi	$r0, [$r6 + #6]
L_12ba2: beqzs8 . + (14)
L_12ba4: sub45 $r1, $r0
L_12ba6: abs	$r15, $r1
L_12baa: sltsi	$r15, $r15, #0xf1
L_12bae: bnezs8 . + (14)
L_12bb0: movi55 $r0, #0
L_12bb2: sbi	$r0, [$r6 + #0x7a]
L_12bb6: j8 . + (6)
L_12bb8: movi55 $r0, #0
L_12bba: ex9.it #173
L_12bbc: pop25 $r14, #24
L_12bbe: push25 $r8, #0
L_12bc0: lwi.gp	$r16, [ + #-4056]
L_12bc4: mov55 $r6, $r1
L_12bc6: sub333 $r4, $r6, $r1
L_12bc8: slt45 $r4, $r2
L_12bca: beqzs8 . + (60)
L_12bcc: lbi.bi	$r7, [$r6], #1
L_12bd0: lwi	$r5, [$r0 + #0x20]
L_12bd4: lbi	$r18, [$r16 + #0x32]
L_12bd8: lw	$r17, [$r5 + ($r7 << #2)]
L_12bdc: lwi333 $r5, [$r0 + #0x18]
L_12bde: lwi333 $r4, [$r0 + #0x1c]
L_12be0: lw	$r5, [$r5 + ($r7 << #2)]
L_12be4: lw	$r4, [$r4 + ($r7 << #2)]
L_12be8: mul	$r5, $r18, $r5
L_12bec: addi45 $r3, #8
L_12bee: divr	$r5, $r5, $r5, $r17
L_12bf2: shi	$r5, [$r3 + #-8]
L_12bf6: lbi	$r5, [$r16 + #0x33]
L_12bfa: mul33 $r4, $r5
L_12bfc: divr	$r4, $r4, $r4, $r17
L_12c00: shi	$r4, [$r3 + #-6]
L_12c04: j8 . + (-62)
L_12c06: pop25 $r8, #0
L_12c08: push25 $r14, #0
L_12c0a: mov55 $r13, $r1
L_12c0c: ex9.it #266
L_12c0e: mov55 $r9, $r0
L_12c10: movi55 $r0, #0
L_12c12: mov55 $r12, $r2
L_12c14: mov55 $r6, $r3
L_12c16: sbi333 $r0, [$r3 + #0]
L_12c18: bnec	$r1, #2, . + (160)
L_12c1c: movi55 $r7, #0
L_12c1e: movi	$r14, #0x2ca0c
L_12c22: slt45 $r7, $r13
L_12c24: beqzs8 . + (214)
L_12c26: lwi	$r0, [$r9 + #4]
L_12c2a: lh	$r5, [$r0 + ($r7 << #1)]
L_12c2e: bnes38 $r7, . + (134)
L_12c30: addi	$r5, $r9, #0x18
L_12c34: lwi333 $r0, [$r5 + #8]
L_12c36: lwi.gp	$r2, [ + #-4056]
L_12c3a: lw	$r1, [$r0 + ($r7 << #2)]
L_12c3e: lwi450 $r0, [$r5]
L_12c40: lbi	$r3, [$r2 + #0x32]
L_12c44: lw	$r0, [$r0 + ($r7 << #2)]
L_12c48: lhsi.gp	$r4, [ + #0x12cc]
L_12c4c: mul33 $r0, $r3
L_12c4e: lbi	$r2, [$r2 + #0x33]
L_12c52: divr	$r10, $r10, $r0, $r1
L_12c56: lwi333 $r0, [$r5 + #4]
L_12c58: srai45 $r4, #1
L_12c5a: lw	$r0, [$r0 + ($r7 << #2)]
L_12c5e: sub	$r4, $r10, $r4
L_12c62: mul33 $r0, $r2
L_12c64: abs	$r15, $r4
L_12c68: lhsi.gp	$r4, [ + #0x12ce]
L_12c6c: divr	$r11, $r11, $r0, $r1
L_12c70: srai45 $r4, #1
L_12c72: sub	$r4, $r11, $r4
L_12c76: abs	$r4, $r4
L_12c7a: add45 $r4, $r15
L_12c7c: sltsi	$r15, $r4, #0x32
L_12c80: bnezs8 . + (46)
L_12c82: lbi333 $r0, [$r6 + #0]
L_12c84: lwi.gp	$r1, [ + #-4056]
L_12c88: sh	$r10, [$r12 + ($r0 << #3)]
L_12c8c: lbi333 $r0, [$r6 + #0]
L_12c8e: add_slli	$r0, $r12, $r0, #3
L_12c92: shi	$r11, [$r0 + #2]
L_12c96: lbi333 $r0, [$r6 + #0]
L_12c98: sb	$r7, [$r14 + ($r0 << #0)]
L_12c9c: lbi333 $r0, [$r6 + #0]
L_12c9e: addi45 $r0, #1
L_12ca0: zeb33 $r0, $r0
L_12ca2: sbi333 $r0, [$r6 + #0]
L_12ca4: lbi	$r1, [$r1 + #0x19]
L_12ca8: slt45 $r0, $r1
L_12caa: bnezs8 . + (10)
L_12cac: j8 . + (78)
L_12cae: jal	. + (10946)
L_12cb2: beqz38 $r0, . + (-48)
L_12cb4: addi45 $r7, #1
L_12cb6: j8 . + (-148)
L_12cb8: lwi.gp	$r1, [ + #-4056]
L_12cbc: movi55 $r0, #0
L_12cbe: movi	$r2, #0x2ca0c
L_12cc2: slt45 $r0, $r13
L_12cc4: beqzs8 . + (34)
L_12cc6: lwi	$r3, [$r9 + #4]
L_12cca: lh	$r5, [$r3 + ($r0 << #1)]
L_12cce: bnes38 $r0, . + (40)
L_12cd0: lbi333 $r3, [$r6 + #0]
L_12cd2: sb	$r0, [$r2 + ($r3 << #0)]
L_12cd6: lbi333 $r4, [$r6 + #0]
L_12cd8: addi45 $r4, #1
L_12cda: zeb33 $r4, $r4
L_12cdc: sbi333 $r4, [$r6 + #0]
L_12cde: lbi	$r3, [$r1 + #0x19]
L_12ce2: slt45 $r4, $r3
L_12ce4: bnezs8 . + (18)
L_12ce6: mov55 $r0, $r9
L_12ce8: movi	$r1, #0x2ca0c
L_12cec: lbi333 $r2, [$r6 + #0]
L_12cee: mov55 $r3, $r12
L_12cf0: jal	. + (-306)
L_12cf4: j8 . + (6)
L_12cf6: addi45 $r0, #1
L_12cf8: j8 . + (-54)
L_12cfa: movi55 $r0, #0
L_12cfc: movi	$r2, #0x2ca0c
L_12d00: lbi333 $r1, [$r6 + #0]
L_12d02: slt45 $r0, $r1
L_12d04: beqzs8 . + (16)
L_12d06: add_slli	$r1, $r12, $r0, #3
L_12d0a: lb	$r3, [$r2 + ($r0 << #0)]
L_12d0e: swi333 $r3, [$r1 + #4]
L_12d10: addi45 $r0, #1
L_12d12: j8 . + (-18)
L_12d14: pop25 $r14, #0
L_12d16: push25 $r10, #32
L_12d18: lbi.gp	$r16, [ + #-55774]
L_12d1c: mov55 $r6, $r0
L_12d1e: maddr32	$r6, $r1, $r16
L_12d22: lwi450 $r18, [$r4]
L_12d24: slli	$r17, $r6, #1
L_12d28: add	$r9, $r18, $r17
L_12d2c: lbi	$r7, [$r9 + #0]
L_12d30: movi55 $r6, #0
L_12d32: lbi.gp	$r19, [ + #-55773]
L_12d36: swi333 $r6, [$r2 + #4]
L_12d38: swi450 $r6, [$r2]
L_12d3a: bnec	$r7, #0xff, . + (10)
L_12d3e: swi450 $r7, [$r3]
L_12d40: swi333 $r7, [$r3 + #4]
L_12d42: j8 . + (42)
L_12d44: lbi	$r7, [$r9 + #0]
L_12d48: lwi333 $r6, [$r4 + #4]
L_12d4a: lh	$r6, [$r6 + ($r7 << #1)]
L_12d4e: movi	$r7, #0xff
L_12d52: fexti33 $r6, #5
L_12d54: swi333 $r7, [$r3 + #4]
L_12d56: lwi333 $r7, [$r4 + #0x10]
L_12d58: swi450 $r6, [$r3]
L_12d5a: add_slli	$r6, $r7, $r6, #1
L_12d5e: lbi333 $r7, [$r6 + #1]
L_12d60: bne	$r0, $r7, . + (10)
L_12d64: lbi333 $r6, [$r6 + #0]
L_12d66: beq	$r1, $r6, . + (252)
L_12d6a: movi55 $r6, #1
L_12d6c: subi333 $r7, $r0, #1
L_12d6e: swi450 $r7, [$sp]
L_12d70: addi333 $r7, $r0, #1
L_12d72: swi37.sp $r7, [ + #4]
L_12d74: swi37.sp $r0, [ + #8]
L_12d76: addri36.sp $r7, #0x10
L_12d78: swi37.sp $r0, [ + #0xc]
L_12d7a: subi333 $r0, $r1, #1
L_12d7c: swi333 $r0, [$r7 + #8]
L_12d7e: ex9.it #236
L_12d80: swi37.sp $r1, [ + #0x10]
L_12d82: swi333 $r1, [$r7 + #4]
L_12d84: addi45 $r1, #1
L_12d86: swi333 $r1, [$r7 + #0xc]
L_12d88: addi	$r0, $r0, #-255
L_12d8c: movi55 $r1, #0
L_12d8e: ex9.it #325
L_12d90: lw	$r9, [$sp + ($r1 << #2)]
L_12d94: bltz	$r9, . + (62)
L_12d98: slts45 $r9, $r16
L_12d9a: beqzs8 . + (56)
L_12d9c: lw	$r10, [$r7 + ($r1 << #2)]
L_12da0: bltz	$r10, . + (50)
L_12da4: slts45 $r10, $r19
L_12da6: beqzs8 . + (44)
L_12da8: maddr32	$r9, $r10, $r16
L_12dac: lb	$r10, [$r18 + ($r9 << #1)]
L_12db0: beqc	$r10, #0xff, . + (34)
L_12db4: lwi	$r9, [$r4 + #4]
L_12db8: bnez38 $r0, . + (12)
L_12dba: lh	$r0, [$r9 + ($r10 << #1)]
L_12dbe: fexti33 $r0, #5
L_12dc0: swi450 $r0, [$r3]
L_12dc2: j8 . + (14)
L_12dc4: lh	$r0, [$r9 + ($r10 << #1)]
L_12dc8: lwi450 $r10, [$r3]
L_12dca: fexti33 $r0, #5
L_12dcc: bne	$r10, $r0, . + (14)
L_12dd0: movi55 $r0, #1
L_12dd2: addi45 $r1, #1
L_12dd4: bnec	$r1, #4, . + (-68)
L_12dd8: pop25 $r10, #32
L_12dda: movi55 $r4, #0
L_12ddc: lw	$r0, [$sp + ($r4 << #2)]
L_12de0: bltz	$r0, . + (122)
L_12de4: slts45 $r0, $r16
L_12de6: beqzs8 . + (116)
L_12de8: lw	$r1, [$r7 + ($r4 << #2)]
L_12dec: bltz	$r1, . + (110)
L_12df0: slts45 $r1, $r19
L_12df2: beqzs8 . + (104)
L_12df4: maddr32	$r0, $r1, $r16
L_12df8: lb	$r1, [$r18 + ($r0 << #1)]
L_12dfc: beqc	$r1, #0xff, . + (94)
L_12e00: lhs	$r0, [$r5 + ($r0 << #1)]
L_12e04: lhs	$r10, [$r5 + ($r17 << #0)]
L_12e08: slts45 $r10, $r0
L_12e0a: beqzs8 . + (80)
L_12e0c: lh	$r1, [$r9 + ($r1 << #1)]
L_12e10: sub45 $r0, $r10
L_12e12: fexti33 $r1, #5
L_12e14: beqc	$r6, #1, . + (24)
L_12e18: beqz38 $r6, . + (8)
L_12e1a: beqc	$r6, #2, . + (36)
L_12e1e: j8 . + (72)
L_12e20: swi450 $r1, [$r3]
L_12e22: lwi450 $r1, [$r2]
L_12e24: movi55 $r6, #1
L_12e26: add45 $r0, $r1
L_12e28: swi450 $r0, [$r2]
L_12e2a: j8 . + (48)
L_12e2c: lwi450 $r10, [$r3]
L_12e2e: beq	$r1, $r10, . + (22)
L_12e32: swi333 $r1, [$r3 + #4]
L_12e34: lwi333 $r1, [$r2 + #4]
L_12e36: movi55 $r6, #2
L_12e38: add45 $r0, $r1
L_12e3a: swi333 $r0, [$r2 + #4]
L_12e3c: j8 . + (30)
L_12e3e: lwi450 $r10, [$r3]
L_12e40: bne	$r1, $r10, . + (12)
L_12e44: lwi450 $r1, [$r2]
L_12e46: add45 $r0, $r1
L_12e48: swi450 $r0, [$r2]
L_12e4a: j8 . + (16)
L_12e4c: lwi	$r10, [$r3 + #4]
L_12e50: bne	$r1, $r10, . + (26)
L_12e54: lwi333 $r1, [$r2 + #4]
L_12e56: add45 $r0, $r1
L_12e58: swi333 $r0, [$r2 + #4]
L_12e5a: addi45 $r4, #1
L_12e5c: bnec	$r4, #4, . + (-128)
L_12e60: j8 . + (6)
L_12e62: movi55 $r0, #1
L_12e64: pop25 $r10, #32
L_12e66: mov55 $r0, $r6
L_12e68: pop25 $r10, #32
L_12e6a: movi55 $r0, #3
L_12e6c: pop25 $r10, #32
L_12e6e: push25 $r14, #144
L_12e70: mov55 $r6, $r0
L_12e72: lbi.gp	$r0, [ + #-55773]
L_12e76: swi450 $r0, [$sp]
L_12e78: lbi.gp	$r0, [ + #-55726]
L_12e7c: swi37.sp $r0, [ + #4]
L_12e7e: lbi.gp	$r0, [ + #-55725]
L_12e82: swi37.sp $r0, [ + #8]
L_12e84: movi55 $r0, #0
L_12e86: swi37.sp $r2, [ + #0x10]
L_12e88: mov55 $r21, $r4
L_12e8a: mov55 $fp, $r1
L_12e8c: lbi.gp	$r19, [ + #-55774]
L_12e90: addri36.sp $r2, #0x40
L_12e92: mov55 $r4, $r0
L_12e94: addri36.sp $r5, #0x68
L_12e96: slt45 $r0, $r3
L_12e98: beqzs8 . + (20)
L_12e9a: add_slli	$r1, $r21, $r0, #3
L_12e9e: lwi333 $r1, [$r1 + #4]
L_12ea0: sb	$r0, [$r5 + ($r1 << #0)]
L_12ea4: sb	$r4, [$r2 + ($r1 << #0)]
L_12ea8: addi45 $r0, #1
L_12eaa: j8 . + (-20)
L_12eac: movi55 $r2, #0
L_12eae: addri36.sp $r4, #0x40
L_12eb0: mov55 $r17, $r2
L_12eb2: movi55 $r8, #1
L_12eb4: slt45 $r2, $r3
L_12eb6: beqzs8 . + (188)
L_12eb8: add_slli	$r0, $r21, $r2, #3
L_12ebc: lwi333 $r0, [$r0 + #4]
L_12ebe: lwi	$r12, [$r6 + #4]
L_12ec2: lh	$r5, [$r12 + ($r0 << #1)]
L_12ec6: bnes38 $r0, . + (168)
L_12ec8: lb	$r0, [$r4 + ($r5 << #0)]
L_12ecc: bnez38 $r0, . + (162)
L_12ece: slli	$r13, $r5, #2
L_12ed2: beq	$r2, $r0, . + (150)
L_12ed6: add_slli	$r1, $r21, $r0, #3
L_12eda: lwi	$r18, [$r1 + #4]
L_12ede: lwi333 $r1, [$r6 + #0x14]
L_12ee0: slli	$r7, $r18, #2
L_12ee4: add	$r16, $r1, $r13
L_12ee8: add45 $r1, $r7
L_12eea: lbi	$r23, [$r16 + #2]
L_12eee: lbi	$r9, [$r16 + #0]
L_12ef2: lbi	$r14, [$r16 + #1]
L_12ef6: lbi	$r24, [$r1 + #3]
L_12efa: lbi	$r11, [$r1 + #0]
L_12efe: lbi	$r10, [$r1 + #1]
L_12f02: lbi	$r16, [$r16 + #3]
L_12f06: lbi333 $r1, [$r1 + #2]
L_12f08: addi45 $r16, #1
L_12f0a: subi45 $r1, #1
L_12f0c: slts45 $r16, $r1
L_12f0e: bnezs8 . + (90)
L_12f10: addi	$r16, $r24, #1
L_12f14: addi	$r1, $r23, #-1
L_12f18: slts45 $r16, $r1
L_12f1a: bnezs8 . + (78)
L_12f1c: subi45 $r11, #1
L_12f1e: addi	$r1, $r14, #1
L_12f22: slts45 $r1, $r11
L_12f24: bnezs8 . + (68)
L_12f26: addi45 $r10, #1
L_12f28: subi45 $r9, #1
L_12f2a: slts45 $r10, $r9
L_12f2c: bnezs8 . + (60)
L_12f2e: lwi	$r9, [$r6 + #0x18]
L_12f32: lwi	$r16, [$r6 + #0x1c]
L_12f36: ex9.it #304
L_12f38: sw	$r17, [$r9 + ($r5 << #2)]
L_12f3c: sb	$r8, [$r4 + ($r5 << #0)]
L_12f40: sw	$r17, [$r16 + ($r5 << #2)]
L_12f44: sw	$r17, [$r1 + ($r5 << #2)]
L_12f48: lh	$r5, [$r12 + ($r18 << #1)]
L_12f4c: bne	$r5, $r18, . + (34)
L_12f50: lb	$r0, [$r4 + ($r5 << #0)]
L_12f54: bnez38 $r0, . + (26)
L_12f56: sw	$r0, [$r9 + ($r7 << #0)]
L_12f5a: sb	$r8, [$r4 + ($r5 << #0)]
L_12f5e: sw	$r0, [$r16 + ($r7 << #0)]
L_12f62: sw	$r0, [$r1 + ($r7 << #0)]
L_12f66: j8 . + (8)
L_12f68: addi45 $r0, #1
L_12f6a: slt45 $r0, $r3
L_12f6c: bnezs8 . + (-154)
L_12f6e: addi45 $r2, #1
L_12f70: j8 . + (-188)
L_12f72: movi55 $r8, #0
L_12f74: movi	$r25, #-128
L_12f78: lwi37.sp $r0, [ + #0x10]
L_12f7a: slt45 $r8, $r0
L_12f7c: beqz	$r15, . + (512)
L_12f80: lwi333 $r0, [$r6 + #4]
L_12f82: lh	$r5, [$r0 + ($r8 << #1)]
L_12f86: bne	$r5, $r8, . + (496)
L_12f8a: addri36.sp $r0, #0x40
L_12f8c: lb	$r0, [$r8 + ($r0 << #0)]
L_12f90: beqc	$r0, #1, . + (8)
L_12f94: j	. + (482)
L_12f98: lwi333 $r1, [$r6 + #0x14]
L_12f9a: addri36.sp $r0, #0x68
L_12f9c: slli	$r16, $r8, #2
L_12fa0: lb	$r0, [$r8 + ($r0 << #0)]
L_12fa4: add45 $r1, $r16
L_12fa6: swi37.sp $r0, [ + #0xc]
L_12fa8: lbi333 $r0, [$r1 + #2]
L_12faa: movi55 $r3, #0
L_12fac: slti45 $r0, #2
L_12fae: bnezs8 . + (6)
L_12fb0: subi45 $r0, #1
L_12fb2: zeb33 $r3, $r0
L_12fb4: lwi450 $r2, [$sp]
L_12fb6: lbi333 $r0, [$r1 + #3]
L_12fb8: subi45 $r2, #2
L_12fba: slts45 $r0, $r2
L_12fbc: beqzs8 . + (6)
L_12fbe: addi45 $r0, #1
L_12fc0: j8 . + (6)
L_12fc2: lwi450 $r0, [$sp]
L_12fc4: subi45 $r0, #1
L_12fc6: ex9.it #316
L_12fc8: lbi333 $r0, [$r1 + #0]
L_12fca: movi55 $r12, #0
L_12fcc: slti45 $r0, #2
L_12fce: bnezs8 . + (8)
L_12fd0: subi45 $r0, #1
L_12fd2: andi	$r12, $r0, #0xff
L_12fd6: lbi333 $r0, [$r1 + #1]
L_12fd8: addi	$r1, $r19, #-2
L_12fdc: slts45 $r0, $r1
L_12fde: beqzs8 . + (6)
L_12fe0: addi45 $r0, #1
L_12fe2: j8 . + (6)
L_12fe4: addi	$r0, $r19, #-1
L_12fe8: andi	$r11, $r0, #0xff
L_12fec: mov55 $r9, $r3
L_12fee: mul	$r14, $r3, $r19
L_12ff2: slts45 $r10, $r9
L_12ff4: bnez	$r15, . + (346)
L_12ff8: add	$r0, $r14, $r12
L_12ffc: mov55 $r13, $r12
L_12ffe: add_slli	$r17, $fp, $r0, #1
L_13002: addi	$r24, $sp, #0x40
L_13006: slts45 $r11, $r13
L_13008: bnez	$r15, . + (316)
L_1300c: lwi450 $r0, [$r6]
L_1300e: sub	$r18, $r17, $fp
L_13012: add45 $r0, $r18
L_13014: lbsi	$r0, [$r0 + #1]
L_13018: lhsi.bi	$r7, [$r17], #2
L_1301c: bltz	$r0, . + (272)
L_13020: subi45 $r7, #0x1e
L_13022: seh33 $r7, $r7
L_13024: blez	$r7, . + (264)
L_13028: mov55 $r0, $r13
L_1302a: mov55 $r1, $r9
L_1302c: addri36.sp $r2, #0x30
L_1302e: addri36.sp $r3, #0x38
L_13030: mov55 $r4, $r6
L_13032: mov55 $r5, $fp
L_13034: swi	$r25, [$sp + #0x2c]
L_13038: swi	$r24, [$sp + #0x28]
L_1303c: swi	$r21, [$sp + #0x24]
L_13040: swi	$r16, [$sp + #0x20]
L_13044: swi	$r18, [$sp + #0x1c]
L_13048: swi	$r17, [$sp + #0x18]
L_1304c: swi	$r19, [$sp + #0x14]
L_13050: jal	. + (-826)
L_13054: lwi	$r19, [$sp + #0x14]
L_13058: lwi	$r17, [$sp + #0x18]
L_1305c: lwi	$r18, [$sp + #0x1c]
L_13060: lwi	$r16, [$sp + #0x20]
L_13064: lwi	$r21, [$sp + #0x24]
L_13068: lwi	$r24, [$sp + #0x28]
L_1306c: lwi	$r25, [$sp + #0x2c]
L_13070: bnec	$r0, #1, . + (38)
L_13074: lwi37.sp $r0, [ + #0x38]
L_13076: lwi333 $r1, [$r6 + #4]
L_13078: lh	$r5, [$r1 + ($r0 << #1)]
L_1307c: bnes38 $r0, . + (176)
L_1307e: lb	$r0, [$r24 + ($r5 << #0)]
L_13082: bnec	$r0, #1, . + (170)
L_13086: ifcall9 . + (122)
L_13088: lwi37.sp $r0, [ + #0x38]
L_1308a: ifcall9 . + (136)
L_1308c: maddr32	$r1, $r9, $r7
L_13090: swi450 $r1, [$r0]
L_13092: lwi37.sp $r0, [ + #0x38]
L_13094: j8 . + (142)
L_13096: bnec	$r0, #2, . + (150)
L_1309a: lwi333 $r2, [$r6 + #4]
L_1309c: lwi37.sp $r0, [ + #0x38]
L_1309e: lh	$r5, [$r2 + ($r0 << #1)]
L_130a2: bnes38 $r0, . + (66)
L_130a4: lb	$r0, [$r24 + ($r5 << #0)]
L_130a8: bnec	$r0, #1, . + (60)
L_130ac: lwi37.sp $r0, [ + #0x30]
L_130ae: lwi37.sp $r1, [ + #0x34]
L_130b0: mul	$r3, $r0, $r7
L_130b4: add45 $r0, $r1
L_130b6: lwi333 $r1, [$r6 + #0x18]
L_130b8: divsr	$r0, $r0, $r3, $r0
L_130bc: add_slli	$r5, $r1, $r5, #2
L_130c0: lwi450 $r1, [$r5]
L_130c2: maddr32	$r1, $r13, $r0
L_130c6: swi450 $r1, [$r5]
L_130c8: lwi37.sp $r1, [ + #0x38]
L_130ca: lwi333 $r3, [$r6 + #0x1c]
L_130cc: ex9.it #361
L_130ce: lwi450 $r3, [$r1]
L_130d0: maddr32	$r3, $r9, $r0
L_130d4: swi450 $r3, [$r1]
L_130d6: lwi37.sp $r1, [ + #0x38]
L_130d8: lwi	$r3, [$r6 + #0x20]
L_130dc: ex9.it #361
L_130de: lwi450 $r3, [$r1]
L_130e0: add45 $r0, $r3
L_130e2: swi450 $r0, [$r1]
L_130e4: lwi37.sp $r0, [ + #0x3c]
L_130e6: lh	$r5, [$r2 + ($r0 << #1)]
L_130ea: bnes38 $r0, . + (66)
L_130ec: lb	$r0, [$r24 + ($r5 << #0)]
L_130f0: bnec	$r0, #1, . + (60)
L_130f4: lwi37.sp $r1, [ + #0x34]
L_130f6: lwi37.sp $r0, [ + #0x30]
L_130f8: mul33 $r7, $r1
L_130fa: add45 $r0, $r1
L_130fc: divsr	$r7, $r7, $r7, $r0
L_13100: lwi333 $r0, [$r6 + #0x18]
L_13102: add_slli	$r5, $r0, $r5, #2
L_13106: lwi450 $r0, [$r5]
L_13108: maddr32	$r0, $r13, $r7
L_1310c: swi450 $r0, [$r5]
L_1310e: ifret16
L_13110: lwi37.sp $r0, [ + #0x3c]
L_13112: lwi333 $r1, [$r6 + #0x1c]
L_13114: ex9.it #77
L_13116: lwi450 $r1, [$r0]
L_13118: ifret16
L_1311a: maddr32	$r1, $r9, $r7
L_1311e: swi450 $r1, [$r0]
L_13120: lwi37.sp $r0, [ + #0x3c]
L_13122: ex9.it #304
L_13124: ifcall	. + (-16)
L_13128: add45 $r7, $r1
L_1312a: swi450 $r7, [$r0]
L_1312c: lwi450 $r0, [$r6]
L_1312e: addi	$r13, $r13, #1
L_13132: add45 $r18, $r0
L_13134: lbi	$r0, [$r18 + #1]
L_13138: or	$r0, $r0, $r25
L_1313c: sbi	$r0, [$r18 + #1]
L_13140: j	. + (-314)
L_13144: addi45 $r9, #1
L_13146: add	$r14, $r14, $r19
L_1314a: j	. + (-344)
L_1314e: lwi37.sp $r0, [ + #0xc]
L_13150: lwi37.sp $r1, [ + #4]
L_13152: add_slli	$r14, $r21, $r0, #3
L_13156: lwi333 $r0, [$r6 + #0x18]
L_13158: ifcall9 . + (10)
L_1315a: shi	$r0, [$r14 + #0]
L_1315e: lwi333 $r0, [$r6 + #0x1c]
L_13160: lwi37.sp $r1, [ + #8]
L_13162: lw	$r0, [$r0 + ($r16 << #0)]
L_13166: mul33 $r0, $r1
L_13168: ex9.it #304
L_1316a: lw	$r1, [$r1 + ($r16 << #0)]
L_1316e: ex9.it #78
L_13170: ifret16
L_13172: shi	$r0, [$r14 + #2]
L_13176: addi45 $r8, #1
L_13178: j	. + (-512)
L_1317c: pop25 $r14, #144
L_1317e: swi.gp	$r0, [ + #-4036]
L_13182: ret5 $lp
L_13184: movi	$r0, #0x2b64c
L_13188: swi.gp	$r0, [ + #-4040]
L_1318c: ret5 $lp
L_1318e: swi.gp	$r0, [ + #-1760]
L_13192: ret5 $lp
L_13194: swi.gp	$r0, [ + #-1764]
L_13198: ret5 $lp
L_1319a: lhsi	$r2, [$r0 + #0]
L_1319e: ex9.it #184
L_131a0: lhsi	$r0, [$r0 + #2]
L_131a4: ex9.it #76
L_131a6: sub45 $r2, $r3
L_131a8: sub45 $r0, $r1
L_131aa: mul33 $r0, $r0
L_131ac: maddr32	$r0, $r2, $r2
L_131b0: ret5 $lp
L_131b2: mov55 $r3, $r0
L_131b4: mov55 $r2, $r0
L_131b6: movi55 $r0, #1
L_131b8: sub333 $r1, $r2, $r0
L_131ba: slti45 $r1, #2
L_131bc: bnezs8 . + (20)
L_131be: add333 $r1, $r2, $r0
L_131c0: srli45 $r1, #1
L_131c2: mul	$r15, $r1, $r1
L_131c6: slt45 $r3, $r15
L_131c8: ex9.it #359
L_131ca: cmovn	$r2, $r1, $r15
L_131ce: j8 . + (-22)
L_131d0: ret5 $lp
L_131d2: lwi.gp	$r4, [ + #-1764]
L_131d6: movi55 $r1, #0xa
L_131d8: lhi	$r3, [$r4 + #0x12]
L_131dc: lhsi	$r2, [$r0 + #0]
L_131e0: mul33 $r1, $r3
L_131e2: subi333 $r3, $r1, #1
L_131e4: srai45 $r3, #1
L_131e6: slts45 $r3, $r2
L_131e8: beqzs8 . + (6)
L_131ea: sub45 $r1, $r2
L_131ec: seh33 $r2, $r1
L_131ee: lhi	$r3, [$r4 + #0x14]
L_131f2: lhsi	$r1, [$r0 + #2]
L_131f6: movi55 $r0, #0xa
L_131f8: mul33 $r0, $r3
L_131fa: subi333 $r3, $r0, #1
L_131fc: srai45 $r3, #1
L_131fe: slts45 $r3, $r1
L_13200: beqzs8 . + (6)
L_13202: sub45 $r0, $r1
L_13204: seh33 $r1, $r0
L_13206: min	$r1, $r1, $r2
L_1320a: movi55 $r0, #0xa
L_1320c: max	$r0, $r1, $r0
L_13210: ret5 $lp
L_13212: push25 $r14, #0
L_13214: mov55 $r10, $r1
L_13216: ex9.it #189
L_13218: maddr32	$r0, $r2, $r1
L_1321c: mov55 $r6, $r2
L_1321e: lwi450 $r1, [$r0]
L_13220: movi	$r12, #0x2d2e8
L_13224: movi	$r11, #0x2d2dc
L_13228: slli333 $r7, $r2, #2
L_1322a: movi	$r9, #0x2d2ac
L_1322e: beqz38 $r1, . + (26)
L_13230: lbi	$r0, [$r0 + #0x18]
L_13234: xlsb33 $r0, $r0
L_13236: beqz38 $r0, . + (18)
L_13238: movi55 $r0, #3
L_1323a: sb	$r0, [$r12 + ($r2 << #0)]
L_1323e: movi55 $r0, #0
L_13240: sb	$r0, [$r11 + ($r2 << #0)]
L_13244: movi55 $r0, #0
L_13246: j8 . + (110)
L_13248: lb	$r0, [$r11 + ($r6 << #0)]
L_1324c: slti45 $r0, #4
L_1324e: beqzs8 . + (12)
L_13250: addi45 $r0, #1
L_13252: movi	$r1, #0x2d2dc
L_13256: sb	$r0, [$r1 + ($r6 << #0)]
L_1325a: lb	$r0, [$r12 + ($r6 << #0)]
L_1325e: movi	$r1, #0x2d2e8
L_13262: slti45 $r0, #3
L_13264: bnezs8 . + (6)
L_13266: movi55 $r0, #0
L_13268: j8 . + (4)
L_1326a: addi45 $r0, #1
L_1326c: sb	$r0, [$r1 + ($r6 << #0)]
L_13270: mov55 $r1, $r10
L_13272: add	$r0, $r9, $r7
L_13276: ex9.it #494
L_13278: ex9.it #140
L_1327a: mul33 $r0, $r1
L_1327c: ex9.it #331
L_1327e: movi	$r2, #0x10000
L_13282: lb	$r1, [$r12 + ($r6 << #0)]
L_13286: slt45 $r2, $r0
L_13288: cmovn	$r0, $r2, $r15
L_1328c: add45 $r1, $r7
L_1328e: movi	$r4, #0x2d24c
L_13292: movi55 $r3, #0
L_13294: sh	$r0, [$r4 + ($r1 << #1)]
L_13298: mov55 $r2, $r3
L_1329a: lb	$r0, [$r11 + ($r6 << #0)]
L_1329e: slt45 $r3, $r0
L_132a0: beqzs8 . + (14)
L_132a2: add333 $r1, $r3, $r7
L_132a4: lh	$r1, [$r4 + ($r1 << #1)]
L_132a8: addi45 $r3, #1
L_132aa: add45 $r2, $r1
L_132ac: j8 . + (-14)
L_132ae: add_srli	$r2, $r2, $r0, #1
L_132b2: ex9.it #411
L_132b4: lhsi	$r1, [$r10 + #0]
L_132b8: sh	$r1, [$r9 + ($r6 << #2)]
L_132bc: add45 $r7, $r9
L_132be: lhsi	$r1, [$r10 + #2]
L_132c2: shi333 $r1, [$r7 + #2]
L_132c4: pop25 $r14, #0
L_132c6: movi	$r0, #0x2b5e4
L_132ca: swi.gp	$r0, [ + #-1768]
L_132ce: addi	$r0, $r0, #0x34
L_132d2: swi.gp	$r0, [ + #-1772]
L_132d6: ret5 $lp
L_132d8: push25 $r8, #0
L_132da: addi	$sp, $sp, #-744
L_132de: mov55 $r6, $r0
L_132e0: shi	$r1, [$sp + #6]
L_132e4: mov55 $r7, $r2
L_132e6: movi	$r1, #0x196dc
L_132ea: ex9.it #492
L_132ec: addri36.sp $r0, #8
L_132ee: ex9.it #8
L_132f0: movi	$r1, #0x19794
L_132f4: ex9.it #492
L_132f6: addri36.sp $r0, #0xc0
L_132f8: ex9.it #8
L_132fa: movi	$r1, #0x1984c
L_132fe: ex9.it #492
L_13300: addi	$r0, $sp, #0x178
L_13304: ex9.it #8
L_13306: addi	$r0, $sp, #0x230
L_1330a: movi	$r1, #0x19904
L_1330e: ex9.it #492
L_13310: ex9.it #8
L_13312: beqc	$r6, #2, . + (18)
L_13316: beqc	$r6, #3, . + (20)
L_1331a: bnec	$r6, #1, . + (22)
L_1331e: addri36.sp $r3, #0xc0
L_13320: movi55 $r6, #0
L_13322: j8 . + (18)
L_13324: addi	$r3, $sp, #0x178
L_13328: j8 . + (10)
L_1332a: addi	$r3, $sp, #0x230
L_1332e: j8 . + (4)
L_13330: addri36.sp $r3, #8
L_13332: movi55 $r6, #1
L_13334: beqz38 $r7, . + (68)
L_13336: movpi45 $r2, #0x2d
L_13338: movi	$r5, #0x5a
L_1333c: movi55 $r4, #0
L_1333e: movi55 $r7, #2
L_13340: ex9.it #101
L_13342: lhs	$r1, [$r3 + ($r2 << #1)]
L_13346: seh33 $r0, $r0
L_13348: slts45 $r0, $r1
L_1334a: beqzs8 . + (6)
L_1334c: beqz38 $r6, . + (14)
L_1334e: j8 . + (22)
L_13350: ex9.it #101
L_13352: seh33 $r0, $r0
L_13354: slts45 $r1, $r0
L_13356: beqzs8 . + (44)
L_13358: beqz38 $r6, . + (12)
L_1335a: add333 $r1, $r2, $r5
L_1335c: ex9.it #232
L_1335e: mov55 $r4, $r2
L_13360: seh33 $r0, $r1
L_13362: j8 . + (10)
L_13364: add333 $r1, $r4, $r2
L_13366: ex9.it #232
L_13368: mov55 $r5, $r2
L_1336a: seh33 $r0, $r1
L_1336c: slts45 $r0, $r5
L_1336e: beqzs8 . + (22)
L_13370: slts45 $r4, $r0
L_13372: beqzs8 . + (18)
L_13374: mov55 $r2, $r0
L_13376: j8 . + (-54)
L_13378: ex9.it #101
L_1337a: seh33 $r0, $r0
L_1337c: lhs	$r0, [$r3 + ($r0 << #1)]
L_13380: j8 . + (4)
L_13382: mov55 $r0, $r2
L_13384: addi	$sp, $sp, #0x2e8
L_13388: pop25 $r8, #0
L_1338a: lwi.gp	$r1, [ + #0xe7c]
L_1338e: bnec	$r1, #2, . + (44)
L_13392: lbi	$r1, [$r0 + #0x3c]
L_13396: bnec	$r1, #1, . + (36)
L_1339a: movi55 $r1, #0
L_1339c: ex9.it #396
L_1339e: lhi	$r1, [$r0 + #0x30]
L_133a2: shi	$r1, [$r0 + #0x34]
L_133a6: lhi	$r1, [$r0 + #0x32]
L_133aa: shi	$r1, [$r0 + #0x36]
L_133ae: lhi	$r1, [$r0 + #0x1c]
L_133b2: ex9.it #419
L_133b4: lhi	$r1, [$r0 + #0x1e]
L_133b8: j8 . + (68)
L_133ba: lhsi	$r1, [$r0 + #0x34]
L_133be: movpi45 $r4, #0x13
L_133c0: lhsi	$r2, [$r0 + #0x30]
L_133c4: ex9.it #511
L_133c6: movpi45 $r3, #0x14
L_133c8: lhsi	$r1, [$r0 + #0x36]
L_133cc: ex9.it #172
L_133ce: shi	$r2, [$r0 + #0x34]
L_133d2: lhsi	$r2, [$r0 + #0x32]
L_133d6: ex9.it #511
L_133d8: lhsi	$r1, [$r0 + #0x20]
L_133dc: ex9.it #172
L_133de: shi	$r2, [$r0 + #0x36]
L_133e2: lhsi	$r2, [$r0 + #0x1c]
L_133e6: ex9.it #511
L_133e8: lhsi	$r1, [$r0 + #0x1e]
L_133ec: ex9.it #172
L_133ee: shi	$r2, [$r0 + #0x20]
L_133f2: lhsi	$r2, [$r0 + #0x22]
L_133f6: maddr32	$r1, $r2, $r4
L_133fa: ex9.it #279
L_133fc: shi	$r1, [$r0 + #0x22]
L_13400: lbi	$r1, [$r0 + #0x7a]
L_13404: bnez38 $r1, . + (16)
L_13406: lhi	$r1, [$r0 + #0x34]
L_1340a: ex9.it #419
L_1340c: lhi	$r1, [$r0 + #0x36]
L_13410: shi	$r1, [$r0 + #0x22]
L_13414: ret5 $lp
L_13416: push25 $r8, #8
L_13418: lhi	$r1, [$r0 + #0x20]
L_1341c: lhi	$r2, [$r0 + #0x34]
L_13420: lhi	$r3, [$r0 + #0x36]
L_13424: sub333 $r2, $r1, $r2
L_13426: lhi	$r1, [$r0 + #0x22]
L_1342a: lhsi	$r4, [$r0 + #0x10]
L_1342e: sub45 $r1, $r3
L_13430: seh33 $r2, $r2
L_13432: seh33 $r1, $r1
L_13434: sltsi	$r15, $r4, #0x3c
L_13438: srai45 $r2, #1
L_1343a: srai45 $r1, #1
L_1343c: beqzs8 . + (8)
L_1343e: bgez	$r2, . + (16)
L_13442: j8 . + (26)
L_13444: sltsi	$r15, $r4, #0xc44
L_13448: bnezs8 . + (6)
L_1344a: bgtz	$r2, . + (18)
L_1344e: lbi	$r3, [$r0 + #0x78]
L_13452: slti45 $r3, #0x14
L_13454: bnezs8 . + (8)
L_13456: movi55 $r3, #9
L_13458: ex9.it #172
L_1345a: j8 . + (4)
L_1345c: movi55 $r2, #0
L_1345e: lhsi	$r3, [$r0 + #0x12]
L_13462: shi	$r2, [$sp + #0]
L_13466: sltsi	$r15, $r3, #0x3c
L_1346a: beqzs8 . + (8)
L_1346c: bgez	$r1, . + (16)
L_13470: j8 . + (28)
L_13472: sltsi	$r15, $r3, #0x13c4
L_13476: bnezs8 . + (6)
L_13478: bgtz	$r1, . + (20)
L_1347c: lbi	$r2, [$r0 + #0x78]
L_13480: slti45 $r2, #0x14
L_13482: bnezs8 . + (10)
L_13484: movi55 $r2, #9
L_13486: divsr	$r1, $r1, $r1, $r2
L_1348a: j8 . + (4)
L_1348c: movi55 $r1, #0
L_1348e: shi	$r1, [$sp + #2]
L_13492: lwi.gp	$r1, [ + #-5288]
L_13496: slti	$r15, $r1, #0x320
L_1349a: beqzs8 . + (12)
L_1349c: movi55 $r1, #0
L_1349e: shi	$r1, [$sp + #0]
L_134a2: shi	$r1, [$sp + #2]
L_134a6: lwi	$r1, [$r0 + #0x44]
L_134aa: ex9.it #333
L_134ac: ex9.it #248
L_134ae: and33 $r5, $r1
L_134b0: movi	$r1, #0x10000
L_134b4: beqs38 $r1, . + (72)
L_134b6: lwi.gp	$r1, [ + #0x1324]
L_134ba: lhsi.gp	$r7, [ + #-1956]
L_134be: slti45 $r1, #6
L_134c0: lhsi.gp	$r1, [ + #-1954]
L_134c4: bnezs8 . + (12)
L_134c6: shi	$r1, [$sp + #0]
L_134ca: shi	$r7, [$sp + #2]
L_134ce: j8 . + (46)
L_134d0: lhi	$r2, [$sp + #0]
L_134d4: movi55 $r6, #7
L_134d6: seh33 $r2, $r2
L_134d8: maddr32	$r2, $r1, $r6
L_134dc: movi55 $r5, #8
L_134de: divsr	$r2, $r2, $r2, $r5
L_134e2: seh33 $r2, $r2
L_134e4: shi	$r2, [$sp + #0]
L_134e8: lhi	$r1, [$sp + #2]
L_134ec: seh33 $r1, $r1
L_134ee: maddr32	$r1, $r7, $r6
L_134f2: divsr	$r1, $r1, $r1, $r5
L_134f6: seh33 $r1, $r1
L_134f8: shi	$r1, [$sp + #2]
L_134fc: lhi	$r2, [$sp + #0]
L_13500: movi55 $r1, #-2
L_13502: maddr32	$r4, $r2, $r1
L_13506: seh33 $r4, $r4
L_13508: shi	$r4, [$sp + #4]
L_1350c: lhi	$r2, [$sp + #2]
L_13510: maddr32	$r3, $r2, $r1
L_13514: seh33 $r3, $r3
L_13516: shi	$r3, [$sp + #6]
L_1351a: lhi	$r1, [$sp + #4]
L_1351e: shi	$r1, [$r0 + #0x14]
L_13522: lhi	$r1, [$sp + #6]
L_13526: shi	$r1, [$r0 + #0x16]
L_1352a: lhi	$r0, [$sp + #0]
L_1352e: shi.gp	$r0, [ + #-1954]
L_13532: ex9.it #483
L_13534: shi.gp	$r0, [ + #-1956]
L_13538: pop25 $r8, #8
L_1353a: push25 $r14, #8
L_1353c: swi37.sp $r5, [ + #4]
L_1353e: seh	$r19, $r5
L_13542: srai	$r18, $r5, #0x10
L_13546: lhsi	$r7, [$r1 + #2]
L_1354a: ex9.it #457
L_1354c: ex9.it #189
L_1354e: maddr32	$r0, $r3, $r1
L_13552: lbi	$r9, [$sp + #0x38]
L_13556: lbi	$r1, [$r0 + #0x18]
L_1355a: movi55 $r10, #1
L_1355c: andi	$r15, $r1, #1
L_13560: movi	$r6, #0x2d1d0
L_13564: movi	$r1, #0x2d1a0
L_13568: beqzs8 . + (34)
L_1356a: movi	$r16, #0x2d224
L_1356e: sh	$r5, [$r16 + ($r3 << #1)]
L_13572: ex9.it #283
L_13574: movi	$r16, #0x2d20c
L_13578: movi	$r5, #0x2d200
L_1357c: sh	$r7, [$r16 + ($r3 << #1)]
L_13580: sw	$r7, [$r1 + ($r3 << #2)]
L_13584: sb	$r10, [$r5 + ($r3 << #0)]
L_13588: j8 . + (188)
L_1358a: lwi450 $r16, [$r0]
L_1358c: bnec	$r16, #3, . + (62)
L_13590: lw	$r16, [$r6 + ($r3 << #2)]
L_13594: movi	$r17, #0x2d200
L_13598: add45 $r5, $r16
L_1359a: lw	$r16, [$r1 + ($r3 << #2)]
L_1359e: ex9.it #283
L_135a0: add45 $r16, $r7
L_135a2: lb	$r7, [$r17 + ($r3 << #0)]
L_135a6: sw	$r16, [$r1 + ($r3 << #2)]
L_135aa: addi45 $r7, #1
L_135ac: zeb33 $r7, $r7
L_135ae: slti45 $r7, #0xf
L_135b0: sb	$r7, [$r17 + ($r3 << #0)]
L_135b4: bnezs8 . + (144)
L_135b6: divsr	$r5, $r5, $r5, $r7
L_135ba: divsr	$r7, $r7, $r16, $r7
L_135be: ex9.it #283
L_135c0: sw	$r7, [$r1 + ($r3 << #2)]
L_135c4: sb	$r10, [$r17 + ($r3 << #0)]
L_135c8: j8 . + (124)
L_135ca: bnec	$r16, #1, . + (116)
L_135ce: movi	$r16, #0x2d224
L_135d2: lwi.gp	$r17, [ + #-1760]
L_135d6: lhs	$r11, [$r16 + ($r3 << #1)]
L_135da: movi	$r16, #0x2d20c
L_135de: lhs	$r10, [$r16 + ($r3 << #1)]
L_135e2: lbi	$r16, [$r17 + #0]
L_135e6: bnec	$r16, #1, . + (212)
L_135ea: sub	$r15, $r11, $r5
L_135ee: sub	$r16, $r10, $r7
L_135f2: abs	$r15, $r15
L_135f6: abs	$r16, $r16
L_135fa: lhi	$r8, [$r17 + #8]
L_135fe: add45 $r16, $r15
L_13600: slt45 $r8, $r16
L_13602: beqzs8 . + (184)
L_13604: lhi	$r17, [$r17 + #6]
L_13608: slts45 $r11, $r17
L_1360a: beqzs8 . + (6)
L_1360c: slts45 $r5, $r17
L_1360e: beqzs8 . + (212)
L_13610: lwi.gp	$r8, [ + #-1764]
L_13614: lhi	$r16, [$r8 + #0x12]
L_13618: subi45 $r16, #1
L_1361a: sub45 $r16, $r17
L_1361c: slts45 $r16, $r11
L_1361e: beqzs8 . + (6)
L_13620: slts45 $r16, $r5
L_13622: beqzs8 . + (192)
L_13624: slts45 $r10, $r17
L_13626: beqzs8 . + (6)
L_13628: slts45 $r7, $r17
L_1362a: beqzs8 . + (184)
L_1362c: lhi	$r16, [$r8 + #0x14]
L_13630: subi45 $r16, #1
L_13632: sub45 $r16, $r17
L_13634: slts45 $r16, $r10
L_13636: beqzs8 . + (132)
L_13638: slts45 $r16, $r7
L_1363a: beqzs8 . + (168)
L_1363c: j8 . + (126)
L_1363e: ex9.it #283
L_13640: sw	$r7, [$r1 + ($r3 << #2)]
L_13644: bnec	$r9, #1, . + (104)
L_13648: lwi450 $r0, [$r0]
L_1364a: bnec	$r0, #1, . + (98)
L_1364e: lwi.gp	$r16, [ + #-1760]
L_13652: lhi	$r0, [$r16 + #0xa]
L_13656: slt45 $r4, $r0
L_13658: beqzs8 . + (84)
L_1365a: lw	$r0, [$r6 + ($r3 << #2)]
L_1365e: lw	$r7, [$r1 + ($r3 << #2)]
L_13662: sub	$r5, $r0, $r19
L_13666: abs	$r15, $r5
L_1366a: sub	$r5, $r7, $r18
L_1366e: abs	$r5, $r5
L_13672: lhi	$r4, [$r16 + #0xc]
L_13676: add45 $r5, $r15
L_13678: slts45 $r5, $r4
L_1367a: beqzs8 . + (50)
L_1367c: lhi	$r5, [$r16 + #0xe]
L_13680: movi	$r17, #0x2d1d0
L_13684: addi	$r16, $r5, #-1
L_13688: maddr32	$r0, $r16, $r19
L_1368c: srli	$r4, $r5, #1
L_13690: maddr32	$r7, $r16, $r18
L_13694: add45 $r0, $r4
L_13696: divsr	$r0, $r0, $r0, $r5
L_1369a: add45 $r4, $r7
L_1369c: sw	$r0, [$r17 + ($r3 << #2)]
L_136a0: divsr	$r4, $r4, $r4, $r5
L_136a4: movi	$r0, #0x2d1a0
L_136a8: sw	$r4, [$r0 + ($r3 << #2)]
L_136ac: lw	$r0, [$r6 + ($r3 << #2)]
L_136b0: shi333 $r0, [$r2 + #0]
L_136b2: lw	$r0, [$r1 + ($r3 << #2)]
L_136b6: shi333 $r0, [$r2 + #2]
L_136b8: pop25 $r14, #8
L_136ba: lw	$r16, [$r6 + ($r3 << #2)]
L_136be: movi	$r17, #0x2d200
L_136c2: add45 $r5, $r16
L_136c4: lw	$r16, [$r1 + ($r3 << #2)]
L_136c8: add45 $r16, $r7
L_136ca: lb	$r7, [$r17 + ($r3 << #0)]
L_136ce: addi45 $r7, #1
L_136d0: zeb33 $r7, $r7
L_136d2: divsr	$r5, $r5, $r5, $r7
L_136d6: sb	$r7, [$r17 + ($r3 << #0)]
L_136da: ex9.it #283
L_136dc: divsr	$r7, $r7, $r16, $r7
L_136e0: j8 . + (-160)
L_136e2: sw	$r11, [$r6 + ($r3 << #2)]
L_136e6: sw	$r10, [$r1 + ($r3 << #2)]
L_136ea: j8 . + (-166)
L_136ec: push25 $r14, #0
L_136ee: movi	$r6, #0x4c
L_136f2: maddr32	$r0, $r3, $r6
L_136f6: lwi.gp	$r10, [ + #-1760]
L_136fa: lwi450 $r0, [$r0]
L_136fc: ex9.it #457
L_136fe: ex9.it #185
L_13700: ex9.it #76
L_13702: lbi	$r16, [$r10 + #0x16]
L_13706: lhi.gp	$r9, [ + #-6364]
L_1370a: movi	$r17, #0x2d194
L_1370e: movi	$r18, #0x2d188
L_13712: slli333 $r7, $r3, #4
L_13714: movi	$r6, #0x2ce88
L_13718: movi	$r19, #0x2ce7c
L_1371c: bnec	$r0, #1, . + (32)
L_13720: movi55 $r4, #0
L_13722: sh	$r5, [$r6 + ($r7 << #2)]
L_13726: add_slli	$r6, $r6, $r3, #6
L_1372a: sb	$r4, [$r17 + ($r3 << #0)]
L_1372e: sb	$r0, [$r18 + ($r3 << #0)]
L_13732: shi333 $r1, [$r6 + #2]
L_13734: sb	$r16, [$r19 + ($r3 << #0)]
L_13738: j	. + (292)
L_1373c: lb	$r0, [$r18 + ($r3 << #0)]
L_13740: slti45 $r0, #0x10
L_13742: beqzs8 . + (8)
L_13744: addi45 $r0, #1
L_13746: sb	$r0, [$r18 + ($r3 << #0)]
L_1374a: lb	$r0, [$r17 + ($r3 << #0)]
L_1374e: movi	$r11, #0x2d194
L_13752: slti45 $r0, #0xf
L_13754: bnezs8 . + (6)
L_13756: movi55 $r0, #0
L_13758: j8 . + (4)
L_1375a: addi45 $r0, #1
L_1375c: sb	$r0, [$r11 + ($r3 << #0)]
L_13760: lb	$r17, [$r17 + ($r3 << #0)]
L_13764: add	$r0, $r17, $r7
L_13768: sh	$r5, [$r6 + ($r0 << #2)]
L_1376c: add_slli	$r0, $r6, $r0, #2
L_13770: shi333 $r1, [$r0 + #2]
L_13772: lhi	$r1, [$r10 + #0x1a]
L_13776: slts45 $r4, $r1
L_13778: beqzs8 . + (32)
L_1377a: lhi	$r5, [$r10 + #0x18]
L_1377e: slts45 $r5, $r4
L_13780: beqzs8 . + (22)
L_13782: sub	$r0, $r16, $r9
L_13786: sub45 $r4, $r5
L_13788: mul33 $r0, $r4
L_1378a: sub	$r16, $r1, $r5
L_1378e: divsr	$r16, $r16, $r0, $r16
L_13792: add45 $r16, $r9
L_13794: j8 . + (4)
L_13796: mov55 $r16, $r9
L_13798: lb	$r4, [$r19 + ($r3 << #0)]
L_1379c: movi55 $r0, #3
L_1379e: mov55 $r1, $r4
L_137a0: maddr32	$r1, $r16, $r0
L_137a4: movi55 $r0, #4
L_137a6: addi45 $r1, #2
L_137a8: ex9.it #194
L_137aa: slts45 $r4, $r0
L_137ac: movi	$r5, #0x2ce7c
L_137b0: beqzs8 . + (10)
L_137b2: addi45 $r4, #1
L_137b4: slts45 $r4, $r0
L_137b6: cmovn	$r0, $r4, $r15
L_137ba: sb	$r0, [$r5 + ($r3 << #0)]
L_137be: lb	$r1, [$r18 + ($r3 << #0)]
L_137c2: lb	$r4, [$r19 + ($r3 << #0)]
L_137c6: movi55 $r3, #0xa
L_137c8: mul33 $r1, $r3
L_137ca: slts45 $r1, $r4
L_137cc: bnezs8 . + (10)
L_137ce: slt45 $r4, $r3
L_137d0: mov55 $r1, $r4
L_137d2: cmovn	$r1, $r3, $r15
L_137d6: movi55 $r5, #0
L_137d8: movi55 $r16, #0xa
L_137da: divsr	$r16, $r16, $r1, $r16
L_137de: mov55 $r0, $r5
L_137e0: mov55 $r3, $r5
L_137e2: beq	$r5, $r16, . + (38)
L_137e6: add	$r4, $r17, $r7
L_137ea: lhs	$r18, [$r6 + ($r4 << #2)]
L_137ee: add_slli	$r4, $r6, $r4, #2
L_137f2: lhsi	$r4, [$r4 + #2]
L_137f6: add45 $r3, $r18
L_137f8: add45 $r0, $r4
L_137fa: beqz	$r17, . + (8)
L_137fe: subi45 $r17, #1
L_13800: j8 . + (4)
L_13802: movi55 $r17, #0xf
L_13804: addi45 $r5, #1
L_13806: j8 . + (-36)
L_13808: movi55 $r4, #0xa
L_1380a: mul33 $r3, $r4
L_1380c: mul33 $r0, $r4
L_1380e: divsr	$r4, $r5, $r1, $r4
L_13812: beqz38 $r5, . + (24)
L_13814: add45 $r7, $r17
L_13816: lhs	$r4, [$r6 + ($r7 << #2)]
L_1381a: add_slli	$r7, $r6, $r7, #2
L_1381e: maddr32	$r3, $r5, $r4
L_13822: lhsi	$r4, [$r7 + #2]
L_13826: maddr32	$r0, $r5, $r4
L_1382a: bltz	$r3, . + (12)
L_1382e: srai	$r5, $r1, #1
L_13832: add45 $r5, $r3
L_13834: j8 . + (10)
L_13836: movi55 $r5, #2
L_13838: divsr	$r5, $r5, $r1, $r5
L_1383c: sub333 $r5, $r3, $r5
L_1383e: divsr	$r5, $r5, $r5, $r1
L_13842: seh33 $r5, $r5
L_13844: bltz	$r0, . + (12)
L_13848: srai	$r3, $r1, #1
L_1384c: add45 $r0, $r3
L_1384e: j8 . + (10)
L_13850: movi55 $r3, #2
L_13852: divsr	$r3, $r3, $r1, $r3
L_13856: sub45 $r0, $r3
L_13858: ex9.it #204
L_1385a: seh33 $r1, $r1
L_1385c: shi333 $r5, [$r2 + #0]
L_1385e: shi333 $r1, [$r2 + #2]
L_13860: pop25 $r14, #0
L_13862: push25 $r8, #0
L_13864: movi	$r6, #0x4c
L_13868: maddr32	$r0, $r3, $r6
L_1386c: ex9.it #457
L_1386e: lwi450 $r0, [$r0]
L_13870: ex9.it #76
L_13872: ex9.it #185
L_13874: movi	$r6, #0x2ce4c
L_13878: slli333 $r7, $r3, #2
L_1387a: movi	$r16, #0x2ce40
L_1387e: bnec	$r0, #1, . + (20)
L_13882: add333 $r0, $r6, $r7
L_13884: shi333 $r1, [$r0 + #2]
L_13886: ex9.it #287
L_13888: sh	$r5, [$r6 + ($r3 << #2)]
L_1388c: sb	$r0, [$r16 + ($r3 << #0)]
L_13890: j8 . + (150)
L_13892: sltsi	$r15, $r4, #0x29
L_13896: beqzs8 . + (8)
L_13898: lhi.gp	$r4, [ + #-6366]
L_1389c: j8 . + (38)
L_1389e: sltsi	$r15, $r4, #0x12c
L_138a2: beqzs8 . + (30)
L_138a4: lhi.gp	$r0, [ + #-6366]
L_138a8: addi	$r4, $r4, #-40
L_138ac: subri	$r17, $r0, #0x64
L_138b0: mul	$r4, $r17, $r4
L_138b4: movi	$r17, #0x104
L_138b8: divsr	$r4, $r4, $r4, $r17
L_138bc: add45 $r4, $r0
L_138be: j8 . + (4)
L_138c0: ex9.it #389
L_138c2: lb	$r0, [$r16 + ($r3 << #0)]
L_138c6: movi55 $r17, #3
L_138c8: maddr32	$r0, $r4, $r17
L_138cc: movi55 $r4, #4
L_138ce: addi45 $r0, #2
L_138d0: divsr	$r0, $r0, $r0, $r4
L_138d4: sb	$r0, [$r16 + ($r3 << #0)]
L_138d8: add333 $r4, $r6, $r7
L_138da: zeb33 $r0, $r0
L_138dc: lhs	$r18, [$r6 + ($r3 << #2)]
L_138e0: lhsi	$r17, [$r4 + #2]
L_138e4: subri	$r16, $r0, #0x64
L_138e8: mul	$r4, $r0, $r5
L_138ec: maddr32	$r4, $r16, $r18
L_138f0: movi	$r5, #0x64
L_138f4: bltz	$r4, . + (10)
L_138f8: addi	$r4, $r4, #0x32
L_138fc: j8 . + (6)
L_138fe: addi	$r4, $r4, #-50
L_13902: mul33 $r0, $r1
L_13904: maddr32	$r0, $r16, $r17
L_13908: divsr	$r4, $r4, $r4, $r5
L_1390c: sh	$r4, [$r6 + ($r3 << #2)]
L_13910: ex9.it #140
L_13912: bltz	$r0, . + (10)
L_13916: addi	$r0, $r0, #0x32
L_1391a: j8 . + (6)
L_1391c: addi	$r0, $r0, #-50
L_13920: ex9.it #78
L_13922: add333 $r1, $r6, $r7
L_13924: shi333 $r0, [$r1 + #2]
L_13926: lh	$r0, [$r6 + ($r3 << #2)]
L_1392a: add45 $r6, $r7
L_1392c: shi333 $r0, [$r2 + #0]
L_1392e: lhi333 $r0, [$r6 + #2]
L_13930: shi333 $r0, [$r2 + #2]
L_13932: pop25 $r8, #0
L_13934: push25 $r14, #8
L_13936: ex9.it #503
L_13938: swi37.sp $r0, [ + #4]
L_1393a: ex9.it #222
L_1393c: lwi	$r8, [$sp + #0x38]
L_13940: slt45 $r1, $r0
L_13942: lwi37.sp $r6, [ + #0x3c]
L_13944: beqzs8 . + (8)
L_13946: movi55 $r0, #4
L_13948: sbi.gp	$r0, [ + #-3524]
L_1394c: lbi.gp	$r0, [ + #-3524]
L_13950: beqz38 $r0, . + (8)
L_13952: subi45 $r0, #1
L_13954: sbi.gp	$r0, [ + #-3524]
L_13958: ex9.it #221
L_1395a: maddr32	$r2, $r6, $r0
L_1395e: lbi	$r0, [$r2 + #0x18]
L_13962: mov55 $r7, $r2
L_13964: bmski33 $r0, #1
L_13966: bnez38 $r0, . + (174)
L_13968: lwi450 $r0, [$r2]
L_1396a: beqc	$r0, #4, . + (170)
L_1396e: add_slli	$r0, $r3, $r6, #4
L_13972: addi45 $r0, #0xc
L_13974: add_slli	$r9, $r3, $r6, #4
L_13978: lwi	$r1, [$r0 + #-4]
L_1397c: swi.bi	$r1, [$r0], #-4
L_13980: bne	$r9, $r0, . + (-8)
L_13984: lwi333 $r0, [$r7 + #0x1c]
L_13986: lwi.gp	$r1, [ + #0x310]
L_1398a: movi	$r10, #0x2f2b4
L_1398e: lh	$r1, [$r1 + ($r0 << #1)]
L_13992: lh	$r2, [$r10 + ($r6 << #1)]
L_13996: slt45 $r2, $r1
L_13998: beqzs8 . + (6)
L_1399a: sh	$r1, [$r10 + ($r6 << #1)]
L_1399e: lwi.gp	$r1, [ + #0x328]
L_139a2: lw	$r12, [$r1 + ($r0 << #2)]
L_139a6: swi	$r12, [$r9 + #0]
L_139aa: lbi	$r0, [$r7 + #0x43]
L_139ae: beqc	$r0, #9, . + (102)
L_139b2: sltsi	$r15, $r5, #0x1f4
L_139b6: beqzs8 . + (94)
L_139b8: addri36.sp $r0, #4
L_139ba: mov55 $r11, $r4
L_139bc: jal	. + (-2026)
L_139c0: sltsi	$r15, $r0, #0x1f5
L_139c4: bnezs8 . + (80)
L_139c6: movi55 $r0, #0xa
L_139c8: mul	$r1, $r12, $r0
L_139cc: lwi	$r0, [$r9 + #0xc]
L_139d0: movi55 $r2, #6
L_139d2: mul33 $r0, $r2
L_139d4: slt45 $r1, $r0
L_139d6: beqzs8 . + (62)
L_139d8: lbi.gp	$r0, [ + #-3524]
L_139dc: bnez38 $r0, . + (56)
L_139de: ex9.it #295
L_139e0: lwi450 $r0, [$r1]
L_139e2: ex9.it #74
L_139e4: bnec	$r0, #1, . + (22)
L_139e8: lh	$r0, [$r10 + ($r6 << #1)]
L_139ec: slti45 $r0, #9
L_139ee: bnezs8 . + (12)
L_139f0: ex9.it #257
L_139f2: sw	$r0, [$r11 + ($r6 << #2)]
L_139f6: movi55 $r0, #3
L_139f8: j8 . + (18)
L_139fa: lwi450 $r0, [$r1]
L_139fc: ex9.it #74
L_139fe: bnez38 $r0, . + (22)
L_13a00: movi	$r0, #0x1f4
L_13a04: sw	$r0, [$r11 + ($r6 << #2)]
L_13a08: movi55 $r0, #6
L_13a0a: sb	$r0, [$r8 + ($r6 << #0)]
L_13a0e: movi55 $r0, #9
L_13a10: sbi	$r0, [$r7 + #0x43]
L_13a14: pop25 $r14, #8
L_13a16: movi	$r4, #0x4c
L_13a1a: maddr32	$r0, $r3, $r4
L_13a1e: lbi	$r0, [$r0 + #0x43]
L_13a22: beqc	$r0, #6, . + (8)
L_13a26: bnec	$r0, #9, . + (24)
L_13a2a: add45 $r1, $r3
L_13a2c: lbi333 $r0, [$r1 + #0]
L_13a2e: beqz38 $r0, . + (8)
L_13a30: subi45 $r0, #1
L_13a32: sbi333 $r0, [$r1 + #0]
L_13a34: ret5 $lp
L_13a36: lhi.gp	$r0, [ + #-6362]
L_13a3a: sw	$r0, [$r2 + ($r3 << #2)]
L_13a3e: ret5 $lp
L_13a40: push25 $r8, #8
L_13a42: lwi.gp	$r6, [ + #-5416]
L_13a46: swi37.sp $r3, [ + #4]
L_13a48: lwi450 $r7, [$r6]
L_13a4a: lwi37.sp $r3, [ + #0x20]
L_13a4c: btst	$r7, $r7, #0x1b
L_13a50: movi55 $r16, #0xa
L_13a52: bnec	$r7, #1, . + (6)
L_13a56: movi55 $r16, #5
L_13a58: add45 $r2, $r3
L_13a5a: lbi333 $r7, [$r2 + #0]
L_13a5c: slt45 $r7, $r16
L_13a5e: beqzs8 . + (76)
L_13a60: addi45 $r7, #1
L_13a62: movi	$r6, #0x4c
L_13a66: sbi333 $r7, [$r2 + #0]
L_13a68: maddr32	$r0, $r3, $r6
L_13a6c: lhi.gp	$r18, [ + #-6356]
L_13a70: movi55 $r6, #0
L_13a72: sbi	$r6, [$r0 + #0x43]
L_13a76: movi55 $r17, #0xa
L_13a78: slli333 $r6, $r3, #2
L_13a7a: add333 $r7, $r1, $r6
L_13a7c: mul	$r17, $r18, $r17
L_13a80: swi450 $r17, [$r7]
L_13a82: lbi333 $r2, [$r2 + #0]
L_13a84: bne	$r2, $r16, . + (38)
L_13a88: movi55 $r2, #1
L_13a8a: sbi	$r2, [$r0 + #0x43]
L_13a8e: addri36.sp $r0, #4
L_13a90: ex9.it #353
L_13a92: add45 $r6, $r5
L_13a94: smw.bi $r0, [$r6], $r0, #0x0
L_13a98: lhi.gp	$r0, [ + #-6362]
L_13a9c: slts45 $r0, $r4
L_13a9e: beqzs8 . + (8)
L_13aa0: addi45 $r4, #0x14
L_13aa2: swi450 $r4, [$r7]
L_13aa4: j8 . + (6)
L_13aa6: addi45 $r0, #0x14
L_13aa8: swi450 $r0, [$r7]
L_13aaa: lbi.gp	$r0, [ + #-4917]
L_13aae: beqz38 $r0, . + (8)
L_13ab0: ex9.it #180
L_13ab2: sw	$r0, [$r1 + ($r3 << #2)]
L_13ab6: pop25 $r8, #8
L_13ab8: push25 $r6, #8
L_13aba: slli333 $r3, $r3, #2
L_13abc: swi37.sp $r0, [ + #4]
L_13abe: add45 $r2, $r3
L_13ac0: lhsi	$r4, [$r2 + #0]
L_13ac4: lhsi	$r15, [$sp + #4]
L_13ac8: slts45 $r4, $r15
L_13aca: beqzs8 . + (20)
L_13acc: lwi.gp	$r0, [ + #-1764]
L_13ad0: lhi	$r5, [$r0 + #0x12]
L_13ad4: not33 $r0, $r4
L_13ad6: movi55 $r4, #0xa
L_13ad8: maddr32	$r0, $r5, $r4
L_13adc: seh33 $r4, $r0
L_13ade: lhsi	$r0, [$r2 + #2]
L_13ae2: lhsi	$r15, [$sp + #6]
L_13ae6: slts45 $r0, $r15
L_13ae8: beqzs8 . + (20)
L_13aea: lwi.gp	$r2, [ + #-1764]
L_13aee: not33 $r0, $r0
L_13af0: lhi	$r5, [$r2 + #0x14]
L_13af4: movi55 $r2, #0xa
L_13af6: maddr32	$r0, $r5, $r2
L_13afa: seh33 $r0, $r0
L_13afc: add45 $r1, $r3
L_13afe: lwi450 $r2, [$r1]
L_13b00: min	$r0, $r0, $r4
L_13b04: slts45 $r0, $r2
L_13b06: beqzs8 . + (28)
L_13b08: ex9.it #280
L_13b0a: bnezs8 . + (8)
L_13b0c: addi	$r0, $r0, #-50
L_13b10: swi450 $r0, [$r1]
L_13b12: lhi.gp	$r2, [ + #-6358]
L_13b16: movi55 $r0, #0xa
L_13b18: mul33 $r0, $r2
L_13b1a: lwi450 $r2, [$r1]
L_13b1c: slts45 $r2, $r0
L_13b1e: beqzs8 . + (4)
L_13b20: swi450 $r0, [$r1]
L_13b22: pop25 $r6, #8
L_13b24: movi	$r3, #0x4c
L_13b28: maddr32	$r0, $r2, $r3
L_13b2c: lbi	$r3, [$r0 + #0x45]
L_13b30: bnez38 $r3, . + (24)
L_13b32: lwi333 $r4, [$r0 + #0x1c]
L_13b34: lwi.gp	$r3, [ + #0x328]
L_13b38: lw	$r3, [$r3 + ($r4 << #2)]
L_13b3c: sltsi	$r15, $r3, #0x1771
L_13b40: bnezs8 . + (8)
L_13b42: movi55 $r3, #1
L_13b44: sbi	$r3, [$r0 + #0x45]
L_13b48: lbi	$r3, [$r0 + #0x45]
L_13b4c: bnec	$r3, #1, . + (30)
L_13b50: lhi.gp	$r3, [ + #-6360]
L_13b54: add_slli	$r2, $r1, $r2, #2
L_13b58: movi55 $r1, #0xa
L_13b5a: mul33 $r1, $r3
L_13b5c: lwi450 $r3, [$r2]
L_13b5e: slts45 $r3, $r1
L_13b60: beqzs8 . + (10)
L_13b62: lbi	$r0, [$r0 + #0x43]
L_13b66: bnez38 $r0, . + (4)
L_13b68: swi450 $r1, [$r2]
L_13b6a: ret5 $lp
L_13b6c: push25 $r6, #8
L_13b6e: lwi37.sp $r0, [ + #0x18]
L_13b70: ex9.it #189
L_13b72: maddr32	$r2, $r0, $r1
L_13b76: lbi	$r1, [$r2 + #0x43]
L_13b7a: bnec	$r1, #8, . + (92)
L_13b7e: lbi	$r1, [$r2 + #0x46]
L_13b82: add45 $r5, $r0
L_13b84: bnec	$r1, #3, . + (50)
L_13b88: ex9.it #366
L_13b8a: sub333 $r1, $r4, $r1
L_13b8c: sltsi45 $r1, #0x15
L_13b8e: bnezs8 . + (40)
L_13b90: lbi333 $r1, [$r5 + #0]
L_13b92: slti	$r15, $r1, #0x64
L_13b96: beqzs8 . + (6)
L_13b98: addi45 $r1, #1
L_13b9a: sbi333 $r1, [$r5 + #0]
L_13b9c: lbi333 $r1, [$r5 + #0]
L_13b9e: slti45 $r1, #0x15
L_13ba0: bnezs8 . + (54)
L_13ba2: ex9.it #456
L_13ba4: lwi450 $r1, [$r0]
L_13ba6: lhi.gp	$r2, [ + #-6362]
L_13baa: slts45 $r2, $r1
L_13bac: bnezs8 . + (42)
L_13bae: sltsi45 $r1, #0xb
L_13bb0: bnezs8 . + (38)
L_13bb2: subi45 $r1, #1
L_13bb4: j8 . + (32)
L_13bb6: lbi333 $r1, [$r5 + #0]
L_13bb8: beqz38 $r1, . + (8)
L_13bba: subi45 $r1, #1
L_13bbc: sbi333 $r1, [$r5 + #0]
L_13bbe: pop25 $r6, #8
L_13bc0: ex9.it #456
L_13bc2: lwi450 $r1, [$r0]
L_13bc4: sub45 $r4, $r1
L_13bc6: bgez	$r4, . + (16)
L_13bca: lhi.gp	$r2, [ + #-6362]
L_13bce: slts45 $r1, $r2
L_13bd0: beqzs8 . + (6)
L_13bd2: addi45 $r1, #1
L_13bd4: swi450 $r1, [$r0]
L_13bd6: pop25 $r6, #8
L_13bd8: lhsi	$r2, [$r0 + #0]
L_13bdc: slts45 $r1, $r2
L_13bde: beqzs8 . + (50)
L_13be0: push25 $r6, #0
L_13be2: lwi.gp	$r4, [ + #-1764]
L_13be6: movi55 $r3, #0xa
L_13be8: lhi	$r5, [$r4 + #0x12]
L_13bec: not33 $r6, $r1
L_13bee: maddr32	$r6, $r5, $r3
L_13bf2: slts45 $r2, $r6
L_13bf4: beqzs8 . + (32)
L_13bf6: lhsi	$r2, [$r0 + #2]
L_13bfa: slts45 $r1, $r2
L_13bfc: beqzs8 . + (24)
L_13bfe: lhi	$r0, [$r4 + #0x14]
L_13c02: not33 $r1, $r1
L_13c04: maddr32	$r1, $r0, $r3
L_13c08: slts45 $r2, $r1
L_13c0a: xori	$r0, $r15, #1
L_13c0e: pop25 $r6, #0
L_13c10: movi55 $r0, #1
L_13c12: ret5 $lp
L_13c14: movi55 $r0, #1
L_13c16: pop25 $r6, #0
L_13c18: push25 $r6, #0
L_13c1a: movi	$r3, #0x4c
L_13c1e: maddr32	$r1, $r2, $r3
L_13c22: mov55 $r6, $r1
L_13c24: movi	$r1, #0x1f4
L_13c28: jal	. + (-80)
L_13c2c: sbi	$r0, [$r6 + #0x49]
L_13c30: pop25 $r6, #0
L_13c32: push25 $r6, #8
L_13c34: swi37.sp $r0, [ + #4]
L_13c36: add_slli	$r1, $r1, $r2, #2
L_13c3a: ex9.it #184
L_13c3c: lhsi	$r15, [$sp + #4]
L_13c40: slts45 $r3, $r15
L_13c42: beqzs8 . + (20)
L_13c44: lwi.gp	$r0, [ + #-1764]
L_13c48: not33 $r3, $r3
L_13c4a: lhi	$r2, [$r0 + #0x12]
L_13c4e: movi55 $r0, #0xa
L_13c50: maddr32	$r3, $r2, $r0
L_13c54: seh33 $r3, $r3
L_13c56: lhsi	$r0, [$r1 + #2]
L_13c5a: lhsi	$r15, [$sp + #6]
L_13c5e: slts45 $r0, $r15
L_13c60: beqzs8 . + (20)
L_13c62: lwi.gp	$r1, [ + #-1764]
L_13c66: not33 $r0, $r0
L_13c68: lhi	$r2, [$r1 + #0x14]
L_13c6c: movi55 $r1, #0xa
L_13c6e: maddr32	$r0, $r2, $r1
L_13c72: seh33 $r0, $r0
L_13c74: min	$r0, $r0, $r3
L_13c78: pop25 $r6, #8
L_13c7a: push25 $r8, #8
L_13c7c: swi37.sp $r0, [ + #4]
L_13c7e: ex9.it #221
L_13c80: maddr32	$r1, $r4, $r0
L_13c84: lbi	$r0, [$r1 + #0x49]
L_13c88: bnez38 $r0, . + (66)
L_13c8a: lhsi	$r7, [$r1 + #8]
L_13c8e: beqz38 $r7, . + (36)
L_13c90: lwi.gp	$r6, [ + #-1764]
L_13c94: movi55 $r0, #0xa
L_13c96: lhi	$r5, [$r6 + #0x12]
L_13c9a: mul33 $r5, $r0
L_13c9c: subi45 $r5, #1
L_13c9e: beq	$r7, $r5, . + (20)
L_13ca2: lhsi	$r5, [$r1 + #0xa]
L_13ca6: beqz38 $r5, . + (12)
L_13ca8: lhi	$r6, [$r6 + #0x14]
L_13cac: mul33 $r0, $r6
L_13cae: subi45 $r0, #1
L_13cb0: bnes38 $r0, . + (26)
L_13cb2: movi55 $r0, #2
L_13cb4: sbi	$r0, [$r1 + #0x49]
L_13cb8: movi55 $r0, #6
L_13cba: sb	$r0, [$r3 + ($r4 << #0)]
L_13cbe: movi	$r3, #0x258
L_13cc2: sw	$r3, [$r2 + ($r4 << #2)]
L_13cc6: sbi	$r0, [$r1 + #0x43]
L_13cca: pop25 $r8, #8
L_13ccc: push25 $r8, #8
L_13cce: lwi37.sp $r6, [ + #0x20]
L_13cd0: swi37.sp $r4, [ + #4]
L_13cd2: slli333 $r4, $r6, #2
L_13cd4: add45 $r2, $r4
L_13cd6: lwi450 $r7, [$r2]
L_13cd8: slts45 $r7, $r1
L_13cda: beqzs8 . + (96)
L_13cdc: ex9.it #189
L_13cde: maddr32	$r0, $r6, $r1
L_13ce2: movi55 $r1, #0xb
L_13ce4: sbi	$r1, [$r0 + #0x48]
L_13ce8: addri36.sp $r1, #4
L_13cea: lmw.bi $r1, [$r1], $r1, #0x0
L_13cee: add45 $r4, $r5
L_13cf0: smw.bi $r1, [$r4], $r1, #0x0
L_13cf4: ex9.it #366
L_13cf6: slts45 $r1, $r7
L_13cf8: beqzs8 . + (4)
L_13cfa: swi450 $r1, [$r2]
L_13cfc: movi55 $r1, #0
L_13cfe: sb	$r1, [$r3 + ($r6 << #0)]
# State 0..9: unsigned8 dispatch to normalized state/flag writers.
L_13d02: lbi	$r1, [$r0 + #0x43]
L_13d06: slti45 $r1, #0xa
L_13d08: beqzs8 . + (50)
L_13d0a: movi	$r15, #0x13b14
L_13d0e: ex9.it #467
L_13d10: ex9.it #158
L_13d12: jr5 $r15
# Loads state byte [r0+0x43], tests unsigned state <10; loads u8 at r15+index through EX9 slot467, adds table base through slot158, jumps to r15. Cases write normalized state byte and flag at +0x46.
L_13d14:
    .byte L_13d1e - L_13d14
    .byte L_13d22 - L_13d14
    .byte L_13d3a - L_13d14
    .byte L_13d3a - L_13d14
    .byte L_13d2a - L_13d14
    .byte L_13d3a - L_13d14
    .byte L_13d2e - L_13d14
    .byte L_13d3a - L_13d14
    .byte L_13d3a - L_13d14
    .byte L_13d26 - L_13d14
L_13d1e: movi55 $r1, #3
L_13d20: j8 . + (16)
L_13d22: movi55 $r1, #2
L_13d24: j8 . + (12)
L_13d26: movi55 $r1, #0xa
L_13d28: j8 . + (8)
L_13d2a: movi55 $r1, #5
L_13d2c: j8 . + (4)
L_13d2e: movi55 $r1, #7
L_13d30: sbi	$r1, [$r0 + #0x43]
L_13d34: movi55 $r1, #1
L_13d36: sbi	$r1, [$r0 + #0x46]
L_13d3a: pop25 $r8, #8
L_13d3c: movi	$r2, #0x4c
L_13d40: ex9.it #303
L_13d42: lbi	$r1, [$r0 + #0x46]
L_13d46: bnec	$r1, #3, . + (10)
L_13d4a: movi55 $r1, #8
L_13d4c: sbi	$r1, [$r0 + #0x43]
L_13d50: ret5 $lp
L_13d52: push25 $r14, #8
L_13d54: mov55 $r7, $r2
L_13d56: mov55 $r9, $r1
L_13d58: mov55 $r2, $r5
L_13d5a: mov55 $r1, $r4
L_13d5c: mov55 $r6, $r5
L_13d5e: swi37.sp $r0, [ + #4]
L_13d60: mov55 $r11, $r3
L_13d62: mov55 $r10, $r4
L_13d64: ifcall9 . + (226)
L_13d66: ex9.it #189
L_13d68: maddr32	$r7, $r6, $r1
L_13d6c: lbi	$r1, [$r7 + #0x48]
L_13d70: bnec	$r1, #0xb, . + (64)
L_13d74: lh	$r2, [$r11 + ($r6 << #1)]
L_13d78: addi	$r15, $r2, #0x96
L_13d7c: slt45 $r0, $r15
L_13d7e: beqzs8 . + (8)
L_13d80: movi55 $r0, #0
L_13d82: sbi	$r0, [$r7 + #0x44]
L_13d86: lbi	$r0, [$r7 + #0x44]
L_13d8a: bnec	$r0, #1, . + (38)
L_13d8e: slli333 $r6, $r6, #2
L_13d90: add45 $r9, $r6
L_13d92: addri36.sp $r0, #4
L_13d94: mov55 $r1, $r9
L_13d96: ex9.it #494
L_13d98: slti	$r15, $r0, #0xe1
L_13d9c: beqzs8 . + (14)
L_13d9e: add45 $r6, $r10
L_13da0: lmw.bi $r0, [$r9], $r0, #0x0
L_13da4: smw.bi $r0, [$r6], $r0, #0x0
L_13da8: pop25 $r14, #8
L_13daa: movi55 $r0, #0
L_13dac: sbi	$r0, [$r7 + #0x44]
L_13db0: pop25 $r14, #8
L_13db2: push25 $r14, #8
L_13db4: lwi37.sp $r6, [ + #0x40]
L_13db6: swi37.sp $r0, [ + #4]
L_13db8: ex9.it #221
L_13dba: maddr32	$r3, $r6, $r0
L_13dbe: mov55 $r12, $r2
L_13dc0: lbi	$r2, [$r3 + #0x46]
L_13dc4: mov55 $r11, $r4
L_13dc6: mov55 $r10, $r5
L_13dc8: lwi37.sp $r7, [ + #0x38]
L_13dca: lwi	$r8, [$sp + #0x3c]
L_13dce: mov55 $r9, $r3
L_13dd0: bnec	$r2, #1, . + (194)
L_13dd4: lbi	$r0, [$r3 + #0x43]
L_13dd8: bnec	$r0, #2, . + (24)
L_13ddc: ifcall9 . + (28)
L_13dde: bnec	$r0, #1, . + (12)
L_13de2: movi55 $r0, #0
L_13de4: sbi	$r0, [$r3 + #0x44]
L_13de8: j8 . + (56)
L_13dea: sbi	$r2, [$r3 + #0x44]
L_13dee: j8 . + (50)
L_13df0: beqc	$r0, #3, . + (48)
L_13df4: bnec	$r0, #5, . + (34)
L_13df8: ex9.it #135
L_13dfa: lwi450 $r0, [$r0]
L_13dfc: ex9.it #74
L_13dfe: ifret16
L_13e00: bnec	$r0, #1, . + (12)
L_13e04: movi55 $r0, #0
L_13e06: sbi	$r0, [$r3 + #0x44]
L_13e0a: j8 . + (6)
L_13e0c: sbi	$r2, [$r3 + #0x44]
L_13e10: movi	$r13, #0x12c
L_13e14: j8 . + (16)
L_13e16: beqc	$r0, #7, . + (10)
L_13e1a: movi55 $r13, #0
L_13e1c: bnec	$r0, #0xa, . + (8)
L_13e20: movi	$r13, #0x1f4
L_13e24: movi55 $r2, #0
L_13e26: movi	$r0, #0x2cc04
L_13e2a: slli	$r14, $r6, #2
L_13e2e: sh	$r2, [$r0 + ($r6 << #1)]
L_13e32: add45 $r1, $r14
L_13e34: add	$r0, $r8, $r14
L_13e38: lmw.bi $r1, [$r1], $r1, #0x0
L_13e3c: smw.bi $r1, [$r0], $r1, #0x0
L_13e40: lwi37.sp $r0, [ + #4]
L_13e42: mov55 $r1, $r8
L_13e44: mov55 $r2, $r6
L_13e46: jal	. + (-532)
L_13e4a: slt45 $r0, $r13
L_13e4c: beqzs8 . + (16)
L_13e4e: zeh33 $r0, $r0
L_13e50: add_slli	$r1, $r7, $r6, #1
L_13e54: bnez38 $r0, . + (4)
L_13e56: movi55 $r0, #1
L_13e58: shi333 $r0, [$r1 + #0]
L_13e5a: j8 . + (6)
L_13e5c: sh	$r13, [$r7 + ($r6 << #1)]
L_13e60: add	$r3, $r12, $r14
L_13e64: lhi333 $r1, [$r3 + #0]
L_13e66: ex9.it #376
L_13e68: lhi333 $r3, [$r3 + #2]
L_13e6a: sub333 $r1, $r0, $r1
L_13e6c: ex9.it #101
L_13e6e: seh33 $r1, $r1
L_13e70: movi	$r2, #0x2cbd4
L_13e74: sub45 $r0, $r3
L_13e76: sh	$r1, [$r2 + ($r6 << #2)]
L_13e7a: seh33 $r0, $r0
L_13e7c: add45 $r2, $r14
L_13e7e: shi333 $r0, [$r2 + #2]
L_13e80: mul33 $r0, $r0
L_13e82: maddr32	$r0, $r1, $r1
L_13e86: ex9.it #331
L_13e88: movi	$r1, #0x2cba4
L_13e8c: ex9.it #309
L_13e8e: movi55 $r0, #2
L_13e90: ex9.it #452
L_13e92: lbi	$r1, [$r9 + #0x48]
L_13e96: lbi	$r0, [$r9 + #0x46]
L_13e9a: beqc	$r1, #0xb, . + (8)
L_13e9e: j	. + (360)
L_13ea2: movi	$r14, #0x2cc04
L_13ea6: slli	$r13, $r6, #2
L_13eaa: movi	$fp, #0x2cba4
L_13eae: bnec	$r0, #3, . + (20)
L_13eb2: movi55 $r0, #0
L_13eb4: sw	$r0, [$fp + ($r6 << #2)]
L_13eb8: lh	$r0, [$r7 + ($r6 << #1)]
L_13ebc: sh	$r0, [$r14 + ($r6 << #1)]
L_13ec0: j8 . + (104)
L_13ec2: lbi	$r0, [$r9 + #0x44]
L_13ec6: bnez38 $r0, . + (30)
L_13ec8: add	$r1, $r8, $r13
L_13ecc: addri36.sp $r0, #4
L_13ece: ex9.it #494
L_13ed0: ex9.it #331
L_13ed2: lh	$r1, [$r14 + ($r6 << #1)]
L_13ed6: zeh33 $r0, $r0
L_13ed8: slt45 $r1, $r0
L_13eda: shi.gp	$r0, [ + #-3648]
L_13ede: beqzs8 . + (6)
L_13ee0: sh	$r0, [$r14 + ($r6 << #1)]
L_13ee4: add_slli	$r7, $r7, $r6, #1
L_13ee8: lhi333 $r0, [$r7 + #0]
L_13eea: lh	$r1, [$r14 + ($r6 << #1)]
L_13eee: slt45 $r1, $r0
L_13ef0: bnezs8 . + (14)
L_13ef2: movi	$r1, #0x2cc04
L_13ef6: sh	$r0, [$r1 + ($r6 << #1)]
L_13efa: movi55 $r0, #3
L_13efc: ex9.it #452
L_13efe: add	$r2, $r11, $r13
L_13f02: lw	$r0, [$fp + ($r6 << #2)]
L_13f06: lwi450 $r1, [$r2]
L_13f08: slts45 $r1, $r0
L_13f0a: beqzs8 . + (30)
L_13f0c: lhi333 $r3, [$r7 + #0]
L_13f0e: lh	$r1, [$r14 + ($r6 << #1)]
L_13f12: sub333 $r1, $r3, $r1
L_13f14: mul33 $r0, $r1
L_13f16: ex9.it #366
L_13f18: divsr	$r0, $r0, $r0, $r3
L_13f1c: slts45 $r0, $r1
L_13f1e: swi450 $r0, [$r2]
L_13f20: beqzs8 . + (8)
L_13f22: movi55 $r0, #3
L_13f24: ex9.it #452
L_13f26: swi450 $r1, [$r2]
L_13f28: lw	$r1, [$r11 + ($r6 << #2)]
L_13f2c: movi55 $r0, #5
L_13f2e: mul33 $r0, $r1
L_13f30: slt45 $r0, $r10
L_13f32: beqzs8 . + (20)
L_13f34: add	$r0, $r12, $r13
L_13f38: lhi	$r1, [$sp + #4]
L_13f3c: shi333 $r1, [$r0 + #0]
L_13f3e: lhi	$r1, [$sp + #6]
L_13f42: shi333 $r1, [$r0 + #2]
L_13f44: pop25 $r14, #8
L_13f46: slts45 $r10, $r1
L_13f48: bnezs8 . + (232)
L_13f4a: add	$r11, $r12, $r13
L_13f4e: lhsi	$r13, [$r11 + #0]
L_13f52: lhsi	$r0, [$sp + #4]
L_13f56: lhsi	$r12, [$r11 + #2]
L_13f5a: sub45 $r0, $r13
L_13f5c: lhsi	$r5, [$sp + #6]
L_13f60: abs	$r4, $r0
L_13f64: sub45 $r5, $r12
L_13f66: beqz38 $r4, . + (16)
L_13f68: abs	$r3, $r5
L_13f6c: movi55 $r2, #0xa
L_13f6e: mul33 $r2, $r3
L_13f70: divsr	$r9, $r9, $r2, $r4
L_13f74: j8 . + (4)
L_13f76: mov55 $r9, $r4
L_13f78: mul	$r6, $r0, $r1
L_13f7c: mul33 $r1, $r5
L_13f7e: divsr	$r6, $r6, $r6, $r10
L_13f82: divsr	$r10, $r10, $r1, $r10
L_13f86: sub333 $r6, $r0, $r6
L_13f88: sub45 $r5, $r10
L_13f8a: movi55 $r0, #5
L_13f8c: mul33 $r6, $r0
L_13f8e: mul33 $r0, $r5
L_13f90: movi55 $r7, #4
L_13f92: divsr	$r6, $r6, $r6, $r7
L_13f96: divsr	$r7, $r7, $r0, $r7
L_13f9a: mul	$r0, $r7, $r7
L_13f9e: maddr32	$r0, $r6, $r6
L_13fa2: ex9.it #331
L_13fa4: addi	$r1, $r9, #-5
L_13fa8: slti45 $r1, #0x10
L_13faa: beqzs8 . + (48)
L_13fac: sltsi45 $r0, #0xb
L_13fae: bnezs8 . + (70)
L_13fb0: subi45 $r9, #9
L_13fb2: slti45 $r9, #3
L_13fb4: beqzs8 . + (68)
L_13fb6: sltsi45 $r0, #0xf
L_13fb8: beqzs8 . + (64)
L_13fba: abs	$r0, $r6
L_13fbe: abs	$r1, $r7
L_13fc2: slts45 $r1, $r0
L_13fc4: beqzs8 . + (10)
L_13fc6: mul33 $r7, $r0
L_13fc8: divsr	$r7, $r7, $r7, $r1
L_13fcc: j8 . + (44)
L_13fce: slts45 $r0, $r1
L_13fd0: beqzs8 . + (40)
L_13fd2: mul33 $r6, $r1
L_13fd4: divsr	$r6, $r6, $r6, $r0
L_13fd8: j8 . + (32)
L_13fda: sltsi45 $r0, #0xa
L_13fdc: beqzs8 . + (6)
L_13fde: movi55 $r7, #0
L_13fe0: mov55 $r6, $r7
L_13fe2: addi333 $r0, $r6, #5
L_13fe4: slti45 $r0, #0xb
L_13fe6: beqzs8 . + (4)
L_13fe8: movi55 $r6, #0
L_13fea: addi333 $r0, $r7, #5
L_13fec: slti45 $r0, #0xb
L_13fee: beqzs8 . + (10)
L_13ff0: movi55 $r7, #0
L_13ff2: j8 . + (6)
L_13ff4: movi55 $r7, #0
L_13ff6: mov55 $r6, $r7
L_13ff8: add45 $r6, $r13
L_13ffa: add45 $r7, $r12
L_13ffc: shi	$r6, [$r11 + #0]
L_14000: shi	$r7, [$r11 + #2]
L_14004: pop25 $r14, #8
L_14006: bnec	$r0, #2, . + (42)
L_1400a: ex9.it #366
L_1400c: slts45 $r1, $r10
L_1400e: bnezs8 . + (34)
L_14010: movi	$r2, #0x2cba4
L_14014: lw	$r0, [$r2 + ($r6 << #2)]
L_14018: subi45 $r0, #1
L_1401a: sw	$r0, [$r2 + ($r6 << #2)]
L_1401e: add_slli	$r6, $r11, $r6, #2
L_14022: lwi450 $r0, [$r6]
L_14024: subi45 $r0, #1
L_14026: slts45 $r0, $r1
L_14028: swi450 $r0, [$r6]
L_1402a: beqzs8 . + (6)
L_1402c: movi55 $r0, #3
L_1402e: ex9.it #452
L_14030: pop25 $r14, #8
L_14032: push25 $r14, #24
L_14034: swi37.sp $r0, [ + #0x10]
L_14036: lhsi	$r0, [$r1 + #0]
L_1403a: shi.gp	$r0, [ + #-2980]
L_1403e: lhsi	$r0, [$r1 + #2]
L_14042: shi.gp	$r0, [ + #-2978]
L_14046: lwi37.sp $r7, [ + #0x10]
L_14048: ex9.it #221
L_1404a: maddr32	$r7, $r3, $r0
L_1404e: mov55 $r11, $r2
L_14050: lwi450 $r1, [$r7]
L_14052: mov55 $r6, $r3
L_14054: ex9.it #397
L_14056: movi	$r8, #0x2ce24
L_1405a: movi	$r9, #0x2cdc4
L_1405e: slli	$r10, $r3, #2
L_14062: bnec	$r1, #1, . + (128)
L_14066: movi55 $r0, #0
L_14068: movi	$r2, #0x2ce30
L_1406c: sb	$r0, [$r2 + ($r3 << #0)]
L_14070: sb	$r0, [$r8 + ($r3 << #0)]
L_14074: lwi.gp	$r2, [ + #-2980]
L_14078: movi	$r3, #0x2cdf4
L_1407c: sw	$r2, [$r3 + ($r6 << #2)]
L_14080: sw	$r2, [$r9 + ($r6 << #2)]
L_14084: lhi.gp	$r2, [ + #-6356]
L_14088: sbi	$r1, [$r7 + #0x47]
L_1408c: movi55 $r1, #0xa
L_1408e: mul33 $r1, $r2
L_14090: ex9.it #344
L_14092: sw	$r1, [$r2 + ($r6 << #2)]
L_14096: movi	$r1, #0x2cd88
L_1409a: sb	$r0, [$r1 + ($r6 << #0)]
L_1409e: sbi	$r0, [$r7 + #0x43]
L_140a2: sbi	$r0, [$r7 + #0x44]
L_140a6: sbi	$r0, [$r7 + #0x45]
L_140aa: sbi	$r0, [$r7 + #0x46]
L_140ae: lwi.gp	$r4, [ + #0x328]
L_140b2: movi55 $r0, #0
L_140b4: movi	$r1, #0x2ccc8
L_140b8: lwi333 $r3, [$r7 + #0x1c]
L_140ba: add	$r2, $r0, $r10
L_140be: lw	$r3, [$r4 + ($r3 << #2)]
L_140c2: addi45 $r0, #1
L_140c4: ex9.it #198
L_140c6: bnec	$r0, #4, . + (-14)
L_140ca: movi55 $r1, #0
L_140cc: movi	$r0, #0x2f2b4
L_140d0: ex9.it #502
L_140d2: mov55 $r2, $r6
L_140d4: movi	$r0, #0x2ce3c
L_140d8: lwi37.sp $r1, [ + #0x10]
L_140da: jal	. + (-1218)
L_140de: j	. + (302)
L_140e2: movi	$r12, #0x2cc98
L_140e6: movi	$r1, #0x2ce3c
L_140ea: add	$r0, $r12, $r10
L_140ee: ex9.it #494
L_140f0: ex9.it #331
L_140f2: swi37.sp $r0, [ + #0x14]
L_140f4: movi55 $r0, #0xc
L_140f6: sbi	$r0, [$r7 + #0x48]
L_140fa: lbi	$r0, [$r7 + #0x43]
L_140fe: mov55 $r7, $r12
L_14100: slti45 $r0, #2
L_14102: beqzs8 . + (58)
L_14104: lwi37.sp $r0, [ + #0x10]
L_14106: lwi.gp	$r3, [ + #-2980]
L_1410a: lwi37.sp $r4, [ + #0x14]
L_1410c: movi	$r5, #0x2cdf4
L_14110: swi450 $r6, [$sp]
L_14112: movi	$r1, #0x2cd94
L_14116: movi	$r2, #0x2ce30
L_1411a: jal	. + (-1754)
L_1411e: lwi37.sp $r0, [ + #0x10]
L_14120: mov55 $r2, $r6
L_14122: movi	$r1, #0x2cd94
L_14126: jal	. + (-1538)
L_1412a: ex9.it #247
L_1412c: movi	$r1, #0x2cd94
L_14130: movi	$r2, #0x2cdf4
L_14134: mov55 $r3, $r6
L_14136: jal	. + (-1662)
L_1413a: j8 . + (70)
L_1413c: subi333 $r1, $r0, #2
L_1413e: slti45 $r1, #2
L_14140: bnezs8 . + (16)
L_14142: beqc	$r0, #5, . + (14)
L_14146: subi333 $r1, $r0, #7
L_14148: slti45 $r1, #2
L_1414a: bnezs8 . + (6)
L_1414c: bnec	$r0, #0xa, . + (52)
L_14150: lwi37.sp $r0, [ + #0x10]
L_14152: mov55 $r1, $r6
L_14154: jal	. + (-1048)
L_14158: ex9.it #247
L_1415a: lwi37.sp $r1, [ + #0x10]
L_1415c: ex9.it #344
L_1415e: movi	$r3, #0x2ce24
L_14162: mov55 $r4, $r6
L_14164: jal	. + (-1258)
L_14168: swi450 $r6, [$sp]
L_1416a: ex9.it #247
L_1416c: lw	$r1, [$r9 + ($r6 << #2)]
L_14170: lwi37.sp $r2, [ + #0x10]
L_14172: movi	$r3, #0x2cd94
L_14176: lwi37.sp $r4, [ + #0x14]
L_14178: movi	$r5, #0x2cd88
L_1417c: jal	. + (-1552)
L_14180: movi	$r4, #0x2cd94
L_14184: lwi37.sp $r5, [ + #0x14]
L_14186: swi450 $r8, [$sp]
L_14188: ex9.it #247
L_1418a: movi	$r1, #0x2cdf4
L_1418e: lwi37.sp $r2, [ + #0x10]
L_14190: movi	$r3, #0x2ccc8
L_14194: swi37.sp $r6, [ + #4]
L_14196: jal	. + (-2146)
L_1419a: lwi37.sp $r0, [ + #0x10]
L_1419c: movi	$r1, #0x2ce24
L_141a0: ex9.it #344
L_141a2: mov55 $r3, $r6
L_141a4: jal	. + (-1934)
L_141a8: movi	$r12, #0x2cc50
L_141ac: lwi37.sp $r0, [ + #0x10]
L_141ae: lwi37.sp $r1, [ + #0x14]
L_141b0: ex9.it #344
L_141b2: movi	$r3, #0x2ce24
L_141b6: lwi.gp	$r4, [ + #-2980]
L_141ba: movi	$r5, #0x2cdf4
L_141be: movi	$r8, #0x2cc20
L_141c2: swi450 $r6, [$sp]
L_141c4: jal	. + (-1272)
L_141c8: ex9.it #247
L_141ca: movi	$r1, #0x2cc68
L_141ce: lwi37.sp $r2, [ + #0x10]
L_141d0: mov55 $r3, $r12
L_141d2: mov55 $r4, $r8
L_141d4: mov55 $r5, $r6
L_141d6: jal	. + (-1156)
L_141da: ex9.it #247
L_141dc: movi	$r1, #0x2cc68
L_141e0: swi	$r12, [$sp + #0]
L_141e4: swi	$r8, [$sp + #4]
L_141e8: swi37.sp $r6, [ + #8]
L_141ea: movi	$r2, #0x2cc98
L_141ee: lwi37.sp $r3, [ + #0x10]
L_141f0: movi	$r4, #0x2cd94
L_141f4: lwi37.sp $r5, [ + #0x14]
L_141f6: jal	. + (-1092)
L_141fa: lh	$r0, [$r7 + ($r6 << #2)]
L_141fe: add45 $r7, $r10
L_14200: sh	$r0, [$r9 + ($r6 << #2)]
L_14204: lhi333 $r1, [$r7 + #2]
L_14206: add	$r0, $r9, $r10
L_1420a: shi333 $r1, [$r0 + #2]
L_1420c: lh	$r0, [$r9 + ($r6 << #2)]
L_14210: add45 $r10, $r9
L_14212: shi	$r0, [$r11 + #0]
L_14216: lhi	$r0, [$r10 + #2]
L_1421a: shi	$r0, [$r11 + #2]
L_1421e: lwi.gp	$r1, [ + #-2980]
L_14222: movi	$r0, #0x2cc68
L_14226: sw	$r1, [$r0 + ($r6 << #2)]
L_1422a: lw	$r1, [$r9 + ($r6 << #2)]
L_1422e: movi	$r0, #0x2cc98
L_14232: sw	$r1, [$r0 + ($r6 << #2)]
L_14236: pop25 $r14, #24
L_14238: push25 $r14, #0
L_1423a: lhsi	$r4, [$r1 + #0]
L_1423e: lhsi	$r5, [$r1 + #2]
L_14242: ex9.it #189
L_14244: maddr32	$r0, $r3, $r1
L_14248: movi	$r7, #0x2cb70
L_1424c: lwi450 $r6, [$r0]
L_1424e: slli	$r17, $r3, #2
L_14252: lwi.gp	$r1, [ + #-1760]
L_14256: bnec	$r6, #1, . + (36)
L_1425a: shi333 $r4, [$r2 + #0]
L_1425c: shi333 $r5, [$r2 + #2]
L_1425e: lbi333 $r0, [$r1 + #5]
L_14260: ifcall9 . + (440)
L_14262: beqc	$r0, #1, . + (8)
L_14266: j	. + (458)
L_1426a: movi	$r7, #0x2cb40
L_1426e: movi55 $r0, #0
L_14270: sh	$r0, [$r7 + ($r3 << #2)]
L_14274: add45 $r7, $r17
L_14276: shi333 $r0, [$r7 + #2]
L_14278: pop25 $r14, #0
L_1427a: lbi	$r18, [$r1 + #5]
L_1427e: lbi	$r19, [$r0 + #0x18]
L_14282: bnec	$r18, #1, . + (124)
L_14286: andi	$r0, $r19, #2
L_1428a: beqz38 $r0, . + (116)
L_1428c: movi	$r0, #0x2cb40
L_14290: lhs	$r6, [$r0 + ($r3 << #2)]
L_14294: add45 $r0, $r17
L_14296: lhsi	$r0, [$r0 + #2]
L_1429a: bnez38 $r6, . + (10)
L_1429c: beqz38 $r0, . + (98)
L_1429e: lhs	$r4, [$r7 + ($r3 << #2)]
L_142a2: j8 . + (92)
L_142a4: bnez38 $r0, . + (12)
L_142a6: add	$r0, $r7, $r17
L_142aa: lhsi	$r5, [$r0 + #2]
L_142ae: j8 . + (80)
L_142b0: add	$r1, $r7, $r17
L_142b4: ex9.it #76
L_142b6: subri	$r16, $r6, #0
L_142ba: mul33 $r6, $r1
L_142bc: lhs	$r1, [$r7 + ($r3 << #2)]
L_142c0: mov55 $r10, $r4
L_142c2: msubr32	$r6, $r0, $r1
L_142c6: mul	$r4, $r16, $r16
L_142ca: mul	$r1, $r0, $r0
L_142ce: mul	$r11, $r0, $r16
L_142d2: add	$r9, $r1, $r4
L_142d6: mul	$r4, $r4, $r10
L_142da: msubr32	$r4, $r11, $r5
L_142de: msubr32	$r4, $r0, $r6
L_142e2: neg33 $r0, $r0
L_142e4: mul	$r0, $r0, $r16
L_142e8: mul33 $r1, $r5
L_142ea: maddr32	$r1, $r0, $r10
L_142ee: msubr32	$r1, $r16, $r6
L_142f2: divsr	$r4, $r4, $r4, $r9
L_142f6: divsr	$r1, $r1, $r1, $r9
L_142fa: seh33 $r4, $r4
L_142fc: seh33 $r5, $r1
L_142fe: andi	$r0, $r19, #2
L_14302: beqz38 $r0, . + (248)
L_14304: lwi.gp	$r0, [ + #-1764]
L_14308: mov55 $r1, $r4
L_1430a: lhi	$r6, [$r0 + #0x12]
L_1430e: slts45 $r4, $r6
L_14310: bnezs8 . + (16)
L_14312: lhs	$r19, [$r7 + ($r3 << #2)]
L_14316: slts45 $r19, $r6
L_14318: beqzs8 . + (30)
L_1431a: addi	$r19, $r6, #-1
L_1431e: j8 . + (24)
L_14320: bgez	$r4, . + (16)
L_14324: lhs	$r19, [$r7 + ($r3 << #2)]
L_14328: bltz	$r19, . + (14)
L_1432c: movi55 $r19, #0
L_1432e: j8 . + (8)
L_14330: mov55 $r19, $r4
L_14332: movi55 $r9, #0
L_14334: j8 . + (4)
L_14336: movi55 $r9, #1
L_14338: lhi	$r16, [$r0 + #0x14]
L_1433c: mov55 $r0, $r5
L_1433e: slts45 $r5, $r16
L_14340: bnezs8 . + (20)
L_14342: add	$r9, $r7, $r17
L_14346: lhsi	$r9, [$r9 + #2]
L_1434a: slts45 $r9, $r16
L_1434c: beqzs8 . + (34)
L_1434e: addi	$r9, $r16, #-1
L_14352: j8 . + (28)
L_14354: bgez	$r5, . + (20)
L_14358: add	$r9, $r7, $r17
L_1435c: lhsi	$r10, [$r9 + #2]
L_14360: movi55 $r9, #0
L_14362: min	$r9, $r10, $r9
L_14366: j8 . + (8)
L_14368: bnec	$r9, #1, . + (146)
L_1436c: mov55 $r9, $r5
L_1436e: lhs	$r10, [$r7 + ($r3 << #2)]
L_14372: add	$r11, $r7, $r17
L_14376: lhsi	$r11, [$r11 + #2]
L_1437a: sub	$r13, $r4, $r10
L_1437e: sub	$r12, $r5, $r11
L_14382: beqz	$r13, . + (24)
L_14386: movi55 $r0, #2
L_14388: sub	$r14, $r19, $r10
L_1438c: divsr	$r0, $r0, $r13, $r0
L_14390: maddr32	$r0, $r14, $r12
L_14394: divsr	$r0, $r0, $r0, $r13
L_14398: add45 $r0, $r11
L_1439a: beqz	$r12, . + (24)
L_1439e: movi55 $r1, #2
L_143a0: sub	$r14, $r9, $r11
L_143a4: divsr	$r1, $r1, $r12, $r1
L_143a8: maddr32	$r1, $r14, $r13
L_143ac: divsr	$r1, $r1, $r1, $r12
L_143b0: add45 $r1, $r10
L_143b2: beq	$r9, $r11, . + (118)
L_143b6: slts45 $r1, $r6
L_143b8: bnezs8 . + (10)
L_143ba: subri	$r6, $r6, #1
L_143be: add45 $r6, $r1
L_143c0: j8 . + (8)
L_143c2: bgez	$r1, . + (102)
L_143c6: neg33 $r6, $r1
L_143c8: beq	$r19, $r10, . + (26)
L_143cc: slts45 $r0, $r16
L_143ce: bnezs8 . + (10)
L_143d0: subri	$r16, $r16, #1
L_143d4: add45 $r16, $r0
L_143d6: j8 . + (14)
L_143d8: bgez	$r0, . + (10)
L_143dc: subri	$r16, $r0, #0
L_143e0: j8 . + (4)
L_143e2: movi55 $r16, #0
L_143e4: slts45 $r16, $r6
L_143e6: beqzs8 . + (10)
L_143e8: seh	$r4, $r19
L_143ec: seh33 $r5, $r0
L_143ee: j8 . + (12)
L_143f0: slts45 $r6, $r16
L_143f2: beqzs8 . + (8)
L_143f4: seh33 $r4, $r1
L_143f6: seh	$r5, $r9
L_143fa: bnec	$r18, #1, . + (30)
L_143fe: lh	$r1, [$r7 + ($r3 << #2)]
L_14402: movi	$r0, #0x2cb40
L_14406: sub333 $r1, $r4, $r1
L_14408: sh	$r1, [$r0 + ($r3 << #2)]
L_1440c: add	$r1, $r7, $r17
L_14410: lhi333 $r1, [$r1 + #2]
L_14412: add45 $r0, $r17
L_14414: sub333 $r1, $r5, $r1
L_14416: shi333 $r1, [$r0 + #2]
L_14418: sh	$r4, [$r7 + ($r3 << #2)]
L_1441c: add45 $r7, $r17
L_1441e: shi333 $r5, [$r7 + #2]
L_14420: ifret16
L_14422: shi333 $r4, [$r2 + #0]
L_14424: shi333 $r5, [$r2 + #2]
L_14426: pop25 $r14, #0
L_14428: beq	$r19, $r10, . + (-46)
L_1442c: movi55 $r6, #0
L_1442e: j8 . + (-98)
L_14430: pop25 $r14, #0
L_14432: push25 $r14, #24
L_14434: mov55 $r6, $r0
L_14436: ex9.it #97
L_14438: beqc	$r0, #1, . + (28)
L_1443c: lwi.gp	$r8, [ + #-1760]
L_14440: lwi.gp	$r0, [ + #-6236]
L_14444: lhi	$r1, [$r8 + #0xa]
L_14448: slt45 $r0, $r1
L_1444a: beqzs8 . + (46)
L_1444c: addi45 $r0, #1
L_1444e: swi.gp	$r0, [ + #-6236]
L_14452: j8 . + (38)
L_14454: lwi.gp	$r3, [ + #-1764]
L_14458: mov55 $r0, $r6
L_1445a: movi55 $r1, #0
L_1445c: lbi	$r2, [$r3 + #9]
L_14460: slt45 $r1, $r2
L_14462: beqzs8 . + (-38)
L_14464: lwi450 $r2, [$r0]
L_14466: bnec	$r2, #4, . + (12)
L_1446a: lhi333 $r2, [$r0 + #8]
L_1446c: shi333 $r2, [$r0 + #4]
L_1446e: lhi333 $r2, [$r0 + #0xa]
L_14470: shi333 $r2, [$r0 + #6]
L_14472: addi45 $r1, #1
L_14474: ex9.it #214
L_14476: j8 . + (-26)
L_14478: movi55 $r0, #0
L_1447a: movi55 $r7, #0
L_1447c: sbi.gp	$r0, [ + #-4917]
L_14480: addi	$r9, $r6, #8
L_14484: movi	$r10, #0x2edc0
L_14488: movi55 $r12, #0xa
L_1448a: movi55 $r13, #0xa
L_1448c: mov55 $r14, $r7
L_1448e: lwi.gp	$r11, [ + #-1764]
L_14492: lbi	$r0, [$r11 + #9]
L_14496: slt45 $r7, $r0
L_14498: beqz	$r15, . + (406)
L_1449c: lwi	$r0, [$r9 + #-8]
L_144a0: beqz	$r0, . + (384)
L_144a4: bnec	$r0, #5, . + (8)
L_144a8: j	. + (376)
L_144ac: lhi	$r0, [$r9 + #4]
L_144b0: ex9.it #130
L_144b2: lhi	$r0, [$r9 + #6]
L_144b6: ex9.it #147
L_144b8: addri36.sp $r1, #0x14
L_144ba: mov55 $r2, $r7
L_144bc: mov55 $r0, $r6
L_144be: jal	. + (-4780)
L_144c2: swi37.sp $r0, [ + #0xc]
L_144c4: addri36.sp $r1, #0x14
L_144c6: lbi	$r0, [$r8 + #1]
L_144ca: swi450 $r0, [$sp]
L_144cc: mov55 $r2, $r1
L_144ce: mov55 $r0, $r6
L_144d0: mov55 $r3, $r7
L_144d2: lwi.gp	$r4, [ + #-6236]
L_144d6: lwi.gp	$r5, [ + #-1944]
L_144da: jal	. + (-4000)
L_144de: lbi.gp	$r2, [ + #-4825]
L_144e2: bnec	$r2, #1, . + (104)
L_144e6: lwi	$r1, [$r10 + #0x1c]
L_144ea: lwi.gp	$r0, [ + #0x310]
L_144ee: lh	$r0, [$r0 + ($r1 << #1)]
L_144f2: slti45 $r0, #0xb
L_144f4: beqzs8 . + (86)
L_144f6: lbi.gp	$r0, [ + #-1948]
L_144fa: slti45 $r0, #0xf
L_144fc: beqzs8 . + (78)
L_144fe: ex9.it #216
L_14500: xlsb33 $r1, $r0
L_14502: beqz38 $r1, . + (34)
L_14504: lhi	$r1, [$r10 + #8]
L_14508: addi	$r15, $r1, #-35
L_1450c: slti	$r15, $r15, #0x5fb
L_14510: bnezs8 . + (20)
L_14512: lhi	$r1, [$r10 + #0xa]
L_14516: addi	$r15, $r1, #-501
L_1451a: slti	$r15, $r15, #0x617
L_1451e: beqzs8 . + (6)
L_14520: sbi.gp	$r2, [ + #-4917]
L_14524: bmski33 $r0, #1
L_14526: beqz38 $r0, . + (36)
L_14528: lhi	$r0, [$r10 + #0xa]
L_1452c: addi	$r15, $r0, #-35
L_14530: slti	$r15, $r15, #0x9bb
L_14534: bnezs8 . + (22)
L_14536: lhi	$r0, [$r10 + #8]
L_1453a: addi	$r15, $r0, #-301
L_1453e: slti	$r15, $r15, #0x3e7
L_14542: beqzs8 . + (8)
L_14544: movi55 $r0, #2
L_14546: sbi.gp	$r0, [ + #-4917]
L_1454a: lbi.gp	$r0, [ + #-4917]
L_1454e: beqz38 $r0, . + (16)
L_14550: lbi.gp	$r0, [ + #-1948]
L_14554: slti	$r15, $r0, #0x64
L_14558: beqzs8 . + (14)
L_1455a: addi45 $r0, #1
L_1455c: j8 . + (6)
L_1455e: ex9.it #222
L_14560: bnez38 $r0, . + (6)
L_14562: sbi.gp	$r0, [ + #-1948]
L_14566: lhi	$r0, [$sp + #0x14]
L_1456a: mul	$r0, $r0, $r12
L_1456e: ex9.it #130
L_14570: lhi	$r0, [$sp + #0x16]
L_14574: mul	$r0, $r0, $r12
L_14578: ex9.it #147
L_1457a: lbi	$r0, [$r8 + #3]
L_1457e: beqz38 $r0, . + (16)
L_14580: addri36.sp $r1, #0x14
L_14582: mov55 $r0, $r6
L_14584: mov55 $r2, $r1
L_14586: mov55 $r3, $r7
L_14588: lwi37.sp $r4, [ + #0xc]
L_1458a: jal	. + (-3742)
L_1458e: lbi	$r0, [$r8 + #4]
L_14592: beqz38 $r0, . + (16)
L_14594: addri36.sp $r1, #0x14
L_14596: mov55 $r0, $r6
L_14598: mov55 $r2, $r1
L_1459a: mov55 $r3, $r7
L_1459c: lwi37.sp $r4, [ + #0xc]
L_1459e: jal	. + (-3388)
L_145a2: lbi	$r0, [$r8 + #2]
L_145a6: beqz38 $r0, . + (14)
L_145a8: addri36.sp $r1, #0x14
L_145aa: mov55 $r0, $r6
L_145ac: mov55 $r2, $r1
L_145ae: mov55 $r3, $r7
L_145b0: jal	. + (-1406)
L_145b4: lhsi	$r0, [$sp + #0x14]
L_145b8: addri36.sp $r1, #0x14
L_145ba: addi45 $r0, #5
L_145bc: divsr	$r0, $r0, $r0, $r13
L_145c0: ex9.it #130
L_145c2: lhsi	$r0, [$sp + #0x16]
L_145c6: mov55 $r2, $r1
L_145c8: addi45 $r0, #5
L_145ca: divsr	$r0, $r0, $r0, $r13
L_145ce: ex9.it #147
L_145d0: mov55 $r3, $r7
L_145d2: mov55 $r0, $r6
L_145d4: jal	. + (-924)
L_145d8: lhsi	$r0, [$sp + #0x14]
L_145dc: bgez	$r0, . + (8)
L_145e0: shi	$r14, [$sp + #0x14]
L_145e4: lhi	$r0, [$r11 + #0x12]
L_145e8: lhsi	$r1, [$sp + #0x14]
L_145ec: slts45 $r1, $r0
L_145ee: bnezs8 . + (6)
L_145f0: subi45 $r0, #1
L_145f2: ex9.it #130
L_145f4: lhsi	$r0, [$sp + #0x16]
L_145f8: bgez	$r0, . + (8)
L_145fc: shi	$r14, [$sp + #0x16]
L_14600: lhi	$r0, [$r11 + #0x14]
L_14604: lhsi	$r1, [$sp + #0x16]
L_14608: slts45 $r1, $r0
L_1460a: bnezs8 . + (6)
L_1460c: subi45 $r0, #1
L_1460e: ex9.it #147
L_14610: lhi	$r0, [$sp + #0x14]
L_14614: shi	$r0, [$r9 + #0]
L_14618: lhi	$r0, [$sp + #0x16]
L_1461c: shi	$r0, [$r9 + #2]
L_14620: addi45 $r7, #1
L_14622: addi	$r9, $r9, #0x4c
L_14626: addi	$r10, $r10, #0x4c
L_1462a: j	. + (-412)
L_1462e: ex9.it #97
L_14630: bnec	$r0, #1, . + (38)
L_14634: mov55 $r0, $r6
L_14636: movi55 $r1, #0
L_14638: j8 . + (22)
L_1463a: lwi450 $r2, [$r0]
L_1463c: subi45 $r2, #4
L_1463e: slti45 $r2, #2
L_14640: beqzs8 . + (10)
L_14642: lhi333 $r2, [$r0 + #4]
L_14644: shi333 $r2, [$r0 + #8]
L_14646: lhi333 $r2, [$r0 + #6]
L_14648: shi333 $r2, [$r0 + #0xa]
L_1464a: addi45 $r1, #1
L_1464c: ex9.it #214
L_1464e: lbi	$r2, [$r11 + #9]
L_14652: slt45 $r1, $r2
L_14654: bnezs8 . + (-26)
L_14656: lbi	$r4, [$r11 + #9]
L_1465a: lhsi.gp	$r3, [ + #-1944]
L_1465e: lhsi.gp	$r2, [ + #-1942]
L_14662: lwi.gp	$r1, [ + #-6236]
L_14666: movi55 $r0, #0
L_14668: j8 . + (24)
L_1466a: lwi450 $r5, [$r6]
L_1466c: bnec	$r5, #5, . + (14)
L_14670: lhsi	$r3, [$r6 + #8]
L_14674: lhsi	$r2, [$r6 + #0xa]
L_14678: movi55 $r1, #0
L_1467a: addi45 $r0, #1
L_1467c: addi	$r6, $r6, #0x4c
L_14680: slt45 $r0, $r4
L_14682: bnezs8 . + (-24)
L_14684: shi.gp	$r3, [ + #-1944]
L_14688: shi.gp	$r2, [ + #-1942]
L_1468c: swi.gp	$r1, [ + #-6236]
L_14690: pop25 $r14, #24
L_14692: push25 $r10, #8
L_14694: swi37.sp $r4, [ + #4]
L_14696: seh	$r17, $r4
L_1469a: srai	$r16, $r4, #0x10
L_1469e: lbi	$r4, [$r0 + #0x5a]
L_146a2: lhsi	$r6, [$r1 + #0]
L_146a6: xlsb33 $r4, $r4
L_146a8: ex9.it #76
L_146aa: movi55 $r18, #1
L_146ac: beqz38 $r4, . + (14)
L_146ae: shi.gp	$r6, [ + #-3764]
L_146b2: shi.gp	$r1, [ + #-3766]
L_146b6: ex9.it #305
L_146b8: j8 . + (48)
L_146ba: lwi450 $r4, [$r0]
L_146bc: bnec	$r4, #3, . + (52)
L_146c0: lwi.gp	$r7, [ + #-3776]
L_146c4: lwi.gp	$r4, [ + #-3772]
L_146c8: add45 $r1, $r7
L_146ca: lbi.gp	$r7, [ + #-3768]
L_146ce: add45 $r6, $r4
L_146d0: addi45 $r7, #1
L_146d2: zeb33 $r7, $r7
L_146d4: slti45 $r7, #0xf
L_146d6: ex9.it #305
L_146d8: ex9.it #488
L_146da: sbi.gp	$r7, [ + #-3768]
L_146de: bnezs8 . + (140)
L_146e0: divsr	$r6, $r6, $r6, $r7
L_146e4: ex9.it #305
L_146e6: ex9.it #232
L_146e8: ex9.it #488
L_146ea: sbi.gp	$r18, [ + #-3768]
L_146ee: j8 . + (124)
L_146f0: bnec	$r4, #1, . + (118)
L_146f4: lwi.gp	$r7, [ + #-1768]
L_146f8: lhsi.gp	$r19, [ + #-3764]
L_146fc: lbi333 $r4, [$r7 + #0]
L_146fe: lhsi.gp	$r18, [ + #-3766]
L_14702: bnec	$r4, #1, . + (204)
L_14706: sub	$r4, $r19, $r6
L_1470a: abs	$r15, $r4
L_1470e: lhi	$r9, [$r7 + #8]
L_14712: sub	$r4, $r18, $r1
L_14716: abs	$r4, $r4
L_1471a: add45 $r4, $r15
L_1471c: slli	$r9, $r9, #1
L_14720: slt45 $r9, $r4
L_14722: beqzs8 . + (172)
L_14724: lhi	$r9, [$r7 + #6]
L_14728: slli	$r7, $r9, #1
L_1472c: slts45 $r19, $r7
L_1472e: beqzs8 . + (6)
L_14730: slts45 $r6, $r7
L_14732: beqzs8 . + (196)
L_14734: lwi.gp	$r10, [ + #-1764]
L_14738: lhi	$r4, [$r10 + #0x12]
L_1473c: subi45 $r4, #1
L_1473e: sub45 $r4, $r9
L_14740: slli333 $r4, $r4, #1
L_14742: slts45 $r4, $r19
L_14744: beqzs8 . + (6)
L_14746: slts45 $r4, $r6
L_14748: beqzs8 . + (174)
L_1474a: slts45 $r18, $r7
L_1474c: beqzs8 . + (6)
L_1474e: slts45 $r1, $r7
L_14750: beqzs8 . + (166)
L_14752: lhi	$r7, [$r10 + #0x14]
L_14756: subi45 $r7, #1
L_14758: sub45 $r7, $r9
L_1475a: slli333 $r7, $r7, #1
L_1475c: slts45 $r7, $r18
L_1475e: beqzs8 . + (112)
L_14760: slts45 $r7, $r1
L_14762: beqzs8 . + (148)
L_14764: j8 . + (106)
L_14766: ex9.it #305
L_14768: ex9.it #488
L_1476a: bnec	$r5, #1, . + (86)
L_1476e: lwi450 $r0, [$r0]
L_14770: bnec	$r0, #1, . + (80)
L_14774: lwi.gp	$r1, [ + #-1768]
L_14778: lhi333 $r0, [$r1 + #0xa]
L_1477a: slt45 $r3, $r0
L_1477c: beqzs8 . + (68)
L_1477e: lwi.gp	$r0, [ + #-3772]
L_14782: lwi.gp	$r4, [ + #-3776]
L_14786: sub	$r3, $r0, $r17
L_1478a: abs	$r15, $r3
L_1478e: lhi333 $r5, [$r1 + #0xc]
L_14790: sub	$r3, $r4, $r16
L_14794: abs	$r3, $r3
L_14798: add45 $r3, $r15
L_1479a: slli333 $r5, $r5, #1
L_1479c: slts45 $r3, $r5
L_1479e: beqzs8 . + (34)
L_147a0: lhi333 $r3, [$r1 + #0xe]
L_147a2: subi333 $r5, $r3, #1
L_147a4: maddr32	$r0, $r5, $r17
L_147a8: maddr32	$r4, $r5, $r16
L_147ac: srli	$r1, $r3, #1
L_147b0: add45 $r0, $r1
L_147b2: add45 $r1, $r4
L_147b4: divsr	$r0, $r0, $r0, $r3
L_147b8: ex9.it #279
L_147ba: swi.gp	$r0, [ + #-3772]
L_147be: ex9.it #488
L_147c0: lwi.gp	$r0, [ + #-3772]
L_147c4: shi333 $r0, [$r2 + #0]
L_147c6: lwi.gp	$r0, [ + #-3776]
L_147ca: shi333 $r0, [$r2 + #2]
L_147cc: pop25 $r10, #8
L_147ce: lwi.gp	$r7, [ + #-3776]
L_147d2: lwi.gp	$r4, [ + #-3772]
L_147d6: add45 $r1, $r7
L_147d8: lbi.gp	$r7, [ + #-3768]
L_147dc: add45 $r6, $r4
L_147de: addi45 $r7, #1
L_147e0: zeb33 $r7, $r7
L_147e2: divsr	$r6, $r6, $r6, $r7
L_147e6: sbi.gp	$r7, [ + #-3768]
L_147ea: divsr	$r7, $r7, $r1, $r7
L_147ee: ex9.it #305
L_147f0: swi.gp	$r7, [ + #-3776]
L_147f4: j8 . + (-138)
L_147f6: swi.gp	$r19, [ + #-3772]
L_147fa: swi.gp	$r18, [ + #-3776]
L_147fe: j8 . + (-148)
L_14800: push25 $r8, #0
L_14802: lwi	$r6, [$r0 + #0x44]
L_14806: ex9.it #333
L_14808: ex9.it #248
L_1480a: and33 $r5, $r6
L_1480c: movi	$r6, #0x10000
L_14810: lhsi	$r7, [$r1 + #0]
L_14814: lhsi	$r4, [$r1 + #2]
L_14818: lwi.gp	$r3, [ + #-3784]
L_1481c: lwi.gp	$r1, [ + #-3780]
L_14820: beqs38 $r6, . + (8)
L_14822: lwi450 $r0, [$r0]
L_14824: bnec	$r0, #1, . + (20)
L_14828: movi55 $r0, #0
L_1482a: swi.gp	$r7, [ + #-3780]
L_1482e: swi.gp	$r4, [ + #-3784]
L_14832: swi.gp	$r0, [ + #-3788]
L_14836: j8 . + (116)
L_14838: seh33 $r6, $r1
L_1483a: seh33 $r5, $r3
L_1483c: sub	$r17, $r7, $r6
L_14840: sub	$r16, $r4, $r5
L_14844: lwi.gp	$r1, [ + #-1772]
L_14848: abs	$r0, $r17
L_1484c: abs	$r3, $r16
L_14850: add45 $r3, $r0
L_14852: lhi	$r0, [$r1 + #0x1c]
L_14856: swi.gp	$r3, [ + #-3788]
L_1485a: slts45 $r3, $r0
L_1485c: beqzs8 . + (70)
L_1485e: lbi	$r1, [$r1 + #0x1e]
L_14862: mov55 $r7, $r0
L_14864: subi333 $r4, $r1, #1
L_14866: maddr32	$r7, $r4, $r3
L_1486a: mul33 $r0, $r1
L_1486c: mov55 $r3, $r7
L_1486e: mul	$r1, $r7, $r17
L_14872: srai	$r4, $r0, #1
L_14876: bltz	$r17, . + (8)
L_1487a: add45 $r1, $r4
L_1487c: j8 . + (4)
L_1487e: sub45 $r1, $r4
L_14880: divsr	$r1, $r1, $r1, $r0
L_14884: add45 $r1, $r6
L_14886: swi.gp	$r1, [ + #-3780]
L_1488a: mul	$r1, $r3, $r16
L_1488e: bltz	$r16, . + (8)
L_14892: add45 $r1, $r4
L_14894: j8 . + (4)
L_14896: sub45 $r1, $r4
L_14898: ex9.it #194
L_1489a: add45 $r0, $r5
L_1489c: swi.gp	$r0, [ + #-3784]
L_148a0: j8 . + (10)
L_148a2: swi.gp	$r7, [ + #-3780]
L_148a6: swi.gp	$r4, [ + #-3784]
L_148aa: lwi.gp	$r0, [ + #-3780]
L_148ae: shi333 $r0, [$r2 + #0]
L_148b0: lwi.gp	$r0, [ + #-3784]
L_148b4: shi333 $r0, [$r2 + #2]
L_148b6: pop25 $r8, #0
L_148b8: push25 $r8, #0
L_148ba: lhsi	$r6, [$r1 + #0]
L_148be: lhsi	$r5, [$r1 + #2]
L_148c2: lwi.gp	$r7, [ + #-1768]
L_148c6: ex9.it #272
L_148c8: lhsi.gp	$r18, [ + #-3792]
L_148cc: lhsi.gp	$r16, [ + #-3796]
L_148d0: lhi	$r4, [$r7 + #0x1c]
L_148d4: beqz38 $r1, . + (28)
L_148d6: lwi.gp	$r3, [ + #-5288]
L_148da: movi55 $r1, #0xa
L_148dc: divr	$r3, $r3, $r3, $r1
L_148e0: lwi	$r1, [$r7 + #0x24]
L_148e4: zeh33 $r3, $r3
L_148e6: ex9.it #279
L_148e8: lwi	$r3, [$r7 + #0x28]
L_148ec: mul33 $r1, $r3
L_148ee: j8 . + (34)
L_148f0: addi	$r1, $r6, #-20
L_148f4: zeh33 $r1, $r1
L_148f6: slti	$r15, $r1, #0xc58
L_148fa: beqzs8 . + (20)
L_148fc: addi	$r1, $r5, #-20
L_14900: zeh33 $r1, $r1
L_14902: slti	$r15, $r1, #0x13d8
L_14906: beqzs8 . + (8)
L_14908: lbi	$r1, [$r7 + #0x1e]
L_1490c: j8 . + (4)
L_1490e: ex9.it #179
L_14910: lwi450 $r0, [$r0]
L_14912: bnec	$r0, #1, . + (20)
L_14916: movi55 $r0, #0
L_14918: swi.gp	$r6, [ + #-3792]
L_1491c: swi.gp	$r5, [ + #-3796]
L_14920: swi.gp	$r0, [ + #-3800]
L_14924: j8 . + (100)
L_14926: sub	$r17, $r6, $r18
L_1492a: sub	$r7, $r5, $r16
L_1492e: abs	$r0, $r17
L_14932: abs	$r3, $r7
L_14936: add45 $r3, $r0
L_14938: slli333 $r4, $r4, #1
L_1493a: slts45 $r3, $r4
L_1493c: swi.gp	$r3, [ + #-3800]
L_14940: beqzs8 . + (64)
L_14942: subi333 $r0, $r1, #1
L_14944: mov55 $r5, $r4
L_14946: maddr32	$r5, $r0, $r3
L_1494a: mul33 $r1, $r4
L_1494c: mov55 $r3, $r5
L_1494e: mul	$r0, $r5, $r17
L_14952: srai	$r4, $r1, #1
L_14956: bltz	$r17, . + (8)
L_1495a: add45 $r0, $r4
L_1495c: j8 . + (4)
L_1495e: sub45 $r0, $r4
L_14960: ex9.it #78
L_14962: add45 $r0, $r18
L_14964: swi.gp	$r0, [ + #-3792]
L_14968: mul	$r0, $r3, $r7
L_1496c: bltz	$r7, . + (8)
L_14970: add45 $r0, $r4
L_14972: j8 . + (4)
L_14974: sub45 $r0, $r4
L_14976: ex9.it #204
L_14978: add45 $r1, $r16
L_1497a: swi.gp	$r1, [ + #-3796]
L_1497e: j8 . + (10)
L_14980: swi.gp	$r6, [ + #-3792]
L_14984: swi.gp	$r5, [ + #-3796]
L_14988: lwi.gp	$r0, [ + #-3792]
L_1498c: shi333 $r0, [$r2 + #0]
L_1498e: lwi.gp	$r0, [ + #-3796]
L_14992: shi333 $r0, [$r2 + #2]
L_14994: pop25 $r8, #0
L_14996: push25 $r8, #0
L_14998: lwi.gp	$r6, [ + #-1768]
L_1499c: ex9.it #184
L_1499e: lbi	$r7, [$r6 + #0x10]
L_149a2: lhsi	$r16, [$r1 + #2]
L_149a6: bnez38 $r7, . + (4)
L_149a8: movi55 $r7, #1
L_149aa: ex9.it #272
L_149ac: beqz38 $r1, . + (34)
L_149ae: lwi.gp	$r1, [ + #-5288]
L_149b2: movi55 $r4, #0xa
L_149b4: divr	$r1, $r1, $r1, $r4
L_149b8: lwi	$r4, [$r6 + #0x24]
L_149bc: zeh33 $r1, $r1
L_149be: divr	$r1, $r1, $r4, $r1
L_149c2: lwi	$r4, [$r6 + #0x28]
L_149c6: mul33 $r4, $r1
L_149c8: swi.gp	$r4, [ + #-3828]
L_149cc: j8 . + (12)
L_149ce: lhi	$r1, [$r6 + #0x12]
L_149d2: slli333 $r1, $r1, #1
L_149d4: swi.gp	$r1, [ + #-3828]
L_149d8: lwi450 $r1, [$r0]
L_149da: bnec	$r1, #1, . + (38)
L_149de: movi55 $r0, #0
L_149e0: sbi.gp	$r0, [ + #-3844]
L_149e4: sbi.gp	$r0, [ + #-3846]
L_149e8: lhi	$r0, [$r6 + #0x12]
L_149ec: shi.gp	$r3, [ + #-3832]
L_149f0: slli333 $r0, $r0, #1
L_149f2: shi.gp	$r16, [ + #-3834]
L_149f6: sbi.gp	$r1, [ + #-3830]
L_149fa: swi.gp	$r0, [ + #-3828]
L_149fe: j8 . + (170)
L_14a00: lhsi.gp	$r17, [ + #-3834]
L_14a04: lhsi.gp	$r19, [ + #-3832]
L_14a08: sub	$r4, $r17, $r16
L_14a0c: sub	$r0, $r19, $r3
L_14a10: mul33 $r4, $r4
L_14a12: maddr32	$r4, $r0, $r0
L_14a16: lwi.gp	$r0, [ + #-3828]
L_14a1a: movi	$r18, #0x2cadc
L_14a1e: slt45 $r4, $r0
L_14a20: lbi.gp	$r5, [ + #-3830]
L_14a24: movi55 $r0, #0
L_14a26: bnezs8 . + (4)
L_14a28: movi55 $r0, #1
L_14a2a: sb	$r0, [$r18 + ($r5 << #0)]
L_14a2e: subi333 $r0, $r7, #1
L_14a30: bnes38 $r0, . + (16)
L_14a32: movi55 $r0, #0
L_14a34: sbi.gp	$r0, [ + #-3830]
L_14a38: movi55 $r0, #1
L_14a3a: sbi.gp	$r0, [ + #-3846]
L_14a3e: j8 . + (8)
L_14a40: addi45 $r5, #1
L_14a42: sbi.gp	$r5, [ + #-3830]
L_14a46: lbi.gp	$r1, [ + #-3846]
L_14a4a: bnez38 $r1, . + (24)
L_14a4c: lbi.gp	$r4, [ + #-3830]
L_14a50: mov55 $r0, $r1
L_14a52: slt45 $r0, $r4
L_14a54: beqzs8 . + (98)
L_14a56: lb	$r5, [$r18 + ($r0 << #0)]
L_14a5a: bnez38 $r5, . + (4)
L_14a5c: addi45 $r1, #1
L_14a5e: addi45 $r0, #1
L_14a60: j8 . + (-14)
L_14a62: movi55 $r4, #0
L_14a64: mov55 $r0, $r4
L_14a66: slt45 $r0, $r7
L_14a68: beqzs8 . + (14)
L_14a6a: lb	$r5, [$r18 + ($r0 << #0)]
L_14a6e: bnez38 $r5, . + (4)
L_14a70: addi45 $r4, #1
L_14a72: addi45 $r0, #1
L_14a74: j8 . + (-14)
L_14a76: bnec	$r1, #1, . + (18)
L_14a7a: slts45 $r4, $r7
L_14a7c: beqzs8 . + (12)
L_14a7e: lhi	$r0, [$r6 + #0x14]
L_14a82: slli333 $r0, $r0, #1
L_14a84: swi.gp	$r0, [ + #-3828]
L_14a88: sub333 $r1, $r7, $r4
L_14a8a: mul33 $r3, $r1
L_14a8c: maddr32	$r3, $r4, $r19
L_14a90: mul	$r1, $r1, $r16
L_14a94: maddr32	$r1, $r4, $r17
L_14a98: divsr	$r3, $r3, $r3, $r7
L_14a9c: shi.gp	$r3, [ + #-3832]
L_14aa0: divsr	$r0, $r0, $r1, $r7
L_14aa4: shi.gp	$r0, [ + #-3834]
L_14aa8: lhi.gp	$r0, [ + #-3832]
L_14aac: shi333 $r0, [$r2 + #0]
L_14aae: lhi.gp	$r0, [ + #-3834]
L_14ab2: shi333 $r0, [$r2 + #2]
L_14ab4: pop25 $r8, #0
L_14ab6: sub333 $r5, $r7, $r4
L_14ab8: sub45 $r4, $r1
L_14aba: add45 $r5, $r1
L_14abc: mul33 $r3, $r4
L_14abe: maddr32	$r3, $r5, $r19
L_14ac2: mul	$r0, $r4, $r16
L_14ac6: maddr32	$r0, $r5, $r17
L_14aca: divsr	$r3, $r3, $r3, $r7
L_14ace: shi.gp	$r3, [ + #-3832]
L_14ad2: divsr	$r0, $r0, $r0, $r7
L_14ad6: j8 . + (-50)
L_14ad8: push25 $r8, #0
L_14ada: ex9.it #457
L_14adc: lhsi	$r4, [$r1 + #2]
L_14ae0: ex9.it #272
L_14ae2: lwi.gp	$r3, [ + #-1768]
L_14ae6: beqz38 $r1, . + (10)
L_14ae8: lbi	$r6, [$r3 + #0x22]
L_14aec: movpi45 $r1, #0x14
L_14aee: j8 . + (44)
L_14af0: addi	$r1, $r5, #-50
L_14af4: zeh33 $r1, $r1
L_14af6: slti	$r15, $r1, #0xc1c
L_14afa: beqzs8 . + (14)
L_14afc: addi	$r1, $r4, #-50
L_14b00: zeh33 $r1, $r1
L_14b02: slti	$r15, $r1, #0x139c
L_14b06: bnezs8 . + (12)
L_14b08: lhi	$r1, [$r3 + #0x20]
L_14b0c: addi45 $r1, #3
L_14b0e: zeb33 $r1, $r1
L_14b10: j8 . + (6)
L_14b12: lbi	$r1, [$r3 + #0x20]
L_14b16: lbi	$r6, [$r3 + #0x22]
L_14b1a: lwi450 $r0, [$r0]
L_14b1c: bnec	$r0, #1, . + (14)
L_14b20: shi.gp	$r5, [ + #-3848]
L_14b24: shi.gp	$r4, [ + #-3850]
L_14b28: j8 . + (90)
L_14b2a: lhsi.gp	$r3, [ + #-3850]
L_14b2e: lhsi.gp	$r0, [ + #-3848]
L_14b32: sub45 $r3, $r4
L_14b34: sub45 $r0, $r5
L_14b36: mul33 $r3, $r3
L_14b38: maddr32	$r3, $r0, $r0
L_14b3c: slli333 $r0, $r1, #1
L_14b3e: mul33 $r0, $r0
L_14b40: lbi.gp	$r7, [ + #-3851]
L_14b44: slt45 $r3, $r0
L_14b46: bnec	$r7, #1, . + (26)
L_14b4a: bnezs8 . + (60)
L_14b4c: movi55 $r0, #0
L_14b4e: sbi.gp	$r0, [ + #-3851]
L_14b52: shi.gp	$r5, [ + #-3848]
L_14b56: shi.gp	$r4, [ + #-3850]
L_14b5a: sbi.gp	$r0, [ + #-3852]
L_14b5e: j8 . + (40)
L_14b60: beqzs8 . + (16)
L_14b62: lbi.gp	$r0, [ + #-3852]
L_14b66: slt45 $r0, $r6
L_14b68: beqzs8 . + (8)
L_14b6a: addi45 $r0, #1
L_14b6c: sbi.gp	$r0, [ + #-3852]
L_14b70: lbi.gp	$r0, [ + #-3852]
L_14b74: shi.gp	$r5, [ + #-3848]
L_14b78: slt45 $r0, $r6
L_14b7a: shi.gp	$r4, [ + #-3850]
L_14b7e: bnezs8 . + (8)
L_14b80: movi55 $r0, #1
L_14b82: sbi.gp	$r0, [ + #-3851]
L_14b86: lhi.gp	$r0, [ + #-3848]
L_14b8a: shi333 $r0, [$r2 + #0]
L_14b8c: lhi.gp	$r0, [ + #-3850]
L_14b90: shi333 $r0, [$r2 + #2]
L_14b92: pop25 $r8, #0
L_14b94: push25 $r6, #0
L_14b96: lwi450 $r4, [$r0]
L_14b98: ex9.it #184
L_14b9a: ex9.it #76
L_14b9c: beqc	$r4, #5, . + (6)
L_14ba0: bnez38 $r4, . + (20)
L_14ba2: movpi45 $r0, #0x1e
L_14ba4: swi.gp	$r0, [ + #-3856]
L_14ba8: movi55 $r0, #-1
L_14baa: shi.gp	$r0, [ + #-3858]
L_14bae: shi.gp	$r0, [ + #-3860]
L_14bb2: j8 . + (110)
L_14bb4: bnec	$r4, #1, . + (44)
L_14bb8: lbi	$r0, [$r0 + #0x45]
L_14bbc: bnez38 $r0, . + (12)
L_14bbe: lwi.gp	$r0, [ + #-3856]
L_14bc2: bnez38 $r0, . + (20)
L_14bc4: movi55 $r0, #5
L_14bc6: j8 . + (4)
L_14bc8: movpi45 $r0, #0x1e
L_14bca: swi.gp	$r0, [ + #-3856]
L_14bce: shi.gp	$r3, [ + #-3858]
L_14bd2: shi.gp	$r1, [ + #-3860]
L_14bd6: lhsi.gp	$r3, [ + #-3858]
L_14bda: lhsi.gp	$r1, [ + #-3860]
L_14bde: j8 . + (66)
L_14be0: lwi.gp	$r4, [ + #-3856]
L_14be4: beqz38 $r4, . + (8)
L_14be6: subi45 $r4, #1
L_14be8: swi.gp	$r4, [ + #-3856]
L_14bec: lwi.gp	$r4, [ + #-3856]
L_14bf0: beqz38 $r4, . + (48)
L_14bf2: lbi	$r0, [$r0 + #0x45]
L_14bf6: bnec	$r0, #1, . + (12)
L_14bfa: shi.gp	$r3, [ + #-3858]
L_14bfe: shi.gp	$r1, [ + #-3860]
L_14c02: lhsi.gp	$r5, [ + #-3860]
L_14c06: lhsi.gp	$r4, [ + #-3858]
L_14c0a: sub333 $r0, $r5, $r1
L_14c0c: sub333 $r6, $r4, $r3
L_14c0e: mul33 $r0, $r0
L_14c10: maddr32	$r0, $r6, $r6
L_14c14: sltsi	$r15, $r0, #0x3e8
L_14c18: cmovn	$r1, $r5, $r15
L_14c1c: cmovn	$r3, $r4, $r15
L_14c20: shi333 $r3, [$r2 + #0]
L_14c22: shi333 $r1, [$r2 + #2]
L_14c24: pop25 $r6, #0
L_14c26: push25 $r10, #0
L_14c28: lwi.gp	$r9, [ + #-1772]
L_14c2c: lhsi	$r4, [$r1 + #0]
L_14c30: lbi	$r5, [$r9 + #0x16]
L_14c34: lhsi	$r3, [$r0 + #0x10]
L_14c38: lhsi	$r7, [$r0 + #0x12]
L_14c3c: ex9.it #76
L_14c3e: movi55 $r17, #1
L_14c40: beqz38 $r5, . + (12)
L_14c42: slti45 $r5, #9
L_14c44: movi55 $r17, #8
L_14c46: beqzs8 . + (6)
L_14c48: zeh	$r17, $r5
L_14c4c: lbi	$r5, [$r9 + #0x17]
L_14c50: movi55 $r16, #1
L_14c52: beqz38 $r5, . + (12)
L_14c54: slti45 $r5, #9
L_14c56: movi55 $r16, #8
L_14c58: beqzs8 . + (6)
L_14c5a: zeh	$r16, $r5
L_14c5e: lwi450 $r10, [$r0]
L_14c60: beqc	$r10, #1, . + (20)
L_14c64: lwi	$r0, [$r0 + #0x44]
L_14c68: ex9.it #333
L_14c6a: ex9.it #248
L_14c6c: and33 $r5, $r0
L_14c6e: movi	$r0, #0x10000
L_14c72: bnes38 $r0, . + (46)
L_14c74: movi55 $r0, #0
L_14c76: swi.gp	$r0, [ + #-3864]
L_14c7a: swi.gp	$r0, [ + #-3868]
L_14c7e: movi55 $r0, #1
L_14c80: swi.gp	$r17, [ + #-3872]
L_14c84: swi.gp	$r17, [ + #-3876]
L_14c88: shi.gp	$r3, [ + #-3908]
L_14c8c: shi.gp	$r7, [ + #-3906]
L_14c90: shi.gp	$r4, [ + #-3940]
L_14c94: shi.gp	$r1, [ + #-3938]
L_14c98: swi.gp	$r0, [ + #-3944]
L_14c9c: j	. + (352)
L_14ca0: bnec	$r10, #3, . + (8)
L_14ca4: j	. + (340)
L_14ca8: lwi.gp	$r0, [ + #-3944]
L_14cac: addi	$r18, $r16, #-1
L_14cb0: addi45 $r0, #1
L_14cb2: min	$r0, $r0, $r16
L_14cb6: swi.gp	$r0, [ + #-3944]
L_14cba: lwi.gp	$r0, [ + #-3868]
L_14cbe: slts45 $r0, $r18
L_14cc0: beqzs8 . + (6)
L_14cc2: addi45 $r0, #1
L_14cc4: j8 . + (4)
L_14cc6: movi55 $r0, #0
L_14cc8: swi.gp	$r0, [ + #-3868]
L_14ccc: lwi.gp	$r0, [ + #-3864]
L_14cd0: lwi.gp	$r5, [ + #-3868]
L_14cd4: bnez38 $r0, . + (12)
L_14cd6: bne	$r5, $r18, . + (10)
L_14cda: movi55 $r0, #1
L_14cdc: swi.gp	$r0, [ + #-3864]
L_14ce0: movi	$r6, #0x2ca9c
L_14ce4: slli333 $r0, $r5, #2
L_14ce6: add	$r19, $r6, $r0
L_14cea: shi	$r7, [$r19 + #2]
L_14cee: movi	$r19, #0x2ca7c
L_14cf2: sh	$r4, [$r19 + ($r5 << #2)]
L_14cf6: add45 $r0, $r19
L_14cf8: lwi.gp	$r4, [ + #-3872]
L_14cfc: sh	$r3, [$r6 + ($r5 << #2)]
L_14d00: shi333 $r1, [$r0 + #2]
L_14d02: swi.gp	$r4, [ + #-3876]
L_14d06: blez	$r5, . + (18)
L_14d0a: subi333 $r1, $r5, #1
L_14d0c: lhs	$r0, [$r6 + ($r1 << #2)]
L_14d10: add_slli	$r6, $r6, $r1, #2
L_14d14: sub333 $r3, $r0, $r3
L_14d16: j8 . + (12)
L_14d18: lhs	$r0, [$r6 + ($r18 << #2)]
L_14d1c: add_slli	$r6, $r6, $r18, #2
L_14d20: sub333 $r3, $r0, $r3
L_14d22: lhsi	$r1, [$r6 + #2]
L_14d26: abs	$r0, $r3
L_14d2a: sub45 $r1, $r7
L_14d2c: ex9.it #207
L_14d2e: lhi	$r6, [$r9 + #0x1a]
L_14d32: add45 $r0, $r1
L_14d34: slt45 $r6, $r0
L_14d36: mov55 $r1, $r6
L_14d38: bnezs8 . + (12)
L_14d3a: lhi	$r1, [$r9 + #0x18]
L_14d3e: slt45 $r0, $r1
L_14d40: cmovz	$r1, $r0, $r15
L_14d44: lwi.gp	$r0, [ + #-3864]
L_14d48: bnez38 $r0, . + (8)
L_14d4a: swi.gp	$r17, [ + #-3872]
L_14d4e: j8 . + (32)
L_14d50: beqc	$r10, #4, . + (30)
L_14d54: lhi	$r0, [$r9 + #0x18]
L_14d58: sub	$r3, $r16, $r17
L_14d5c: sub45 $r1, $r0
L_14d5e: mul33 $r1, $r3
L_14d60: sub333 $r0, $r6, $r0
L_14d62: divr	$r0, $r0, $r1, $r0
L_14d66: sub	$r0, $r16, $r0
L_14d6a: swi.gp	$r0, [ + #-3872]
L_14d6e: lwi.gp	$r0, [ + #-3872]
L_14d72: movi55 $r1, #3
L_14d74: mov55 $r3, $r4
L_14d76: maddr32	$r3, $r1, $r0
L_14d7a: movi55 $r1, #4
L_14d7c: mov55 $r0, $r3
L_14d7e: addi45 $r0, #2
L_14d80: ex9.it #78
L_14d82: slts45 $r4, $r0
L_14d84: bnezs8 . + (8)
L_14d86: swi.gp	$r0, [ + #-3872]
L_14d8a: j8 . + (8)
L_14d8c: addi45 $r4, #1
L_14d8e: swi.gp	$r4, [ + #-3872]
L_14d92: lwi.gp	$r0, [ + #-3872]
L_14d96: slts45 $r0, $r17
L_14d98: beqzs8 . + (8)
L_14d9a: swi.gp	$r17, [ + #-3872]
L_14d9e: j8 . + (10)
L_14da0: slts45 $r16, $r0
L_14da2: beqzs8 . + (6)
L_14da4: swi.gp	$r16, [ + #-3872]
L_14da8: lwi.gp	$r0, [ + #-3944]
L_14dac: lwi.gp	$r1, [ + #-3872]
L_14db0: slts45 $r0, $r1
L_14db2: beqzs8 . + (6)
L_14db4: swi.gp	$r0, [ + #-3872]
L_14db8: movi55 $r1, #0
L_14dba: lwi.gp	$r0, [ + #-3872]
L_14dbe: mov55 $r6, $r1
L_14dc0: mov55 $r4, $r1
L_14dc2: beq	$r1, $r0, . + (32)
L_14dc6: lhs	$r3, [$r19 + ($r5 << #2)]
L_14dca: add45 $r4, $r3
L_14dcc: add_slli	$r3, $r19, $r5, #2
L_14dd0: lhsi	$r3, [$r3 + #2]
L_14dd4: add45 $r6, $r3
L_14dd6: beqz38 $r5, . + (6)
L_14dd8: subi45 $r5, #1
L_14dda: j8 . + (4)
L_14ddc: mov55 $r5, $r18
L_14dde: addi45 $r1, #1
L_14de0: j8 . + (-30)
L_14de2: movi55 $r3, #2
L_14de4: divsr	$r0, $r0, $r1, $r3
L_14de8: add45 $r4, $r0
L_14dea: add45 $r0, $r6
L_14dec: divsr	$r4, $r4, $r4, $r1
L_14df0: ex9.it #204
L_14df2: seh33 $r4, $r4
L_14df4: seh33 $r1, $r1
L_14df6: j8 . + (6)
L_14df8: movi55 $r1, #0
L_14dfa: mov55 $r4, $r1
L_14dfc: shi333 $r4, [$r2 + #0]
L_14dfe: shi333 $r1, [$r2 + #2]
L_14e00: pop25 $r10, #0
L_14e02: push25 $r14, #88
L_14e04: mov55 $r6, $r0
L_14e06: ex9.it #333
L_14e08: lwi	$r0, [$r0 + #0x44]
L_14e0c: ex9.it #248
L_14e0e: and33 $r5, $r0
L_14e10: movi	$r0, #0x10000
L_14e14: bnes38 $r0, . + (8)
L_14e16: movi55 $r0, #-1
L_14e18: swi.gp	$r0, [ + #-6240]
L_14e1c: lwi.gp	$r1, [ + #-1772]
L_14e20: lwi.gp	$r0, [ + #-6240]
L_14e24: lhi333 $r1, [$r1 + #0xa]
L_14e26: slt45 $r0, $r1
L_14e28: beqzs8 . + (8)
L_14e2a: addi45 $r0, #1
L_14e2c: swi.gp	$r0, [ + #-6240]
L_14e30: lwi450 $r0, [$r6]
L_14e32: beqz	$r0, . + (648)
L_14e36: bnec	$r0, #5, . + (8)
L_14e3a: j	. + (640)
L_14e3e: lbi.gp	$r0, [ + #0x1cc]
L_14e42: beqz38 $r0, . + (24)
L_14e44: lbi.gp	$r0, [ + #0x1ce]
L_14e48: beqz38 $r0, . + (18)
L_14e4a: lbi.gp	$r0, [ + #0x1cd]
L_14e4e: beqc	$r0, #0x1f, . + (12)
L_14e52: lbi.gp	$r0, [ + #0x1cf]
L_14e56: bnec	$r0, #0x31, . + (20)
L_14e5a: lbi	$r0, [$r6 + #0x45]
L_14e5e: bnez38 $r0, . + (12)
L_14e60: lbi	$r0, [$r6 + #0x5a]
L_14e64: xlsb33 $r0, $r0
L_14e66: beqz	$r0, . + (608)
L_14e6a: lhi	$r1, [$r6 + #0x30]
L_14e6e: lhi	$r0, [$r6 + #0x1c]
L_14e72: movi	$r9, #0x64
L_14e76: sub45 $r0, $r1
L_14e78: seh33 $r0, $r0
L_14e7a: shi	$r0, [$sp + #0]
L_14e7e: lhi	$r1, [$r6 + #0x32]
L_14e82: lhi	$r0, [$r6 + #0x1e]
L_14e86: movi55 $r10, #1
L_14e88: sub45 $r0, $r1
L_14e8a: seh33 $r0, $r0
L_14e8c: shi	$r0, [$sp + #2]
L_14e90: lhi	$r0, [$sp + #0]
L_14e94: seh33 $r0, $r0
L_14e96: srai45 $r0, #1
L_14e98: shi	$r0, [$sp + #0]
L_14e9c: ex9.it #483
L_14e9e: seh33 $r0, $r0
L_14ea0: srai45 $r0, #1
L_14ea2: shi	$r0, [$sp + #2]
L_14ea6: ex9.it #299
L_14ea8: seh33 $r0, $r1
L_14eaa: ex9.it #299
L_14eac: lhi	$r3, [$sp + #2]
L_14eb0: seh33 $r1, $r1
L_14eb2: lhi	$r2, [$sp + #2]
L_14eb6: mul33 $r1, $r0
L_14eb8: maddr32	$r1, $r3, $r2
L_14ebc: movi55 $r0, #3
L_14ebe: seh33 $r1, $r1
L_14ec0: movi55 $r2, #1
L_14ec2: ifcall9 . + (88)
L_14ec4: mov55 $r7, $r0
L_14ec6: mul	$r0, $r0, $r9
L_14eca: movi	$r1, #0x46
L_14ece: ex9.it #204
L_14ed0: zeh33 $r7, $r7
L_14ed2: mul33 $r7, $r7
L_14ed4: seh33 $r1, $r1
L_14ed6: movi55 $r2, #1
L_14ed8: movi55 $r0, #0
L_14eda: ifcall9 . + (64)
L_14edc: subri	$r1, $r7, #0x1324
L_14ee0: sbi	$r0, [$r6 + #0x78]
L_14ee4: seh33 $r1, $r1
L_14ee6: movi55 $r2, #1
L_14ee8: movi55 $r0, #3
L_14eea: ifcall9 . + (48)
L_14eec: ex9.it #299
L_14eee: max	$r11, $r0, $r10
L_14ef2: seh33 $r1, $r1
L_14ef4: ex9.it #207
L_14ef6: mul	$r1, $r1, $r9
L_14efa: mov55 $r2, $r10
L_14efc: ex9.it #470
L_14efe: movi55 $r0, #2
L_14f00: seh33 $r1, $r1
L_14f02: ifcall9 . + (24)
L_14f04: lhi	$r1, [$sp + #2]
L_14f08: mov55 $r7, $r0
L_14f0a: seh33 $r1, $r1
L_14f0c: ex9.it #207
L_14f0e: mul	$r1, $r1, $r9
L_14f12: movi55 $r0, #2
L_14f14: ex9.it #470
L_14f16: mov55 $r2, $r10
L_14f18: seh33 $r1, $r1
L_14f1a: jal	. + (-7234)
L_14f1e: ex9.it #299
L_14f20: seh33 $r1, $r1
L_14f22: bgtz	$r1, . + (10)
L_14f26: neg33 $r3, $r7
L_14f28: seh33 $r4, $r3
L_14f2a: j8 . + (4)
L_14f2c: mov55 $r4, $r7
L_14f2e: lhi	$r1, [$sp + #2]
L_14f32: shi	$r4, [$r6 + #0x24]
L_14f36: seh33 $r1, $r1
L_14f38: bgtz	$r1, . + (8)
L_14f3c: neg33 $r0, $r0
L_14f3e: seh33 $r0, $r0
L_14f40: movi	$r7, #0x199bc
L_14f44: mov55 $r2, $r7
L_14f46: addri36.sp $r3, #8
L_14f48: lmw.bim $r16, [$r2], $r18, #0x0
L_14f4c: mov55 $r1, $r3
L_14f4e: smw.bim $r16, [$r1], $r18, #0x0
L_14f52: shi	$r0, [$r6 + #0x26]
L_14f56: lhi	$r9, [$r2 + #0]
L_14f5a: shi	$r9, [$r1 + #0]
L_14f5e: movi	$r1, #0x199cc
L_14f62: mov55 $r19, $r1
L_14f64: addri36.sp $r2, #0x18
L_14f66: lmw.bim $r16, [$r19], $r18, #0x0
L_14f6a: mov55 $r5, $r2
L_14f6c: smw.bim $r16, [$r5], $r18, #0x0
L_14f70: lhi	$r19, [$r19 + #0]
L_14f74: shi	$r19, [$r5 + #0]
L_14f78: addri36.sp $r5, #0x28
L_14f7a: lmw.bim $r16, [$r7], $r18, #0x0
L_14f7e: mov55 $r7, $r5
L_14f80: smw.bim $r16, [$r7], $r18, #0x0
L_14f84: shi	$r9, [$r7 + #0]
L_14f88: addri36.sp $r7, #0x38
L_14f8a: lmw.bim $r16, [$r1], $r18, #0x0
L_14f8e: mov55 $r1, $r7
L_14f90: smw.bim $r16, [$r1], $r18, #0x0
L_14f94: shi	$r19, [$r1 + #0]
L_14f98: addi	$r19, $sp, #0x48
L_14f9c: movi	$r1, #0x199dc
L_14fa0: lmw.bim $r16, [$r1], $r18, #0x0
L_14fa4: mov55 $r9, $r19
L_14fa6: smw.bim $r16, [$r9], $r18, #0x0
L_14faa: lhi333 $r1, [$r1 + #0]
L_14fac: shi	$r1, [$r9 + #0]
L_14fb0: lbi	$r1, [$r6 + #0x78]
L_14fb4: slti	$r15, $r1, #0x28
L_14fb8: bnezs8 . + (142)
L_14fba: abs	$r18, $r4
L_14fbe: abs	$r9, $r0
L_14fc2: sub	$r1, $r18, $r9
L_14fc6: seh33 $r1, $r1
L_14fc8: ex9.it #207
L_14fca: seh33 $r1, $r1
L_14fcc: movi55 $r16, #1
L_14fce: lhs	$r17, [$r19 + ($r16 << #1)]
L_14fd2: slts45 $r17, $r1
L_14fd4: beqzs8 . + (10)
L_14fd6: addi45 $r16, #1
L_14fd8: zeh	$r16, $r16
L_14fdc: j8 . + (-14)
L_14fde: movi55 $r17, #0xd
L_14fe0: min	$r16, $r17, $r16
L_14fe4: addi	$r17, $r16, #-1
L_14fe8: slts45 $r9, $r18
L_14fea: zeh	$r17, $r17
L_14fee: beqzs8 . + (6)
L_14ff0: mov55 $r5, $r2
L_14ff2: mov55 $r7, $r3
L_14ff4: lhs	$r18, [$r19 + ($r17 << #1)]
L_14ff8: lhs	$r2, [$r19 + ($r16 << #1)]
L_14ffc: lhs	$r3, [$r7 + ($r17 << #1)]
L_15000: sub45 $r1, $r18
L_15002: lhs	$r7, [$r7 + ($r16 << #1)]
L_15006: sub	$r18, $r2, $r18
L_1500a: lhs	$r2, [$r5 + ($r17 << #1)]
L_1500e: lhs	$r5, [$r5 + ($r16 << #1)]
L_15012: zeh33 $r1, $r1
L_15014: sub45 $r7, $r3
L_15016: sub45 $r5, $r2
L_15018: mul33 $r7, $r1
L_1501a: mul33 $r1, $r5
L_1501c: seh33 $r7, $r7
L_1501e: seh33 $r1, $r1
L_15020: divsr	$r1, $r1, $r1, $r18
L_15024: divsr	$r7, $r7, $r7, $r18
L_15028: add45 $r2, $r1
L_1502a: add45 $r3, $r7
L_1502c: seh33 $r1, $r3
L_1502e: seh33 $r2, $r2
L_15030: mul	$r3, $r4, $r1
L_15034: mul33 $r0, $r2
L_15036: ex9.it #140
L_15038: divsr	$r3, $r3, $r3, $r1
L_1503c: ex9.it #78
L_1503e: shi	$r3, [$r6 + #0x24]
L_15042: shi	$r0, [$r6 + #0x26]
L_15046: lhsi	$r15, [$r6 + #0x24]
L_1504a: sltsi	$r15, $r15, #0x3d
L_1504e: bnezs8 . + (10)
L_15050: movi	$r0, #0x3c
L_15054: shi	$r0, [$r6 + #0x24]
L_15058: lhsi	$r15, [$r6 + #0x24]
L_1505c: sltsi	$r15, $r15, #-60
L_15060: beqzs8 . + (10)
L_15062: movi	$r0, #-60
L_15066: shi	$r0, [$r6 + #0x24]
L_1506a: lhsi	$r15, [$r6 + #0x26]
L_1506e: sltsi	$r15, $r15, #0x3d
L_15072: bnezs8 . + (10)
L_15074: movi	$r0, #0x3c
L_15078: shi	$r0, [$r6 + #0x26]
L_1507c: lhsi	$r15, [$r6 + #0x26]
L_15080: sltsi	$r15, $r15, #-60
L_15084: beqzs8 . + (10)
L_15086: movi	$r0, #-60
L_1508a: shi	$r0, [$r6 + #0x26]
L_1508e: lhi	$r0, [$r6 + #0x24]
L_15092: addri36.sp $r1, #4
L_15094: ex9.it #270
L_15096: lhi	$r0, [$r6 + #0x26]
L_1509a: mov55 $r2, $r1
L_1509c: ex9.it #84
L_1509e: mov55 $r0, $r6
L_150a0: jal	. + (-1146)
L_150a4: addri36.sp $r1, #4
L_150a6: mov55 $r0, $r6
L_150a8: mov55 $r2, $r1
L_150aa: jal	. + (-2218)
L_150ae: ex9.it #376
L_150b0: shi	$r0, [$r6 + #0x28]
L_150b4: ex9.it #101
L_150b6: shi	$r0, [$r6 + #0x2a]
L_150ba: lwi450 $r0, [$r6]
L_150bc: bnec	$r0, #5, . + (10)
L_150c0: movi55 $r0, #0
L_150c2: swi.gp	$r0, [ + #-6240]
L_150c6: pop25 $r14, #88
L_150c8: push25 $r14, #0
L_150ca: lwi450 $r0, [$r0]
L_150cc: ex9.it #184
L_150ce: ex9.it #76
L_150d0: bnec	$r0, #4, . + (16)
L_150d4: lbi.gp	$r0, [ + #-3948]
L_150d8: addi45 $r0, #1
L_150da: sbi.gp	$r0, [ + #-3948]
L_150de: j8 . + (34)
L_150e0: movi55 $r4, #0
L_150e2: sbi.gp	$r4, [ + #-3948]
L_150e6: bnec	$r0, #1, . + (26)
L_150ea: movi55 $r0, #0
L_150ec: swi.gp	$r0, [ + #-3952]
L_150f0: swi.gp	$r0, [ + #-3956]
L_150f4: shi.gp	$r3, [ + #-3960]
L_150f8: shi.gp	$r1, [ + #-3958]
L_150fc: j	. + (264)
L_15100: lbi.gp	$r4, [ + #0x1322]
L_15104: xlsb33 $r4, $r4
L_15106: bnez38 $r4, . + (-28)
L_15108: ex9.it #55
L_1510a: beqc	$r0, #1, . + (-32)
L_1510e: lhi.gp	$r0, [ + #-3960]
L_15112: lwi.gp	$r7, [ + #-3956]
L_15116: zeh	$r10, $r3
L_1511a: movi	$r16, #0x2ca60
L_1511e: sub	$r0, $r10, $r0
L_15122: sh	$r0, [$r16 + ($r7 << #2)]
L_15126: lhi.gp	$r0, [ + #-3958]
L_1512a: zeh	$r9, $r1
L_1512e: add_slli	$r5, $r16, $r7, #2
L_15132: sub	$r0, $r9, $r0
L_15136: shi333 $r0, [$r5 + #2]
L_15138: lwi.gp	$r5, [ + #-3952]
L_1513c: shi.gp	$r3, [ + #-3960]
L_15140: shi.gp	$r1, [ + #-3958]
L_15144: bnec	$r5, #1, . + (168)
L_15148: lhsi.gp	$r17, [ + #-3972]
L_1514c: lhsi.gp	$r18, [ + #-3970]
L_15150: movpi45 $r11, #0x1f
L_15152: sub	$r3, $r17, $r3
L_15156: sub	$r1, $r18, $r1
L_1515a: mul	$r3, $r3, $r11
L_1515e: mul	$r1, $r1, $r11
L_15162: movi55 $r19, #0xa
L_15164: mov55 $r12, $r11
L_15166: zeb33 $r6, $r7
L_15168: divsr	$r17, $r17, $r3, $r19
L_1516c: divsr	$r18, $r18, $r1, $r19
L_15170: mov55 $r0, $r4
L_15172: mov55 $r11, $r19
L_15174: slli333 $r1, $r6, #2
L_15176: lhs	$r3, [$r16 + ($r6 << #2)]
L_1517a: bnec	$r5, #1, . + (42)
L_1517e: add45 $r1, $r16
L_15180: ex9.it #76
L_15182: mul	$r3, $r3, $r12
L_15186: mul	$r1, $r1, $r12
L_1518a: divsr	$r3, $r3, $r3, $r11
L_1518e: divsr	$r19, $r19, $r1, $r11
L_15192: add45 $r0, $r3
L_15194: add45 $r4, $r19
L_15196: add45 $r0, $r17
L_15198: add45 $r4, $r18
L_1519a: bnez38 $r6, . + (20)
L_1519c: mov55 $r6, $r5
L_1519e: addi45 $r5, #1
L_151a0: zeb33 $r5, $r5
L_151a2: j8 . + (-46)
L_151a4: add45 $r1, $r16
L_151a6: ex9.it #76
L_151a8: add45 $r0, $r3
L_151aa: add45 $r4, $r1
L_151ac: beqz38 $r6, . + (8)
L_151ae: subi45 $r6, #1
L_151b0: zeb33 $r6, $r6
L_151b2: j8 . + (4)
L_151b4: movi55 $r6, #1
L_151b6: bnec	$r5, #2, . + (-24)
L_151ba: movi55 $r1, #4
L_151bc: ex9.it #78
L_151be: divsr	$r4, $r4, $r4, $r1
L_151c2: movi	$r3, #-1000
L_151c6: seh33 $r0, $r0
L_151c8: movi	$r1, #0x3e8
L_151cc: max	$r0, $r0, $r3
L_151d0: seh33 $r4, $r4
L_151d2: min	$r0, $r0, $r1
L_151d6: min	$r4, $r4, $r1
L_151da: add	$r3, $r10, $r0
L_151de: add	$r1, $r9, $r4
L_151e2: seh33 $r3, $r3
L_151e4: seh33 $r1, $r1
L_151e6: bgtz	$r7, . + (102)
L_151ea: j8 . + (6)
L_151ec: bgtz	$r7, . + (12)
L_151f0: addi45 $r7, #1
L_151f2: swi.gp	$r7, [ + #-3956]
L_151f6: j8 . + (14)
L_151f8: swi.gp	$r4, [ + #-3956]
L_151fc: bnez38 $r5, . + (8)
L_151fe: movi55 $r0, #1
L_15200: swi.gp	$r0, [ + #-3952]
L_15204: bltz	$r3, . + (16)
L_15208: sltsi	$r15, $r3, #0xc80
L_1520c: bnezs8 . + (10)
L_1520e: movi	$r3, #0xc7f
L_15212: j8 . + (4)
L_15214: movi55 $r3, #0
L_15216: bltz	$r1, . + (16)
L_1521a: sltsi	$r15, $r1, #0x1400
L_1521e: bnezs8 . + (10)
L_15220: movi	$r1, #0x13ff
L_15224: j8 . + (4)
L_15226: movi55 $r1, #0
L_15228: lbi.gp	$r0, [ + #-3948]
L_1522c: slti45 $r0, #2
L_1522e: bnezs8 . + (16)
L_15230: lhi.gp	$r0, [ + #-3972]
L_15234: shi333 $r0, [$r2 + #0]
L_15236: lhi.gp	$r0, [ + #-3970]
L_1523a: shi333 $r0, [$r2 + #2]
L_1523c: j8 . + (24)
L_1523e: shi333 $r3, [$r2 + #0]
L_15240: shi333 $r1, [$r2 + #2]
L_15242: shi.gp	$r3, [ + #-3972]
L_15246: shi.gp	$r1, [ + #-3970]
L_1524a: j8 . + (10)
L_1524c: movi55 $r0, #0
L_1524e: swi.gp	$r0, [ + #-3956]
L_15252: j8 . + (-78)
L_15254: pop25 $r14, #0
L_15256: push25 $r10, #0
L_15258: lwi.gp	$r18, [ + #-1768]
L_1525c: ex9.it #184
L_1525e: lbi	$r6, [$r18 + #0x16]
L_15262: ex9.it #76
L_15264: beqz38 $r6, . + (10)
L_15266: slti45 $r6, #9
L_15268: beqzs8 . + (10)
L_1526a: zeh33 $r6, $r6
L_1526c: j8 . + (8)
L_1526e: movi55 $r6, #1
L_15270: j8 . + (4)
L_15272: movi55 $r6, #8
L_15274: lbi	$r4, [$r18 + #0x17]
L_15278: beqz38 $r4, . + (10)
L_1527a: slti45 $r4, #9
L_1527c: beqzs8 . + (10)
L_1527e: zeh33 $r4, $r4
L_15280: j8 . + (8)
L_15282: movi55 $r4, #1
L_15284: j8 . + (4)
L_15286: movi55 $r4, #8
L_15288: lwi450 $r19, [$r0]
L_1528a: movi55 $r5, #0
L_1528c: bnec	$r19, #1, . + (36)
L_15290: swi.gp	$r5, [ + #-3976]
L_15294: swi.gp	$r5, [ + #-3980]
L_15298: swi.gp	$r6, [ + #-3984]
L_1529c: swi.gp	$r6, [ + #-3988]
L_152a0: shi.gp	$r3, [ + #-4020]
L_152a4: shi.gp	$r1, [ + #-4018]
L_152a8: swi.gp	$r19, [ + #-4024]
L_152ac: j	. + (346)
L_152b0: bnec	$r19, #3, . + (8)
L_152b4: j	. + (334)
L_152b8: lwi.gp	$r0, [ + #-4024]
L_152bc: addi	$r16, $r4, #-1
L_152c0: addi45 $r0, #1
L_152c2: min	$r0, $r0, $r4
L_152c6: swi.gp	$r0, [ + #-4024]
L_152ca: lwi.gp	$r0, [ + #-3980]
L_152ce: slts45 $r0, $r16
L_152d0: beqzs8 . + (10)
L_152d2: addi45 $r0, #1
L_152d4: swi.gp	$r0, [ + #-3980]
L_152d8: j8 . + (6)
L_152da: swi.gp	$r5, [ + #-3980]
L_152de: lwi.gp	$r0, [ + #-3976]
L_152e2: lwi.gp	$r5, [ + #-3980]
L_152e6: bnez38 $r0, . + (12)
L_152e8: bne	$r5, $r16, . + (10)
L_152ec: movi55 $r0, #1
L_152ee: swi.gp	$r0, [ + #-3976]
L_152f2: movi	$r7, #0x2ca2c
L_152f6: add_slli	$r0, $r7, $r5, #2
L_152fa: lwi.gp	$r17, [ + #-3984]
L_152fe: sh	$r3, [$r7 + ($r5 << #2)]
L_15302: shi333 $r1, [$r0 + #2]
L_15304: swi.gp	$r17, [ + #-3988]
L_15308: blez	$r5, . + (24)
L_1530c: addi	$r9, $r5, #-1
L_15310: lhs	$r0, [$r7 + ($r9 << #2)]
L_15314: add_slli	$r9, $r7, $r9, #2
L_15318: sub333 $r3, $r0, $r3
L_1531a: lhsi	$r0, [$r9 + #2]
L_1531e: j8 . + (16)
L_15320: lhs	$r0, [$r7 + ($r16 << #2)]
L_15324: sub333 $r3, $r0, $r3
L_15326: add_slli	$r0, $r7, $r16, #2
L_1532a: lhsi	$r0, [$r0 + #2]
L_1532e: sub333 $r1, $r0, $r1
L_15330: abs	$r3, $r3
L_15334: ex9.it #207
L_15336: add45 $r1, $r3
L_15338: lhi	$r3, [$r18 + #0x1a]
L_1533c: slli333 $r3, $r3, #1
L_1533e: slt45 $r3, $r1
L_15340: mov55 $r0, $r3
L_15342: bnezs8 . + (12)
L_15344: lhi	$r0, [$r18 + #0x18]
L_15348: slli333 $r0, $r0, #1
L_1534a: slt45 $r1, $r0
L_1534c: ex9.it #359
L_1534e: lwi.gp	$r1, [ + #-3976]
L_15352: bnez38 $r1, . + (8)
L_15354: swi.gp	$r6, [ + #-3984]
L_15358: j8 . + (32)
L_1535a: beqc	$r19, #4, . + (30)
L_1535e: lhi	$r1, [$r18 + #0x18]
L_15362: slli	$r18, $r1, #1
L_15366: sub45 $r0, $r18
L_15368: sub333 $r1, $r4, $r6
L_1536a: mul33 $r0, $r1
L_1536c: sub45 $r3, $r18
L_1536e: divr	$r0, $r0, $r0, $r3
L_15372: sub333 $r0, $r4, $r0
L_15374: swi.gp	$r0, [ + #-3984]
L_15378: lwi.gp	$r0, [ + #-3984]
L_1537c: movi55 $r1, #3
L_1537e: mov55 $r3, $r17
L_15380: maddr32	$r3, $r1, $r0
L_15384: movi55 $r1, #4
L_15386: mov55 $r0, $r3
L_15388: addi45 $r0, #2
L_1538a: ex9.it #78
L_1538c: slts45 $r17, $r0
L_1538e: bnezs8 . + (8)
L_15390: swi.gp	$r0, [ + #-3984]
L_15394: j8 . + (8)
L_15396: addi45 $r17, #1
L_15398: swi.gp	$r17, [ + #-3984]
L_1539c: lwi.gp	$r0, [ + #-3984]
L_153a0: slts45 $r0, $r6
L_153a2: beqzs8 . + (8)
L_153a4: swi.gp	$r6, [ + #-3984]
L_153a8: j8 . + (10)
L_153aa: slts45 $r4, $r0
L_153ac: beqzs8 . + (6)
L_153ae: swi.gp	$r4, [ + #-3984]
L_153b2: lwi.gp	$r0, [ + #-4024]
L_153b6: lwi.gp	$r1, [ + #-3984]
L_153ba: slts45 $r0, $r1
L_153bc: beqzs8 . + (6)
L_153be: swi.gp	$r0, [ + #-3984]
L_153c2: movi55 $r1, #0
L_153c4: lwi.gp	$r0, [ + #-3984]
L_153c8: mov55 $r6, $r1
L_153ca: mov55 $r4, $r1
L_153cc: beq	$r1, $r0, . + (32)
L_153d0: lhs	$r3, [$r7 + ($r5 << #2)]
L_153d4: add45 $r4, $r3
L_153d6: add_slli	$r3, $r7, $r5, #2
L_153da: lhsi	$r3, [$r3 + #2]
L_153de: add45 $r6, $r3
L_153e0: beqz38 $r5, . + (6)
L_153e2: subi45 $r5, #1
L_153e4: j8 . + (4)
L_153e6: mov55 $r5, $r16
L_153e8: addi45 $r1, #1
L_153ea: j8 . + (-30)
L_153ec: movi55 $r3, #2
L_153ee: divsr	$r0, $r0, $r1, $r3
L_153f2: add333 $r3, $r4, $r0
L_153f4: add45 $r0, $r6
L_153f6: divsr	$r3, $r3, $r3, $r1
L_153fa: ex9.it #204
L_153fc: seh33 $r3, $r3
L_153fe: seh33 $r1, $r1
L_15400: j8 . + (6)
L_15402: mov55 $r1, $r5
L_15404: mov55 $r3, $r5
L_15406: shi333 $r3, [$r2 + #0]
L_15408: shi333 $r1, [$r2 + #2]
L_1540a: pop25 $r10, #0
L_1540c: push25 $r10, #0
L_1540e: lwi450 $r4, [$r0]
L_15410: ex9.it #184
L_15412: lhsi	$r7, [$r1 + #2]
L_15416: lwi.gp	$r1, [ + #-1768]
L_1541a: bnec	$r4, #1, . + (38)
L_1541e: shi333 $r3, [$r2 + #0]
L_15420: shi333 $r7, [$r2 + #2]
L_15422: lbi333 $r0, [$r1 + #5]
L_15424: shi.gp	$r3, [ + #-4028]
L_15428: shi.gp	$r7, [ + #-4026]
L_1542c: beqc	$r0, #1, . + (8)
L_15430: j	. + (410)
L_15434: movi55 $r0, #0
L_15436: shi.gp	$r0, [ + #-4032]
L_1543a: shi.gp	$r0, [ + #-4030]
L_1543e: pop25 $r10, #0
L_15440: lbi333 $r6, [$r1 + #5]
L_15442: lbi	$r16, [$r0 + #0x5a]
L_15446: bnec	$r6, #1, . + (106)
L_1544a: andi	$r0, $r16, #2
L_1544e: beqz38 $r0, . + (98)
L_15450: lhsi.gp	$r4, [ + #-4032]
L_15454: lhsi.gp	$r0, [ + #-4030]
L_15458: bnez38 $r4, . + (10)
L_1545a: beqz38 $r0, . + (86)
L_1545c: lhsi.gp	$r3, [ + #-4028]
L_15460: j8 . + (80)
L_15462: lhsi.gp	$r1, [ + #-4026]
L_15466: bnez38 $r0, . + (6)
L_15468: mov55 $r7, $r1
L_1546a: j8 . + (70)
L_1546c: neg33 $r5, $r4
L_1546e: mul33 $r4, $r1
L_15470: lhsi.gp	$r1, [ + #-4028]
L_15474: mov55 $r18, $r3
L_15476: msubr32	$r4, $r0, $r1
L_1547a: mul	$r3, $r5, $r5
L_1547e: mul	$r1, $r0, $r0
L_15482: mul	$r19, $r0, $r5
L_15486: add	$r17, $r1, $r3
L_1548a: mul	$r3, $r3, $r18
L_1548e: msubr32	$r3, $r19, $r7
L_15492: msubr32	$r3, $r0, $r4
L_15496: neg33 $r0, $r0
L_15498: mul33 $r0, $r5
L_1549a: mul33 $r1, $r7
L_1549c: maddr32	$r1, $r0, $r18
L_154a0: msubr32	$r1, $r5, $r4
L_154a4: divsr	$r3, $r3, $r3, $r17
L_154a8: divsr	$r1, $r1, $r1, $r17
L_154ac: seh33 $r3, $r3
L_154ae: seh33 $r7, $r1
L_154b0: andi	$r0, $r16, #2
L_154b4: beqz38 $r0, . + (220)
L_154b6: sltsi	$r15, $r3, #0xc80
L_154ba: bnezs8 . + (18)
L_154bc: lhsi.gp	$r5, [ + #-4028]
L_154c0: sltsi	$r15, $r5, #0xc80
L_154c4: beqzs8 . + (30)
L_154c6: movi	$r5, #0xc7f
L_154ca: j8 . + (24)
L_154cc: bgez	$r3, . + (16)
L_154d0: lhsi.gp	$r5, [ + #-4028]
L_154d4: bltz	$r5, . + (14)
L_154d8: movi55 $r5, #0
L_154da: j8 . + (8)
L_154dc: mov55 $r5, $r3
L_154de: movi55 $r0, #0
L_154e0: j8 . + (4)
L_154e2: movi55 $r0, #1
L_154e4: sltsi	$r15, $r7, #0x1400
L_154e8: bnezs8 . + (14)
L_154ea: lhsi.gp	$r4, [ + #-4026]
L_154ee: sltsi	$r15, $r4, #0x1400
L_154f2: bnezs8 . + (196)
L_154f4: j8 . + (24)
L_154f6: bgez	$r7, . + (16)
L_154fa: lhsi.gp	$r0, [ + #-4026]
L_154fe: movi55 $r4, #0
L_15500: min	$r4, $r0, $r4
L_15504: j8 . + (8)
L_15506: bnec	$r0, #1, . + (138)
L_1550a: mov55 $r4, $r7
L_1550c: lhsi.gp	$r17, [ + #-4028]
L_15510: lhsi.gp	$r16, [ + #-4026]
L_15514: sub	$r15, $r3, $r17
L_15518: mov55 $r1, $r3
L_1551a: mov55 $r0, $r7
L_1551c: sub	$r18, $r7, $r16
L_15520: beqzs8 . + (22)
L_15522: movi55 $r0, #2
L_15524: sub	$r9, $r5, $r17
L_15528: divsr	$r0, $r0, $r15, $r0
L_1552c: maddr32	$r0, $r9, $r18
L_15530: divsr	$r0, $r0, $r0, $r15
L_15534: add45 $r0, $r16
L_15536: beqz	$r18, . + (24)
L_1553a: movi55 $r1, #2
L_1553c: sub	$r9, $r4, $r16
L_15540: divsr	$r1, $r1, $r18, $r1
L_15544: maddr32	$r1, $r9, $r15
L_15548: divsr	$r1, $r1, $r1, $r18
L_1554c: add45 $r1, $r17
L_1554e: beq	$r4, $r16, . + (116)
L_15552: sltsi	$r15, $r1, #0xc80
L_15556: bnezs8 . + (8)
L_15558: addi	$r16, $r1, #-3199
L_1555c: j8 . + (10)
L_1555e: bgez	$r1, . + (100)
L_15562: subri	$r16, $r1, #0
L_15566: beq	$r5, $r17, . + (28)
L_1556a: sltsi	$r15, $r0, #0x1400
L_1556e: bnezs8 . + (8)
L_15570: addi	$r17, $r0, #-5119
L_15574: j8 . + (10)
L_15576: bgez	$r0, . + (70)
L_1557a: subri	$r17, $r0, #0
L_1557e: slts45 $r17, $r16
L_15580: beqzs8 . + (8)
L_15582: seh33 $r3, $r5
L_15584: seh33 $r7, $r0
L_15586: j8 . + (10)
L_15588: slts45 $r16, $r17
L_1558a: beqzs8 . + (6)
L_1558c: seh33 $r3, $r1
L_1558e: seh33 $r7, $r4
L_15590: bnec	$r6, #1, . + (24)
L_15594: lhi.gp	$r0, [ + #-4028]
L_15598: sub333 $r0, $r3, $r0
L_1559a: shi.gp	$r0, [ + #-4032]
L_1559e: lhi.gp	$r0, [ + #-4026]
L_155a2: sub333 $r0, $r7, $r0
L_155a4: shi.gp	$r0, [ + #-4030]
L_155a8: shi.gp	$r3, [ + #-4028]
L_155ac: shi.gp	$r7, [ + #-4026]
L_155b0: shi333 $r3, [$r2 + #0]
L_155b2: shi333 $r7, [$r2 + #2]
L_155b4: pop25 $r10, #0
L_155b6: movi	$r4, #0x13ff
L_155ba: j8 . + (-174)
L_155bc: beqz	$r16, . + (-44)
L_155c0: j8 . + (-62)
L_155c2: movi55 $r16, #0
L_155c4: bne	$r5, $r17, . + (-90)
L_155c8: j8 . + (-56)
L_155ca: pop25 $r10, #0
L_155cc: push25 $r8, #8
L_155ce: mov55 $r6, $r0
L_155d0: lbi	$r0, [$r0 + #0x5a]
L_155d4: xlsb33 $r0, $r0
L_155d6: beqz38 $r0, . + (18)
L_155d8: movi55 $r0, #0
L_155da: shi.gp	$r0, [ + #-1952]
L_155de: shi.gp	$r0, [ + #-1950]
L_155e2: movi55 $r0, #-1
L_155e4: swi.gp	$r0, [ + #-6244]
L_155e8: lwi.gp	$r7, [ + #-1768]
L_155ec: lwi.gp	$r0, [ + #-6244]
L_155f0: lhi333 $r1, [$r7 + #0xa]
L_155f2: slt45 $r0, $r1
L_155f4: beqzs8 . + (8)
L_155f6: addi45 $r0, #1
L_155f8: swi.gp	$r0, [ + #-6244]
L_155fc: lwi450 $r0, [$r6]
L_155fe: beqz38 $r0, . + (226)
L_15600: beqc	$r0, #5, . + (224)
L_15604: lhi	$r0, [$r6 + #0x14]
L_15608: addri36.sp $r1, #4
L_1560a: ex9.it #270
L_1560c: lhi	$r0, [$r6 + #0x16]
L_15610: mov55 $r2, $r1
L_15612: ex9.it #84
L_15614: mov55 $r0, $r6
L_15616: jal	. + (-2690)
L_1561a: addri36.sp $r1, #4
L_1561c: mov55 $r0, $r6
L_1561e: mov55 $r2, $r1
L_15620: lwi.gp	$r3, [ + #-6244]
L_15624: lwi.gp	$r4, [ + #-1952]
L_15628: lbi333 $r5, [$r7 + #1]
L_1562a: jal	. + (-3992)
L_1562e: lbi333 $r0, [$r7 + #2]
L_15630: beqz38 $r0, . + (12)
L_15632: addri36.sp $r1, #4
L_15634: mov55 $r0, $r6
L_15636: mov55 $r2, $r1
L_15638: jal	. + (-3234)
L_1563c: lbi333 $r0, [$r7 + #3]
L_1563e: beqz38 $r0, . + (12)
L_15640: addri36.sp $r1, #4
L_15642: mov55 $r0, $r6
L_15644: mov55 $r2, $r1
L_15646: jal	. + (-1008)
L_1564a: addri36.sp $r1, #4
L_1564c: mov55 $r0, $r6
L_1564e: mov55 $r2, $r1
L_15650: jal	. + (-1416)
L_15654: lbi333 $r0, [$r7 + #4]
L_15656: beqz38 $r0, . + (12)
L_15658: addri36.sp $r1, #4
L_1565a: mov55 $r0, $r6
L_1565c: mov55 $r2, $r1
L_1565e: jal	. + (-3494)
L_15662: ex9.it #326
L_15664: beqc	$r0, #1, . + (20)
L_15668: lbi.gp	$r0, [ + #-5645]
L_1566c: beqc	$r0, #1, . + (12)
L_15670: lbi.gp	$r0, [ + #-5706]
L_15674: bnec	$r0, #1, . + (14)
L_15678: addri36.sp $r1, #4
L_1567a: mov55 $r0, $r6
L_1567c: mov55 $r2, $r1
L_1567e: jal	. + (-2982)
L_15682: lbi	$r0, [$r6 + #0x5a]
L_15686: bmski33 $r0, #1
L_15688: beqz38 $r0, . + (14)
L_1568a: lhi	$r0, [$r6 + #0x14]
L_1568e: ex9.it #270
L_15690: lhi	$r0, [$r6 + #0x16]
L_15694: ex9.it #84
L_15696: addri36.sp $r1, #4
L_15698: mov55 $r0, $r6
L_1569a: mov55 $r2, $r1
L_1569c: jal	. + (-656)
L_156a0: lhsi	$r0, [$sp + #4]
L_156a4: bgez	$r0, . + (8)
L_156a8: movi55 $r0, #0
L_156aa: ex9.it #270
L_156ac: lhsi	$r15, [$sp + #4]
L_156b0: sltsi	$r15, $r15, #0xc80
L_156b4: bnezs8 . + (8)
L_156b6: movi	$r0, #0xc7f
L_156ba: ex9.it #270
L_156bc: lhsi	$r0, [$sp + #6]
L_156c0: bgez	$r0, . + (8)
L_156c4: movi55 $r0, #0
L_156c6: ex9.it #84
L_156c8: lhsi	$r15, [$sp + #6]
L_156cc: sltsi	$r15, $r15, #0x1400
L_156d0: bnezs8 . + (8)
L_156d2: movi	$r0, #0x13ff
L_156d6: ex9.it #84
L_156d8: ex9.it #376
L_156da: shi333 $r0, [$r6 + #0xc]
L_156dc: ex9.it #101
L_156de: shi333 $r0, [$r6 + #0xe]
L_156e0: lwi450 $r0, [$r6]
L_156e2: bnec	$r0, #5, . + (22)
L_156e6: lhi333 $r0, [$r6 + #0xc]
L_156e8: shi.gp	$r0, [ + #-1952]
L_156ec: lhi333 $r0, [$r6 + #0xe]
L_156ee: shi.gp	$r0, [ + #-1950]
L_156f2: movi55 $r0, #0
L_156f4: swi.gp	$r0, [ + #-6244]
L_156f8: pop25 $r8, #8
L_156fa: swi.gp	$r0, [ + #-1524]
L_156fe: ret5 $lp
L_15700: swi.gp	$r0, [ + #-1528]
L_15704: ret5 $lp
L_15706: movi	$r0, #0x2d398
L_1570a: movi55 $r1, #1
L_1570c: swi.gp	$r0, [ + #-1700]
L_15710: movi55 $r0, #0
L_15712: sbi.gp	$r0, [ + #-1696]
L_15716: sbi.gp	$r0, [ + #-1695]
L_1571a: sbi.gp	$r0, [ + #-1694]
L_1571e: sbi.gp	$r0, [ + #-1693]
L_15722: sbi.gp	$r0, [ + #-1692]
L_15726: ex9.it #436
L_15728: sbi.gp	$r0, [ + #-1690]
L_1572c: sbi.gp	$r0, [ + #-1751]
L_15730: sbi.gp	$r0, [ + #-1752]
L_15734: sbi.gp	$r0, [ + #-1750]
L_15738: swi.gp	$r1, [ + #-6232]
L_1573c: sbi.gp	$r0, [ + #-5257]
L_15740: movi	$r1, #0x2d348
L_15744: movi55 $r0, #0
L_15746: movi55 $r2, #-1
L_15748: ex9.it #260
L_1574a: addi45 $r0, #1
L_1574c: bnec	$r0, #0x4f, . + (-4)
L_15750: ret5 $lp
L_15752: push25 $r6, #0
L_15754: ex9.it #369
L_15756: pop25 $r6, #0
L_15758: push25 $r6, #0
L_1575a: mov55 $r6, $r0
L_1575c: jal	. + (-53182)
L_15760: beqz38 $r0, . + (8)
L_15762: movi55 $r0, #0
L_15764: sbi333 $r0, [$r6 + #4]
L_15766: sbi333 $r0, [$r6 + #5]
L_15768: pop25 $r6, #0
L_1576a: lbi.gp	$r0, [ + #-1744]
L_1576e: ret5 $lp
L_15770: lbi.gp	$r0, [ + #-1756]
L_15774: ret5 $lp
L_15776: movi	$r3, #0x2d308
L_1577a: sbi333 $r0, [$r3 + #0]
L_1577c: sbi333 $r1, [$r3 + #1]
L_1577e: sbi333 $r2, [$r3 + #2]
L_15780: ret5 $lp
L_15782: swi.gp	$r0, [ + #-6232]
L_15786: ret5 $lp
L_15788: lwi.gp	$r0, [ + #-6232]
L_1578c: ret5 $lp
L_1578e: lwi.gp	$r0, [ + #-5260]
L_15792: sethi	$r5, #0xff00f
L_15796: ex9.it #248
L_15798: and33 $r5, $r0
L_1579a: sethi	$r0, #0x1000
L_1579e: bnes38 $r0, . + (12)
L_157a0: movi55 $r0, #0
L_157a2: sbi.gp	$r0, [ + #-5257]
L_157a6: movi55 $r0, #1
L_157a8: ret5 $lp
L_157aa: movi55 $r0, #0
L_157ac: ret5 $lp
L_157ae: lbi.gp	$r1, [ + #-1694]
L_157b2: lbi.gp	$r5, [ + #-1693]
L_157b6: ex9.it #301
L_157b8: bne	$r1, $r5, . + (32)
L_157bc: lbi.gp	$r2, [ + #-1696]
L_157c0: bnec	$r2, #1, . + (24)
L_157c4: lbi333 $r0, [$r0 + #5]
L_157c6: addi333 $r5, $r1, #1
L_157c8: divsr	$r0, $r5, $r5, $r0
L_157cc: zeb33 $r5, $r5
L_157ce: sbi.gp	$r5, [ + #-1694]
L_157d2: sbi.gp	$r5, [ + #-1693]
L_157d6: ret5 $lp
L_157d8: lbi333 $r0, [$r0 + #5]
L_157da: addi45 $r5, #1
L_157dc: divsr	$r0, $r5, $r5, $r0
L_157e0: zeb33 $r5, $r5
L_157e2: sbi.gp	$r5, [ + #-1693]
L_157e6: bnes38 $r1, . + (8)
L_157e8: movi55 $r0, #1
L_157ea: sbi.gp	$r0, [ + #-1696]
L_157ee: ret5 $lp
L_157f0: lbi.gp	$r5, [ + #-1694]
L_157f4: lbi.gp	$r1, [ + #-1693]
L_157f8: bnes38 $r1, . + (8)
L_157fa: lbi.gp	$r0, [ + #-1696]
L_157fe: beqz38 $r0, . + (30)
L_15800: ex9.it #301
L_15802: addi45 $r5, #1
L_15804: lbi333 $r0, [$r0 + #5]
L_15806: divsr	$r0, $r5, $r5, $r0
L_1580a: zeb33 $r5, $r5
L_1580c: sbi.gp	$r5, [ + #-1694]
L_15810: bne	$r1, $r5, . + (10)
L_15814: movi55 $r0, #0
L_15816: sbi.gp	$r0, [ + #-1696]
L_1581a: movi55 $r0, #1
L_1581c: ret5 $lp
L_1581e: lbi.gp	$r5, [ + #-1694]
L_15822: lbi.gp	$r0, [ + #-1693]
L_15826: bnes38 $r0, . + (14)
L_15828: lbi.gp	$r0, [ + #-1696]
L_1582c: movi55 $r1, #0
L_1582e: subi45 $r0, #1
L_15830: ex9.it #325
L_15832: ret5 $lp
L_15834: movi55 $r0, #1
L_15836: ret5 $lp
L_15838: lbi.gp	$r5, [ + #-1694]
L_1583c: lbi.gp	$r0, [ + #-1693]
L_15840: bnes38 $r0, . + (14)
L_15842: lbi.gp	$r1, [ + #-1696]
L_15846: movi55 $r0, #0
L_15848: slt	$r0, $r0, $r1
L_1584c: ret5 $lp
L_1584e: movi55 $r0, #1
L_15850: ret5 $lp
L_15852: push25 $r6, #0
L_15854: movi55 $r5, #0
L_15856: addi45 $r4, #1
L_15858: slt45 $r5, $r4
L_1585a: beqzs8 . + (14)
L_1585c: lb.bi	$r6, [$r0], $r2 << #0
L_15860: sb.bi	$r6, [$r1], $r3 << #0
L_15864: addi45 $r5, #1
L_15866: j8 . + (-14)
L_15868: pop25 $r6, #0
L_1586a: push25 $r14, #8
L_1586c: lwi.gp	$r10, [ + #-6232]
L_15870: movi55 $r5, #-1
L_15872: mov55 $r11, $r0
L_15874: mov55 $r9, $r1
L_15876: lwi450 $r6, [$r1]
L_15878: sbi	$r5, [$sp + #7]
L_1587c: beqc	$r10, #1, . + (8)
L_15880: j	. + (310)
L_15884: movi55 $r7, #0
L_15886: lwi.gp	$r3, [ + #-1528]
L_1588a: lbi.gp	$r17, [ + #-1756]
L_1588e: lwi.gp	$r18, [ + #0x12c8]
L_15892: lwi.gp	$r19, [ + #-1524]
L_15896: mov55 $r1, $r10
L_15898: movi	$r12, #0x4c
L_1589c: mov55 $r13, $r7
L_1589e: mov55 $r14, $r7
L_158a0: movi55 $r8, #6
L_158a2: movi55 $r16, #2
L_158a4: subi333 $r2, $r1, #1
L_158a6: lbi	$r0, [$r3 + #9]
L_158aa: zeb33 $r2, $r2
L_158ac: slt45 $r2, $r0
L_158ae: beqzs8 . + (36)
L_158b0: mul	$r20, $r2, $r12
L_158b4: lwi450 $r0, [$r11]
L_158b6: add	$r21, $r0, $r20
L_158ba: lwi450 $r0, [$r21]
L_158bc: subi333 $r2, $r0, #1
L_158be: slti45 $r2, #2
L_158c0: bnezs8 . + (6)
L_158c2: bnec	$r0, #4, . + (236)
L_158c6: lwi	$r0, [$r11 + #0xc]
L_158ca: lwi450 $r0, [$r0]
L_158cc: subi333 $r2, $r0, #1
L_158ce: slti45 $r2, #2
L_158d0: beqzs8 . + (16)
L_158d2: movi55 $r0, #-1
L_158d4: sbi	$r0, [$r6 + #0x3e]
L_158d8: sbi	$r0, [$r6 + #0x3f]
L_158dc: j	. + (268)
L_158e0: beqc	$r0, #4, . + (-14)
L_158e4: ex9.it #27
L_158e6: bnec	$r0, #2, . + (10)
L_158ea: lbi.gp	$r0, [ + #0x133f]
L_158ee: bnez38 $r0, . + (-28)
L_158f0: bnec	$r17, #1, . + (8)
L_158f4: beqc	$r18, #5, . + (-34)
L_158f8: lbi	$r0, [$r19 + #3]
L_158fc: lhsi	$r2, [$r21 + #8]
L_15900: lhsi	$r4, [$r21 + #0xa]
L_15904: lhi	$r23, [$r3 + #4]
L_15908: lhi	$r22, [$r3 + #6]
L_1590c: beqz38 $r0, . + (46)
L_1590e: lhi	$r0, [$r3 + #0x12]
L_15912: addi	$r24, $r23, #-1
L_15916: subi45 $r0, #1
L_15918: divsr	$r15, $r15, $r0, $r16
L_1591c: maddr32	$r15, $r2, $r24
L_15920: addi	$r24, $r22, #-1
L_15924: divsr	$r2, $r2, $r15, $r0
L_15928: lhi	$r0, [$r3 + #0x14]
L_1592c: subi45 $r0, #1
L_1592e: divsr	$r15, $r15, $r0, $r16
L_15932: maddr32	$r15, $r4, $r24
L_15936: divsr	$r4, $r4, $r15, $r0
L_1593a: slts45 $r2, $r23
L_1593c: bnezs8 . + (6)
L_1593e: addi	$r2, $r23, #-1
L_15942: slts45 $r4, $r22
L_15944: max	$r0, $r2, $r13
L_15948: bnezs8 . + (6)
L_1594a: addi	$r4, $r22, #-1
L_1594e: mov55 $r15, $r6
L_15950: maddr32	$r15, $r7, $r8
L_15954: movi55 $r2, #2
L_15956: or_slli	$r2, $r2, $r1, #3
L_1595a: max	$r4, $r4, $r14
L_1595e: sbi	$r2, [$r15 + #0]
L_15962: srai	$r2, $r0, #4
L_15966: sbi	$r2, [$r15 + #1]
L_1596a: srai	$r2, $r4, #4
L_1596e: sbi	$r2, [$r15 + #2]
L_15972: fexti33 $r4, #3
L_15974: or_slli	$r4, $r4, $r0, #4
L_15978: lhi	$r0, [$r21 + #0x14]
L_1597c: sbi	$r4, [$r15 + #3]
L_15980: beqz38 $r0, . + (12)
L_15982: lwi450 $r2, [$r11]
L_15984: add45 $r2, $r20
L_15986: lbi	$r0, [$r2 + #0x14]
L_1598a: j8 . + (4)
L_1598c: movi55 $r0, #1
L_1598e: sbi	$r0, [$r15 + #4]
L_15992: lhi	$r0, [$r21 + #0x16]
L_15996: beqz38 $r0, . + (6)
L_15998: zeb33 $r0, $r0
L_1599a: j8 . + (4)
L_1599c: movi55 $r0, #1
L_1599e: addi45 $r7, #1
L_159a0: sbi	$r0, [$r15 + #5]
L_159a4: zeb33 $r7, $r7
L_159a6: sbi	$r5, [$r6 + #0x3c]
L_159aa: sbi	$r5, [$r6 + #0x3d]
L_159ae: addi45 $r1, #1
L_159b0: zeb33 $r1, $r1
L_159b2: j	. + (-270)
L_159b6: lbi.gp	$r7, [ + #-1752]
L_159ba: beqz38 $r7, . + (46)
L_159bc: lbi.gp	$r0, [ + #-1751]
L_159c0: beqc	$r0, #0x19, . + (38)
L_159c4: slti45 $r0, #0x1e
L_159c6: lbi333 $r1, [$r6 + #0]
L_159c8: beqzs8 . + (10)
L_159ca: fexti33 $r1, #2
L_159cc: or_slli	$r0, $r1, $r0, #3
L_159d0: j8 . + (6)
L_159d2: movi	$r0, #0xf0
L_159d6: sbi333 $r0, [$r6 + #0]
L_159d8: lbi.gp	$r0, [ + #-1750]
L_159dc: sbi333 $r0, [$r6 + #2]
L_159de: movi55 $r0, #1
L_159e0: sbi333 $r7, [$r6 + #1]
L_159e2: sbi.gp	$r0, [ + #-1744]
L_159e6: movi55 $r7, #0
L_159e8: lbi.gp	$r0, [ + #-1744]
L_159ec: beqc	$r0, #0xff, . + (26)
L_159f0: beqz38 $r0, . + (22)
L_159f2: bnez38 $r7, . + (20)
L_159f4: movi55 $r0, #-1
L_159f6: sbi.gp	$r0, [ + #-1744]
L_159fa: lwi.gp	$r0, [ + #-1748]
L_159fe: addi45 $r0, #1
L_15a00: swi.gp	$r0, [ + #-1748]
L_15a04: j8 . + (6)
L_15a06: sbi.gp	$r7, [ + #-1744]
L_15a0a: bnec	$r10, #1, . + (44)
L_15a0e: lwi.gp	$r0, [ + #-1528]
L_15a12: lbi	$r4, [$r0 + #9]
L_15a16: slt45 $r7, $r4
L_15a18: beqzs8 . + (30)
L_15a1a: movi55 $r1, #6
L_15a1c: movi55 $r0, #6
L_15a1e: mul33 $r1, $r7
L_15a20: sub45 $r4, $r7
L_15a22: lwi450 $r2, [$r9]
L_15a24: mul33 $r4, $r0
L_15a26: zeb33 $r1, $r1
L_15a28: add45 $r1, $r2
L_15a2a: addi	$r0, $sp, #7
L_15a2e: movi55 $r2, #0
L_15a30: mov55 $r3, $r10
L_15a32: subi45 $r4, #1
L_15a34: ifcall9 . + (612)
L_15a36: lwi450 $r0, [$r9]
L_15a38: movi55 $r1, #0
L_15a3a: mov55 $r5, $r0
L_15a3c: addi	$r2, $r0, #0x40
L_15a40: lbi.bi	$r3, [$r5], #1
L_15a44: add45 $r1, $r3
L_15a46: zeb33 $r1, $r1
L_15a48: bnes38 $r2, . + (-8)
L_15a4a: neg33 $r1, $r1
L_15a4c: sbi	$r1, [$r0 + #0x40]
L_15a50: beqc	$r10, #1, . + (8)
L_15a54: j	. + (370)
L_15a58: lwi	$r0, [$r11 + #0xc]
L_15a5c: lwi450 $r1, [$r0]
L_15a5e: subi333 $r2, $r1, #1
L_15a60: slti45 $r2, #2
L_15a62: bnezs8 . + (10)
L_15a64: beqc	$r1, #4, . + (8)
L_15a68: j	. + (330)
L_15a6c: lhsi	$r4, [$r0 + #0xc]
L_15a70: lhsi	$r3, [$r0 + #0xe]
L_15a74: sltsi	$r15, $r4, #0xc80
L_15a78: beqzs8 . + (10)
L_15a7a: movi55 $r1, #0
L_15a7c: max	$r4, $r4, $r1
L_15a80: j8 . + (6)
L_15a82: movi	$r4, #0xc7f
L_15a86: sltsi	$r15, $r3, #0x1400
L_15a8a: beqzs8 . + (10)
L_15a8c: movi55 $r1, #0
L_15a8e: max	$r3, $r3, $r1
L_15a92: j8 . + (6)
L_15a94: movi	$r3, #0x13ff
L_15a98: lbi	$r0, [$r0 + #0x72]
L_15a9c: beqc	$r0, #1, . + (8)
L_15aa0: movi55 $r0, #1
L_15aa2: j8 . + (6)
L_15aa4: movi	$r0, #0xf0
L_15aa8: ex9.it #217
L_15aaa: lwi	$r2, [$r11 + #0xc]
L_15aae: bnec	$r0, #0xf0, . + (92)
L_15ab2: movi55 $r0, #0
L_15ab4: sbi	$r0, [$r2 + #0x72]
L_15ab8: lwi	$r1, [$r11 + #0xc]
L_15abc: movi	$r2, #0xff00
L_15ac0: lwi	$r0, [$r1 + #0x50]
L_15ac4: lwi	$r3, [$r1 + #0x54]
L_15ac8: and	$r1, $r0, $r2
L_15acc: srli45 $r1, #8
L_15ace: sbi	$r1, [$r6 + #0x42]
L_15ad2: srli	$r1, $r0, #0x18
L_15ad6: sbi	$r1, [$r6 + #0x44]
L_15ada: sethi	$r1, #0xff0
L_15ade: sbi	$r0, [$r6 + #0x43]
L_15ae2: and33 $r0, $r1
L_15ae4: srli45 $r0, #0x10
L_15ae6: and33 $r2, $r3
L_15ae8: and33 $r1, $r3
L_15aea: sbi	$r0, [$r6 + #0x45]
L_15aee: srli45 $r2, #8
L_15af0: srli	$r0, $r3, #0x18
L_15af4: srli45 $r1, #0x10
L_15af6: sbi	$r2, [$r6 + #0x46]
L_15afa: sbi	$r3, [$r6 + #0x47]
L_15afe: sbi	$r0, [$r6 + #0x48]
L_15b02: sbi	$r1, [$r6 + #0x49]
L_15b06: movi55 $r4, #9
L_15b08: j8 . + (148)
L_15b0a: slli333 $r4, $r4, #2
L_15b0c: slli333 $r3, $r3, #2
L_15b0e: srai	$r0, $r4, #8
L_15b12: sbi	$r0, [$r6 + #0x42]
L_15b16: srai	$r0, $r3, #8
L_15b1a: sbi	$r4, [$r6 + #0x43]
L_15b1e: sbi	$r0, [$r6 + #0x44]
L_15b22: sbi	$r3, [$r6 + #0x45]
L_15b26: lbi	$r0, [$r2 + #0x43]
L_15b2a: bnez38 $r0, . + (14)
L_15b2c: lhi	$r0, [$r2 + #0x4a]
L_15b30: bnez38 $r0, . + (18)
L_15b32: lhi	$r0, [$r2 + #0x4c]
L_15b36: j8 . + (8)
L_15b38: bnec	$r0, #1, . + (10)
L_15b3c: movi55 $r0, #0
L_15b3e: shi	$r0, [$r2 + #0x4a]
L_15b42: lhi	$r0, [$r2 + #0x4a]
L_15b46: sbi	$r0, [$r6 + #0x47]
L_15b4a: srli	$r1, $r0, #8
L_15b4e: lhsi	$r0, [$r2 + #0x2a]
L_15b52: sbi	$r1, [$r6 + #0x46]
L_15b56: lhi	$r1, [$r2 + #0x28]
L_15b5a: sbi	$r0, [$r6 + #0x49]
L_15b5e: movi55 $r0, #0
L_15b60: sbi	$r1, [$r6 + #0x48]
L_15b64: sbi	$r0, [$r6 + #0x4a]
L_15b68: lbi	$r0, [$r2 + #0x43]
L_15b6c: sbi	$r0, [$r6 + #0x4b]
L_15b70: movi55 $r0, #0
L_15b72: sbi	$r0, [$r6 + #0x4c]
L_15b76: lbi	$r0, [$r2 + #0x41]
L_15b7a: beqc	$r0, #1, . + (16)
L_15b7e: beqc	$r0, #2, . + (16)
L_15b82: bnec	$r0, #3, . + (18)
L_15b86: ex9.it #119
L_15b88: j8 . + (14)
L_15b8a: movpi45 $r0, #0x20
L_15b8c: j8 . + (10)
L_15b8e: movi	$r0, #0x40
L_15b92: j8 . + (4)
L_15b94: movpi45 $r0, #0x10
L_15b96: sbi	$r0, [$r6 + #0x4d]
L_15b9a: movi55 $r4, #0xd
L_15b9c: lwi450 $r2, [$r9]
L_15b9e: addi	$r1, $r4, #0x41
L_15ba2: add45 $r1, $r2
L_15ba4: addi	$r0, $sp, #7
L_15ba8: movi55 $r2, #0
L_15baa: movi55 $r3, #1
L_15bac: subri	$r4, $r4, #0xd
L_15bb0: j8 . + (52)
L_15bb2: lwi450 $r1, [$r9]
L_15bb4: addi	$r0, $sp, #7
L_15bb8: addi	$r1, $r1, #0x41
L_15bbc: movi55 $r2, #0
L_15bbe: movi55 $r3, #1
L_15bc0: movi55 $r4, #0xd
L_15bc2: ifcall9 . + (214)
L_15bc4: j8 . + (38)
L_15bc6: bnec	$r10, #3, . + (36)
L_15bca: lbi.gp	$r0, [ + #-1751]
L_15bce: bnec	$r0, #0x19, . + (-28)
L_15bd2: ex9.it #217
L_15bd4: lwi450 $r1, [$r9]
L_15bd6: addi	$r0, $sp, #7
L_15bda: addi	$r1, $r1, #0x42
L_15bde: movi55 $r2, #0
L_15be0: movi55 $r3, #1
L_15be2: movi55 $r4, #0xc
L_15be4: ifcall9 . + (180)
L_15be6: movi55 $r0, #1
L_15be8: j8 . + (4)
L_15bea: movi55 $r0, #0
L_15bec: lwi450 $r2, [$r9]
L_15bee: movi55 $r1, #0
L_15bf0: addi	$r5, $r2, #0x41
L_15bf4: addi	$r3, $r2, #0x4e
L_15bf8: lbi.bi	$r4, [$r5], #1
L_15bfc: add45 $r1, $r4
L_15bfe: zeb33 $r1, $r1
L_15c00: bnes38 $r3, . + (-8)
L_15c02: neg33 $r1, $r1
L_15c04: sbi	$r1, [$r2 + #0x4e]
L_15c08: lbi.gp	$r1, [ + #-1756]
L_15c0c: beqc	$r1, #0xff, . + (20)
L_15c10: beqz38 $r1, . + (16)
L_15c12: bnez38 $r0, . + (14)
L_15c14: movi55 $r0, #-1
L_15c16: sbi.gp	$r0, [ + #-1756]
L_15c1a: bnez38 $r7, . + (14)
L_15c1c: movi55 $r0, #1
L_15c1e: j8 . + (12)
L_15c20: sbi.gp	$r0, [ + #-1756]
L_15c24: bnez38 $r7, . + (4)
L_15c26: beqz38 $r0, . + (-10)
L_15c28: movi55 $r0, #0
L_15c2a: sbi.gp	$r0, [ + #-1690]
L_15c2e: pop25 $r14, #8
L_15c30: push25 $r6, #0
L_15c32: movi55 $r2, #1
L_15c34: mov55 $r3, $r2
L_15c36: movi	$r4, #0x4e
L_15c3a: ifcall9 . + (94)
L_15c3c: pop25 $r6, #0
L_15c3e: lbi333 $r0, [$r0 + #0]
L_15c40: srli45 $r0, #3
L_15c42: addi45 $r0, #0x15
L_15c44: fexti33 $r0, #4
L_15c46: slti	$r0, $r0, #0x14
L_15c4a: ret5 $lp
L_15c4c: push25 $r10, #0
L_15c4e: mov55 $r7, $r0
L_15c50: movi55 $r0, #0
L_15c52: ex9.it #436
L_15c54: lbi333 $r0, [$r7 + #0]
L_15c56: mov55 $r6, $r1
L_15c58: bnez38 $r0, . + (8)
L_15c5a: lbi	$r0, [$r7 + #8]
L_15c5e: beqz38 $r0, . + (98)
L_15c60: lbi.gp	$r0, [ + #-1695]
L_15c64: bnec	$r0, #1, . + (24)
L_15c68: lbi.gp	$r1, [ + #-1692]
L_15c6c: sbi.gp	$r1, [ + #-1694]
L_15c70: sbi.gp	$r1, [ + #-1693]
L_15c74: movi55 $r1, #0
L_15c76: sbi.gp	$r1, [ + #-1696]
L_15c7a: ex9.it #436
L_15c7c: jal	. + (-1230)
L_15c80: ex9.it #301
L_15c82: lbi.gp	$r9, [ + #-1693]
L_15c86: lbi333 $r4, [$r0 + #4]
L_15c88: lwi.gp	$r1, [ + #-1700]
L_15c8c: movi55 $r2, #1
L_15c8e: lwi333 $r0, [$r7 + #4]
L_15c90: maddr32	$r1, $r9, $r4
L_15c94: mov55 $r3, $r2
L_15c96: subi45 $r4, #1
L_15c98: jal	. + (-1094)
L_15c9c: lbi.gp	$r0, [ + #-1690]
L_15ca0: bnec	$r0, #1, . + (12)
L_15ca4: sbi.gp	$r0, [ + #-1695]
L_15ca8: sbi.gp	$r9, [ + #-1692]
L_15cac: lbi.gp	$r0, [ + #-1755]
L_15cb0: beqc	$r0, #0xff, . + (10)
L_15cb4: addi45 $r0, #1
L_15cb6: sbi.gp	$r0, [ + #-1755]
L_15cba: movi55 $r0, #1
L_15cbc: ex9.it #306
L_15cbe: j8 . + (6)
L_15cc0: sbi.gp	$r0, [ + #-1755]
L_15cc4: lbi.gp	$r1, [ + #-1691]
L_15cc8: lwi450 $r0, [$r6]
L_15cca: bnec	$r1, #1, . + (84)
L_15cce: lbi333 $r0, [$r0 + #5]
L_15cd0: bnez38 $r0, . + (40)
L_15cd2: ifcall9 . + (106)
L_15cd4: bnez38 $r0, . + (26)
L_15cd6: ex9.it #301
L_15cd8: lbi.gp	$r3, [ + #-1692]
L_15cdc: lbi333 $r2, [$r0 + #4]
L_15cde: lwi450 $r1, [$r6]
L_15ce0: lwi.gp	$r0, [ + #-1700]
L_15ce4: maddr32	$r0, $r3, $r2
L_15ce8: lwi450 $r1, [$r1]
L_15cea: ifcall9 . + (114)
L_15cec: j8 . + (16)
L_15cee: jal	. + (-54702)
L_15cf2: ifcall9 . + (74)
L_15cf4: bnez38 $r0, . + (-2)
L_15cf6: j8 . + (-32)
L_15cf8: ifcall9 . + (68)
L_15cfa: bnez38 $r0, . + (4)
L_15cfc: ifcall9 . + (138)
L_15cfe: lwi450 $r1, [$r6]
L_15d00: movi55 $r0, #0
L_15d02: ex9.it #436
L_15d04: sbi.gp	$r0, [ + #-1695]
L_15d08: movi55 $r0, #1
L_15d0a: sbi333 $r0, [$r1 + #5]
L_15d0c: lwi450 $r1, [$r6]
L_15d0e: sbi333 $r0, [$r1 + #4]
L_15d10: lwi450 $r6, [$r6]
L_15d12: lwi450 $r0, [$r6]
L_15d14: ifcall9 . + (104)
L_15d16: bnec	$r0, #1, . + (122)
L_15d1a: sbi333 $r0, [$r6 + #7]
L_15d1c: j8 . + (116)
L_15d1e: lbi333 $r1, [$r0 + #4]
L_15d20: bnez38 $r1, . + (8)
L_15d22: ifcall9 . + (208)
L_15d24: beqz38 $r0, . + (108)
L_15d26: j8 . + (22)
L_15d28: lbi333 $r0, [$r0 + #5]
L_15d2a: bnec	$r0, #1, . + (12)
L_15d2e: ifcall9 . + (14)
L_15d30: bnez38 $r0, . + (96)
L_15d32: ifcall9 . + (84)
L_15d34: j8 . + (92)
L_15d36: jal	. + (-1304)
L_15d3a: bnez38 $r0, . + (-12)
L_15d3c: jal	. + (-54678)
L_15d40: mov55 $r7, $r0
L_15d42: bnez38 $r0, . + (78)
L_15d44: jal	. + (-1364)
L_15d48: ex9.it #301
L_15d4a: lbi.gp	$r9, [ + #-1694]
L_15d4e: lbi333 $r2, [$r0 + #4]
L_15d50: lwi450 $r1, [$r6]
L_15d52: lwi.gp	$r0, [ + #-1700]
L_15d56: lwi450 $r1, [$r1]
L_15d58: maddr32	$r0, $r9, $r2
L_15d5c: jal	. + (-300)
L_15d60: lbi.gp	$r1, [ + #-1695]
L_15d64: bnec	$r1, #1, . + (34)
L_15d68: lbi.gp	$r5, [ + #-1692]
L_15d6c: bne	$r5, $r9, . + (26)
L_15d70: lwi450 $r0, [$r6]
L_15d72: sbi.gp	$r7, [ + #-1695]
L_15d76: sbi333 $r1, [$r0 + #5]
L_15d78: lwi450 $r7, [$r6]
L_15d7a: lwi450 $r0, [$r7]
L_15d7c: jal	. + (-318)
L_15d80: bnec	$r0, #1, . + (6)
L_15d84: sbi333 $r0, [$r7 + #7]
L_15d86: jal	. + (-54720)
L_15d8a: lwi450 $r0, [$r6]
L_15d8c: movi55 $r1, #1
L_15d8e: sbi333 $r1, [$r0 + #4]
L_15d90: pop25 $r10, #0
L_15d92: push25 $r8, #0
L_15d94: mov55 $r2, $r0
L_15d96: mov55 $r7, $r1
L_15d98: movi	$r0, #0x2d328
L_15d9c: lwi333 $r1, [$r2 + #4]
L_15d9e: swi450 $r1, [$r0]
L_15da0: lbi.gp	$r1, [ + #-1744]
L_15da4: ex9.it #471
L_15da6: lwi333 $r1, [$r2 + #8]
L_15da8: swi333 $r1, [$r0 + #4]
L_15daa: lwi333 $r1, [$r2 + #0xc]
L_15dac: swi333 $r1, [$r0 + #0xc]
L_15dae: lbi.gp	$r1, [ + #-1756]
L_15db2: movi	$r6, #0x2d348
L_15db6: sbi	$r1, [$r0 + #0x10]
L_15dba: movi	$r1, #0x2d324
L_15dbe: swi.gp	$r6, [ + #-1724]
L_15dc2: jal	. + (-1368)
L_15dc6: movi	$r0, #0x2d318
L_15dca: lbi.gp	$r1, [ + #-1744]
L_15dce: sbi333 $r1, [$r0 + #0]
L_15dd0: lbi.gp	$r1, [ + #-1756]
L_15dd4: ex9.it #471
L_15dd6: lwi450 $r1, [$r7]
L_15dd8: swi.gp	$r1, [ + #-1740]
L_15ddc: movi	$r1, #0x2d314
L_15de0: swi333 $r6, [$r0 + #4]
L_15de2: jal	. + (-406)
L_15de6: movi55 $r1, #0
L_15de8: movi	$r0, #0x2d308
L_15dec: sbi333 $r1, [$r0 + #0]
L_15dee: sbi333 $r1, [$r0 + #1]
L_15df0: sbi333 $r1, [$r0 + #2]
L_15df2: jal	. + (-1466)
L_15df6: pop25 $r8, #0
L_15df8: swi.gp	$r0, [ + #-776]
L_15dfc: ret5 $lp
L_15dfe: swi.gp	$r0, [ + #-780]
L_15e02: ret5 $lp
L_15e04: push25 $r14, #0
L_15e06: addi	$sp, $sp, #-1496
L_15e0a: swi37.sp $r3, [ + #0x14]
L_15e0c: addi	$r6, $sp, #0x338
L_15e10: movi55 $r3, #0
L_15e12: swi37.sp $r1, [ + #8]
L_15e14: swi37.sp $r2, [ + #0x18]
L_15e16: lwi.gp	$r5, [ + #-780]
L_15e1a: mov55 $r1, $r6
L_15e1c: movi55 $r16, #1
L_15e1e: mov55 $r17, $r3
L_15e20: lwi37.sp $r2, [ + #8]
L_15e22: swi37.sp $r2, [ + #0x10]
L_15e24: slt45 $r3, $r2
L_15e26: beqzs8 . + (84)
L_15e28: lhsi	$r4, [$r0 + #0]
L_15e2c: lhsi	$r2, [$r0 + #2]
L_15e30: shi	$r4, [$r1 + #0x14]
L_15e34: shi	$r4, [$r1 + #0x18]
L_15e38: lwi37.sp $r4, [ + #0x14]
L_15e3a: shi	$r2, [$r1 + #0x16]
L_15e3e: shi	$r2, [$r1 + #0x1a]
L_15e42: lwi333 $r2, [$r0 + #4]
L_15e44: lwi333 $r4, [$r4 + #0x10]
L_15e46: swi	$r2, [$r1 + #0x2c]
L_15e4a: add_slli	$r2, $r4, $r2, #1
L_15e4e: lbi333 $r7, [$r2 + #1]
L_15e50: beqz38 $r7, . + (22)
L_15e52: lbi333 $r4, [$r5 + #2]
L_15e54: subi45 $r4, #1
L_15e56: beq	$r7, $r4, . + (16)
L_15e5a: lbi333 $r4, [$r2 + #0]
L_15e5c: beqz38 $r4, . + (10)
L_15e5e: lbi333 $r2, [$r5 + #3]
L_15e60: subi45 $r2, #1
L_15e62: bne	$r4, $r2, . + (10)
L_15e66: sbi	$r16, [$r1 + #0x35]
L_15e6a: j8 . + (6)
L_15e6c: sbi	$r17, [$r1 + #0x35]
L_15e70: addi45 $r3, #1
L_15e72: addi	$r1, $r1, #0x38
L_15e76: addi45 $r0, #8
L_15e78: j8 . + (-88)
L_15e7a: lbi.gp	$r1, [ + #-784]
L_15e7e: movi55 $r0, #0
L_15e80: swi37.sp $r1, [ + #4]
L_15e82: addi	$r14, $sp, #0x38
L_15e86: mov55 $r2, $r0
L_15e88: addi	$r9, $sp, #0x68
L_15e8c: addi	$r17, $sp, #0x98
L_15e90: addi	$r16, $sp, #0xc8
L_15e94: movi	$r7, #0x38
L_15e98: ex9.it #264
L_15e9a: mul	$r3, $r0, $r7
L_15e9e: sw	$r2, [$r14 + ($r0 << #2)]
L_15ea2: add333 $r5, $r6, $r3
L_15ea4: sw	$r2, [$r9 + ($r0 << #2)]
L_15ea8: sw	$r2, [$r17 + ($r0 << #2)]
L_15eac: sw	$r2, [$r16 + ($r0 << #2)]
L_15eb0: add45 $r3, $r4
L_15eb2: addi45 $r0, #1
L_15eb4: sbi	$r2, [$r5 + #0x2a]
L_15eb8: sbi	$r2, [$r3 + #0x2a]
L_15ebc: bnec	$r0, #0xc, . + (-34)
L_15ec0: lwi37.sp $r0, [ + #8]
L_15ec2: lwi.gp	$r3, [ + #-776]
L_15ec6: slt45 $r0, $r1
L_15ec8: mov55 $r7, $r6
L_15eca: movi55 $r11, #0
L_15ecc: swi	$r15, [$sp + #0xc]
L_15ed0: movi55 $r23, #-1
L_15ed2: sethi	$r24, #0x10000
L_15ed6: movi55 $r22, #0xa
L_15ed8: sethi	$lp, #0x90000
L_15edc: movi	$r25, #0x2710
L_15ee0: lwi37.sp $r0, [ + #8]
L_15ee2: slt45 $r11, $r0
L_15ee4: beqz	$r15, . + (446)
L_15ee8: movi55 $r0, #1
L_15eea: sw	$r0, [$r14 + ($r11 << #2)]
L_15eee: addri36.sp $r0, #0xf8
L_15ef0: add_slli	$r16, $r0, $r11, #2
L_15ef4: movi55 $r12, #0
L_15ef6: movi	$r0, #0x2d428
L_15efa: slt	$r15, $r12, $r1
L_15efe: beqz	$r15, . + (410)
L_15f02: lwi37.sp $r2, [ + #0xc]
L_15f04: lhsi	$r4, [$r7 + #0x18]
L_15f08: beqz38 $r2, . + (18)
L_15f0a: lhsi	$r2, [$r0 + #0x1c]
L_15f0e: lhsi	$r5, [$r0 + #0x1e]
L_15f12: sub45 $r4, $r2
L_15f14: lhsi	$r2, [$r7 + #0x1a]
L_15f18: j8 . + (16)
L_15f1a: lhsi	$r2, [$r0 + #0x18]
L_15f1e: lhsi	$r5, [$r0 + #0x1a]
L_15f22: sub45 $r4, $r2
L_15f24: lhsi	$r2, [$r7 + #0x1a]
L_15f28: sub45 $r2, $r5
L_15f2a: mul33 $r2, $r2
L_15f2c: maddr32	$r2, $r4, $r4
L_15f30: lhsi	$r20, [$r7 + #0x18]
L_15f34: lhsi	$r4, [$r0 + #0x18]
L_15f38: lhsi	$r21, [$r7 + #0x1a]
L_15f3c: sub	$r18, $r20, $r4
L_15f40: lhsi	$r4, [$r0 + #0x1a]
L_15f44: lwi	$r8, [$r0 + #0x24]
L_15f48: sub	$r19, $r21, $r4
L_15f4c: mul	$r5, $r19, $r19
L_15f50: maddr32	$r5, $r18, $r18
L_15f54: lwi450 $r10, [$r0]
L_15f56: bnec	$r8, #4, . + (14)
L_15f5a: lbi	$r4, [$r0 + #0x28]
L_15f5e: bnec	$r4, #1, . + (6)
L_15f62: ifcall9 . + (194)
L_15f64: lwi	$r4, [$r0 + #0x30]
L_15f68: bnez38 $r4, . + (178)
L_15f6a: lwi333 $r4, [$r3 + #4]
L_15f6c: slt45 $r5, $r4
L_15f6e: bnez	$r15, . + (1626)
L_15f72: lhi333 $r4, [$r3 + #2]
L_15f74: mul	$r4, $r10, $r4
L_15f78: slt45 $r5, $r4
L_15f7a: beqzs8 . + (96)
L_15f7c: lwi333 $r4, [$r3 + #8]
L_15f7e: slt45 $r5, $r4
L_15f80: beqzs8 . + (22)
L_15f82: lwi333 $r4, [$r0 + #0x10]
L_15f84: lwi	$r10, [$r0 + #0xc]
L_15f88: mul	$r4, $r19, $r4
L_15f8c: maddr32	$r4, $r18, $r10
L_15f90: bgtz	$r4, . + (1592)
L_15f94: j8 . + (70)
L_15f96: lwi333 $r4, [$r3 + #0xc]
L_15f98: slt45 $r5, $r4
L_15f9a: beqzs8 . + (64)
L_15f9c: lwi333 $r4, [$r0 + #0x10]
L_15f9e: lwi	$r17, [$r0 + #0xc]
L_15fa2: mul	$r4, $r19, $r4
L_15fa6: maddr32	$r4, $r18, $r17
L_15faa: movi	$r13, #0x64
L_15fae: mul	$r18, $r4, $r13
L_15fb2: lbi	$r17, [$r3 + #0x27]
L_15fb6: divr	$r18, $r18, $r18, $r10
L_15fba: mul	$r18, $r18, $r18
L_15fbe: mul	$r17, $r5, $r17
L_15fc2: mul	$r18, $r18, $r10
L_15fc6: divr	$r17, $r17, $r17, $r13
L_15fca: divr	$r15, $r15, $r18, $r25
L_15fce: slt45 $r17, $r15
L_15fd0: beqzs8 . + (10)
L_15fd2: blez	$r4, . + (8)
L_15fd6: bnez	$r10, . + (1522)
L_15fda: lbi	$r4, [$r0 + #0x29]
L_15fde: lbi	$r17, [$r3 + #0]
L_15fe2: slt45 $r17, $r4
L_15fe4: beqzs8 . + (10)
L_15fe6: lwi333 $r4, [$r3 + #0x10]
L_15fe8: slt45 $r5, $r4
L_15fea: bnez	$r15, . + (1502)
L_15fee: lbi	$r4, [$r3 + #0x26]
L_15ff2: beqz	$r4, . + (1512)
L_15ff6: lwi333 $r4, [$r3 + #0xc]
L_15ff8: mul33 $r4, $r4
L_15ffa: slt45 $r5, $r4
L_15ffc: beqz	$r15, . + (1502)
L_16000: bnec	$r8, #4, . + (6)
L_16004: ifcall9 . + (32)
L_16006: lbi	$r4, [$r7 + #0x35]
L_1600a: bnez	$r4, . + (1488)
L_1600e: lbi	$r4, [$r0 + #0x35]
L_16012: bnez	$r4, . + (1480)
L_16016: add45 $r2, $lp
L_16018: j8 . + (110)
L_1601a: beqc	$r4, #2, . + (6)
L_1601e: ifcall9 . + (6)
L_16020: bnec	$r8, #4, . + (8)
L_16024: j	. + (1462)
L_16028: lwi	$r15, [$r3 + #4]
L_1602c: lhsi	$r10, [$r0 + #0x1c]
L_16030: slt45 $r5, $r15
L_16032: lwi	$r13, [$r0 + #0xc]
L_16036: lhsi	$r4, [$r0 + #0x1e]
L_1603a: lwi	$r17, [$r0 + #0x10]
L_1603e: bnez	$r15, . + (1418)
L_16042: lbi	$r5, [$r3 + #0x28]
L_16046: add45 $r4, $r17
L_16048: mul	$r8, $r13, $r5
L_1604c: mul	$r5, $r17, $r5
L_16050: divsr	$r8, $r8, $r8, $r22
L_16054: divsr	$r5, $r5, $r5, $r22
L_16058: sub	$r4, $r21, $r4
L_1605c: add45 $r10, $r13
L_1605e: sub	$r10, $r20, $r10
L_16062: mul33 $r4, $r4
L_16064: mul33 $r5, $r5
L_16066: maddr32	$r4, $r10, $r10
L_1606a: maddr32	$r5, $r8, $r8
L_1606e: slts45 $r4, $r5
L_16070: beqz	$r15, . + (1386)
L_16074: mul	$r4, $r19, $r17
L_16078: maddr32	$r4, $r18, $r13
L_1607c: blez	$r4, . + (1374)
L_16080: sethi	$r4, #0x80000
L_16084: add45 $r2, $r4
L_16086: swi450 $r2, [$r16]
L_16088: addi	$r12, $r12, #1
L_1608c: addi	$r16, $r16, #0x30
L_16090: addi	$r0, $r0, #0x38
L_16094: j	. + (-410)
L_16098: addi45 $r11, #1
L_1609a: addi	$r7, $r7, #0x38
L_1609e: j	. + (-446)
L_160a2: movi55 $r0, #0
L_160a4: movi55 $r2, #1
L_160a6: slt45 $r0, $r1
L_160a8: beqzs8 . + (10)
L_160aa: sw	$r2, [$r9 + ($r0 << #2)]
L_160ae: addi45 $r0, #1
L_160b0: j8 . + (-10)
L_160b2: lwi37.sp $r0, [ + #8]
L_160b4: slti45 $r0, #0xb
L_160b6: beqzs8 . + (20)
L_160b8: slti45 $r1, #0xb
L_160ba: beqzs8 . + (16)
L_160bc: mov55 $r2, $r14
L_160be: mov55 $r3, $r9
L_160c0: addri36.sp $r4, #0xf8
L_160c2: addri36.sp $r5, #0x20
L_160c4: jal	. + (1930)
L_160c8: j8 . + (16)
L_160ca: lwi37.sp $r0, [ + #8]
L_160cc: mov55 $r2, $r14
L_160ce: mov55 $r3, $r9
L_160d0: addri36.sp $r4, #0xf8
L_160d2: addri36.sp $r5, #0x20
L_160d4: jal	. + (1780)
L_160d8: lwi.gp	$r1, [ + #-776]
L_160dc: sethi	$r5, #0x8ffff
L_160e0: swi37.sp $r1, [ + #0xc]
L_160e2: movi	$r2, #0x38
L_160e6: movi55 $r1, #0
L_160e8: addi	$r4, $r5, #0xfff
L_160ec: mov55 $r3, $r1
L_160ee: movi	$r13, #0x2d428
L_160f2: movi55 $r24, #0xc
L_160f4: swi37.sp $r4, [ + #0x1c]
L_160f6: mov55 $r8, $r2
L_160f8: addi	$lp, $sp, #0xc8
L_160fc: movi55 $r10, #1
L_160fe: addi	$r25, $sp, #0x98
L_16102: movi55 $r20, #2
L_16104: movi55 $r23, #3
L_16106: slt45 $r3, $r0
L_16108: movi	$r11, #0x2d428
L_1610c: beqz	$r15, . + (492)
L_16110: addri36.sp $r4, #0x20
L_16112: add_slli	$r5, $r4, $r3, #1
L_16116: lbi333 $r5, [$r5 + #1]
L_16118: mov55 $r7, $r13
L_1611a: maddr32	$r7, $r5, $r2
L_1611e: lwi	$r7, [$r7 + #0x30]
L_16122: bnec	$r7, #2, . + (30)
L_16126: lb	$r7, [$r4 + ($r3 << #1)]
L_1612a: maddr32	$r7, $r5, $r24
L_1612e: addri36.sp $r4, #0xf8
L_16130: lw	$r7, [$r4 + ($r7 << #2)]
L_16134: bltz	$r7, . + (12)
L_16138: sw	$r10, [$r9 + ($r5 << #2)]
L_1613c: j	. + (438)
L_16140: addri36.sp $r4, #0x20
L_16142: lb	$r7, [$r4 + ($r3 << #1)]
L_16146: ex9.it #264
L_16148: maddr32	$r4, $r5, $r2
L_1614c: mov55 $r16, $r6
L_1614e: ex9.it #373
L_16150: lbi	$r18, [$r4 + #0x36]
L_16154: lwi37.sp $r4, [ + #0x14]
L_16156: addi	$r17, $r16, #0x30
L_1615a: addi	$r16, $r16, #0x28
L_1615e: lwi	$r11, [$r16 + #4]
L_16162: lwi	$r19, [$r4 + #0xc]
L_16166: lwi37.sp $r4, [ + #0xc]
L_16168: lh	$r15, [$r19 + ($r11 << #1)]
L_1616c: lhi	$r11, [$r4 + #0x2a]
L_16170: sbi	$r18, [$r17 + #6]
L_16174: slt45 $r11, $r15
L_16176: beqzs8 . + (24)
L_16178: bnez	$r18, . + (22)
L_1617c: lbi	$r18, [$r16 + #2]
L_16180: movi55 $r4, #1
L_16182: ori	$r18, $r18, #1
L_16186: sbi	$r18, [$r16 + #2]
L_1618a: sbi	$r4, [$r17 + #6]
L_1618e: mov55 $r16, $r6
L_16190: ex9.it #373
L_16192: ex9.it #264
L_16194: maddr32	$r4, $r5, $r2
L_16198: addi	$r15, $r16, #0x18
L_1619c: lhsi	$r11, [$r15 + #2]
L_161a0: lhsi	$r18, [$r4 + #0x1a]
L_161a4: lhsi	$r12, [$r16 + #0x18]
L_161a8: sub	$r18, $r11, $r18
L_161ac: lhsi	$r19, [$r4 + #0x18]
L_161b0: mul	$r22, $r18, $r18
L_161b4: swi	$r18, [$r16 + #0x10]
L_161b8: add45 $r18, $r11
L_161ba: shi	$r18, [$r15 + #6]
L_161be: mov55 $r18, $r7
L_161c0: sub	$r19, $r12, $r19
L_161c4: maddr32	$r18, $r5, $r24
L_161c8: mov55 $r21, $r16
L_161ca: maddr32	$r22, $r19, $r19
L_161ce: mov55 $r17, $r4
L_161d0: addri36.sp $r4, #0xf8
L_161d2: swi.bi	$r22, [$r21], #8
L_161d6: lw	$r18, [$r4 + ($r18 << #2)]
L_161da: lwi37.sp $r4, [ + #0x1c]
L_161dc: swi	$r19, [$r21 + #4]
L_161e0: add45 $r19, $r12
L_161e2: shi	$r19, [$r15 + #4]
L_161e6: slt45 $r4, $r18
L_161e8: addi	$r21, $r16, #0x10
L_161ec: beqzs8 . + (26)
L_161ee: addi45 $r17, #0x10
L_161f0: swi	$r20, [$r16 + #0x30]
L_161f4: lhi	$r16, [$r17 + #4]
L_161f8: shi	$r16, [$r21 + #4]
L_161fc: lhi	$r16, [$r17 + #6]
L_16200: shi	$r16, [$r21 + #6]
L_16204: j8 . + (8)
L_16206: movi55 $r4, #0
L_16208: swi	$r4, [$r16 + #0x30]
L_1620c: mov55 $r16, $r6
L_1620e: ex9.it #264
L_16210: ex9.it #373
L_16212: maddr32	$r4, $r5, $r2
L_16216: addi	$r11, $r16, #0x10
L_1621a: addi	$r19, $r4, #0x10
L_1621e: lhsi	$r12, [$r19 + #4]
L_16222: lhsi	$r18, [$r11 + #4]
L_16226: lhsi	$r19, [$r19 + #6]
L_1622a: sub45 $r18, $r12
L_1622c: swi	$r18, [$r16 + #4]
L_16230: lhsi	$r18, [$r11 + #6]
L_16234: sub45 $r18, $r19
L_16236: lbi	$r19, [$r4 + #0x20]
L_1623a: sbi	$r19, [$r16 + #0x20]
L_1623e: lwi	$r19, [$r4 + #0x24]
L_16242: swi	$r18, [$r16 + #8]
L_16246: addi	$r18, $r16, #0x20
L_1624a: bnec	$r19, #3, . + (46)
L_1624e: lbi	$r18, [$r16 + #0x36]
L_16252: lbi	$r17, [$r4 + #0x28]
L_16256: bnec	$r18, #1, . + (6)
L_1625a: subi45 $r17, #1
L_1625c: sbi	$r17, [$r16 + #0x28]
L_16260: mov55 $r16, $r6
L_16262: ex9.it #373
L_16264: mov55 $r4, $r10
L_16266: lbi	$r17, [$r16 + #0x28]
L_1626a: addi	$r16, $r16, #0x20
L_1626e: cmovn	$r4, $r23, $r17
L_16272: swi	$r4, [$r16 + #4]
L_16276: j8 . + (6)
L_16278: swi	$r20, [$r18 + #4]
L_1627c: mov55 $r16, $r13
L_1627e: maddr32	$r16, $r5, $r2
L_16282: lbi	$r16, [$r16 + #0x34]
L_16286: slti	$r15, $r16, #0xc8
L_1628a: beqzs8 . + (8)
L_1628c: addi45 $r16, #1
L_1628e: andi	$r16, $r16, #0xff
L_16292: mov55 $r18, $r6
L_16294: ex9.it #264
L_16296: maddr32	$r18, $r7, $r8
L_1629a: maddr32	$r4, $r5, $r8
L_1629e: sbi	$r16, [$r18 + #0x34]
L_162a2: lbi	$r16, [$r4 + #0x29]
L_162a6: lwi37.sp $r4, [ + #0xc]
L_162a8: addi	$r17, $r18, #0x28
L_162ac: lwi333 $r5, [$r4 + #0x14]
L_162ae: lwi450 $r18, [$r18]
L_162b0: sbi	$r16, [$r17 + #1]
L_162b4: slt45 $r5, $r18
L_162b6: beqzs8 . + (10)
L_162b8: slti45 $r16, #0xa
L_162ba: beqzs8 . + (24)
L_162bc: addi45 $r16, #1
L_162be: j8 . + (16)
L_162c0: lwi37.sp $r4, [ + #0xc]
L_162c2: lwi333 $r5, [$r4 + #0x18]
L_162c4: slt45 $r18, $r5
L_162c6: beqzs8 . + (12)
L_162c8: beqz	$r16, . + (10)
L_162cc: subi45 $r16, #1
L_162ce: sbi	$r16, [$r17 + #1]
L_162d2: lwi37.sp $r4, [ + #0x10]
L_162d4: mov55 $r5, $r6
L_162d6: subi45 $r4, #1
L_162d8: maddr32	$r5, $r7, $r2
L_162dc: swi37.sp $r4, [ + #0x10]
L_162de: lwi37.sp $r4, [ + #4]
L_162e0: lbi	$r5, [$r5 + #0x20]
L_162e4: subi45 $r4, #1
L_162e6: sw	$r10, [$lp + ($r5 << #2)]
L_162ea: sw	$r10, [$r25 + ($r7 << #2)]
L_162ee: swi37.sp $r4, [ + #4]
L_162f0: addi45 $r1, #1
L_162f2: addi45 $r3, #1
L_162f4: j	. + (-494)
L_162f8: movi55 $r5, #0
L_162fa: mov55 $r3, $r9
L_162fc: mov55 $r0, $r11
L_162fe: mov55 $r10, $r5
L_16300: addri36.sp $r7, #0x98
L_16302: movi	$r12, #0x38
L_16306: movi55 $r9, #1
L_16308: movi55 $r13, #5
L_1630a: movi55 $r8, #4
L_1630c: lbi.gp	$r2, [ + #-784]
L_16310: slt45 $r5, $r2
L_16312: lwi.gp	$r2, [ + #-780]
L_16316: beqzs8 . + (6)
L_16318: lwi37.sp $r4, [ + #4]
L_1631a: bnez38 $r4, . + (20)
L_1631c: movi55 $r3, #0
L_1631e: movi55 $r16, #1
L_16320: mov55 $r0, $r6
L_16322: mov55 $r10, $r3
L_16324: mov55 $r17, $r3
L_16326: mov55 $r8, $r16
L_16328: movi55 $r12, #3
L_1632a: j	. + (294)
L_1632e: lbi	$r15, [$r2 + #9]
L_16332: slt45 $r1, $r15
L_16334: beqzs8 . + (-24)
L_16336: lwi.bi	$r16, [$r3], #4
L_1633a: bnec	$r16, #1, . + (242)
L_1633e: lwi	$r16, [$r0 + #0x24]
L_16342: bnec	$r16, #3, . + (16)
L_16346: lwi37.sp $r2, [ + #4]
L_16348: swi	$r10, [$r3 + #-4]
L_1634c: subi45 $r2, #1
L_1634e: swi37.sp $r2, [ + #4]
L_16350: j8 . + (220)
L_16352: subi45 $r16, #1
L_16354: slti45 $r16, #2
L_16356: beqzs8 . + (26)
L_16358: lwi	$r16, [$r0 + #0x30]
L_1635c: bnec	$r16, #2, . + (12)
L_16360: movi55 $r4, #0
L_16362: sbi	$r4, [$r0 + #0x28]
L_16366: j8 . + (20)
L_16368: lwi37.sp $r4, [ + #0xc]
L_1636a: lbi	$r16, [$r4 + #1]
L_1636e: j8 . + (8)
L_16370: lbi	$r16, [$r0 + #0x28]
L_16374: subi45 $r16, #1
L_16376: sbi	$r16, [$r0 + #0x28]
L_1637a: lbi	$r16, [$r0 + #0x28]
L_1637e: bnec	$r16, #1, . + (16)
L_16382: lbi	$r17, [$r0 + #0x2a]
L_16386: ori	$r17, $r17, #4
L_1638a: sbi	$r17, [$r0 + #0x2a]
L_1638e: lbi	$r2, [$r2 + #0x19]
L_16392: movi55 $lp, #0
L_16394: slt	$r15, $lp, $r2
L_16398: beqzs8 . + (70)
L_1639a: lw	$r15, [$r7 + ($lp << #2)]
L_1639e: bnezs8 . + (58)
L_163a0: lw	$r15, [$r14 + ($lp << #2)]
L_163a4: bnezs8 . + (52)
L_163a6: mov55 $r2, $r8
L_163a8: cmovz	$r2, $r13, $r16
L_163ac: swi	$r2, [$r0 + #0x24]
L_163b0: mov55 $r15, $r0
L_163b2: mov55 $r2, $r6
L_163b4: lmw.bim $r16, [$r15], $r25, #0x0
L_163b8: maddr32	$r2, $lp, $r12
L_163bc: smw.bim $r16, [$r2], $r25, #0x0
L_163c0: lmw.bi $r16, [$r15], $r19, #0x0
L_163c4: smw.bi $r16, [$r2], $r19, #0x0
L_163c8: lbi	$r2, [$r0 + #0x20]
L_163cc: addri36.sp $r4, #0xc8
L_163ce: sw	$r9, [$r4 + ($r2 << #2)]
L_163d2: sw	$r9, [$r7 + ($lp << #2)]
L_163d6: j8 . + (74)
L_163d8: addi	$lp, $lp, #1
L_163dc: j8 . + (-72)
L_163de: movi55 $r15, #0
L_163e0: j8 . + (6)
L_163e2: addi	$r15, $r15, #1
L_163e6: beq	$r15, $r2, . + (70)
L_163ea: lw	$r4, [$r7 + ($r15 << #2)]
L_163ee: bnez38 $r4, . + (-12)
L_163f0: mov55 $r2, $r8
L_163f2: cmovz	$r2, $r13, $r16
L_163f6: swi	$r2, [$r0 + #0x24]
L_163fa: mov55 $lp, $r0
L_163fc: mov55 $r2, $r6
L_163fe: lmw.bim $r16, [$lp], $r25, #0x0
L_16402: maddr32	$r2, $r15, $r12
L_16406: smw.bim $r16, [$r2], $r25, #0x0
L_1640a: lmw.bi $r16, [$lp], $r19, #0x0
L_1640e: smw.bi $r16, [$r2], $r19, #0x0
L_16412: lbi	$r2, [$r0 + #0x20]
L_16416: addri36.sp $r4, #0xc8
L_16418: sw	$r9, [$r4 + ($r2 << #2)]
L_1641c: sw	$r9, [$r7 + ($r15 << #2)]
L_16420: lwi37.sp $r2, [ + #4]
L_16422: swi	$r10, [$r3 + #-4]
L_16426: subi45 $r2, #1
L_16428: swi37.sp $r2, [ + #4]
L_1642a: addi45 $r1, #1
L_1642c: addi45 $r5, #1
L_1642e: addi	$r0, $r0, #0x38
L_16432: j	. + (-294)
L_16436: lwi37.sp $r4, [ + #0x10]
L_16438: beqz38 $r4, . + (224)
L_1643a: lbi	$r18, [$r2 + #9]
L_1643e: slt45 $r1, $r18
L_16440: beqzs8 . + (216)
L_16442: lw	$r5, [$r14 + ($r3 << #2)]
L_16446: beqc	$r5, #1, . + (18)
L_1644a: addi45 $r3, #1
L_1644c: addi	$r0, $r0, #0x38
L_16450: lwi37.sp $r4, [ + #8]
L_16452: slt45 $r3, $r4
L_16454: bnezs8 . + (-30)
L_16456: j8 . + (194)
L_16458: addri36.sp $r4, #0x98
L_1645a: lw	$r5, [$r4 + ($r3 << #2)]
L_1645e: bnez38 $r5, . + (-20)
L_16460: addri36.sp $r4, #0xc8
L_16462: lw	$r7, [$r4 + ($r5 << #2)]
L_16466: bnez38 $r7, . + (170)
L_16468: lwi37.sp $r4, [ + #0xc]
L_1646a: swi450 $r7, [$r0]
L_1646c: swi333 $r7, [$r0 + #4]
L_1646e: swi333 $r7, [$r0 + #8]
L_16470: swi333 $r7, [$r0 + #0xc]
L_16472: swi333 $r7, [$r0 + #0x10]
L_16474: lhi	$r7, [$r0 + #0x18]
L_16478: shi	$r7, [$r0 + #0x1c]
L_1647c: lhi	$r7, [$r0 + #0x1a]
L_16480: shi	$r7, [$r0 + #0x1e]
L_16484: add333 $r7, $r4, $r1
L_16486: lwi37.sp $r4, [ + #0x14]
L_16488: lbi	$r7, [$r7 + #0x1c]
L_1648c: lwi	$r19, [$r4 + #0xc]
L_16490: lwi37.sp $r4, [ + #0xc]
L_16492: sbi	$r5, [$r0 + #0x20]
L_16496: sbi	$r7, [$r0 + #0x28]
L_1649a: lwi	$r9, [$r0 + #0x2c]
L_1649e: lhi	$r18, [$r4 + #0x2a]
L_164a2: bnez38 $r7, . + (26)
L_164a4: lh	$r7, [$r19 + ($r9 << #1)]
L_164a8: slt45 $r18, $r7
L_164aa: beqzs8 . + (8)
L_164ac: swi	$r16, [$r0 + #0x24]
L_164b0: j8 . + (30)
L_164b2: swi	$r12, [$r0 + #0x24]
L_164b6: sbi	$r8, [$r0 + #0x28]
L_164ba: j8 . + (20)
L_164bc: lh	$r15, [$r19 + ($r9 << #1)]
L_164c0: swi	$r12, [$r0 + #0x24]
L_164c4: slt45 $r18, $r15
L_164c6: bnezs8 . + (8)
L_164c8: addi45 $r7, #1
L_164ca: sbi	$r7, [$r0 + #0x28]
L_164ce: lh	$r7, [$r19 + ($r9 << #1)]
L_164d2: addri36.sp $r4, #0xc8
L_164d4: slt45 $r18, $r7
L_164d6: mov55 $r7, $r8
L_164d8: cmovz	$r7, $r17, $r15
L_164dc: sw	$r16, [$r4 + ($r5 << #2)]
L_164e0: addri36.sp $r4, #0x98
L_164e2: sbi	$r7, [$r0 + #0x36]
L_164e6: sw	$r16, [$r4 + ($r3 << #2)]
L_164ea: lbi	$r7, [$r0 + #0x2a]
L_164ee: lwi37.sp $r4, [ + #0x10]
L_164f0: ori	$r7, $r7, #1
L_164f4: subi45 $r4, #1
L_164f6: sbi	$r17, [$r0 + #0x34]
L_164fa: sbi	$r17, [$r0 + #0x29]
L_164fe: sbi	$r7, [$r0 + #0x2a]
L_16502: swi	$r10, [$r0 + #0x30]
L_16506: sw	$r10, [$r14 + ($r3 << #2)]
L_1650a: swi37.sp $r4, [ + #0x10]
L_1650c: addi45 $r1, #1
L_1650e: j8 . + (-196)
L_16510: addi45 $r5, #1
L_16512: slt45 $r5, $r18
L_16514: bnezs8 . + (-180)
L_16516: j8 . + (-204)
L_16518: movi55 $r0, #0
L_1651a: movi	$r3, #0x4c
L_1651e: mov55 $r4, $r0
L_16520: lbi	$r1, [$r2 + #9]
L_16524: slt45 $r0, $r1
L_16526: beqzs8 . + (16)
L_16528: mul	$r1, $r0, $r3
L_1652c: lwi37.sp $r5, [ + #0x18]
L_1652e: addi45 $r0, #1
L_16530: sw	$r4, [$r5 + ($r1 << #0)]
L_16534: j8 . + (-20)
L_16536: movi55 $r0, #0
L_16538: sbi.gp	$r0, [ + #-784]
L_1653c: movi55 $r1, #0
L_1653e: addi	$r9, $sp, #0x98
L_16542: movi	$r10, #0x4c
L_16546: movi	$r12, #0x38
L_1654a: lbi	$r0, [$r2 + #0x19]
L_1654e: slt45 $r1, $r0
L_16550: beqzs8 . + (146)
L_16552: ex9.it #265
L_16554: bnec	$r0, #1, . + (108)
L_16558: lwi	$r7, [$r6 + #0x24]
L_1655c: beqc	$r7, #5, . + (38)
L_16560: lbi.gp	$r3, [ + #-784]
L_16564: mov55 $r0, $r11
L_16566: mov55 $r5, $r6
L_16568: lmw.bim $r16, [$r5], $r25, #0x0
L_1656c: maddr32	$r0, $r3, $r12
L_16570: addi45 $r3, #1
L_16572: smw.bim $r16, [$r0], $r25, #0x0
L_16576: lmw.bi $r16, [$r5], $r19, #0x0
L_1657a: smw.bi $r16, [$r0], $r19, #0x0
L_1657e: sbi.gp	$r3, [ + #-784]
L_16582: lbi	$r0, [$r6 + #0x20]
L_16586: lwi37.sp $r3, [ + #0x18]
L_16588: maddr32	$r3, $r0, $r10
L_1658c: mov55 $r0, $r3
L_1658e: swi450 $r7, [$r3]
L_16590: lbi	$r3, [$r6 + #0x2a]
L_16594: sbi	$r3, [$r0 + #0x18]
L_16598: lhi	$r3, [$r6 + #0x14]
L_1659c: shi333 $r3, [$r0 + #4]
L_1659e: lhi	$r3, [$r6 + #0x16]
L_165a2: shi333 $r3, [$r0 + #6]
L_165a4: lwi	$r3, [$r6 + #0x2c]
L_165a8: swi333 $r3, [$r0 + #0x1c]
L_165aa: lwi450 $r3, [$r6]
L_165ac: addi	$r4, $r0, #0x24
L_165b0: swi450 $r3, [$r4]
L_165b2: lwi333 $r3, [$r6 + #4]
L_165b4: swi333 $r3, [$r4 + #4]
L_165b6: lwi333 $r3, [$r6 + #8]
L_165b8: swi333 $r3, [$r4 + #8]
L_165ba: lwi	$r3, [$r6 + #0x30]
L_165be: swi333 $r3, [$r4 + #0xc]
L_165c0: addi45 $r1, #1
L_165c2: addi	$r6, $r6, #0x38
L_165c6: j8 . + (-124)
L_165c8: beqc	$r8, #3, . + (6)
L_165cc: ifcall9 . + (10)
L_165ce: ex9.it #461
L_165d0: bnez	$r4, . + (-1354)
L_165d4: add45 $r2, $r24
L_165d6: j	. + (-1360)
L_165da: swi	$r23, [$r16 + #0]
L_165de: j	. + (-1366)
L_165e2: addi	$sp, $sp, #0x5d8
L_165e6: pop25 $r14, #0
L_165e8: movi55 $r0, #0
L_165ea: sbi.gp	$r0, [ + #-784]
L_165ee: ret5 $lp
L_165f0: movi	$r0, #0x2b5b0
L_165f4: swi.gp	$r0, [ + #-788]
L_165f8: ret5 $lp
L_165fa: push25 $r14, #56
L_165fc: lbi	$r5, [$r1 + #9]
L_16600: lhsi	$r17, [$r0 + #4]
L_16604: lhsi	$r18, [$r0 + #6]
L_16608: beqz38 $r5, . + (30)
L_1660a: lwi.gp	$r4, [ + #-780]
L_1660e: lbi333 $r3, [$r4 + #2]
L_16610: subi45 $r3, #1
L_16612: beqs38 $r3, . + (20)
L_16614: lbi	$r3, [$r1 + #8]
L_16618: beqz38 $r3, . + (14)
L_1661a: lbi333 $r4, [$r4 + #3]
L_1661c: subi45 $r4, #1
L_1661e: xor33 $r4, $r3
L_16620: slti	$r4, $r4, #1
L_16624: j8 . + (4)
L_16626: movi55 $r4, #1
L_16628: lbi.gp	$r6, [ + #0x1346]
L_1662c: movi55 $r3, #0
L_1662e: sbi.gp	$r3, [ + #-1478]
L_16632: movi	$r3, #0x2d3f0
L_16636: bnez38 $r6, . + (58)
L_16638: lwi	$r16, [$r0 + #0x44]
L_1663c: ex9.it #333
L_1663e: ex9.it #248
L_16640: and	$r5, $r16, $r5
L_16644: movi	$r16, #0x10000
L_16648: bne	$r5, $r16, . + (42)
L_1664c: movi55 $r5, #3
L_1664e: swi.gp	$r5, [ + #-1484]
L_16652: movi55 $r5, #1
L_16654: sbi.gp	$r6, [ + #-1480]
L_16658: shi.gp	$r6, [ + #-1496]
L_1665c: shi.gp	$r6, [ + #-1494]
L_16660: shi.gp	$r6, [ + #-1500]
L_16664: shi.gp	$r6, [ + #-1498]
L_16668: sbi.gp	$r5, [ + #0x1346]
L_1666c: movi55 $r6, #1
L_1666e: j8 . + (4)
L_16670: movi55 $r6, #0
L_16672: lwi.gp	$r5, [ + #-1484]
L_16676: beqz38 $r2, . + (100)
L_16678: beqz38 $r5, . + (6)
L_1667a: bnec	$r5, #5, . + (18)
L_1667e: lwi.gp	$r6, [ + #-788]
L_16682: lbi	$r6, [$r6 + #0x1c]
L_16686: sbi.gp	$r6, [ + #-1480]
L_1668a: movi55 $r6, #1
L_1668c: lhsi.gp	$r16, [ + #-1494]
L_16690: lhsi.gp	$r19, [ + #-1496]
L_16694: sub	$r16, $r18, $r16
L_16698: mul	$r16, $r16, $r16
L_1669c: sub	$r19, $r17, $r19
L_166a0: mov55 $r11, $r16
L_166a2: maddr32	$r11, $r19, $r19
L_166a6: beqc	$r5, #3, . + (14)
L_166aa: beqz38 $r5, . + (10)
L_166ac: beqc	$r5, #5, . + (8)
L_166b0: j	. + (270)
L_166b4: lwi.gp	$r5, [ + #-788]
L_166b8: lhi333 $r7, [$r1 + #0]
L_166ba: lhi	$r5, [$r5 + #0x22]
L_166be: slt45 $r5, $r7
L_166c0: lbi.gp	$r7, [ + #-1480]
L_166c4: beqzs8 . + (14)
L_166c6: lhi333 $r1, [$r1 + #2]
L_166c8: slt45 $r5, $r1
L_166ca: beqzs8 . + (8)
L_166cc: beqz38 $r7, . + (6)
L_166ce: subi45 $r7, #1
L_166d0: zeb33 $r7, $r7
L_166d2: movi55 $r5, #3
L_166d4: bnez38 $r7, . + (236)
L_166d6: movi55 $r5, #1
L_166d8: j8 . + (232)
L_166da: subi333 $r1, $r5, #1
L_166dc: slti45 $r1, #2
L_166de: beqzs8 . + (10)
L_166e0: lwi.gp	$r1, [ + #-788]
L_166e4: lbi333 $r1, [$r1 + #1]
L_166e6: j8 . + (18)
L_166e8: bnec	$r5, #3, . + (8)
L_166ec: sbi.gp	$r2, [ + #-1480]
L_166f0: lbi.gp	$r1, [ + #-1480]
L_166f4: beqz38 $r1, . + (8)
L_166f6: subi45 $r1, #1
L_166f8: sbi.gp	$r1, [ + #-1480]
L_166fc: lbi.gp	$r7, [ + #-1480]
L_16700: bnec	$r7, #1, . + (12)
L_16704: movi55 $r1, #4
L_16706: sbi.gp	$r1, [ + #-1478]
L_1670a: j8 . + (6)
L_1670c: movi55 $r1, #5
L_1670e: beqz38 $r7, . + (4)
L_16710: movi55 $r1, #4
L_16712: swi.gp	$r1, [ + #-1484]
L_16716: mov55 $r5, $r3
L_16718: mov55 $r1, $sp
L_1671a: lmw.bim $r16, [$r5], $r25, #0x0
L_1671e: ex9.it #126
L_16720: lmw.bi $r16, [$r5], $r19, #0x0
L_16724: smw.bi $r16, [$r1], $r19, #0x0
L_16728: lwi.gp	$r11, [ + #-1520]
L_1672c: lwi.gp	$r10, [ + #-1516]
L_16730: lwi.gp	$r9, [ + #-1512]
L_16734: lhsi.gp	$r8, [ + #-1500]
L_16738: mov55 $r5, $r25
L_1673a: lhsi.gp	$r12, [ + #-1498]
L_1673e: lhsi.gp	$r17, [ + #-1496]
L_16742: lhsi.gp	$r18, [ + #-1494]
L_16746: lbi.gp	$r6, [ + #-1478]
L_1674a: lbi.gp	$r4, [ + #-1467]
L_1674e: movi55 $r1, #0
L_16750: swi450 $r1, [$r0]
L_16752: beqc	$r5, #5, . + (60)
L_16756: swi450 $r11, [$sp]
L_16758: ex9.it #474
L_1675a: swi	$r9, [$sp + #8]
L_1675e: shi	$r8, [$sp + #0x14]
L_16762: shi	$r12, [$sp + #0x16]
L_16766: shi	$r17, [$sp + #0x18]
L_1676a: shi	$r18, [$sp + #0x1a]
L_1676e: swi37.sp $r5, [ + #0x24]
L_16770: sbi	$r7, [$sp + #0x28]
L_16774: sbi	$r6, [$sp + #0x2a]
L_16778: sbi	$r4, [$sp + #0x35]
L_1677c: mov55 $r1, $sp
L_1677e: lmw.bim $r16, [$r1], $r25, #0x0
L_16782: smw.bim $r16, [$r3], $r25, #0x0
L_16786: lmw.bi $r16, [$r1], $r19, #0x0
L_1678a: smw.bi $r16, [$r3], $r19, #0x0
L_1678e: sbi.gp	$r2, [ + #-792]
L_16792: swi450 $r5, [$r0]
L_16794: sbi	$r6, [$r0 + #0x5a]
L_16798: shi	$r8, [$r0 + #8]
L_1679c: shi	$r12, [$r0 + #0xa]
L_167a0: swi	$r11, [$r0 + #0x5c]
L_167a4: swi	$r10, [$r0 + #0x60]
L_167a8: swi	$r9, [$r0 + #0x64]
L_167ac: swi	$r2, [$r0 + #0x68]
L_167b0: bnec	$r5, #1, . + (22)
L_167b4: shi	$r8, [$r0 + #0x38]
L_167b8: shi	$r12, [$r0 + #0x3a]
L_167bc: j8 . + (10)
L_167be: movi55 $r5, #2
L_167c0: mov55 $r12, $r18
L_167c2: mov55 $r8, $r17
L_167c4: j8 . + (-118)
L_167c6: pop25 $r14, #56
L_167c8: push25 $r14, #0
L_167ca: mov55 $r11, $r0
L_167cc: mov55 $r17, $r0
L_167ce: movi55 $r0, #0
L_167d0: mov55 $r18, $r1
L_167d2: mov55 $r9, $r0
L_167d4: mov55 $r19, $r0
L_167d6: movi55 $r7, #1
L_167d8: mov55 $r8, $r0
L_167da: movi	$r14, #0x30
L_167de: beqz	$r17, . + (110)
L_167e2: beqz	$r18, . + (106)
L_167e6: bnec	$r7, #1, . + (102)
L_167ea: movi55 $r6, #0
L_167ec: mov55 $r7, $r6
L_167ee: movi55 $r10, #-2
L_167f0: beq	$r6, $r11, . + (60)
L_167f4: lw	$r16, [$r2 + ($r6 << #2)]
L_167f8: bnec	$r16, #1, . + (48)
L_167fc: add_slli	$r20, $r4, $r6, #2
L_16800: movi55 $r16, #0
L_16802: beq	$r16, $r1, . + (38)
L_16806: lw	$r13, [$r3 + ($r16 << #2)]
L_1680a: bnec	$r13, #1, . + (26)
L_1680e: mul	$r12, $r16, $r14
L_16812: lw	$r12, [$r20 + ($r12 << #0)]
L_16816: slt	$r15, $r12, $r10
L_1681a: beqzs8 . + (10)
L_1681c: mov55 $r9, $r16
L_1681e: mov55 $r19, $r6
L_16820: mov55 $r7, $r13
L_16822: mov55 $r10, $r12
L_16824: addi45 $r16, #1
L_16826: j8 . + (-36)
L_16828: addi45 $r6, #1
L_1682a: j8 . + (-58)
L_1682c: beqz38 $r7, . + (-78)
L_1682e: add_slli	$r6, $r5, $r0, #1
L_16832: sbi	$r19, [$r6 + #0]
L_16836: sbi	$r9, [$r6 + #1]
L_1683a: addi333 $r6, $r0, #1
L_1683c: sw	$r8, [$r2 + ($r19 << #2)]
L_16840: zeb33 $r0, $r6
L_16842: sw	$r8, [$r3 + ($r9 << #2)]
L_16846: subi45 $r17, #1
L_16848: subi45 $r18, #1
L_1684a: j8 . + (-108)
L_1684c: pop25 $r14, #0
L_1684e: push25 $r14, #0
L_16850: addi	$sp, $sp, #-720
L_16854: mov55 $r6, $r0
L_16856: beqz38 $r0, . + (212)
L_16858: beqz38 $r1, . + (208)
L_1685a: slt45 $r1, $r0
L_1685c: beqzs8 . + (96)
L_1685e: mov55 $r7, $r1
L_16860: movi	$r17, #0x30
L_16864: movi55 $r18, #-1
L_16866: mov55 $r16, $r4
L_16868: maddr32	$r16, $r7, $r17
L_1686c: movi55 $r0, #0
L_1686e: sw	$r18, [$r16 + ($r0 << #2)]
L_16872: addi45 $r0, #1
L_16874: bne	$r0, $r6, . + (-6)
L_16878: addi45 $r7, #1
L_1687a: bne	$r7, $r6, . + (-20)
L_1687e: mov55 $r0, $r1
L_16880: mov55 $r14, $r1
L_16882: mov55 $r1, $r6
L_16884: mov55 $r13, $r5
L_16886: mov55 $r10, $r4
L_16888: mov55 $r12, $r3
L_1688a: mov55 $r11, $r2
L_1688c: jal	. + (-33608)
L_16890: addri36.sp $r0, #0x90
L_16892: jal	. + (-33600)
L_16896: mov55 $r0, $r10
L_16898: jal	. + (-33572)
L_1689c: mov55 $r0, $sp
L_1689e: jal	. + (-33554)
L_168a2: mov55 $r9, $r6
L_168a4: movi55 $r0, #0xc
L_168a6: movi55 $r6, #0
L_168a8: jal	. + (-33530)
L_168ac: jal	. + (-33524)
L_168b0: mov55 $r5, $r6
L_168b2: movi55 $r17, #0xc
L_168b4: movi	$r3, #0x30
L_168b8: mov55 $r2, $r6
L_168ba: j8 . + (64)
L_168bc: slt45 $r0, $r1
L_168be: beqzs8 . + (-64)
L_168c0: movi55 $r7, #0
L_168c2: movi	$r17, #0x30
L_168c6: movi55 $r18, #-1
L_168c8: mov55 $r16, $r4
L_168ca: maddr32	$r16, $r7, $r17
L_168ce: mov55 $r0, $r6
L_168d0: sw	$r18, [$r16 + ($r0 << #2)]
L_168d4: addi45 $r0, #1
L_168d6: bne	$r0, $r1, . + (-6)
L_168da: addi45 $r7, #1
L_168dc: bne	$r7, $r1, . + (-20)
L_168e0: j8 . + (-98)
L_168e2: add	$r0, $r16, $r4
L_168e6: lb	$r0, [$sp + ($r0 << #0)]
L_168ea: beqc	$r0, #1, . + (30)
L_168ee: addi45 $r16, #1
L_168f0: bne	$r16, $r9, . + (-14)
L_168f4: addi45 $r5, #1
L_168f6: beq	$r5, $r14, . + (52)
L_168fa: mov55 $r1, $r10
L_168fc: mul	$r4, $r5, $r17
L_16900: maddr32	$r1, $r5, $r3
L_16904: movi55 $r16, #0
L_16906: j8 . + (-36)
L_16908: lw	$r0, [$r1 + ($r16 << #2)]
L_1690c: beqc	$r0, #-1, . + (-30)
L_16910: add_slli	$r0, $r13, $r6, #1
L_16914: addi333 $r7, $r6, #1
L_16916: sbi	$r16, [$r0 + #0]
L_1691a: sbi333 $r5, [$r0 + #1]
L_1691c: zeb33 $r6, $r7
L_1691e: sw	$r2, [$r11 + ($r16 << #2)]
L_16922: sw	$r2, [$r12 + ($r5 << #2)]
L_16926: j8 . + (-56)
L_16928: mov55 $r6, $r1
L_1692a: mov55 $r0, $r6
L_1692c: addi	$sp, $sp, #0x2d0
L_16930: pop25 $r14, #0
L_16932: swi.gp	$r0, [ + #-480]
L_16936: ret5 $lp
L_16938: swi.gp	$r0, [ + #-484]
L_1693c: ret5 $lp
L_1693e: push25 $r14, #0
L_16940: lbi.gp	$r2, [ + #-4917]
L_16944: lwi.gp	$r6, [ + #-480]
L_16948: beqz38 $r2, . + (10)
L_1694a: movi55 $r2, #0xa
L_1694c: swi333 $r2, [$r6 + #4]
L_1694e: movpi45 $r2, #0x1e
L_16950: j8 . + (6)
L_16952: movpi45 $r2, #0x14
L_16954: swi333 $r2, [$r6 + #4]
L_16956: sbi333 $r2, [$r6 + #0]
L_16958: sbi333 $r2, [$r6 + #1]
L_1695a: addi	$r9, $r1, #-1
L_1695e: movi55 $r12, #2
L_16960: lwi.gp	$r20, [ + #-484]
L_16964: lwi.gp	$r4, [ + #-552]
L_16968: lwi.gp	$r2, [ + #-556]
L_1696c: divsr	$r12, $r12, $r9, $r12
L_16970: movi	$r19, #0x2d7ec
L_16974: addi45 $r0, #0xc
L_16976: movi	$r3, #0x2d7bc
L_1697a: movi	$r7, #0x2d79c
L_1697e: movi	$r18, #0x2d7a8
L_16982: movi55 $r17, #0
L_16984: movi55 $r8, #1
L_16986: movi	$r10, #0x2d70c
L_1698a: movi55 $r13, #0xc
L_1698c: movi55 $r14, #4
L_1698e: lbi	$r1, [$r20 + #9]
L_16992: slt45 $r17, $r1
L_16994: beqz	$r15, . + (424)
L_16998: lwi	$r5, [$r0 + #-12]
L_1699c: beqz	$r5, . + (400)
L_169a0: lbi	$r11, [$r0 + #0xc]
L_169a4: andi	$r1, $r11, #1
L_169a8: beqz38 $r1, . + (22)
L_169aa: sbi	$r8, [$r19 + #0]
L_169ae: lhsi	$r1, [$r0 + #0]
L_169b2: shi333 $r1, [$r3 + #0]
L_169b4: lhsi	$r1, [$r0 + #2]
L_169b8: shi333 $r1, [$r3 + #2]
L_169ba: j	. + (370)
L_169be: beqc	$r5, #4, . + (8)
L_169c2: j	. + (260)
L_169c6: lbi	$r2, [$r19 + #0]
L_169ca: bnez38 $r2, . + (118)
L_169cc: lbi	$r1, [$r18 + #0]
L_169d0: beqc	$r1, #1, . + (12)
L_169d4: lbi333 $r5, [$r7 + #0]
L_169d6: mov55 $r4, $r2
L_169d8: mov55 $r1, $r2
L_169da: j8 . + (60)
L_169dc: lbi	$r21, [$r7 + #0]
L_169e0: add_slli	$r1, $r10, $r17, #2
L_169e4: mov55 $r4, $r2
L_169e6: mov55 $r5, $r2
L_169e8: movi55 $r16, #1
L_169ea: bne	$r5, $r21, . + (6)
L_169ee: movi55 $r16, #2
L_169f0: lhsi	$r15, [$r1 + #0]
L_169f4: addi45 $r5, #1
L_169f6: maddr32	$r4, $r16, $r15
L_169fa: lhsi	$r15, [$r1 + #2]
L_169fe: zeb33 $r5, $r5
L_16a00: maddr32	$r2, $r16, $r15
L_16a04: addi	$r1, $r1, #0x30
L_16a08: bnec	$r5, #3, . + (-32)
L_16a0c: divsr	$r4, $r4, $r4, $r14
L_16a10: divsr	$r2, $r2, $r2, $r14
L_16a14: j8 . + (48)
L_16a16: mov55 $r16, $r17
L_16a18: maddr32	$r16, $r1, $r13
L_16a1c: addi45 $r1, #1
L_16a1e: lhs	$r15, [$r10 + ($r16 << #2)]
L_16a22: add_slli	$r16, $r10, $r16, #2
L_16a26: lhsi	$r16, [$r16 + #2]
L_16a2a: zeb33 $r1, $r1
L_16a2c: add45 $r4, $r15
L_16a2e: slt45 $r5, $r1
L_16a30: add45 $r2, $r16
L_16a32: beqzs8 . + (-28)
L_16a34: addi45 $r5, #1
L_16a36: divsr	$r4, $r4, $r4, $r5
L_16a3a: divsr	$r2, $r2, $r2, $r5
L_16a3e: j8 . + (6)
L_16a40: mov55 $r2, $r1
L_16a42: mov55 $r4, $r1
L_16a44: abs	$r1, $r2
L_16a48: abs	$r5, $r4
L_16a4c: add45 $r5, $r1
L_16a4e: lwi333 $r1, [$r6 + #4]
L_16a50: slt45 $r1, $r5
L_16a52: beqzs8 . + (218)
L_16a54: andi	$r1, $r11, #4
L_16a58: lbi333 $r5, [$r6 + #0]
L_16a5a: lbi	$r22, [$r6 + #2]
L_16a5e: beqz38 $r1, . + (26)
L_16a60: mul33 $r5, $r4
L_16a62: lhi333 $r1, [$r3 + #0]
L_16a64: divsr	$r5, $r5, $r5, $r22
L_16a68: add45 $r5, $r1
L_16a6a: shi333 $r5, [$r3 + #0]
L_16a6c: lbi333 $r1, [$r6 + #1]
L_16a6e: lbi333 $r5, [$r6 + #3]
L_16a70: mul33 $r1, $r2
L_16a72: divsr	$r1, $r1, $r1, $r5
L_16a76: j8 . + (52)
L_16a78: add	$r1, $r22, $r12
L_16a7c: divsr	$r16, $r16, $r1, $r9
L_16a80: lbi	$r21, [$r6 + #3]
L_16a84: min	$r5, $r16, $r5
L_16a88: add	$r15, $r21, $r12
L_16a8c: lbi333 $r1, [$r6 + #1]
L_16a8e: divsr	$r15, $r15, $r15, $r9
L_16a92: mul33 $r5, $r4
L_16a94: lhi	$r16, [$r3 + #0]
L_16a98: divsr	$r5, $r5, $r5, $r22
L_16a9c: min	$r1, $r15, $r1
L_16aa0: mul33 $r1, $r2
L_16aa2: add45 $r5, $r16
L_16aa4: shi333 $r5, [$r3 + #0]
L_16aa6: divsr	$r1, $r1, $r1, $r21
L_16aaa: lhi333 $r5, [$r3 + #2]
L_16aac: ori	$r11, $r11, #2
L_16ab0: add45 $r1, $r5
L_16ab2: shi333 $r1, [$r3 + #2]
L_16ab4: lhsi	$r1, [$r3 + #0]
L_16ab8: shi333 $r1, [$r0 + #0]
L_16aba: lhsi	$r1, [$r3 + #2]
L_16abe: shi333 $r1, [$r0 + #2]
L_16ac0: sbi	$r11, [$r0 + #0xc]
L_16ac4: j8 . + (104)
L_16ac6: beqc	$r5, #5, . + (102)
L_16aca: lbi	$r5, [$r19 + #0]
L_16ace: bnec	$r5, #1, . + (16)
L_16ad2: sbi	$r1, [$r18 + #0]
L_16ad6: sbi333 $r1, [$r7 + #0]
L_16ad8: sbi	$r1, [$r19 + #0]
L_16adc: j8 . + (32)
L_16ade: lbi333 $r5, [$r7 + #0]
L_16ae0: slti45 $r5, #2
L_16ae2: beqzs8 . + (8)
L_16ae4: addi45 $r5, #1
L_16ae6: sbi333 $r5, [$r7 + #0]
L_16ae8: j8 . + (4)
L_16aea: sbi333 $r1, [$r7 + #0]
L_16aec: lbi	$r1, [$r18 + #0]
L_16af0: bnez38 $r1, . + (12)
L_16af2: lbi333 $r1, [$r7 + #0]
L_16af4: bnec	$r1, #2, . + (8)
L_16af8: sbi	$r8, [$r18 + #0]
L_16afc: lbi333 $r1, [$r7 + #0]
L_16afe: mov55 $r5, $r17
L_16b00: maddr32	$r5, $r1, $r13
L_16b04: lhsi	$r11, [$r0 + #0]
L_16b08: mov55 $r1, $r5
L_16b0a: lhi333 $r5, [$r3 + #0]
L_16b0c: sub	$r5, $r11, $r5
L_16b10: sh	$r5, [$r10 + ($r1 << #2)]
L_16b14: lhsi	$r16, [$r0 + #2]
L_16b18: lhi333 $r5, [$r3 + #2]
L_16b1a: add_slli	$r1, $r10, $r1, #2
L_16b1e: sub	$r5, $r16, $r5
L_16b22: shi333 $r5, [$r1 + #2]
L_16b24: shi	$r11, [$r3 + #0]
L_16b28: shi	$r16, [$r3 + #2]
L_16b2c: addi45 $r17, #1
L_16b2e: addi45 $r19, #1
L_16b30: ex9.it #214
L_16b32: addi45 $r3, #4
L_16b34: addi45 $r7, #1
L_16b36: addi45 $r18, #1
L_16b38: j	. + (-426)
L_16b3c: swi.gp	$r4, [ + #-552]
L_16b40: swi.gp	$r2, [ + #-556]
L_16b44: pop25 $r14, #0
L_16b46: movi	$r0, #0x2b5d8
L_16b4a: swi.gp	$r0, [ + #-488]
L_16b4e: ret5 $lp
L_16b50: push25 $r10, #0
L_16b52: lwi450 $r2, [$r0]
L_16b54: beqz	$r2, . + (500)
L_16b58: lbi	$r7, [$r0 + #0x5a]
L_16b5c: xlsb33 $r3, $r7
L_16b5e: beqz38 $r3, . + (28)
L_16b60: movi55 $r1, #1
L_16b62: sbi.gp	$r1, [ + #-728]
L_16b66: lhi	$r1, [$r0 + #0x10]
L_16b6a: lhi	$r0, [$r0 + #0x12]
L_16b6e: shi.gp	$r1, [ + #-732]
L_16b72: shi.gp	$r0, [ + #-730]
L_16b76: j	. + (466)
L_16b7a: beqc	$r2, #4, . + (8)
L_16b7e: j	. + (346)
L_16b82: lbi.gp	$r2, [ + #-4794]
L_16b86: bnez	$r2, . + (450)
L_16b8a: lbi.gp	$r3, [ + #-728]
L_16b8e: swi.gp	$r2, [ + #-736]
L_16b92: swi.gp	$r2, [ + #-740]
L_16b96: bnez38 $r3, . + (130)
L_16b98: lbi.gp	$r2, [ + #-743]
L_16b9c: lbi.gp	$r5, [ + #-744]
L_16ba0: movi	$r6, #0x2d6ec
L_16ba4: beqc	$r2, #1, . + (10)
L_16ba8: mov55 $r2, $r3
L_16baa: mov55 $r4, $r3
L_16bac: j8 . + (66)
L_16bae: mov55 $r4, $r3
L_16bb0: mov55 $r2, $r3
L_16bb2: andi	$r16, $r4, #0xff
L_16bb6: bne	$r16, $r5, . + (8)
L_16bba: movi55 $r16, #2
L_16bbc: j8 . + (4)
L_16bbe: movi55 $r16, #1
L_16bc0: lhs	$r17, [$r6 + ($r4 << #2)]
L_16bc4: maddr32	$r3, $r16, $r17
L_16bc8: add_slli	$r17, $r6, $r4, #2
L_16bcc: lhsi	$r17, [$r17 + #2]
L_16bd0: addi45 $r4, #1
L_16bd2: maddr32	$r2, $r16, $r17
L_16bd6: bnec	$r4, #3, . + (-36)
L_16bda: movi55 $r4, #4
L_16bdc: divsr	$r3, $r3, $r3, $r4
L_16be0: divsr	$r2, $r2, $r2, $r4
L_16be4: swi.gp	$r3, [ + #-736]
L_16be8: swi.gp	$r2, [ + #-740]
L_16bec: j8 . + (44)
L_16bee: lhs	$r16, [$r6 + ($r4 << #2)]
L_16bf2: add45 $r3, $r16
L_16bf4: add_slli	$r16, $r6, $r4, #2
L_16bf8: addi45 $r4, #1
L_16bfa: lhsi	$r16, [$r16 + #2]
L_16bfe: zeb33 $r4, $r4
L_16c00: slt45 $r5, $r4
L_16c02: add45 $r2, $r16
L_16c04: beqzs8 . + (-22)
L_16c06: addi45 $r5, #1
L_16c08: divsr	$r3, $r3, $r3, $r5
L_16c0c: divsr	$r5, $r5, $r2, $r5
L_16c10: swi.gp	$r3, [ + #-736]
L_16c14: swi.gp	$r5, [ + #-740]
L_16c18: lwi.gp	$r5, [ + #-488]
L_16c1c: lwi.gp	$r4, [ + #-736]
L_16c20: lwi.gp	$r18, [ + #-740]
L_16c24: lwi333 $r3, [$r5 + #4]
L_16c26: abs	$r15, $r4
L_16c2a: abs	$r2, $r18
L_16c2e: add45 $r2, $r15
L_16c30: slli333 $r3, $r3, #1
L_16c32: slt45 $r3, $r2
L_16c34: beqz	$r15, . + (276)
L_16c38: lhi.gp	$r19, [ + #-732]
L_16c3c: lbi333 $r3, [$r5 + #0]
L_16c3e: addi	$r15, $r19, #-240
L_16c42: slti	$r15, $r15, #0xaa0
L_16c46: bnezs8 . + (4)
L_16c48: slli333 $r3, $r3, #1
L_16c4a: lhi.gp	$r16, [ + #-730]
L_16c4e: lbi333 $r2, [$r5 + #1]
L_16c50: addi	$r15, $r16, #-240
L_16c54: slti	$r15, $r15, #0x1220
L_16c58: bnezs8 . + (4)
L_16c5a: slli333 $r2, $r2, #1
L_16c5c: andi	$r6, $r7, #4
L_16c60: lbi	$r9, [$r5 + #2]
L_16c64: lbi	$r17, [$r5 + #3]
L_16c68: beqz38 $r6, . + (30)
L_16c6a: mul33 $r4, $r3
L_16c6c: mul	$r1, $r18, $r2
L_16c70: divsr	$r4, $r4, $r4, $r9
L_16c74: divsr	$r1, $r1, $r1, $r17
L_16c78: add	$r3, $r19, $r4
L_16c7c: shi.gp	$r3, [ + #-732]
L_16c80: add	$r2, $r16, $r1
L_16c84: j8 . + (54)
L_16c86: subi45 $r1, #1
L_16c88: movi55 $r6, #2
L_16c8a: divsr	$r6, $r6, $r1, $r6
L_16c8e: add	$r5, $r9, $r6
L_16c92: divsr	$r5, $r5, $r5, $r1
L_16c96: add45 $r6, $r17
L_16c98: divsr	$r1, $r1, $r6, $r1
L_16c9c: min	$r5, $r3, $r5
L_16ca0: min	$r1, $r2, $r1
L_16ca4: mul33 $r4, $r5
L_16ca6: mul	$r1, $r18, $r1
L_16caa: divsr	$r3, $r3, $r4, $r9
L_16cae: divsr	$r2, $r2, $r1, $r17
L_16cb2: add45 $r3, $r19
L_16cb4: shi.gp	$r3, [ + #-732]
L_16cb8: add45 $r2, $r16
L_16cba: lhi.gp	$r1, [ + #-732]
L_16cbe: shi.gp	$r2, [ + #-730]
L_16cc2: shi	$r1, [$r0 + #0x10]
L_16cc6: ori	$r7, $r7, #2
L_16cca: lhi.gp	$r1, [ + #-730]
L_16cce: shi	$r1, [$r0 + #0x12]
L_16cd2: sbi	$r7, [$r0 + #0x5a]
L_16cd6: j8 . + (114)
L_16cd8: beqc	$r2, #5, . + (112)
L_16cdc: lbi.gp	$r1, [ + #-728]
L_16ce0: bnec	$r1, #1, . + (18)
L_16ce4: sbi.gp	$r3, [ + #-743]
L_16ce8: sbi.gp	$r3, [ + #-744]
L_16cec: sbi.gp	$r3, [ + #-728]
L_16cf0: j8 . + (42)
L_16cf2: lbi.gp	$r1, [ + #-744]
L_16cf6: slti45 $r1, #2
L_16cf8: beqzs8 . + (10)
L_16cfa: addi45 $r1, #1
L_16cfc: sbi.gp	$r1, [ + #-744]
L_16d00: j8 . + (6)
L_16d02: sbi.gp	$r3, [ + #-744]
L_16d06: lbi.gp	$r1, [ + #-743]
L_16d0a: bnez38 $r1, . + (16)
L_16d0c: lbi.gp	$r1, [ + #-744]
L_16d10: bnec	$r1, #2, . + (10)
L_16d14: movi55 $r1, #1
L_16d16: sbi.gp	$r1, [ + #-743]
L_16d1a: lhsi	$r3, [$r0 + #0x10]
L_16d1e: lhi.gp	$r2, [ + #-732]
L_16d22: lbi.gp	$r1, [ + #-744]
L_16d26: movi	$r4, #0x2d6ec
L_16d2a: sub333 $r2, $r3, $r2
L_16d2c: sh	$r2, [$r4 + ($r1 << #2)]
L_16d30: lhsi	$r2, [$r0 + #0x12]
L_16d34: lhi.gp	$r0, [ + #-730]
L_16d38: add_slli	$r1, $r4, $r1, #2
L_16d3c: sub333 $r0, $r2, $r0
L_16d3e: shi333 $r0, [$r1 + #2]
L_16d40: shi.gp	$r3, [ + #-732]
L_16d44: shi.gp	$r2, [ + #-730]
L_16d48: pop25 $r10, #0
L_16d4a: push25 $r6, #0
L_16d4c: lbi	$r1, [$r0 + #0x43]
L_16d50: sbi	$r1, [$r0 + #0x44]
L_16d54: lbi	$r1, [$r0 + #0x45]
L_16d58: sbi	$r1, [$r0 + #0x43]
L_16d5c: bnez38 $r1, . + (64)
L_16d5e: movi55 $r1, #8
L_16d60: sbi.gp	$r1, [ + #-760]
L_16d64: lhi	$r1, [$r0 + #0x4a]
L_16d68: shi.gp	$r1, [ + #-762]
L_16d6c: lhi.gp	$r1, [ + #-764]
L_16d70: slti	$r15, $r1, #0x2710
L_16d74: beqzs8 . + (8)
L_16d76: addi45 $r1, #1
L_16d78: shi.gp	$r1, [ + #-764]
L_16d7c: lhi333 $r1, [$r0 + #0xc]
L_16d7e: addi	$r15, $r1, #-120
L_16d82: slti	$r15, $r15, #0xb90
L_16d86: beqzs8 . + (14)
L_16d88: lhi333 $r0, [$r0 + #0xe]
L_16d8a: addi	$r15, $r0, #-120
L_16d8e: slti	$r15, $r15, #0x1310
L_16d92: bnezs8 . + (78)
L_16d94: movi55 $r0, #0
L_16d96: shi.gp	$r0, [ + #-764]
L_16d9a: j8 . + (70)
L_16d9c: lbi.gp	$r1, [ + #-760]
L_16da0: beqz38 $r1, . + (54)
L_16da2: lhi.gp	$r2, [ + #-764]
L_16da6: slti	$r15, $r2, #0xfb
L_16daa: bnezs8 . + (44)
L_16dac: lwi.gp	$r2, [ + #-5404]
L_16db0: bnec	$r2, #1, . + (48)
L_16db4: lhi.gp	$r2, [ + #-5518]
L_16db8: bnez38 $r2, . + (8)
L_16dba: lhi.gp	$r2, [ + #-5544]
L_16dbe: beqz38 $r2, . + (34)
L_16dc0: movi55 $r2, #0
L_16dc2: sbi	$r2, [$r0 + #0x43]
L_16dc6: subi45 $r1, #1
L_16dc8: lhi.gp	$r2, [ + #-762]
L_16dcc: shi	$r2, [$r0 + #0x4a]
L_16dd0: sbi.gp	$r1, [ + #-760]
L_16dd4: j8 . + (12)
L_16dd6: movi55 $r1, #0
L_16dd8: shi	$r1, [$r0 + #0x4a]
L_16ddc: shi.gp	$r1, [ + #-764]
L_16de0: ex9.it #55
L_16de2: lwi.gp	$r1, [ + #0x12c8]
L_16de6: bnec	$r0, #1, . + (24)
L_16dea: bnec	$r1, #1, . + (20)
L_16dee: movi55 $r0, #-1
L_16df0: sbi.gp	$r0, [ + #-765]
L_16df4: sbi.gp	$r0, [ + #-766]
L_16df8: sbi.gp	$r0, [ + #-6228]
L_16dfc: j8 . + (10)
L_16dfe: bnec	$r1, #1, . + (182)
L_16e02: bnec	$r0, #1, . + (98)
L_16e06: lhsi.gp	$r0, [ + #0x12d4]
L_16e0a: addi	$r1, $r0, #-300
L_16e0e: zeh33 $r1, $r1
L_16e10: slti	$r15, $r1, #0xa28
L_16e14: beqzs8 . + (16)
L_16e16: lhi.gp	$r1, [ + #0x12d6]
L_16e1a: addi	$r15, $r1, #-300
L_16e1e: slti	$r15, $r15, #0x11a8
L_16e22: bnezs8 . + (218)
L_16e24: movi55 $r2, #0
L_16e26: lhsi.gp	$r1, [ + #0x12d6]
L_16e2a: sltsi	$r15, $r0, #0x12c
L_16e2e: sbi.gp	$r2, [ + #-765]
L_16e32: shi.gp	$r0, [ + #-768]
L_16e36: shi.gp	$r1, [ + #-772]
L_16e3a: beqzs8 . + (8)
L_16e3c: sbi.gp	$r2, [ + #-6228]
L_16e40: j8 . + (188)
L_16e42: sltsi	$r15, $r0, #0xb54
L_16e46: movi55 $r0, #1
L_16e48: beqzs8 . + (10)
L_16e4a: sltsi	$r15, $r1, #0x12c
L_16e4e: beqzs8 . + (10)
L_16e50: movi55 $r0, #2
L_16e52: sbi.gp	$r0, [ + #-6228]
L_16e56: j8 . + (166)
L_16e58: sltsi	$r15, $r1, #0x12d4
L_16e5c: movi55 $r0, #3
L_16e5e: beqzs8 . + (-12)
L_16e60: movi55 $r0, #-1
L_16e62: j8 . + (-16)
L_16e64: lbi.gp	$r0, [ + #-6228]
L_16e68: beqz38 $r0, . + (6)
L_16e6a: bnec	$r0, #1, . + (18)
L_16e6e: lhsi.gp	$r0, [ + #0x12d4]
L_16e72: lbi.gp	$r2, [ + #-765]
L_16e76: movi	$r1, #0x2d6e0
L_16e7a: j8 . + (22)
L_16e7c: beqc	$r0, #2, . + (8)
L_16e80: bnec	$r0, #3, . + (124)
L_16e84: lhsi.gp	$r0, [ + #0x12d6]
L_16e88: lbi.gp	$r2, [ + #-765]
L_16e8c: movi	$r1, #0x2d6dc
L_16e90: lhs	$r15, [$r1 + ($r2 << #1)]
L_16e94: sub45 $r0, $r15
L_16e96: ex9.it #317
L_16e98: zeh33 $r0, $r0
L_16e9a: slti	$r15, $r0, #0x65
L_16e9e: bnezs8 . + (94)
L_16ea0: lbi.gp	$r0, [ + #-765]
L_16ea4: beqc	$r0, #0xff, . + (88)
L_16ea8: sbi.gp	$r0, [ + #-766]
L_16eac: movi55 $r0, #0
L_16eae: sbi.gp	$r0, [ + #-765]
L_16eb2: j8 . + (50)
L_16eb4: bnec	$r1, #2, . + (72)
L_16eb8: bnec	$r0, #1, . + (28)
L_16ebc: movi55 $r0, #0
L_16ebe: sbi.gp	$r0, [ + #-765]
L_16ec2: lhi.gp	$r0, [ + #0x12d4]
L_16ec6: shi.gp	$r0, [ + #-768]
L_16eca: lhi.gp	$r0, [ + #0x12d6]
L_16ece: shi.gp	$r0, [ + #-772]
L_16ed2: j8 . + (42)
L_16ed4: lbi.gp	$r0, [ + #-766]
L_16ed8: beqc	$r0, #0xff, . + (36)
L_16edc: movi55 $r1, #0
L_16ede: sbi.gp	$r1, [ + #-765]
L_16ee2: beqz38 $r0, . + (20)
L_16ee4: lhi.gp	$r0, [ + #-768]
L_16ee8: shi.gp	$r0, [ + #0x12d4]
L_16eec: lhi.gp	$r0, [ + #-772]
L_16ef0: shi.gp	$r0, [ + #0x12d6]
L_16ef4: j8 . + (8)
L_16ef6: movi55 $r0, #-1
L_16ef8: sbi.gp	$r0, [ + #-766]
L_16efc: pop25 $r6, #0
L_16efe: swi.gp	$r0, [ + #-472]
L_16f02: ret5 $lp
L_16f04: swi.gp	$r0, [ + #-476]
L_16f08: ret5 $lp
L_16f0a: push25 $r6, #0
L_16f0c: beqz38 $r0, . + (8)
L_16f0e: beqc	$r0, #1, . + (16)
L_16f12: pop25 $r6, #0
L_16f14: jal	. + (-33394)
L_16f18: swi.gp	$r0, [ + #-468]
L_16f1c: pop25 $r6, #0
L_16f1e: jal	. + (-33386)
L_16f22: swi.gp	$r0, [ + #-464]
L_16f26: pop25 $r6, #0
# Selector 0..8: unsigned8 dispatch to pointer/stride alternatives.
L_16f28: mov55 $r1, $r0
L_16f2a: slti45 $r0, #9
L_16f2c: lwi.gp	$r0, [ + #-464]
L_16f30: beqzs8 . + (70)
L_16f32: movi	$r15, #0x16d3c
L_16f36: ex9.it #467
L_16f38: ex9.it #158
L_16f3a: jr5 $r15
# Tests unsigned selector r0<9, saves selector to r1, loads default pointer from GP-464; loads u8 offset through EX9 slot467, adds table base through slot158, jumps to r15. Most cases return base plus fixed stride offsets; index1 returns unchanged pointer.
L_16f3c:
    .byte L_16f46 - L_16f3c
    .byte L_16f78 - L_16f3c
    .byte L_16f4c - L_16f3c
    .byte L_16f52 - L_16f3c
    .byte L_16f58 - L_16f3c
    .byte L_16f5e - L_16f3c
    .byte L_16f64 - L_16f3c
    .byte L_16f6a - L_16f3c
    .byte L_16f70 - L_16f3c
# Declared single alignment byte after switch offsets
    .balign 2, 0
L_16f46: lwi.gp	$r0, [ + #-468]
L_16f4a: ret5 $lp
L_16f4c: addi	$r0, $r0, #0xf0
L_16f50: ret5 $lp
L_16f52: addi	$r0, $r0, #0x1e0
L_16f56: ret5 $lp
L_16f58: addi	$r0, $r0, #0x2d0
L_16f5c: ret5 $lp
L_16f5e: addi	$r0, $r0, #0x3c0
L_16f62: ret5 $lp
L_16f64: addi	$r0, $r0, #0x5a0
L_16f68: ret5 $lp
L_16f6a: addi	$r0, $r0, #0x780
L_16f6e: ret5 $lp
L_16f70: addi	$r0, $r0, #0x960
L_16f74: ret5 $lp
L_16f76: movi55 $r0, #0
L_16f78: ret5 $lp
L_16f7a: nop16
L_16f7c: push25 $r10, #0
L_16f7e: mov55 $r6, $r2
L_16f80: mov55 $r7, $r1
L_16f82: lwi.gp	$r2, [ + #-464]
L_16f86: lwi.gp	$r1, [ + #-468]
L_16f8a: mov55 $r9, $r3
L_16f8c: mov55 $r10, $r4
L_16f8e: jal	. + (-33598)
L_16f92: lwi.gp	$r1, [ + #-476]
L_16f96: lbi333 $r0, [$r1 + #2]
L_16f98: lbi333 $r1, [$r1 + #3]
L_16f9a: jal	. + (-33546)
L_16f9e: ex9.it #119
L_16fa0: min	$r0, $r6, $r0
L_16fa4: jal	. + (-33494)
L_16fa8: movi55 $r6, #0
L_16faa: lwi.gp	$r1, [ + #-472]
L_16fae: lbi	$r0, [$r1 + #0x10]
L_16fb2: slt45 $r6, $r0
L_16fb4: beqzs8 . + (32)
L_16fb6: mov55 $r0, $r6
L_16fb8: lb	$r1, [$r1 + ($r6 << #0)]
L_16fbc: jal	. + (-33492)
L_16fc0: lwi.gp	$r0, [ + #-472]
L_16fc4: add333 $r1, $r0, $r6
L_16fc6: lbi	$r1, [$r1 + #8]
L_16fca: mov55 $r0, $r6
L_16fcc: jal	. + (-33432)
L_16fd0: addi45 $r6, #1
L_16fd2: j8 . + (-40)
L_16fd4: movi55 $r0, #0
L_16fd6: ifcall9 . + (22)
L_16fd8: ifcall9 . + (24)
L_16fda: movi55 $r0, #1
L_16fdc: ifcall9 . + (16)
L_16fde: ifcall9 . + (18)
L_16fe0: bnec	$r10, #1, . + (10)
L_16fe4: movi55 $r0, #2
L_16fe6: ifcall9 . + (6)
L_16fe8: ifcall9 . + (8)
L_16fea: movi55 $r0, #3
L_16fec: jal	. + (-33558)
L_16ff0: jal	. + (-33660)
L_16ff4: jal	. + (-33582)
L_16ff8: ex9.it #138
L_16ffa: movi55 $r4, #1
L_16ffc: movi55 $r0, #0
L_16ffe: lbi	$r1, [$r9 + #0]
L_17002: slt45 $r0, $r1
L_17004: beqzs8 . + (86)
L_17006: lwi333 $r1, [$r7 + #4]
L_17008: lh	$r5, [$r1 + ($r0 << #1)]
L_1700c: beqs38 $r0, . + (74)
L_1700e: lwi333 $r1, [$r7 + #0x14]
L_17010: slli333 $r2, $r0, #2
L_17012: slli333 $r5, $r5, #2
L_17014: add333 $r3, $r1, $r2
L_17016: add45 $r1, $r5
L_17018: lbi333 $r3, [$r3 + #2]
L_1701a: lbi333 $r6, [$r1 + #2]
L_1701c: slt45 $r3, $r6
L_1701e: beqzs8 . + (4)
L_17020: sbi333 $r3, [$r1 + #2]
L_17022: lwi333 $r1, [$r7 + #0x14]
L_17024: add333 $r3, $r1, $r2
L_17026: add45 $r1, $r5
L_17028: lbi333 $r3, [$r3 + #3]
L_1702a: lbi333 $r6, [$r1 + #3]
L_1702c: slt45 $r6, $r3
L_1702e: beqzs8 . + (4)
L_17030: sbi333 $r3, [$r1 + #3]
L_17032: lwi333 $r1, [$r7 + #0x14]
L_17034: lb	$r3, [$r1 + ($r2 << #0)]
L_17038: add45 $r1, $r5
L_1703a: lbi333 $r6, [$r1 + #0]
L_1703c: slt45 $r3, $r6
L_1703e: beqzs8 . + (4)
L_17040: sbi333 $r3, [$r1 + #0]
L_17042: lwi333 $r3, [$r7 + #0x14]
L_17044: add45 $r2, $r3
L_17046: add45 $r5, $r3
L_17048: lbi333 $r1, [$r2 + #1]
L_1704a: lbi333 $r2, [$r5 + #1]
L_1704c: slt45 $r2, $r1
L_1704e: beqzs8 . + (4)
L_17050: sbi333 $r1, [$r5 + #1]
L_17052: sbi.gp	$r4, [ + #-4822]
L_17056: addi45 $r0, #1
L_17058: j8 . + (-90)
L_1705a: pop25 $r10, #0
L_1705c: push25 $r14, #16
L_1705e: sethi	$r4, #0xc0120
L_17062: add45 $r4, $r1
L_17064: movi	$r6, #0x308ff
L_17068: slt45 $r6, $r4
L_1706a: mov55 $r2, $r0
L_1706c: mov55 $r5, $r1
L_1706e: srli	$r3, $r1, #0x10
L_17072: bnez	$r15, . + (260)
L_17076: bnez38 $r0, . + (10)
L_17078: sethi	$r0, #0x3ff00
L_1707c: beq	$r5, $r0, . + (578)
L_17080: mov55 $r4, $r2
L_17082: mov55 $r5, $r1
L_17084: movd44 $r0, $r4
L_17086: movi55 $r2, #0
L_17088: sethi	$r3, #0xbff00
L_1708c: ex9.it #39
L_1708e: movd44 $r2, $r0
L_17090: movd44 $r6, $r0
L_17092: ex9.it #38
L_17094: movd44 $r12, $r0
L_17096: movd44 $r2, $r0
L_17098: ifcall9 . + (476)
L_1709a: movi	$r2, #0x19a38
L_1709e: movd44 $r10, $r0
L_170a0: ex9.it #85
L_170a2: ifcall9 . + (466)
L_170a4: movi	$r2, #0x19a30
L_170a8: ifcall9 . + (490)
L_170aa: movi	$r2, #0x19a40
L_170ae: movd44 $r8, $r0
L_170b0: ex9.it #85
L_170b2: movd44 $r0, $r12
L_170b4: ex9.it #38
L_170b6: ifcall9 . + (424)
L_170b8: movi	$r2, #0x19a50
L_170bc: ex9.it #85
L_170be: movd44 $r8, $r0
L_170c0: ifcall9 . + (436)
L_170c2: movi	$r2, #0x19a48
L_170c6: ifcall9 . + (460)
L_170c8: movi	$r2, #0x19a58
L_170cc: ifcall9 . + (438)
L_170ce: movd44 $r0, $r12
L_170d0: ex9.it #38
L_170d2: lmw.bi $r4, [$sp], $r5, #0x0
L_170d6: movd44 $r2, $r0
L_170d8: movd44 $r0, $r4
L_170da: ex9.it #39
L_170dc: movi	$r2, #0x19a68
L_170e0: ifcall9 . + (398)
L_170e2: movi	$r2, #0x19a60
L_170e6: ifcall9 . + (428)
L_170e8: movi	$r2, #0x19a70
L_170ec: ifcall9 . + (406)
L_170ee: movd44 $r0, $r12
L_170f0: ex9.it #38
L_170f2: ifcall9 . + (426)
L_170f4: movi	$r2, #0x19a78
L_170f8: movd44 $r12, $r0
L_170fa: ex9.it #85
L_170fc: ifcall9 . + (412)
L_170fe: movd44 $r2, $r0
L_17100: movd44 $r0, $r12
L_17102: ex9.it #39
L_17104: ifcall9 . + (402)
L_17106: ifcall9 . + (416)
L_17108: ex9.it #39
L_1710a: ifcall9 . + (396)
L_1710c: ifcall9 . + (338)
L_1710e: ifcall9 . + (392)
L_17110: swi37.sp $r0, [ + #8]
L_17112: swi37.sp $r1, [ + #0xc]
L_17114: movi55 $r2, #0
L_17116: sethi	$r3, #0x41a00
L_1711a: ifcall9 . + (346)
L_1711c: movd44 $r10, $r0
L_1711e: ifcall9 . + (22)
L_17120: movd44 $r2, $r10
L_17122: ex9.it #105
L_17124: movd44 $r2, $r0
L_17126: movd44 $r10, $r0
L_17128: ex9.it #38
L_1712a: movi	$r2, #0x19a28
L_1712e: ex9.it #85
L_17130: ex9.it #38
L_17132: movd44 $r8, $r0
L_17134: movd44 $r2, $r0
L_17136: movd44 $r0, $r6
L_17138: ex9.it #39
L_1713a: movd44 $r12, $r0
L_1713c: movd44 $r2, $r0
L_1713e: movd44 $r0, $r6
L_17140: ex9.it #105
L_17142: movd44 $r2, $r8
L_17144: ex9.it #39
L_17146: movd44 $r8, $r0
L_17148: movd44 $r2, $r10
L_1714a: movd44 $r0, $r6
L_1714c: ex9.it #105
L_1714e: movd44 $r2, $r0
L_17150: movi	$r0, #0x19a28
L_17154: lmw.bi $r0, [$r0], $r1, #0x0
L_17158: ex9.it #38
L_1715a: smw.bi $r0, [$sp], $r1, #0x0
L_1715e: movd44 $r2, $r6
L_17160: movd44 $r0, $r10
L_17162: ex9.it #39
L_17164: lmw.bi $r16, [$sp], $r17, #0x0
L_17168: movd44 $r2, $r0
L_1716a: movd44 $r0, $r16
L_1716c: ex9.it #38
L_1716e: ifcall9 . + (240)
L_17170: ifcall9 . + (310)
L_17172: j	. + (324)
L_17176: addi	$r15, $r3, #-16
L_1717a: movi	$r6, #0x7fdf
L_1717e: slt45 $r6, $r15
L_17180: beqzs8 . + (80)
L_17182: slli333 $r4, $r1, #1
L_17184: or_srli	$r4, $r4, $r0, #0x1f
L_17188: or_slli	$r4, $r4, $r0, #1
L_1718c: bnez38 $r4, . + (10)
L_1718e: movi55 $r0, #1
L_17190: jal	. + (332)
L_17194: ifcall9 . + (38)
L_17196: bnez38 $r0, . + (10)
L_17198: sethi	$r0, #0x7ff00
L_1719c: beq	$r5, $r0, . + (296)
L_171a0: srli	$r0, $r3, #0xf
L_171a4: mov55 $r4, $r2
L_171a6: bnez38 $r0, . + (12)
L_171a8: andi	$r5, $r3, #0x7ff0
L_171ac: movi	$r0, #0x7ff0
L_171b0: bnes38 $r0, . + (14)
L_171b2: mov55 $r5, $r1
L_171b4: movd44 $r0, $r4
L_171b6: jal	. + (336)
L_171ba: j	. + (268)
L_171be: mov55 $r5, $r1
L_171c0: movd44 $r0, $r4
L_171c2: movi55 $r2, #0
L_171c4: sethi	$r3, #0x43300
L_171c8: ex9.it #38
L_171ca: sethi	$r5, #0xfcc00
L_171ce: add45 $r5, $r1
L_171d0: sethi	$r9, #0xc01a0
L_171d4: add45 $r9, $r5
L_171d6: srli	$r6, $r9, #0xd
L_171da: fexti33 $r6, #6
L_171dc: srli	$r1, $r9, #0x14
L_171e0: movi	$r7, #0x199f0
L_171e4: add_slli	$r4, $r7, $r6, #4
L_171e8: sub_slli	$r2, $r5, $r1, #0x14
L_171ec: lwi	$r3, [$r4 + #0x894]
L_171f0: add_slli	$r10, $r7, $r6, #4
L_171f4: mov55 $r1, $r2
L_171f6: lwi	$r2, [$r4 + #0x890]
L_171fa: ex9.it #105
L_171fc: lwi	$r2, [$r10 + #0x898]
L_17200: lwi	$r3, [$r10 + #0x89c]
L_17204: ex9.it #105
L_17206: lwi	$r2, [$r10 + #0x90]
L_1720a: lwi	$r3, [$r10 + #0x94]
L_1720e: ex9.it #38
L_17210: movd44 $r6, $r0
L_17212: srai	$r0, $r9, #0x14
L_17216: ex9.it #262
L_17218: movi	$r2, #0x199f0
L_1721c: ifcall9 . + (102)
L_1721e: ex9.it #38
L_17220: lwi	$r2, [$r10 + #0x98]
L_17224: lwi	$r3, [$r10 + #0x9c]
L_17228: ex9.it #39
L_1722a: movd44 $r2, $r6
L_1722c: movd44 $r8, $r0
L_1722e: ex9.it #39
L_17230: movd44 $r12, $r0
L_17232: movd44 $r2, $r6
L_17234: ifcall9 . + (64)
L_17236: movd44 $r10, $r0
L_17238: movd44 $r2, $r12
L_1723a: movd44 $r0, $r8
L_1723c: ex9.it #105
L_1723e: movd44 $r2, $r6
L_17240: ex9.it #39
L_17242: lmw.bi $r4, [$sp], $r5, #0x0
L_17246: movi	$r2, #0x199f8
L_1724a: movd44 $r8, $r0
L_1724c: ex9.it #85
L_1724e: movd44 $r0, $r4
L_17250: ex9.it #38
L_17252: ifcall9 . + (12)
L_17254: movi	$r2, #0x19a00
L_17258: movd44 $r8, $r0
L_1725a: ex9.it #85
L_1725c: ifcall9 . + (60)
L_1725e: movd44 $r2, $r0
L_17260: movd44 $r0, $r8
L_17262: ex9.it #39
L_17264: movd44 $r8, $r0
L_17266: movd44 $r2, $r10
L_17268: ifcall9 . + (12)
L_1726a: movi	$r2, #0x19a10
L_1726e: ex9.it #85
L_17270: swi37.sp $r0, [ + #8]
L_17272: swi37.sp $r1, [ + #0xc]
L_17274: movd44 $r0, $r6
L_17276: ex9.it #38
L_17278: movi	$r2, #0x19a08
L_1727c: ifcall9 . + (22)
L_1727e: movi	$r2, #0x19a20
L_17282: ex9.it #85
L_17284: smw.bi $r0, [$sp], $r1, #0x0
L_17288: ifret16
L_1728a: ifcall	. + (-22)
L_1728e: movi	$r2, #0x19a18
L_17292: ex9.it #85
L_17294: ex9.it #39
L_17296: movd44 $r2, $r0
L_17298: movd44 $r0, $r10
L_1729a: ex9.it #38
L_1729c: lmw.bi $r16, [$sp], $r17, #0x0
L_172a0: movd44 $r2, $r0
L_172a2: movd44 $r0, $r16
L_172a4: ex9.it #39
L_172a6: lwi37.sp $r4, [ + #8]
L_172a8: lwi37.sp $r5, [ + #0xc]
L_172aa: movd44 $r2, $r0
L_172ac: movd44 $r0, $r4
L_172ae: ifret16
L_172b0: ex9.it #38
L_172b2: movd44 $r2, $r0
L_172b4: movd44 $r0, $r8
L_172b6: ex9.it #39
L_172b8: movd44 $r2, $r12
L_172ba: ex9.it #39
L_172bc: j8 . + (10)
L_172be: movi55 $r0, #0
L_172c0: mov55 $r1, $r0
L_172c2: j8 . + (4)
L_172c4: mov55 $r0, $r2
L_172c6: movd44 $r2, $r0
L_172c8: movd44 $r0, $r2
L_172ca: pop25 $r14, #16
L_172cc: push25 $r10, #0
L_172ce: movd44 $r6, $r0
L_172d0: mov55 $r9, $r2
L_172d2: jal	. + (2570)
L_172d6: swi450 $r9, [$r0]
L_172d8: movd44 $r0, $r6
L_172da: pop25 $r10, #0
L_172dc: push25 $r6, #8
L_172de: bnez38 $r0, . + (10)
L_172e0: movi55 $r0, #0
L_172e2: sethi	$r1, #0x3ff00
L_172e6: j8 . + (8)
L_172e8: movi55 $r0, #0
L_172ea: sethi	$r1, #0xbff00
L_172ee: swi450 $r0, [$sp]
L_172f0: movi55 $r2, #0
L_172f2: swi37.sp $r1, [ + #4]
L_172f4: lwi450 $r0, [$sp]
L_172f6: mov55 $r3, $r2
L_172f8: lwi37.sp $r1, [ + #4]
L_172fa: jal	. + (1602)
L_172fe: movpi45 $r2, #0x22
L_17300: jal	. + (-52)
L_17304: pop25 $r6, #8
L_17306: push25 $r14, #0
L_17308: movd44 $r2, $r0
L_1730a: movd44 $r10, $r0
L_1730c: ex9.it #105
L_1730e: movd44 $r2, $r0
L_17310: jal	. + (1580)
L_17314: movd44 $r6, $r0
L_17316: movd44 $r0, $r10
L_17318: jal	. + (20)
L_1731c: beqz38 $r0, . + (12)
L_1731e: movd44 $r0, $r6
L_17320: movpi45 $r2, #0x21
L_17322: jal	. + (-86)
L_17326: pop25 $r14, #0
L_17328: movd44 $r0, $r6
L_1732a: pop25 $r14, #0
L_1732c: beqz38 $r1, . + (8)
L_1732e: sethi	$r5, #0x80000
L_17332: bnes38 $r1, . + (8)
L_17334: bnez38 $r0, . + (6)
L_17336: movi55 $r0, #2
L_17338: ret5 $lp
L_1733a: sethi	$r5, #0xfff00
L_1733e: sethi	$r4, #0x7fe00
L_17342: subi333 $r4, $r4, #1
L_17344: add333 $r3, $r1, $r5
L_17346: slt45 $r4, $r3
L_17348: beqzs8 . + (12)
L_1734a: sethi	$r5, #0x7ff00
L_1734e: add333 $r3, $r1, $r5
L_17350: slt45 $r4, $r3
L_17352: bnezs8 . + (6)
L_17354: movi55 $r0, #4
L_17356: ret5 $lp
L_17358: sethi	$r4, #0x100
L_1735c: subi333 $r4, $r4, #1
L_1735e: slt45 $r4, $r1
L_17360: beqzs8 . + (12)
L_17362: sethi	$r5, #0x80000
L_17366: add333 $r3, $r1, $r5
L_17368: slt45 $r4, $r3
L_1736a: bnezs8 . + (6)
L_1736c: movi55 $r0, #3
L_1736e: ret5 $lp
L_17370: sethi	$r5, #0x7ff00
L_17374: beqs38 $r1, . + (8)
L_17376: sethi	$r5, #0xfff00
L_1737a: bnes38 $r1, . + (4)
L_1737c: beqz38 $r0, . + (6)
L_1737e: movi55 $r0, #0
L_17380: ret5 $lp
L_17382: movi55 $r0, #1
L_17384: ret5 $lp
L_17386: nop16
L_17388: smw.adm $r6, [$sp], $r6, #0x2
L_1738c: xor	$r6, $r1, $r3
L_17390: srai45 $r6, #0x1f
L_17392: bgez	$r3, . + (12)
L_17396: neg33 $r3, $r3
L_17398: beqz38 $r2, . + (6)
L_1739a: neg33 $r2, $r2
L_1739c: subi333 $r3, $r3, #1
L_1739e: bgez	$r1, . + (12)
L_173a2: neg33 $r1, $r1
L_173a4: beqz38 $r0, . + (6)
L_173a6: neg33 $r0, $r0
L_173a8: subi333 $r1, $r1, #1
L_173aa: movi55 $r4, #0
L_173ac: jal	. + (120)
L_173b0: beqz38 $r6, . + (16)
L_173b2: or	$r4, $r1, $r0
L_173b6: beqz38 $r4, . + (10)
L_173b8: neg33 $r1, $r1
L_173ba: beqz38 $r0, . + (6)
L_173bc: neg33 $r0, $r0
L_173be: subi333 $r1, $r1, #1
L_173c0: lmw.bim $r6, [$sp], $r6, #0x2
L_173c4: ret5 $lp
L_173c6: nop16
L_173c8: movi55 $r4, #0
L_173ca: j8 . + (90)
L_173cc: srli	$r20, $r2, #0x10
L_173d0: zeh33 $r5, $r0
L_173d2: srli45 $r0, #0x10
L_173d4: divr	$r4, $r1, $r1, $r20
L_173d8: zeh33 $r3, $r2
L_173da: slli	$r1, $r1, #0x10
L_173de: or33 $r1, $r0
L_173e0: mul	$r0, $r4, $r3
L_173e4: slt45 $r1, $r0
L_173e6: beqzs8 . + (18)
L_173e8: subi333 $r4, $r4, #1
L_173ea: add333 $r1, $r1, $r2
L_173ec: slt45 $r1, $r2
L_173ee: bnezs8 . + (10)
L_173f0: slt45 $r1, $r0
L_173f2: beqzs8 . + (6)
L_173f4: subi333 $r4, $r4, #1
L_173f6: add333 $r1, $r1, $r2
L_173f8: sub333 $r1, $r1, $r0
L_173fa: divr	$r1, $r0, $r1, $r20
L_173fe: slli	$r0, $r0, #0x10
L_17402: or33 $r0, $r5
L_17404: mul33 $r3, $r1
L_17406: slt45 $r0, $r3
L_17408: beqzs8 . + (18)
L_1740a: add333 $r0, $r0, $r2
L_1740c: subi333 $r1, $r1, #1
L_1740e: slt45 $r0, $r2
L_17410: bnezs8 . + (10)
L_17412: slt45 $r0, $r3
L_17414: beqzs8 . + (6)
L_17416: add333 $r0, $r0, $r2
L_17418: subi333 $r1, $r1, #1
L_1741a: sub333 $r0, $r0, $r3
L_1741c: slli	$r4, $r4, #0x10
L_17420: or33 $r1, $r4
L_17422: ret5 $lp
L_17424: smw.adm $sp, [$sp], $sp, #0xa
L_17428: movd44 $r16, $r0
L_1742a: movd44 $r18, $r2
L_1742c: mov55 $fp, $r4
L_1742e: bnez38 $r3, . + (154)
L_17430: slt45 $r17, $r18
L_17432: beqzs8 . + (50)
L_17434: clz	$r0, $r18
L_17438: mov55 $r21, $r0
L_1743a: beqz38 $r0, . + (26)
L_1743c: sll	$r18, $r18, $r0
L_17440: subri	$r5, $r0, #0x20
L_17444: srl	$r5, $r16, $r5
L_17448: sll	$r17, $r17, $r0
L_1744c: or	$r17, $r17, $r5
L_17450: sll	$r16, $r16, $r0
L_17454: movd44 $r0, $r16
L_17456: mov55 $r2, $r18
L_17458: jal	. + (-140)
L_1745c: mov55 $r22, $r1
L_1745e: mov55 $r16, $r0
L_17460: movi55 $r23, #0
L_17462: j8 . + (70)
L_17464: beqz38 $r2, . + (80)
L_17466: clz	$r0, $r18
L_1746a: mov55 $r21, $r0
L_1746c: bnez38 $r0, . + (8)
L_1746e: sub45 $r17, $r18
L_17470: movi55 $r23, #1
L_17472: j8 . + (40)
L_17474: subri	$r15, $r0, #0x20
L_17478: sll	$r18, $r18, $r0
L_1747c: mov55 $r2, $r18
L_1747e: srl	$r4, $r16, $r15
L_17482: sll	$r5, $r17, $r0
L_17486: sll	$r16, $r16, $r0
L_1748a: or	$r0, $r5, $r4
L_1748e: srl	$r1, $r17, $r15
L_17492: jal	. + (-198)
L_17496: mov55 $r23, $r1
L_17498: mov55 $r17, $r0
L_1749a: movd44 $r0, $r16
L_1749c: mov55 $r2, $r18
L_1749e: jal	. + (-210)
L_174a2: mov55 $r22, $r1
L_174a4: addi	$r16, $r0, #0
L_174a8: beqz	$fp, . + (166)
L_174ac: movi55 $r17, #0
L_174ae: srl	$r16, $r16, $r21
L_174b2: j8 . + (150)
L_174b4: divr	$r17, $r16, $r18, $r18
L_174b8: movi55 $r1, #0
L_174ba: movi55 $r0, #0
L_174bc: beqz	$fp, . + (152)
L_174c0: swi450 $r16, [$fp]
L_174c2: swi	$r17, [$fp + #4]
L_174c6: j8 . + (142)
L_174c8: slt45 $r17, $r19
L_174ca: bnezs8 . + (-18)
L_174cc: clz	$r0, $r19
L_174d0: mov55 $r21, $r0
L_174d2: beqz38 $r0, . + (136)
L_174d4: subri	$r4, $r0, #0x20
L_174d8: srl	$r5, $r18, $r4
L_174dc: sll	$r2, $r19, $r0
L_174e0: or33 $r2, $r5
L_174e2: mov55 $r19, $r2
L_174e4: sll	$r18, $r18, $r0
L_174e8: srl	$r3, $r16, $r4
L_174ec: sll	$r16, $r16, $r0
L_174f0: sll	$r0, $r17, $r0
L_174f4: srl	$r1, $r17, $r4
L_174f8: or33 $r0, $r3
L_174fa: jal	. + (-302)
L_174fe: mov55 $r22, $r1
L_17500: mov55 $r17, $r0
L_17502: mulr64	$r0, $r1, $r18
L_17506: slt45 $r17, $r1
L_17508: bnezs8 . + (10)
L_1750a: bne	$r1, $r17, . + (24)
L_1750e: slt45 $r16, $r0
L_17510: beqzs8 . + (18)
L_17512: addi	$r22, $r22, #-1
L_17516: sub45 $r1, $r19
L_17518: sub	$r3, $r0, $r18
L_1751c: slt45 $r0, $r3
L_1751e: sub45 $r1, $r15
L_17520: mov55 $r0, $r3
L_17522: movi55 $r23, #0
L_17524: beqz	$fp, . + (42)
L_17528: sub	$r0, $r16, $r0
L_1752c: sub	$r1, $r17, $r1
L_17530: slt45 $r16, $r0
L_17532: sub45 $r1, $r15
L_17534: subri	$r4, $r21, #0x20
L_17538: sll	$r17, $r1, $r4
L_1753c: srl	$r16, $r0, $r21
L_17540: or	$r16, $r16, $r17
L_17544: srl	$r17, $r1, $r21
L_17548: swi450 $r16, [$fp]
L_1754a: swi	$r17, [$fp + #4]
L_1754e: mov55 $r0, $r22
L_17550: addi	$r1, $r23, #0
L_17554: lmw.bim $sp, [$sp], $sp, #0xa
L_17558: ret5 $lp
L_1755a: slt45 $r19, $r17
L_1755c: bnezs8 . + (6)
L_1755e: slt45 $r16, $r18
L_17560: bnezs8 . + (18)
L_17562: movi55 $r22, #1
L_17564: sub	$r4, $r16, $r18
L_17568: sub45 $r17, $r19
L_1756a: slt45 $r16, $r4
L_1756c: sub45 $r17, $r15
L_1756e: mov55 $r16, $r4
L_17570: j8 . + (4)
L_17572: movi55 $r22, #0
L_17574: movi55 $r23, #0
L_17576: bnez	$fp, . + (-46)
L_1757a: j8 . + (-44)
L_1757c: btgl	$r3, $r3, #0x1f
L_17580: slli333 $r4, $r1, #1
L_17582: smw.adm $r6, [$sp], $r10, #0x2
L_17586: slli333 $r6, $r3, #1
L_17588: sethi	$lp, #0x80000
L_1758c: slt45 $r4, $r6
L_1758e: bnezs8 . + (10)
L_17590: bne	$r4, $r6, . + (18)
L_17594: slt45 $r0, $r2
L_17596: beqzs8 . + (12)
L_17598: movd44 $r8, $r0
L_1759a: movd44 $r0, $r2
L_1759c: movd44 $r2, $r8
L_1759e: slli333 $r4, $r1, #1
L_175a0: slli333 $r6, $r3, #1
L_175a2: xor33 $r3, $r1
L_175a4: and	$r3, $r3, $lp
L_175a8: srli	$r7, $r4, #0x15
L_175ac: srli	$r10, $r6, #0x15
L_175b0: slli	$r9, $r4, #0xa
L_175b4: slli	$r8, $r6, #0xa
L_175b8: movi	$r5, #0x7ff
L_175bc: beq	$r5, $r7, . + (392)
L_175c0: or	$r15, $r4, $r0
L_175c4: beqz	$r15, . + (376)
L_175c8: or	$r15, $r6, $r2
L_175cc: beqz	$r15, . + (292)
L_175d0: sub	$r6, $r7, $r10
L_175d4: slti	$r15, $r6, #0x40
L_175d8: beqz	$r15, . + (280)
L_175dc: srli	$r5, $r0, #0x15
L_175e0: or	$r9, $r9, $r5
L_175e4: slli	$r0, $r0, #0xb
L_175e8: srli	$r5, $r2, #0x15
L_175ec: or	$r8, $r8, $r5
L_175f0: slli	$r2, $r2, #0xb
L_175f4: slti45 $r7, #2
L_175f6: bnezs8 . + (78)
L_175f8: or	$r9, $r9, $lp
L_175fc: beqz	$r10, . + (8)
L_17600: or	$r8, $r8, $lp
L_17604: subi333 $r5, $r6, #1
L_17606: cmovz	$r6, $r5, $r10
L_1760a: beqz38 $r6, . + (58)
L_1760c: subri	$r5, $r6, #0x20
L_17610: blez	$r5, . + (24)
L_17614: sll	$r4, $r2, $r5
L_17618: sll	$r5, $r8, $r5
L_1761c: srl	$r8, $r8, $r6
L_17620: srl	$r2, $r2, $r6
L_17624: or33 $r2, $r5
L_17626: j8 . + (24)
L_17628: neg33 $r6, $r5
L_1762a: addi	$r5, $r5, #0x20
L_1762e: sll	$r4, $r8, $r5
L_17632: or33 $r4, $r2
L_17634: cmovz	$r4, $r6, $r6
L_17638: srl	$r2, $r8, $r6
L_1763c: movi55 $r8, #0
L_1763e: beqz38 $r4, . + (6)
L_17640: ori	$r2, $r2, #2
L_17644: beqz38 $r3, . + (178)
L_17646: bne	$r7, $r10, . + (12)
L_1764a: bne	$r9, $r8, . + (8)
L_1764e: beq	$r0, $r2, . + (240)
L_17652: slt45 $r9, $r8
L_17654: bnezs8 . + (10)
L_17656: bne	$r9, $r8, . + (20)
L_1765a: slt45 $r0, $r2
L_1765c: beqzs8 . + (14)
L_1765e: subi333 $r7, $r7, #1
L_17660: slli	$r4, $r8, #0x1f
L_17664: srli45 $r8, #1
L_17666: srli45 $r2, #1
L_17668: or33 $r2, $r4
L_1766a: mov55 $r4, $r0
L_1766c: sub333 $r0, $r0, $r2
L_1766e: slt45 $r4, $r0
L_17670: sub45 $r9, $r15
L_17672: sub45 $r9, $r8
L_17674: slti45 $r7, #2
L_17676: bnezs8 . + (146)
L_17678: bnez	$r9, . + (22)
L_1767c: movpi45 $r4, #0x20
L_1767e: slt	$r4, $r7, $r4
L_17682: mov55 $r5, $r7
L_17684: bnez38 $r4, . + (26)
L_17686: mov55 $r9, $r0
L_17688: movi55 $r0, #0
L_1768a: addi	$r7, $r7, #-32
L_1768e: clz	$r5, $r9
L_17692: slt45 $r5, $r7
L_17694: subri	$r4, $r15, #1
L_17698: min	$r5, $r5, $r7
L_1769c: beqz38 $r5, . + (26)
L_1769e: sub333 $r7, $r7, $r5
L_176a0: sub333 $r5, $r5, $r4
L_176a2: subri	$r6, $r5, #0x20
L_176a6: srl	$r4, $r0, $r6
L_176aa: sll	$r9, $r9, $r5
L_176ae: sll	$r0, $r0, $r5
L_176b2: or	$r9, $r9, $r4
L_176b6: addi	$r0, $r0, #0x400
L_176ba: slti	$r15, $r0, #0x400
L_176be: add45 $r9, $r15
L_176c0: slt45 $r9, $r15
L_176c2: add45 $r7, $r15
L_176c4: srli	$r5, $r0, #0xb
L_176c8: xlsb33 $r5, $r5
L_176ca: mov55 $r6, $r0
L_176cc: sub333 $r0, $r0, $r5
L_176ce: slt45 $r6, $r0
L_176d0: sub45 $r9, $r15
L_176d2: slli	$r9, $r9, #1
L_176d6: slli	$r5, $r9, #0x14
L_176da: srli45 $r9, #0xc
L_176dc: srli45 $r0, #0xb
L_176de: or33 $r0, $r5
L_176e0: slli	$r5, $r7, #0x14
L_176e4: or	$r9, $r9, $r5
L_176e8: and	$r1, $r1, $lp
L_176ec: or	$r1, $r1, $r9
L_176f0: lmw.bim $r6, [$sp], $r10, #0x2
L_176f4: ret5 $lp
L_176f6: add333 $r0, $r0, $r2
L_176f8: slt45 $r0, $r2
L_176fa: add45 $r9, $r15
L_176fc: slt45 $r9, $r15
L_176fe: add45 $r9, $r8
L_17700: bnezs8 . + (20)
L_17702: slt45 $r9, $r8
L_17704: bnezs8 . + (16)
L_17706: bnez38 $r7, . + (-80)
L_17708: srli45 $r0, #0xb
L_1770a: slli	$r4, $r9, #0x15
L_1770e: srli45 $r9, #0xb
L_17710: or33 $r0, $r4
L_17712: j8 . + (-42)
L_17714: subri	$r5, $r7, #0x7fe
L_17718: beqz38 $r5, . + (28)
L_1771a: andi	$r15, $r0, #1
L_1771e: ori	$r5, $r0, #2
L_17722: cmovn	$r0, $r5, $r15
L_17726: slli	$r4, $r9, #0x1f
L_1772a: srli45 $r9, #1
L_1772c: srli45 $r0, #1
L_1772e: or33 $r0, $r4
L_17730: addi333 $r7, $r7, #1
L_17732: j8 . + (-124)
L_17734: movi55 $r0, #0
L_17736: sethi	$r9, #0x7ff00
L_1773a: j8 . + (-82)
L_1773c: beqz38 $r3, . + (-76)
L_1773e: movi55 $r0, #0
L_17740: movi55 $r1, #0
L_17742: j8 . + (-82)
L_17744: or	$r9, $r9, $r0
L_17748: bne	$r9, $lp, . + (10)
L_1774c: bne	$r5, $r10, . + (-92)
L_17750: beqz38 $r3, . + (-96)
L_17752: movi55 $r0, #0
L_17754: movi	$r1, #-524288
L_17758: j8 . + (-104)
L_1775a: nop16
L_1775c: push25 $r10, #16
L_1775e: slli333 $r4, $r1, #1
L_17760: srli45 $r4, #0x15
L_17762: slli	$r7, $r1, #0xb
L_17766: srli	$r6, $r0, #0x15
L_1776a: or33 $r7, $r6
L_1776c: slli	$r6, $r0, #0xb
L_17770: sethi	$fp, #0x80000
L_17774: slli333 $r5, $r3, #1
L_17776: srli45 $r5, #0x15
L_17778: slli	$r9, $r3, #0xb
L_1777c: srli	$r8, $r2, #0x15
L_17780: or	$r9, $r9, $r8
L_17784: slli	$r8, $r2, #0xb
L_17788: xor33 $r3, $r1
L_1778a: and	$r10, $r3, $fp
L_1778e: movi	$r3, #0x7ff
L_17792: beqz38 $r4, . + (176)
L_17794: beq	$r3, $r4, . + (234)
L_17798: or	$r7, $r7, $fp
L_1779c: beqz	$r5, . + (270)
L_177a0: beq	$r3, $r5, . + (242)
L_177a4: or	$r9, $r9, $fp
L_177a8: swi	$r10, [$sp + #0xc]
L_177ac: addi	$r0, $r5, #-1022
L_177b0: add333 $r4, $r4, $r0
L_177b2: mulr64	$r2, $r7, $r9
L_177b6: mulr64	$r0, $r6, $r9
L_177ba: add	$r9, $r2, $r1
L_177be: slt45 $r9, $r1
L_177c0: add	$r10, $r3, $r15
L_177c4: mulr64	$r2, $r7, $r8
L_177c8: add333 $r7, $r2, $r0
L_177ca: slt45 $r7, $r0
L_177cc: add45 $r9, $r15
L_177ce: slt45 $r9, $r15
L_177d0: add45 $r10, $r15
L_177d2: add45 $r9, $r3
L_177d4: slt45 $r9, $r3
L_177d6: add45 $r10, $r15
L_177d8: mulr64	$r0, $r6, $r8
L_177dc: add333 $r7, $r7, $r1
L_177de: slt45 $r7, $r1
L_177e0: add45 $r9, $r15
L_177e2: slt45 $r9, $r15
L_177e4: add	$r1, $r10, $r15
L_177e8: or33 $r7, $r0
L_177ea: ori	$r0, $r9, #1
L_177ee: cmovz	$r0, $r9, $r7
L_177f2: sltsi45 $r1, #0
L_177f4: bnezs8 . + (14)
L_177f6: mov55 $r15, $r0
L_177f8: add333 $r0, $r0, $r0
L_177fa: slt45 $r0, $r15
L_177fc: add333 $r1, $r1, $r1
L_177fe: add45 $r1, $r15
L_17800: subi333 $r4, $r4, #1
L_17802: lwi	$r10, [$sp + #0xc]
L_17806: blez	$r4, . + (226)
L_1780a: subri	$r5, $r4, #0x7ff
L_1780e: blez	$r5, . + (140)
L_17812: addi	$r0, $r0, #0x400
L_17816: slti	$r15, $r0, #0x400
L_1781a: beqzs8 . + (8)
L_1781c: add45 $r1, $r15
L_1781e: slt45 $r1, $r15
L_17820: add45 $r4, $r15
L_17822: srli	$r2, $r0, #0xb
L_17826: xlsb33 $r2, $r2
L_17828: sub333 $r0, $r0, $r2
L_1782a: srli45 $r0, #0xb
L_1782c: slli	$r2, $r1, #0x15
L_17830: or33 $r0, $r2
L_17832: slli333 $r1, $r1, #1
L_17834: srli45 $r1, #0xc
L_17836: slli	$r5, $r4, #0x14
L_1783a: or33 $r1, $r5
L_1783c: or	$r1, $r1, $r10
L_17840: pop25 $r10, #16
L_17842: or	$r15, $r7, $r6
L_17846: beqzs8 . + (50)
L_17848: srli	$r15, $r6, #0x1f
L_1784c: add333 $r7, $r7, $r7
L_1784e: add45 $r7, $r15
L_17850: add333 $r6, $r6, $r6
L_17852: bnez38 $r7, . + (10)
L_17854: mov55 $r7, $r6
L_17856: movi55 $r6, #0
L_17858: addi	$r4, $r4, #-32
L_1785c: clz	$r0, $r7
L_17860: beqz38 $r0, . + (-196)
L_17862: sub333 $r4, $r4, $r0
L_17864: subri	$r2, $r0, #0x20
L_17868: srl	$r2, $r6, $r2
L_1786c: sll	$r6, $r6, $r0
L_17870: sll	$r7, $r7, $r0
L_17874: or33 $r7, $r2
L_17876: j8 . + (-218)
L_17878: beqs38 $r3, . + (42)
L_1787a: mov55 $r1, $r10
L_1787c: pop25 $r10, #16
L_1787e: or33 $r7, $r6
L_17880: bne	$r7, $fp, . + (34)
L_17884: bnez38 $r5, . + (12)
L_17886: slli	$r2, $r9, #1
L_1788a: or	$r2, $r2, $r8
L_1788e: beqz38 $r2, . + (20)
L_17890: bnes38 $r3, . + (10)
L_17892: or	$r9, $r9, $r8
L_17896: bne	$r9, $fp, . + (12)
L_1789a: movi55 $r0, #0
L_1789c: sethi	$r1, #0x7ff00
L_178a0: j8 . + (-100)
L_178a2: movi55 $r0, #0
L_178a4: movi	$r1, #-524288
L_178a8: pop25 $r10, #16
L_178aa: or	$r0, $r9, $r8
L_178ae: beqz38 $r0, . + (-52)
L_178b0: srli	$r15, $r8, #0x1f
L_178b4: add45 $r9, $r9
L_178b6: add45 $r9, $r15
L_178b8: add45 $r8, $r8
L_178ba: bnez	$r9, . + (12)
L_178be: mov55 $r9, $r8
L_178c0: movi55 $r8, #0
L_178c2: addi	$r5, $r5, #-32
L_178c6: clz	$r0, $r9
L_178ca: beqz	$r0, . + (-290)
L_178ce: sub333 $r5, $r5, $r0
L_178d0: subri	$r2, $r0, #0x20
L_178d4: srl	$r2, $r8, $r2
L_178d8: sll	$r8, $r8, $r0
L_178dc: sll	$r9, $r9, $r0
L_178e0: or	$r9, $r9, $r2
L_178e4: j	. + (-316)
L_178e8: movi55 $r6, #0
L_178ea: subri	$r3, $r4, #1
L_178ee: slti	$r15, $r3, #0x20
L_178f2: bnezs8 . + (20)
L_178f4: mov55 $r6, $r0
L_178f6: mov55 $r0, $r1
L_178f8: movi55 $r1, #0
L_178fa: addi	$r3, $r3, #-32
L_178fe: beqz38 $r0, . + (8)
L_17900: slti	$r15, $r3, #0x20
L_17904: beqzs8 . + (52)
L_17906: beqz38 $r3, . + (32)
L_17908: subri	$r2, $r3, #0x20
L_1790c: sll	$r7, $r1, $r2
L_17910: sll	$r5, $r0, $r2
L_17914: srl	$r0, $r0, $r3
L_17918: srl	$r1, $r1, $r3
L_1791c: or33 $r0, $r7
L_1791e: or33 $r6, $r5
L_17920: beqz38 $r6, . + (6)
L_17922: ori	$r0, $r0, #1
L_17926: addi	$r0, $r0, #0x400
L_1792a: slti	$r15, $r0, #0x400
L_1792e: add45 $r1, $r15
L_17930: srli	$r4, $r1, #0x1f
L_17934: j	. + (-274)
L_17938: movi55 $r0, #0
L_1793a: j8 . + (-192)
L_1793c: push25 $r10, #16
L_1793e: slli333 $r4, $r1, #1
L_17940: srli45 $r4, #0x15
L_17942: slli	$r7, $r1, #0xb
L_17946: srli	$r6, $r0, #0x15
L_1794a: or33 $r7, $r6
L_1794c: slli	$r6, $r0, #0xb
L_17950: sethi	$fp, #0x80000
L_17954: slli333 $r5, $r3, #1
L_17956: srli45 $r5, #0x15
L_17958: slli	$r9, $r3, #0xb
L_1795c: srli	$r8, $r2, #0x15
L_17960: or	$r9, $r9, $r8
L_17964: slli	$r8, $r2, #0xb
L_17968: xor33 $r3, $r1
L_1796a: and	$r10, $r3, $fp
L_1796e: movi	$r3, #0x7ff
L_17972: beqz	$r4, . + (394)
L_17976: beq	$r3, $r4, . + (478)
L_1797a: or	$r7, $r7, $fp
L_1797e: beqz	$r5, . + (502)
L_17982: beq	$r3, $r5, . + (486)
L_17986: or	$r9, $r9, $fp
L_1798a: sub333 $r4, $r4, $r5
L_1798c: addi	$r4, $r4, #0x3ff
L_17990: srli45 $r6, #1
L_17992: slli	$r1, $r7, #0x1f
L_17996: or33 $r6, $r1
L_17998: srli45 $r7, #1
L_1799a: srli	$r2, $r9, #0x10
L_1799e: divr	$r3, $r7, $r7, $r2
L_179a2: zeh	$r1, $r9
L_179a6: mul	$r5, $r1, $r3
L_179aa: slli	$r7, $r7, #0x10
L_179ae: srli	$r0, $r6, #0x10
L_179b2: or33 $r7, $r0
L_179b4: mov55 $r0, $r7
L_179b6: sub333 $r7, $r7, $r5
L_179b8: slt45 $r0, $r7
L_179ba: beqzs8 . + (10)
L_179bc: subi333 $r3, $r3, #1
L_179be: add45 $r7, $r9
L_179c0: slt45 $r7, $r9
L_179c2: beqzs8 . + (-6)
L_179c4: divr	$r2, $r7, $r7, $r2
L_179c8: mul	$r5, $r1, $r2
L_179cc: slli	$r7, $r7, #0x10
L_179d0: zeh33 $r0, $r6
L_179d2: or33 $r7, $r0
L_179d4: mov55 $r0, $r7
L_179d6: sub333 $r7, $r7, $r5
L_179d8: slt45 $r0, $r7
L_179da: beqzs8 . + (10)
L_179dc: subi333 $r2, $r2, #1
L_179de: add45 $r7, $r9
L_179e0: slt45 $r7, $r9
L_179e2: beqzs8 . + (-6)
L_179e4: slli	$r3, $r3, #0x10
L_179e8: add333 $r3, $r3, $r2
L_179ea: mulr64	$r0, $r3, $r8
L_179ee: neg33 $r6, $r0
L_179f0: mov55 $r0, $r7
L_179f2: sub333 $r7, $r7, $r1
L_179f4: slt45 $r0, $r7
L_179f6: beqz38 $r6, . + (10)
L_179f8: mov55 $r0, $r7
L_179fa: subi333 $r7, $r7, #1
L_179fc: bnezs8 . + (6)
L_179fe: slt45 $r0, $r7
L_17a00: beqzs8 . + (24)
L_17a02: subi333 $r3, $r3, #1
L_17a04: add45 $r6, $r8
L_17a06: slt	$r0, $r6, $r8
L_17a0a: add45 $r7, $r9
L_17a0c: slt45 $r7, $r9
L_17a0e: beqz38 $r0, . + (8)
L_17a10: addi333 $r7, $r7, #1
L_17a12: bnezs8 . + (6)
L_17a14: slti45 $r7, #1
L_17a16: beqzs8 . + (-20)
L_17a18: bne	$r7, $r9, . + (14)
L_17a1c: mov55 $r1, $r8
L_17a1e: mov55 $r7, $r6
L_17a20: movi55 $r2, #0
L_17a22: movi55 $r0, #0
L_17a24: j8 . + (86)
L_17a26: srli	$r1, $r9, #0x10
L_17a2a: divr	$r2, $r7, $r7, $r1
L_17a2e: zeh	$r0, $r9
L_17a32: mul	$r15, $r0, $r2
L_17a36: slli	$r7, $r7, #0x10
L_17a3a: srli	$r5, $r6, #0x10
L_17a3e: or33 $r7, $r5
L_17a40: mov55 $r5, $r7
L_17a42: sub45 $r7, $r15
L_17a44: slt45 $r5, $r7
L_17a46: beqzs8 . + (10)
L_17a48: subi333 $r2, $r2, #1
L_17a4a: add45 $r7, $r9
L_17a4c: slt45 $r7, $r9
L_17a4e: beqzs8 . + (-6)
L_17a50: divr	$r1, $r7, $r7, $r1
L_17a54: mul	$r5, $r0, $r1
L_17a58: slli	$r7, $r7, #0x10
L_17a5c: zeh33 $r0, $r6
L_17a5e: or33 $r7, $r0
L_17a60: mov55 $r0, $r7
L_17a62: sub333 $r7, $r7, $r5
L_17a64: slt45 $r0, $r7
L_17a66: beqzs8 . + (10)
L_17a68: subi333 $r1, $r1, #1
L_17a6a: add45 $r7, $r9
L_17a6c: slt45 $r7, $r9
L_17a6e: beqzs8 . + (-6)
L_17a70: slli	$r2, $r2, #0x10
L_17a74: add333 $r2, $r2, $r1
L_17a76: mulr64	$r0, $r2, $r8
L_17a7a: neg33 $r6, $r0
L_17a7c: mov55 $r0, $r7
L_17a7e: sub333 $r7, $r7, $r1
L_17a80: slt45 $r0, $r7
L_17a82: beqz38 $r6, . + (10)
L_17a84: mov55 $r0, $r7
L_17a86: subi333 $r7, $r7, #1
L_17a88: bnezs8 . + (6)
L_17a8a: slt45 $r0, $r7
L_17a8c: beqzs8 . + (24)
L_17a8e: subi333 $r2, $r2, #1
L_17a90: add45 $r6, $r8
L_17a92: slt	$r0, $r6, $r8
L_17a96: add45 $r7, $r9
L_17a98: slt45 $r7, $r9
L_17a9a: beqz38 $r0, . + (8)
L_17a9c: addi333 $r7, $r7, #1
L_17a9e: bnezs8 . + (6)
L_17aa0: slti45 $r7, #1
L_17aa2: beqzs8 . + (-20)
L_17aa4: sltsi45 $r3, #0
L_17aa6: bnezs8 . + (14)
L_17aa8: mov55 $r15, $r2
L_17aaa: add333 $r2, $r2, $r2
L_17aac: slt45 $r2, $r15
L_17aae: add333 $r3, $r3, $r3
L_17ab0: add45 $r3, $r15
L_17ab2: subi333 $r4, $r4, #1
L_17ab4: or33 $r7, $r6
L_17ab6: ori	$r0, $r2, #1
L_17aba: cmovz	$r0, $r2, $r7
L_17abe: mov55 $r1, $r3
L_17ac0: blez	$r4, . + (242)
L_17ac4: subri	$r5, $r4, #0x7ff
L_17ac8: blez	$r5, . + (148)
L_17acc: addi	$r0, $r0, #0x400
L_17ad0: slti	$r15, $r0, #0x400
L_17ad4: beqzs8 . + (8)
L_17ad6: add45 $r1, $r15
L_17ad8: slt45 $r1, $r15
L_17ada: add45 $r4, $r15
L_17adc: srli	$r2, $r0, #0xb
L_17ae0: xlsb33 $r2, $r2
L_17ae2: sub333 $r0, $r0, $r2
L_17ae4: srli45 $r0, #0xb
L_17ae6: slli	$r2, $r1, #0x15
L_17aea: or33 $r0, $r2
L_17aec: slli333 $r1, $r1, #1
L_17aee: srli45 $r1, #0xc
L_17af0: slli	$r5, $r4, #0x14
L_17af4: or33 $r1, $r5
L_17af6: or	$r1, $r1, $r10
L_17afa: pop25 $r10, #16
L_17afc: or	$r15, $r7, $r6
L_17b00: beqzs8 . + (54)
L_17b02: srli	$r15, $r6, #0x1f
L_17b06: add333 $r7, $r7, $r7
L_17b08: add45 $r7, $r15
L_17b0a: add333 $r6, $r6, $r6
L_17b0c: bnez38 $r7, . + (10)
L_17b0e: mov55 $r7, $r6
L_17b10: movi55 $r6, #0
L_17b12: addi	$r4, $r4, #-32
L_17b16: clz	$r0, $r7
L_17b1a: beqz	$r0, . + (-412)
L_17b1e: sub333 $r4, $r4, $r0
L_17b20: subri	$r2, $r0, #0x20
L_17b24: srl	$r2, $r6, $r2
L_17b28: sll	$r6, $r6, $r0
L_17b2c: sll	$r7, $r7, $r0
L_17b30: or33 $r7, $r2
L_17b32: j	. + (-436)
L_17b36: beqs38 $r3, . + (14)
L_17b38: bnez38 $r5, . + (8)
L_17b3a: or	$r15, $r9, $r8
L_17b3e: beqzs8 . + (14)
L_17b40: mov55 $r1, $r10
L_17b42: pop25 $r10, #16
L_17b44: or	$r9, $r9, $r8
L_17b48: beq	$r9, $fp, . + (-8)
L_17b4c: movi55 $r0, #0
L_17b4e: movi	$r1, #-524288
L_17b52: pop25 $r10, #16
L_17b54: or33 $r7, $r6
L_17b56: bne	$r7, $fp, . + (-10)
L_17b5a: beqs38 $r3, . + (-14)
L_17b5c: movi55 $r0, #0
L_17b5e: sethi	$r1, #0x7ff00
L_17b62: or	$r1, $r1, $r10
L_17b66: pop25 $r10, #16
L_17b68: or	$r9, $r9, $r8
L_17b6c: bne	$r9, $fp, . + (-32)
L_17b70: movi55 $r0, #0
L_17b72: j8 . + (-50)
L_17b74: or	$r15, $r9, $r8
L_17b78: beqzs8 . + (-28)
L_17b7a: srli	$r15, $r8, #0x1f
L_17b7e: add45 $r9, $r9
L_17b80: add45 $r9, $r15
L_17b82: add45 $r8, $r8
L_17b84: bnez	$r9, . + (12)
L_17b88: mov55 $r9, $r8
L_17b8a: movi55 $r8, #0
L_17b8c: addi	$r5, $r5, #-32
L_17b90: clz	$r0, $r9
L_17b94: beqz	$r0, . + (-522)
L_17b98: sub333 $r5, $r5, $r0
L_17b9a: subri	$r2, $r0, #0x20
L_17b9e: srl	$r2, $r8, $r2
L_17ba2: sll	$r8, $r8, $r0
L_17ba6: sll	$r9, $r9, $r0
L_17baa: or	$r9, $r9, $r2
L_17bae: j	. + (-548)
L_17bb2: movi55 $r6, #0
L_17bb4: subri	$r3, $r4, #1
L_17bb8: slti	$r15, $r3, #0x20
L_17bbc: bnezs8 . + (20)
L_17bbe: mov55 $r6, $r0
L_17bc0: mov55 $r0, $r1
L_17bc2: movi55 $r1, #0
L_17bc4: addi	$r3, $r3, #-32
L_17bc8: beqz38 $r0, . + (8)
L_17bca: slti	$r15, $r3, #0x20
L_17bce: beqzs8 . + (52)
L_17bd0: beqz38 $r3, . + (32)
L_17bd2: subri	$r2, $r3, #0x20
L_17bd6: sll	$r7, $r1, $r2
L_17bda: sll	$r5, $r0, $r2
L_17bde: srl	$r0, $r0, $r3
L_17be2: srl	$r1, $r1, $r3
L_17be6: or33 $r0, $r7
L_17be8: or33 $r6, $r5
L_17bea: beqz38 $r6, . + (6)
L_17bec: ori	$r0, $r0, #1
L_17bf0: addi	$r0, $r0, #0x400
L_17bf4: slti	$r15, $r0, #0x400
L_17bf8: add45 $r1, $r15
L_17bfa: srli	$r4, $r1, #0x1f
L_17bfe: j	. + (-290)
L_17c02: movi55 $r0, #0
L_17c04: j8 . + (-196)
L_17c06: nop16
L_17c08: smw.adm $r6, [$sp], $r6, #0x0
L_17c0c: slli333 $r5, $r1, #1
L_17c0e: srli45 $r5, #0x15
L_17c10: slli	$r4, $r1, #0xb
L_17c14: srli	$r6, $r0, #0x15
L_17c18: or33 $r4, $r6
L_17c1a: slli	$r3, $r0, #0xb
L_17c1e: sethi	$r6, #0x80000
L_17c22: or33 $r4, $r6
L_17c24: slti	$r15, $r5, #0x43e
L_17c28: beqzs8 . + (66)
L_17c2a: subri	$r2, $r5, #0x43e
L_17c2e: movpi45 $r6, #0x20
L_17c30: slt45 $r2, $r6
L_17c32: bnezs8 . + (12)
L_17c34: mov55 $r3, $r4
L_17c36: movi55 $r4, #0
L_17c38: addi	$r2, $r2, #-32
L_17c3c: bnez38 $r3, . + (-14)
L_17c3e: beqz38 $r2, . + (22)
L_17c40: mov55 $r0, $r4
L_17c42: srl	$r3, $r3, $r2
L_17c46: srl	$r4, $r4, $r2
L_17c4a: subri	$r2, $r2, #0x20
L_17c4e: sll	$r0, $r0, $r2
L_17c52: or33 $r3, $r0
L_17c54: sltsi45 $r1, #0
L_17c56: beqzs8 . + (10)
L_17c58: neg33 $r4, $r4
L_17c5a: beqz38 $r3, . + (6)
L_17c5c: neg33 $r3, $r3
L_17c5e: subi45 $r4, #1
L_17c60: mov55 $r0, $r3
L_17c62: mov55 $r1, $r4
L_17c64: lmw.bim $r6, [$sp], $r6, #0x0
L_17c68: ret5 $lp
L_17c6a: sltsi45 $r1, #0
L_17c6c: bnezs8 . + (14)
L_17c6e: subri	$r15, $r5, #0x7ff
L_17c72: bnezs8 . + (16)
L_17c74: slli333 $r6, $r4, #1
L_17c76: or33 $r6, $r3
L_17c78: beqz38 $r6, . + (10)
L_17c7a: sethi	$r4, #0x80000
L_17c7e: movi55 $r3, #0
L_17c80: j8 . + (-32)
L_17c82: sethi	$r4, #0x7ffff
L_17c86: ori	$r4, $r4, #0xfff
L_17c8a: movi55 $r3, #-1
L_17c8c: j8 . + (-44)
L_17c8e: nop16
L_17c90: smw.adm $r6, [$sp], $r6, #0x0
L_17c94: movi55 $r1, #0
L_17c96: mov55 $r5, $r1
L_17c98: mov55 $r3, $r1
L_17c9a: mov55 $r2, $r0
L_17c9c: beqz38 $r2, . + (32)
L_17c9e: sltsi45 $r2, #0
L_17ca0: beqzs8 . + (14)
L_17ca2: sethi	$r5, #0x80000
L_17ca6: neg33 $r2, $r2
L_17ca8: beqz38 $r1, . + (6)
L_17caa: neg33 $r1, $r1
L_17cac: subi45 $r2, #1
L_17cae: movi	$r3, #0x41e
L_17cb2: clz	$r4, $r2
L_17cb6: sub333 $r3, $r3, $r4
L_17cb8: sll	$r2, $r2, $r4
L_17cbc: srli	$r4, $r1, #0xb
L_17cc0: slli	$r6, $r2, #0x15
L_17cc4: or33 $r4, $r6
L_17cc6: slli333 $r6, $r2, #1
L_17cc8: srli45 $r6, #0xc
L_17cca: or33 $r5, $r6
L_17ccc: slli	$r6, $r3, #0x14
L_17cd0: or33 $r5, $r6
L_17cd2: mov55 $r0, $r4
L_17cd4: mov55 $r1, $r5
L_17cd6: lmw.bim $r6, [$sp], $r6, #0x0
L_17cda: ret5 $lp
L_17cdc: movi	$r0, #0x2d814
L_17ce0: ret5 $lp
L_17ce2: nop16
L_17ce4: mov55 $r3, $r0
L_17ce6: slti45 $r2, #4
L_17ce8: bnezs8 . + (80)
L_17cea: andi	$r4, $r3, #3
L_17cee: andi	$r5, $r1, #3
L_17cf2: beqz38 $r5, . + (22)
L_17cf4: subri	$r5, $r5, #4
L_17cf8: sub333 $r2, $r2, $r5
L_17cfa: add333 $r5, $r5, $r1
L_17cfc: lbi.bi	$r4, [$r1], #1
L_17d00: sbi.bi	$r4, [$r3], #1
L_17d04: bne	$r1, $r5, . + (-8)
L_17d08: srli	$r5, $r2, #5
L_17d0c: beqz38 $r5, . + (20)
L_17d0e: slli333 $r5, $r5, #5
L_17d10: add333 $r5, $r5, $r1
L_17d12: fexti33 $r2, #4
L_17d14: lmw.bim $r16, [$r1], $r23, #0x0
L_17d18: smw.bim $r16, [$r3], $r23, #0x0
L_17d1c: bne	$r1, $r5, . + (-8)
L_17d20: srli	$r5, $r2, #2
L_17d24: beqz38 $r5, . + (20)
L_17d26: slli333 $r5, $r5, #2
L_17d28: add333 $r5, $r5, $r1
L_17d2a: fexti33 $r2, #1
L_17d2c: lmw.bim $r4, [$r1], $r4, #0x0
L_17d30: smw.bim $r4, [$r3], $r4, #0x0
L_17d34: bne	$r1, $r5, . + (-8)
L_17d38: beqz38 $r2, . + (16)
L_17d3a: add333 $r5, $r1, $r2
L_17d3c: lbi.bi	$r4, [$r1], #1
L_17d40: sbi.bi	$r4, [$r3], #1
L_17d44: bne	$r1, $r5, . + (-8)
L_17d48: ret5 $lp
L_17d4a: nop16
L_17d4c: mov55 $r3, $r0
L_17d4e: slti45 $r2, #4
L_17d50: bnezs8 . + (96)
L_17d52: andi	$r5, $r3, #3
L_17d56: beqz38 $r5, . + (18)
L_17d58: subri	$r5, $r5, #4
L_17d5c: sub333 $r2, $r2, $r5
L_17d5e: add333 $r5, $r5, $r0
L_17d60: sbi.bi	$r1, [$r3], #1
L_17d64: bne	$r3, $r5, . + (-4)
L_17d68: zeb33 $r1, $r1
L_17d6a: slli	$r4, $r1, #8
L_17d6e: or33 $r1, $r4
L_17d70: slli	$r4, $r1, #0x10
L_17d74: or33 $r1, $r4
L_17d76: srli	$r5, $r2, #5
L_17d7a: beqz	$r5, . + (34)
L_17d7e: slli333 $r5, $r5, #5
L_17d80: add333 $r5, $r5, $r3
L_17d82: fexti33 $r2, #4
L_17d84: mov55 $r16, $r1
L_17d86: mov55 $r17, $r1
L_17d88: mov55 $r18, $r1
L_17d8a: mov55 $r19, $r1
L_17d8c: mov55 $r20, $r1
L_17d8e: mov55 $r21, $r1
L_17d90: mov55 $r22, $r1
L_17d92: mov55 $r23, $r1
L_17d94: smw.bim $r16, [$r3], $r23, #0x0
L_17d98: bne	$r3, $r5, . + (-4)
L_17d9c: srli	$r5, $r2, #2
L_17da0: beqz38 $r5, . + (16)
L_17da2: slli333 $r5, $r5, #2
L_17da4: add333 $r5, $r5, $r3
L_17da6: fexti33 $r2, #1
L_17da8: smw.bim $r1, [$r3], $r1, #0x0
L_17dac: bne	$r3, $r5, . + (-4)
L_17db0: beqz38 $r2, . + (12)
L_17db2: add333 $r5, $r3, $r2
L_17db4: sbi.bi	$r1, [$r3], #1
L_17db8: bne	$r3, $r5, . + (-4)
L_17dbc: ret5 $lp
# Explicit NOP16 completes the halfword alignment before the EX9 table.
L_17dbe: nop16
agent18_upper_end:

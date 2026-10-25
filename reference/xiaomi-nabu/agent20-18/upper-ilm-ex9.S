# SPDX-License-Identifier: GPL-3.0-only
# Independent GNU NDS32 assembly verification of 512 EX9 instruction-table slots.
# Firmware file [0x17dc0,0x185c0), runtime [0x17bc0,0x183c0).
# Each entry is one physical 32-bit instruction. No copied opcode/data words.
# EX9 J/JAL uses PC[31:25] concatenated with target[24:1] and a zero low bit.
# For this low-address firmware, the semantic target is the absolute runtime address.
# `. + target` asks the ordinary assembler to produce precisely that halfword field;
# it is an encoding cross-check and does not assert ordinary PC-relative table execution.
# Disable GAS and LD relaxation. The .flag directive preserves physical mnemonics.
.section .text
.flag verbatim
.global agent18_ex9_table
.type agent18_ex9_table,@object
agent18_ex9_table:
.L_ex9_000: lwi.gp	$r0, [ + #-5436]
.L_ex9_001: jal . + 0x84aa
.L_ex9_002: jal . + 0x23de
.L_ex9_003: movi	$r0, #0x3f04a
.L_ex9_004: movi	$r1, #0x55
.L_ex9_005: jal . + 0x25fe
.L_ex9_006: jal . + 0xe328
.L_ex9_007: jal . + 0x94f0
.L_ex9_008: jal . + 0x17ae4
.L_ex9_009: jal . + 0x94f0
.L_ex9_010: jal . + 0x85e8
.L_ex9_011: movi	$r0, #0x38d40
.L_ex9_012: lwi.gp	$r0, [ + #-5356]
.L_ex9_013: j . + 0x2aa
.L_ex9_014: jal . + 0x84aa
.L_ex9_015: jal . + 0x433c
.L_ex9_016: jal . + 0x85e8
.L_ex9_017: jal . + 0x23de
.L_ex9_018: lwi.gp	$r1, [ + #-5436]
.L_ex9_019: lwi.gp	$r0, [ + #-4500]
.L_ex9_020: jal . + 0x8c04
.L_ex9_021: lwi.gp	$r1, [ + #-5284]
.L_ex9_022: lbi.gp	$r0, [ + #-4629]
.L_ex9_023: jal . + 0x854a
.L_ex9_024: lbi.gp	$r0, [ + #-5619]
.L_ex9_025: lwi.gp	$r0, [ + #-5284]
.L_ex9_026: lwi.gp	$r1, [ + #-5356]
.L_ex9_027: lwi.gp	$r0, [ + #-5440]
.L_ex9_028: jal . + 0xe328
.L_ex9_029: movi	$r2, #0x190
.L_ex9_030: lbi.gp	$r0, [ + #-5596]
.L_ex9_031: lmw.bim $r6, [$sp], $r6, #0x2
.L_ex9_032: j . + 0x12340
.L_ex9_033: swi.gp	$r0, [ + #-5292]
.L_ex9_034: movi	$r3, #0x190
.L_ex9_035: lbsi.gp	$r0, [ + #-4902]
.L_ex9_036: movi	$r1, #0x2fef0
.L_ex9_037: lwi.gp	$r0, [ + #-5292]
.L_ex9_038: jal . + 0x1755c
.L_ex9_039: jal . + 0x17380
.L_ex9_040: jal . + 0xf66c
.L_ex9_041: sbi.gp	$r0, [ + #-4902]
.L_ex9_042: jal . + 0x17188
.L_ex9_043: movi	$r2, #0x180
.L_ex9_044: sbi	$r0, [$sp + #7]
.L_ex9_045: jal . + 0xe11c
.L_ex9_046: swi.gp	$r0, [ + #-4772]
.L_ex9_047: lbi.gp	$r0, [ + #0xe64]
.L_ex9_048: movi	$r0, #0x55
.L_ex9_049: swi.gp	$r0, [ + #-5296]
.L_ex9_050: nop
.L_ex9_051: addi	$r0, $r6, #-184
.L_ex9_052: add	$r15, $r0, $r15
.L_ex9_053: add_slli	$r3, $r2, $r3, #2
.L_ex9_054: movi	$r3, #0x55
.L_ex9_055: lbi.gp	$r0, [ + #0x130d]
.L_ex9_056: lbi.gp	$r0, [ + #-55620]
.L_ex9_057: jal . + 0xbd64
.L_ex9_058: movi	$r2, #0x3f04a
.L_ex9_059: sbi.gp	$r0, [ + #-5586]
.L_ex9_060: movi	$r2, #0x55
.L_ex9_061: movi	$r0, #-86
.L_ex9_062: jal . + 0x2d6a
.L_ex9_063: lwi.gp	$r0, [ + #-5328]
.L_ex9_064: jal . + 0xfb8c
.L_ex9_065: bitci	$r0, $r0, #0xe0
.L_ex9_066: btst	$r0, $r0, #2
.L_ex9_067: movi	$r3, #0x38d41
.L_ex9_068: andi	$r0, $r0, #0x7fff
.L_ex9_069: jal . + 0x88d0
.L_ex9_070: lbi.gp	$r1, [ + #-4742]
.L_ex9_071: sbi.gp	$r0, [ + #-5665]
.L_ex9_072: sbi.gp	$r0, [ + #-5596]
.L_ex9_073: swi.gp	$r0, [ + #0xe80]
.L_ex9_074: btst	$r0, $r0, #0x1b
.L_ex9_075: jal . + 0x17b4c
.L_ex9_076: lhsi	$r1, [$r1 + #2]
.L_ex9_077: add_slli	$r0, $r1, $r0, #2
.L_ex9_078: divsr	$r0, $r0, $r0, $r1
.L_ex9_079: sbi	$r0, [$sp + #0xf]
.L_ex9_080: movi	$r0, #0x2c504
.L_ex9_081: sbi	$r1, [$r0 + #0x1c]
.L_ex9_082: sbi	$r9, [$r7 + #0]
.L_ex9_083: movi	$r1, #0x3f04a
.L_ex9_084: shi	$r0, [$sp + #6]
.L_ex9_085: lmw.bi $r2, [$r2], $r3, #0x0
.L_ex9_086: btst	$r0, $r0, #4
.L_ex9_087: movi	$r0, #0x2eca8
.L_ex9_088: jal . + 0x2cec
.L_ex9_089: jal . + 0xec40
.L_ex9_090: jal . + 0x25fe
.L_ex9_091: jal . + 0x23e6
.L_ex9_092: sbi.gp	$r0, [ + #-4901]
.L_ex9_093: lwi.gp	$r0, [ + #-5124]
.L_ex9_094: swi.gp	$r0, [ + #0xe7c]
.L_ex9_095: ori	$r0, $r0, #4
.L_ex9_096: jal . + 0x41d0
.L_ex9_097: lbi.gp	$r0, [ + #-4782]
.L_ex9_098: lwi.gp	$r1, [ + #-5340]
.L_ex9_099: sbi	$r0, [$r9 + #1]
.L_ex9_100: j . + 0xa4e6
.L_ex9_101: lhi	$r0, [$sp + #6]
.L_ex9_102: andi	$r6, $r6, #0x7fff
.L_ex9_103: jal . + 0xfb8c
.L_ex9_104: movi	$r0, #0x2fef0
.L_ex9_105: jal . + 0x1737c
.L_ex9_106: ori	$r0, $r0, #1
.L_ex9_107: jal . + 0x28e2
.L_ex9_108: movi	$r0, #0x2e144
.L_ex9_109: lwi.gp	$r0, [ + #-444]
.L_ex9_110: lb	$r0, [$r15 + ($r0 << #0)]
.L_ex9_111: lbi.gp	$r0, [ + #-5676]
.L_ex9_112: lwi.gp	$r1, [ + #-4500]
.L_ex9_113: jal . + 0xe8b6
.L_ex9_114: jal . + 0xfaf2
.L_ex9_115: movi	$r7, #0x3f04a
.L_ex9_116: lhi.gp	$r0, [ + #-5542]
.L_ex9_117: lwi.gp	$r0, [ + #-5000]
.L_ex9_118: swi.gp	$r0, [ + #-5440]
.L_ex9_119: movi	$r0, #0xff
.L_ex9_120: movi	$r9, #0x55
.L_ex9_121: movi	$r1, #0x31c20
.L_ex9_122: jal . + 0xbd64
.L_ex9_123: andi	$r9, $r0, #0xff
.L_ex9_124: lwi.gp	$r0, [ + #-5288]
.L_ex9_125: mtsr	$r0, $int_mask2
.L_ex9_126: smw.bim $r16, [$r1], $r25, #0x0
.L_ex9_127: jal . + 0x8c04
.L_ex9_128: sbi.gp	$r0, [ + #-6328]
.L_ex9_129: jal . + 0xdf48
.L_ex9_130: shi	$r0, [$sp + #0x14]
.L_ex9_131: movi	$r0, #0x2edc0
.L_ex9_132: movi	$r0, #0x2fe50
.L_ex9_133: movi	$r0, #0x50
.L_ex9_134: movi	$r1, #0x3f403
.L_ex9_135: lwi.gp	$r0, [ + #-5416]
.L_ex9_136: sbi	$r0, [$r6 + #0x50]
.L_ex9_137: sbi.gp	$r0, [ + #-4802]
.L_ex9_138: sbi	$r0, [$r9 + #0]
.L_ex9_139: lbi	$r0, [$r9 + #1]
.L_ex9_140: movi	$r1, #0x64
.L_ex9_141: lbi	$r0, [$r10 + #0]
.L_ex9_142: jal . + 0x43a0
.L_ex9_143: jal . + 0xffac
.L_ex9_144: movi	$r3, #0x80
.L_ex9_145: sbi.gp	$r0, [ + #-4899]
.L_ex9_146: shi.gp	$r0, [ + #-5546]
.L_ex9_147: shi	$r0, [$sp + #0x16]
.L_ex9_148: lbi.gp	$r0, [ + #-5697]
.L_ex9_149: sbi.gp	$r0, [ + #-5672]
.L_ex9_150: lbi.gp	$r0, [ + #0x13d2]
.L_ex9_151: movi	$r6, #0x3f04a
.L_ex9_152: lwi.gp	$r0, [ + #-440]
.L_ex9_153: sbi.gp	$r0, [ + #-5686]
.L_ex9_154: jal . + 0xe542
.L_ex9_155: lbi.gp	$r0, [ + #-5657]
.L_ex9_156: sbi.gp	$r2, [ + #-6311]
.L_ex9_157: andi	$r1, $r1, #0x7fff
.L_ex9_158: add	$r15, $r1, $r15
.L_ex9_159: lbi.gp	$r0, [ + #-55621]
.L_ex9_160: lhi	$r15, [$r15 + #0]
.L_ex9_161: movi	$r0, #0x20000
.L_ex9_162: sbi	$r1, [$r0 + #0x51]
.L_ex9_163: lmw.bim $r16, [$r0], $r25, #0x0
.L_ex9_164: jal . + 0x10bb8
.L_ex9_165: jal . + 0xf63e
.L_ex9_166: shi.gp	$r0, [ + #0x1338]
.L_ex9_167: andi	$r0, $r0, #0xfe
.L_ex9_168: movi	$r6, #0x2fef0
.L_ex9_169: movi	$r0, #0x69
.L_ex9_170: ori	$r0, $r0, #0x20
.L_ex9_171: lwi.gp	$r0, [ + #0x12c8]
.L_ex9_172: divsr	$r2, $r2, $r2, $r3
.L_ex9_173: sbi	$r0, [$r10 + #0]
.L_ex9_174: sbi.gp	$r0, [ + #-5641]
.L_ex9_175: movi	$r1, #0x2fe50
.L_ex9_176: lbi.gp	$r0, [ + #-4912]
.L_ex9_177: swi.gp	$r0, [ + #0xe60]
.L_ex9_178: sbi.gp	$r0, [ + #-5649]
.L_ex9_179: movi	$r1, #0x32
.L_ex9_180: movi	$r0, #0x32
.L_ex9_181: addi	$r2, $sp, #7
.L_ex9_182: lbi	$r0, [$r0 + #0x51]
.L_ex9_183: movi	$r0, #0x30000
.L_ex9_184: lhsi	$r3, [$r1 + #0]
.L_ex9_185: bitci	$r0, $r0, #2
.L_ex9_186: bitci	$r0, $r0, #4
.L_ex9_187: ori	$r0, $r0, #0x10
.L_ex9_188: movi	$r1, #0x26238
.L_ex9_189: movi	$r1, #0x4c
.L_ex9_190: lbsi.gp	$r0, [ + #-4901]
.L_ex9_191: jal . + 0x7bce
.L_ex9_192: j . + 0x2e2
.L_ex9_193: movi	$r7, #0x55
.L_ex9_194: divsr	$r0, $r0, $r1, $r0
.L_ex9_195: sbi.gp	$r0, [ + #-5594]
.L_ex9_196: lbi.gp	$r0, [ + #-5640]
.L_ex9_197: j . + 0x250
.L_ex9_198: sw	$r3, [$r1 + ($r2 << #2)]
.L_ex9_199: shi.gp	$r0, [ + #-4080]
.L_ex9_200: movi	$r0, #0x78
.L_ex9_201: abs	$r2, $r2
.L_ex9_202: movi	$r0, #0x373e8
.L_ex9_203: lwi.gp	$r0, [ + #-352]
.L_ex9_204: divsr	$r1, $r1, $r0, $r1
.L_ex9_205: lwi.gp	$r5, [ + #-5376]
.L_ex9_206: bitci	$r0, $r0, #0x10
.L_ex9_207: abs	$r1, $r1
.L_ex9_208: jal . + 0xe91a
.L_ex9_209: jal . + 0xd3d4
.L_ex9_210: sbi.gp	$r0, [ + #-5620]
.L_ex9_211: lbi.gp	$r0, [ + #-5665]
.L_ex9_212: jal . + 0x105cc
.L_ex9_213: slti	$r0, $r0, #1
.L_ex9_214: addi	$r0, $r0, #0x4c
.L_ex9_215: lbi.gp	$r0, [ + #-6327]
.L_ex9_216: lbi.gp	$r0, [ + #-4918]
.L_ex9_217: sbi	$r0, [$r6 + #0x41]
.L_ex9_218: lbi.gp	$r0, [ + #0xe84]
.L_ex9_219: add_slli	$r0, $r0, $r6, #2
.L_ex9_220: swi.gp	$r0, [ + #-5216]
.L_ex9_221: movi	$r0, #0x4c
.L_ex9_222: lbi.gp	$r0, [ + #-5592]
.L_ex9_223: andi	$r1, $r1, #0xfe
.L_ex9_224: movi	$r2, #0x2c548
.L_ex9_225: shi.gp	$r0, [ + #-4078]
.L_ex9_226: lbi.gp	$r0, [ + #-4203]
.L_ex9_227: sbi	$r0, [$r11 + #0]
.L_ex9_228: ori	$r0, $r0, #2
.L_ex9_229: jal . + 0xea44
.L_ex9_230: lwi.gp	$r0, [ + #-348]
.L_ex9_231: jal . + 0xe8b6
.L_ex9_232: divsr	$r1, $r1, $r1, $r7
.L_ex9_233: ori	$r0, $r0, #0x80
.L_ex9_234: sbi.gp	$r0, [ + #-5657]
.L_ex9_235: sbi.gp	$r0, [ + #-5619]
.L_ex9_236: lbi	$r0, [$r9 + #0]
.L_ex9_237: swi.gp	$r6, [ + #-5072]
.L_ex9_238: add_slli	$r3, $r4, $r3, #8
.L_ex9_239: sbi.bi	$r1, [$r5], #1
.L_ex9_240: lbi.gp	$r0, [ + #-5603]
.L_ex9_241: sbi.gp	$r0, [ + #-4190]
.L_ex9_242: jal . + 0x16d7c
.L_ex9_243: lbi.gp	$r0, [ + #-4508]
.L_ex9_244: addi	$r1, $sp, #0xf
.L_ex9_245: lbi.gp	$r0, [ + #-4104]
.L_ex9_246: shi.gp	$r0, [ + #-5492]
.L_ex9_247: lwi.gp	$r0, [ + #-2980]
.L_ex9_248: addi	$r5, $r5, #0xf00
.L_ex9_249: jal . + 0x854a
.L_ex9_250: movi	$r1, #0x2c3be
.L_ex9_251: jal . + 0xd200
.L_ex9_252: addi	$r1, $r1, #0xfff
.L_ex9_253: addi	$r0, $sp, #0xe
.L_ex9_254: jal . + 0x11854
.L_ex9_255: mfsr	$r1, $int_mask2
.L_ex9_256: shi.gp	$r0, [ + #-4828]
.L_ex9_257: movi	$r0, #0x12c
.L_ex9_258: lwi.gp	$r0, [ + #0xe60]
.L_ex9_259: jal . + 0x2ac8
.L_ex9_260: sb	$r2, [$r1 + ($r0 << #0)]
.L_ex9_261: movi	$r1, #0x3f248
.L_ex9_262: jal . + 0x17a90
.L_ex9_263: movi	$r2, #0x2eca8
.L_ex9_264: movi	$r4, #0x2d428
.L_ex9_265: lw	$r0, [$r9 + ($r1 << #2)]
.L_ex9_266: lwi.gp	$r1, [ + #-5440]
.L_ex9_267: jal . + 0x1037c
.L_ex9_268: sbi.gp	$r0, [ + #-4753]
.L_ex9_269: sbi.gp	$r0, [ + #-5603]
.L_ex9_270: shi	$r0, [$sp + #4]
.L_ex9_271: lwi	$r6, [$r5 + #0x58]
.L_ex9_272: lbi.gp	$r1, [ + #0x130d]
.L_ex9_273: lbi.gp	$r0, [ + #-5710]
.L_ex9_274: lbi.gp	$r0, [ + #-55474]
.L_ex9_275: sbi.gp	$r0, [ + #-4191]
.L_ex9_276: movi	$r2, #0x2ed80
.L_ex9_277: movi	$r1, #-131072
.L_ex9_278: jal . + 0x10c06
.L_ex9_279: divsr	$r1, $r1, $r1, $r3
.L_ex9_280: sltsi	$r15, $r0, #0x33
.L_ex9_281: lwi.gp	$r1, [ + #-4964]
.L_ex9_282: movi	$r0, #0x2fef1
.L_ex9_283: sw	$r5, [$r6 + ($r3 << #2)]
.L_ex9_284: lhi	$r15, [$r10 + #0]
.L_ex9_285: sbi.gp	$r0, [ + #-5695]
.L_ex9_286: jal . + 0x950a
.L_ex9_287: movi	$r0, #0x64
.L_ex9_288: movi	$r1, #0x3f1c0
.L_ex9_289: andi	$r0, $r0, #0xf0
.L_ex9_290: movi	$r1, #0x2eca8
.L_ex9_291: jal . + 0x7bce
.L_ex9_292: movi	$r2, #0xffff
.L_ex9_293: movi	$r0, #0x2dce8
.L_ex9_294: addi	$r0, $sp, #6
.L_ex9_295: lwi.gp	$r1, [ + #-5416]
.L_ex9_296: movi	$r0, #0x397de
.L_ex9_297: movi	$r0, #0x397dc
.L_ex9_298: andi	$r0, $r0, #0xfd
.L_ex9_299: lhi	$r1, [$sp + #0]
.L_ex9_300: movi	$r1, #0x2dce8
.L_ex9_301: lwi.gp	$r0, [ + #-1524]
.L_ex9_302: mtsr	$r1, $int_pend2
.L_ex9_303: maddr32	$r0, $r1, $r2
.L_ex9_304: lwi	$r1, [$r6 + #0x20]
.L_ex9_305: swi.gp	$r6, [ + #-3772]
.L_ex9_306: sbi.gp	$r0, [ + #0x133e]
.L_ex9_307: addi	$r3, $sp, #7
.L_ex9_308: lbi.gp	$r1, [ + #-5619]
.L_ex9_309: sw	$r0, [$r1 + ($r6 << #2)]
.L_ex9_310: shi.gp	$r0, [ + #-4804]
.L_ex9_311: movi	$r1, #0x38d40
.L_ex9_312: jal . + 0x86ee
.L_ex9_313: lbi.gp	$r0, [ + #-4798]
.L_ex9_314: lhi.gp	$r2, [ + #-55724]
.L_ex9_315: lbi.gp	$r3, [ + #-55774]
.L_ex9_316: andi	$r10, $r0, #0xff
.L_ex9_317: abs	$r0, $r0
.L_ex9_318: lbi.gp	$r0, [ + #-4911]
.L_ex9_319: movi	$r7, #0x2c7e4
.L_ex9_320: sbi.gp	$r0, [ + #-4104]
.L_ex9_321: swi.gp	$r0, [ + #-4768]
.L_ex9_322: sbi.gp	$r0, [ + #-4483]
.L_ex9_323: movi	$r1, #0x36510
.L_ex9_324: movi	$r2, #0x2e050
.L_ex9_325: slt	$r0, $r1, $r0
.L_ex9_326: lbi	$r0, [$r6 + #0x7d]
.L_ex9_327: seh	$r15, $r15
.L_ex9_328: jal . + 0x921c
.L_ex9_329: lbi	$r0, [$sp + #7]
.L_ex9_330: sbi.gp	$r0, [ + #-5570]
.L_ex9_331: jal . + 0x12fb2
.L_ex9_332: swi.gp	$r1, [ + #-5296]
.L_ex9_333: sethi	$r5, #0xfff
.L_ex9_334: jal . + 0x942e
.L_ex9_335: movi	$r0, #0x38d42
.L_ex9_336: lwi.gp	$r0, [ + #-5376]
.L_ex9_337: slti	$r15, $r0, #0x12c
.L_ex9_338: btst	$r0, $r0, #1
.L_ex9_339: movi	$r0, #0x52
.L_ex9_340: jal . + 0xecf8
.L_ex9_341: lbi	$r1, [$r9 + #1]
.L_ex9_342: shi.gp	$r1, [ + #-5498]
.L_ex9_343: lbi.gp	$r0, [ + #-5629]
.L_ex9_344: movi	$r2, #0x2cd94
.L_ex9_345: movi	$r6, #0x64
.L_ex9_346: lhi.gp	$r0, [ + #-4080]
.L_ex9_347: movi	$r1, #0x2dc20
.L_ex9_348: movi	$r1, #0x2c401
.L_ex9_349: jal . + 0x1046a
.L_ex9_350: movi	$r1, #0x2c6d0
.L_ex9_351: lbi.gp	$r0, [ + #-5618]
.L_ex9_352: movi	$r3, #0x32
.L_ex9_353: lmw.bi $r0, [$r0], $r0, #0x0
.L_ex9_354: mtsr	$r0, 194
.L_ex9_355: btst	$r0, $r0, #0x1c
.L_ex9_356: lbi.gp	$r0, [ + #-5664]
.L_ex9_357: sbi.gp	$r0, [ + #-4912]
.L_ex9_358: srli	$r0, $r6, #8
.L_ex9_359: cmovz	$r0, $r1, $r15
.L_ex9_360: sbi.gp	$r0, [ + #-5670]
.L_ex9_361: add_slli	$r1, $r3, $r1, #2
.L_ex9_362: jal . + 0x3ac8
.L_ex9_363: jal . + 0x15582
.L_ex9_364: shi.gp	$r0, [ + #-54758]
.L_ex9_365: lwi.gp	$r0, [ + #0x774]
.L_ex9_366: lhi.gp	$r1, [ + #-6362]
.L_ex9_367: swi.gp	$r0, [ + #-5084]
.L_ex9_368: movi	$r0, #0x26238
.L_ex9_369: jal . + 0x8596
.L_ex9_370: lbi.gp	$r0, [ + #-4791]
.L_ex9_371: jal . + 0x3e3c
.L_ex9_372: jal . + 0x12388
.L_ex9_373: maddr32	$r16, $r7, $r2
.L_ex9_374: movi	$r0, #0x37a28
.L_ex9_375: swi.gp	$r0, [ + #-5072]
.L_ex9_376: lhi	$r0, [$sp + #4]
.L_ex9_377: movi	$r0, #0x3f7ce
.L_ex9_378: sbi.gp	$r0, [ + #-4203]
.L_ex9_379: movi	$r1, #0x21680
.L_ex9_380: jal . + 0x862e
.L_ex9_381: jal . + 0xed04
.L_ex9_382: lbi.gp	$r0, [ + #-4786]
.L_ex9_383: btst	$r0, $r0, #3
.L_ex9_384: lwi.gp	$r1, [ + #-4636]
.L_ex9_385: jal . + 0xd2fc
.L_ex9_386: movi	$r1, #0x34c10
.L_ex9_387: lbi.gp	$r0, [ + #-5653]
.L_ex9_388: jal . + 0x432c
.L_ex9_389: movi	$r4, #0x64
.L_ex9_390: movi	$r2, #0x64
.L_ex9_391: swi.gp	$r0, [ + #0xe6c]
.L_ex9_392: jal . + 0xffb6
.L_ex9_393: jal . + 0x9230
.L_ex9_394: slli	$r2, $r2, #0xf
.L_ex9_395: lwi	$r1, [$r3 + #0x50]
.L_ex9_396: sbi	$r1, [$r0 + #0x3c]
.L_ex9_397: bitci	$r1, $r1, #2
.L_ex9_398: jal . + 0x93d6
.L_ex9_399: shi	$r0, [$r6 + #0x12]
.L_ex9_400: movi	$r1, #0x30fa0
.L_ex9_401: movi	$r8, #0x55
.L_ex9_402: lbi.gp	$r0, [ + #-5587]
.L_ex9_403: sbi.gp	$r0, [ + #-5693]
.L_ex9_404: lwi.gp	$r1, [ + #-4704]
.L_ex9_405: sbi	$r9, [$r0 + #0]
.L_ex9_406: lbi.gp	$r5, [ + #0x130d]
.L_ex9_407: lwi.gp	$r0, [ + #-4060]
.L_ex9_408: sbi	$r1, [$r0 + #0x52]
.L_ex9_409: jal . + 0x949e
.L_ex9_410: addi	$r0, $r9, #1
.L_ex9_411: divsr	$r0, $r0, $r2, $r0
.L_ex9_412: or_srli	$r5, $r5, $r4, #0x1f
.L_ex9_413: movi	$r7, #0x20000
.L_ex9_414: movi	$r0, #0x3f41c
.L_ex9_415: jal . + 0xe26c
.L_ex9_416: sbi	$r0, [$r6 + #0x7d]
.L_ex9_417: add_slli	$r15, $r9, $r15, #1
.L_ex9_418: sbi.gp	$r0, [ + #-4745]
.L_ex9_419: shi	$r1, [$r0 + #0x20]
.L_ex9_420: movi	$r0, #0x3f00e
.L_ex9_421: sbi.gp	$r0, [ + #-4786]
.L_ex9_422: jal . + 0x899a
.L_ex9_423: lh	$r0, [$r15 + ($r0 << #1)]
.L_ex9_424: jal . + 0xffac
.L_ex9_425: jal . + 0xcee4
.L_ex9_426: sbi.gp	$r0, [ + #-55474]
.L_ex9_427: movi	$r0, #0x96
.L_ex9_428: sbi.gp	$r0, [ + #-4790]
.L_ex9_429: jal . + 0xecd2
.L_ex9_430: movi	$r0, #0x38d41
.L_ex9_431: ori	$r0, $r0, #8
.L_ex9_432: lh	$r0, [$r0 + ($r5 << #1)]
.L_ex9_433: slti	$r15, $r0, #0x33
.L_ex9_434: lbi.gp	$r0, [ + #-4756]
.L_ex9_435: lwi.gp	$r1, [ + #0x778]
.L_ex9_436: sbi.gp	$r0, [ + #-1691]
.L_ex9_437: lmw.bi $r16, [$r2], $r19, #0x0
.L_ex9_438: movi	$r1, #0xa5
.L_ex9_439: movi	$r1, #0x50
.L_ex9_440: jal . + 0x486a
.L_ex9_441: jal . + 0x10a68
.L_ex9_442: jal . + 0x11ca4
.L_ex9_443: lbi.gp	$r1, [ + #-6327]
.L_ex9_444: swi.gp	$r0, [ + #-5204]
.L_ex9_445: movi	$r1, #0x2c730
.L_ex9_446: lbi.gp	$r0, [ + #-4784]
.L_ex9_447: movi	$r0, #0x3f7c8
.L_ex9_448: lbi.gp	$r0, [ + #-4483]
.L_ex9_449: lw	$r1, [$r1 + ($r11 << #2)]
.L_ex9_450: sbi.gp	$r0, [ + #-5607]
.L_ex9_451: lwi.gp	$r0, [ + #-5072]
.L_ex9_452: sbi	$r0, [$r9 + #0x46]
.L_ex9_453: movi	$r0, #0x2fe60
.L_ex9_454: jal . + 0xed40
.L_ex9_455: abs	$r15, $r0
.L_ex9_456: add_slli	$r0, $r3, $r0, #2
.L_ex9_457: lhsi	$r5, [$r1 + #0]
.L_ex9_458: lwi.gp	$r1, [ + #0xe6c]
.L_ex9_459: sltsi	$r15, $r2, #0x33
.L_ex9_460: jal . + 0x88c2
.L_ex9_461: lbi	$r4, [$r0 + #0x34]
.L_ex9_462: sbi.gp	$r0, [ + #-4911]
.L_ex9_463: addi	$r0, $r0, #0x40
.L_ex9_464: sbi.gp	$r0, [ + #-5669]
.L_ex9_465: andi	$r1, $r0, #3
.L_ex9_466: lwi.gp	$r0, [ + #-384]
.L_ex9_467: lb	$r1, [$r15 + ($r1 << #0)]
.L_ex9_468: swi.gp	$r0, [ + #-5420]
.L_ex9_469: lbi	$r3, [$r0 + #0xe]
.L_ex9_470: divsr	$r1, $r1, $r1, $r11
.L_ex9_471: sbi	$r1, [$r0 + #8]
.L_ex9_472: jal . + 0x87de
.L_ex9_473: sbi	$r0, [$r6 + #0x40]
.L_ex9_474: swi	$r10, [$sp + #4]
.L_ex9_475: addi	$r0, $r0, #0x240
.L_ex9_476: lbi.gp	$r5, [ + #-4559]
.L_ex9_477: shi.gp	$r1, [ + #-5534]
.L_ex9_478: lwi	$r2, [$r7 + #0x28]
.L_ex9_479: lwi	$r2, [$r7 + #0x30]
.L_ex9_480: srai	$r3, $r2, #0x1f
.L_ex9_481: sbi.gp	$r0, [ + #-5600]
.L_ex9_482: movi	$r1, #0x2c434
.L_ex9_483: lhi	$r0, [$sp + #2]
.L_ex9_484: movi	$r1, #0x2c39a
.L_ex9_485: movi	$r1, #0x21810
.L_ex9_486: jal . + 0x950a
.L_ex9_487: movi	$r1, #0x2dba8
.L_ex9_488: swi.gp	$r1, [ + #-3776]
.L_ex9_489: jal . + 0x11966
.L_ex9_490: lwi.gp	$r0, [ + #-5336]
.L_ex9_491: lwi.gp	$r0, [ + #-4392]
.L_ex9_492: movi	$r2, #0xb6
.L_ex9_493: swi.gp	$r6, [ + #-5440]
.L_ex9_494: jal . + 0x12f9a
.L_ex9_495: slti	$r15, $r0, #0xa0
.L_ex9_496: lwi.gp	$r1, [ + #-444]
.L_ex9_497: jal . + 0x1934
.L_ex9_498: lbi.gp	$r1, [ + #-4629]
.L_ex9_499: movi	$r0, #0x2c4c4
.L_ex9_500: swi.gp	$r0, [ + #0xe70]
.L_ex9_501: addi	$r2, $sp, #6
.L_ex9_502: sh	$r1, [$r0 + ($r6 << #1)]
.L_ex9_503: lbi.gp	$r1, [ + #-4825]
.L_ex9_504: lwi.gp	$r3, [ + #-5388]
.L_ex9_505: shi.gp	$r0, [ + #-5548]
.L_ex9_506: jal . + 0x19ba
.L_ex9_507: add_slli	$r0, $r1, $r0, #8
.L_ex9_508: lhi.gp	$r0, [ + #-54758]
.L_ex9_509: jal . + 0xec90
.L_ex9_510: lbi.gp	$r0, [ + #-55774]
.L_ex9_511: maddr32	$r2, $r1, $r4
.size agent18_ex9_table, . - agent18_ex9_table

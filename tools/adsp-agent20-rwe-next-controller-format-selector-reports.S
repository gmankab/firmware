// SPDX-License-Identifier: GPL-3.0-only
// 18 Hexagon V66 format-specific diagnostic reports [0x1289c,0x129bc).
// The messages are source-authored addresses, not firmware instruction words.
.text
.globl adsp_format_diagnostics
adsp_format_diagnostics:
diagnostic_case_00:
{ call platform_format; immext(#0xf000fb80); r0=##0xf000fb9d }
{ jump common_handler }
diagnostic_case_01:
{ call platform_format; immext(#0xf000fb80); r0=##0xf000fbbd }
{ jump common_handler }
diagnostic_case_02:
{ call platform_format; immext(#0xf000fbc0); r0=##0xf000fbda }
{ jump common_handler }
diagnostic_case_03:
{ call platform_format; immext(#0xf000fbc0); r0=##0xf000fbf9 }
{ jump common_handler }
diagnostic_case_04:
{ call platform_format; immext(#0xf000fc00); r0=##0xf000fc1e }
{ jump common_handler }
diagnostic_case_05:
{ call platform_format; immext(#0xf000fc00); r0=##0xf000fc3d }
{ jump common_handler }
diagnostic_case_06:
{ call platform_format; immext(#0xf000fc40); r0=##0xf000fc4b }
{ jump common_handler }
diagnostic_case_07:
{ call platform_format; immext(#0xf000fc40); r0=##0xf000fc59 }
{ jump common_handler }
diagnostic_case_08:
{ call platform_format; immext(#0xf000fc40); r0=##0xf000fc6f }
{ jump common_handler }
diagnostic_case_09:
{ call platform_format; immext(#0xf000fc80); r0=##0xf000fc90 }
{ jump common_handler }
diagnostic_case_10:
{ call platform_format; immext(#0xf000fc80); r0=##0xf000fcbc }
{ jump common_handler }
diagnostic_case_11:
{ call platform_format; immext(#0xf000fcc0); r0=##0xf000fcdf }
{ jump common_handler }
diagnostic_case_12:
{ call platform_format; immext(#0xf000fc80); r0=##0xf000fc9f }
{ jump common_handler }
diagnostic_case_13:
{ call platform_format; immext(#0xf000fc80); r0=##0xf000fcae }
{ jump common_handler }
diagnostic_case_14:
{ call platform_format; immext(#0xf000fd40); r0=##0xf000fd45 }
{ jump common_handler }
diagnostic_case_15:
{ call platform_format; immext(#0xf000fcc0); r0=##0xf000fcf1 }
{ jump common_handler }
diagnostic_case_16:
{ call platform_format; immext(#0xf000fd00); r0=##0xf000fd03 }
{ jump common_handler }
diagnostic_case_17:
{ call platform_format; immext(#0xf000fd00); r0=##0xf000fd1b }
{ jump common_handler }
.set platform_format,adsp_format_diagnostics+(0x8be14438-0x8be1289c)
.set common_handler,adsp_format_diagnostics+(0x8be12860-0x8be1289c)

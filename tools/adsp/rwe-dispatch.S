// SPDX-License-Identifier: GPL-3.0-only
// ADSP secondary RWE dispatcher beginning at MBN file 0x608c, physical
// 0x8be0308c, after the controller state prologue. Source code continues
// after this deliberately bounded first subsection.
.text
.globl adsp_agent20_rwe_dispatch
adsp_agent20_rwe_dispatch:
{ call controller_dispatch }
{ r23:22 = combine(r1,r0) }
{ call controller_check_0 }
{ call controller_check_1 }
{ r1:0 = combine(r23,r22) }
{ p0 = cmp.eq(r0,#2); if (p0.new) jump:nt third_mode }
{ p0 = cmp.eq(r0,#1); if (p0.new) jump:t second_mode }
{ sp = ##-0xfffc220 }
{ call bootstrap_input }
{ immext(#0xffffe940); call warm_start }
{ immext(#0xffffe980); call cold_start }
{ immext(#0x12c00); call processor_start }
{ immext(#0x12440); call runtime_start }
.set controller_dispatch,adsp_agent20_rwe_dispatch+(0x8be2240c-0x8be0308c)
.set controller_check_0,adsp_agent20_rwe_dispatch+(0x8be035f4-0x8be0308c)
.set controller_check_1,adsp_agent20_rwe_dispatch+(0x8be035f8-0x8be0308c)
.set third_mode,adsp_agent20_rwe_dispatch+(0x8be03394-0x8be0308c)
.set second_mode,adsp_agent20_rwe_dispatch+(0x8be03148-0x8be0308c)
.set bootstrap_input,adsp_agent20_rwe_dispatch+(0x8be03c30-0x8be0308c)
.set warm_start,adsp_agent20_rwe_dispatch+(0x8be01a08-0x8be0308c)
.set cold_start,adsp_agent20_rwe_dispatch+(0x8be01a54-0x8be0308c)
.set processor_start,adsp_agent20_rwe_dispatch+(0x8be15d00-0x8be0308c)
.set runtime_start,adsp_agent20_rwe_dispatch+(0x8be15544-0x8be0308c)

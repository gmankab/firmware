// SPDX-License-Identifier: GPL-3.0-only
// Agent 13: validate an active WLAN record via callback, link state and
// a header halfword, then return a normalized Boolean/status indicator.
// File [0x42f18,0x42f74), VMA [0xb0002f18,0xb0002f74).
.text
.globl agent13_runtime_record_readiness
agent13_runtime_record_readiness:
{ call 0x7fc90; r16 = r0; memd(r29+#-16) = r17:16; allocframe(#8) }
{ p0 = cmp.eq(r0,#3); if (!p0.new) r0 = add(r16,#0); if (!p0.new) jump:t codec_raw }
{ r0 = memw(r16+#448); if (cmp.eq(r0.new,#0)) jump:nt missing_state }
{ r2 = memw(r16+#20) }
{ r2 = memuh(r2+#68); if (cmp.eq(r2.new,#0)) jump:nt flag_one }
{ call -0xf90; r1 = asl(r2,#10) }
{ jump trim_12 }
codec_raw:
{ call 0x7f990 }
trim_12:
{ r0 = extractu(r0,#12,#0); jump finish }
missing_state:
{ call 0x7f9ac; r0 = r16 }
{ jump finish }
flag_one:
{ r0 = #1 }
finish:
{ p0 = cmp.eq(r0,#0); if (p0.new) r0 = #1; r17:16 = memd(r29+#0) }
{ r0 = zxth(r0); dealloc_return }

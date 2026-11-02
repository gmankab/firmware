// Conditional controller-state query [0x39568,0x39588): preserve original
// caller descriptor while only probing the platform when the status is set.
.text
.globl adsp_late_conditional_query
adsp_late_conditional_query:
{ p0 = cmp.eq(r2,#0); r16 = r0; memd(r29+#-16) = r17:16; allocframe(#8) }
{ immext(#0x108440); if (!p0) call platform_check; if (!p0) r0 = add(r16,#0) }
{ r0 = r16; r17:16 = memd(r29+#0); dealloc_return }
.set platform_check, adsp_late_conditional_query+0x108e68

// Reconstructed 20-byte Hexagon LL/SC register updater at file 0x394a0.
.text
.globl adsp_late_atomic_add
adsp_late_atomic_add:
retry:
{ r4 = memw_locked(r2) }
{ r4 = add(r4,r3) }
{ memw_locked(r2,p0) = r4 }
{ if (!p0) jump:nt retry }
{ jumpr lr }

// SPDX-License-Identifier: GPL-3.0-only
// Source-level reconstruction of Hexagon ADSP timer and controller checks.
// First RX PT_LOAD offset 0x3000, this code at FILE OFFSET 0x4218,
// RX-relative 0x1218, physical address 0x8be01218.
//
// The first block is a single 3-instruction packet; the control loop
// spins on a controller word until the elapsed time meets the threshold.
// Interrupt vector MMIO setup, entered before the timer and controller
// helper routines below. The later branch skips the timer poll helper.
.text
.globl adsp_interrupt_vector_init
adsp_interrupt_vector_init:
{ r0 = ##0x173a0040 }
{ memw(r0+#0) = ##7 }
{ r0 = ##0x173a0044 }
{ memw(r0+#0) = ##7 }
{ r0 = ##0x173a0048 }
{ memw(r0+#0) = ##7 }
{ barrier }
{ jump adsp_controller_mode }

.text
.p2align 2
.globl adsp_wait_for_tick
adsp_wait_for_tick:
{
  r8 = r1
  r9 = r2
  r10 = r3
}
{ r2 = r0 }
load_global_timer:
{ r0 = ##0x173a2000 }
{ r1 = memw(r0+#0) }
poll_timer:
{ r3 = memw(r0+#0) }
{ r3 = sub(r3,r1) }
{
  p0 = cmp.gtu(r3,r2)
  if (!p0.new) jump:t poll_timer
}
{
  r1 = r8
  r2 = r9
  r3 = r10
}
{ jumpr r13 }

// Controller mode selector, entered independently after the wait routine.
adsp_controller_mode:
{ r0 = ##0x01fc8000 }
{ r1 = memw(r0+#0) }
{ r2 = and(r1,##0x100) }
{
  p0 = cmp.eq(r2,##0x100)
  if (p0.new) jump:t primary_mode
}
{ r2 = and(r1,##0x200) }
{
  p0 = cmp.eq(r2,##0x200)
  if (p0.new) jump:t secondary_mode
}
{ jump fallback_mode }

// These targets are not included in this fragment. Keep their exact
// displacements address-derived, without copying OEM code bytes.
// Three controller setup cases. The branches select the same nine
// MMIO configuration registers but use class-specific mode settings.
// The compiler emits the two-word Hexagon immext packets from ##const,
// rather than importing the OEM instruction sequence.
primary_mode:
{ r14 = ##0x32 }
{ r15 = ##0x44 }
{ r16 = ##1 }
{ r0 = ##0x17300218 }
{ memw(r0+#0) = ##0x20485699 }
{ r0 = ##0x1730021c }
{ memw(r0+#0) = ##0x2267 }
{ r0 = ##0x17300220 }
{ memw(r0+#0) = ##0x24 }
{ r0 = ##0x17300224 }
{ memw(r0+#0) = ##0x0 }
{ r0 = ##0x17300228 }
{ memw(r0+#0) = ##0x2 }
{ r0 = ##0x1730022c }
{ memw(r0+#0) = ##0x20 }
{ r0 = ##0x1730020c }
{ memw(r0+#0) = ##0x0 }
{ r0 = ##0x17300210 }
{ memw(r0+#0) = ##0x805 }
{ r0 = ##0x17300214 }
{ memw(r0+#0) = ##0xd0 }
{ jump controller_done }

secondary_mode:
{ r14 = ##0x3d }
{ r15 = ##0x44 }
{ r16 = ##1 }
{ r0 = ##0x17300218 }
{ memw(r0+#0) = ##0x20485699 }
{ r0 = ##0x1730021c }
{ memw(r0+#0) = ##0x2267 }
{ r0 = ##0x17300220 }
{ memw(r0+#0) = ##0x24 }
{ r0 = ##0x17300224 }
{ memw(r0+#0) = ##0x0 }
{ r0 = ##0x17300228 }
{ memw(r0+#0) = ##0x0 }
{ r0 = ##0x1730022c }
{ memw(r0+#0) = ##0x20 }
{ r0 = ##0x1730020c }
{ memw(r0+#0) = ##0x0 }
{ r0 = ##0x17300210 }
{ memw(r0+#0) = ##0x805 }
{ r0 = ##0x17300214 }
{ memw(r0+#0) = ##0xd0 }
{ jump controller_done }

fallback_mode:
{ r14 = ##0x3d }
{ r15 = ##0x44 }
{ r16 = ##1 }
{ r0 = ##0x17300218 }
{ memw(r0+#0) = ##0x20485699 }
{ r0 = ##0x1730021c }
{ memw(r0+#0) = ##0x2267 }
{ r0 = ##0x17300220 }
{ memw(r0+#0) = ##0x24 }
{ r0 = ##0x17300224 }
{ memw(r0+#0) = ##0x0 }
{ r0 = ##0x17300228 }
{ memw(r0+#0) = ##0x0 }
{ r0 = ##0x1730022c }
{ memw(r0+#0) = ##0x20 }
{ r0 = ##0x1730020c }
{ memw(r0+#0) = ##0x0 }
{ r0 = ##0x17300210 }
{ memw(r0+#0) = ##0x805 }
{ r0 = ##0x17300214 }
{ memw(r0+#0) = ##0xd0 }

controller_done:


// Device MMIO configuration, mask updates, cache invalidation and
// post-initialization polling. Continued from controller_done above.
config_device:
{ barrier }
{
    r0 = ##0x17300204
    r1 = r14
}
{ memw(r0+#0)=r1 }
{ barrier }
{ r0 = ##0x17300208 }
{ r1 = r15 }
{ memw(r0+#0)=r1 }
{ barrier }
{ r0 = ##0x17300200 }
{
  r7 = r0
  r5.h = #0
}
{ r5.l = #1 }
{
  r5 = sub(#-1,r5)
  r4 = memw(r0+#0)
}
{ r4 = and(r4,r5) }
{ memw(r0+#0) = r4 }
{ barrier }
{ r0 = ##0x17300238 }
{
  r7 = r0
  r5.h = #0
}
{ r5.l = #7 }
{
  r5 = sub(#-1,r5)
  r4 = memw(r0+#0)
}
{ r4 = and(r4,r5) }
{ memw(r0+#0) = r4 }
{ barrier }
{ r0 = ##0x17300200 }
{
  r7 = r0
  r5.h = #0
}
{
  r5.l = #4
  dcinva(r7)
}
{
  r6 = sub(#-1,r5)
  r4 = memw(r0+#0)
}
{ r4 = and(r4,r6) }
{ r4 = or(r4,r5) }
{ memw(r0+#0) = r4 }
{ barrier }
{ r0 = ##0x17300238 }
{ r1 = memw(r0+#0) }
{ r2 = ##1 }
{ r1 = or(r1,r2) }
{ memw(r0+#0) = r1 }
{ barrier }

{ r4 = ##0x17300200 }
{ r2 = ##-0x80000000 }
poll_other_controller:
{ r0 = ##0xc0 }
{ r13 = add(PC,#8) }
{ jump counter_helper }
{ r1 = memw(r4+#0) }
{ r3 = and(r1,r2) }
{
  p0 = cmp.eq(r2,r3)
  if (!p0.new) jump:t poll_other_controller
}
.set counter_helper, config_device - 0x270
// Reenable codec MMIO control after controller-specific register setup.
{ r0 = ##0x1730020c }
{
  r7 = r0
  r5.h = #0
}
{
  r5.l = #1
  dcinva(r7)
}
{
  r6 = sub(#-1,r5)
  r4 = memw(r0+#0)
}
{ r4 = and(r4,r6) }
{ r4 = or(r4,r5) }
{ memw(r0+#0) = r4 }
{ barrier }

{ r0 = ##0x17300200 }
{
  r7 = r0
  r5.h = #0
}
{
  r5.l = #1
  dcinva(r7)
}
{
  r6 = sub(#-1,r5)
  r4 = memw(r0+#0)
}
{ r4 = and(r4,r6) }
{ r4 = or(r4,r5) }
{ memw(r0+#0) = r4 }
{ barrier }

// Publish IRQ-class flags to the controller's masked status register.
{ r0 = ##0x1730002c }
{ r1 = memw(r0+#0) }
{
  r2 = ##0x700
  r3 = ##0x1f
}
{ r2 = or(r2,r3) }
{ r2 = sub(#-1,r2) }
{ r1 = and(r1,r2) }
{
  r3 = asl(r16,#0)
  r2 = ##0x200
}
{ r2 = or(r2,r3) }
{ r1 = or(r1,r2) }
{ memw(r0+#0) = r1 }
{ barrier }

{ r0 = ##0x17300028 }
{ r1 = memw(r0+#0) }
{ r2 = ##1 }
{ r1 = or(r1,r2) }
{ memw(r0+#0) = r1 }
{ barrier }
poll_legacy_controller:
{ r1 = memw(r0+#0) }
{ r3 = and(r1,r2) }
{
  p0 = cmp.eq(r3,r2)
  if (p0.new) jump:t poll_legacy_controller
}
// Disabling the earlier exception vectors allows startup to proceed.
{ r0 = ##0x173a0040 }
{ memw(r0+#0) = ##0 }
// Clear remaining exception slots, finish the MMIO setup, and expose
// minimal startup helpers. Later SYSCFG/exception handlers are outside
// this source fragment.
{ r0 = ##0x173a0044 }
{ memw(r0+#0) = ##0 }
{ r0 = ##0x173a0048 }
{ memw(r0+#0) = ##0 }
{ barrier }
{ jumpr lr }
{ nop }
{ nop }
{ sp = add(pc,##0x20458) }
{ jumpr lr }
{ jumpr lr }

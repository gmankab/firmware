// SPDX-License-Identifier: GPL-3.0-only
// Select the normal or reduced-verbosity path for an accepted record,
// publishing its source-record validation diagnostic before plane sizing.
.text
.globl modem_late_aux_session_builder_validity
modem_late_aux_session_builder_validity:
.set reduced_log,modem_late_aux_session_builder_validity+0x24
.set emit_plane_headers,modem_late_aux_session_builder_validity+0x34
.set buffer_factory,modem_late_aux_session_builder_validity-0x5cc
.set alternate_log,modem_late_aux_session_builder_validity+0x2d1c
{
  p0 = r2
  if (!p0.new) r3 = ##507578319
  if (!p0.new) jump:t reduced_log
}
{
  call buffer_factory
  r2 = #637
  r3 = ##507578319
}
{jump emit_plane_headers}
reduced_log:
{
  r1:0 = combine(##507576841,#2)
  r2 = #637
}
{call alternate_log}

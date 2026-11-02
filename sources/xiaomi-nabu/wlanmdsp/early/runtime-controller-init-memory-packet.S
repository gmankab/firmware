// SPDX-License-Identifier: GPL-3.0-only
// Dependent byte-load and new-value store in the same original four-word packet.
.text
.globl agent13_controller_compound_memory
agent13_controller_compound_memory:
{
  r2 = memub(r25+#4)
  memw(r3+#972) = r2.new
}:mem_noshuf

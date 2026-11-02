// Independent Hexagon assembly: three controller-diagnostic packets.
// First call uses the current registers, concurrently assembling the
// next error message and severity code for the second diagnostic helper.
.text
.space 0x890
controller_bad_plane:
{ call #0xf831c
  r0=##0xc0226180
  r18=#212 }
{ call #0x5024
  r1:0=combine(r18,##0xc0226175) }
controller_bad_bank:
{ call #0xf8300
  r0=##0xc02261a6
  r18=#284 }
{ call #0x5008
  r1:0=combine(r18,##0xc0226175) }
controller_bad_register:
{ call #0xf82e4
  r0=##0xc02261a6
  r18=#307 }
{ call #0x4fec
  r1:0=combine(r18,##0xc0226175) }

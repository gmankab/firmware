# Reverse-engineering pass — 2026-10-22

**This is research evidence, not a working reconstructed firmware implementation.**

REA 6.7.0 with Binwalk 3.1.0 inspected all nine APK payloads; its signature
hits and extraction suggestions are recorded under `evidence/rea/`.
Ghidra 12.1.4 native headless analysis separately decompiled Qualcomm ELF32
ARM and Hexagon functions to C-like pseudocode. The corresponding OEM-derived
pseudocode is in a *separately packaged* research archive, NOT this GPLv3 Git
repository. The script used to generate it IS included under GPLv3.

| Image | Processor/container | Candidate functions found (latest pass) | C pseudocode emissions | Completeness |
|---|---|---:|---:|---|
| a640_zap.mbn | Hexagon ELF | 1 | 1 syntax output, semantically broken | Entry ends in `halt_baddata()`; invalid data |
| venus.mbn | ARM ELF | 1,593 | 1,588 | 5 decompiler failures |
| wlanmdsp.mbn | Hexagon ELF | 6,113 on pass 2 | 5,930 successful over two passes (3,163 + 2,767) | Two failures total; 181 not attempted versus second pass count |
| modem.mbn | Hexagon ELF | 5,348 | 5,346 | 2 decompiler failures |
| cdsp.mbn | Hexagon ELF | 10,786 | **10,777 over two passes** (5,295 + 5,482) | All 10,786 attempted, 9 failures; address sets disjoint |
| adsp.mbn | Hexagon ELF | 23,212 in extended run | 5,994 / 5,998 attempted | First run timed out before finding any; extended 340-second autoanalysis + bounded decompilation recovered a subset; 17,214 candidates unattempted |
| ADSP bootstrap analysis view | Hexagon ELF (temporary modified copy) | 29 | 25 / 29 attempted | Four decompiler failures, including entry; this is **not** bootable firmware |
| cs35l41-dsp1-spk-prot.wmfw | Cirrus HALO DSP | 11 regions | n/a | Container parsed, ISA decompiler unavailable |
| novatek_nt36523_fw.bin | Novatek raw | header=0x200, 31 tentative descriptors | n/a | Controller ISA unconfirmed, no decompilation |
| modemuw.jsn | Text config | n/a | n/a | 713 bytes rebuilt from typed Rust values, exact SHA256 |

**Caveat:** Discovered functions, attempted functions, and successfully emitted
pseudocode are different measurements. Ghidra's heuristics may misidentify
functions and the output is not compilable original C source, even where
reported successful. No ELF `.mbn` has been rebuilt, and no firmware has
passed silicon hardware tests. Certificate chains are present in the six
Qualcomm authentication metadata segments, but the private signing keys and
hardware trust policy have not been established.

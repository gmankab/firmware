# Agent 07 — `wlanmdsp.mbn` executable reconstruction

## Provenance / baseline limitation

Intended integration base is checkpoint 50, `b7ed4b587633b9b475a40f5a3ace5cc90db620ff`, branch `agent/07-wlanmdsp`. **That Git bundle/commit is not available in this conversation's mounted container**. The available `recompile-firmware.bundle` ends at `69ebf0d` and contains the compatible earlier ancestor `1ef37874e2993321700f4e934639fe447a26056d`. Work is rooted at that available ancestor and restricted to new, WLAN-specific source, tests, tools and this agent progress note. These commits are therefore intended for `git cherry-pick` on the actual checkpoint-50 repository; **do not claim they have the requested direct parent**.

The exact private WLAN reference is not checked into Git. It was extracted from the user's `nabu-downloads-ubuntu24.04-amd64.tar.zst.part{001,002,003.1,003.2,004,005.1,005.2,006}` archive in that explicit order and verified against `reference/xiaomi-nabu/firmware.sha256`:

- WLAN revision SHA-256: `87c2c7bb1fe97eaf007a947832feba0898644b4d6626ac8ef49d8b94dd82604b`.
- Little-endian ELF32, e_machine=164 (Qualcomm Hexagon DSP6), `e_flags=0x66`, entry address `0xb0000000`.
- Executable `PT_LOAD`: file offset `0x40000`, vaddr/paddr `0xb0000000`, `p_filesz=p_memsz=0x313d38`, `PF_R|PF_X`.
- Other mapped data starts at file offset `0x380000` (RW, much larger runtime `p_memsz`), whereas the NOTE and Qualcomm hash/auth metadata are **not program text**.

## Independent encoding and ISA limits

The supplied LLVM 6.0.1 `llvm-mc` (Hexagon target) can independently **decode** the opening instructions with v60, v62 and v65 selection; **assembling** this source requires `hexagonv62` or `hexagonv65` because LLVM6 rejects `framelimit` for v60. The file flag `0x66` indicates a newer Hexagon version, which this LLVM 6 assembler cannot target (`hexagonv66` rejected). However, the reconstructed fragment produces identical bytes across the supported assembler CPU choices (v62 and v65) **and** against the exact original firmware. That verifies the fragment, not the full instruction-set version.

`wlan-startup.S` is independently authored packetized Hexagon assembly, documenting:

- `[0xb0000000, 0xb00000a8)`: QuRT/TLS and global-pointer initialization, BSS cleanup calls, subsystem callback setup and idle loop.
- `[0xb00000a8, 0xb00000f8)`: zero-initialization routine, handling initial unaligned bytes, 8-byte aligned stores and trailing bytes. Hexagon packet grouping matters: each backward jump executes together with its associated memory store.
- `[0xb00000f8, 0xb0000100)`: extended relative branch to the existing runtime heap stub at vaddr `0xb0001140`; intentionally forced by the assembler's `##` immediate notation.
- `[0xb0000100, 0xb0000154)`: C-runtime constructor/finalizer initialization, including a duplex `r3/r1` constant packet. Source order is required for byte-identical VLIW encoding.

Build without OEM reference using:

```sh
python3 tools/build-wlanmdsp-startup.py /tmp/wlan-startup.bin \
  --llvm-mc /path/to/llvm6/bin/llvm-mc
```

Optionally supply `--reference /private/wlanmdsp.mbn` to compare the independently generated output to the *full hash-verified reference* at file offset `0x40000`. This rejects wrong revisions and the first differing byte with both file offset and vaddr. `--manifest /tmp/wlan-startup.json` records generated digest and coverage. The builder refuses unresolved text relocations, invalid ELF32, and raw embedded opcode directives (`.incbin`, `.word`, etc.).

### Checkpoint A: independently generated startup

- File range: `[0x40000,0x40154)`; virtual address `[0xb0000000,0xb0000154)`.
- Total: **340 bytes of independently written Hexagon assembly**, no original opcode words included.
- Exact output SHA-256: `11a77a2cd5e9a41e87f622bd559c19a4c2c9a1036c3071ed63b52e7fb9a688c1`.
- Comparison: **340/340 identical bytes; first differing offset: none**.
- Coverage excludes padding and every later program function; this is **not signed/bootable firmware**.

## Remaining work

First examine the function after `0xb0000154` without mistaking padding for code: the following file bytes have large repeated `0x00c0007f` words. Program text is far larger than the initial verified fragment, and reconstructing 3.2 MiB of original functional source remains open. The signature/hash program header and the external DSP loader are also not independently reconstructed. Full image emission must be refused until all executable bytes, relocation/linker data and external authentication are accounted for.

# Additional tools / data needed for a genuine full replacement

**Already available:** user-provided firmware APK, REA 6.7.0, Ghidra 12.1.4
with Hexagon and ARM processor modules, Binwalk 3.1.0, Rust 1.99.1 and native
build tools. These suffice for the present static analysis and research bundle.

**Useful or necessary later, depending on the task:**

* An instruction-level **Cirrus HALO** disassembler/decompiler and architecture
  documentation for the CS35L41 speaker-protection `WMFW` program region.
  Ghidra's Qualcomm Hexagon language is not a HALO DSP language.
* A confirmed **Novatek NT36523 MCU instruction set**, SRAM map, boot address,
  and firmware update format. Our 31 descriptors are only a plausible
  NT36xxx-family interpretation.
* A suitably configured **Hexagon Rust code-generation toolchain** (not
  shipped by this Rust 1.99.1 release's target list), plus HALO and Novatek
  platform toolchains, linker scripts, memory layouts and peripheral APIs.
  Building `armv7` Rust isn't enough for Hexagon code.
* An actual Xiaomi Nabu running the exact target firmware/kernel (or
  trustworthy hardware logs) to test whether PAS image authentication accepts
  rebuilt or unsigned images. We intentionally do not disable security checks
  or attempt signing; bootloader unlocking alone doesn't answer this question.
* Detailed behavior specifications or rights-cleared original source for the
  millions of machine-code instructions and data objects. Decompiled C-like
  pseudocode is a research aid, not recovered buildable original source.

No amount of compiler optimization can reproduce arbitrary cryptographic
signatures without the appropriate keys and identical signed inputs. A
byte-identical signed firmware image is incompatible with a modified payload
unless the cryptographic checks, key material or signed scope permit it. We
have not established any such permission or exception for Nabu.

**Pocketblue / Fedora legal review:** The APK metadata says `proprietary`.
That alone does not demonstrate Fedora-compatible firmware redistribution
terms. GPLv3 on these newly authored research tools does not grant rights to
redistribute the original binaries or automatically GPL-license pseudocode.
See Fedora firmware licensing guidance for separate binary-firmware criteria.

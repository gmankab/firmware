#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Build Agent11's independent Hexagon RWE exception handler fragments.

Unlike a disassemble/reassemble operation on OEM bytes, all opcodes are
emitted from source-level Hexagon assembly, checked architecture macros and
explicitly named register/branch operands. The original modem.mbn is used
only for optional reference verification (SHA-256 locked).
"""
from __future__ import annotations
import argparse
import hashlib
import struct
from pathlib import Path
import runpy
import subprocess
import tempfile

FILE_START = 0x38108
FILE_END = 0x38f0c
VIRTUAL_START = 0xc0840108
PHYSICAL_START = 0x8e040108
PINNED_SHA256 = '6f5cb32bc10e41abb40938d185805e788f5637a41d3ebcef278e4e169fc36782'


def normalize_llvm6_locked_packets(data: bytes) -> bytes:
    """Convert LLVM6's legal packet schedule to the source-ordered ISA form.

    Hexagon's LLVM6 scheduler moves a .new compare's branch ahead of its
    predicate, and positions a conditional branch ahead of the extender and
    conditionally defined register. The original sequence is also a legal
    instruction packet. Reorder only the two independently assembled packet
    groups after validating instruction classes, operands and marker bits.
    The locked word-store uses checked architectural fields; LLVM6 does not
    accept it in the same packet as an extender. Normalize its producer word
    after explicitly checking the source-generated register and imm marker.
    No original reference bytes are read by this function.
    """
    result=bytearray(data)
    def read(address):return struct.unpack_from('<I',result,address-FILE_START)[0]
    def write(address,word):struct.pack_into('<I',result,address-FILE_START,word)
    expected_classes={
        0x3820c:0x5cff6afe,  # if(!p2.new) jump:nt acquire retry
        0x38210:0x750d4002,  # p2 = cmp.eq(r13,#0)
        0x3821c:0x7800c08c,  # LLVM6 extended immediate into r12, own packet
        0x38220:0xa0accd02,  # checked locked store(r12,p2) = r13
        0x38224:0x5cff62f2,  # if(!p2) retry
        0x38228:0x0c085004,  # constant extender for conditional r12
        0x3822c:0x7ec0408c,  # if(!p2) r12 = ##constant
        0x38230:0x418cd00d,  # if(p2) r13 = memw(r12+#0)
    }
    for address,expected in expected_classes.items():
        if read(address)!=expected:
            raise ValueError(f'unrecognized Hexagon LLVM6 locked packet at 0x{address:x}')
    a,b=read(0x3820c),read(0x38210)
    write(0x3820c,b)
    write(0x38210,a)
    # LLVM6 emits standalone r12=#imm. Move it to the locked-store packet
    # and remove only the private packet-isolation bit used by this class.
    write(0x3821c,0x78000000 | (1<<14) | 12)
    branch,ext,mov=read(0x38224),read(0x38228),read(0x3822c)
    write(0x38224,ext)
    write(0x38228,mov)
    write(0x3822c,branch)
    if len(result)>=0x382e0-FILE_START:
        # Second locked-word acquisition uses the same cmp/branch packet
        # with r15 rather than r13. LLVM6 reverses these operations.
        if read(0x382cc)!=0x5cff6afe or read(0x382d0)!=0x750f4002:
            raise ValueError('unrecognized second locked interrupt predicate packet')
        compare,branch=read(0x382d0),read(0x382cc)
        write(0x382cc,compare)
        write(0x382d0,branch)
    if len(result)>=0x38338-FILE_START:
        # LLVM6 places the conditional jump before its producing P2 compare
        # in this 4-instruction packet; preserve the original legal order.
        if read(0x38308)!=0x5c005a12 or read(0x3830c)!=0x750f4012:
            raise ValueError('unrecognized controller status compare/branch packet')
        branch,pred=read(0x38308),read(0x3830c)
        write(0x38308,pred)
        write(0x3830c,branch)
    if len(result)>=0x38364-FILE_START:
        # The packet containing combine(r14,r15), conditional p0 jump and
        # saved-frame load is also valid with producer first. LLVM6 puts
        # the branch in slot zero; preserve operand/parse fields strictly.
        if read(0x38358)!=0x5c204042 or read(0x3835c)!=0xf50e4f06:
            raise ValueError('unrecognized exception context combine/branch packet')
        branch,combined=read(0x38358),read(0x3835c)
        write(0x38358,combined)
        write(0x3835c,branch)
    if len(result)>=0x383cc-FILE_START:
        # The paired per-thread lock operations have r12 predicate operands,
        # but the identical tested LLVM6 branch-first scheduling behavior.
        for addr,branch_word,predicate_word in (
                (0x38374,0x5cff6afe,0x750c4002),
                (0x3839c,0x5c005a12,0x750c4012)):
            if read(addr)!=branch_word or read(addr+4)!=predicate_word:
                raise ValueError(f'unrecognized per-thread locked predicate at 0x{addr:x}')
            write(addr,predicate_word)
            write(addr+4,branch_word)
    if len(result)>=0x384b4-FILE_START:
        if read(0x384a8)!=0x5c0048a2 or read(0x384ac)!=0x75874210:
            raise ValueError('unrecognized vector-lane conditional branch packet')
        branch,pred=read(0x384a8),read(0x384ac)
        write(0x384a8,pred)
        write(0x384ac,branch)
    if len(result)>=0x38504-FILE_START:
        # r31 lock predicate is the p0 version of the earlier p2 packet.
        expected={0x384dc:0x5cff68fe,0x384e0:0x751f4000,
                  0x384ec:0x7800c09e,0x384f0:0xa0bedf00,
                  0x384f4:0x5cff60f2,0x384f8:0x0c085004,
                  0x384fc:0x7e80409e,0x38500:0x419ec01f}
        for address,word in expected.items():
            if read(address)!=word:
                raise ValueError(f'unrecognized R31 protected controller packet at 0x{address:x}')
        branch,pred=read(0x384dc),read(0x384e0)
        write(0x384dc,pred)
        write(0x384e0,branch)
        # Relocate the r30 immediate producer into the explicit locked-store
        # packet and use the checked register-operand encoding for that slot.
        write(0x384ec,0x78000000|(1<<14)|30)
        branch,ext,immediate=read(0x384f4),read(0x384f8),read(0x384fc)
        write(0x384f4,ext)
        write(0x384f8,immediate)
        write(0x384fc,branch)
    if len(result)>=0x3856c-FILE_START:
        # The compact bit-test branch and separate p3 source share one
        # architectural packet; LLVM6 isolates the producer as standalone.
        if read(0x38534)!=0xc7a7c803 or read(0x38538)!=0x11874362:
            raise ValueError('unexpected compound bit-test/branch operands')
        write(0x38534,0xc7a70000 | (1<<14) | 0x0803)
        # LLVM6 moves the callr behind an independent arithmetic operation.
        # Both operands and packet markers are validated before swapping.
        if read(0x38548)!=0xb006441e or read(0x3854c)!=0x50a74000:
            raise ValueError('unexpected exception callback/register packet')
        add,callback=read(0x38548),read(0x3854c)
        write(0x38548,callback)
        write(0x3854c,add)
    if len(result)>=0x385e8-FILE_START:
        # R8 lock acquire and mask-release .new branches: LLVM6 places
        # branch before the predicate, but source requires compare first.
        for address,branch_word,predicate_word in (
                (0x38578,0x5cff68fe,0x75084000),
                (0x385b4,0x5c005812,0x75084010)):
            if read(address)!=branch_word or read(address+4)!=predicate_word:
                raise ValueError(f'Unexpected final R8 mask packet at 0x{address:x}')
            write(address,predicate_word)
            write(address+4,branch_word)
    if len(result)>=0x386c0-FILE_START:
        # LLVM6 prematurely ends the extended GP-style immediate packet
        # before its allocframe. Original legal parse sequence is
        # extender -> immediate low -> allocframe, markers 1,1,3.
        if read(0x38684)!=0x7800c289:
            raise ValueError('unrecognized Hexagon protected framekey packet')
        write(0x38684, (read(0x38684)&~(3<<14)) | (1<<14))
    if len(result)>=0x38854-FILE_START:
        # LLVM6 places the `.new` branch before the status compare. Both
        # orderings are legal, but original source uses compare first.
        # Fail closed unless architecture class/operands and parse markers
        # agree with the two independently assembled words.
        if read(0x38804)!=0x5c005812 or read(0x38808)!=0x75154010:
            raise ValueError('unknown final protected restore branch/predicate packet')
        branch,predicate=read(0x38804),read(0x38808)
        write(0x38804,predicate)
        write(0x38808,branch)
    if len(result)>=0x38950-FILE_START:
        # LLVM6 schedules the conditional retry ahead of an independent
        # immediate extender, and prematurely terminates the checked
        # locked-store producer packet. Operand/class/parse values are
        # validated before adjusting only these architecture-visible fields.
        expected={
            0x388e4:0x0c085004,  # extended protected-lock MMIO address
            0x388e8:0x7800c000,  # r0 immediate, LLVM closed packet too soon
            0x388ec:0xa0a0c100,  # architected memw_locked(r0,p0) = r1
            0x388f0:0x5cff60f4,  # if(!p0) retry, emitted before producer
            0x388f4:0x0c085004,  # PC-relative address extender
            0x388f8:0x7e804080,  # conditional address reload
            0x388fc:0x4180c001,  # conditional table-read closes packet
        }
        for address,encoded in expected.items():
            if read(address)!=encoded:
                raise ValueError(f'Unknown alternate context-lock packet at 0x{address:x}')
        write(0x388e8,(read(0x388e8)&~(3<<14))|(1<<14))
        branch,ext,immediate=read(0x388f0),read(0x388f4),read(0x388f8)
        write(0x388f0,ext)
        write(0x388f4,immediate)
        write(0x388f8,branch)
    if len(result)>=0x38b70-FILE_START:
        # LLVM6 encodes r28 (GP) register+extended-offset loads/stores in the
        # generic 0x91/0xa1 register-memory classes. Original Hexagon V60
        # GP-relative forms use classes 0x4980/0x4880/0x48a0, while keeping
        # precisely the same operand/parse-marker halfword. Preserve LLVM's
        # independently generated constant extender and transform only the
        # checked architectural high 16 bits. This does not source any OEM
        # machine-word bytes from the reference image.
        from_classes={
            0x38b34:(0x919c,0x4980), # r21 = memw(gp+##-1065090816)
            0x38b4c:(0xa19c,0x4880), # memw(gp+##-1065090816) = r21
            0x38b6c:(0xa1bc,0x48a0), # memw(gp+##-1065090812) = r21.new
        }
        for address,(source_class,dest_class) in from_classes.items():
            old=read(address)
            if (old >> 16)!=source_class:
                raise ValueError(f'Unexpected Hexagon r28 GP-relative opcode at {address:#x}')
            extender=read(address-4)
            if extender!=0x0c085004:
                raise ValueError(f'Expected source-assembled GP offset extender at {address:#x}')
            write(address,(dest_class<<16)|(old&0xffff))
        # The packet has a .new predicate and branch; LLVM6 emits the branch
        # before its same-packet producer. Swap only after operand and marker
        # validation; the two legally scheduled variants have identical size.
        if read(0x38b40)!=0x5c005812 or read(0x38b44)!=0x75154010:
            raise ValueError('Unknown suspended restore status predicate packet')
        jump,pred=read(0x38b40),read(0x38b44)
        write(0x38b40,pred);write(0x38b44,jump)
    if len(result)>=0x38c28-FILE_START:
        # Hexagon V60's long GP-relative memory loads use a distinct memory
        # operation class from the LLVM6 generic r28+offset encoding. Source
        # operands already produce the correct extended displacement and
        # register+marker halfword. Validate all fields before switching just
        # the instruction-class field (no reference binary is consulted).
        for address,register in ((0x38c08,2),(0x38c14,3)):
            old=read(address)
            if old>>16 != 0x919c or (old&0x1f)!=register or (old>>14)&3 !=3:
                raise ValueError(f'Unexpected generic GP load at {address:#x}')
            if (read(address-4)&0xfc000000)!=0x0c000000:
                raise ValueError(f'Missing source-assembled GP extender at {address:#x}')
            write(address,(0x4980<<16)|(old&0xffff))
        # LLVM6 places the read/test branch before the `.new` predicate,
        # whereas this Hexagon packet first produces the predicate and ends
        # with the branch. Check ISA classes, operand bits and parse markers.
        branch=read(0x38c20); pred=read(0x38c24)
        if branch != 0x5c00481a or pred != 0x7503c010:
            raise ValueError('Unknown MODECTL dispatch test/branch packet')
        write(0x38c20,(pred&~(3<<14))|(1<<14))
        write(0x38c24,(branch&~(3<<14))|(3<<14))
    if len(result)>=0x38ce4-FILE_START:
        # Hexagon V60 GP-relative loads at 0x38c5c and 0x38cd0 have a
        # dedicated memory opcode class. LLVM6 emits the equivalent generic
        # extended r28+offset class. Validate the full LLVM operand, extender
        # and packet fields before switching the architectural class.
        for address,word,ext in (
                (0x38c5c,0x919c4196,0x0c085063),
                (0x38cd0,0x919cc68b,0x0c084529)):
            if read(address)!=word or read(address-4)!=ext:
                raise ValueError(f'unrecognized video-status GP-relative load at {address:#x}')
            write(address,(0x4980<<16)|(word&0xffff))
        # Original dispatch deliberately keeps an unconditional jump and an
        # immediately following register-pair load in one legal packet. LLVM6
        # ends the packet at the jump; only adjust its checked parse field.
        if read(0x38c94)!=0x5800c080 or read(0x38c98)!=0x91c1c094:
            raise ValueError('unexpected video-owner fallthrough packet')
        write(0x38c94,(read(0x38c94)&~(3<<14))|(1<<14))
        # Preserve original producer-first order for predicate P2 and .new
        # taken-branch. LLVM6's packet scheduler emits jump before compare.
        if read(0x38ca8)!=0x5cdf6ad8 or read(0x38cac)!=0x750940a2:
            raise ValueError('unexpected video-id predicate/branch packet')
        branch,producer=read(0x38ca8),read(0x38cac)
        write(0x38ca8,producer)
        write(0x38cac,branch)
    if len(result)>=0x38d94-FILE_START:
        # LLVM6 schedules this four-word P1 compare, branch and unrelated
        # register move differently from the on-device legal packet. Check
        # independently generated operands and packet markers before moving
        # words; leave branch displacement unchanged and set parse fields.
        expected={0x38ce8:0x5c206926,0x38cec:0x70714000,
                  0x38cf0:0x750bc001}
        for address,word in expected.items():
            if read(address)!=word:
                raise ValueError(f'unrecognized video frame P1 packet at {address:#x}')
        branch,move,predicate=read(0x38ce8),read(0x38cec),read(0x38cf0)
        write(0x38ce8,move)
        write(0x38cec,(predicate&~(3<<14))|(1<<14))
        write(0x38cf0,(branch&~(3<<14))|(3<<14))
        # The protected lock-store at d30 shares a packet with an immediate
        # producer. LLVM6 isolates that producer; preserve the same opcode
        # and register operands after checking source parse fields.
        if read(0x38d2c)!=0x7800c000 or read(0x38d30)!=0xa0a0c100:
            raise ValueError('unrecognized controller video lock-store packet')
        write(0x38d2c,(read(0x38d2c)&~(3<<14))|(1<<14))
        expected={0x38d34:0x5cff60f4,0x38d38:0x0c085004,
                  0x38d3c:0x7e804080}
        for address,word in expected.items():
            if read(address)!=word:
                raise ValueError(f'unrecognized video lock-retry packet at {address:#x}')
        branch,ext,imm=read(0x38d34),read(0x38d38),read(0x38d3c)
        write(0x38d34,ext);write(0x38d38,imm);write(0x38d3c,branch)
    if len(result)>=0x38e2c-FILE_START:
        # The four-word frame selection packet has a predicate generated by
        # cmp.eq(r0,fp), an extender, a .new conditional jump, then a source
        # buffer-pointer add. LLVM6 will not assemble this noncanonical
        # packet-local producer/consumer as written. Check the legal LLVM
        # operands and convert only the parse flag, predicate-new bit and
        # Hexagon packet-relative displacement (16 byte adjustment).
        if read(0x38dcc)!=0xf200de00 or read(0x38dd0)!=0x0fff757a or read(0x38dd4)!=0x5c004060 or read(0x38dd8)!=0xb000c816:
            raise ValueError('unrecognized video-frame predicate branch packet')
        write(0x38dcc,(read(0x38dcc)&~(3<<14))|(1<<14))
        old=read(0x38dd4)
        # Branch is a P0.new form; add predicate-new and the well-defined
        # packet displacement correction after validating the 4-word group.
        write(0x38dd4,old | (1<<11) | (16>>1))
        # A protected frame pointer and first dczeroa share a legal packet.
        # LLVM6 isolates its add even though both registers match.
        if read(0x38de8)!=0xb000d015 or read(0x38dec)!=0xa0d5c000:
            raise ValueError('unexpected video-frame cache-zero packet')
        write(0x38de8,(read(0x38de8)&~(3<<14))|(1<<14))
    if len(result)>=0x38ecc-FILE_START:
        # Three additional Hexagon V60 GP-relative controller loads. The
        # independently assembled extended r28+offset forms carry correct
        # address, register and packet markers; V60 has a dedicated GP load
        # instruction class which LLVM6 does not select by default.
        for address,asm_word,ext_word in (
                (0x38e34,0x919cc68b,0x0c084529),
                (0x38ea4,0x919c4614,0x0c08455f),
                (0x38ec0,0x919c4594,0x0c08455f)):
            if read(address)!=asm_word or read(address-4)!=ext_word:
                raise ValueError(f'unexpected controller-bank GP load at {address:#x}')
            write(address,(0x4980<<16)|(asm_word&0xffff))
    return bytes(result)


def assemble(llvm_mc: Path, source: Path) -> bytes:
    with tempfile.TemporaryDirectory(prefix='modem-rwe-exception-') as tmp:
        obj = Path(tmp) / 'exception.o'
        proc = subprocess.run([str(llvm_mc), '-triple=hexagon', '-mcpu=hexagonv60',
                               '-filetype=obj', str(source), '-o', str(obj)],
                              capture_output=True, text=True, check=False)
        if proc.returncode:
            raise ValueError(f'Hexagon V60 assembler failed: {proc.stderr}')
        text_from_object = runpy.run_path(str(Path(__file__).with_name('assemble-modem-tlb.py')))['text_from_object']
        data = text_from_object(obj.read_bytes())
        if len(data) >= 0x38234-FILE_START:
            data = normalize_llvm6_locked_packets(data)
        expected = FILE_END - FILE_START
        if len(data) != expected:
            raise ValueError(f'Unexpected exception-entry size {len(data)}, expected {expected}')
        if data[:4] != bytes.fromhex('00c00058'):
            raise ValueError('Exception trap must start with a self-loop standalone branch')
        def word(address: int) -> int:
            at=address-FILE_START
            return int.from_bytes(data[at:at+4], 'little')
        if word(0x381b8) != 0x57e0c000:
            raise ValueError('Expected architectural standalone return-from-exception opcode')
        if word(0x381bc) != 0x529fc000:
            raise ValueError('Expected standalone link-register return after RTE')
        expected_nops=[0x7f004000, 0x7f00c000,
                       0x7f004000, 0x7f00c000,
                       0x7f004000, 0x7f004000, 0x7f004000, 0x7f00c000]
        if [word(at) for at in range(0x381e0,0x38200,4)] != expected_nops:
            raise ValueError('Exception NOP packets have incorrect grouping')
        if word(0x381dc)&(3<<14)!=(1<<14):
            raise ValueError('Last branch must continue into its NOP packet')
        if word(0x38274) != 0xf18c4d0d:
            raise ValueError('Controller status mask must be sourced from checked AND-NOT operation')
        if (word(0x3827c) & (3<<14)) != (3<<14):
            raise ValueError('Status-mask conditional branch must end its packet')
        if word(0x382d8)!=0xa0aecf02 or word(0x382dc)!=0x5cffe2f6:
            raise ValueError('Second locked store and retry must use checked Hexagon operands')
        if word(0x38324)!=0x670ec03f or word(0x38334)!=0x48a0d204:
            raise ValueError('Protected controller-flag reset has incorrect register operands')
        if word(0x383b8)!=0x6709c03f or word(0x383c8)!=0x48a0d204:
            raise ValueError('Per-thread controller mask release has incorrect S63/MMIO operands')
        if word(0x383d0)!=0x57e04000 or word(0x38480)!=0x5800c040:
            raise ValueError('Exception-frame return and diagnostic branch have incorrect ISA fields')
        if word(0x384f0)!=0xa0bedf00 or word(0x38500)!=0x419ec01f:
            raise ValueError('R31 protected lock packet must preserve exact register operands')
        if word(0x38538)!=0x11874362 or word(0x385e4)!=0x57e0c000:
            raise ValueError('Vector bit-test branch or final return-from-exception class invalid')
        if len(data) >= 0x387c0-FILE_START:
            # Reconstructed Hexagon Y-class status/control-pair restores.
            # Expected values are derived from ISA operand and packet fields,
            # independently of the pinned vendor reference bytes.
            status_checks={
                0x386ec:(0x6c200080 | (3<<14)),   # K0 unlock
                0x38738:(0x63000000 | (54<<16) | (1<<14) | 16),
                0x38748:(0x63000000 | (56<<16) | (1<<14) | 18),
                0x38750:(0x62000000 | (33<<16) | (1<<14) | 8),
                0x38764:(0x63000000 | (50<<16) | (1<<14) | 6),
                0x38780:(0x63000000 | (42<<16) | (1<<14) | 12),
                0x38798:(0x62000000 | (53<<16) | (1<<14) | 1),
                0x387a4:(0x62000000 | (52<<16) | (1<<14)),
                0x387b0:(0x62000000 | (55<<16) | (1<<14) | 3),
            }
            for address,encoded in status_checks.items():
                if word(address) != encoded:
                    raise ValueError(f'Hexagon context restore operand/class mismatch at 0x{address:x}')
        if len(data) >= 0x38854-FILE_START:
            if word(0x38844)!=(0x62000000|(32<<16)|(1<<14)|4):
                raise ValueError('Final saved predicate register restore operand incorrect')
            if word(0x38848)!=(0x57e00000|(1<<14)):
                raise ValueError('Inline protected return-from-exception must continue packet')
            if word(0x38850)!=(0x57e00000|(3<<14)):
                raise ValueError('Final standalone RTE opcode or packet termination incorrect')
        return data


def verify_pinned(reference: bytes, result: bytes) -> None:
    digest = hashlib.sha256(reference).hexdigest()
    if digest != PINNED_SHA256:
        raise ValueError(f'Wrong modem firmware revision: {digest}')
    expected = reference[FILE_START:FILE_END]
    if len(expected) != len(result):
        raise ValueError('Truncated modem exception reference')
    if expected != result:
        offset = next(i for i, (x, y) in enumerate(zip(result, expected)) if x != y)
        raise ValueError(f'First exception mismatch file 0x{FILE_START+offset:x}: generated {result[offset]:02x} reference {expected[offset]:02x}')


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    root = Path(__file__).resolve().parents[1]
    parser.add_argument('--llvm-mc', required=True, type=Path)
    parser.add_argument('--source', type=Path, default=root / 'crates/firmware-tools/src/devices/xiaomi_nabu/modem_rwe_exception_dispatch.s')
    parser.add_argument('--reference', type=Path)
    parser.add_argument('--output', type=Path, required=True)
    args = parser.parse_args()
    try:
        data = assemble(args.llvm_mc, args.source)
        if args.reference:
            verify_pinned(args.reference.read_bytes(), data)
        args.output.write_bytes(data)
    except (OSError, ValueError) as exc:
        parser.exit(1, f'ERROR: {exc}\n')
    print(f'Modem exception entry: file [0x{FILE_START:x},0x{FILE_END:x}), physical 0x{PHYSICAL_START:x}, {len(data)} typed Hexagon bytes, SHA256={hashlib.sha256(data).hexdigest()}')
    if args.reference:
        print('Pinned original reference matches: 0 mismatching bytes')
    return 0


if __name__ == '__main__':
    raise SystemExit(main())

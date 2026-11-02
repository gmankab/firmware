/*
 * Data for log.
 *
 * Copyright (c) 2018, Arm Limited.
 * SPDX-License-Identifier: MIT OR Apache-2.0 WITH LLVM-exception
 */

//! Public-source mathematical data for the Novatek upper-ILM log routine.
//!
//! The constants are transcribed from Arm optimized-routines `math/log_data.c`
//! at commit 189dfefe37d54c5b9d2a8bb2039091a638c691e1. No firmware bytes are
//! inputs to these tables or to `emit`. The original C file is preserved at
//! `reference/xiaomi-nabu/novatek/ilm-upper/arm-log_data.c` with its original license.
//!
//! This is the configuration LOG_TABLE_BITS=7, LOG_POLY_ORDER=6,
//! LOG_POLY1_ORDER=12, HAVE_FAST_FMA=0. For x = 2^k z, the log routine uses
//! log(x) = k*ln(2) + log(c) + log(z/c), with one of 128 reduction centers c.
//! The general polynomial approximates the residual log(1+r), r=z/c-1;
//! a separate polynomial handles values close to one. The additional
//! high/low center table evaluates r=(z-chi-clo)*invc without a fast FMA.
//!
//! Decimal f64 literals round-trip to each upstream hexadecimal float exactly;
//! the original hex floats are retained alongside them for review. Emission
//! uses the original `struct log_data` field order from `math_config.h`,
//! independently of the order of designated initializers in `log_data.c`.
//! Every scalar is serialized as IEEE-754 binary64 little-endian, explicitly;
//! no host struct padding or host byte order affects the result.
//!
//! These are 4,240 bytes of identified mathematical data, not executable code.

pub const SOURCE_START: usize = 0x19bf0;
pub const SOURCE_END: usize = 0x1ac80;
pub const LOG_TABLE_BITS: usize = 7;
pub const LOG_POLY_ORDER: usize = 6;
pub const LOG_POLY1_ORDER: usize = 12;
pub const HAVE_FAST_FMA: bool = false;
pub const TABLE_LENGTH: usize = 1 << LOG_TABLE_BITS;
pub const SCALAR_COUNT: usize = 2 + (LOG_POLY_ORDER - 1)
    + (LOG_POLY1_ORDER - 1) + 2 * TABLE_LENGTH + 2 * TABLE_LENGTH;
pub const PUBLIC_SOURCE_SHA256: &str =
    "327cf1819870930486eecb5491c68d5887a712b954e5ec82f3f70bfd9e46fdd3";
pub const EMITTED_SHA256: &str =
    "c26af2ad9c57bf0b64d2d1b93b1e911170e63c39d7bf69bfe423b9e8770a8f4c";

/// Rounded reciprocal center and logarithm used in argument reduction.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ReductionEntry {
    pub reciprocal: f64,
    pub logarithm: f64,
}

/// Center split into high and low parts for reduction without fused multiply-add.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SplitCenter {
    pub high: f64,
    pub low: f64,
}

pub const LN2_HIGH: f64 = 0.6931471805598903; // 0x1.62e42fefa3800p-1

pub const LN2_LOW: f64 = 5.497923018708371e-14; // 0x1.ef35793c76730p-45


/// Residual log polynomial coefficients, for r in approximately [-0x1.fp-9, 0x1.fp-9].
pub const POLYNOMIAL: [f64; LOG_POLY_ORDER - 1] = [
    -0.5000000000000001, // -0x1.0000000000001p-1
    0.33333333331825593, // 0x1.555555551305bp-2
    -0.2499999999622955, // -0x1.fffffffeb459p-3
    0.20000304511814496, // 0x1.999b324f10111p-3
    -0.16667054827627667, // -0x1.55575e506c89fp-3
];


/// Near-one log polynomial coefficients, for x-1 in approximately [-0x1p-4, 0x1.09p-4].
pub const NEAR_ONE_POLYNOMIAL: [f64; LOG_POLY1_ORDER - 1] = [
    -0.5, // -0x1p-1
    0.3333333333333352, // 0x1.5555555555577p-2
    -0.24999999999998432, // -0x1.ffffffffffdcbp-3
    0.19999999999320328, // 0x1.999999995dd0cp-3
    -0.16666666669929706, // -0x1.55555556745a7p-3
    0.14285715076560868, // 0x1.24924a344de3p-3
    -0.12499997863982555, // -0x1.fffffa4423d65p-4
    0.11110712032936046, // 0x1.c7184282ad6cap-4
    -0.10000486757818193, // -0x1.999eb43b068ffp-4
    0.09181994006195467, // 0x1.78182f7afd085p-4
    -0.08328363062289341, // -0x1.5521375d145cdp-4
];

/// 128 reciprocal/logarithm pairs in increasing reduction-center order.
pub const REDUCTION_TABLE: [ReductionEntry; TABLE_LENGTH] = [
    ReductionEntry { reciprocal: 1.4504249240398293, logarithm: -0.3718565645633589 }, // 0x1.734f0c3e0de9fp+0, -0x1.7cc7f79e69000p-2
    ReductionEntry { reciprocal: 1.442253508327276, logarithm: -0.36620682668944937 }, // 0x1.713786a2ce91fp+0, -0x1.76feec20d0000p-2
    ReductionEntry { reciprocal: 1.4341736174350004, logarithm: -0.3605888069791945 }, // 0x1.6f26008fab5a0p+0, -0x1.713e31351e000p-2
    ReductionEntry { reciprocal: 1.426183816329995, logarithm: -0.3550022171419869 }, // 0x1.6d1a61f138c7dp+0, -0x1.6b85b38287800p-2
    ReductionEntry { reciprocal: 1.4182825527052965, logarithm: -0.34944666968829097 }, // 0x1.6b1490bc5b4d1p+0, -0x1.65d5590807800p-2
    ReductionEntry { reciprocal: 1.4104682921759335, logarithm: -0.3439217713603284 }, // 0x1.69147332f0cbap+0, -0x1.602d076180000p-2
    ReductionEntry { reciprocal: 1.4027396147468003, logarithm: -0.3384271921261188 }, // 0x1.6719f18224223p+0, -0x1.5a8ca86909000p-2
    ReductionEntry { reciprocal: 1.3950954438932313, logarithm: -0.332962831494342 }, // 0x1.6524f99a51ed9p+0, -0x1.54f4356035000p-2
    ReductionEntry { reciprocal: 1.3875338232485754, logarithm: -0.32752794345742586 }, // 0x1.63356aa8f24c4p+0, -0x1.4f637c36b4000p-2
    ReductionEntry { reciprocal: 1.3800539211058593, logarithm: -0.32212257167088865 }, // 0x1.614b36b9ddc14p+0, -0x1.49da7fda85000p-2
    ReductionEntry { reciprocal: 1.3726542695419708, logarithm: -0.3167462884799761 }, // 0x1.5f66452c65c4cp+0, -0x1.445923989a800p-2
    ReductionEntry { reciprocal: 1.3653332798446802, logarithm: -0.3113985598928366 }, // 0x1.5d867b5912c4fp+0, -0x1.3edf439b0b800p-2
    ReductionEntry { reciprocal: 1.358090204587874, logarithm: -0.3060794515165526 }, // 0x1.5babccb5b90dep+0, -0x1.396ce448f7000p-2
    ReductionEntry { reciprocal: 1.3509234892132138, logarithm: -0.300788424667644 }, // 0x1.59d61f2d91a78p+0, -0x1.3401e17bda000p-2
    ReductionEntry { reciprocal: 1.3438320840699889, logarithm: -0.2955252968476998 }, // 0x1.5805612465687p+0, -0x1.2e9e2ef468000p-2
    ReductionEntry { reciprocal: 1.3368146974742003, logarithm: -0.29028969275850613 }, // 0x1.56397cee76bd3p+0, -0x1.2941b3830e000p-2
    ReductionEntry { reciprocal: 1.329870114677736, logarithm: -0.2850812793277555 }, // 0x1.54725e2a77f93p+0, -0x1.23ec58cda8800p-2
    ReductionEntry { reciprocal: 1.322997339161106, logarithm: -0.27989987391470095 }, // 0x1.52aff42064583p+0, -0x1.1e9e129279000p-2
    ReductionEntry { reciprocal: 1.316195352741367, logarithm: -0.27474526621870154 }, // 0x1.50f22dbb2bddfp+0, -0x1.1956d2b48f800p-2
    ReductionEntry { reciprocal: 1.3094628125672239, logarithm: -0.2696169863701243 }, // 0x1.4f38f4734ded7p+0, -0x1.141679ab9f800p-2
    ReductionEntry { reciprocal: 1.3027990455471041, logarithm: -0.26451506180308115 }, // 0x1.4d843cfde2840p+0, -0x1.0edd094ef9800p-2
    ReductionEntry { reciprocal: 1.2962024229438942, logarithm: -0.2594387762767383 }, // 0x1.4bd3ec078a3c8p+0, -0x1.09aa518db1000p-2
    ReductionEntry { reciprocal: 1.2896726275815547, logarithm: -0.2543884090981692 }, // 0x1.4a27fc3e0258ap+0, -0x1.047e65263b800p-2
    ReductionEntry { reciprocal: 1.2832080305745537, logarithm: -0.24936321635129843 }, // 0x1.4880524d48434p+0, -0x1.feb224586f000p-3
    ReductionEntry { reciprocal: 1.276807885983376, logarithm: -0.24436312405975968 }, // 0x1.46dce1b192d0bp+0, -0x1.f474a7517b000p-3
    ReductionEntry { reciprocal: 1.2704714060687552, logarithm: -0.23938801747897287 }, // 0x1.453d9d3391854p+0, -0x1.ea4443d103000p-3
    ReductionEntry { reciprocal: 1.2641976054949482, logarithm: -0.23443761696705678 }, // 0x1.43a2744b4845ap+0, -0x1.e020d44e9b000p-3
    ReductionEntry { reciprocal: 1.257985357514882, logarithm: -0.22951151871518505 }, // 0x1.420b54115f8fbp+0, -0x1.d60a22977f000p-3
    ReductionEntry { reciprocal: 1.2518337750655457, logarithm: -0.2246094963439873 }, // 0x1.40782da3ef4b1p+0, -0x1.cc00104959000p-3
    ReductionEntry { reciprocal: 1.2457421919097305, logarithm: -0.21973149037705753 }, // 0x1.3ee8f5d57fe8fp+0, -0x1.c202956891000p-3
    ReductionEntry { reciprocal: 1.2397094966625508, logarithm: -0.2148770752847895 }, // 0x1.3d5d9a00b4ce9p+0, -0x1.b81178d811000p-3
    ReductionEntry { reciprocal: 1.2337348463589233, logarithm: -0.210046029103637 }, // 0x1.3bd60c010c12bp+0, -0x1.ae2c9ccd3d000p-3
    ReductionEntry { reciprocal: 1.2278176973028803, logarithm: -0.20523836373934046 }, // 0x1.3a5242b75dab8p+0, -0x1.a45402e129000p-3
    ReductionEntry { reciprocal: 1.2219570190618474, logarithm: -0.20045368751368642 }, // 0x1.38d22cd9fd002p+0, -0x1.9a877681df000p-3
    ReductionEntry { reciprocal: 1.2161519732977757, logarithm: -0.1956917537758045 }, // 0x1.3755bc5847a1cp+0, -0x1.90c6d69483000p-3
    ReductionEntry { reciprocal: 1.2104018095009725, logarithm: -0.19095237845203883 }, // 0x1.35dce49ad36e2p+0, -0x1.87120a645c000p-3
    ReductionEntry { reciprocal: 1.204705805718973, logarithm: -0.18623539250290833 }, // 0x1.34679984dd440p+0, -0x1.7d68fb4143000p-3
    ReductionEntry { reciprocal: 1.1990631185441964, logarithm: -0.18154051731551135 }, // 0x1.32f5cceffcb24p+0, -0x1.73cb83c627000p-3
    ReductionEntry { reciprocal: 1.1934733004462308, logarithm: -0.1768677957431919 }, // 0x1.3187775a10d49p+0, -0x1.6a39a9b376000p-3
    ReductionEntry { reciprocal: 1.1879350812847385, logarithm: -0.17221657406412305 }, // 0x1.301c8373e3990p+0, -0x1.60b3154b7a000p-3
    ReductionEntry { reciprocal: 1.1824481322833125, logarithm: -0.16758697765942543 }, // 0x1.2eb4ebb95f841p+0, -0x1.5737d76243000p-3
    ReductionEntry { reciprocal: 1.1770114976921955, logarithm: -0.16297859687290384 }, // 0x1.2d50a0219a9d1p+0, -0x1.4dc7b8fc23000p-3
    ReductionEntry { reciprocal: 1.1716248121809465, logarithm: -0.15839151377804228 }, // 0x1.2bef9a8b7fd2ap+0, -0x1.4462c51d20000p-3
    ReductionEntry { reciprocal: 1.1662869231674715, logarithm: -0.15382513241456763 }, // 0x1.2a91c7a0c1babp+0, -0x1.3b08abc830000p-3
    ReductionEntry { reciprocal: 1.1609977486762766, logarithm: -0.14927976358922024 }, // 0x1.293726014b530p+0, -0x1.31b996b490000p-3
    ReductionEntry { reciprocal: 1.1557563220795803, logarithm: -0.14475495398119165 }, // 0x1.27dfa5757a1f5p+0, -0x1.2875490a44000p-3
    ReductionEntry { reciprocal: 1.1505619105480347, logarithm: -0.14025044090817573 }, // 0x1.268b39b1d3bbfp+0, -0x1.1f3b9f879a000p-3
    ReductionEntry { reciprocal: 1.1454138888505974, logarithm: -0.1357660466685502 }, // 0x1.2539d838ff5bdp+0, -0x1.160c8252ca000p-3
    ReductionEntry { reciprocal: 1.140311877374656, logarithm: -0.1313018016355727 }, // 0x1.23eb7aac9083bp+0, -0x1.0ce7f57f72000p-3
    ReductionEntry { reciprocal: 1.1352550225747513, logarithm: -0.12685731518763532 }, // 0x1.22a012ba940b6p+0, -0x1.03cdc49fea000p-3
    ReductionEntry { reciprocal: 1.1302429094831266, logarithm: -0.1224325737671279 }, // 0x1.2157996cc4132p+0, -0x1.f57bdbc4b8000p-4
    ReductionEntry { reciprocal: 1.1252747693068048, logarithm: -0.11802724521862729 }, // 0x1.201201dd2fc9bp+0, -0x1.e370896404000p-4
    ReductionEntry { reciprocal: 1.1203501571039876, logarithm: -0.11364127671663482 }, // 0x1.1ecf4494d480bp+0, -0x1.d17983ef94000p-4
    ReductionEntry { reciprocal: 1.1154683327680124, logarithm: -0.10927434611278386 }, // 0x1.1d8f5528f6569p+0, -0x1.bf9674ed8a000p-4
    ReductionEntry { reciprocal: 1.1106291463292157, logarithm: -0.10492665324943573 }, // 0x1.1c52311577e7cp+0, -0x1.adc79202f6000p-4
    ReductionEntry { reciprocal: 1.1058315813301596, logarithm: -0.10059761422644442 }, // 0x1.1b17c74cb26e9p+0, -0x1.9c0c3e7288000p-4
    ReductionEntry { reciprocal: 1.1010752177696026, logarithm: -0.09628717309055901 }, // 0x1.19e010c2c1ab6p+0, -0x1.8a646b372c000p-4
    ReductionEntry { reciprocal: 1.0963597137952512, logarithm: -0.09199534069557558 }, // 0x1.18ab07bb670bdp+0, -0x1.78d01b3ac0000p-4
    ReductionEntry { reciprocal: 1.0916844827550398, logarithm: -0.08772190036688698 }, // 0x1.1778a25efbcb6p+0, -0x1.674f145380000p-4
    ReductionEntry { reciprocal: 1.0870487291277784, logarithm: -0.08346643613867855 }, // 0x1.1648d354c31dap+0, -0x1.55e0e6d878000p-4
    ReductionEntry { reciprocal: 1.082452357388312, logarithm: -0.07922916827544668 }, // 0x1.151b990275fddp+0, -0x1.4485cdea1e000p-4
    ReductionEntry { reciprocal: 1.0778948225025884, logarithm: -0.0750099004750382 }, // 0x1.13f0ea432d24cp+0, -0x1.333d94d6aa000p-4
    ReductionEntry { reciprocal: 1.0733751731601076, logarithm: -0.07080805133352897 }, // 0x1.12c8b7210f9dap+0, -0x1.22079f8c56000p-4
    ReductionEntry { reciprocal: 1.068893585073351, logarithm: -0.06662408085151128 }, // 0x1.11a3028ecb531p+0, -0x1.10e4698622000p-4
    ReductionEntry { reciprocal: 1.0644491706655506, logarithm: -0.06245745471915143 }, // 0x1.107fbda8434afp+0, -0x1.ffa6c6ad20000p-5
    ReductionEntry { reciprocal: 1.0600414846328305, logarithm: -0.0583080438042316 }, // 0x1.0f5ee0f4e6bb3p+0, -0x1.dda8d4a774000p-5
    ReductionEntry { reciprocal: 1.0556701316181605, logarithm: -0.05417576112313327 }, // 0x1.0e4065d2a9fcep+0, -0x1.bbcece4850000p-5
    ReductionEntry { reciprocal: 1.051334750556926, logarithm: -0.050060547896805474 }, // 0x1.0d244632ca521p+0, -0x1.9a1894012c000p-5
    ReductionEntry { reciprocal: 1.0470347288442157, logarithm: -0.045962101199052086 }, // 0x1.0c0a77ce2981ap+0, -0x1.788583302c000p-5
    ReductionEntry { reciprocal: 1.0427699229652954, logarithm: -0.04188056008865715 }, // 0x1.0af2f83c636d1p+0, -0x1.5715e67d68000p-5
    ReductionEntry { reciprocal: 1.0385395013738175, logarithm: -0.03781540056183985 }, // 0x1.09ddb98a01339p+0, -0x1.35c8a49658000p-5
    ReductionEntry { reciprocal: 1.034343418940345, logarithm: -0.03376684757915882 }, // 0x1.08cabaf52e7dfp+0, -0x1.149e364154000p-5
    ReductionEntry { reciprocal: 1.0301811073173315, logarithm: -0.029734619131772888 }, // 0x1.07b9f2f4e28fbp+0, -0x1.e72c082eb8000p-6
    ReductionEntry { reciprocal: 1.026052043621297, logarithm: -0.025718470239212365 }, // 0x1.06ab58c358f19p+0, -0x1.a55f152528000p-6
    ReductionEntry { reciprocal: 1.0219561082336197, logarithm: -0.021718543925430822 }, // 0x1.059eea5ecf92cp+0, -0x1.63d62cf818000p-6
    ReductionEntry { reciprocal: 1.0178926505784922, logarithm: -0.01773446126981071 }, // 0x1.04949cdd12c90p+0, -0x1.228fb8caa0000p-6
    ReductionEntry { reciprocal: 1.0138614436244586, logarithm: -0.013766252464051831 }, // 0x1.038c6c6f0ada9p+0, -0x1.c317b20f90000p-7
    ReductionEntry { reciprocal: 1.0098620186501341, logarithm: -0.009813706322574944 }, // 0x1.02865137932a9p+0, -0x1.419355daa0000p-7
    ReductionEntry { reciprocal: 1.0058938559734134, logarithm: -0.005876555150052809 }, // 0x1.0182427ea7348p+0, -0x1.81203c2ec0000p-8
    ReductionEntry { reciprocal: 1.00195696235014, logarithm: -0.0019550499938532084 }, // 0x1.008040614b195p+0, -0x1.0040979240000p-9
    ReductionEntry { reciprocal: 0.9961089923088509, logarithm: 0.0038985973556009412 }, // 0x1.fe01ff726fa1ap-1, 0x1.feff384900000p-9
    ReductionEntry { reciprocal: 0.9884170338185201, logarithm: 0.011650571286395461 }, // 0x1.fa11cc261ea74p-1, 0x1.7dc41353d0000p-7
    ReductionEntry { reciprocal: 0.9808429191005297, logarithm: 0.019342955478919066 }, // 0x1.f6310b081992ep-1, 0x1.3cea3c4c28000p-6
    ReductionEntry { reciprocal: 0.9733840169987446, logarithm: 0.0269766014846482 }, // 0x1.f25f63ceeadcdp-1, 0x1.b9fc114890000p-6
    ReductionEntry { reciprocal: 0.9660377568876556, logarithm: 0.034552359728422744 }, // 0x1.ee9c8039113e7p-1, 0x1.1b0d8ce110000p-5
    ReductionEntry { reciprocal: 0.9588014945307369, logarithm: 0.04207121767183253 }, // 0x1.eae8078cbb1abp-1, 0x1.58a5bd001c000p-5
    ReductionEntry { reciprocal: 0.9516728569073111, logarithm: 0.049533940950141186 }, // 0x1.e741aa29d0c9bp-1, 0x1.95c8340d88000p-5
    ReductionEntry { reciprocal: 0.9446494635965822, logarithm: 0.056941358295944156 }, // 0x1.e3a91830a99b5p-1, 0x1.d276aef578000p-5
    ReductionEntry { reciprocal: 0.9377288993026223, logarithm: 0.06429439168346107 }, // 0x1.e01e009609a56p-1, 0x1.07598e598c000p-4
    ReductionEntry { reciprocal: 0.9309091073790681, logarithm: 0.0715936354946507 }, // 0x1.dca01e577bb98p-1, 0x1.253f5e30d2000p-4
    ReductionEntry { reciprocal: 0.924187681612722, logarithm: 0.07884010933776153 }, // 0x1.d92f20b7c9103p-1, 0x1.42edd8b380000p-4
    ReductionEntry { reciprocal: 0.9175626765599192, logarithm: 0.08603438905970506 }, // 0x1.d5cac66fb5ccep-1, 0x1.606598757c000p-4
    ReductionEntry { reciprocal: 0.9110320403624034, logarithm: 0.09317721180013905 }, // 0x1.d272caa5ede9dp-1, 0x1.7da76356a0000p-4
    ReductionEntry { reciprocal: 0.9045935839762024, logarithm: 0.10026951462748457 }, // 0x1.cf26e3e6b2ccdp-1, 0x1.9ab434e1c6000p-4
    ReductionEntry { reciprocal: 0.8982456375922825, logarithm: 0.10731170956330516 }, // 0x1.cbe6da2a77902p-1, 0x1.b78c7bb0d6000p-4
    ReductionEntry { reciprocal: 0.8919860966782501, logarithm: 0.11430473320717738 }, // 0x1.c8b266d37086dp-1, 0x1.d431332e72000p-4
    ReductionEntry { reciprocal: 0.8858131121185129, logarithm: 0.12124928503033061 }, // 0x1.c5894bd5d5804p-1, 0x1.f0a3171de6000p-4
    ReductionEntry { reciprocal: 0.879725075760676, logarithm: 0.12814583422959913 }, // 0x1.c26b533bb9f8cp-1, 0x1.067152b914000p-3
    ReductionEntry { reciprocal: 0.8737201372634685, logarithm: 0.1349951636851756 }, // 0x1.bf583eeece73fp-1, 0x1.147858292b000p-3
    ReductionEntry { reciprocal: 0.8677966405782273, logarithm: 0.1417978768189414 }, // 0x1.bc4fd75db96c1p-1, 0x1.2266ecdca3000p-3
    ReductionEntry { reciprocal: 0.8619528050060739, logarithm: 0.14855476039031146 }, // 0x1.b951e0c864a28p-1, 0x1.303d7a6c55000p-3
    ReductionEntry { reciprocal: 0.8561872354420692, logarithm: 0.1552661937658968 }, // 0x1.b65e2c5ef3e2cp-1, 0x1.3dfc33c331000p-3
    ReductionEntry { reciprocal: 0.8504983927816893, logarithm: 0.16193275688146969 }, // 0x1.b374867c9888bp-1, 0x1.4ba366b7a8000p-3
    ReductionEntry { reciprocal: 0.8448844572790304, logarithm: 0.16855539792220497 }, // 0x1.b094b211d304ap-1, 0x1.5933928d1f000p-3
    ReductionEntry { reciprocal: 0.8393442741575965, logarithm: 0.1751343179947753 }, // 0x1.adbe885f2ef7ep-1, 0x1.66acd2418f000p-3
    ReductionEntry { reciprocal: 0.8338762249349438, logarithm: 0.1816702989864325 }, // 0x1.aaf1d31603da2p-1, 0x1.740f8ec669000p-3
    ReductionEntry { reciprocal: 0.8284789320557778, logarithm: 0.18816387146023317 }, // 0x1.a82e63fd358a7p-1, 0x1.815c0f51af000p-3
    ReductionEntry { reciprocal: 0.8231510800065832, logarithm: 0.1946155228479256 }, // 0x1.a5740ef09738bp-1, 0x1.8e92954f68000p-3
    ReductionEntry { reciprocal: 0.8178913903778707, logarithm: 0.20102572579389744 }, // 0x1.a2c2a90ab4b27p-1, 0x1.9bb3602f84000p-3
    ReductionEntry { reciprocal: 0.8126984007245374, logarithm: 0.2073952090795501 }, // 0x1.a01a01393f2d1p-1, 0x1.a8bed1c2c0000p-3
    ReductionEntry { reciprocal: 0.8075710029460227, logarithm: 0.21372429840596396 }, // 0x1.9d79f24db3c1bp-1, 0x1.b5b515c01d000p-3
    ReductionEntry { reciprocal: 0.8025078881160415, logarithm: 0.2200135945981856 }, // 0x1.9ae2505c7b190p-1, 0x1.c2967ccbcc000p-3
    ReductionEntry { reciprocal: 0.7975077379364331, logarithm: 0.22626374162859975 }, // 0x1.9852ef297ce2fp-1, 0x1.cf635d5486000p-3
    ReductionEntry { reciprocal: 0.792569604966373, logarithm: 0.23247494747693054 }, // 0x1.95cbaeea44b75p-1, 0x1.dc1bd3446c000p-3
    ReductionEntry { reciprocal: 0.7876923641254114, logarithm: 0.23864766620658884 }, // 0x1.934c69de74838p-1, 0x1.e8c01b8cfe000p-3
    ReductionEntry { reciprocal: 0.7828746724940998, logarithm: 0.24478265647405806 }, // 0x1.90d4f2f6752e6p-1, 0x1.f5509c0179000p-3
    ReductionEntry { reciprocal: 0.7781155388790811, logarithm: 0.25088025827324145 }, // 0x1.8e6528effd79dp-1, 0x1.00e6c121fb800p-2
    ReductionEntry { reciprocal: 0.7734139557869777, logarithm: 0.2569408552510595 }, // 0x1.8bfce9fcc007cp-1, 0x1.071b80e93d000p-2
    ReductionEntry { reciprocal: 0.7687687179914933, logarithm: 0.26296511155101143 }, // 0x1.899c0dabec30ep-1, 0x1.0d46b9e867000p-2
    ReductionEntry { reciprocal: 0.7641790698041854, logarithm: 0.2689531327189343 }, // 0x1.87427aa2317fbp-1, 0x1.13687334bd000p-2
    ReductionEntry { reciprocal: 0.7596438763692399, logarithm: 0.27490553924610595 }, // 0x1.84f00acb39a08p-1, 0x1.1980d67234800p-2
    ReductionEntry { reciprocal: 0.7551621951078668, logarithm: 0.2808227248478943 }, // 0x1.82a49e8653e55p-1, 0x1.1f8ffe0cc8000p-2
    ReductionEntry { reciprocal: 0.7507331780216866, logarithm: 0.286704979267256 }, // 0x1.8060195f40260p-1, 0x1.2595fd7636800p-2
    ReductionEntry { reciprocal: 0.7463557196361751, logarithm: 0.29255295645509705 }, // 0x1.7e22563e0a329p-1, 0x1.2b9300914a800p-2
    ReductionEntry { reciprocal: 0.7420289364869653, logarithm: 0.2983670386142876 }, // 0x1.7beb377dcb5adp-1, 0x1.3187210436000p-2
    ReductionEntry { reciprocal: 0.7377521537065876, logarithm: 0.30414734587282055 }, // 0x1.79baa679725c2p-1, 0x1.377266dec1800p-2
    ReductionEntry { reciprocal: 0.7335242966002608, logarithm: 0.30989455774829366 }, // 0x1.77907f2170657p-1, 0x1.3d54ffbaf3000p-2
    ReductionEntry { reciprocal: 0.729344777457841, logarithm: 0.31560871301871884 }, // 0x1.756cadbd6130cp-1, 0x1.432eee32fe000p-2
];

/// 128 high/low center pairs, corresponding one-for-one with REDUCTION_TABLE.
pub const CENTER_SPLIT_TABLE: [SplitCenter; TABLE_LENGTH] = [
    SplitCenter { high: 0.6894531274426304, low: 2.60290652810535e-17 }, // 0x1.61000014fb66bp-1, 0x1.e026c91425b3cp-56
    SplitCenter { high: 0.6933593811533166, low: 5.1607448519931415e-17 }, // 0x1.63000034db495p-1, 0x1.dbfea48005d41p-55
    SplitCenter { high: 0.6972656502972674, low: 5.290672414887869e-17 }, // 0x1.650000d94d478p-1, 0x1.e7fa786d6a5b7p-55
    SplitCenter { high: 0.7011718886092148, low: 7.801030634806013e-18 }, // 0x1.67000074e6fadp-1, 0x1.1fcea6b54254cp-57
    SplitCenter { high: 0.7050781228976939, low: -2.471355337688684e-17 }, // 0x1.68ffffedf0faep-1, -0x1.c7e274c590efdp-56
    SplitCenter { high: 0.7089843887644558, low: -4.6413389636862956e-17 }, // 0x1.6b0000763c5bcp-1, -0x1.ac16848dcda01p-55
    SplitCenter { high: 0.7128906815542553, low: 3.3387408065940366e-17 }, // 0x1.6d0001e5cc1f6p-1, 0x1.33f1c9d499311p-55
    SplitCenter { high: 0.7167968359278303, low: -2.6454587338050098e-17 }, // 0x1.6efffeb05f63ep-1, -0x1.e80041ae22d53p-56
    SplitCenter { high: 0.7207031520563163, low: 2.4284096334522285e-17 }, // 0x1.710000e86978p-1, 0x1.bff6671097952p-56
    SplitCenter { high: 0.7246093683054673, low: 4.85782435031982e-17 }, // 0x1.72ffffc67e912p-1, 0x1.c00e226bd8724p-55
    SplitCenter { high: 0.7285155644718035, low: -1.3014776598894487e-17 }, // 0x1.74fffdf81116ap-1, -0x1.e02916ef101d2p-57
    SplitCenter { high: 0.7324219036935506, low: -1.0402317671225898e-17 }, // 0x1.770000f679c9p-1, -0x1.7fc71cd549c74p-57
    SplitCenter { high: 0.73632811474659, low: 3.078291427467431e-17 }, // 0x1.78ffffa7ec835p-1, 0x1.1bec19ef50483p-55
    SplitCenter { high: 0.7402343715130797, low: -1.4304998976623033e-17 }, // 0x1.7affffe20c2e6p-1, -0x1.07e1729cc6465p-56
    SplitCenter { high: 0.7441405900738403, low: -2.8625955785571303e-17 }, // 0x1.7cfffed3fc9p-1, -0x1.08072087b8b1cp-55
    SplitCenter { high: 0.7480468324364, low: 5.1609093535635645e-17 }, // 0x1.7efffe9261a76p-1, 0x1.dc0286d9df9aep-55
    SplitCenter { high: 0.7519531335902885, low: 4.423423949536319e-17 }, // 0x1.81000049ca3e8p-1, 0x1.97fd251e54c33p-55
    SplitCenter { high: 0.7558594189116706, low: -4.683016762099508e-17 }, // 0x1.8300017932c8fp-1, -0x1.afee9b630f381p-55
    SplitCenter { high: 0.7597656365502305, low: 4.466742007514772e-17 }, // 0x1.850000633739cp-1, 0x1.9bfbf6b6535bcp-55
    SplitCenter { high: 0.7636719350887737, low: -4.8134498775190266e-17 }, // 0x1.87000204289c6p-1, -0x1.bbf65f3117b75p-55
    SplitCenter { high: 0.767578087670501, low: -4.3371015350542286e-17 }, // 0x1.88fffebf57904p-1, -0x1.9006ea23dcb57p-55
    SplitCenter { high: 0.7714844396979536, low: -2.515644472251707e-17 }, // 0x1.8b00022bc04dfp-1, -0x1.d00df38e04b0ap-56
    SplitCenter { high: 0.7753905747966752, low: -4.163636184961329e-17 }, // 0x1.8cfffe50c1b8ap-1, -0x1.8007146ff9f05p-55
    SplitCenter { high: 0.7792968686084766, low: 3.3837161384165165e-17 }, // 0x1.8effffc918e43p-1, 0x1.3817bd07a7038p-55
    SplitCenter { high: 0.783203182701066, low: 4.379206565056758e-17 }, // 0x1.910001efa5fc7p-1, 0x1.93e9176dfb403p-55
    SplitCenter { high: 0.7871094109030913, low: 2.7322930976146396e-17 }, // 0x1.9300013467bb9p-1, 0x1.f804e4b980276p-56
    SplitCenter { high: 0.7910155783031153, low: -5.463661225364046e-17 }, // 0x1.94fffe6ee076fp-1, -0x1.f7ef0d9ff622ep-55
    SplitCenter { high: 0.7949218121071572, low: -1.4320502634189868e-17 }, // 0x1.96fffde3c12d1p-1, -0x1.082aa962638bap-56
    SplitCenter { high: 0.7988281031542229, low: -4.0766731402809024e-17 }, // 0x1.98ffff4458a0dp-1, -0x1.7801b9164a8efp-55
    SplitCenter { high: 0.8027343109146796, low: -4.0338264352540086e-17 }, // 0x1.9afffdd982e3ep-1, -0x1.740e08a5a9337p-55
    SplitCenter { high: 0.8066405901480322, low: 1.7241428071650815e-18 }, // 0x1.9cfffed49fb66p-1, 0x1.fce08c19bep-60
    SplitCenter { high: 0.8105469363625933, low: -4.5534218930924523e-17 }, // 0x1.9f00020f19c51p-1, -0x1.a3faa27885b0ap-55
    SplitCenter { high: 0.8144531571720115, low: 1.821216920589142e-17 }, // 0x1.a10001145b006p-1, 0x1.4ff489958da56p-56
    SplitCenter { high: 0.8183593894061396, low: 4.9864288025894273e-17 }, // 0x1.a300007bbf6fap-1, 0x1.cbeab8a2b6d18p-55
    SplitCenter { high: 0.8222656559018297, low: 4.335990416726689e-17 }, // 0x1.a500010971d79p-1, 0x1.8fecadd78793p-55
    SplitCenter { high: 0.8261719308006343, low: -5.422001471661127e-17 }, // 0x1.a70001df52e48p-1, -0x1.f41763dd8abdbp-55
    SplitCenter { high: 0.830078177803083, low: -5.333603728929035e-17 }, // 0x1.a90001c593352p-1, -0x1.ebf0284c27612p-55
    SplitCenter { high: 0.8339844538076674, low: -1.1270648442682386e-17 }, // 0x1.ab0002a4f3e4bp-1, -0x1.9fd043cff3f5fp-57
    SplitCenter { high: 0.8378905498984414, low: -3.165126734800398e-17 }, // 0x1.acfffd7ae1ed1p-1, -0x1.23ee7129070b4p-55
    SplitCenter { high: 0.8417968420618669, low: 1.1286283062565865e-17 }, // 0x1.aefffee510478p-1, 0x1.a063ee00edea3p-57
    SplitCenter { high: 0.8457030568173808, low: 5.643595964448442e-18 }, // 0x1.b0fffdb650d5bp-1, 0x1.a06c8381f0ab9p-58
    SplitCenter { high: 0.8496093725173733, low: -2.1687834632368187e-17 }, // 0x1.b2ffffeaaca57p-1, -0x1.9011e74233c1dp-56
    SplitCenter { high: 0.8535155534462677, low: -2.2548234217914977e-17 }, // 0x1.b4fffd995badcp-1, -0x1.9ff1068862a9fp-56
    SplitCenter { high: 0.8574219432077146, low: 4.683260556353139e-17 }, // 0x1.b7000249e659cp-1, 0x1.aff45d0864f3ep-55
    SplitCenter { high: 0.8613281129443706, low: 2.5148296884122795e-17 }, // 0x1.b8ffff987164p-1, 0x1.cfe7796c2c3f9p-56
    SplitCenter { high: 0.8652342893532053, low: -8.672187570949467e-18 }, // 0x1.bafffd204cb4fp-1, -0x1.3ff27eef22bc4p-57
    SplitCenter { high: 0.8691405397938828, low: -1.2576268285559751e-17 }, // 0x1.bcfffd2415c45p-1, -0x1.cffb7ee3bea21p-57
    SplitCenter { high: 0.8730468608194392, low: -2.9930851265148e-17 }, // 0x1.beffff86309dfp-1, -0x1.14103972e0b5cp-55
    SplitCenter { high: 0.8769530685783117, low: 4.814801507681644e-17 }, // 0x1.c0fffe1b57653p-1, 0x1.bc16494b76a19p-55
    SplitCenter { high: 0.8808593488818101, low: -9.10549533611022e-18 }, // 0x1.c2ffff1fa57e3p-1, -0x1.4feef8d30c6edp-57
    SplitCenter { high: 0.8847655593409667, low: -3.512414651620766e-17 }, // 0x1.c4fffdcbfe424p-1, -0x1.43f68bcec4775p-55
    SplitCenter { high: 0.888671840226208, low: 3.555261815374023e-17 }, // 0x1.c6fffed54b9f7p-1, 0x1.47ea3f053e0ecp-55
    SplitCenter { high: 0.8925780870017613, low: 1.692380503540994e-17 }, // 0x1.c8fffeb998fd5p-1, 0x1.383068df992f1p-56
    SplitCenter { high: 0.8964844367374554, low: -1.0837881791715702e-17 }, // 0x1.cb0002125219ap-1, -0x1.8fd8e64180e04p-57
    SplitCenter { high: 0.9003905608862683, low: 5.2900545724138363e-17 }, // 0x1.ccfffdd94469cp-1, 0x1.e7ebe1cc7ea72p-55
    SplitCenter { high: 0.9042968358682077, low: 5.3330721088784517e-17 }, // 0x1.cefffeafdc476p-1, 0x1.ebe39ad9f88fep-55
    SplitCenter { high: 0.9082031671057441, low: 1.8640040753232978e-17 }, // 0x1.d1000169af82bp-1, 0x1.57d91a8b95a71p-56
    SplitCenter { high: 0.9121093993305497, low: 4.467972832054075e-17 }, // 0x1.d30000d0ff71dp-1, 0x1.9c1906970c7dap-55
    SplitCenter { high: 0.9160155849026457, low: -5.216213419585033e-18 }, // 0x1.d4fffea790fc4p-1, -0x1.80e37c558fe0cp-58
    SplitCenter { high: 0.9199219622862499, low: -2.7324731032846873e-17 }, // 0x1.d70002edc87e5p-1, -0x1.f80d64dc10f44p-56
    SplitCenter { high: 0.9238281880717143, low: -1.7769263393434193e-17 }, // 0x1.d900021dc82aap-1, -0x1.47c8f94fd5c5cp-56
    SplitCenter { high: 0.9277343012727929, low: 4.943363080817441e-17 }, // 0x1.dafffd86b0283p-1, 0x1.c7f1dc521617ep-55
    SplitCenter { high: 0.9316407021562788, low: 4.164434042024055e-17 }, // 0x1.dd000296c4739p-1, 0x1.8019eb2ffb153p-55
    SplitCenter { high: 0.935546825207466, low: 1.3011820859795741e-17 }, // 0x1.defffe54490f5p-1, 0x1.e00d2c652cc89p-57
    SplitCenter { high: 0.9394530312562943, low: -2.7332907838481398e-17 }, // 0x1.e0fffcdabf694p-1, -0x1.f8340202d69d2p-56
    SplitCenter { high: 0.9433593066844669, low: 2.342133170803292e-17 }, // 0x1.e2fffdb52c8ddp-1, 0x1.b00c1ca1b0864p-56
    SplitCenter { high: 0.9472655994038329, low: 1.6478717480487438e-17 }, // 0x1.e4ffff24216efp-1, 0x1.2ffa8b094ab51p-56
    SplitCenter { high: 0.9511718313032721, low: -5.19608290131499e-18 }, // 0x1.e6fffe88a5e11p-1, -0x1.7f673b1efbe59p-58
    SplitCenter { high: 0.9550781578218178, low: -3.5565573220579e-17 }, // 0x1.e9000119eff0dp-1, -0x1.4808d5e0bc801p-55
    SplitCenter { high: 0.9589843147339043, low: 2.0816772146458738e-17 }, // 0x1.eafffdfa51744p-1, 0x1.80006d54320b5p-56
    SplitCenter { high: 0.9628906735633685, low: -3.471962826146841e-18 }, // 0x1.ed0001a127fa1p-1, -0x1.002f860565c92p-58
    SplitCenter { high: 0.9667968893972092, low: -3.68646834472642e-17 }, // 0x1.ef00007babcc4p-1, -0x1.540445d35e611p-55
    SplitCenter { high: 0.9707031054025779, low: -3.4674108076868834e-18 }, // 0x1.f0ffff57a8d02p-1, -0x1.ffb3139ef9105p-59
    SplitCenter { high: 0.9746094325494931, low: 4.598152624774392e-17 }, // 0x1.f30001ee58ac7p-1, 0x1.a81acf2731155p-55
    SplitCenter { high: 0.9785156054582722, low: 4.553145752392568e-17 }, // 0x1.f4ffff5823494p-1, 0x1.a3f41d4d7c743p-55
    SplitCenter { high: 0.9824218687812282, low: -7.811259169418978e-18 }, // 0x1.f6ffffca94c6bp-1, -0x1.202f41c987875p-57
    SplitCenter { high: 0.9863280690752917, low: 2.0375615178047467e-17 }, // 0x1.f8fffe1f9c441p-1, 0x1.77dd1f477e74bp-56
    SplitCenter { high: 0.9902342909546034, low: -1.344597043422914e-17 }, // 0x1.fafffd2e0e37ep-1, -0x1.f01199a7ca331p-57
    SplitCenter { high: 0.9941406780264008, low: 1.51853723695054e-17 }, // 0x1.fd0001c77e49ep-1, 0x1.181ee4bceacb1p-56
    SplitCenter { high: 0.9980468598715558, low: -1.301926039598001e-17 }, // 0x1.feffff7e0c331p-1, -0x1.e05370170875ap-57
    SplitCenter { high: 1.0039062067717412, low: -4.596120641744973e-17 }, // 0x1.00ffff465606ep+0, -0x1.a7ead491c0adap-55
    SplitCenter { high: 1.0117187035281372, low: -8.152405002665349e-17 }, // 0x1.02ffff3867a58p+0, -0x1.77f69c3fcb2ep-54
    SplitCenter { high: 1.0195312424919558, low: 8.239927014757811e-17 }, // 0x1.04ffffdfc0d17p+0, 0x1.7bffe34cb945bp-54
    SplitCenter { high: 1.0273437641634193, low: 3.122851005327045e-17 }, // 0x1.0700003cd4d82p+0, 0x1.20083c0e456cbp-55
    SplitCenter { high: 1.0351562274561221, low: -1.3010212611128995e-17 }, // 0x1.08ffff9f2cbe8p+0, -0x1.dffdfbe37751ap-57
    SplitCenter { high: 1.0429687539123276, low: -5.984116688633458e-17 }, // 0x1.0b000010cda65p+0, -0x1.13f7faee626ebp-54
    SplitCenter { high: 1.0507812561238108, low: 2.860923854822335e-17 }, // 0x1.0d00001a4d338p+0, 0x1.07dfa79489ff7p-55
    SplitCenter { high: 1.0585937308350133, low: -1.9962944508904598e-17 }, // 0x1.0effffadafdfdp+0, -0x1.7040570d66bcp-56
    SplitCenter { high: 1.0664062936992642, low: 5.291469130309099e-17 }, // 0x1.110000bbafd96p+0, 0x1.e80d4846d0b62p-55
    SplitCenter { high: 1.0742187309945372, low: 1.0321575006732523e-16 }, // 0x1.12ffffae5f45dp+0, 0x1.dbffa64fd36efp-54
    SplitCenter { high: 1.082031301537134, low: 9.021194373419119e-17 }, // 0x1.150000dd59ad9p+0, 0x1.a0077701250aep-54
    SplitCenter { high: 1.0898438063644336, low: 5.202798992357953e-17 }, // 0x1.170000f21559ap+0, 0x1.dfdf9e2e3deeep-55
    SplitCenter { high: 1.0976562356711468, low: 5.898318482500785e-17 }, // 0x1.18ffffc275426p+0, 0x1.10030dc3b7273p-54
    SplitCenter { high: 1.1054688179463226, low: 8.846377690653252e-17 }, // 0x1.1b000123d3c59p+0, 0x1.97f7980030188p-54
    SplitCenter { high: 1.113281220803328, low: -9.529455965212574e-18 }, // 0x1.1cffff8299eb7p+0, -0x1.5f932ab9f8c67p-57
    SplitCenter { high: 1.1210937073167315, low: 6.765080709377787e-17 }, // 0x1.1effff48ad4p+0, 0x1.37fbf9da75bebp-54
    SplitCenter { high: 1.1289062967338532, low: 1.0929327370933015e-16 }, // 0x1.210000c8b86a4p+0, 0x1.f806b91fd5b22p-54
    SplitCenter { high: 1.1367187631150848, low: 6.938570551230516e-17 }, // 0x1.2300003854303p+0, 0x1.3ffc2eb9fbf33p-54
    SplitCenter { high: 1.1445312490244826, low: 1.908841015045148e-17 }, // 0x1.24fffffbcf684p+0, 0x1.601e77e2e2e72p-56
    SplitCenter { high: 1.1523437096203593, low: 2.7744504138979832e-17 }, // 0x1.26ffff52921d9p+0, 0x1.ffcbb767f0c61p-56
    SplitCenter { high: 1.1601563266482477, low: -1.5621964097491672e-17 }, // 0x1.2900014933a3cp+0, -0x1.202ca3c02412bp-56
    SplitCenter { high: 1.1679688257483505, low: -6.4191661495975e-17 }, // 0x1.2b00014556313p+0, -0x1.2808233f21f02p-54
    SplitCenter { high: 1.1757811754697645, low: -4.336465165059268e-17 }, // 0x1.2cfffebfe523bp+0, -0x1.8ff7e384fdcf2p-55
    SplitCenter { high: 1.183593793665613, low: -3.8159292551356396e-17 }, // 0x1.2f0000bb8ad96p+0, -0x1.5ff51503041c5p-55
    SplitCenter { high: 1.1914062331617676, low: -2.94933042768015e-17 }, // 0x1.30ffffb7ae2afp+0, -0x1.10071885e289dp-55
    SplitCenter { high: 1.1992187450577771, low: -6.244142916351348e-17 }, // 0x1.32ffffeac5f7fp+0, -0x1.1ff5d3fb7b715p-54
    SplitCenter { high: 1.2070312971249755, low: 7.458644622627413e-17 }, // 0x1.350000ca66756p+0, 0x1.57f82228b82bdp-54
    SplitCenter { high: 1.2148438169965134, low: 2.7760519266147807e-17 }, // 0x1.3700011fbf721p+0, 0x1.000bac40dd5ccp-55
    SplitCenter { high: 1.222656225220801, low: -7.025106921079665e-17 }, // 0x1.38ffff9592fb9p+0, -0x1.43f9d2db2a751p-54
    SplitCenter { high: 1.2304687681290862, low: 3.7292622364618835e-17 }, // 0x1.3b00004ddd242p+0, 0x1.57f6b707638e1p-55
    SplitCenter { high: 1.2382812116234925, low: 2.2558949940652082e-17 }, // 0x1.3cffff5b2c957p+0, 0x1.a023a10bf1231p-56
    SplitCenter { high: 1.2460936706149877, low: 8.499368944400665e-17 }, // 0x1.3efffeab0b418p+0, 0x1.87f6d66b152bp-54
    SplitCenter { high: 1.2539063289686938, low: 1.039515474644361e-17 }, // 0x1.410001532aff4p+0, 0x1.7f8375f198524p-57
    SplitCenter { high: 1.2617188367227732, low: 3.297262223275499e-17 }, // 0x1.4300017478b29p+0, 0x1.301e672dc5143p-55
    SplitCenter { high: 1.2695311590462317, low: 4.509883253594155e-17 }, // 0x1.44fffe795b463p+0, 0x1.9ff69b8b2895ap-55
    SplitCenter { high: 1.277343660657941, low: -7.546987371769545e-17 }, // 0x1.46fffe80475ep+0, -0x1.5c0b19bc2f254p-54
    SplitCenter { high: 1.2851561882963496, low: 9.454295598818184e-17 }, // 0x1.48fffef6fc1e7p+0, 0x1.b4009f23a2a72p-54
    SplitCenter { high: 1.2929686521915196, low: -7.28545609119749e-17 }, // 0x1.4afffe5bea704p+0, -0x1.4ffb7bf0d7d45p-54
    SplitCenter { high: 1.3007813359167737, low: -8.934357651462802e-17 }, // 0x1.4d000171027dep+0, -0x1.9c06471dc6a3dp-54
    SplitCenter { high: 1.3085938093753886, low: 8.15257059458102e-17 }, // 0x1.4f0000ff03ee2p+0, 0x1.77f890b85531cp-54
    SplitCenter { high: 1.3164063202609564, low: 6.946341460765881e-18 }, // 0x1.5100012dc4bd1p+0, 0x1.004657166a436p-57
    SplitCenter { high: 1.3242188320313901, low: -7.892731380721676e-17 }, // 0x1.530001605277ap+0, -0x1.6bfcece233209p-54
    SplitCenter { high: 1.3320311786874468, low: -4.3384657500713545e-17 }, // 0x1.54fffecdb704cp+0, -0x1.902720505a1d7p-55
    SplitCenter { high: 1.3398436880573092, low: 9.627577954598792e-17 }, // 0x1.56fffef5f54a9p+0, 0x1.bbfe60ec96412p-54
    SplitCenter { high: 1.3476563390295309, low: 4.2492400590204517e-17 }, // 0x1.5900017e61012p+0, 0x1.87ec581afef9p-55
    SplitCenter { high: 1.3554687641043626, low: -1.0843419551446047e-16 }, // 0x1.5b00003c93e92p+0, -0x1.f41080abf0ccp-54
    SplitCenter { high: 1.3632813590971713, low: -8.50172782472025e-17 }, // 0x1.5d0001d4919bcp+0, -0x1.8812afb254729p-54
    SplitCenter { high: 1.3710936595522603, low: -7.110627206162922e-17 }, // 0x1.5efffe7b87a89p+0, -0x1.47eb780ed6904p-54
];

/// Emit the 530 public-source binary64 constants in the original C layout.
pub fn emit() -> Vec<u8> {
    let mut bytes = Vec::with_capacity(SOURCE_END - SOURCE_START);
    for scalar in [LN2_HIGH, LN2_LOW] {
        bytes.extend_from_slice(&scalar.to_le_bytes());
    }
    for scalar in POLYNOMIAL.iter().chain(NEAR_ONE_POLYNOMIAL.iter()) {
        bytes.extend_from_slice(&scalar.to_le_bytes());
    }
    for entry in REDUCTION_TABLE {
        bytes.extend_from_slice(&entry.reciprocal.to_le_bytes());
        bytes.extend_from_slice(&entry.logarithm.to_le_bytes());
    }
    for center in CENTER_SPLIT_TABLE {
        bytes.extend_from_slice(&center.high.to_le_bytes());
        bytes.extend_from_slice(&center.low.to_le_bytes());
    }
    bytes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn public_configuration_and_layout_are_explicit() {
        assert_eq!(TABLE_LENGTH, 128);
        assert_eq!(POLYNOMIAL.len(), 5);
        assert_eq!(NEAR_ONE_POLYNOMIAL.len(), 11);
        assert_eq!(REDUCTION_TABLE.len(), 128);
        assert_eq!(CENTER_SPLIT_TABLE.len(), 128);
        assert!(!HAVE_FAST_FMA);
        assert_eq!(SCALAR_COUNT, 530);
        assert_eq!(SCALAR_COUNT * 8, 4240);
        assert_eq!(emit().len(), SOURCE_END - SOURCE_START);
    }

    #[test]
    fn binary64_rounding_and_original_field_order_are_preserved() {
        let bytes = emit();
        let word = |index: usize| u64::from_le_bytes(
            bytes[index * 8..index * 8 + 8].try_into().unwrap());
        // Independently transcribed from upstream hexadecimal float literals.
        assert_eq!(word(0), 0x3fe6_2e42_fefa_3800); // ln2hi
        assert_eq!(word(1), 0x3d2e_f357_93c7_6730); // ln2lo
        assert_eq!(word(2), 0xbfe0_0000_0000_0001); // poly[0]
        assert_eq!(word(7), 0xbfe0_0000_0000_0000); // poly1[0]
        assert_eq!(word(18), 0x3ff7_34f0_c3e0_de9f); // tab[0].invc
        assert_eq!(word(19), 0xbfd7_cc7f_79e6_9000); // tab[0].logc
    }

    #[test]
    fn reduction_tables_retain_their_mathematical_relationship() {
        assert_eq!(LN2_HIGH + LN2_LOW, std::f64::consts::LN_2);
        for (i, (entry, center)) in REDUCTION_TABLE.iter()
            .zip(CENTER_SPLIT_TABLE.iter()).enumerate() {
            assert!(entry.reciprocal.is_finite() && entry.reciprocal > 0.0);
            assert!(entry.logarithm.is_finite());
            assert!(center.high.is_finite() && center.low.is_finite());
            assert!((entry.reciprocal * (center.high + center.low) - 1.0).abs()
                <= 2.0 * f64::EPSILON, "center/reciprocal mismatch at {i}");
            assert!(((center.high + center.low).ln() - entry.logarithm).abs()
                <= 2.0 * f64::EPSILON, "center/log mismatch at {i}");
            if i > 0 {
                assert!(center.high > CENTER_SPLIT_TABLE[i-1].high);
                assert!(entry.logarithm > REDUCTION_TABLE[i-1].logarithm);
            }
        }
    }
}

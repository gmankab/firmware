use arm32::{
    branch, branch_exchange_immediate, data_immediate, data_immediate_with_flags, data_register, data_register_shifted_by_register, load_halfword, load_halfword_postindexed, multiply_accumulate, multiply_long, multiply_register, store_halfword, store_halfword_postindexed, transfer_byte, transfer_byte_postindexed, transfer_byte_register, transfer_byte_register_preindexed_writeback, transfer_doubleword, transfer_halfword_register, transfer_multiple, transfer_word, transfer_word_postindexed, transfer_word_preindexed_writeback, transfer_word_register, transfer_word_register_preindexed_writeback,
    BlockMode, Condition as C, DataOp as Op, Register as R, Shift,
};

pub const START: usize = 0x76610;
pub const END: usize = 0x79214;
pub const FILE_MAPPING_DELTA: usize = 0x4000;
pub const CODE_BYTES: usize = 11068;
pub const DATA_BYTES: usize = 200;

fn r(number: u8) -> R { R::new(number).unwrap() }

fn data_value(at: usize) -> Option<u32> {
    match at {
        0x766e4 => Some(0x7674c),
        0x766e8 => Some(0x78f8c),
        0x766ec => Some(0x766fc),
        0x766f0 => Some(0x766fc),
        0x766f4 => Some(0x766fc),
        0x766f8 => Some(0x78fc8),
        0x767b4 => Some(0x767c8),
        0x767b8 => Some(0x76804),
        0x767bc => Some(0x76864),
        0x767c0 => Some(0x76920),
        0x767c4 => Some(0x7910c),
        0x774d0 => Some(0xf3ce9),
        0x7763c => Some(0x221004),
        0x77a58 => Some(0x1fe001),
        0x77bdc => Some(0x88888889),
        0x77be0 => Some(0x77c08),
        0x77be4 => Some(0x77cf4),
        0x77be8 => Some(0x77d98),
        0x77bec => Some(0x78034),
        0x77bf0 => Some(0x78170),
        0x77bf4 => Some(0x77c08),
        0x77bf8 => Some(0x77c08),
        0x77bfc => Some(0x77881),
        0x77c00 => Some(0xf3f9a),
        0x77c04 => Some(0xf3ffe),
        0x77cf0 => Some(0xf3e2c),
        0x77d94 => Some(0xf417b),
        0x77e4c => Some(0xf4016),
        0x77e50 => Some(0xe2788),
        0x78030 => Some(0xf407b),
        0x78168 => Some(0xf40b0),
        0x7816c => Some(0x3008001),
        0x781f4 => Some(0xe00e0008),
        0x78318 => Some(0xf3ed2),
        0x7831c => Some(0xf3f42),
        0x783a8 => Some(0xf3f71),
        0x787b4 => Some(0xf3e7e),
        0x78ce4 => Some(0xf40ff7ff),
        0x78ce8 => Some(0xf7fff7ff),
        0x791e8 => Some(0xf3ce9),
        0x791ec => Some(0x10ceac),
        0x791f0 => Some(0xf417b),
        0x791f4 => Some(0xe2788),
        0x791f8 => Some(0xf4145),
        0x791fc => Some(0xec034),
        0x79200 => Some(0xf41c3),
        0x79204 => Some(0xf40f9),
        0x79208 => Some(0xf412c),
        0x7920c => Some(0xf3f59),
        0x79210 => Some(0xf3e08),
        _ => None,
    }
}

fn code_address(at: usize) -> bool {
    (START..END).contains(&at) && at % 4 == 0 && data_value(at).is_none()
}

struct TypedA32 {
    at: usize,
    bytes: Vec<u8>,
}

impl TypedA32 {
    fn check(&self, expected: usize) -> Result<(), String> {
        if self.at != expected {
            return Err(format!("worker address {:#x}, expected {expected:#x}", self.at));
        }
        Ok(())
    }

    fn emit(&mut self, instruction: u32) -> Result<(), String> {
        if !code_address(self.at) {
            return Err(format!("invalid worker code address {:#x}", self.at));
        }
        self.bytes.extend_from_slice(&instruction.to_le_bytes());
        self.at += 4;
        Ok(())
    }

    fn data(&mut self, at: usize, value: u32) -> Result<(), String> {
        self.check(at)?;
        if at % 4 != 0 || data_value(at) != Some(value) {
            return Err(format!("invalid worker data at {at:#x}"));
        }
        self.bytes.extend_from_slice(&value.to_le_bytes());
        self.at += 4;
        Ok(())
    }

    fn literal(&mut self, condition: C, destination: R, target: usize) -> Result<(), String> {
        if data_value(target).is_none() {
            return Err(format!("invalid worker literal {target:#x}"));
        }
        self.emit(transfer_word(condition, true, destination, R::PC,
            target as i32 - self.at as i32 - 8)?)
    }

    fn branch(&mut self, condition: C, target: usize, link: bool) -> Result<(), String> {
        let valid = match self.at {
            0x76638 => condition == C::Al && target == 0x17b0 && link == true,
            0x76658 => condition == C::Al && target == 0x17b0 && link == true,
            0x76668 => condition == C::Ge && target == 0x76684 && link == false,
            0x76670 => condition == C::Eq && target == 0x76698 && link == false,
            0x76678 => condition == C::Ne && target == 0x766a8 && link == false,
            0x76680 => condition == C::Al && target == 0x766b4 && link == false,
            0x76684 => condition == C::Eq && target == 0x766a0 && link == false,
            0x7668c => condition == C::Ne && target == 0x766a8 && link == false,
            0x76694 => condition == C::Al && target == 0x766b4 && link == false,
            0x7669c => condition == C::Al && target == 0x766b4 && link == false,
            0x766a4 => condition == C::Al && target == 0x766b4 && link == false,
            0x766bc => condition == C::Al && target == 0x3084 && link == true,
            0x766c4 => condition == C::Al && target == 0x293c && link == true,
            0x766d4 => condition == C::Hi && target == 0x791c8 && link == false,
            0x7671c => condition == C::Ne && target == 0x791c8 && link == false,
            0x76728 => condition == C::Ne && target == 0x79020 && link == false,
            0x76734 => condition == C::Al && target == 0x73924 && link == true,
            0x7673c => condition == C::Ne && target == 0x79030 && link == false,
            0x76744 => condition == C::Al && target == 0x5ab4c && link == true,
            0x76748 => condition == C::Al && target == 0x7907c && link == false,
            0x767a4 => condition == C::Al && target == 0x78f7c && link == false,
            0x767e0 => condition == C::Ne && target == 0x769cc && link == false,
            0x767e8 => condition == C::Ls && target == 0x767f8 && link == false,
            0x767f4 => condition == C::Eq && target == 0x76c68 && link == false,
            0x76800 => condition == C::Al && target == 0x78f7c && link == false,
            0x76814 => condition == C::Eq && target == 0x76830 && link == false,
            0x7682c => condition == C::Ls && target == 0x769e0 && link == false,
            0x76838 => condition == C::Al && target == 0x35d44 && link == true,
            0x76844 => condition == C::Eq && target == 0x79148 && link == false,
            0x76850 => condition == C::Ne && target == 0x76a00 && link == false,
            0x76860 => condition == C::Al && target == 0x777f4 && link == false,
            0x76878 => condition == C::Eq && target == 0x772d8 && link == false,
            0x76888 => condition == C::Eq && target == 0x772d8 && link == false,
            0x768bc => condition == C::Al && target == 0x58f60 && link == true,
            0x768c8 => condition == C::Eq && target == 0x76908 && link == false,
            0x768d0 => condition == C::Al && target == 0x5886c && link == true,
            0x768d8 => condition == C::Ne && target == 0x768e8 && link == false,
            0x768e4 => condition == C::Eq && target == 0x76908 && link == false,
            0x76914 => condition == C::Ne && target == 0x76a64 && link == false,
            0x7691c => condition == C::Al && target == 0x76ae0 && link == false,
            0x76930 => condition == C::Al && target == 0x35c14 && link == true,
            0x7693c => condition == C::Eq && target == 0x769ec && link == false,
            0x76964 => condition == C::Ne && target == 0x76dc0 && link == false,
            0x76984 => condition == C::Al && target == 0x3578c && link == true,
            0x7698c => condition == C::Eq && target == 0x76c60 && link == false,
            0x769a4 => condition == C::Al && target == 0x356e8 && link == true,
            0x769ac => condition == C::Eq && target == 0x76d74 && link == false,
            0x769c8 => condition == C::Al && target == 0x76db0 && link == false,
            0x769d0 => condition == C::Ls && target == 0x769e0 && link == false,
            0x769dc => condition == C::Eq && target == 0x76d38 && link == false,
            0x769e8 => condition == C::Al && target == 0x78f7c && link == false,
            0x769f8 => condition == C::Al && target == 0x17b0 && link == true,
            0x769fc => condition == C::Al && target == 0x78ef0 && link == false,
            0x76a1c => condition == C::Ne && target == 0x777f0 && link == false,
            0x76a28 => condition == C::Eq && target == 0x76a3c && link == false,
            0x76a38 => condition == C::Eq && target == 0x777d8 && link == false,
            0x76a4c => condition == C::Eq && target == 0x777f0 && link == false,
            0x76a5c => condition == C::Al && target == 0x71790 && link == true,
            0x76a60 => condition == C::Al && target == 0x777f0 && link == false,
            0x76a7c => condition == C::Eq && target == 0x76a9c && link == false,
            0x76ad8 => condition == C::Al && target == 0x38be4 && link == true,
            0x76ae8 => condition == C::Eq && target == 0x76b40 && link == false,
            0x76af8 => condition == C::Eq && target == 0x76b40 && link == false,
            0x76b14 => condition == C::Eq && target == 0x76b38 && link == false,
            0x76b44 => condition == C::Eq && target == 0x76b84 && link == false,
            0x76b54 => condition == C::Eq && target == 0x76b84 && link == false,
            0x76b74 => condition == C::Al && target == 0x6b860 && link == true,
            0x76b88 => condition == C::Eq && target == 0x76f84 && link == false,
            0x76bac => condition == C::Eq && target == 0x76bc4 && link == false,
            0x76bc0 => condition == C::Al && target == 0x6b8a0 && link == true,
            0x76c14 => condition == C::Ne && target == 0x76f84 && link == false,
            0x76c3c => condition == C::Cc && target == 0x76f54 && link == false,
            0x76c48 => condition == C::Ne && target == 0x76f84 && link == false,
            0x76c5c => condition == C::Al && target == 0x76f70 && link == false,
            0x76c64 => condition == C::Al && target == 0x78ef0 && link == false,
            0x76c70 => condition == C::Al && target == 0x35d44 && link == true,
            0x76c7c => condition == C::Eq && target == 0x79148 && link == false,
            0x76ca4 => condition == C::Ne && target == 0x76d0c && link == false,
            0x76cb8 => condition == C::Eq && target == 0x76ccc && link == false,
            0x76cc8 => condition == C::Al && target == 0x715ec && link == true,
            0x76cd4 => condition == C::Eq && target == 0x76ce8 && link == false,
            0x76ce4 => condition == C::Al && target == 0x61b5c && link == true,
            0x76cf0 => condition == C::Ne && target == 0x76d0c && link == false,
            0x76cfc => condition == C::Eq && target == 0x76d0c && link == false,
            0x76d08 => condition == C::Al && target == 0x69e44 && link == true,
            0x76d1c => condition == C::Al && target == 0x35df4 && link == true,
            0x76d28 => condition == C::Eq && target == 0x78f7c && link == false,
            0x76d34 => condition == C::Al && target == 0x77818 && link == false,
            0x76d40 => condition == C::Al && target == 0x35d44 && link == true,
            0x76d4c => condition == C::Eq && target == 0x79148 && link == false,
            0x76d58 => condition == C::Ne && target == 0x776e0 && link == false,
            0x76d70 => condition == C::Al && target == 0x77770 && link == false,
            0x76d88 => condition == C::Eq && target == 0x76db0 && link == false,
            0x76d94 => condition == C::Al && target == 0x34b24 && link == true,
            0x76da4 => condition == C::Al && target == 0x356e8 && link == true,
            0x76dac => condition == C::Ne && target == 0x76d8c && link == false,
            0x76dd8 => condition == C::Eq && target == 0x76e0c && link == false,
            0x76e00 => condition == C::Al && target == 0x2ec94 && link == true,
            0x76e08 => condition == C::Al && target == 0x78e34 && link == false,
            0x76e14 => condition == C::Ne && target == 0x78cf8 && link == false,
            0x76e20 => condition == C::Eq && target == 0x76e70 && link == false,
            0x76e2c => condition == C::Al && target == 0x802a4 && link == true,
            0x76e38 => condition == C::Cc && target == 0x78ef4 && link == false,
            0x76e4c => condition == C::Al && target == 0x7f990 && link == true,
            0x76e58 => condition == C::Eq && target == 0x78f24 && link == false,
            0x76e60 => condition == C::Al && target == 0x8025c && link == true,
            0x76e78 => condition == C::Ne && target == 0x78cf8 && link == false,
            0x76e84 => condition == C::Ne && target == 0x78cf8 && link == false,
            0x76e94 => condition == C::Eq && target == 0x77b5c && link == false,
            0x76e9c => condition == C::Al && target == 0x802a4 && link == true,
            0x76eac => condition == C::Cc && target == 0x78ef4 && link == false,
            0x76ec8 => condition == C::Al && target == 0x81850 && link == true,
            0x76ee0 => condition == C::Eq && target == 0x78cec && link == false,
            0x76f08 => condition == C::Al && target == 0x7f990 && link == true,
            0x76f34 => condition == C::Cc && target == 0x76ef8 && link == false,
            0x76f50 => condition == C::Al && target == 0x78cf0 && link == false,
            0x76f5c => condition == C::Ne && target == 0x76f84 && link == false,
            0x76f74 => condition == C::Al && target == 0x2ec94 && link == true,
            0x76f80 => condition == C::Al && target == 0x2ec94 && link == true,
            0x76f90 => condition == C::Eq && target == 0x76fcc && link == false,
            0x76fa8 => condition == C::Ne && target == 0x76fd4 && link == false,
            0x76fb8 => condition == C::Eq && target == 0x776ac && link == false,
            0x76fc8 => condition == C::Al && target == 0x76ffc && link == false,
            0x76fd0 => condition == C::Al && target == 0x7701c && link == false,
            0x76fec => condition == C::Al && target == 0x17b0 && link == true,
            0x77020 => condition == C::Eq && target == 0x77080 && link == false,
            0x77030 => condition == C::Eq && target == 0x77080 && link == false,
            0x7704c => condition == C::Eq && target == 0x77074 && link == false,
            0x77098 => condition == C::Eq && target == 0x770b0 && link == false,
            0x770b8 => condition == C::Eq && target == 0x771f8 && link == false,
            0x770c8 => condition == C::Eq && target == 0x77148 && link == false,
            0x770f4 => condition == C::Al && target == 0x2ec94 && link == true,
            0x77100 => condition == C::Al && target == 0x2ec94 && link == true,
            0x77144 => condition == C::Al && target == 0x771c8 && link == false,
            0x77154 => condition == C::Eq && target == 0x77164 && link == false,
            0x77160 => condition == C::Eq && target == 0x7717c && link == false,
            0x7716c => condition == C::Ne && target == 0x7717c && link == false,
            0x77178 => condition == C::Eq && target == 0x771c8 && link == false,
            0x7718c => condition == C::Al && target == 0x2ec94 && link == true,
            0x77198 => condition == C::Al && target == 0x2ec94 && link == true,
            0x771c4 => condition == C::Al && target == 0x2a990 && link == true,
            0x771d8 => condition == C::Al && target == 0x54c1c && link == true,
            0x77208 => condition == C::Eq && target == 0x772d8 && link == false,
            0x77228 => condition == C::Cc && target == 0x77290 && link == false,
            0x77234 => condition == C::Eq && target == 0x77290 && link == false,
            0x77298 => condition == C::Eq && target == 0x772cc && link == false,
            0x772a4 => condition == C::Ne && target == 0x772cc && link == false,
            0x772b4 => condition == C::Ne && target == 0x772d0 && link == false,
            0x772c8 => condition == C::Al && target == 0x772d0 && link == false,
            0x772d8 => condition == C::Al && target == 0x2f8ac && link == true,
            0x772ec => condition == C::Eq && target == 0x77390 && link == false,
            0x77310 => condition == C::Cc && target == 0x77328 && link == false,
            0x7732c => condition == C::Hi && target == 0x77340 && link == false,
            0x77334 => condition == C::Ne && target == 0x77340 && link == false,
            0x7733c => condition == C::Ne && target == 0x77390 && link == false,
            0x77360 => condition == C::Al && target == 0x2a990 && link == true,
            0x77380 => condition == C::Al && target == 0x2ec94 && link == true,
            0x7738c => condition == C::Al && target == 0x2ec94 && link == true,
            0x7739c => condition == C::Ne && target == 0x773ac && link == false,
            0x773a4 => condition == C::Al && target == 0x62dd0 && link == true,
            0x773a8 => condition == C::Al && target == 0x773b4 && link == false,
            0x773b0 => condition == C::Al && target == 0x644d4 && link == true,
            0x773bc => condition == C::Ne && target == 0x7742c && link == false,
            0x773e0 => condition == C::Al && target == 0x356e8 && link == true,
            0x773f0 => condition == C::Eq && target == 0x77448 && link == false,
            0x773fc => condition == C::Eq && target == 0x787cc && link == false,
            0x77408 => condition == C::Eq && target == 0x78798 && link == false,
            0x77410 => condition == C::Eq && target == 0x787a0 && link == false,
            0x77418 => condition == C::Eq && target == 0x78790 && link == false,
            0x77428 => condition == C::Al && target == 0x787c4 && link == false,
            0x77434 => condition == C::Al && target == 0x2ec94 && link == true,
            0x77440 => condition == C::Al && target == 0x2ec94 && link == true,
            0x77444 => condition == C::Al && target == 0x78a98 && link == false,
            0x77458 => condition == C::Ne && target == 0x774c4 && link == false,
            0x77460 => condition == C::Al && target == 0x623c8 && link == true,
            0x774b0 => condition == C::Ne && target == 0x774d4 && link == false,
            0x774bc => condition == C::Al && target == 0x7df7c && link == true,
            0x774c0 => condition == C::Al && target == 0x774e8 && link == false,
            0x774c8 => condition == C::Al && target == 0x65e94 && link == true,
            0x774cc => condition == C::Al && target == 0x774f8 && link == false,
            0x774e4 => condition == C::Al && target == 0x17b0 && link == true,
            0x774f0 => condition == C::Al && target == 0x7dc70 && link == true,
            0x77508 => condition == C::Eq && target == 0x77518 && link == false,
            0x77514 => condition == C::Al && target == 0x761e0 && link == true,
            0x77524 => condition == C::Eq && target == 0x776a4 && link == false,
            0x77530 => condition == C::Eq && target == 0x77784 && link == false,
            0x77598 => condition == C::Eq && target == 0x77830 && link == false,
            0x775a0 => condition == C::Al && target == 0x2efc && link == true,
            0x775a8 => condition == C::Eq && target == 0x77840 && link == false,
            0x775d0 => condition == C::Al && target == 0x17b0 && link == true,
            0x775f0 => condition == C::Al && target == 0x17b0 && link == true,
            0x775f8 => condition == C::Eq && target == 0x77968 && link == false,
            0x77618 => condition == C::Al && target == 0x81850 && link == true,
            0x77620 => condition == C::Eq && target == 0x77640 && link == false,
            0x77638 => condition == C::Al && target == 0x77698 && link == false,
            0x77648 => condition == C::Cs && target == 0x77694 && link == false,
            0x77654 => condition == C::Eq && target == 0x77694 && link == false,
            0x77688 => condition == C::Cc && target == 0x77668 && link == false,
            0x77690 => condition == C::Al && target == 0x77698 && link == false,
            0x7769c => condition == C::Cc && target == 0x77608 && link == false,
            0x776a0 => condition == C::Al && target == 0x77970 && link == false,
            0x776a8 => condition == C::Al && target == 0x78248 && link == false,
            0x776c0 => condition == C::Eq && target == 0x77000 && link == false,
            0x776d8 => condition == C::Al && target == 0x6b8c8 && link == true,
            0x776dc => condition == C::Al && target == 0x77000 && link == false,
            0x776fc => condition == C::Ne && target == 0x77764 && link == false,
            0x77710 => condition == C::Eq && target == 0x77724 && link == false,
            0x77720 => condition == C::Al && target == 0x71328 && link == true,
            0x7772c => condition == C::Eq && target == 0x77740 && link == false,
            0x7773c => condition == C::Al && target == 0x61b5c && link == true,
            0x77748 => condition == C::Ne && target == 0x77764 && link == false,
            0x77754 => condition == C::Eq && target == 0x77764 && link == false,
            0x77760 => condition == C::Al && target == 0x69e44 && link == true,
            0x7777c => condition == C::Al && target == 0x35df4 && link == true,
            0x77780 => condition == C::Al && target == 0x78f7c && link == false,
            0x77794 => condition == C::Eq && target == 0x77854 && link == false,
            0x777a8 => condition == C::Ne && target == 0x77a24 && link == false,
            0x777b4 => condition == C::Eq && target == 0x777d0 && link == false,
            0x777bc => condition == C::Al && target == 0x31ec && link == true,
            0x777c8 => condition == C::Al && target == 0x205c && link == true,
            0x777d4 => condition == C::Al && target == 0x77a50 && link == false,
            0x777e0 => condition == C::Al && target == 0x6b38c && link == true,
            0x777ec => condition == C::Eq && target == 0x76a3c && link == false,
            0x77800 => condition == C::Al && target == 0x35df4 && link == true,
            0x7780c => condition == C::Eq && target == 0x78f7c && link == false,
            0x7781c => condition == C::Al && target == 0x2ec94 && link == true,
            0x77828 => condition == C::Al && target == 0x2ec94 && link == true,
            0x7782c => condition == C::Al && target == 0x78f7c && link == false,
            0x7783c => condition == C::Al && target == 0x77970 && link == false,
            0x7784c => condition == C::Al && target == 0x17b0 && link == true,
            0x77850 => condition == C::Al && target == 0x78248 && link == false,
            0x7787c => condition == C::Eq && target == 0x778a8 && link == false,
            0x778a4 => condition == C::Ne && target == 0x7788c && link == false,
            0x778c0 => condition == C::Ne && target == 0x78248 && link == false,
            0x778d0 => condition == C::Eq && target == 0x78b84 && link == false,
            0x778d8 => condition == C::Ne && target == 0x78c2c && link == false,
            0x778ec => condition == C::Eq && target == 0x78f3c && link == false,
            0x77960 => condition == C::Ne && target == 0x77914 && link == false,
            0x77964 => condition == C::Al && target == 0x78f40 && link == false,
            0x7797c => condition == C::Cc && target == 0x77bb4 && link == false,
            0x7798c => condition == C::Al && target == 0x31ec && link == true,
            0x7799c => condition == C::Eq && target == 0x77a5c && link == false,
            0x779a4 => condition == C::Ne && target == 0x77b44 && link == false,
            0x779b0 => condition == C::Eq && target == 0x77bb4 && link == false,
            0x779c4 => condition == C::Cs && target == 0x77b80 && link == false,
            0x779d4 => condition == C::Al && target == 0x81850 && link == true,
            0x77a08 => condition == C::Ne && target == 0x779c8 && link == false,
            0x77a20 => condition == C::Al && target == 0x77b84 && link == false,
            0x77a48 => condition == C::Al && target == 0x17b0 && link == true,
            0x77a54 => condition == C::Al && target == 0x78248 && link == false,
            0x77a64 => condition == C::Eq && target == 0x77bb4 && link == false,
            0x77a70 => condition == C::Eq && target == 0x77af0 && link == false,
            0x77a7c => condition == C::Eq && target == 0x77ae8 && link == false,
            0x77a94 => condition == C::Ne && target == 0x77a88 && link == false,
            0x77aa0 => condition == C::Al && target == 0x83d84 && link == true,
            0x77aac => condition == C::Al && target == 0x81838 && link == true,
            0x77ad8 => condition == C::Cc && target == 0x77aa0 && link == false,
            0x77b00 => condition == C::Cs && target == 0x77b98 && link == false,
            0x77b10 => condition == C::Al && target == 0x81850 && link == true,
            0x77b2c => condition == C::Ne && target == 0x77b08 && link == false,
            0x77b40 => condition == C::Al && target == 0x77b9c && link == false,
            0x77b50 => condition == C::Al && target == 0x17b0 && link == true,
            0x77b58 => condition == C::Al && target == 0x77bb4 && link == false,
            0x77b64 => condition == C::Al && target == 0x7f990 && link == true,
            0x77b70 => condition == C::Eq && target == 0x78ef0 && link == false,
            0x77b78 => condition == C::Al && target == 0x8025c && link == true,
            0x77b7c => condition == C::Al && target == 0x78cf8 && link == false,
            0x77b8c => condition == C::Al && target == 0x81850 && link == true,
            0x77b94 => condition == C::Al && target == 0x77bb0 && link == false,
            0x77ba4 => condition == C::Al && target == 0x81850 && link == true,
            0x77bc0 => condition == C::Hi && target == 0x77ce8 && link == false,
            0x77c10 => condition == C::Eq && target == 0x78224 && link == false,
            0x77c34 => condition == C::Eq && target == 0x77cbc && link == false,
            0x77c44 => condition == C::Eq && target == 0x77cbc && link == false,
            0x77cb8 => condition == C::Al && target == 0x77ccc && link == false,
            0x77ce0 => condition == C::Ne && target == 0x77c30 && link == false,
            0x77cec => condition == C::Al && target == 0x7815c && link == false,
            0x77cfc => condition == C::Eq && target == 0x78224 && link == false,
            0x77d8c => condition == C::Ne && target == 0x77d0c && link == false,
            0x77d90 => condition == C::Al && target == 0x78224 && link == false,
            0x77da0 => condition == C::Eq && target == 0x78224 && link == false,
            0x77dd4 => condition == C::Eq && target == 0x77e18 && link == false,
            0x77ddc => condition == C::Ne && target == 0x77e54 && link == false,
            0x77e14 => condition == C::Al && target == 0x77f58 && link == false,
            0x77e48 => condition == C::Al && target == 0x77f58 && link == false,
            0x78028 => condition == C::Ne && target == 0x77dbc && link == false,
            0x7802c => condition == C::Al && target == 0x78224 && link == false,
            0x78050 => condition == C::Eq && target == 0x78118 && link == false,
            0x78114 => condition == C::Ne && target == 0x78060 && link == false,
            0x78128 => condition == C::Ne && target == 0x78140 && link == false,
            0x7813c => condition == C::Ne && target == 0x78224 && link == false,
            0x78150 => condition == C::Al && target == 0x17b0 && link == true,
            0x78160 => condition == C::Ne && target == 0x78224 && link == false,
            0x78164 => condition == C::Al && target == 0x78248 && link == false,
            0x78178 => condition == C::Eq && target == 0x78224 && link == false,
            0x78190 => condition == C::Eq && target == 0x781f8 && link == false,
            0x781f0 => condition == C::Al && target == 0x78208 && link == false,
            0x78220 => condition == C::Ne && target == 0x7818c && link == false,
            0x78228 => condition == C::Al && target == 0x2f1c && link == true,
            0x78234 => condition == C::Al && target == 0x205c && link == true,
            0x78244 => condition == C::Al && target == 0x205c && link == true,
            0x78290 => condition == C::Eq && target == 0x782a0 && link == false,
            0x7829c => condition == C::Eq && target == 0x78380 && link == false,
            0x782e8 => condition == C::Ls && target == 0x78320 && link == false,
            0x782fc => condition == C::Cs && target == 0x78490 && link == false,
            0x78314 => condition == C::Al && target == 0x7848c && link == false,
            0x78320 => condition == C::Cs && target == 0x784b0 && link == false,
            0x78338 => condition == C::Cs && target == 0x78490 && link == false,
            0x78344 => condition == C::Cs && target == 0x7847c && link == false,
            0x78370 => condition == C::Al && target == 0x17b0 && link == true,
            0x7837c => condition == C::Al && target == 0x78490 && link == false,
            0x78384 => condition == C::Eq && target == 0x783ac && link == false,
            0x78390 => condition == C::Ne && target == 0x783ac && link == false,
            0x783a4 => condition == C::Al && target == 0x782b0 && link == false,
            0x783dc => condition == C::Lt && target == 0x78454 && link == false,
            0x78400 => condition == C::Al && target == 0x8187c && link == true,
            0x78424 => condition == C::Al && target == 0x17b0 && link == true,
            0x7844c => condition == C::Al && target == 0x17b0 && link == true,
            0x78460 => condition == C::Lt && target == 0x7857c && link == false,
            0x78478 => condition == C::Al && target == 0x78564 && link == false,
            0x784a8 => condition == C::Al && target == 0x8187c && link == true,
            0x784b4 => condition == C::Ls && target == 0x784ec && link == false,
            0x784cc => condition == C::Cs && target == 0x7855c && link == false,
            0x784e8 => condition == C::Al && target == 0x7855c && link == false,
            0x784ec => condition == C::Cs && target == 0x7857c && link == false,
            0x78508 => condition == C::Cs && target == 0x7855c && link == false,
            0x78514 => condition == C::Cs && target == 0x78548 && link == false,
            0x7853c => condition == C::Al && target == 0x17b0 && link == true,
            0x78544 => condition == C::Al && target == 0x7855c && link == false,
            0x78574 => condition == C::Al && target == 0x8187c && link == true,
            0x785c8 => condition == C::Eq && target == 0x7865c && link == false,
            0x785dc => condition == C::Eq && target == 0x78708 && link == false,
            0x785e8 => condition == C::Eq && target == 0x7868c && link == false,
            0x78638 => condition == C::Ne && target == 0x78628 && link == false,
            0x78658 => condition == C::Al && target == 0x786f8 && link == false,
            0x7865c => condition == C::Al && target == 0x2f8d0 && link == true,
            0x78664 => condition == C::Eq && target == 0x78ab0 && link == false,
            0x78670 => condition == C::Ne && target == 0x785cc && link == false,
            0x78680 => condition == C::Ne && target == 0x785cc && link == false,
            0x78688 => condition == C::Al && target == 0x78ab4 && link == false,
            0x786d0 => condition == C::Ne && target == 0x786c0 && link == false,
            0x78710 => condition == C::Eq && target == 0x78728 && link == false,
            0x78720 => condition == C::Al && target == 0x503e4 && link == true,
            0x78724 => condition == C::Al && target == 0x78748 && link == false,
            0x78734 => condition == C::Al && target == 0x4ff84 && link == true,
            0x78744 => condition == C::Al && target == 0x502b8 && link == true,
            0x78750 => condition == C::Eq && target == 0x78768 && link == false,
            0x7875c => condition == C::Ne && target == 0x78768 && link == false,
            0x78764 => condition == C::Al && target == 0x617ec && link == true,
            0x78770 => condition == C::Eq && target == 0x787ac && link == false,
            0x7877c => condition == C::Eq && target == 0x78798 && link == false,
            0x78784 => condition == C::Eq && target == 0x787a0 && link == false,
            0x7878c => condition == C::Ne && target == 0x787b8 && link == false,
            0x78794 => condition == C::Al && target == 0x787a4 && link == false,
            0x7879c => condition == C::Al && target == 0x787a4 && link == false,
            0x787a8 => condition == C::Al && target == 0x787cc && link == false,
            0x787b0 => condition == C::Al && target == 0x787cc && link == false,
            0x787c8 => condition == C::Al && target == 0x17b0 && link == true,
            0x787f4 => condition == C::Al && target == 0x35c14 && link == true,
            0x78800 => condition == C::Eq && target == 0x791d0 && link == false,
            0x78944 => condition == C::Al && target == 0x471c && link == true,
            0x78954 => condition == C::Ne && target == 0x7896c && link == false,
            0x789d8 => condition == C::Ne && target == 0x789c8 && link == false,
            0x78a00 => condition == C::Ne && target == 0x789f0 && link == false,
            0x78a0c => condition == C::Eq && target == 0x78a1c && link == false,
            0x78aac => condition == C::Al && target == 0x78ef4 && link == false,
            0x78ab8 => condition == C::Ne && target == 0x785cc && link == false,
            0x78ac8 => condition == C::Eq && target == 0x78ae8 && link == false,
            0x78ad4 => condition == C::Al && target == 0x3701c && link == true,
            0x78ae4 => condition == C::Al && target == 0x78aec && link == false,
            0x78af4 => condition == C::Al && target == 0x2ec94 && link == true,
            0x78b00 => condition == C::Al && target == 0x2ec94 && link == true,
            0x78b10 => condition == C::Cc && target == 0x78b34 && link == false,
            0x78b1c => condition == C::Eq && target == 0x78c60 && link == false,
            0x78b30 => condition == C::Al && target == 0x78c64 && link == false,
            0x78b80 => condition == C::Al && target == 0x78cc8 && link == false,
            0x78b94 => condition == C::Eq && target == 0x78f4c && link == false,
            0x78c24 => condition == C::Ne && target == 0x78bc4 && link == false,
            0x78c28 => condition == C::Al && target == 0x78f50 && link == false,
            0x78c44 => condition == C::Al && target == 0x2ec94 && link == true,
            0x78c50 => condition == C::Al && target == 0x2ec94 && link == true,
            0x78c5c => condition == C::Al && target == 0x78f60 && link == false,
            0x78ce0 => condition == C::Al && target == 0x785cc && link == false,
            0x78d00 => condition == C::Eq && target == 0x78d1c && link == false,
            0x78d0c => condition == C::Ne && target == 0x78d1c && link == false,
            0x78d18 => condition == C::Cc && target == 0x78d90 && link == false,
            0x78d24 => condition == C::Ne && target == 0x78d90 && link == false,
            0x78d30 => condition == C::Eq && target == 0x78d90 && link == false,
            0x78d50 => condition == C::Al && target == 0x81850 && link == true,
            0x78d64 => condition == C::Al && target == 0x2ec94 && link == true,
            0x78d70 => condition == C::Al && target == 0x2ec94 && link == true,
            0x78d7c => condition == C::Al && target == 0x81850 && link == true,
            0x78d8c => condition == C::Eq && target == 0x78f2c && link == false,
            0x78d98 => condition == C::Ne && target == 0x78e08 && link == false,
            0x78db0 => condition == C::Al && target == 0x3f34c && link == true,
            0x78dcc => condition == C::Ne && target == 0x78e08 && link == false,
            0x78ddc => condition == C::Ne && target == 0x78df0 && link == false,
            0x78de8 => condition == C::Al && target == 0x3f20c && link == true,
            0x78dec => condition == C::Al && target == 0x78dfc && link == false,
            0x78df8 => condition == C::Al && target == 0x3ee98 && link == true,
            0x78e2c => condition == C::Al && target == 0x2ec94 && link == true,
            0x78e38 => condition == C::Al && target == 0x2ec94 && link == true,
            0x78e60 => condition == C::Al && target == 0x35ca0 && link == true,
            0x78e78 => condition == C::Ne && target == 0x78eac && link == false,
            0x78ea0 => condition == C::Al && target == 0x80928 && link == true,
            0x78ea8 => condition == C::Ne && target == 0x78f00 && link == false,
            0x78eb4 => condition == C::Al && target == 0x292c && link == true,
            0x78eec => condition == C::Al && target == 0x292c && link == true,
            0x78ef8 => condition == C::Eq && target == 0x78f7c && link == false,
            0x78efc => condition == C::Al && target == 0x79148 && link == false,
            0x78f14 => condition == C::Al && target == 0x2ec94 && link == true,
            0x78f20 => condition == C::Al && target == 0x2ec94 && link == true,
            0x78f28 => condition == C::Al && target == 0x78ef4 && link == false,
            0x78f38 => condition == C::Al && target == 0x769f4 && link == false,
            0x78f48 => condition == C::Al && target == 0x78f58 && link == false,
            0x78f64 => condition == C::Al && target == 0x205c && link == true,
            0x78f78 => condition == C::Al && target == 0x78248 && link == false,
            0x78f84 => condition == C::Ls && target == 0x767a8 && link == false,
            0x78f88 => condition == C::Al && target == 0x79148 && link == false,
            0x78fac => condition == C::Ne && target == 0x791c8 && link == false,
            0x78fc4 => condition == C::Al && target == 0x292c && link == false,
            0x78fe8 => condition == C::Ne && target == 0x791c8 && link == false,
            0x79010 => condition == C::Al && target == 0x2a878 && link == true,
            0x7901c => condition == C::Al && target == 0x791c0 && link == false,
            0x7902c => condition == C::Al && target == 0x34fac && link == false,
            0x79038 => condition == C::Ne && target == 0x79044 && link == false,
            0x79040 => condition == C::Al && target == 0x761bc && link == true,
            0x79048 => condition == C::Al && target == 0x74be4 && link == true,
            0x79050 => condition == C::Al && target == 0x79684 && link == true,
            0x79060 => condition == C::Al && target == 0x252a8 && link == true,
            0x79068 => condition == C::Al && target == 0x39bd0 && link == true,
            0x79070 => condition == C::Al && target == 0x5ab4c && link == true,
            0x79078 => condition == C::Al && target == 0x59604 && link == true,
            0x790b0 => condition == C::Ne && target == 0x790c0 && link == false,
            0x790bc => condition == C::Al && target == 0x791c0 && link == false,
            0x790f0 => condition == C::Eq && target == 0x79184 && link == false,
            0x790f8 => condition == C::Eq && target == 0x79180 && link == false,
            0x79100 => condition == C::Ne && target == 0x7918c && link == false,
            0x79108 => condition == C::Al && target == 0x79184 && link == false,
            0x79114 => condition == C::Ne && target == 0x7912c && link == false,
            0x79120 => condition == C::Al && target == 0x35df4 && link == true,
            0x79128 => condition == C::Ne && target == 0x79118 && link == false,
            0x79134 => condition == C::Al && target == 0x35df4 && link == true,
            0x7913c => condition == C::Ne && target == 0x7912c && link == false,
            0x79144 => condition == C::Al && target == 0x79314 && link == true,
            0x79158 => condition == C::Al && target == 0x2ec94 && link == true,
            0x79164 => condition == C::Al && target == 0x2ec94 && link == true,
            0x79170 => condition == C::Eq && target == 0x791c8 && link == false,
            0x79178 => condition == C::Al && target == 0x5373c && link == true,
            0x7917c => condition == C::Al && target == 0x791c8 && link == false,
            0x79188 => condition == C::Al && target == 0x791a0 && link == false,
            0x7919c => condition == C::Al && target == 0x17b0 && link == true,
            0x791a8 => condition == C::Al && target == 0x2a878 && link == true,
            0x791c4 => condition == C::Al && target == 0x292c && link == true,
            0x791e0 => condition == C::Al && target == 0x17b0 && link == true,
            0x791e4 => condition == C::Al && target == 0x79148 && link == false,
            _ => false,
        };
        if !valid || target % 4 != 0 || data_value(target).is_some()
            || (!link && !code_address(target) && !matches!(target, 0x292c | 0x34fac)) {
            return Err(format!("invalid worker branch at {:#x} to {target:#x}", self.at));
        }
        self.emit(branch(condition, self.at as u32, target as u32, link)?)
    }

    fn thumb_call(&mut self, target: usize) -> Result<(), String> {
        let expected = match self.at {
            0x77248 | 0x7725c | 0x7792c | 0x779e4 => 0x83c4c,
            0x78bec => 0x82be4,
            _ => return Err(format!("invalid worker BLX site {:#x}", self.at)),
        };
        if target != expected {
            return Err(format!("invalid worker Thumb target {target:#x}"));
        }
        self.emit(branch_exchange_immediate(self.at as u32, target as u32)?)
    }
}

pub fn fragment() -> Result<Vec<u8>, String> {
    let mut c = TypedA32 { at: START, bytes: Vec::with_capacity(END - START) };
    c.check(START)?;
    c.emit(transfer_multiple(C::Al, false, r(13), 0x4ff0, BlockMode::DecrementBefore, true)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(11), r(13), 0x1c)?)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(13), r(13), 0x8c)?)?;
    c.literal(C::Al, r(5), 0x774d0)?;
    c.emit(data_register(C::Al, Op::Mov, r(8), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x1e)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(8), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x900)?)?;
    c.emit(data_immediate(C::Ne, Op::Mov, r(0), r(0), 0x1)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x17b0, true)?;
    c.emit(transfer_word(C::Al, true, r(4), r(8), 44)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x22)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x900)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(4), 0x0)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(4), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Ne, Op::Mov, r(0), r(0), 0x1)?)?;
    c.branch(C::Al, 0x17b0, true)?;
    c.emit(transfer_word(C::Al, true, r(1), r(8), 644)?)?;
    c.literal(C::Al, r(5), 0x7763c)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x2000)?)?;
    c.branch(C::Ge, 0x76684, false)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x2)?)?;
    c.branch(C::Eq, 0x76698, false)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x1000)?)?;
    c.branch(C::Ne, 0x766a8, false)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0xd)?)?;
    c.branch(C::Al, 0x766b4, false)?;
    c.branch(C::Eq, 0x766a0, false)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x4000)?)?;
    c.branch(C::Ne, 0x766a8, false)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0xe)?)?;
    c.branch(C::Al, 0x766b4, false)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0xb)?)?;
    c.branch(C::Al, 0x766b4, false)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0xc)?)?;
    c.branch(C::Al, 0x766b4, false)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x8000)?)?;
    c.emit(data_immediate(C::Eq, Op::Mov, r(0), r(0), 0xc)?)?;
    c.emit(transfer_doubleword(C::Al, true, r(2), r(8), 16)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(3), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x3084, true)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 20)?)?;
    c.branch(C::Al, 0x293c, true)?;
    c.emit(transfer_word(C::Al, true, r(0), r(8), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(0), r(0), 0x5)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x5)?)?;
    c.branch(C::Hi, 0x791c8, false)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(0), Shift::Lsl, 2, false)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(1), r(15), 0x0)?)?;
    c.emit(transfer_word_register(C::Al, true, r(15), r(0), r(1), Shift::Lsl, 0)?)?;
    c.data(0x766e4, 0x7674c)?;
    c.data(0x766e8, 0x78f8c)?;
    c.data(0x766ec, 0x766fc)?;
    c.data(0x766f0, 0x766fc)?;
    c.data(0x766f4, 0x766fc)?;
    c.data(0x766f8, 0x78fc8)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(8), 12)?)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(8), 13)?)?;
    c.emit(data_register(C::Al, Op::And, r(0), r(1), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(8), 14)?)?;
    c.emit(data_register(C::Al, Op::And, r(0), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_byte(C::Al, false, r(0), r(11), -56)?)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(11), -56)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x1)?)?;
    c.branch(C::Ne, 0x791c8, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(8), 280)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x1)?)?;
    c.branch(C::Ne, 0x79020, false)?;
    c.emit(transfer_word(C::Al, true, r(6), r(8), 8)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(8), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x73924, true)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(6), 0x2)?)?;
    c.branch(C::Ne, 0x79030, false)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(8), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x5ab4c, true)?;
    c.branch(C::Al, 0x7907c, false)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(8), 0x660)?)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(9), r(11), 0x38)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(0), 0x1000)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(13), 72)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(8), 0x298)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(0), 0xc00)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(13), 60)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(8), 0x128)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(0), 0xc00)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(13), 84)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(8), 0xc8)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(0), 0x1400)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(13), 64)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(8), 0x4d0)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(0), 0x1000)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(11), -80)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(8), 0x5c)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(13), 80)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(8), 0x124)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(13), 76)?)?;
    c.emit(transfer_word(C::Al, false, r(8), r(11), -60)?)?;
    c.emit(transfer_word(C::Al, false, r(4), r(11), -68)?)?;
    c.branch(C::Al, 0x78f7c, false)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(0), Shift::Lsl, 2, false)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(1), r(15), 0x0)?)?;
    c.emit(transfer_word_register(C::Al, true, r(15), r(0), r(1), Shift::Lsl, 0)?)?;
    c.data(0x767b4, 0x767c8)?;
    c.data(0x767b8, 0x76804)?;
    c.data(0x767bc, 0x76864)?;
    c.data(0x767c0, 0x76920)?;
    c.data(0x767c4, 0x7910c)?;
    c.emit(transfer_word(C::Al, true, r(7), r(8), 44)?)?;
    c.emit(transfer_byte(C::Al, true, r(2), r(8), 990)?)?;
    c.emit(transfer_word(C::Al, true, r(5), r(7), 24)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(7), 412)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(2), 0x1)?)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(5), 351)?)?;
    c.branch(C::Ne, 0x769cc, false)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.branch(C::Ls, 0x767f8, false)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(7), 7)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Eq, 0x76c68, false)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(7), 0)?)?;
    c.branch(C::Al, 0x78f7c, false)?;
    c.emit(transfer_word(C::Al, true, r(7), r(8), 44)?)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(7), 368)?)?;
    c.emit(transfer_word(C::Al, true, r(5), r(7), 24)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x1)?)?;
    c.branch(C::Eq, 0x76830, false)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(5), 351)?)?;
    c.emit(transfer_byte(C::Al, true, r(2), r(5), 352)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(7), 412)?)?;
    c.emit(data_register(C::Al, Op::Add, r(1), r(2), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.branch(C::Ls, 0x769e0, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(8), 52)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(0), 0x10)?)?;
    c.branch(C::Al, 0x35d44, true)?;
    c.emit(data_register(C::Al, Op::Mov, r(6), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(6), 0x0)?)?;
    c.branch(C::Eq, 0x79148, false)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(7), 368)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x1)?)?;
    c.branch(C::Ne, 0x76a00, false)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(6), 9)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Ne, Op::Mov, r(0), r(0), 0x4)?)?;
    c.branch(C::Al, 0x777f4, false)?;
    c.emit(transfer_word(C::Al, true, r(9), r(8), 44)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(4), r(0), 0x0)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 476)?)?;
    c.emit(transfer_word(C::Al, true, r(10), r(9), 24)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Eq, 0x772d8, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(11), -60)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(0), 296)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Eq, 0x772d8, false)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(4), r(0), r(10), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, true, r(10), r(9), 32)?)?;
    c.emit(transfer_word(C::Al, true, r(6), r(9), 36)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(13), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x264)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(4), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(0), r(0), 0x400)?)?;
    c.emit(transfer_word(C::Al, true, r(3), r(13), 80)?)?;
    c.emit(data_register(C::Al, Op::Add, r(7), r(9), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(13), 76)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(7), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x58f60, true)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(9), 1636)?)?;
    c.emit(data_immediate(C::Al, Op::Tst, r(0), r(0), 0x2)?)?;
    c.branch(C::Eq, 0x76908, false)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(4), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x5886c, true)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Ne, 0x768e8, false)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(4), 281)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Eq, 0x76908, false)?;
    c.emit(transfer_word(C::Al, true, r(1), r(6), 92)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 132)?)?;
    c.emit(data_immediate(C::Al, Op::Rsb, r(2), r(1), 0x0)?)?;
    c.emit(data_register(C::Al, Op::And, r(0), r(0), r(2), Shift::Lsl, 0, true)?)?;
    c.emit(data_register(C::Eq, Op::Mov, r(0), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(4), 132)?)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(0), r(0), 0x1)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(9), 1676)?)?;
    c.emit(transfer_word(C::Al, false, r(4), r(11), -64)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(7), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Tst, r(0), r(0), 0x8000)?)?;
    c.branch(C::Ne, 0x76a64, false)?;
    c.emit(transfer_word(C::Al, true, r(5), r(11), -60)?)?;
    c.branch(C::Al, 0x76ae0, false)?;
    c.emit(transfer_word(C::Al, true, r(7), r(8), 44)?)?;
    c.emit(transfer_word(C::Al, true, r(10), r(7), 24)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(6), r(7), 0x640)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(6), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x35c14, true)?;
    c.emit(data_register(C::Al, Op::Mov, r(5), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(5), 0x0)?)?;
    c.branch(C::Eq, 0x769ec, false)?;
    c.emit(transfer_word(C::Al, false, r(6), r(11), -64)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(6), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(5), 296)?)?;
    c.emit(transfer_word_preindexed_writeback(C::Al, false, r(0), r(2), 272)?)?;
    c.emit(transfer_word_preindexed_writeback(C::Al, false, r(0), r(6), 276)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(8), 280)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x2)?)?;
    c.branch(C::Ne, 0x76dc0, false)?;
    c.emit(transfer_word(C::Al, false, r(2), r(11), -72)?)?;
    c.emit(transfer_word(C::Al, false, r(10), r(11), -76)?)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(10), r(11), 0x20)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(9), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, true, r(8), r(13), 72)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(10), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(8), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x3578c, true)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Eq, 0x76c60, false)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(9), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, true, r(9), r(11), -56)?)?;
    c.emit(transfer_word(C::Al, true, r(4), r(11), -52)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(8), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(10), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x356e8, true)?;
    c.emit(data_register(C::Al, Op::Orr, r(1), r(9), r(4), Shift::Lsl, 0, true)?)?;
    c.branch(C::Eq, 0x76d74, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(11), -52)?)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(9), r(11), 0x38)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(5), 272)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(11), -32)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(5), 200)?)?;
    c.emit(transfer_word(C::Al, true, r(10), r(11), -60)?)?;
    c.branch(C::Al, 0x76db0, false)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.branch(C::Ls, 0x769e0, false)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(7), 7)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Eq, 0x76d38, false)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x2)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(7), 0)?)?;
    c.branch(C::Al, 0x78f7c, false)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0xe8)?)?;
    c.literal(C::Al, r(2), 0x774d0)?;
    c.branch(C::Al, 0x17b0, true)?;
    c.branch(C::Al, 0x78ef0, false)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(6), 3)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.emit(transfer_word(C::Ne, true, r(0), r(7), 36)?)?;
    c.emit(data_immediate(C::Ne, Op::Mov, r(1), r(0), 0x1)?)?;
    c.emit(transfer_byte(C::Ne, false, r(1), r(0), 11)?)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(6), 2)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Ne, 0x777f0, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 104)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Eq, 0x76a3c, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 2104)?)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(0), 224)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Eq, 0x777d8, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 472)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.emit(transfer_word(C::Ne, true, r(0), r(5), 480)?)?;
    c.emit(data_immediate(C::Ne, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Eq, 0x777f0, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(7), 1796)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(6), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x71790, true)?;
    c.branch(C::Al, 0x777f0, false)?;
    c.emit(data_immediate(C::Al, Op::Bic, r(0), r(0), 0x8000)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(9), 1636)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(9), 1780)?)?;
    c.emit(transfer_word(C::Al, true, r(5), r(11), -60)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x2)?)?;
    c.branch(C::Eq, 0x76a9c, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 3444)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(5), 3448)?)?;
    c.emit(multiply_register(C::Al, r(2), r(1), r(0))?)?;
    c.literal(C::Al, r(0), 0x77a58)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(2), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Cc, Op::Mov, r(0), r(0), 0x1)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0xa3)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x200)?)?;
    c.emit(data_immediate(C::Eq, Op::Mov, r(1), r(0), 0x140)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(5), 3352)?)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(5), 3568)?)?;
    c.emit(transfer_word(C::Al, true, r(3), r(13), 84)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x3)?)?;
    c.emit(data_immediate(C::Ne, Op::Mov, r(2), r(0), 0x1)?)?;
    c.emit(data_immediate(C::Ne, Op::Mov, r(1), r(0), 0x4)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(13), 60)?)?;
    c.emit(data_register(C::Eq, Op::Mov, r(1), r(0), r(2), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, true, r(2), r(5), 644)?)?;
    c.branch(C::Al, 0x38be4, true)?;
    c.emit(transfer_word(C::Al, true, r(0), r(7), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(4), r(0), 0x1)?)?;
    c.emit(data_immediate(C::Al, Op::Tst, r(0), r(0), 0x400)?)?;
    c.branch(C::Eq, 0x76b40, false)?;
    c.emit(transfer_word(C::Al, true, r(1), r(11), -64)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(1), 104)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x0)?)?;
    c.branch(C::Eq, 0x76b40, false)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x254)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(10), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(2), r(2), 0x1000)?)?;
    c.emit(transfer_word_register_preindexed_writeback(C::Al, true, r(3), r(1), r(2), Shift::Lsl, 0)?)?;
    c.emit(transfer_word(C::Al, true, r(2), r(9), 1664)?)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(3), r(2), Shift::Lsl, 0, false)?)?;
    c.branch(C::Eq, 0x76b38, false)?;
    c.emit(transfer_word(C::Al, false, r(2), r(1), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x234)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(10), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x1000)?)?;
    c.emit(transfer_word_register_preindexed_writeback(C::Al, true, r(1), r(0), r(1), Shift::Lsl, 0)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x8)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(0), 0)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(7), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Bic, r(0), r(0), 0x400)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(7), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Tst, r(0), r(0), 0x8)?)?;
    c.branch(C::Eq, 0x76b84, false)?;
    c.emit(transfer_word(C::Al, true, r(1), r(11), -64)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(1), 104)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x0)?)?;
    c.branch(C::Eq, 0x76b84, false)?;
    c.emit(transfer_word(C::Al, true, r(2), r(10), 116)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x268)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x234)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(0), r(0), 0x400)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x1000)?)?;
    c.emit(data_register(C::Al, Op::Add, r(0), r(9), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Add, r(1), r(10), r(1), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x6b860, true)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 1636)?)?;
    c.emit(data_immediate(C::Al, Op::Bic, r(0), r(0), 0x8)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(9), 1636)?)?;
    c.emit(data_immediate(C::Al, Op::Tst, r(0), r(0), 0x1)?)?;
    c.branch(C::Eq, 0x76f84, false)?;
    c.emit(transfer_word(C::Al, true, r(1), r(11), -64)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(8), r(0), r(7), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(1), 612)?)?;
    c.emit(data_immediate(C::Al, Op::Tst, r(0), r(0), 0x4)?)?;
    c.emit(transfer_byte(C::Ne, false, r(4), r(6), 3)?)?;
    c.emit(transfer_word(C::Al, false, r(6), r(11), -72)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(1), 104)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Eq, 0x76bc4, false)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x234)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(0), r(0), 0x1000)?)?;
    c.emit(data_register(C::Al, Op::Add, r(1), r(10), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 1672)?)?;
    c.branch(C::Al, 0x6b8a0, true)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 1636)?)?;
    c.emit(data_immediate(C::Al, Op::Bic, r(0), r(0), 0x1)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(9), 1636)?)?;
    c.emit(transfer_word(C::Al, true, r(6), r(11), -64)?)?;
    c.emit(transfer_word(C::Al, true, r(2), r(9), 1672)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(6), 2616)?)?;
    c.literal(C::Al, r(3), 0x77bdc)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(2), Shift::Lsr, 16, false)?)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(1), r(2), Shift::Lsr, 16, false)?)?;
    c.emit(data_register(C::Ls, Op::Mov, r(1), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(2), r(1), 0x1)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x3c)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x1)?)?;
    c.emit(multiply_long(C::Al, false, r(7), r(4), r(2), r(3))?)?;
    c.emit(data_register(C::Hi, Op::Add, r(1), r(1), r(4), Shift::Lsr, 5, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(7), r(0), r(8), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(6), 2612)?)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(6), 200)?)?;
    c.emit(transfer_word(C::Al, true, r(6), r(11), -72)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x1)?)?;
    c.branch(C::Ne, 0x76f84, false)?;
    c.emit(transfer_word(C::Al, true, r(2), r(5), 3444)?)?;
    c.emit(transfer_word(C::Al, true, r(3), r(5), 3448)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(5), 3392)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(2), Shift::Lsr, 4, false)?)?;
    c.emit(multiply_register(C::Al, r(6), r(2), r(0))?)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(3), Shift::Lsr, 4, false)?)?;
    c.emit(multiply_register(C::Al, r(2), r(6), r(0))?)?;
    c.literal(C::Al, r(0), 0x77bfc)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(2), r(0), Shift::Lsl, 0, false)?)?;
    c.branch(C::Cc, 0x76f54, false)?;
    c.emit(transfer_word(C::Al, true, r(6), r(11), -72)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x1)?)?;
    c.branch(C::Ne, 0x76f84, false)?;
    c.literal(C::Al, r(1), 0x77c00)?;
    c.literal(C::Al, r(2), 0x77c04)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x8)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), r(0), 0x42)?)?;
    c.branch(C::Al, 0x76f70, false)?;
    c.emit(transfer_word(C::Al, true, r(8), r(11), -60)?)?;
    c.branch(C::Al, 0x78ef0, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(8), 48)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(0), 0x1c)?)?;
    c.branch(C::Al, 0x35d44, true)?;
    c.emit(data_register(C::Al, Op::Mov, r(6), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(6), 0x0)?)?;
    c.branch(C::Eq, 0x79148, false)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(6), 9)?)?;
    c.emit(transfer_byte(C::Al, false, r(0), r(7), 7)?)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(6), 3)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.emit(transfer_word(C::Ne, true, r(0), r(7), 36)?)?;
    c.emit(data_immediate(C::Ne, Op::Mov, r(1), r(0), 0x1)?)?;
    c.emit(transfer_byte(C::Ne, false, r(1), r(0), 11)?)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(6), 2)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Ne, 0x76d0c, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 472)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.emit(transfer_word(C::Ne, true, r(0), r(5), 480)?)?;
    c.emit(data_immediate(C::Ne, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Eq, 0x76ccc, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(7), 1796)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(6), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x715ec, true)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(5), 188)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Eq, 0x76ce8, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(7), 36)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(6), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x61b5c, true)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(6), 2)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Ne, 0x76d0c, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 104)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Eq, 0x76d0c, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(7), 32)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(6), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x69e44, true)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(7), 0)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(8), 48)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(0), 0x1c)?)?;
    c.branch(C::Al, 0x35df4, true)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(8), 848)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Eq, 0x78f7c, false)?;
    c.emit(transfer_word(C::Al, true, r(2), r(7), 412)?)?;
    c.literal(C::Al, r(1), 0x77cf0)?;
    c.branch(C::Al, 0x77818, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(8), 48)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(0), 0x1c)?)?;
    c.branch(C::Al, 0x35d44, true)?;
    c.emit(data_register(C::Al, Op::Mov, r(6), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(6), 0x0)?)?;
    c.branch(C::Eq, 0x79148, false)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(7), 368)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x1)?)?;
    c.branch(C::Ne, 0x776e0, false)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(6), 9)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Ne, Op::Mov, r(1), r(0), 0x4)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(7), 0)?)?;
    c.branch(C::Al, 0x77770, false)?;
    c.emit(transfer_word(C::Al, true, r(4), r(13), 72)?)?;
    c.emit(transfer_word(C::Al, true, r(10), r(11), -60)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(9), r(11), 0x38)?)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(8), r(11), 0x20)?)?;
    c.branch(C::Eq, 0x76db0, false)?;
    c.emit(transfer_word(C::Al, true, r(1), r(11), -32)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(10), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x34b24, true)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(4), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(9), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(8), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x356e8, true)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Ne, 0x76d8c, false)?;
    c.emit(data_register(C::Al, Op::Mov, r(8), r(0), r(10), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, true, r(4), r(11), -68)?)?;
    c.emit(transfer_word(C::Al, true, r(10), r(11), -76)?)?;
    c.emit(transfer_word(C::Al, true, r(2), r(11), -72)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.emit(transfer_byte(C::Al, false, r(0), r(5), 184)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(10), 300)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(5), 308)?)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(8), 848)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Eq, 0x76e0c, false)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(7), 369)?)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(7), 373)?)?;
    c.emit(transfer_byte(C::Al, true, r(6), r(7), 380)?)?;
    c.emit(transfer_word(C::Al, true, r(3), r(7), 400)?)?;
    c.emit(transfer_word(C::Al, true, r(2), r(7), 416)?)?;
    c.emit(transfer_word(C::Al, false, r(6), r(13), 0)?)?;
    c.emit(transfer_multiple(C::Al, false, r(13), 0x3, BlockMode::IncrementBefore, false)?)?;
    c.literal(C::Al, r(1), 0x77d94)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x4)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x4)?)?;
    c.branch(C::Al, 0x78e34, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(8), 280)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x1)?)?;
    c.branch(C::Ne, 0x78cf8, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(7), 1572)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Eq, 0x76e70, false)?;
    c.emit(transfer_word(C::Al, false, r(2), r(11), -72)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(8), 124)?)?;
    c.branch(C::Al, 0x802a4, true)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x1)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x2)?)?;
    c.branch(C::Cc, 0x78ef4, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(8), 124)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(3), r(5), 0x11c)?)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(1), r(11), 0x20)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(6), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x7f990, true)?;
    c.emit(transfer_word(C::Al, true, r(0), r(11), -32)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Eq, 0x78f24, false)?;
    c.emit(data_immediate(C::Al, Op::Add, r(1), r(5), 0x1b8)?)?;
    c.branch(C::Al, 0x8025c, true)?;
    c.emit(transfer_word(C::Al, true, r(0), r(11), -32)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(5), 436)?)?;
    c.emit(transfer_word(C::Al, true, r(2), r(11), -72)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(2), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Ne, 0x78cf8, false)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(5), 184)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x1)?)?;
    c.branch(C::Ne, 0x78cf8, false)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(8), 1043)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(8), 124)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x0)?)?;
    c.branch(C::Eq, 0x77b5c, false)?;
    c.emit(transfer_word(C::Al, false, r(2), r(11), -72)?)?;
    c.branch(C::Al, 0x802a4, true)?;
    c.emit(transfer_word(C::Al, true, r(6), r(8), 1572)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x1)?)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(0), r(6), Shift::Lsl, 0, false)?)?;
    c.branch(C::Cc, 0x78ef4, false)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(10), 351)?)?;
    c.emit(transfer_byte(C::Al, true, r(2), r(10), 352)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(7), 412)?)?;
    c.emit(data_register(C::Al, Op::Add, r(1), r(1), r(2), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(1), r(1), 0x1)?)?;
    c.emit(data_immediate(C::Al, Op::And, r(1), r(1), 0xff)?)?;
    c.branch(C::Al, 0x81850, true)?;
    c.emit(transfer_word(C::Al, true, r(0), r(8), 48)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(6), 0x0)?)?;
    c.emit(data_register(C::Al, Op::Add, r(0), r(0), r(1), Shift::Lsl, 2, false)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(0), 104)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(5), 300)?)?;
    c.branch(C::Eq, 0x78cec, false)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(5), 0x118)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(9), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Mvn, r(6), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(4), r(0), 0x0)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(11), -76)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(8), 124)?)?;
    c.emit(transfer_word(C::Al, true, r(2), r(11), -72)?)?;
    c.emit(transfer_word(C::Al, true, r(3), r(11), -76)?)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(1), r(11), 0x20)?)?;
    c.branch(C::Al, 0x7f990, true)?;
    c.emit(transfer_word(C::Al, true, r(1), r(5), 272)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 300)?)?;
    c.emit(transfer_word(C::Al, true, r(2), r(11), -72)?)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(1), r(6), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word_register(C::Al, false, r(1), r(0), r(9), Shift::Lsl, 2)?)?;
    c.emit(data_register(C::Cc, Op::Mov, r(6), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(8), 1572)?)?;
    c.emit(data_register(C::Cc, Op::Mov, r(4), r(0), r(9), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(9), r(9), 0x1)?)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(9), r(1), Shift::Lsl, 0, false)?)?;
    c.branch(C::Cc, 0x76ef8, false)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(4), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(9), r(11), 0x38)?)?;
    c.emit(transfer_word(C::Ne, true, r(1), r(0), 0)?)?;
    c.emit(transfer_word_register(C::Ne, false, r(1), r(0), r(4), Shift::Lsl, 2)?)?;
    c.emit(transfer_word(C::Ne, false, r(6), r(0), 0)?)?;
    c.emit(transfer_word(C::Al, true, r(4), r(11), -68)?)?;
    c.branch(C::Al, 0x78cf0, false)?;
    c.emit(transfer_word(C::Al, true, r(6), r(11), -72)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x2)?)?;
    c.branch(C::Ne, 0x76f84, false)?;
    c.literal(C::Al, r(1), 0x77e4c)?;
    c.literal(C::Al, r(2), 0x77c04)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x8)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), r(0), 0x46)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(3), r(3), 0x200)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.literal(C::Al, r(1), 0x77e50)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x8)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.emit(transfer_word(C::Al, false, r(7), r(11), -72)?)?;
    c.emit(transfer_word(C::Al, true, r(8), r(7), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Tst, r(0), r(8), 0x16)?)?;
    c.branch(C::Eq, 0x76fcc, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(11), -64)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(4), r(0), 0x0)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(0), 4)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x2000)?)?;
    c.emit(data_immediate(C::Ne, Op::Cmp, r(0), r(0), 0x2)?)?;
    c.branch(C::Ne, 0x76fd4, false)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(6), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Tst, r(0), r(8), 0x14)?)?;
    c.branch(C::Eq, 0x776ac, false)?;
    c.emit(transfer_byte(C::Al, false, r(0), r(1), 3)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 1684)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(1), 100)?)?;
    c.branch(C::Al, 0x76ffc, false)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(4), r(0), 0x0)?)?;
    c.branch(C::Al, 0x7701c, false)?;
    c.literal(C::Al, r(2), 0x774d0)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x62)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(4), r(0), 0x1)?)?;
    c.emit(data_register(C::Al, Op::Bic, r(0), r(0), r(8), Shift::Lsr, 2, false)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x200)?)?;
    c.branch(C::Al, 0x17b0, true)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(6), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_byte(C::Al, false, r(4), r(0), 4)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(4), r(0), 0x0)?)?;
    c.emit(transfer_word(C::Al, true, r(5), r(11), -72)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(9), 1760)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 1636)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x0)?)?;
    c.emit(data_immediate(C::Eq, Op::Bic, r(0), r(0), 0x10)?)?;
    c.emit(data_immediate(C::Al, Op::Bic, r(8), r(0), 0x6)?)?;
    c.emit(transfer_word(C::Eq, false, r(0), r(5), 0)?)?;
    c.emit(transfer_word(C::Al, false, r(8), r(5), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Tst, r(0), r(8), 0x800)?)?;
    c.branch(C::Eq, 0x77080, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(11), -64)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(0), 104)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Eq, 0x77080, false)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x25c)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(10), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x1000)?)?;
    c.emit(transfer_word_register_preindexed_writeback(C::Al, true, r(2), r(0), r(1), Shift::Lsl, 0)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(9), 1668)?)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(2), r(1), Shift::Lsl, 0, false)?)?;
    c.branch(C::Eq, 0x77074, false)?;
    c.emit(transfer_word(C::Al, true, r(2), r(9), 1672)?)?;
    c.emit(multiply_register(C::Al, r(3), r(2), r(1))?)?;
    c.emit(transfer_word(C::Al, false, r(3), r(0), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x234)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(0), r(0), 0x1000)?)?;
    c.emit(transfer_word_register_preindexed_writeback(C::Al, true, r(0), r(10), r(0), Shift::Lsl, 0)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(0), r(0), 0x10)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(10), 0)?)?;
    c.emit(transfer_word(C::Al, true, r(8), r(9), 1636)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(11), -72)?)?;
    c.emit(data_immediate(C::Al, Op::Bic, r(8), r(8), 0x800)?)?;
    c.emit(transfer_word(C::Al, false, r(8), r(0), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Tst, r(0), r(8), 0x20)?)?;
    c.emit(transfer_word(C::Ne, true, r(0), r(9), 1688)?)?;
    c.emit(data_immediate(C::Ne, Op::Bic, r(8), r(8), 0x20)?)?;
    c.emit(transfer_byte(C::Ne, false, r(0), r(9), 848)?)?;
    c.emit(transfer_word(C::Ne, false, r(8), r(9), 1636)?)?;
    c.emit(data_immediate(C::Al, Op::Tst, r(0), r(8), 0x200)?)?;
    c.branch(C::Eq, 0x770b0, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 1768)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(6), 1692)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 1636)?)?;
    c.emit(data_immediate(C::Al, Op::Bic, r(8), r(0), 0x200)?)?;
    c.emit(transfer_word(C::Al, false, r(8), r(9), 1636)?)?;
    c.emit(transfer_word(C::Al, true, r(10), r(11), -64)?)?;
    c.emit(data_immediate(C::Al, Op::Tst, r(0), r(8), 0x20000)?)?;
    c.branch(C::Eq, 0x771f8, false)?;
    c.emit(transfer_word(C::Al, true, r(1), r(13), 64)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(1), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Eq, 0x77148, false)?;
    c.emit(transfer_word(C::Al, false, r(4), r(1), 0)?)?;
    c.literal(C::Al, r(1), 0x78030)?;
    c.literal(C::Al, r(2), 0x77c04)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), r(0), 0x8f)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(5), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x8)?)?;
    c.emit(transfer_word(C::Al, true, r(4), r(11), -60)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(3), r(3), 0x200)?)?;
    c.emit(transfer_byte(C::Al, false, r(5), r(4), 4061)?)?;
    c.emit(transfer_word(C::Al, false, r(5), r(10), 2508)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.literal(C::Al, r(1), 0x77e50)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x8)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.emit(transfer_word(C::Al, true, r(0), r(10), 2104)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x56)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x100)?)?;
    c.emit(transfer_halfword_register(C::Al, false, r(5), r(0), r(1))?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x158)?)?;
    c.emit(transfer_halfword_register(C::Al, false, r(5), r(0), r(1))?)?;
    c.emit(transfer_word(C::Al, false, r(5), r(10), 1140)?)?;
    c.emit(transfer_word(C::Al, false, r(5), r(10), 1236)?)?;
    c.emit(transfer_word(C::Al, false, r(5), r(10), 1144)?)?;
    c.emit(transfer_word(C::Al, false, r(5), r(10), 1240)?)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(0), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.emit(transfer_word(C::Ne, false, r(5), r(10), 1196)?)?;
    c.emit(transfer_word(C::Ne, false, r(5), r(10), 1200)?)?;
    c.emit(transfer_word(C::Al, false, r(5), r(10), 1096)?)?;
    c.emit(transfer_word(C::Al, false, r(5), r(10), 1100)?)?;
    c.branch(C::Al, 0x771c8, false)?;
    c.emit(transfer_word(C::Al, true, r(4), r(11), -60)?)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(4), 4061)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Eq, 0x77164, false)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(4), 4072)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Eq, 0x7717c, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 4064)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Ne, 0x7717c, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 4068)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Eq, 0x771c8, false)?;
    c.literal(C::Al, r(1), 0x78168)?;
    c.literal(C::Al, r(2), 0x77c04)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x8)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), r(0), 0x2a0)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.literal(C::Al, r(1), 0x77e50)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x8)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x0)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(4), 20)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 32)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), r(0), 0x7)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(13), 8)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(13), 4)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(3), r(3), 0x1000)?)?;
    c.literal(C::Al, r(2), 0x7816c)?;
    c.emit(transfer_word(C::Al, false, r(2), r(13), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x2)?)?;
    c.branch(C::Al, 0x2a990, true)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(4), 4063)?)?;
    c.emit(transfer_word(C::Al, true, r(2), r(9), 1792)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(1), r(0), 0x1)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(13), 84)?)?;
    c.branch(C::Al, 0x54c1c, true)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(4), r(0), 0x0)?)?;
    c.emit(transfer_word(C::Al, false, r(4), r(10), 1696)?)?;
    c.emit(transfer_word(C::Al, false, r(4), r(10), 1704)?)?;
    c.emit(transfer_word(C::Al, false, r(4), r(10), 1712)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 1636)?)?;
    c.emit(data_immediate(C::Al, Op::Bic, r(8), r(0), 0x20000)?)?;
    c.emit(transfer_word(C::Al, false, r(8), r(9), 1636)?)?;
    c.emit(data_immediate(C::Al, Op::Tst, r(0), r(8), 0x10000)?)?;
    c.emit(transfer_word(C::Ne, true, r(3), r(10), 2104)?)?;
    c.emit(transfer_byte(C::Ne, true, r(0), r(3), 326)?)?;
    c.emit(data_immediate(C::Ne, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Eq, 0x772d8, false)?;
    c.emit(transfer_word(C::Al, true, r(7), r(9), 1784)?)?;
    c.emit(transfer_word(C::Al, true, r(6), r(10), 76)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(10), 80)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(5), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(7), 0x2)?)?;
    c.emit(transfer_word(C::Cs, true, r(2), r(9), 1788)?)?;
    c.emit(data_immediate(C::Cs, Op::Cmp, r(0), r(2), 0x2)?)?;
    c.branch(C::Cc, 0x77290, false)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(6), 0x0)?)?;
    c.emit(data_immediate(C::Ne, Op::Cmp, r(0), r(1), 0x0)?)?;
    c.branch(C::Eq, 0x77290, false)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(5), r(0), 0x64)?)?;
    c.emit(transfer_word(C::Al, false, r(3), r(11), -76)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(4), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(multiply_register(C::Al, r(0), r(2), r(5))?)?;
    c.thumb_call(0x83c4c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x64)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(5), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(multiply_register(C::Al, r(0), r(7), r(1))?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(6), Shift::Lsl, 0, false)?)?;
    c.thumb_call(0x83c4c)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, true, r(3), r(11), -76)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(4), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(4), r(0), 0x0)?)?;
    c.emit(data_register(C::Cc, Op::Mov, r(0), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(5), r(0), 0x4)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x27)?)?;
    c.emit(data_immediate(C::Hi, Op::Mov, r(5), r(0), 0x3)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x3b)?)?;
    c.emit(data_immediate(C::Hi, Op::Mov, r(5), r(0), 0x2)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x4f)?)?;
    c.emit(data_immediate(C::Hi, Op::Mov, r(5), r(0), 0x1)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(10), 2508)?)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.branch(C::Eq, 0x772cc, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(10), 32)?)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(0), r(6), Shift::Lsl, 0, false)?)?;
    c.branch(C::Ne, 0x772cc, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(10), 36)?)?;
    c.emit(transfer_word(C::Al, true, r(2), r(11), -72)?)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.branch(C::Ne, 0x772d0, false)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(3), 336)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.emit(transfer_word(C::Eq, false, r(5), r(10), 2508)?)?;
    c.emit(transfer_word(C::Eq, true, r(8), r(2), 0)?)?;
    c.branch(C::Al, 0x772d0, false)?;
    c.emit(transfer_word(C::Al, true, r(2), r(11), -72)?)?;
    c.emit(data_immediate(C::Al, Op::Bic, r(0), r(8), 0x10000)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(2), 0)?)?;
    c.branch(C::Al, 0x2f8ac, true)?;
    c.emit(transfer_word(C::Al, true, r(8), r(11), -60)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.emit(transfer_byte(C::Ne, true, r(0), r(8), 988)?)?;
    c.emit(data_immediate(C::Ne, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Eq, 0x77390, false)?;
    c.literal(C::Al, r(0), 0x781f4)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(7), r(0), 0xe90)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(7), r(7), 0x7000)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(0), 0)?)?;
    c.emit(transfer_word(C::Al, true, r(3), r(8), 1204)?)?;
    c.emit(transfer_word(C::Al, true, r(2), r(8), 3256)?)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(3), r(7), Shift::Lsl, 0, false)?)?;
    c.branch(C::Cc, 0x77328, false)?;
    c.emit(transfer_word(C::Al, true, r(1), r(9), 1672)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(1), Shift::Lsr, 18, false)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0xe)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Hi, Op::Mov, r(1), r(0), 0x1)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(2), 0x3c)?)?;
    c.branch(C::Hi, 0x77340, false)?;
    c.emit(data_immediate_with_flags(C::Al, Op::And, r(0), r(0), 0x3000, true)?)?;
    c.branch(C::Ne, 0x77340, false)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x1)?)?;
    c.branch(C::Ne, 0x77390, false)?;
    c.emit(transfer_word(C::Al, true, r(1), r(8), 20)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(8), 32)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), r(0), 0x8)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x2)?)?;
    c.emit(transfer_word(C::Al, false, r(4), r(13), 0)?)?;
    c.emit(transfer_word(C::Al, false, r(4), r(13), 4)?)?;
    c.emit(transfer_word(C::Al, false, r(4), r(13), 8)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(3), r(3), 0x1000)?)?;
    c.branch(C::Al, 0x2a990, true)?;
    c.emit(transfer_word(C::Al, true, r(0), r(8), 3256)?)?;
    c.literal(C::Al, r(1), 0x78318)?;
    c.literal(C::Al, r(2), 0x7831c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), r(0), 0x6e)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(3), r(3), 0x600)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(13), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x8)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.literal(C::Al, r(1), 0x77e50)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x8)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.emit(transfer_word(C::Al, true, r(0), r(10), 4)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x4000)?)?;
    c.emit(data_immediate(C::Ne, Op::Cmp, r(0), r(0), 0x1000)?)?;
    c.branch(C::Ne, 0x773ac, false)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(8), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x62dd0, true)?;
    c.branch(C::Al, 0x773b4, false)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(8), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x644d4, true)?;
    c.emit(data_register(C::Al, Op::Mov, r(6), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(6), 0x1)?)?;
    c.branch(C::Ne, 0x7742c, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 420)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(7), r(9), 0x5a0)?)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(2), r(11), 0x38)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(5), r(9), 0x170)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(7), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(0), 84)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(9), 480)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(11), -80)?)?;
    c.branch(C::Al, 0x356e8, true)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(9), 373)?)?;
    c.emit(transfer_word(C::Al, false, r(6), r(11), -72)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Eq, 0x77448, false)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(5), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Eq, 0x787cc, false)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(9), 4)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x2)?)?;
    c.branch(C::Eq, 0x78798, false)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x1)?)?;
    c.branch(C::Eq, 0x787a0, false)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Eq, 0x78790, false)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x388)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x400)?)?;
    c.branch(C::Al, 0x787c4, false)?;
    c.literal(C::Al, r(1), 0x783a8)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x4)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.literal(C::Al, r(1), 0x77e50)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x4)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.branch(C::Al, 0x78a98, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(10), 4)?)?;
    c.emit(transfer_word(C::Al, false, r(5), r(13), 68)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x4000)?)?;
    c.emit(data_immediate(C::Ne, Op::Cmp, r(0), r(0), 0x1000)?)?;
    c.branch(C::Ne, 0x774c4, false)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(8), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x623c8, true)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(10), 1493)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(1), r(0), 0x1)?)?;
    c.emit(transfer_byte(C::Al, false, r(0), r(9), 1314)?)?;
    c.emit(transfer_byte(C::Al, false, r(1), r(10), 1493)?)?;
    c.emit(data_immediate(C::Al, Op::And, r(1), r(1), 0xff)?)?;
    c.emit(transfer_word(C::Al, true, r(2), r(10), 912)?)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(2), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x11c)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x400)?)?;
    c.emit(transfer_byte(C::Ls, false, r(4), r(10), 1493)?)?;
    c.emit(data_register(C::Al, Op::Add, r(5), r(9), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(1), r(10), 0x5a0)?)?;
    c.emit(data_register(C::Al, Op::Add, r(0), r(1), r(0), Shift::Lsl, 2, false)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(0), 20)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(9), 1324)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(10), 1440)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(9), 1340)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(10), 4)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x1000)?)?;
    c.branch(C::Ne, 0x774d4, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(13), 68)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(10), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x7df7c, true)?;
    c.branch(C::Al, 0x774e8, false)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(8), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x65e94, true)?;
    c.branch(C::Al, 0x774f8, false)?;
    c.data(0x774d0, 0xf3ce9)?;
    c.literal(C::Al, r(2), 0x774d0)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0xa7)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x600)?)?;
    c.branch(C::Al, 0x17b0, true)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(10), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x7dc70, true)?;
    c.emit(transfer_word(C::Al, true, r(5), r(13), 68)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 420)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(0), 24)?)?;
    c.emit(transfer_word(C::Al, false, r(4), r(9), 968)?)?;
    c.emit(data_immediate(C::Al, Op::Tst, r(0), r(1), 0x40)?)?;
    c.branch(C::Eq, 0x77518, false)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(10), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x761e0, true)?;
    c.emit(transfer_word(C::Al, true, r(2), r(10), 1016)?)?;
    c.emit(transfer_word(C::Al, false, r(7), r(13), 56)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(2), 0x0)?)?;
    c.branch(C::Eq, 0x776a4, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(10), 2092)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Eq, 0x77784, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 40)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(0), 12)?)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(0), 112)?)?;
    c.emit(data_register(C::Al, Op::Rsb, r(2), r(1), r(1), Shift::Lsl, 3, false)?)?;
    c.emit(transfer_word_register(C::Al, true, r(3), r(0), r(2), Shift::Lsl, 2)?)?;
    c.emit(transfer_byte(C::Al, true, r(2), r(10), 351)?)?;
    c.emit(data_register(C::Al, Op::Eor, r(1), r(2), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_byte(C::Al, false, r(1), r(0), 112)?)?;
    c.emit(transfer_word(C::Al, false, r(3), r(9), 464)?)?;
    c.emit(transfer_word(C::Al, false, r(3), r(13), 20)?)?;
    c.emit(transfer_word(C::Al, false, r(3), r(9), 1488)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), r(0), 0x7)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(10), 4)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(10), 2012)?)?;
    c.emit(transfer_word(C::Al, true, r(2), r(10), 2016)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x2000)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x4)?)?;
    c.emit(data_immediate(C::Eq, Op::Mov, r(0), r(0), 0x5)?)?;
    c.emit(data_register_shifted_by_register(C::Al, Op::Mov, r(2), r(0), r(2), Shift::Asr, r(0), false)?)?;
    c.emit(data_register_shifted_by_register(C::Al, Op::Add, r(0), r(3), r(1), Shift::Asr, r(0), false)?)?;
    c.emit(data_immediate(C::Al, Op::Bic, r(0), r(0), 0x7)?)?;
    c.emit(multiply_register(C::Al, r(8), r(0), r(2))?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(10), 2104)?)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(0), 232)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Eq, 0x77830, false)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(8), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x2efc, true)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Eq, 0x77840, false)?;
    c.emit(transfer_word(C::Al, false, r(0), r(11), -64)?)?;
    c.literal(C::Al, r(5), 0x774d0)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x6)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(10), 2104)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x400)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(0), 292)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Ne, Op::Mov, r(0), r(0), 0x1)?)?;
    c.branch(C::Al, 0x17b0, true)?;
    c.emit(transfer_word(C::Al, true, r(0), r(10), 2104)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x7)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x400)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(0), 296)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Ne, Op::Mov, r(0), r(0), 0x1)?)?;
    c.branch(C::Al, 0x17b0, true)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(8), 0x0)?)?;
    c.branch(C::Eq, 0x77968, false)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(7), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(6), r(0), 0x1)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(5), r(0), 0x1)?)?;
    c.emit(transfer_word(C::Al, true, r(4), r(10), 2104)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 292)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(1), r(0), 0x1)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(6), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x81850, true)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x0)?)?;
    c.branch(C::Eq, 0x77640, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(11), -64)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(4), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(6), r(6), 0x1)?)?;
    c.emit(transfer_byte_register(C::Al, false, r(4), r(0), r(7), Shift::Lsl, 0)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(7), r(7), 0x1)?)?;
    c.branch(C::Al, 0x77698, false)?;
    c.data(0x7763c, 0x221004)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(6), r(0), 0x1)?)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(7), r(8), Shift::Lsl, 0, false)?)?;
    c.branch(C::Cs, 0x77694, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 296)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Eq, 0x77694, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(11), -64)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(4), r(0), 0x0)?)?;
    c.emit(data_register(C::Al, Op::Add, r(1), r(0), r(7), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(data_register(C::Al, Op::Add, r(2), r(7), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_byte_register(C::Al, false, r(5), r(1), r(0), Shift::Lsl, 0)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(0), 0x1)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(2), r(2), 0x1)?)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(2), r(8), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Cc, true, r(2), r(10), 2104)?)?;
    c.emit(transfer_word(C::Cc, true, r(2), r(2), 296)?)?;
    c.emit(data_register(C::Cc, Op::Cmp, r(0), r(0), r(2), Shift::Lsl, 0, false)?)?;
    c.branch(C::Cc, 0x77668, false)?;
    c.emit(data_register(C::Al, Op::Add, r(7), r(7), r(0), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x77698, false)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(4), r(0), 0x0)?)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(7), r(8), Shift::Lsl, 0, false)?)?;
    c.branch(C::Cc, 0x77608, false)?;
    c.branch(C::Al, 0x77970, false)?;
    c.emit(transfer_word(C::Al, false, r(4), r(9), 1488)?)?;
    c.branch(C::Al, 0x78248, false)?;
    c.emit(transfer_byte(C::Al, false, r(0), r(1), 4)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(11), -64)?)?;
    c.emit(transfer_word(C::Al, true, r(5), r(11), -72)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(0), 104)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Eq, 0x77000, false)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x234)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(9), 1680)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(0), r(0), 0x1000)?)?;
    c.emit(data_register(C::Al, Op::Add, r(2), r(10), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 1676)?)?;
    c.branch(C::Al, 0x6b8c8, true)?;
    c.branch(C::Al, 0x77000, false)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(6), 3)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.emit(transfer_word(C::Ne, true, r(0), r(7), 36)?)?;
    c.emit(data_immediate(C::Ne, Op::Mov, r(1), r(0), 0x1)?)?;
    c.emit(transfer_byte(C::Ne, false, r(1), r(0), 11)?)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(6), 2)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Ne, 0x77764, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 472)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.emit(transfer_word(C::Ne, true, r(0), r(5), 480)?)?;
    c.emit(data_immediate(C::Ne, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Eq, 0x77724, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(7), 1796)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(6), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x71328, true)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(5), 188)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Eq, 0x77740, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(7), 36)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(6), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x61b5c, true)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(6), 2)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Ne, 0x77764, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 104)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Eq, 0x77764, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(7), 32)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(6), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x69e44, true)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x2)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(7), 0)?)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(6), 9)?)?;
    c.emit(transfer_byte(C::Al, false, r(0), r(7), 7)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(8), 48)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(0), 0x1c)?)?;
    c.branch(C::Al, 0x35df4, true)?;
    c.branch(C::Al, 0x78f7c, false)?;
    c.emit(transfer_word(C::Al, true, r(12), r(9), 40)?)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(10), 385)?)?;
    c.emit(transfer_word(C::Al, true, r(8), r(12), 12)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Eq, 0x77854, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 964)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.emit(transfer_word(C::Ne, true, r(1), r(0), 0)?)?;
    c.emit(data_immediate(C::Ne, Op::Cmp, r(0), r(1), 0x0)?)?;
    c.branch(C::Ne, 0x77a24, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(8), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Eq, 0x777d0, false)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.branch(C::Al, 0x31ec, true)?;
    c.emit(transfer_word(C::Al, true, r(1), r(10), 1016)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(8), 0)?)?;
    c.branch(C::Al, 0x205c, true)?;
    c.emit(transfer_word(C::Al, true, r(0), r(8), 0)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(9), 464)?)?;
    c.branch(C::Al, 0x77a50, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(7), 32)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(6), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x6b38c, true)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(6), 2)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Eq, 0x76a3c, false)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x2)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(7), 0)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(8), 52)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(0), 0x10)?)?;
    c.branch(C::Al, 0x35df4, true)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(8), 848)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Eq, 0x78f7c, false)?;
    c.emit(transfer_word(C::Al, true, r(2), r(7), 412)?)?;
    c.literal(C::Al, r(1), 0x787b4)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x20)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.literal(C::Al, r(1), 0x77e50)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x20)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.branch(C::Al, 0x78f7c, false)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(5), r(0), 0x1)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(11), -64)?)?;
    c.branch(C::Al, 0x77970, false)?;
    c.literal(C::Al, r(2), 0x774d0)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x400)?)?;
    c.branch(C::Al, 0x17b0, true)?;
    c.branch(C::Al, 0x78248, false)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(8), 112)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(5), r(0), 0x0)?)?;
    c.emit(data_register(C::Al, Op::Rsb, r(1), r(0), r(0), Shift::Lsl, 3, false)?)?;
    c.emit(data_register(C::Al, Op::Add, r(0), r(12), r(0), Shift::Lsl, 2, false)?)?;
    c.emit(transfer_word_register(C::Al, true, r(14), r(8), r(1), Shift::Lsl, 2)?)?;
    c.emit(transfer_word(C::Al, false, r(14), r(9), 464)?)?;
    c.emit(transfer_word(C::Al, false, r(14), r(9), 1488)?)?;
    c.emit(transfer_word(C::Al, true, r(6), r(10), 620)?)?;
    c.emit(transfer_word(C::Al, true, r(4), r(0), 44)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(6), 0x0)?)?;
    c.branch(C::Eq, 0x778a8, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(10), 2724)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(7), r(0), r(4), Shift::Lsl, 0, false)?)?;
    c.emit(load_halfword_postindexed(C::Al, r(3), r(7), 2)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(1), r(1), 0x1)?)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(0), r(3), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Hi, Op::Add, r(3), r(14), r(3), Shift::Lsl, 1, false)?)?;
    c.emit(store_halfword(C::Hi, r(5), r(3), 0)?)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(1), r(6), Shift::Lsl, 0, false)?)?;
    c.branch(C::Ne, 0x7788c, false)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(9), 369)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Ne, Op::Mvn, r(0), r(0), 0x0)?)?;
    c.emit(transfer_word(C::Ne, false, r(0), r(12), 60)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 400)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x1)?)?;
    c.branch(C::Ne, 0x78248, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(10), 616)?)?;
    c.emit(transfer_word(C::Al, false, r(14), r(11), -64)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x5)?)?;
    c.branch(C::Eq, 0x78b84, false)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x2)?)?;
    c.branch(C::Ne, 0x78c2c, false)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(6), 0x0)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(13), 36)?)?;
    c.emit(transfer_word(C::Al, false, r(8), r(13), 40)?)?;
    c.emit(transfer_word(C::Al, false, r(12), r(13), 32)?)?;
    c.branch(C::Eq, 0x78f3c, false)?;
    c.emit(transfer_word(C::Al, false, r(6), r(11), -76)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(8), r(0), 0x0)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(10), 2020)?)?;
    c.emit(transfer_word(C::Al, true, r(6), r(12), 60)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(10), 2028)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(7), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Rsb, r(0), r(0), 0x0)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(13), 52)?)?;
    c.emit(data_immediate(C::Al, Op::And, r(5), r(0), 0x7)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(13), 52)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(6), 0x1)?)?;
    c.emit(data_register(C::Al, Op::Sub, r(6), r(0), r(1), Shift::Lsl, 0, true)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(7), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Cc, Op::Mov, r(6), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(6), Shift::Lsl, 0, false)?)?;
    c.thumb_call(0x83c4c)?;
    c.emit(transfer_word(C::Al, true, r(14), r(11), -64)?)?;
    c.emit(multiply_accumulate(C::Al, r(1), r(0), r(5), r(6))?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0xff)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(8), r(8), 0x1)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(0), r(0), 0xff00)?)?;
    c.emit(store_halfword_postindexed(C::Al, r(1), r(4), 2)?)?;
    c.emit(data_register(C::Al, Op::And, r(0), r(1), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x400)?)?;
    c.emit(data_register(C::Al, Op::Add, r(0), r(14), r(0), Shift::Lsl, 1, false)?)?;
    c.emit(store_halfword(C::Al, r(1), r(0), 0)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(11), -76)?)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(8), r(0), Shift::Lsl, 0, false)?)?;
    c.branch(C::Ne, 0x77914, false)?;
    c.branch(C::Al, 0x78f40, false)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(5), r(0), 0x1)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(4), r(0), 0x0)?)?;
    c.emit(transfer_word(C::Al, true, r(3), r(10), 2104)?)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(3), 232)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x6)?)?;
    c.branch(C::Cc, 0x77bb4, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(11), -64)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(8), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x31ec, true)?;
    c.emit(transfer_word(C::Al, true, r(3), r(10), 2104)?)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(3), 232)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x7)?)?;
    c.branch(C::Eq, 0x77a5c, false)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x6)?)?;
    c.branch(C::Ne, 0x77b44, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 400)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Eq, 0x77bb4, false)?;
    c.emit(transfer_word(C::Al, true, r(6), r(10), 700)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(3), 296)?)?;
    c.emit(data_register(C::Al, Op::Add, r(5), r(0), r(6), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(6), r(5), Shift::Lsl, 0, false)?)?;
    c.branch(C::Cs, 0x77b80, false)?;
    c.emit(transfer_word(C::Al, true, r(1), r(10), 2028)?)?;
    c.emit(transfer_word(C::Al, true, r(7), r(10), 2020)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(6), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x81850, true)?;
    c.emit(data_register(C::Al, Op::Mov, r(4), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(7), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(4), Shift::Lsl, 0, false)?)?;
    c.thumb_call(0x83c4c)?;
    c.emit(transfer_word(C::Al, true, r(3), r(11), -64)?)?;
    c.emit(data_immediate(C::Al, Op::Rsb, r(1), r(7), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(6), r(6), 0x1)?)?;
    c.emit(data_immediate(C::Al, Op::And, r(1), r(1), 0x7)?)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(6), r(5), Shift::Lsl, 0, false)?)?;
    c.emit(multiply_accumulate(C::Al, r(2), r(0), r(1), r(4))?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.emit(transfer_byte_register(C::Al, false, r(0), r(3), r(2), Shift::Lsl, 0)?)?;
    c.branch(C::Ne, 0x779c8, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(10), 2104)?)?;
    c.emit(transfer_word(C::Al, true, r(6), r(10), 700)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(4), r(0), 0x0)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(5), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(0), 296)?)?;
    c.branch(C::Al, 0x77b84, false)?;
    c.emit(transfer_word_preindexed_writeback(C::Al, true, r(1), r(0), 8)?)?;
    c.literal(C::Al, r(2), 0x774d0)?;
    c.emit(data_register(C::Al, Op::Add, r(0), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x2b)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(9), 464)?)?;
    c.emit(data_immediate(C::Al, Op::Tst, r(0), r(0), 0xff)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x500)?)?;
    c.emit(data_immediate(C::Eq, Op::Mov, r(0), r(0), 0x1)?)?;
    c.branch(C::Al, 0x17b0, true)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 464)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(9), 1488)?)?;
    c.branch(C::Al, 0x78248, false)?;
    c.data(0x77a58, 0x1fe001)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 400)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Eq, 0x77bb4, false)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(10), 704)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Eq, 0x77af0, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(10), 2028)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Eq, 0x77ae8, false)?;
    c.emit(transfer_word(C::Al, true, r(1), r(10), 696)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x0)?)?;
    c.emit(store_halfword_postindexed(C::Al, r(2), r(1), 2)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(2), r(2), 0x1)?)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(2), r(0), Shift::Lsl, 0, false)?)?;
    c.branch(C::Ne, 0x77a88, false)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(7), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(6), r(0), 0x0)?)?;
    c.branch(C::Al, 0x83d84, true)?;
    c.emit(transfer_word(C::Al, true, r(4), r(10), 2028)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(4), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x81838, true)?;
    c.emit(transfer_word(C::Al, true, r(0), r(10), 696)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(6), r(6), 0x1)?)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(6), r(4), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Add, r(2), r(0), r(7), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Add, r(0), r(0), r(1), Shift::Lsl, 1, false)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(7), r(7), 0x2)?)?;
    c.emit(load_halfword(C::Al, r(3), r(2), 0)?)?;
    c.emit(load_halfword(C::Al, r(1), r(0), 0)?)?;
    c.emit(store_halfword(C::Al, r(1), r(2), 0)?)?;
    c.emit(store_halfword(C::Al, r(3), r(0), 0)?)?;
    c.branch(C::Cc, 0x77aa0, false)?;
    c.emit(transfer_word(C::Al, true, r(3), r(10), 2104)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(5), r(0), 0x1)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(transfer_byte(C::Al, false, r(0), r(10), 704)?)?;
    c.emit(transfer_word(C::Al, true, r(6), r(10), 700)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(3), 296)?)?;
    c.emit(data_register(C::Al, Op::Add, r(7), r(0), r(6), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(6), r(7), Shift::Lsl, 0, false)?)?;
    c.branch(C::Cs, 0x77b98, false)?;
    c.emit(transfer_word(C::Al, true, r(4), r(11), -64)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(10), 2028)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(6), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x81850, true)?;
    c.emit(transfer_word(C::Al, true, r(0), r(10), 696)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(6), r(6), 0x1)?)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(6), r(7), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Add, r(0), r(0), r(1), Shift::Lsl, 1, false)?)?;
    c.emit(load_halfword(C::Al, r(0), r(0), 0)?)?;
    c.emit(transfer_byte_register(C::Al, false, r(5), r(4), r(0), Shift::Lsl, 0)?)?;
    c.branch(C::Ne, 0x77b08, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(10), 2104)?)?;
    c.emit(transfer_word(C::Al, true, r(6), r(10), 700)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(5), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(0), 296)?)?;
    c.branch(C::Al, 0x77b9c, false)?;
    c.literal(C::Al, r(2), 0x774d0)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x450)?)?;
    c.branch(C::Al, 0x17b0, true)?;
    c.emit(transfer_word(C::Al, true, r(3), r(10), 2104)?)?;
    c.branch(C::Al, 0x77bb4, false)?;
    c.emit(data_immediate(C::Al, Op::Add, r(3), r(5), 0x118)?)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(1), r(11), 0x20)?)?;
    c.branch(C::Al, 0x7f990, true)?;
    c.emit(transfer_word(C::Al, true, r(0), r(11), -32)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Eq, 0x78ef0, false)?;
    c.emit(data_immediate(C::Al, Op::Add, r(1), r(5), 0x30)?)?;
    c.branch(C::Al, 0x8025c, true)?;
    c.branch(C::Al, 0x78cf8, false)?;
    c.emit(data_register(C::Al, Op::Mov, r(5), r(0), r(3), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(10), 2028)?)?;
    c.emit(data_register(C::Al, Op::Add, r(0), r(0), r(6), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x81850, true)?;
    c.emit(transfer_word(C::Al, false, r(1), r(10), 700)?)?;
    c.branch(C::Al, 0x77bb0, false)?;
    c.emit(data_register(C::Al, Op::Mov, r(5), r(0), r(3), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(10), 2028)?)?;
    c.emit(data_register(C::Al, Op::Add, r(0), r(0), r(6), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x81850, true)?;
    c.emit(transfer_word(C::Al, false, r(1), r(10), 700)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(3), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_byte(C::Al, true, r(7), r(3), 232)?)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(0), r(7), 0x1)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x6)?)?;
    c.branch(C::Hi, 0x77ce8, false)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(8), Shift::Lsr, 3, false)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(1), r(15), 0x10)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(0), Shift::Lsl, 2, false)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(11), -76)?)?;
    c.emit(transfer_word(C::Al, false, r(3), r(13), 24)?)?;
    c.emit(transfer_word_register(C::Al, true, r(15), r(0), r(1), Shift::Lsl, 0)?)?;
    c.data(0x77bdc, 0x88888889)?;
    c.data(0x77be0, 0x77c08)?;
    c.data(0x77be4, 0x77cf4)?;
    c.data(0x77be8, 0x77d98)?;
    c.data(0x77bec, 0x78034)?;
    c.data(0x77bf0, 0x78170)?;
    c.data(0x77bf4, 0x77c08)?;
    c.data(0x77bf8, 0x77c08)?;
    c.data(0x77bfc, 0x77881)?;
    c.data(0x77c00, 0xf3f9a)?;
    c.data(0x77c04, 0xf3ffe)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(4), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(2), 0x0)?)?;
    c.branch(C::Eq, 0x78224, false)?;
    c.emit(transfer_word(C::Al, false, r(7), r(13), 52)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(14), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(5), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(4), r(0), 0x0)?)?;
    c.emit(transfer_word(C::Al, true, r(8), r(9), 400)?)?;
    c.emit(transfer_word(C::Al, true, r(6), r(13), 20)?)?;
    c.emit(transfer_word(C::Al, true, r(12), r(11), -76)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(8), 0x0)?)?;
    c.branch(C::Eq, 0x77cbc, false)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(8), 0x2)?)?;
    c.emit(transfer_word(C::Eq, true, r(0), r(10), 2092)?)?;
    c.emit(data_immediate(C::Eq, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Eq, 0x77cbc, false)?;
    c.emit(transfer_word(C::Al, true, r(1), r(11), -64)?)?;
    c.emit(transfer_byte_register_preindexed_writeback(C::Al, true, r(4), r(1), r(14), Shift::Lsl, 0)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(14), r(14), 0x8)?)?;
    c.emit(transfer_byte(C::Al, true, r(3), r(1), 6)?)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(1), 7)?)?;
    c.emit(transfer_byte(C::Al, true, r(2), r(1), 1)?)?;
    c.emit(transfer_byte(C::Al, true, r(7), r(1), 4)?)?;
    c.emit(transfer_byte(C::Al, true, r(12), r(1), 3)?)?;
    c.emit(data_immediate(C::Al, Op::And, r(3), r(3), 0x1)?)?;
    c.emit(data_immediate(C::Al, Op::And, r(0), r(0), 0x1)?)?;
    c.emit(data_immediate(C::Al, Op::And, r(7), r(7), 0x1)?)?;
    c.emit(data_immediate(C::Al, Op::And, r(2), r(2), 0x1)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(3), r(0), r(3), Shift::Lsl, 10, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(2), Shift::Lsl, 26, false)?)?;
    c.emit(data_register(C::Al, Op::Orr, r(0), r(3), r(0), Shift::Lsl, 26, false)?)?;
    c.emit(transfer_byte(C::Al, true, r(3), r(1), 2)?)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(1), 5)?)?;
    c.emit(data_immediate(C::Al, Op::And, r(1), r(1), 0x1)?)?;
    c.emit(data_immediate(C::Al, Op::And, r(3), r(3), 0x1)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(1), Shift::Lsl, 26, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(3), r(0), r(3), Shift::Lsl, 10, false)?)?;
    c.emit(data_register(C::Al, Op::Orr, r(7), r(1), r(7), Shift::Lsl, 10, false)?)?;
    c.emit(data_immediate(C::Al, Op::And, r(1), r(12), 0x1)?)?;
    c.emit(transfer_word(C::Al, true, r(12), r(11), -76)?)?;
    c.emit(data_register(C::Al, Op::Orr, r(3), r(3), r(1), Shift::Lsl, 26, false)?)?;
    c.emit(data_immediate(C::Al, Op::And, r(1), r(4), 0x1)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(4), r(0), 0x0)?)?;
    c.emit(data_register(C::Al, Op::Orr, r(1), r(2), r(1), Shift::Lsl, 10, false)?)?;
    c.branch(C::Al, 0x77ccc, false)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(7), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(5), r(5), 0x1)?)?;
    c.emit(transfer_multiple(C::Al, false, r(6), 0x8a, BlockMode::IncrementAfter, false)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(6), 12)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(6), r(6), 0x10)?)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(5), r(12), Shift::Lsl, 0, false)?)?;
    c.branch(C::Ne, 0x77c30, false)?;
    c.emit(transfer_word(C::Al, true, r(7), r(13), 52)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(4), r(0), 0x0)?)?;
    c.branch(C::Al, 0x7815c, false)?;
    c.data(0x77cf0, 0xf3e2c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(4), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(2), 0x0)?)?;
    c.branch(C::Eq, 0x78224, false)?;
    c.emit(transfer_word(C::Al, true, r(1), r(11), -64)?)?;
    c.emit(transfer_word(C::Al, true, r(2), r(13), 20)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(transfer_byte(C::Al, true, r(7), r(1), 1)?)?;
    c.emit(transfer_byte(C::Al, true, r(3), r(1), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(8), r(0), 0x2)?)?;
    c.emit(transfer_byte(C::Al, true, r(6), r(1), 2)?)?;
    c.emit(transfer_byte(C::Al, true, r(5), r(1), 3)?)?;
    c.emit(transfer_byte(C::Al, true, r(4), r(1), 4)?)?;
    c.emit(transfer_byte(C::Al, true, r(14), r(1), 5)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(0), 0x1)?)?;
    c.emit(data_register(C::Al, Op::And, r(3), r(8), r(3), Shift::Lsl, 1, false)?)?;
    c.emit(data_immediate(C::Al, Op::And, r(7), r(7), 0x1)?)?;
    c.emit(data_immediate(C::Al, Op::And, r(6), r(6), 0x1)?)?;
    c.emit(data_immediate(C::Al, Op::And, r(5), r(5), 0x1)?)?;
    c.emit(data_register(C::Al, Op::And, r(4), r(8), r(4), Shift::Lsl, 1, false)?)?;
    c.emit(data_register(C::Al, Op::Orr, r(12), r(3), r(7), Shift::Lsl, 17, false)?)?;
    c.emit(transfer_byte(C::Al, true, r(3), r(1), 6)?)?;
    c.emit(transfer_byte(C::Al, true, r(7), r(1), 7)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(6), r(0), r(6), Shift::Lsl, 1, false)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(1), r(1), 0x8)?)?;
    c.emit(data_register(C::Al, Op::Orr, r(5), r(6), r(5), Shift::Lsl, 17, false)?)?;
    c.emit(transfer_word(C::Al, false, r(12), r(2), 0)?)?;
    c.emit(data_immediate(C::Al, Op::And, r(3), r(3), 0x1)?)?;
    c.emit(data_immediate(C::Al, Op::And, r(6), r(7), 0x1)?)?;
    c.emit(data_immediate(C::Al, Op::And, r(7), r(14), 0x1)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(3), r(0), r(3), Shift::Lsl, 1, false)?)?;
    c.emit(data_register(C::Al, Op::Orr, r(7), r(4), r(7), Shift::Lsl, 17, false)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(4), r(0), 0x0)?)?;
    c.emit(data_register(C::Al, Op::Orr, r(3), r(3), r(6), Shift::Lsl, 17, false)?)?;
    c.emit(transfer_multiple(C::Al, false, r(2), 0xa0, BlockMode::IncrementBefore, false)?)?;
    c.emit(transfer_word(C::Al, false, r(3), r(2), 12)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(2), r(2), 0x10)?)?;
    c.emit(transfer_word(C::Al, true, r(3), r(11), -76)?)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(0), r(3), Shift::Lsl, 0, false)?)?;
    c.branch(C::Ne, 0x77d0c, false)?;
    c.branch(C::Al, 0x78224, false)?;
    c.data(0x77d94, 0xf417b)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(4), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(2), 0x0)?)?;
    c.branch(C::Eq, 0x78224, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(11), -64)?)?;
    c.emit(transfer_word(C::Al, true, r(6), r(13), 20)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(1), r(0), 0x7)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(10), 4)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(13), 16)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(13), 44)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x1000)?)?;
    c.literal(C::Al, r(2), 0x78ce4)?;
    c.emit(transfer_word(C::Al, false, r(2), r(13), 48)?)?;
    c.literal(C::Al, r(2), 0x78ce8)?;
    c.emit(transfer_word(C::Al, false, r(2), r(13), 52)?)?;
    c.branch(C::Eq, 0x77e18, false)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x4000)?)?;
    c.branch(C::Ne, 0x77e54, false)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x30)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(12), r(0), 0x10)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(0), r(0), 0x100000)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(12), r(12), 0x200000)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(0), 0x300000)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(13), 28)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x100000)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(13), 36)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x70)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(13), 32)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x50)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(0), r(0), 0x600000)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(13), 40)?)?;
    c.branch(C::Al, 0x77f58, false)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x200000)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(14), r(0), 0x30)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(12), r(0), 0x10)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(13), 36)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x10)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(14), r(14), 0x100000)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(12), r(12), 0x200000)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(13), 32)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x20)?)?;
    c.emit(transfer_word(C::Al, false, r(14), r(13), 28)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(0), r(0), 0x300000)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(13), 40)?)?;
    c.branch(C::Al, 0x77f58, false)?;
    c.data(0x77e4c, 0xf4016)?;
    c.data(0x77e50, 0xe2788)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(1), -7)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(7), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(12), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(14), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(1), -6)?)?;
    c.emit(transfer_word(C::Ne, true, r(7), r(3), 236)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(1), -5)?)?;
    c.emit(transfer_word(C::Ne, true, r(12), r(3), 240)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.emit(transfer_word(C::Ne, true, r(0), r(3), 244)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(12), r(0), r(12), Shift::Lsl, 20, false)?)?;
    c.emit(data_immediate(C::Eq, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(13), 40)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(transfer_byte(C::Al, true, r(2), r(1), -4)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(2), 0x0)?)?;
    c.emit(transfer_word(C::Ne, true, r(2), r(3), 248)?)?;
    c.emit(data_immediate(C::Eq, Op::Mov, r(2), r(0), 0x0)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(13), 28)?)?;
    c.emit(transfer_byte(C::Al, true, r(2), r(1), -3)?)?;
    c.emit(transfer_byte(C::Al, true, r(5), r(1), -2)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(2), 0x0)?)?;
    c.emit(transfer_word(C::Al, true, r(2), r(13), 40)?)?;
    c.emit(transfer_word(C::Ne, true, r(0), r(3), 252)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(5), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(5), r(0), 0x0)?)?;
    c.emit(transfer_word(C::Ne, true, r(5), r(3), 256)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(4), r(0), r(0), Shift::Lsl, 4, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(7), Shift::Lsl, 4, false)?)?;
    c.emit(data_immediate(C::Al, Op::And, r(7), r(2), 0x3f)?)?;
    c.emit(transfer_byte(C::Al, true, r(2), r(1), -1)?)?;
    c.emit(data_immediate(C::Al, Op::And, r(0), r(0), 0x3f0)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(8), r(0), r(7), Shift::Lsl, 4, false)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(2), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x3f00000)?)?;
    c.emit(data_register(C::Al, Op::And, r(2), r(2), r(5), Shift::Lsl, 20, false)?)?;
    c.emit(transfer_word(C::Ne, true, r(14), r(3), 260)?)?;
    c.emit(data_immediate(C::Al, Op::And, r(5), r(12), 0x3f00000)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(13), 40)?)?;
    c.emit(data_immediate(C::Al, Op::And, r(2), r(4), 0x3f0)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(13), 32)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(13), 36)?)?;
    c.emit(transfer_byte(C::Al, true, r(2), r(1), 0)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(13), 28)?)?;
    c.emit(data_immediate(C::Al, Op::And, r(7), r(14), 0x3f)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(2), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::And, r(4), r(0), 0x3f)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(13), 32)?)?;
    c.emit(transfer_word(C::Ne, true, r(2), r(3), 264)?)?;
    c.emit(transfer_word(C::Al, true, r(3), r(13), 40)?)?;
    c.emit(data_register(C::Al, Op::Orr, r(3), r(3), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Orr, r(0), r(8), r(4), Shift::Lsl, 20, false)?)?;
    c.emit(data_immediate(C::Eq, Op::Mov, r(2), r(0), 0x0)?)?;
    c.emit(transfer_word(C::Al, false, r(3), r(13), 40)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(13), 28)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(13), 36)?)?;
    c.emit(data_register(C::Al, Op::Orr, r(12), r(5), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(7), Shift::Lsl, 4, false)?)?;
    c.emit(data_immediate(C::Al, Op::And, r(5), r(2), 0x3f)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(13), 32)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(5), Shift::Lsl, 20, false)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(13), 36)?)?;
    c.emit(transfer_word(C::Al, true, r(2), r(13), 52)?)?;
    c.emit(transfer_byte(C::Al, true, r(7), r(1), -7)?)?;
    c.emit(transfer_byte(C::Al, true, r(5), r(1), -6)?)?;
    c.emit(data_register(C::Al, Op::And, r(4), r(12), r(2), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(1), -5)?)?;
    c.emit(transfer_byte(C::Al, true, r(2), r(1), -4)?)?;
    c.emit(transfer_byte(C::Al, true, r(12), r(1), -2)?)?;
    c.emit(transfer_byte(C::Al, true, r(3), r(1), 0)?)?;
    c.emit(data_immediate(C::Al, Op::And, r(7), r(7), 0x1)?)?;
    c.emit(data_immediate(C::Al, Op::And, r(5), r(5), 0x1)?)?;
    c.emit(data_immediate(C::Al, Op::And, r(0), r(0), 0x1)?)?;
    c.emit(data_immediate(C::Al, Op::And, r(2), r(2), 0x1)?)?;
    c.emit(data_register(C::Al, Op::Orr, r(7), r(4), r(7), Shift::Lsl, 11, false)?)?;
    c.emit(transfer_byte(C::Al, true, r(4), r(1), -3)?)?;
    c.emit(data_register(C::Al, Op::Orr, r(7), r(7), r(5), Shift::Lsl, 27, false)?)?;
    c.emit(transfer_byte(C::Al, true, r(5), r(1), -1)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(1), r(1), 0x8)?)?;
    c.emit(transfer_word(C::Al, false, r(7), r(6), 0)?)?;
    c.emit(transfer_word(C::Al, true, r(8), r(13), 48)?)?;
    c.emit(transfer_word(C::Al, true, r(14), r(13), 28)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(7), r(8), 0x3f00000)?)?;
    c.emit(data_register(C::Al, Op::And, r(7), r(14), r(7), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Orr, r(0), r(7), r(0), Shift::Lsl, 11, false)?)?;
    c.emit(data_register(C::Al, Op::Orr, r(0), r(0), r(2), Shift::Lsl, 27, false)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x7f00000)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(6), 4)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(2), r(2), 0xf0000000)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(13), 32)?)?;
    c.emit(transfer_word(C::Al, true, r(7), r(13), 36)?)?;
    c.emit(data_register(C::Al, Op::And, r(0), r(0), r(8), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::And, r(2), r(7), r(2), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Orr, r(0), r(2), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::And, r(2), r(5), 0x1)?)?;
    c.emit(data_register(C::Al, Op::Orr, r(0), r(0), r(2), Shift::Lsl, 11, false)?)?;
    c.emit(data_immediate(C::Al, Op::And, r(2), r(3), 0x1)?)?;
    c.emit(data_register(C::Al, Op::Orr, r(0), r(0), r(2), Shift::Lsl, 27, false)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(6), 12)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(13), 40)?)?;
    c.emit(transfer_word(C::Al, true, r(2), r(13), 52)?)?;
    c.emit(data_register(C::Al, Op::And, r(0), r(0), r(2), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::And, r(2), r(4), 0x1)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(4), r(0), 0x0)?)?;
    c.emit(data_register(C::Al, Op::Orr, r(0), r(0), r(2), Shift::Lsl, 11, false)?)?;
    c.emit(data_immediate(C::Al, Op::And, r(2), r(12), 0x1)?)?;
    c.emit(data_register(C::Al, Op::Orr, r(0), r(0), r(2), Shift::Lsl, 27, false)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(6), 8)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(6), r(6), 0x10)?)?;
    c.emit(transfer_word(C::Al, true, r(2), r(13), 44)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(11), -76)?)?;
    c.emit(transfer_word(C::Al, true, r(3), r(13), 24)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(2), r(2), 0x1)?)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(2), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(13), 16)?)?;
    c.branch(C::Ne, 0x77dbc, false)?;
    c.branch(C::Al, 0x78224, false)?;
    c.data(0x78030, 0xf407b)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(8), r(0), 0x4)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(4), r(0), 0x1000)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(2), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(7), r(0), 0x4)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(8), r(8), 0x40000)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(4), r(4), 0x10000000)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(8), Shift::Lsl, 0, false)?)?;
    c.branch(C::Eq, 0x78118, false)?;
    c.emit(transfer_word(C::Al, true, r(1), r(11), -64)?)?;
    c.emit(transfer_word(C::Al, true, r(2), r(13), 20)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(8), r(0), 0x0)?)?;
    c.emit(transfer_byte(C::Al, true, r(6), r(1), 2)?)?;
    c.emit(transfer_byte(C::Al, true, r(5), r(1), 3)?)?;
    c.emit(transfer_byte(C::Al, true, r(3), r(1), 0)?)?;
    c.emit(transfer_byte(C::Al, true, r(12), r(1), 1)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(8), r(8), 0x1)?)?;
    c.emit(data_register(C::Al, Op::And, r(6), r(7), r(6), Shift::Lsl, 2, false)?)?;
    c.emit(data_immediate(C::Al, Op::And, r(5), r(5), 0x1)?)?;
    c.emit(data_register(C::Al, Op::Orr, r(6), r(6), r(5), Shift::Lsl, 18, false)?)?;
    c.emit(data_register(C::Al, Op::Orr, r(7), r(6), r(4), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, false, r(7), r(13), 52)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(7), r(0), 0xfc)?)?;
    c.emit(data_register(C::Al, Op::And, r(3), r(7), r(3), Shift::Lsl, 2, false)?)?;
    c.emit(transfer_byte(C::Al, true, r(6), r(1), 7)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(7), r(0), 0x1000)?)?;
    c.emit(transfer_byte(C::Al, true, r(4), r(1), 4)?)?;
    c.emit(transfer_byte(C::Al, true, r(5), r(1), 6)?)?;
    c.emit(transfer_byte(C::Al, true, r(14), r(1), 5)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(1), r(1), 0x8)?)?;
    c.emit(data_register(C::Al, Op::Orr, r(3), r(3), r(12), Shift::Lsl, 18, false)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(7), r(7), 0x10000000)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(12), r(0), 0xfc)?)?;
    c.emit(data_register(C::Al, Op::And, r(3), r(3), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Orr, r(3), r(3), r(7), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(7), r(0), 0x4)?)?;
    c.emit(transfer_word(C::Al, false, r(3), r(2), 0)?)?;
    c.emit(data_register(C::Al, Op::And, r(5), r(7), r(5), Shift::Lsl, 2, false)?)?;
    c.emit(data_immediate(C::Al, Op::And, r(6), r(6), 0x1)?)?;
    c.emit(data_register(C::Al, Op::And, r(4), r(12), r(4), Shift::Lsl, 2, false)?)?;
    c.emit(transfer_word(C::Al, true, r(3), r(13), 52)?)?;
    c.emit(data_register(C::Al, Op::Orr, r(7), r(4), r(14), Shift::Lsl, 18, false)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(4), r(0), 0x1000)?)?;
    c.emit(data_register(C::Al, Op::Orr, r(5), r(5), r(6), Shift::Lsl, 18, false)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(6), r(0), 0x1000)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(6), r(6), 0x10000000)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(4), r(4), 0x10000000)?)?;
    c.emit(data_register(C::Al, Op::And, r(7), r(7), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Orr, r(5), r(5), r(6), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Orr, r(6), r(7), r(4), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(7), r(0), 0x4)?)?;
    c.emit(transfer_word(C::Al, false, r(3), r(2), 4)?)?;
    c.emit(transfer_word(C::Al, false, r(6), r(2), 8)?)?;
    c.emit(transfer_word(C::Al, false, r(5), r(2), 12)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(2), r(2), 0x10)?)?;
    c.emit(transfer_word(C::Al, true, r(3), r(11), -76)?)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(8), r(3), Shift::Lsl, 0, false)?)?;
    c.branch(C::Ne, 0x78060, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(10), 604)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(13), 24)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(4), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Ne, 0x78140, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(1), 428)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x1)?)?;
    c.emit(transfer_word(C::Ne, true, r(0), r(1), 420)?)?;
    c.emit(data_immediate(C::Ne, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Ne, 0x78224, false)?;
    c.literal(C::Al, r(2), 0x774d0)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0xe1)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x400)?)?;
    c.branch(C::Al, 0x17b0, true)?;
    c.emit(transfer_word(C::Al, true, r(0), r(10), 2104)?)?;
    c.emit(transfer_byte(C::Al, true, r(7), r(0), 232)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(7), 0x0)?)?;
    c.branch(C::Ne, 0x78224, false)?;
    c.branch(C::Al, 0x78248, false)?;
    c.data(0x78168, 0xf40b0)?;
    c.data(0x7816c, 0x3008001)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(4), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(2), 0x0)?)?;
    c.branch(C::Eq, 0x78224, false)?;
    c.emit(transfer_word(C::Al, true, r(12), r(9), 400)?)?;
    c.emit(transfer_word(C::Al, true, r(3), r(13), 20)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(14), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(7), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(12), 0x0)?)?;
    c.branch(C::Eq, 0x781f8, false)?;
    c.emit(transfer_word(C::Al, true, r(1), r(11), -64)?)?;
    c.emit(transfer_byte_register_preindexed_writeback(C::Al, true, r(8), r(1), r(14), Shift::Lsl, 0)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(14), r(14), 0x8)?)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(1), 7)?)?;
    c.emit(transfer_byte(C::Al, true, r(6), r(1), 6)?)?;
    c.emit(transfer_byte(C::Al, true, r(2), r(1), 1)?)?;
    c.emit(transfer_byte(C::Al, true, r(5), r(1), 2)?)?;
    c.emit(transfer_byte(C::Al, true, r(4), r(1), 3)?)?;
    c.emit(data_immediate(C::Al, Op::And, r(0), r(0), 0x1)?)?;
    c.emit(data_immediate(C::Al, Op::And, r(6), r(6), 0x1)?)?;
    c.emit(data_immediate(C::Al, Op::And, r(2), r(2), 0x1)?)?;
    c.emit(data_register(C::Al, Op::Orr, r(0), r(6), r(0), Shift::Lsl, 16, false)?)?;
    c.emit(transfer_byte(C::Al, true, r(6), r(1), 5)?)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(1), 4)?)?;
    c.emit(data_immediate(C::Al, Op::And, r(6), r(6), 0x1)?)?;
    c.emit(data_immediate(C::Al, Op::And, r(1), r(1), 0x1)?)?;
    c.emit(data_register(C::Al, Op::Orr, r(6), r(1), r(6), Shift::Lsl, 16, false)?)?;
    c.emit(data_immediate(C::Al, Op::And, r(1), r(4), 0x1)?)?;
    c.emit(data_immediate(C::Al, Op::And, r(4), r(5), 0x1)?)?;
    c.emit(data_register(C::Al, Op::Orr, r(5), r(4), r(1), Shift::Lsl, 16, false)?)?;
    c.emit(data_immediate(C::Al, Op::And, r(1), r(8), 0x1)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(4), r(0), 0x0)?)?;
    c.emit(data_register(C::Al, Op::Orr, r(1), r(1), r(2), Shift::Lsl, 16, false)?)?;
    c.branch(C::Al, 0x78208, false)?;
    c.data(0x781f4, 0xe00e0008)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(5), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(6), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(transfer_multiple(C::Al, false, r(3), 0x62, BlockMode::IncrementAfter, false)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(3), 12)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(7), r(7), 0x1)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(3), r(3), 0x10)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(11), -76)?)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(7), r(0), Shift::Lsl, 0, false)?)?;
    c.branch(C::Ne, 0x7818c, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(11), -64)?)?;
    c.branch(C::Al, 0x2f1c, true)?;
    c.emit(transfer_word(C::Al, true, r(1), r(10), 1016)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(13), 20)?)?;
    c.branch(C::Al, 0x205c, true)?;
    c.emit(transfer_word(C::Al, true, r(1), r(10), 2028)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(10), 696)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(1), Shift::Lsl, 1, false)?)?;
    c.branch(C::Al, 0x205c, true)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x1)?)?;
    c.emit(transfer_word(C::Al, true, r(7), r(9), 40)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 420)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(4), r(0), 0x0)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(9), 936)?)?;
    c.emit(transfer_word(C::Al, true, r(3), r(10), 2104)?)?;
    c.emit(transfer_word(C::Al, true, r(2), r(10), 80)?)?;
    c.emit(transfer_word(C::Al, true, r(6), r(10), 76)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(0), 24)?)?;
    c.emit(transfer_byte(C::Al, true, r(3), r(3), 336)?)?;
    c.emit(transfer_word(C::Al, false, r(4), r(7), 32)?)?;
    c.emit(transfer_word(C::Al, false, r(4), r(7), 36)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(8), r(0), r(2), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate_with_flags(C::Al, Op::And, r(3), r(3), 0x1, true)?)?;
    c.emit(data_register(C::Ne, Op::Mov, r(8), r(0), r(6), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Ne, Op::Mov, r(6), r(0), r(2), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, true, r(2), r(10), 752)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(2), 0x2)?)?;
    c.branch(C::Eq, 0x782a0, false)?;
    c.emit(transfer_byte(C::Al, true, r(2), r(10), 208)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(2), 0x0)?)?;
    c.branch(C::Eq, 0x78380, false)?;
    c.emit(data_immediate(C::Al, Op::Add, r(5), r(0), 0x4c)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(2), r(0), 0x48)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(3), r(0), 0x68)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(1), r(0), 0x64)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(2), 0)?)?;
    c.emit(transfer_word(C::Al, true, r(5), r(5), 0)?)?;
    c.emit(transfer_word(C::Al, true, r(2), r(3), 0)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(1), 0)?)?;
    c.emit(transfer_byte(C::Al, false, r(4), r(7), 40)?)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(0), r(6), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(7), 16)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(7), 20)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(7), 24)?)?;
    c.emit(transfer_word(C::Al, false, r(5), r(7), 28)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(10), 44)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(10), 48)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(10), 52)?)?;
    c.emit(transfer_word(C::Al, false, r(5), r(10), 56)?)?;
    c.branch(C::Ls, 0x78320, false)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x1)?)?;
    c.emit(transfer_byte(C::Al, false, r(1), r(7), 40)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(6), Shift::Lsl, 3, false)?)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(1), r(0), Shift::Lsl, 0, false)?)?;
    c.branch(C::Cs, 0x78490, false)?;
    c.emit(transfer_word(C::Al, false, r(6), r(7), 24)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(10), 32)?)?;
    c.emit(data_register(C::Al, Op::Sub, r(0), r(0), r(6), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(0), Shift::Lsr, 1, false)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(7), 16)?)?;
    c.branch(C::Al, 0x7848c, false)?;
    c.data(0x78318, 0xf3ed2)?;
    c.data(0x7831c, 0xf3f42)?;
    c.branch(C::Cs, 0x784b0, false)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x1)?)?;
    c.emit(transfer_byte(C::Al, false, r(1), r(7), 40)?)?;
    c.emit(data_register(C::Al, Op::Add, r(1), r(0), r(0), Shift::Lsl, 2, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(1), Shift::Lsl, 2, false)?)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(1), r(6), Shift::Lsl, 0, false)?)?;
    c.branch(C::Cs, 0x78490, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(10), 32)?)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(0), r(6), Shift::Lsl, 0, false)?)?;
    c.branch(C::Cs, 0x7847c, false)?;
    c.emit(transfer_word(C::Al, false, r(4), r(7), 16)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(7), 24)?)?;
    c.emit(data_register(C::Al, Op::Add, r(0), r(0), r(0), Shift::Lsl, 2, false)?)?;
    c.literal(C::Al, r(2), 0x791e8)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0xb3)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(0), Shift::Lsl, 2, false)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x300)?)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(0), r(6), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Cs, Op::Mov, r(0), r(0), 0x1)?)?;
    c.branch(C::Al, 0x17b0, true)?;
    c.emit(transfer_word(C::Al, true, r(0), r(7), 24)?)?;
    c.emit(transfer_word(C::Al, true, r(5), r(7), 28)?)?;
    c.branch(C::Al, 0x78490, false)?;
    c.emit(data_immediate_with_flags(C::Al, Op::And, r(0), r(1), 0x40, true)?)?;
    c.branch(C::Eq, 0x783ac, false)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(10), 382)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x1)?)?;
    c.branch(C::Ne, 0x783ac, false)?;
    c.emit(data_immediate(C::Al, Op::Add, r(5), r(9), 0x390)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(2), r(9), 0x38c)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(3), r(9), 0x388)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(1), r(9), 0x384)?)?;
    c.branch(C::Al, 0x782b0, false)?;
    c.data(0x783a8, 0xf3f71)?;
    c.emit(transfer_word(C::Al, true, r(0), r(10), 44)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(7), 16)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(10), 48)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(7), 20)?)?;
    c.emit(transfer_word(C::Al, true, r(5), r(10), 52)?)?;
    c.emit(transfer_word(C::Al, false, r(5), r(7), 24)?)?;
    c.emit(data_register(C::Al, Op::Sub, r(1), r(5), r(6), Shift::Lsl, 0, true)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(10), 56)?)?;
    c.emit(data_immediate(C::Mi, Op::Rsb, r(1), r(1), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x1)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(7), 28)?)?;
    c.emit(transfer_byte(C::Al, false, r(4), r(7), 40)?)?;
    c.branch(C::Lt, 0x78454, false)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(5), Shift::Lsr, 4, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(6), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), r(0), 0x0)?)?;
    c.emit(transfer_byte(C::Al, false, r(0), r(7), 40)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(5), Shift::Lsl, 28, false)?)?;
    c.emit(data_register(C::Al, Op::Add, r(0), r(0), r(6), Shift::Lsr, 1, true)?)?;
    c.emit(data_immediate(C::Al, Op::Adc, r(1), r(1), 0x0)?)?;
    c.branch(C::Al, 0x8187c, true)?;
    c.emit(transfer_word(C::Al, false, r(0), r(7), 32)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(6), Shift::Lsl, 3, false)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x380)?)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.literal(C::Al, r(5), 0x791e8)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Cs, Op::Mov, r(0), r(0), 0x1)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x17b0, true)?;
    c.emit(transfer_word(C::Al, true, r(0), r(7), 24)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x81)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x300)?)?;
    c.emit(data_register(C::Al, Op::Add, r(0), r(0), r(0), Shift::Lsl, 2, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(0), Shift::Lsl, 2, false)?)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(0), r(6), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Cs, Op::Mov, r(0), r(0), 0x1)?)?;
    c.branch(C::Al, 0x17b0, true)?;
    c.emit(transfer_word(C::Al, true, r(0), r(7), 28)?)?;
    c.emit(data_register(C::Al, Op::Sub, r(1), r(0), r(8), Shift::Lsl, 0, true)?)?;
    c.emit(data_immediate(C::Mi, Op::Rsb, r(1), r(1), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x1)?)?;
    c.branch(C::Lt, 0x7857c, false)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(7), 40)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x2)?)?;
    c.emit(transfer_byte(C::Al, false, r(1), r(7), 40)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(0), Shift::Lsr, 4, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(0), Shift::Lsl, 28, false)?)?;
    c.branch(C::Al, 0x78564, false)?;
    c.emit(data_register(C::Al, Op::Sub, r(0), r(0), r(6), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(0), Shift::Lsr, 1, false)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(7), 16)?)?;
    c.emit(transfer_word(C::Al, false, r(6), r(7), 24)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(6), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(0), Shift::Lsr, 4, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(0), Shift::Lsl, 28, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(6), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), r(0), 0x0)?)?;
    c.emit(data_register(C::Al, Op::Add, r(0), r(0), r(6), Shift::Lsr, 1, true)?)?;
    c.emit(data_immediate(C::Al, Op::Adc, r(1), r(1), 0x0)?)?;
    c.branch(C::Al, 0x8187c, true)?;
    c.emit(transfer_word(C::Al, false, r(0), r(7), 32)?)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(5), r(8), Shift::Lsl, 0, false)?)?;
    c.branch(C::Ls, 0x784ec, false)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(7), 40)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(0), r(0), 0x2)?)?;
    c.emit(transfer_byte(C::Al, false, r(0), r(7), 40)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(8), Shift::Lsl, 3, false)?)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.branch(C::Cs, 0x7855c, false)?;
    c.emit(transfer_word(C::Al, false, r(8), r(7), 28)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(5), r(0), r(8), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(10), 36)?)?;
    c.emit(data_register(C::Al, Op::Sub, r(0), r(0), r(8), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(0), Shift::Lsr, 1, false)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(7), 20)?)?;
    c.branch(C::Al, 0x7855c, false)?;
    c.branch(C::Cs, 0x7857c, false)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(7), 40)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(0), r(0), 0x2)?)?;
    c.emit(transfer_byte(C::Al, false, r(0), r(7), 40)?)?;
    c.emit(data_register(C::Al, Op::Add, r(0), r(5), r(5), Shift::Lsl, 2, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(0), Shift::Lsl, 2, false)?)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(0), r(8), Shift::Lsl, 0, false)?)?;
    c.branch(C::Cs, 0x7855c, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(10), 36)?)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(0), r(8), Shift::Lsl, 0, false)?)?;
    c.branch(C::Cs, 0x78548, false)?;
    c.emit(transfer_word(C::Al, false, r(4), r(7), 20)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(7), 28)?)?;
    c.emit(data_register(C::Al, Op::Add, r(0), r(0), r(0), Shift::Lsl, 2, false)?)?;
    c.literal(C::Al, r(2), 0x791e8)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x3d4)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(0), Shift::Lsl, 2, false)?)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(0), r(8), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Cs, Op::Mov, r(0), r(0), 0x1)?)?;
    c.branch(C::Al, 0x17b0, true)?;
    c.emit(transfer_word(C::Al, true, r(5), r(7), 28)?)?;
    c.branch(C::Al, 0x7855c, false)?;
    c.emit(data_register(C::Al, Op::Sub, r(0), r(0), r(8), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(5), r(0), r(8), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(0), Shift::Lsr, 1, false)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(7), 20)?)?;
    c.emit(transfer_word(C::Al, false, r(8), r(7), 28)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(5), Shift::Lsr, 4, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(5), Shift::Lsl, 28, false)?)?;
    c.emit(data_register(C::Al, Op::Add, r(0), r(0), r(8), Shift::Lsr, 1, true)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(8), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Adc, r(1), r(1), 0x0)?)?;
    c.branch(C::Al, 0x8187c, true)?;
    c.emit(transfer_word(C::Al, false, r(0), r(7), 36)?)?;
    c.emit(transfer_word(C::Al, true, r(2), r(9), 40)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(10), 80)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(10), 76)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(5), r(0), 0xff)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(5), r(5), 0xff00)?)?;
    c.emit(transfer_word(C::Al, true, r(3), r(2), 24)?)?;
    c.emit(transfer_word(C::Al, true, r(2), r(2), 28)?)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(2), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Hi, Op::Mov, r(1), r(0), 0x1)?)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(3), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Hi, Op::Mov, r(0), r(0), 0x1)?)?;
    c.emit(data_register(C::Al, Op::Orr, r(0), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_byte(C::Al, false, r(0), r(9), 1437)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 400)?)?;
    c.emit(transfer_word(C::Al, true, r(8), r(11), -60)?)?;
    c.emit(transfer_word(C::Al, true, r(7), r(13), 56)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Eq, 0x7865c, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(10), 2104)?)?;
    c.emit(transfer_word(C::Al, true, r(5), r(13), 68)?)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(0), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x0)?)?;
    c.branch(C::Eq, 0x78708, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 400)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Eq, 0x7868c, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(10), 292)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(10), 288)?)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(10), 333)?)?;
    c.emit(transfer_byte(C::Al, false, r(0), r(10), 332)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(10), 300)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(10), 1212)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(10), 1056)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(10), 1060)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(10), 1216)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(10), 1064)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x74)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(0), r(0), 0x400)?)?;
    c.emit(data_register(C::Al, Op::Add, r(0), r(10), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, true, r(2), r(0), 96)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(1), r(1), 0x1)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x4)?)?;
    c.emit(transfer_word_postindexed(C::Al, false, r(2), r(0), 4)?)?;
    c.branch(C::Ne, 0x78628, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(10), 1228)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(10), 1132)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(10), 1224)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(10), 1116)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(10), 1220)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(10), 1260)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(1), r(10), 0x4d0)?)?;
    c.branch(C::Al, 0x786f8, false)?;
    c.branch(C::Al, 0x2f8d0, true)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Eq, 0x78ab0, false)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(10), 2632)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Ne, 0x785cc, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(10), 2104)?)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(0), 336)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x0)?)?;
    c.branch(C::Ne, 0x785cc, false)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(0), 324)?)?;
    c.branch(C::Al, 0x78ab4, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(10), 2012)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(10), 288)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(transfer_byte(C::Al, false, r(0), r(10), 332)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(10), 300)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(10), 1172)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(10), 1056)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(10), 1060)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(10), 1176)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(10), 1064)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x74)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x400)?)?;
    c.emit(data_register(C::Al, Op::Add, r(1), r(10), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, true, r(2), r(1), 56)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(0), 0x1)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x4)?)?;
    c.emit(transfer_word_postindexed(C::Al, false, r(2), r(1), 4)?)?;
    c.branch(C::Ne, 0x786c0, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(10), 1188)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(10), 1132)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(10), 1184)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(10), 1116)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(10), 1180)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(10), 1260)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0xa8)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x400)?)?;
    c.emit(data_register(C::Al, Op::Add, r(1), r(10), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(1), 0)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(10), 1264)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(10), 1268)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(10), 2104)?)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(0), 339)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Eq, 0x78728, false)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(8), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(10), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x503e4, true)?;
    c.branch(C::Al, 0x78748, false)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(8), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(10), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x4ff84, true)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(8), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(10), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x502b8, true)?;
    c.emit(transfer_word(C::Al, true, r(0), r(10), 268)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Eq, 0x78768, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(10), 4)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x2000)?)?;
    c.branch(C::Ne, 0x78768, false)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(8), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x617ec, true)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(5), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Eq, 0x787ac, false)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(9), 4)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x2)?)?;
    c.branch(C::Eq, 0x78798, false)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x1)?)?;
    c.branch(C::Eq, 0x787a0, false)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Ne, 0x787b8, false)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.branch(C::Al, 0x787a4, false)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x3)?)?;
    c.branch(C::Al, 0x787a4, false)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x2)?)?;
    c.emit(transfer_byte(C::Al, false, r(0), r(9), 1436)?)?;
    c.branch(C::Al, 0x787cc, false)?;
    c.emit(transfer_byte(C::Al, false, r(4), r(9), 1436)?)?;
    c.branch(C::Al, 0x787cc, false)?;
    c.data(0x787b4, 0xf3e7e)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x71)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x700)?)?;
    c.literal(C::Al, r(2), 0x791e8)?;
    c.branch(C::Al, 0x17b0, true)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 1808)?)?;
    c.emit(data_register(C::Al, Op::Add, r(1), r(9), r(0), Shift::Lsl, 2, false)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(1), 1800)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(9), 1492)?)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(10), 351)?)?;
    c.emit(data_register(C::Al, Op::Eor, r(0), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(9), 1808)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x3)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(9), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(9), 0x640)?)?;
    c.branch(C::Al, 0x35c14, true)?;
    c.emit(data_register(C::Al, Op::Mov, r(5), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(5), 0x0)?)?;
    c.branch(C::Eq, 0x791d0, false)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(6), r(0), 0x0)?)?;
    c.emit(transfer_word(C::Al, false, r(6), r(5), 304)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 500)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(5), 244)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 504)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(5), 248)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 36)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(0), 368)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(0), 76)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(5), 260)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 456)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(5), 264)?)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(9), 369)?)?;
    c.emit(transfer_byte(C::Al, false, r(0), r(5), 7)?)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(9), 370)?)?;
    c.emit(transfer_byte(C::Al, false, r(0), r(5), 5)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 400)?)?;
    c.emit(transfer_byte(C::Al, false, r(0), r(5), 0)?)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(9), 380)?)?;
    c.emit(transfer_byte(C::Al, false, r(0), r(5), 1)?)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(9), 1436)?)?;
    c.emit(transfer_byte(C::Al, false, r(0), r(5), 10)?)?;
    c.emit(transfer_byte(C::Al, false, r(0), r(5), 9)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 972)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(5), 20)?)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(9), 371)?)?;
    c.emit(transfer_byte(C::Al, false, r(0), r(5), 6)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 412)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(5), 12)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 416)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(5), 16)?)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(9), 378)?)?;
    c.emit(transfer_byte(C::Al, false, r(0), r(5), 8)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.emit(transfer_byte(C::Al, false, r(0), r(5), 4)?)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(9), 373)?)?;
    c.emit(transfer_byte(C::Al, false, r(0), r(5), 2)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 508)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(5), 432)?)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(9), 1569)?)?;
    c.emit(transfer_byte(C::Al, false, r(0), r(5), 428)?)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(9), 398)?)?;
    c.emit(transfer_byte(C::Al, false, r(0), r(5), 204)?)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(9), 399)?)?;
    c.emit(transfer_byte(C::Al, false, r(0), r(5), 206)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 420)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(0), 52)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(5), 68)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(0), 60)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(5), 292)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(0), 56)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(5), 288)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(0), 92)?)?;
    c.emit(transfer_word(C::Al, true, r(2), r(0), 96)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(5), 60)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(5), 56)?)?;
    c.emit(transfer_doubleword(C::Al, true, r(2), r(7), 0)?)?;
    c.emit(transfer_word(C::Al, false, r(3), r(5), 196)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(5), 192)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x50)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(0), 24)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(5), 32)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(0), 152)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(5), 64)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(0), 32)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(5), 40)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(0), 28)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(5), 36)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(0), 48)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(5), 44)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(0), 40)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(5), 76)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(0), 44)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(5), 72)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 480)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(5), 188)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x11c)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(0), r(0), 0x400)?)?;
    c.emit(data_register(C::Al, Op::Add, r(1), r(9), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(5), 0x50)?)?;
    c.branch(C::Al, 0x471c, true)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(9), 373)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x1)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.branch(C::Ne, 0x7896c, false)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(10), 208)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(transfer_word(C::Ne, true, r(0), r(10), 2708)?)?;
    c.emit(data_immediate(C::Ne, Op::Sub, r(0), r(0), 0x1)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(5), 424)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(1), r(5), 0x138)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 1448)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(3), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(5), 212)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 1468)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(5), 228)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 1480)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(5), 236)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 1484)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(5), 240)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 1488)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(5), 252)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 1492)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(5), 268)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 1456)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(5), 220)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 1460)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(5), 224)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x7e)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(0), r(0), 0x100)?)?;
    c.emit(data_register(C::Al, Op::Add, r(0), r(9), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_byte_postindexed(C::Al, true, r(7), r(2), 2)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(6), r(6), 0x1)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(6), 0x4)?)?;
    c.emit(transfer_byte_postindexed(C::Al, false, r(7), r(3), 1)?)?;
    c.branch(C::Ne, 0x789c8, false)?;
    c.emit(transfer_byte(C::Al, true, r(2), r(9), 1001)?)?;
    c.emit(transfer_byte(C::Al, false, r(2), r(5), 316)?)?;
    c.emit(transfer_byte(C::Al, true, r(2), r(9), 1002)?)?;
    c.emit(transfer_byte(C::Al, false, r(2), r(5), 317)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x0)?)?;
    c.emit(transfer_byte_postindexed(C::Al, true, r(3), r(0), 2)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(2), r(2), 0x1)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(2), 0x4)?)?;
    c.emit(transfer_byte_postindexed(C::Al, false, r(3), r(1), 1)?)?;
    c.branch(C::Ne, 0x789f0, false)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(10), 385)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Eq, 0x78a1c, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 964)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.emit(transfer_word(C::Ne, false, r(4), r(9), 964)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 1500)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(12), r(9), 0x5e0)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(1), r(5), 0x14c)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(14), r(0), 0x0)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(5), 328)?)?;
    c.emit(transfer_multiple(C::Al, true, r(12), 0xdc, BlockMode::IncrementAfter, true)?)?;
    c.emit(transfer_multiple(C::Al, false, r(1), 0xdc, BlockMode::IncrementAfter, true)?)?;
    c.emit(transfer_multiple(C::Al, true, r(12), 0xdc, BlockMode::IncrementAfter, true)?)?;
    c.emit(transfer_multiple(C::Al, false, r(1), 0xdc, BlockMode::IncrementAfter, true)?)?;
    c.emit(transfer_multiple(C::Al, true, r(12), 0xdd, BlockMode::IncrementAfter, false)?)?;
    c.emit(transfer_multiple(C::Al, false, r(1), 0xdd, BlockMode::IncrementAfter, false)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 1304)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(5), 396)?)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(9), 831)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(5), 404)?)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(9), 830)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(5), 408)?)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(9), 832)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(5), 412)?)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(9), 834)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(5), 400)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 1572)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(5), 320)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 1576)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(5), 324)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 1580)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(5), 416)?)?;
    c.emit(transfer_word(C::Al, false, r(14), r(9), 1580)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 1584)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(5), 420)?)?;
    c.emit(transfer_word(C::Al, true, r(6), r(11), -72)?)?;
    c.emit(transfer_word(C::Al, true, r(4), r(11), -68)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(6), 0x1)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(9), r(11), 0x38)?)?;
    c.emit(data_immediate(C::Ne, Op::Mov, r(1), r(0), 0x1)?)?;
    c.branch(C::Al, 0x78ef4, false)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(9), 1437)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Ne, 0x785cc, false)?;
    c.emit(transfer_word(C::Al, true, r(1), r(13), 64)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(1), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Eq, 0x78ae8, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(13), 84)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(4), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x3701c, true)?;
    c.emit(transfer_word(C::Al, false, r(0), r(10), 2508)?)?;
    c.emit(transfer_word(C::Al, true, r(2), r(4), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(4), r(0), 0x0)?)?;
    c.branch(C::Al, 0x78aec, false)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x0)?)?;
    c.literal(C::Al, r(1), 0x7920c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.literal(C::Al, r(1), 0x791f4)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.emit(transfer_word(C::Al, true, r(0), r(10), 2508)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(10), 1140)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x2)?)?;
    c.branch(C::Cc, 0x78b34, false)?;
    c.emit(data_register(C::Al, Op::Mov, r(12), r(0), r(7), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x0)?)?;
    c.branch(C::Eq, 0x78c60, false)?;
    c.emit(transfer_word(C::Al, true, r(2), r(10), 2720)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(0), r(2), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Ne, Op::Mov, r(1), r(0), 0x1)?)?;
    c.branch(C::Al, 0x78c64, false)?;
    c.emit(transfer_word(C::Al, true, r(2), r(10), 2104)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), r(0), 0x56)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(3), r(3), 0x100)?)?;
    c.emit(data_immediate(C::Ne, Op::Mov, r(1), r(0), 0x1)?)?;
    c.emit(transfer_byte(C::Al, false, r(4), r(2), 325)?)?;
    c.emit(transfer_halfword_register(C::Al, false, r(4), r(2), r(3))?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), r(0), 0x158)?)?;
    c.emit(transfer_halfword_register(C::Al, false, r(4), r(2), r(3))?)?;
    c.emit(transfer_word(C::Al, false, r(4), r(10), 1140)?)?;
    c.emit(transfer_word(C::Al, false, r(4), r(10), 1236)?)?;
    c.emit(transfer_word(C::Al, false, r(4), r(10), 1144)?)?;
    c.emit(transfer_word(C::Al, false, r(4), r(10), 1240)?)?;
    c.emit(transfer_byte(C::Al, true, r(2), r(2), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(2), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x0)?)?;
    c.emit(transfer_word(C::Ne, false, r(4), r(10), 1196)?)?;
    c.emit(transfer_word(C::Ne, false, r(4), r(10), 1200)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(10), 1096)?)?;
    c.branch(C::Al, 0x78cc8, false)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(6), 0x0)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(13), 36)?)?;
    c.emit(transfer_word(C::Al, false, r(8), r(13), 40)?)?;
    c.emit(transfer_word(C::Al, false, r(12), r(13), 32)?)?;
    c.branch(C::Eq, 0x78f4c, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(12), 72)?)?;
    c.emit(transfer_word(C::Al, true, r(5), r(12), 60)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(8), r(0), 0x0)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(11), -76)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(10), 2020)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(10), 2028)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(13), 48)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(13), 52)?)?;
    c.emit(data_immediate(C::Al, Op::Rsb, r(0), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::And, r(0), r(0), 0x7)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(13), 44)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(13), 48)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(5), 0x1)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(7), r(0), r(6), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Sub, r(5), r(0), r(1), Shift::Lsl, 0, true)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(13), 52)?)?;
    c.emit(data_register(C::Cc, Op::Mov, r(5), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(11), -76)?)?;
    c.emit(data_register(C::Al, Op::Add, r(0), r(0), r(5), Shift::Lsl, 1, false)?)?;
    c.emit(load_halfword(C::Al, r(6), r(0), 0)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(6), Shift::Lsl, 0, false)?)?;
    c.thumb_call(0x82be4)?;
    c.emit(transfer_word(C::Al, true, r(2), r(13), 44)?)?;
    c.emit(transfer_word(C::Al, true, r(14), r(11), -64)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(8), r(8), 0x1)?)?;
    c.emit(multiply_accumulate(C::Al, r(1), r(0), r(2), r(6))?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0xff)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(6), r(0), r(7), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(0), r(0), 0xff00)?)?;
    c.emit(store_halfword_postindexed(C::Al, r(1), r(4), 2)?)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(8), r(6), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::And, r(0), r(1), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x400)?)?;
    c.emit(data_register(C::Al, Op::Add, r(0), r(14), r(0), Shift::Lsl, 1, false)?)?;
    c.emit(store_halfword(C::Al, r(1), r(0), 0)?)?;
    c.branch(C::Ne, 0x78bc4, false)?;
    c.branch(C::Al, 0x78f50, false)?;
    c.literal(C::Al, r(1), 0x79204)?;
    c.literal(C::Al, r(2), 0x79208)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), r(0), 0x3)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(13), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x10)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(3), r(3), 0x600)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.literal(C::Al, r(1), 0x791f4)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x10)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.emit(transfer_word(C::Al, true, r(14), r(11), -64)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(10), 1016)?)?;
    c.branch(C::Al, 0x78f60, false)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x1)?)?;
    c.emit(transfer_word(C::Al, true, r(2), r(10), 2104)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(7), r(0), 0x56)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), r(0), 0x3)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(7), r(7), 0x100)?)?;
    c.emit(transfer_byte(C::Al, false, r(3), r(2), 325)?)?;
    c.emit(data_register(C::Al, Op::Add, r(3), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_halfword_register(C::Al, false, r(3), r(2), r(7))?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(7), r(0), 0x158)?)?;
    c.emit(transfer_halfword_register(C::Al, false, r(3), r(2), r(7))?)?;
    c.emit(data_register(C::Al, Op::Mov, r(7), r(0), r(12), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, true, r(3), r(10), 1276)?)?;
    c.emit(transfer_word(C::Al, false, r(3), r(10), 1140)?)?;
    c.emit(transfer_word(C::Al, false, r(3), r(10), 1236)?)?;
    c.emit(transfer_word(C::Al, true, r(3), r(10), 1280)?)?;
    c.emit(transfer_word(C::Al, false, r(3), r(10), 1144)?)?;
    c.emit(transfer_word(C::Al, false, r(3), r(10), 1240)?)?;
    c.emit(transfer_byte(C::Al, true, r(2), r(2), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(2), 0x0)?)?;
    c.emit(transfer_word(C::Ne, true, r(2), r(10), 1284)?)?;
    c.emit(transfer_word(C::Ne, false, r(2), r(10), 1196)?)?;
    c.emit(transfer_word(C::Ne, true, r(2), r(10), 1288)?)?;
    c.emit(transfer_word(C::Ne, false, r(2), r(10), 1200)?)?;
    c.emit(transfer_word(C::Al, true, r(2), r(10), 1292)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(10), 1096)?)?;
    c.emit(transfer_word(C::Al, true, r(2), r(10), 1296)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x0)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(10), 1100)?)?;
    c.emit(transfer_word(C::Ne, false, r(4), r(10), 1696)?)?;
    c.emit(transfer_word(C::Ne, false, r(4), r(10), 1704)?)?;
    c.emit(transfer_word(C::Ne, false, r(4), r(10), 1712)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(10), 2720)?)?;
    c.branch(C::Al, 0x785cc, false)?;
    c.data(0x78ce4, 0xf40ff7ff)?;
    c.data(0x78ce8, 0xf7fff7ff)?;
    c.emit(transfer_word(C::Al, true, r(2), r(11), -72)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(0), 0)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(2), 0)?)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(8), 990)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x1)?)?;
    c.branch(C::Eq, 0x78d1c, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 308)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Ne, 0x78d1c, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(10), 24)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x2)?)?;
    c.branch(C::Cc, 0x78d90, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 296)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Ne, 0x78d90, false)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(5), 2)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x1)?)?;
    c.branch(C::Eq, 0x78d90, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(8), 1556)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(1), r(0), 0x1)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(8), 1556)?)?;
    c.emit(data_register(C::Al, Op::Add, r(1), r(8), r(0), Shift::Lsl, 2, false)?)?;
    c.emit(transfer_word(C::Al, true, r(6), r(1), 2272)?)?;
    c.emit(transfer_word(C::Al, false, r(6), r(5), 296)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(8), 1532)?)?;
    c.branch(C::Al, 0x81850, true)?;
    c.emit(data_register(C::Al, Op::Mov, r(3), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.literal(C::Al, r(1), 0x791f8)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(6), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.literal(C::Al, r(1), 0x791f4)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.emit(transfer_word(C::Al, true, r(1), r(8), 1532)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(8), 1556)?)?;
    c.branch(C::Al, 0x81850, true)?;
    c.emit(transfer_word(C::Al, false, r(1), r(8), 1556)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 296)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Eq, 0x78f2c, false)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(7), 373)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Ne, 0x78e08, false)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1b8)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(0), r(0), 0x400)?)?;
    c.emit(data_register(C::Al, Op::Add, r(2), r(7), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(10), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x3f34c, true)?;
    c.emit(transfer_word(C::Al, true, r(1), r(7), 1452)?)?;
    c.emit(data_register(C::Al, Op::Add, r(0), r(1), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(7), 1452)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(10), 2104)?)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(0), 339)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Ne, 0x78e08, false)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(10), 350)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(2), r(7), 0x5c0)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x2)?)?;
    c.branch(C::Ne, 0x78df0, false)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(10), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x3f20c, true)?;
    c.branch(C::Al, 0x78dfc, false)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(10), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x3ee98, true)?;
    c.emit(transfer_word(C::Al, true, r(1), r(7), 1476)?)?;
    c.emit(data_register(C::Al, Op::Add, r(0), r(1), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(7), 1476)?)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(7), 369)?)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(7), 373)?)?;
    c.emit(transfer_byte(C::Al, true, r(6), r(7), 380)?)?;
    c.emit(transfer_word(C::Al, true, r(3), r(7), 400)?)?;
    c.emit(transfer_word(C::Al, true, r(2), r(7), 416)?)?;
    c.emit(transfer_word(C::Al, false, r(6), r(13), 0)?)?;
    c.emit(transfer_multiple(C::Al, false, r(13), 0x3, BlockMode::IncrementBefore, false)?)?;
    c.literal(C::Al, r(1), 0x791f0)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x2)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x2)?)?;
    c.literal(C::Al, r(1), 0x791f4)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.emit(transfer_word(C::Al, true, r(0), r(10), 300)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(5), 308)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(7), 1452)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(5), 216)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(7), 1476)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(5), 232)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(7), 428)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(5), 24)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(11), -64)?)?;
    c.branch(C::Al, 0x35ca0, true)?;
    c.emit(transfer_word(C::Al, true, r(0), r(7), 420)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(0), 52)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(8), 280)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x2)?)?;
    c.branch(C::Ne, 0x78eac, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(8), 16)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(8), 276)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(11), -56)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x168)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(0), r(0), 0x1400)?)?;
    c.emit(transfer_word_register(C::Al, true, r(0), r(1), r(0), Shift::Lsl, 0)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(11), -52)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(1), 0x90)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(9), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x80928, true)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x1)?)?;
    c.branch(C::Ne, 0x78f00, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(8), 48)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(0), 4)?)?;
    c.branch(C::Al, 0x292c, true)?;
    c.emit(transfer_word(C::Al, true, r(0), r(7), 412)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(0), 0x1)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(7), 412)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(10), 2104)?)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(0), 339)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Ne, Op::Mov, r(0), r(0), 0x2)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(7), 0)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(10), 748)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Ne, Op::Mov, r(0), r(0), 0x1)?)?;
    c.emit(transfer_byte(C::Al, false, r(0), r(7), 5)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(7), 20)?)?;
    c.branch(C::Al, 0x292c, true)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x1)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x0)?)?;
    c.branch(C::Eq, 0x78f7c, false)?;
    c.branch(C::Al, 0x79148, false)?;
    c.literal(C::Al, r(1), 0x791fc)?;
    c.literal(C::Al, r(2), 0x79200)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), r(0), 0x99)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x8)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(3), r(3), 0x100)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.literal(C::Al, r(1), 0x791f4)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x8)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.branch(C::Al, 0x78ef4, false)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x61)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x100)?)?;
    c.branch(C::Al, 0x769f4, false)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(6), r(0), 0x0)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(13), 32)?)?;
    c.emit(transfer_word(C::Al, false, r(6), r(0), 60)?)?;
    c.branch(C::Al, 0x78f58, false)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(5), r(0), 0x0)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(13), 32)?)?;
    c.emit(transfer_word(C::Al, false, r(5), r(0), 60)?)?;
    c.emit(transfer_word(C::Al, true, r(8), r(13), 40)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(13), 36)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(14), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x205c, true)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(10), 351)?)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(8), 112)?)?;
    c.emit(data_register(C::Al, Op::Eor, r(0), r(1), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_byte(C::Al, false, r(0), r(8), 112)?)?;
    c.branch(C::Al, 0x78248, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x4)?)?;
    c.branch(C::Ls, 0x767a8, false)?;
    c.branch(C::Al, 0x79148, false)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(8), 12)?)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(8), 13)?)?;
    c.emit(data_register(C::Al, Op::And, r(0), r(1), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(8), 14)?)?;
    c.emit(data_register(C::Al, Op::And, r(0), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_byte(C::Al, false, r(0), r(11), -56)?)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(11), -56)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x1)?)?;
    c.branch(C::Ne, 0x791c8, false)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x3)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(8), 0)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(8), 36)?)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(13), r(11), 0x1c)?)?;
    c.emit(transfer_multiple(C::Al, true, r(13), 0x4ff0, BlockMode::IncrementAfter, true)?)?;
    c.branch(C::Al, 0x292c, false)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(8), 12)?)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(8), 13)?)?;
    c.emit(data_register(C::Al, Op::And, r(0), r(1), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(8), 14)?)?;
    c.emit(data_register(C::Al, Op::And, r(0), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_byte(C::Al, false, r(0), r(11), -56)?)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(11), -56)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x1)?)?;
    c.branch(C::Ne, 0x791c8, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(8), 20)?)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(1), r(11), 0x38)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(11), -48)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(transfer_word(C::Al, false, r(5), r(11), -52)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(11), -44)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x10)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(11), -56)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(8), 32)?)?;
    c.branch(C::Al, 0x2a878, true)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x2)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(8), 0)?)?;
    c.branch(C::Al, 0x791c0, false)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(8), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(13), r(11), 0x1c)?)?;
    c.emit(transfer_multiple(C::Al, true, r(13), 0x4ff0, BlockMode::IncrementAfter, true)?)?;
    c.branch(C::Al, 0x34fac, false)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(8), 990)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x1)?)?;
    c.branch(C::Ne, 0x79044, false)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(8), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x761bc, true)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(8), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x74be4, true)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(8), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x79684, true)?;
    c.literal(C::Al, r(0), 0x791ec)?;
    c.emit(transfer_word(C::Al, true, r(1), r(8), 20)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(0), 24)?)?;
    c.branch(C::Al, 0x252a8, true)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(8), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x39bd0, true)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(8), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x5ab4c, true)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(8), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x59604, true)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x3f8)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x1)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(0), r(0), 0x1400)?)?;
    c.emit(transfer_word_register(C::Al, false, r(1), r(8), r(0), Shift::Lsl, 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x3f)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(8), 296)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 24)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(0), 4)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x1000)?)?;
    c.emit(data_immediate(C::Eq, Op::Mov, r(0), r(0), 0x3b)?)?;
    c.emit(transfer_word(C::Eq, false, r(0), r(8), 296)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(8), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x8)?)?;
    c.branch(C::Ne, 0x790c0, false)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(8), 0)?)?;
    c.branch(C::Al, 0x791c0, false)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(11), -44)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(5), 0x2)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(11), -52)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(8), 20)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(11), -48)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x14)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(11), -56)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(8), 8)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(0), r(0), 0x1000000)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x1)?)?;
    c.branch(C::Eq, 0x79184, false)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x4)?)?;
    c.branch(C::Eq, 0x79180, false)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x2)?)?;
    c.branch(C::Ne, 0x7918c, false)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(0), 0x1)?)?;
    c.branch(C::Al, 0x79184, false)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(8), 990)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x1)?)?;
    c.branch(C::Ne, 0x7912c, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(8), 52)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(0), 0x10)?)?;
    c.branch(C::Al, 0x35df4, true)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Ne, 0x79118, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(8), 48)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(0), 0x1c)?)?;
    c.branch(C::Al, 0x35df4, true)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Ne, 0x7912c, false)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(8), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x79314, true)?;
    c.emit(transfer_byte(C::Al, true, r(3), r(8), 1172)?)?;
    c.emit(transfer_byte(C::Al, true, r(2), r(8), 3104)?)?;
    c.literal(C::Al, r(1), 0x79210)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.literal(C::Al, r(1), 0x791f4)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(8), 3104)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Eq, 0x791c8, false)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(8), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x5373c, true)?;
    c.branch(C::Al, 0x791c8, false)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(0), 0x3)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(11), -40)?)?;
    c.branch(C::Al, 0x791a0, false)?;
    c.literal(C::Al, r(2), 0x791e8)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x7d)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x900)?)?;
    c.branch(C::Al, 0x17b0, true)?;
    c.emit(transfer_word(C::Al, true, r(0), r(8), 32)?)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(1), r(11), 0x38)?)?;
    c.branch(C::Al, 0x2a878, true)?;
    c.emit(transfer_word(C::Al, true, r(0), r(8), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x2)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x9)?)?;
    c.emit(data_immediate(C::Eq, Op::Mov, r(1), r(0), 0x5)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(8), 0)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(8), 36)?)?;
    c.branch(C::Al, 0x292c, true)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(13), r(11), 0x1c)?)?;
    c.emit(transfer_multiple(C::Al, true, r(13), 0x8ff0, BlockMode::IncrementAfter, true)?)?;
    c.literal(C::Al, r(2), 0x791e8)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x95)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x700)?)?;
    c.branch(C::Al, 0x17b0, true)?;
    c.branch(C::Al, 0x79148, false)?;
    c.data(0x791e8, 0xf3ce9)?;
    c.data(0x791ec, 0x10ceac)?;
    c.data(0x791f0, 0xf417b)?;
    c.data(0x791f4, 0xe2788)?;
    c.data(0x791f8, 0xf4145)?;
    c.data(0x791fc, 0xec034)?;
    c.data(0x79200, 0xf41c3)?;
    c.data(0x79204, 0xf40f9)?;
    c.data(0x79208, 0xf412c)?;
    c.data(0x7920c, 0xf3f59)?;
    c.data(0x79210, 0xf3e08)?;
    c.check(END)?;
    if c.bytes.len() != CODE_BYTES + DATA_BYTES {
        return Err("invalid worker fragment length".into());
    }
    Ok(c.bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn word(bytes: &[u8], at: usize) -> u32 {
        u32::from_le_bytes(bytes[at - START..at - START + 4].try_into().unwrap())
    }

    #[test]
    fn complete_code_and_data_layout() {
        let bytes = fragment().unwrap();
        assert_eq!(bytes.len(), 11268);
        assert_eq!(bytes.len(), END - START);
        assert_eq!(CODE_BYTES + DATA_BYTES, bytes.len());
        let mut data_bytes = 0;
        let mut code_bytes = 0;
        for at in (START..END).step_by(4) {
            if let Some(value) = data_value(at) {
                assert_eq!(word(&bytes, at), value, "data {at:#x}");
                data_bytes += 4;
            } else {
                assert!(code_address(at));
                code_bytes += 4;
            }
        }
        assert_eq!(data_bytes, DATA_BYTES);
        assert_eq!(code_bytes, CODE_BYTES);
        assert_eq!(word(&bytes, 0x77bfc), 0x77881);
        for at in (0x791d0..0x791e8).step_by(4) {
            assert!(code_address(at));
        }
        assert_eq!(word(&bytes, 0x791cc), transfer_multiple(C::Al, true,
            R::SP, 0x8ff0, BlockMode::IncrementAfter, true).unwrap());
        assert_eq!(word(&bytes, 0x791d0), transfer_word(C::Al, true,
            r(2), R::PC, 0x10).unwrap());
        assert_eq!(word(&bytes, 0x791e4), branch(C::Al,
            0x791e4, 0x79148, false).unwrap());
    }

    #[test]
    fn control_flow_and_literal_targets() {
        let bytes = fragment().unwrap();
        let mut thumb_calls = 0;
        for at in (START..END).step_by(4).filter(|at| code_address(*at)) {
            let instruction = word(&bytes, at);
            if instruction & 0xfe00_0000 == 0xfa00_0000 {
                let target = match at {
                    0x77248 | 0x7725c | 0x7792c | 0x779e4 => 0x83c4c,
                    0x78bec => 0x82be4,
                    _ => panic!("unexpected BLX at {at:#x}"),
                };
                assert_eq!(instruction, branch_exchange_immediate(at as u32, target).unwrap());
                thumb_calls += 1;
            } else {
                assert_ne!(instruction >> 28, 15);
                if instruction & 0x0e00_0000 == 0x0a00_0000 {
                    let delta = ((instruction << 8) as i32 >> 6) as isize;
                    let target = (at as isize + 8 + delta) as usize;
                    assert_eq!(target % 4, 0);
                    assert!(data_value(target).is_none());
                    if instruction & 0x0100_0000 == 0 {
                        assert!(code_address(target) || matches!(target, 0x292c | 0x34fac));
                    }
                }
            }
            if instruction & 0x0f7f_0000 == 0x051f_0000 {
                let offset = (instruction & 0xfff) as usize;
                let target = if instruction & 0x0080_0000 != 0 {
                    at + 8 + offset
                } else {
                    at + 8 - offset
                };
                assert!(data_value(target).is_some());
            }
        }
        assert_eq!(thumb_calls, 5);
        for (start, targets) in [
            (0x766e4, &[0x7674c, 0x78f8c, 0x766fc, 0x766fc, 0x766fc, 0x78fc8][..]),
            (0x767b4, &[0x767c8, 0x76804, 0x76864, 0x76920, 0x7910c][..]),
            (0x77be0, &[0x77c08, 0x77cf4, 0x77d98, 0x78034, 0x78170, 0x77c08, 0x77c08][..]),
        ] {
            for (index, &target) in targets.iter().enumerate() {
                assert_eq!(word(&bytes, start + index * 4), target as u32);
                assert!(code_address(target));
            }
        }
    }

    #[test]
    fn exact_guards_are_transactional() {
        let nop = data_register(C::Al, Op::Mov, r(0), r(0), r(0),
            Shift::Lsl, 0, false).unwrap();
        for at in (START..END).step_by(4) {
            let mut c = TypedA32 { at, bytes: Vec::new() };
            assert_eq!(c.emit(nop).is_ok(), code_address(at));
            let mut c = TypedA32 { at, bytes: Vec::new() };
            assert!(c.data(at, data_value(at).unwrap_or(0).wrapping_add(1)).is_err());
            assert_eq!(c.at, at);
            assert!(c.bytes.is_empty());
            if let Some(value) = data_value(at) {
                c.data(at, value).unwrap();
                assert_eq!(c.bytes, value.to_le_bytes());
            }
        }
        for at in [START - 4, START + 1, END - 1, END, END + 4] {
            let mut c = TypedA32 { at, bytes: Vec::new() };
            assert!(c.emit(nop).is_err());
            assert!(c.data(at, 0).is_err());
            assert!(c.bytes.is_empty());
            assert_eq!(c.at, at);
        }
        let mut c = TypedA32 { at: START, bytes: Vec::new() };
        assert!(c.check(END).is_err());
        assert!(c.data(0x766e4, 0x7674c).is_err());
        for target in [START, START + 1, 0x766e5, END, usize::MAX] {
            assert!(c.literal(C::Al, r(0), target).is_err());
            assert!(c.branch(C::Al, target, false).is_err());
            assert!(c.thumb_call(target).is_err());
        }
        assert!(c.bytes.is_empty());
        c.at = 0x77248;
        assert!(c.thumb_call(0x83c4d).is_err());
        assert!(c.thumb_call(0x82be4).is_err());
        assert!(c.bytes.is_empty());
        c.thumb_call(0x83c4c).unwrap();
    }

    #[test]
    fn authenticated_reference_when_available() {
        let Some(path) = std::env::var_os("NABU_VENUS_REFERENCE")
            .or_else(|| std::env::var_os("VENUS_REFERENCE")) else { return };
        let reference = std::fs::read(path).unwrap();
        assert_eq!(reference.len(), 1159200);
        assert_eq!(crate::wmfw_sha256::hex_digest(&reference),
            "9d4af65d7ede845e900f1b29ff425b7a8e2947056e695e246e58a2091445a085");
        assert_eq!(fragment().unwrap(),
            reference[START + FILE_MAPPING_DELTA..END + FILE_MAPPING_DELTA]);
    }
}

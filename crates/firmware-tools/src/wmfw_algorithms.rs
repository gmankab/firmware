// SPDX-License-Identifier: GPL-3.0-only
//! Independently authored Cirrus WMFW v3 algorithm metadata (not HALO code).
//!
//! Linux `wmfw.h` defines these algorithm/control schemas. Version 3 uses
//! length-prefixed, 4-byte-padded strings instead of v1's 256-byte arrays.
//! The high-level tables below model *names, offsets, flags and lengths*,
//! never copies of the proprietary DSP machine-code or coefficient values.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Control {
    pub offset: u16,
    pub memory_type: u16,
    pub name: String,
    pub full_name: String,
    pub description: String,
    pub control_type: u16,
    pub flags: u16,
    pub size_bytes: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Algorithm {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub controls: Vec<Control>,
}

#[derive(Clone, Copy)]
struct ControlSpec {
    offset: u16,
    short: &'static str,
    suffix: &'static str,
    mem_type: u16,
    flags: u16,
    length: u32,
}

const fn c(offset: u16, short: &'static str, flags: u16) -> ControlSpec {
    ControlSpec { offset, short, suffix: short, mem_type: 5, flags, length: 4 }
}
const fn size(offset: u16, short: &'static str, flags: u16, length: u32) -> ControlSpec {
    ControlSpec { offset, short, suffix: short, mem_type: 5, flags, length }
}
const fn alias(offset: u16, short: &'static str, suffix: &'static str,
               flags: u16, length: u32) -> ControlSpec {
    ControlSpec { offset, short, suffix, mem_type: 5, flags, length }
}

// Firmware framework controls from the public Halo/CSPL control interface.
const FRAMEWORK: &[ControlSpec] = &[
    c(0x00, "HALO_STATE", 5),
    c(0x01, "HALO_HEARTBEAT", 5),
    c(0x02, "AUDIO_BLK_SIZE", 5),
    size(0x03, "BUILD_JOB_NAME", 3, 12),
    c(0x06, "BUILD_JOB_NUMBER", 3),
    c(0x07, "GLOBAL_SAMPLE_RATE_HW", 5),
    c(0x08, "MULTIRATE_NUMBER_BANDS", 5),
    c(0x09, "FIRMWARE_STATE", 3),
    c(0x0a, "EVENT_TIMEOUT", 3),
    c(0x0b, "DC_OFFSET_HOLD_TIME", 3),
    c(0x0c, "ENABLE_FULL_US_BYPASS", 3),
    c(0x0d, "MAX_LRCLK_DELAY", 3),
];

// CSPL controls are exposed to the codec firmware host interface. The short
// strings are the firmware's stable control identifiers; suffix overrides
// carry descriptive fully-qualified control identifiers.
const CSPL: &[ControlSpec] = &[
    c(0x00, "CSPL_ENABLE", 3),
    c(0x01, "CSPL_COMMAND", 3),
    c(0x02, "CSPL_STATE", 5),
    c(0x03, "CSPL_ERRORNO", 5),
    c(0x04, "CSPL_TEMPERATURE", 5),
    alias(0x05, "UPDT_PRMS", "HAS_UPDATE_PARAMS", 5, 4),
    c(0x06, "CAL_R", 0),
    c(0x07, "CAL_AMBIENT", 3),
    c(0x08, "CAL_STATUS", 0),
    c(0x09, "CAL_CHECKSUM", 0),
    c(0x0a, "CAL_R_SELECTED", 7),
    c(0x0b, "CAL_SET_STATUS", 7),
    c(0x0c, "DIAG_F0", 7),
    c(0x0d, "DIAG_Z_LOW_DIFF", 7),
    c(0x0e, "DIAG_F0_STATUS", 7),
    alias(0x0f, "RTLOG_ENABLE", "LOGGER_XM_STRUCT_ENABLE", 3, 4),
    alias(0x10, "RTLOG_STATE", "LOGGER_XM_STRUCT_STATE", 7, 4),
    alias(0x11, "RTLOG_COUNT", "LOGGER_XM_STRUCT_COUNT", 3, 4),
    alias(0x14, "RTLOG_FRMPRWIN", "LOGGER_XM_STRUCT_FRAMESPERCAPTUREWINDOW", 3, 4),
    alias(0x15, "RTLOG_VARIABLE", "LOGGER_XM_STRUCT_VARIABLE", 3, 60),
    alias(0x24, "RTLOG_DATA", "LOGGER_XM_STRUCT_DATA", 5, 120),
    alias(0x42, "BDLOG_MAX_TEMP", "THRESHOLDLOGGER_XM_STRUCT_MAX_TEMP", 7, 4),
    alias(0x43, "BDLOG_MAX_EXC", "THRESHOLDLOGGER_XM_STRUCT_MAX_EXC", 7, 4),
    alias(0x44, "BDLOG_OVER_TEMP_COUNT", "THRESHOLDLOGGER_XM_STRUCT_OVER_TEMP_COUNT", 7, 4),
    alias(0x45, "BDLOG_OVER_EXC_COUNT", "THRESHOLDLOGGER_XM_STRUCT_OVER_EXC_COUNT", 7, 4),
    alias(0x46, "BDLOG_ABNORMAL_MUTE", "THRESHOLDLOGGER_XM_STRUCT_ABNORMAL_MUTE", 7, 4),
    alias(0x47, "BDLOG_VMON_LOW_FLAG", "THRESHOLDLOGGER_XM_STRUCT_VMON_LOW_FLAG", 7, 4),
    c(0x48, "REDUCE_POWER", 3),
    alias(0x49, "CH_BAL", "CHANNEL_BALANCE", 3, 4),
    c(0x4a, "ATTENUATION", 3),
    c(0x4b, "SPK_OUTPUT_POWER", 5),
    c(0x4c, "ALGO_FRAME_DELAY", 7),
    ControlSpec {offset:0, short:"CSPL_UPDATE_PARAMS_CONFIG", suffix:"CSPL_UPDATE_PARAMS_CONFIG",
                 mem_type:6, flags:3, length:400},
];

const EVENT_LOGGER: &[ControlSpec] = &[
    alias(0x00, "EVNTLG_ENABLED", "ENABLED", 7, 4),
    c(0x01, "WAS_DISABLED", 5),
    c(0x02, "BUFFER", 5),
    c(0x03, "BUFFER_SIZE", 5),
    c(0x04, "TRANSFER_COMPLETED", 7),
    c(0x05, "NUM_LOST_EVENTS", 5),
    c(0x06, "LOST_EVENT_COUNTER", 5),
    c(0x07, "TRIGGER_INTERRUPT", 5),
    c(0x08, "BUFFER_COUNT", 5),
    c(0x09, "FREE_SLOTS", 5),
    c(0x0a, "W_PTR", 5),
    size(0x0b, "EVENTS", 5, 256),
];

fn authored(id: u32, name: &str, prefix: &str, specs: &[ControlSpec]) -> Algorithm {
    Algorithm {
        id, name: name.to_owned(), description: String::new(),
        controls: specs.iter().map(|ctl| Control {
            offset: ctl.offset, memory_type: ctl.mem_type,
            name: ctl.short.to_owned(),
            full_name: format!("{prefix}{}", ctl.suffix),
            description: String::new(),
            control_type: 4, flags: ctl.flags, size_bytes: ctl.length,
        }).collect(),
    }
}

pub fn framework() -> Algorithm {
    authored(0x0004_00a4, "FIRMWARE_HALO_CSPL", "FIRMWARE_HALO_CSPL_", FRAMEWORK)
}

/// Core speaker-protection algorithm and controls (CSPL).
pub fn cspl() -> Algorithm {
    authored(0x0000_00cd, "CSPL", "CSPL_", CSPL)
}

/// Firmware event-logging algorithm controls, separate from CSPL.
pub fn event_logger() -> Algorithm {
    authored(0x0000_f20b, "EVENT_LOGGER", "EVENT_LOGGER_", EVENT_LOGGER)
}

fn append_padded_string(output: &mut Vec<u8>, s: &str, width: usize) -> Result<(), String> {
    let length = s.len();
    let max = if width == 1 { u8::MAX as usize } else { u16::MAX as usize };
    if length > max { return Err(format!("WMFW v3 string length {length} exceeds {max}")); }
    if !s.is_ascii() { return Err("WMFW v3 authored names must be ASCII".into()); }
    match width {
        1 => output.push(length as u8),
        2 => output.extend_from_slice(&(length as u16).to_le_bytes()),
        _ => return Err("WMFW counted-string width must be 1 or 2".into()),
    }
    output.extend_from_slice(s.as_bytes());
    let consumed = length + width;
    let padding = (4 - consumed % 4) % 4;
    output.extend(std::iter::repeat_n(0, padding));
    Ok(())
}

/// Emit one algorithm region's payload, with structurally checked v3
/// lengths. In particular, `size` is the encoded coefficient's metadata size,
/// not the number of algorithm-data bytes in the control register.
pub fn encode_algorithm(alg: &Algorithm) -> Result<Vec<u8>, String> {
    let count = u32::try_from(alg.controls.len()).map_err(|_| "too many WMFW controls")?;
    let mut data = Vec::new();
    data.extend_from_slice(&alg.id.to_le_bytes());
    append_padded_string(&mut data, &alg.name, 1)?;
    append_padded_string(&mut data, &alg.description, 2)?;
    data.extend_from_slice(&count.to_le_bytes());
    for ctl in &alg.controls {
        let mut body = Vec::new();
        append_padded_string(&mut body, &ctl.name, 1)?;
        append_padded_string(&mut body, &ctl.full_name, 1)?;
        append_padded_string(&mut body, &ctl.description, 2)?;
        body.extend_from_slice(&ctl.control_type.to_le_bytes());
        body.extend_from_slice(&ctl.flags.to_le_bytes());
        body.extend_from_slice(&ctl.size_bytes.to_le_bytes());
        let block_size = u32::try_from(body.len()).map_err(|_| "WMFW control metadata size exceeds u32")?;
        data.extend_from_slice(&ctl.offset.to_le_bytes());
        data.extend_from_slice(&ctl.memory_type.to_le_bytes());
        data.extend_from_slice(&block_size.to_le_bytes());
        data.extend_from_slice(&body);
    }
    Ok(data)
}

fn take<'a>(buf: &'a [u8], pos: &mut usize, n: usize) -> Result<&'a [u8], String> {
    let end = pos.checked_add(n).ok_or("WMFW field offset overflow")?;
    let result = buf.get(*pos..end).ok_or("WMFW truncated metadata field")?;
    *pos = end;
    Ok(result)
}
fn read16(buf: &[u8], pos: &mut usize) -> Result<u16, String> {
    Ok(u16::from_le_bytes(take(buf,pos,2)?.try_into().unwrap()))
}
fn read32(buf: &[u8], pos: &mut usize) -> Result<u32, String> {
    Ok(u32::from_le_bytes(take(buf,pos,4)?.try_into().unwrap()))
}
fn read_string(buf: &[u8], pos: &mut usize, width: usize) -> Result<String, String> {
    let field_start = *pos;
    let len = match width {
        1 => take(buf,pos,1)?[0] as usize,
        2 => read16(buf,pos)? as usize,
        _ => return Err("unsupported counted-string width".into()),
    };
    let text = take(buf,pos,len)?;
    if !text.is_ascii() {return Err("non-ASCII WMFW v3 string".into());}
    let total = width + len;
    let pad = (4 - total % 4) % 4;
    if take(buf,pos,pad)?.iter().any(|byte| *byte != 0) {
        return Err(format!("nonzero WMFW string padding at offset 0x{field_start:x}"));
    }
    String::from_utf8(text.to_vec()).map_err(|_| "invalid WMFW UTF-8".into())
}

/// Parse a complete WMFW v3 algorithm region. This is a checked diagnostic
/// parser, **not** proof that an opaque region was independently rebuilt.
pub fn parse_algorithm(bytes: &[u8]) -> Result<Algorithm, String> {
    let mut pos = 0;
    let id = read32(bytes,&mut pos)?;
    let name = read_string(bytes,&mut pos,1)?;
    let description = read_string(bytes,&mut pos,2)?;
    let ncontrols = read32(bytes,&mut pos)? as usize;
    if ncontrols > bytes.len() / 16 {
        return Err("unreasonable WMFW v3 coefficient count".into());
    }
    let mut controls = Vec::with_capacity(ncontrols);
    for _ in 0..ncontrols {
        let offset = read16(bytes,&mut pos)?;
        let memory_type = read16(bytes,&mut pos)?;
        let size = read32(bytes,&mut pos)? as usize;
        let end = pos.checked_add(size).ok_or("WMFW v3 control overflow")?;
        let field = bytes.get(pos..end).ok_or("WMFW control metadata truncated")?;
        let mut inner = 0;
        let name = read_string(field,&mut inner,1)?;
        let full_name = read_string(field,&mut inner,1)?;
        let description = read_string(field,&mut inner,2)?;
        let control_type = read16(field,&mut inner)?;
        let flags = read16(field,&mut inner)?;
        let size_bytes = read32(field,&mut inner)?;
        if inner != size { return Err("WMFW control metadata block size incorrect".into()); }
        controls.push(Control {offset,memory_type,name,full_name,description,control_type,flags,size_bytes});
        pos = end;
    }
    if pos != bytes.len() {
        return Err(format!("WMFW region has {} unexplained trailing bytes",bytes.len()-pos));
    }
    Ok(Algorithm{id,name,description,controls})
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn framework_metadata_bytes_and_controls() {
        let alg = framework();
        let built = encode_algorithm(&alg).unwrap();
        assert_eq!(built.len(), 948);
        assert_eq!(parse_algorithm(&built).unwrap(), alg);
        assert_eq!(alg.controls.len(), 12);
        assert_eq!(alg.controls[3].size_bytes, 12);
        assert!(built.windows(10).any(|b|b==b"HALO_STATE"));
    }
    #[test]
    fn cspl_and_event_logger_have_exact_wmfw_v3_layout() {
        let speaker = cspl();
        assert_eq!(speaker.controls.len(), 33);
        assert_eq!(speaker.controls.last().unwrap().memory_type, 6);
        assert_eq!(speaker.controls.last().unwrap().size_bytes, 400);
        let speaker_bytes = encode_algorithm(&speaker).unwrap();
        assert_eq!(speaker_bytes.len(), 2_100);
        assert_eq!(parse_algorithm(&speaker_bytes).unwrap(), speaker);
        let events = event_logger();
        assert_eq!(events.controls.len(), 12);
        assert_eq!(events.controls.last().unwrap().size_bytes, 256);
        let event_bytes = encode_algorithm(&events).unwrap();
        assert_eq!(event_bytes.len(), 760);
        assert_eq!(parse_algorithm(&event_bytes).unwrap(), events);
    }
    #[test]
    fn v3_parser_rejects_nonzero_padding_and_truncation() {
        let mut built = encode_algorithm(&framework()).unwrap();
        for n in 0..built.len() {
            let shortened = &built[..n];
            assert!(parse_algorithm(shortened).is_err());
        }
        // Algorithm name occupies a 20-byte field, ends at offset 24.
        built[23] = 0x01;
        assert!(parse_algorithm(&built).unwrap_err().contains("padding"));
    }
}

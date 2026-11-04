
#[cfg(test)] mod tests {
    #[test]
    fn controller_frame_restore_and_static_string_tables_are_exact(){
        let bytes=fragment().unwrap();
        let at=|vma:usize| {
            let off=vma-START;
            u32::from_le_bytes(bytes[off..off+4].try_into().unwrap())
        };
        assert_eq!(at(0xc6438),0x000f_b3df);
        for (idx,val) in [0x000f_dc9f,0x000f_dc98,0x000f_dc9c,0x000f_e0bf].iter().enumerate(){
            assert_eq!(at(0xc659c+4*idx),*val);
        }
        assert_eq!(END,0xc65ac);
        assert_eq!(bytes.len(),END-START);
    }

    #[test]
    fn controller_error_routing_literal_tables_are_classified_as_data(){
        let bytes=fragment().unwrap();
        let at=|vma:usize| {
            let i=vma-START;
            u32::from_le_bytes(bytes[i..i+4].try_into().unwrap())
        };
        assert_eq!(at(0xc6150),0x000f_b7da);
        assert_eq!(at(0xc61bc),0x000f_b9b0);
        assert_eq!(at(0xc624c),0x000f_b7f8);
        assert_eq!(at(0xc62c4),0x000f_b3df);
        assert_eq!(at(0xc6330),0x0010_408c);
        assert!(END>=0xc6334);
        assert_eq!(bytes.len(),END-START);
    }

    #[test]
    fn live_frame_registration_keeps_literal_pools_separate(){
        let bytes=fragment().unwrap();
        let word=|addr:usize| {
            let i=addr-START;
            u32::from_le_bytes(bytes[i..i+4].try_into().unwrap())
        };
        assert_eq!(word(0xc5f34),0x000f_b6b3);
        assert_eq!(word(0xc5fa4),0x000f_b6cf);
        assert_eq!(word(0xc5fa8),0x000f_b958);
        assert!(END>=0xc5fac);
        assert_eq!(bytes.len(),END-START);
    }

    #[test]
    fn controller_record_suffix_has_three_distinguished_data_pools(){
        let bytes=fragment().unwrap();
        let word=|addr:usize| {
            let off=addr-START;
            u32::from_le_bytes(bytes[off..off+4].try_into().unwrap())
        };
        for (idx,val) in [0x000f_b3df,0x000f_ad07,0x000e_2788,0x1000_0801].iter().enumerate(){
            assert_eq!(word(0xc5aec+4*idx),*val);
        }
        for (idx,val) in [0x000f_b65b,0x000f_b641,0x000f_b679].iter().enumerate(){
            assert_eq!(word(0xc5bcc+4*idx),*val);
        }
        for (idx,val) in [0x000f_ad7c,0x000f_b7bb].iter().enumerate(){
            assert_eq!(word(0xc5cd0+4*idx),*val);
        }
        assert!(END>=0xc5cd8);
        assert_eq!(bytes.len(),END-START);
    }

    use super::*;
    #[test]
    fn codec_descriptor_continuation_stops_before_unclassified_literals() {
        let b=fragment().unwrap();
        assert!(END>=0xc5aec);
        assert_eq!(b.len(),END-START);
        // The following address is known to start an inline data pool.
        assert!(END<MAX_OWNED_END);
    }

    #[test]
    fn next_codec_descriptor_function_starts_with_original_prologue(){
        let b=fragment().unwrap();
        let off=0xc4b28-START;
        let word=u32::from_le_bytes(b[off..off+4].try_into().unwrap());
        let save=R::new(4).unwrap().bit()|R::new(5).unwrap().bit()|
            R::new(6).unwrap().bit()|R::new(7).unwrap().bit()|
            R::new(8).unwrap().bit()|R::new(9).unwrap().bit()|
            R::new(10).unwrap().bit()|R::R11.bit()|R::LR.bit();
        assert_eq!(word,arm32::push_registers(C::Al,save).unwrap());
        assert!(b.len()>=0xc5000-START);
    }

    #[test]
    fn frame_plane_span_diagnostic_words_are_not_code(){
        let b=fragment().unwrap();
        let at=|a:usize| {let i=a-START;u32::from_le_bytes(b[i..i+4].try_into().unwrap())};
        assert_eq!(at(0xc4b1c),0x000f_b38f);
        assert_eq!(at(0xc4b20),0x000f_b3bf);
        assert_eq!(at(0xc4b24),0x000e_2788);
        assert!(b.len()>=0xc4b28-START);
    }

    #[test]
    fn frame_scaling_literals_are_six_noninstructions() {
        let b=fragment().unwrap();
        let read=|a:usize| {let i=a-START;u32::from_le_bytes(b[i..i+4].try_into().unwrap())};
        for (i,w) in [0x000f_b34e,0x000e_2788,0x000f_b369,
                       0x000f_b318,0x000f_b334,0x000f_b382].iter().enumerate(){
            assert_eq!(read(0xc47b0+4*i),*w);
        }
        assert!(b.len()>=0xc47c8-START);
    }

    #[test]
    fn packed_plane_field_decoder_stops_at_new_prologue() {
        let b=fragment().unwrap();
        let w=|a:usize| { let i=a-START;u32::from_le_bytes(b[i..i+4].try_into().unwrap()) };
        assert_eq!(w(0xc4334),arm32::transfer_doubleword(C::Al,true,R::R2,R::R1,0).unwrap());
        assert_eq!(w(0xc4500),arm32::rotate_right_with_extend(C::Al,R::R2,R::R2,false).unwrap());
        assert_eq!(w(0xc4690),arm32::bx(C::Al,R::LR));
        assert!(b.len()>=0xc4694-START);
    }

    #[test]
    fn codec_policy_literal_table_is_classified_as_data(){
        let b=fragment().unwrap();let read=|addr:usize|{let i=addr-START;u32::from_le_bytes(b[i..i+4].try_into().unwrap())};
        for (i,w) in [0x10000401,0x000fac88,0x000e2788,0x000faca2,0x000fb50e,0x000face1,0x0007ffff,0x000fb528,0x000fbbf8,0x000fb3df,0x000fb604,0x000fb555,0x000fb56c,0x000fac1b,0x000fb5d0,0x000fb59a,0x000fb2d0,0x000fb304,0x0010408c].iter().enumerate(){
            assert_eq!(read(0xc4198+4*i),*w);
        }
        assert!(b.len()>=0xc420c-START);
    }

    #[test]
    fn controller_stream_output_diagnostics_are_data() {
        let b=fragment().unwrap();
        let word=|a:usize| {
            let i=a-START;
            u32::from_le_bytes(b[i..i+4].try_into().unwrap())
        };
        for (i,w) in [0x000f_b239,0x000e_2788,0x000f_b262,0x000f_b28d].iter().enumerate(){
            assert_eq!(word(0xc3bdc+4*i),*w);
        }
        assert_eq!(word(0xc3c10),arm32::bx(C::Al,R::LR));
        assert!(b.len()>=0xc3c14-START);
    }

    #[test]
    fn controller_output_paths_end_at_next_prologue() {
        let b=fragment().unwrap();
        let read=|addr:usize| {
            let i=addr-START;
            u32::from_le_bytes(b[i..i+4].try_into().unwrap())
        };
        assert_eq!(read(0xc32cc),arm32::transfer_word(C::Al,true,R::R2,R::R0,4).unwrap());
        assert_eq!(read(0xc3570),arm32::bx(C::Al,R::LR));
        assert!(b.len()>=0xc3574-START);
    }

    #[test]
    fn codec_frame_resource_literals_are_identified_data() {
        let b=fragment().unwrap();
        let at=|addr:usize|{
            let off=addr-START;
            u32::from_le_bytes(b[off..off+4].try_into().unwrap())
        };
        assert_eq!(at(0xc32bc),0xe000_1810);
        assert_eq!(at(0xc32c0),0x7fff_8000);
        assert_eq!(at(0xc32c4),0x000f_b226);
        assert_eq!(at(0xc32c8),0x000e_2788);
        assert!(b.len()>=0xc32cc-START);
    }

    #[test]
    fn stream_descriptor_table_data_ends_before_next_function(){
        let b=fragment().unwrap();let get=|a:usize|{let i=a-START;u32::from_le_bytes(b[i..i+4].try_into().unwrap())};
        assert_eq!(0xc2874-0xc2860,5*4);
        assert_eq!(get(0xc2860),0x000faf54);
        assert_eq!(get(0xc2870),0x7fff8000);
    }

    #[test]
    fn controller_cleanup_and_plane_literals_are_noncode_words(){
        let b=fragment().unwrap();let w=|a:usize|{let i=a-START;u32::from_le_bytes(b[i..i+4].try_into().unwrap())};
        assert_eq!(w(0xc1d04),0x000faf54);
        assert_eq!(w(0xc1d60),0x000fb01c);
        assert_eq!(w(0xc1d64),0x000e2788);
        assert_eq!(w(0xc1f60),0x000fb05d);
        assert_eq!(w(0xc1f8c),0x000fb2c4);
    }

    #[test]
    fn output_controller_register_pointer_pool_is_noncode_data(){
        let bytes=fragment().unwrap();let w=|a:usize|{let i=a-START;u32::from_le_bytes(bytes[i..i+4].try_into().unwrap())};
        assert_eq!(w(0xc1b60),0x000faf54);
        assert_eq!(w(0xc1b74),0x0010408c);
        assert_eq!(0xc1b78-0xc1b60,24);
    }

    #[test]
    fn controller_object_registry_words_are_data(){
        let b=fragment().unwrap();let word=|a:usize|{let i=a-START;u32::from_le_bytes(b[i..i+4].try_into().unwrap())};
        assert_eq!(0xc18e4-0xc18c0,9*4);
        assert_eq!(word(0xc18c0),0x000fa71c);
        assert_eq!(word(0xc18e0),0x000f4f2d);
    }

    #[test]
    fn hardware_buffer_resource_cleanup_literals_match_address_map(){
        let bytes=fragment().unwrap();
        let word=|a:usize|{let i=a-START;u32::from_le_bytes(bytes[i..i+4].try_into().unwrap())};
        assert_eq!(0xc1488-0xc1474,5*4);
        assert_eq!(word(0xc1474),0x000fa71c);
        assert_eq!(word(0xc1484),0x000fabdc);
    }

    #[test]
    fn frame_profile_literal_pool_is_explicit_noncode_data(){
        let bytes=fragment().unwrap();
        let word=|at:usize|{let i=at-START;u32::from_le_bytes(bytes[i..i+4].try_into().unwrap())};
        assert_eq!(0xc11bc-0xc1164,22*4);
        assert_eq!(word(0xc1164),0x000fa71c);
        assert_eq!(word(0xc11b8),0x000f5954);
    }

    #[test]
    fn device_controller_window_reports_are_classified_literals(){
        let bytes=fragment().unwrap();
        let word=|a:usize|{let i=a-START;u32::from_le_bytes(bytes[i..i+4].try_into().unwrap())};
        assert_eq!(word(0xc05e0),0x000fa71c);
        assert_eq!(word(0xc0978),0x000fa71c);
        assert_eq!(word(0xc099c),0x000faeae);
        assert!(END>=0xc5aec);
        assert_eq!(0xc5aec-0xc5000,2796);
        assert_eq!(0xc5000-0xc4b28,1240);
        assert_eq!(0xc4b28-0xc47c8,864);
        assert_eq!(0xc47c8-0xc4694,308);
        assert_eq!(0xc4694-0xc420c,1160);
        assert_eq!(0xc420c-0xc3c14,1528);
        assert_eq!(0xc3c14-0xc3574,1696);
        assert_eq!(0xc3574-0xc32cc,680);
        assert_eq!(0xc32cc-0xc2874,2648);
        assert_eq!(0xc2874-0xc1f90,2276);
        assert_eq!(0xc1f90-0xc1b78,1048);
        assert_eq!(0xc1b78-0xc18e4,660);
        assert_eq!(0xc18e4-0xc1514,976);
        assert_eq!(0xc1514-0xc11bc,856);
        assert_eq!(0xc11bc-0xc09a0,2076);
    }

    #[test]
    fn profile_allocator_literal_pool_contains_real_data(){
        let bytes=fragment().unwrap();
        let word=|a:usize|{let i=a-START;u32::from_le_bytes(bytes[i..i+4].try_into().unwrap())};
        assert_eq!(0xc049c-0xc0470,11*4);
        assert_eq!(word(0xc0470),0x02000102);
        assert_eq!(word(0xc0498),0x000f558e);
    }

    /// Opt-in artifact verification avoids unnecessary recompilation of the
    /// large CLI binary during rapid function-level checkpoint development.
    /// Neither original firmware nor an emitted firmware fragment is checked
    /// into Git; paths come exclusively from the caller's environment.
    #[test]
    fn opt_in_private_reference_compare_and_emit(){
        let output=fragment().unwrap();
        if let Some(path)=std::env::var_os("AGENT20_06_EMIT_BIN"){
            std::fs::write(path,&output).unwrap();
        }
        if let Some(path)=std::env::var_os("NABU_VENUS_REFERENCE").or_else(|| std::env::var_os("AGENT20_06_REFERENCE_VENUS")){
            let original=std::fs::read(path).unwrap();
            let expected=original.get(FILE_OFFSET..FILE_OFFSET+output.len())
                .expect("original reference does not contain the owned interval");
            if expected!=output{
                let i=expected.iter().zip(&output).position(|(a,b)|a!=b).unwrap();
                panic!("agent06 mismatch at runtime 0x{:x}: expected {:02x}, got {:02x}",
                    START+i,expected[i],output[i]);
            }
        }
    }

    #[test]
    fn codec_output_final_literal_pool_has_23_data_words(){
        let bytes=fragment().unwrap();
        let word=|at:usize|{
            let i=at-START;u32::from_le_bytes(bytes[i..i+4].try_into().unwrap())
        };
        assert_eq!(0xbfe54-0xbfdf8,23*4);
        assert_eq!(word(0xbfdf8),0x000fa754);
        assert_eq!(word(0xbfe50),0x000fa6f2);
        assert!(END>=0xc5aec);
        assert_eq!(0xc5aec-0xc5000,2796);
        assert_eq!(0xc5000-0xc4b28,1240);
        assert_eq!(0xc4b28-0xc47c8,864);
        assert_eq!(0xc47c8-0xc4694,308);
        assert_eq!(0xc4694-0xc420c,1160);
        assert_eq!(0xc420c-0xc3c14,1528);
        assert_eq!(0xc3c14-0xc3574,1696);
        assert_eq!(0xc3574-0xc32cc,680);
        assert_eq!(0xc32cc-0xc2874,2648);
        assert_eq!(0xc2874-0xc1f90,2276);
        assert_eq!(0xc1f90-0xc1b78,1048);
        assert_eq!(0xc1b78-0xc18e4,660);
        assert_eq!(0xc18e4-0xc1514,976);
        assert_eq!(0xc1514-0xc11bc,856);
        assert_eq!(0xc11bc-0xc09a0,2076);
        assert_eq!(0xc09a0-0xc049c,1284);
        assert_eq!(0xc049c-0xbfe54,1608);
    }

    #[test]
    fn queued_codec_status_literal_words_remain_nonexecutable_data(){
        let bytes=fragment().unwrap();
        let get=|at:usize|{let i=at-START;u32::from_le_bytes(bytes[i..i+4].try_into().unwrap())};
        assert_eq!(get(0xbeb70),0x000fa40a);
        assert_eq!(get(0xbeb74),0x000fa438);
        assert_eq!(get(0xbec30),0x000fa520);
        assert_eq!(get(0xbec70),0x000fa2af);
        assert_eq!(get(0xbed60),0x0010bb79);
        assert_eq!(get(0xbee94),0x000fa6a8);
        assert_eq!(get(0xbee98),0x000e2788);
        assert_eq!(get(0xbee9c),0x000fa6c9);
    }

    #[test] fn validates_exclusive_agent06_span() {
        assert_eq!(START,0xa8000);assert!(END>=0xbe7dc);
        assert_eq!(FILE_OFFSET,0xac000);
        assert_eq!(fragment().unwrap().len(),END-START);
    }
    #[test]
    fn frame_source_diagnostics_preserve_original_noncode_data(){
        let b=fragment().unwrap();
        let at=|a:usize|{
            let i=a-START;
            u32::from_le_bytes(b[i..i+4].try_into().unwrap())
        };
        assert_eq!(at(0xb9eb4),0x000f_9ad3);
        assert_eq!(at(0xb9f78),0x000f_9b0b);
        assert_eq!(at(0xb9f7c),0x000e_2788);
        for (i,v) in [0x000f_9b32,0x000e_2788,0x000f_9b5e,0x000f_9b9a,
                      0x000f_9ad3,0x000f_9bab,0x000f_9bd8].iter().enumerate(){
            assert_eq!(at(0xba1f4+i*4),*v);
        }
        assert_eq!(0xba210-0xb9c64,1452);
    }
    #[test]
    fn controller_release_status_message_words_are_literal_data(){
        let b=fragment().unwrap();
        for (start,expected) in [
            (0xb9b6c,&[0x000f_9a31,0x000f_9a5e,0x000e_2788][..]),
            (0xb9c54,&[0x000f_9a7b,0x000f_9ab4,0x000e_2788,0x000f_9a31][..]),
        ] {
            for (i,word) in expected.iter().enumerate(){
                let pos=start+4*i-START;
                assert_eq!(u32::from_le_bytes(b[pos..pos+4].try_into().unwrap()),*word);
            }
        }
        assert_eq!(0xb9c64-0xb9904,864);
    }
    #[test]
    fn frame_status_literal_pools_do_not_overlap_executable_instructions(){
        let b=fragment().unwrap();
        let word=|a:usize|{
            let pos=a-START;
            u32::from_le_bytes(b[pos..pos+4].try_into().unwrap())
        };
        assert_eq!(word(0xb9800),0x0022_1008);
        for (i,v) in [0x000f_984c,0x000e_2788].iter().enumerate(){
            assert_eq!(word(0xb9830+4*i),*v);
        }
        for (i,v) in [0x7fe0_0002,0x7f10_0002,0x0101_0200].iter().enumerate(){
            assert_eq!(word(0xb9860+4*i),*v);
        }
        for (i,v) in [0x000f_98c3,0x000f_98e0,0x000e_2788,0x000f_9892,
                       0x000f_98c3,0x000f_98e0,0x000f_98f7,0x000f_690d,
                       0x000f_6864,0x000f_9928,0x000f_9987,0x000f_99bf,
                       0x000f_99f9].iter().enumerate(){
            assert_eq!(word(0xb98d0+4*i),*v);
        }
        assert_eq!(0xb9904-0xb9208,1788);
    }
    #[test]
    fn frame_input_layout_table_matches_original_handler_addresses(){
        let bytes=fragment().unwrap();
        let values=[0xb9208,0xb9210,0xb9218,0xb976c];
        for (i,expected) in values.iter().enumerate(){
            let at=0xb91f8+4*i-START;
            assert_eq!(u32::from_le_bytes(bytes[at..at+4].try_into().unwrap()),*expected);
        }
    }
    #[test]
    fn frame_descriptor_tables_preserve_four_arm_targets_each(){
        let bytes=fragment().unwrap();
        for (start,words) in [(0xb88b8,[0xb88c8,0xb8910,0xb88e4,0xb88ec]),
                              (0xb8ab0,[0xb8ac0,0xb8ac8,0xb8ad0,0xb8ad8])] {
            for (i,target) in words.iter().enumerate(){
                let at=start+4*i-START;
                assert_eq!(u32::from_le_bytes(bytes[at..at+4].try_into().unwrap()),*target);
            }
        }
        assert_eq!(0xb8c00-0xb87d8,1064);
    }
    #[test]
    fn release_codec_context_has_five_handlers_and_four_messages(){
        let output=fragment().unwrap();
        for (i,word) in FIVE_WAY_CLEANUP_DISPATCH.iter().enumerate(){
            let off=0xb6f48 + 4*i - START;
            assert_eq!(u32::from_le_bytes(output[off..off+4].try_into().unwrap()),*word);
        }
        for (i,word) in CONTROLLER_RELEASE_MESSAGES.iter().enumerate(){
            let off=0xb71a8 + 4*i - START;
            assert_eq!(u32::from_le_bytes(output[off..off+4].try_into().unwrap()),*word);
        }
        assert_eq!(0xb71b8-0xb6edc,732);
    }
    #[test]
    fn slot_cleanup_diagnostic_literals_are_non_instruction_words(){
        let bytes=fragment().unwrap();
        for (i,expected) in CODEC_CONTEXT_CLOSE_DIAGNOSTICS.iter().enumerate(){
            let pos=0xb76d4+i*4-START;
            assert_eq!(u32::from_le_bytes(bytes[pos..pos+4].try_into().unwrap()),*expected);
        }
        assert_eq!(0xb76f4-0xb71b8,1340);
    }
    #[test]
    fn codec_stream_lifecycle_records_have_five_log_words_and_three_ids(){
        let bytes=fragment().unwrap();
        let at=|address:usize|{
            let i=address-START;
            u32::from_le_bytes(bytes[i..i+4].try_into().unwrap())
        };
        for (i,v) in RESTART_CODEC_LOGS.iter().enumerate(){assert_eq!(at(0xb7938+4*i),*v);}
        assert_eq!(at(0xb79e4),CODEC_EVENT_READY);
        assert_eq!(at(0xb7b0c),CODEC_EVENT_STREAM_ON);
        assert_eq!(at(0xb7b94),CODEC_EVENT_STREAM_OFF);
    }
    #[test]
    fn codec_crc32_and_message_records_are_data_separated(){
        let bytes=fragment().unwrap();
        let at=|adr:usize|{
            let i=adr-START;
            u32::from_le_bytes(bytes[i..i+4].try_into().unwrap())
        };
        for (i,v) in BUFFER_CRC_TABLE_AND_POLYNOMIAL.iter().enumerate(){assert_eq!(at(0xb7ca8+i*4),*v);}
        for (i,v) in ACTIVE_BUFFER_TRACE.iter().enumerate(){assert_eq!(at(0xb7d90+i*4),*v);}
        assert_eq!(at(0xb7f84),0x000f_95ba);
        assert_eq!(at(0xb7f90),0x000f_9638);
        for (i,v) in CODEC_WORKER_TRACE.iter().enumerate(){assert_eq!(at(0xb80c8+i*4),*v);}
        assert_eq!(0xb80d8-0xb7bd8,1280);
    }
    #[test]
    fn codec_plane_size_dispatch_and_diagnostics_are_literal_data(){
        let bytes=fragment().unwrap();
        let word=|adr:usize|{
            let index=adr-START;
            u32::from_le_bytes(bytes[index..index+4].try_into().unwrap())
        };
        for (i,v) in CODEC_PLANE_SIZE_DISPATCH.iter().enumerate(){assert_eq!(word(0xb82cc+i*4),*v);}
        for (i,v) in CODEC_PLANE_CONFIGURATION_LOGS.iter().enumerate(){assert_eq!(word(0xb84dc+i*4),*v);}
        assert_eq!(0xb84fc-0xb80d8,1060);
    }
    #[test]
    fn codec_input_buffer_and_channel_warnings_are_literal_data(){
        let bytes=fragment().unwrap();
        let at=|addr:usize|{
            let i=addr-START;
            u32::from_le_bytes(bytes[i..i+4].try_into().unwrap())
        };
        for (i,val) in CODEC_INPUT_RECORD_WARNINGS.iter().enumerate(){assert_eq!(at(0xb86b0+4*i),*val);}
        for (i,val) in CODEC_CHANNEL_SETUP_WARNINGS.iter().enumerate(){assert_eq!(at(0xb87a4+4*i),*val);}
        assert_eq!(0xb87d8-0xb84fc,732);
    }
    #[test] fn codec_dispatch_jump_table_classified_as_data() {
        let bytes=fragment().unwrap();
        let word=|address:usize|{
            let index=address-START;
            u32::from_le_bytes(bytes[index..index+4].try_into().unwrap())
        };
        let mut destinations=std::collections::BTreeMap::new();
        for i in 0..41 {
            let target=word(0xae984+4*i);
            assert!(matches!(target,0xaea28|0xaeb74|0xaeb98|0xaebac|0xaebe0|0xaec28|0xaec3c));
            *destinations.entry(target).or_insert(0usize)+=1;
        }
        assert_eq!(destinations.get(&0xaea28),Some(&16usize));
        assert_eq!(destinations.get(&0xaec3c),Some(&19usize));
        assert_eq!(destinations.get(&0xaeb74),Some(&2usize));
        assert_eq!(destinations.values().copied().sum::<usize>(),41);
        assert_eq!(word(0xaf24c),0x000f_6cb9);
        assert_eq!(word(0xaf28c),0x000f_870c);
    }

    #[test] fn controller_mode_dispatch_and_diagnostics_are_data(){
        let bytes=fragment().unwrap();
        for (i,value) in [0x000f8fbc,0x000f94c5,0x000e2788,0xfffffe01].iter().enumerate() {
            let addr=0xb6ab4+4*i-START;
            assert_eq!(u32::from_le_bytes(bytes[addr..addr+4].try_into().unwrap()),*value);
        }
        for (i,value) in [0x000f91e5,0x000f9231,0x000e2788,0x000f927a,0x000f8fbc,0x000f9250].iter().enumerate() {
            let addr=0xb6c34+4*i-START;
            assert_eq!(u32::from_le_bytes(bytes[addr..addr+4].try_into().unwrap()),*value);
        }
        for (i,value) in [0x000f92c6,0x000e2788,0x000f8fbc].iter().enumerate() {
            let addr=0xb6d28+4*i-START;
            assert_eq!(u32::from_le_bytes(bytes[addr..addr+4].try_into().unwrap()),*value);
        }
        for (i,value) in [0x000b6e44,0x000b6e9c,0x000b6e50,0x000b6ea8,0x000b6e5c,0x000b6eb4].iter().enumerate() {
            let addr=0xb6e2c+4*i-START;
            assert_eq!(u32::from_le_bytes(bytes[addr..addr+4].try_into().unwrap()),*value);
        }
        for (i,value) in [0x000f8fbc,0x000f92e6,0x000e2788,0x000f9315,0x000f9340].iter().enumerate() {
            let addr=0xb6ec8+4*i-START;
            assert_eq!(u32::from_le_bytes(bytes[addr..addr+4].try_into().unwrap()),*value);
        }
    }

    #[test] fn registered_controller_literal_pool_is_data(){
        let bytes=fragment().unwrap();
        let expected=[0x02fd0300,0x03badb00,0xfc452500,
            0x000f912d,0x000f9112,0x000e2788,
            0x000f8fbc,0x000f90c0,0x000f9190];
        for (i,word) in expected.iter().enumerate(){
            let j=0xb6898+4*i-START;
            assert_eq!(u32::from_le_bytes(bytes[j..j+4].try_into().unwrap()),*word);
        }
    }

    #[test] fn codec_descriptor_global_words_are_data(){
        let bytes=fragment().unwrap();
        let expected=[0x0010b6dc,0x0010b6ec,0x000b60ec,0x000f904f,
            0x000f8f74,0x000e2788,0x000f9085,0x000f9020,
            0x000f8fbc,0x000f8ff5,0x000f8f88,0x000f8f34];
        for (i,word) in expected.iter().enumerate(){
            let j=0xb65b4+4*i-START;
            assert_eq!(u32::from_le_bytes(bytes[j..j+4].try_into().unwrap()),*word);
        }
    }

    #[test] fn plane_status_constant_pool_has_exact_values(){
        let bytes=fragment().unwrap();
        for (i,word) in [0x000f8f04,0x000e2788,0x03badb00].iter().enumerate() {
            let j=0xb6258+4*i-START;
            assert_eq!(u32::from_le_bytes(bytes[j..j+4].try_into().unwrap()),*word);
        }
    }

    #[test] fn stream_class_controller_data_is_not_decoded_as_code() {
        let bytes=fragment().unwrap();
        let expected=[0x000f79ef,0x000e2788,0x000f8e3e,0x000f7a25,
            0x000f8e6c,0x000f8e29,0x000f8d97,0x0010dd88,
            0x0010dd98,0x0010dd94,0x000f7058,0x000f8eed,
            0x000f8ea2,0x000f8ecb,0x000f7a4c,0x000f7a7c,
            0x000f7095,0x000f8dfc];
        for (i,word) in expected.iter().enumerate(){
            let address=0xb60a4+4*i-START;
            assert_eq!(u32::from_le_bytes(bytes[address..address+4].try_into().unwrap()),*word);
        }
        assert_eq!(0xb60ec-0xa8000,57580);
    }

    #[test] fn checkpoint64_frame_queue_carries_two_pointer_words() {
        let bytes=fragment().unwrap();
        assert!(END>=0xc5aec);
        assert_eq!(0xc5aec-0xc5000,2796);
        assert_eq!(0xc5000-0xc4b28,1240);
        assert_eq!(0xc4b28-0xc47c8,864);
        assert_eq!(0xc47c8-0xc4694,308);
        assert_eq!(0xc4694-0xc420c,1160);
        assert_eq!(0xc420c-0xc3c14,1528);
        assert_eq!(0xc3c14-0xc3574,1696);
        assert_eq!(0xc3574-0xc32cc,680);
        assert_eq!(0xc32cc-0xc2874,2648);
        assert_eq!(0xc2874-0xc1f90,2276);
        assert_eq!(0xc1f90-0xc1b78,1048);
        assert_eq!(0xc1b78-0xc18e4,660);
        assert_eq!(0xc18e4-0xc1514,976);
        assert_eq!(0xc1514-0xc11bc,856);
        assert_eq!(0xc11bc-0xc09a0,2076);
        assert_eq!(0xc09a0-0xc049c,1284);
        assert_eq!(0xc049c-0xbfe54,1608);
        assert_eq!(0xbfe54-0xbf400,2644);
        assert_eq!(0xbf400-0xbeea0,1376);
        assert_eq!(0xbeb70-0xbe7dc,916);
        assert_eq!(0xbeea0-0xbeb70,816);
        for (at,val) in [(0xbe9e8,STREAM_PENDING_RECORD_MESSAGE),
                         (0xbe9f8,STREAM_ALTERNATE_RECORD_MESSAGE)] {
            let off=at-START;
            assert_eq!(u32::from_le_bytes(bytes[off..off+4].try_into().unwrap()),val);
        }
    }

    #[test] fn checkpoint63_four_way_plane_policy_is_data() {
        let output=fragment().unwrap();
        assert!(END>=0xbe7dc);
        assert_eq!(0xbe7dc-0xbe000,2012);
        for (i,value) in CODEC_PLANE_POLICY_TARGETS.iter().enumerate(){
            let offset=0xbe44c+4*i-START;
            assert_eq!(u32::from_le_bytes(output[offset..offset+4].try_into().unwrap()),*value);
        }
    }

    #[test] fn checkpoint62_eight_way_codec_mode_switch_is_data() {
        let output=fragment().unwrap();
        assert_eq!(0xbe000-0xbdc00,1024);
        assert!(END>=0xbe000);
        for (index,target) in CONTROLLER_FRAME_MODE_BRANCHES.iter().enumerate(){
            let pos=0xbdc84+index*4-START;
            assert_eq!(u32::from_le_bytes(output[pos..pos+4].try_into().unwrap()),*target);
        }
    }

    #[test] fn checkpoint61_buffer_reconciliation_has_seven_handler_addresses() {
        let out=fragment().unwrap();
        assert_eq!(out.len(),END-START);
        assert_eq!(0xbdc00-0xbd648,1464);
        assert!(END>=0xbdc00);
        for (index,value) in CODEC_RECOVERY_DISPATCH_TARGETS.iter().enumerate(){
            let position=0xbd850+4*index-START;
            assert_eq!(u32::from_le_bytes(out[position..position+4].try_into().unwrap()),*value);
        }
    }

    #[test] fn checkpoint60_controller_init_eight_literals_are_data() {
        let bytes=fragment().unwrap();
        assert_eq!(0xbd648-0xbd000,1608);
        assert!(END>=0xbd648);
        for (i,word) in CONTROLLER_RECORD_INIT_LITERALS.iter().enumerate(){
            let offset=0xbd540+4*i-START;
            assert_eq!(u32::from_le_bytes(bytes[offset..offset+4].try_into().unwrap()),*word);
        }
    }

    #[test] fn checkpoint59_profile_literals_are_data() {
        let bytes=fragment().unwrap();
        assert_eq!(0xbd000-0xbc7dc,2084);
        assert!(END>=0xbd000);
        for (index,expected) in HARDWARE_CODEC_PROFILE_LITERALS.iter().enumerate(){
            let off=0xbcfd8+index*4-START;
            assert_eq!(u32::from_le_bytes(bytes[off..off+4].try_into().unwrap()),*expected);
        }
    }

    #[test] fn checkpoint58_controller_return_and_literals_are_explicit() {
        let bytes=fragment().unwrap();
        assert_eq!(0xbc7dc-0xbc598,580);
        assert!(END>=0xbc7dc);
        for (i,w) in STREAM_CONTROLLER_COMMAND_LITERALS.iter().enumerate(){
            let at=0xbc78c+i*4-START;
            assert_eq!(u32::from_le_bytes(bytes[at..at+4].try_into().unwrap()),*w);
        }
    }

    #[test] fn checkpoint57_controller_command_registers_are_contiguous() {
        let output=fragment().unwrap();
        assert_eq!(output.len(),END-START);
        assert_eq!(0xbc598-0xbbcd4,2244);
        assert!(END>=0xbc598);
        // A32 instructions only; the PC-relative data pool occurs after END.
        assert_eq!(u32::from_le_bytes(output[0xbbcd4-START..0xbbcd8-START].try_into().unwrap()),
            arm32::push_registers(C::Al,R::new(4).unwrap().bit()|R::new(5).unwrap().bit()|
                R::new(6).unwrap().bit()|R::new(7).unwrap().bit()|
                R::new(8).unwrap().bit()|R::new(9).unwrap().bit()|
                R::new(10).unwrap().bit()|R::R11.bit()|R::LR.bit()).unwrap());
    }

    #[test] fn checkpoint56_codec_register_writer_has_no_embedded_data_pools() {
        let full=fragment().unwrap();
        assert_eq!(full.len(),END-START);
        assert_eq!(0xbbcd4-0xbbabc,536);
        assert!(END>=0xbbcd4);
    }

    #[test] fn checkpoint55_register_programmer_has_classified_literals() {
        let bytes=fragment().unwrap();
        for (idx,w) in CONTROLLER_PACKET_REG_DIAGNOSTICS.iter().enumerate(){
            let offset=0xbbaac+idx*4-START;
            assert_eq!(u32::from_le_bytes(bytes[offset..offset+4].try_into().unwrap()),*w);
        }
        assert_eq!(0xbbabc-0xbb6d8,996);
    }

    #[test] fn checkpoint54_session_retire_and_warning_pools_are_explicit_data() {
        let bytes=fragment().unwrap();
        for (idx,w) in CODEC_SESSION_RETIRE_DIAGNOSTICS.iter().enumerate(){
            let pos=0xbb644+idx*4-START;
            assert_eq!(u32::from_le_bytes(bytes[pos..pos+4].try_into().unwrap()),*w);
        }
        assert_eq!(u32::from_le_bytes(bytes[0xbb6d4-START..0xbb6d8-START].try_into().unwrap()),EXTERNAL_CODEC_WARNING);
        assert_eq!(0xbb6d8-0xbb570,360);
    }

    #[test] fn checkpoint53_record_format_is_a_literal() {
        let bytes=fragment().unwrap();
        assert_eq!(u32::from_le_bytes(bytes[0xbb56c-START..0xbb570-START].try_into().unwrap()), PACKED_CODEC_RECORD_FORMAT);
        assert_eq!(0xbb570-0xbb278,760);
    }

    #[test] fn checkpoint52_frame_controller_release_is_data() {
        let bytes=fragment().unwrap();
        for (idx,w) in FRAME_CONTROLLER_RELEASE_LITERALS.iter().enumerate(){
            let pos=0xbb254+idx*4-START;
            assert_eq!(u32::from_le_bytes(bytes[pos..pos+4].try_into().unwrap()),*w);
        }
        assert_eq!(0xbb278-0xbb078,512);
    }

    #[test] fn checkpoint51_dma_window_literal_table_is_structured() {
        let bytes=fragment().unwrap();
        for (idx, word) in DMA_WINDOW_CONTROLLER_METADATA.iter().enumerate() {
            let offset=0xbb008+idx*4-START;
            assert_eq!(u32::from_le_bytes(bytes[offset..offset+4].try_into().unwrap()),*word);
        }
        assert_eq!(0xbb078-0xba40c,3180);
    }

    #[test] fn checkpoint50_controller_cleanup_literals_are_data() {
        let all=fragment().unwrap();
        assert_eq!(all.len(),END-START);
        let read=|a:usize|u32::from_le_bytes(all[a-START..a-START+4].try_into().unwrap());
        for (i,&w) in STREAM_END_DIAGNOSTICS.iter().enumerate() {
            assert_eq!(read(0xba294+i*4),w);
        }
        for (i,&w) in CODEC_RESET_GLOBALS.iter().enumerate(){
            assert_eq!(read(0xba3fc+i*4),w);
        }
        assert_eq!(0xba40c-0xba210,508);
    }

    #[test] fn stream_class_jump_table_has_original_targets() {
        let bytes=fragment().unwrap();
        let observed:Vec<u32>=(0..14).map(|i| {
            let offset=0xb5054+4*i-START;
            u32::from_le_bytes(bytes[offset..offset+4].try_into().unwrap())
        }).collect();
        assert_eq!(observed,[0x000b50a0,0x000b50a0,0x000b50d0,0x000b50d0,
            0x000b50d0,0x000b50d0,0x000b50d0,0x000b50d0,
            0x000b508c,0x000b508c,0x000b508c,0x000b50ac,0x000b50ac,0x000b50c4]);
    }

    #[test] fn stream_feedback_literal_pools_are_separate_data() {
        let bytes=fragment().unwrap();
        let read=|addr:usize| {
            let pos=addr-START;
            u32::from_le_bytes(bytes[pos..pos+4].try_into().unwrap())
        };
        for (idx,val) in CODEC_RECORD_SHIFT_DIAGNOSTICS.iter().enumerate(){
            assert_eq!(read(0xb48ac+idx*4),*val);
        }
        for (idx,val) in STREAM_FLUSH_DIAGNOSTICS.iter().enumerate(){
            assert_eq!(read(0xb4e0c+idx*4),*val);
        }
        for (idx,val) in STREAM_FEEDBACK_DIAGNOSTICS.iter().enumerate(){
            assert_eq!(read(0xb4ff8+idx*4),*val);
        }
        assert_eq!(0xb5008-START,0xd008);
    }

    #[test] fn channel_state_warnings_are_named_data() {
        let bytes=fragment().unwrap();
        let at=|addr:usize| {
            let off=addr-START;
            u32::from_le_bytes(bytes[off..off+4].try_into().unwrap())
        };
        assert_eq!(at(0xb41c0),RESET_RECORD_WARNING);
        for (idx,val) in CHANNEL_STATE_REPORTS.iter().enumerate(){
            assert_eq!(at(0xb46c0+idx*4),*val);
        }
        assert_eq!(0xb46cc-START,0xc6cc);
    }

    #[test] fn live_codec_globals_are_three_data_words() {
        let bytes=fragment().unwrap();
        for (idx,expected) in LIVE_CODEC_BUFFER_GLOBALS.iter().enumerate(){
            let offset=0xb40a0+4*idx-START;
            assert_eq!(u32::from_le_bytes(bytes[offset..offset+4].try_into().unwrap()),*expected);
        }
        assert_eq!(0xb40ac-START,0xc0ac);
    }

    #[test] fn queued_codec_frame_literals_are_data() {
        let bytes=fragment().unwrap();
        for (idx,expected) in CODEC_FRAME_QUEUE_DIAGNOSTICS.iter().enumerate(){
            let offset=0xb3bc8+4*idx-START;
            assert_eq!(u32::from_le_bytes(bytes[offset..offset+4].try_into().unwrap()),*expected);
        }
        assert_eq!(0xb3bdc-START,0xb3bdc-0xa8000);
    }

    #[test] fn output_response_literal_pool_is_data() {
        let bytes=fragment().unwrap();
        for (idx,expected) in CODEC_OUTPUT_DIAGNOSTICS.iter().enumerate(){
            let offset=0xb2d30+4*idx-START;
            assert_eq!(u32::from_le_bytes(bytes[offset..offset+4].try_into().unwrap()),*expected);
        }
        assert_eq!(0xb2f08-START,0xaf08);
    }

    #[test] fn literal_pools_are_not_treated_as_opcodes() {
        let bytes=fragment().unwrap();
        let word=|addr:usize|->u32 {let i=addr-START;u32::from_le_bytes(bytes[i..i+4].try_into().unwrap())};
        for (i,val) in CONTROLLER_STATUS_DIAGNOSTICS.iter().enumerate(){assert_eq!(word(0xa8114+4*i),*val);}
        assert_eq!(word(0xa835c),CODEC_STATUS_MESSAGE);
        assert_eq!(word(0xa838c),CONTROLLER_ERROR_CLASS);
        for (i,val) in CONTROLLER_MESSAGE_CONSTANTS.iter().enumerate(){assert_eq!(word(0xa8398+4*i),*val);}
        for (i,val) in RESOURCE_INIT_LOGGING.iter().enumerate(){assert_eq!(word(0xa8bc4+4*i),*val);}
        for (i,val) in RESOURCE_INITIALIZER_CALLBACKS.iter().enumerate(){assert_eq!(word(0xa8c24+4*i),*val);}
        for (i,val) in RESOURCE_ERROR_MESSAGES.iter().enumerate(){assert_eq!(word(0xa8c90+4*i),*val);}
        for (i,val) in MISSING_CODEC_SLOT_MESSAGES.iter().enumerate(){assert_eq!(word(0xa8e90+4*i),*val);}
        for address in [0xa9034,0xa90d8,0xa911c] {assert_eq!(word(address),CODEC_SLOT_CHECK_MESSAGE);}
        assert_eq!(word(0xa9514),SLOT_MANAGER_GLOBAL);
        for (i,&val) in CODEC_BLEND_LITERALS.iter().enumerate(){
            assert_eq!(word(0xaa1f4+4*i),val);
        }
        for (i,&val) in CODEC_OUTPUT_GLOBALS.iter().enumerate(){
            assert_eq!(word(0xaa6e8+4*i),val);
        }
        for (i,&val) in CODEC_CHANNEL_LITERALS.iter().enumerate(){
            assert_eq!(word(0xab030+4*i),val);
        }
        for (i,&val) in CODEC_FIFO_LITERALS.iter().enumerate(){
            assert_eq!(word(0xab768+4*i),val);
        }
        for (i,&val) in SLOT_REMAP_GLOBALS.iter().enumerate(){
            assert_eq!(word(0xab8e4+4*i),val);
        }
        for (i,&val) in CODEC_RELAY_DIAGNOSTICS.iter().enumerate(){
            assert_eq!(word(0xabbe4+4*i),val);
        }
        for (i,&val) in CODEC_REGISTER_PACKET_LITERALS.iter().enumerate(){
            assert_eq!(word(0xac72c+4*i),val);
        }
        for (i,&val) in [0x000f_8519,0x000f_8615,0x000e_2788].iter().enumerate(){
            assert_eq!(word(0xac9e4+4*i),val);
        }
        for (i,&val) in [0x7fff_ff00,0xffff_f000,0xaaaa_aaab,0x5555_5556,
                         0x002b_2002,0x000f_8650,0x000e_2788].iter().enumerate(){
            assert_eq!(word(0xad6f0+4*i),val);
        }
        assert_eq!(word(0xad724),0x0010_dd70);
        assert_eq!(word(0xadc10),0x000f_8726);
        assert_eq!(word(0xadf10),0x000f_6c34);
        assert_eq!(word(0xadf14),0x000e_2788);
        assert_eq!(word(0xae018),0x000f_6c57);
        assert_eq!(word(0xae01c),0x000e_2788);
        assert_eq!(word(0xae58c),0x000f_8519);
        assert_eq!(word(0xae744),0x0010_dd90);
        for (index, expected) in [0x000f_7058,0x000f_88b7,0x000e_2788,
                                  0xffff_fef1,0x000f_8a41,0x000f_89de,
                                  0x000f_88c7,0x000f_8a0a,0x000f_8a77,
                                  0x000f_7095].iter().enumerate(){
            assert_eq!(word(0xb0288+4*index), *expected);
        }
        assert_eq!(word(0xb0444),0x000b_0448); // Following helper address
        assert_eq!(word(0xb06d8),0x000f_88c7); // Diagnostic pointer
        for (index,value) in [0x000f_88c7,0x000f_7058,0x000f_8902,
                             0x000e_2788,0x000f_7095].iter().enumerate(){
            assert_eq!(word(0xb0c50+4*index),*value);
        }
        for (index,value) in [0x000f_8a8e,0x000e_2788,0x000f_8ae6].iter().enumerate(){
            assert_eq!(word(0xb0d00+4*index),*value);
        }
        for (index,value) in [0x000f_7058,0x000f_8915,0x000e_2788,0x000f_7095].iter().enumerate(){
            assert_eq!(word(0xb0f40+4*index),*value);
        }
        assert_eq!(word(0xb11ac),0x000f_7197);
        assert_eq!(word(0xb11b0),0x000e_2788);
        for (idx,w) in [0x000f_8928,0x000e_2788,0x000f_88c7,
                        0x05f5_e100,0x0010_408c,0x000f_8965,
                        0x000f_897e,0x000f_7058,0x000f_7095].iter().enumerate(){
            assert_eq!(word(0xb1a08+4*idx),*w);
        }
        assert_eq!(word(0xb1b3c),0x000f_70f5);
        assert_eq!(word(0xb1b40),0x000e_2788);
        for (idx,w) in [0x000f_8b1b,0x000f_8b5c,0x000e_2788,
                        0x000f_88c7,0x05f5_e100,0x0010_408c,
                        0x000f_8965,0x000f_7058,0x0010_dd88,
                        0x000f_8b6d,0x000f_8b99,0x000b_0448,
                        0x000f_8991,0x000f_7095,0x000f_7125].iter().enumerate(){
            assert_eq!(word(0xb2a44+4*idx),*w);
        }
    }
}

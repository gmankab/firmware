// SPDX-License-Identifier: GPL-3.0-only
use firmware_tools::devices::xiaomi_nabu::{cdsp_early_tlb as tlb,cdsp_early_cfgbase as cfgbase,cdsp_early_system_start as system_start,cdsp_early_interrupt_debug as interrupt_debug,cdsp_early_supervisor_vectors as vectors,cdsp_early_exception_entry as exception_entry,cdsp_early_exception_context as context,cdsp_early_exception_setup as setup,cdsp_early_exception_transition as transition,cdsp_early_exception_wait as wait,cdsp_early_exception_finalize as finalize,cdsp_early_context_snapshot as snapshot,cdsp_early_l2_sync as l2_sync,cdsp_early_phys_dump as dump,cdsp_early_runtime_maintenance as maintenance,cdsp_early_memory_dispatch as dispatcher,cdsp_early_worker_bootstrap as worker,cdsp_early_worker_runtime as runtime,cdsp_early_worker_state as worker_state,cdsp_early_worker_trap as worker_trap,cdsp_early_worker_idle as worker_idle,cdsp_early_worker_thread as thread,cdsp_early_worker_guard as guard};
use std::{env,error::Error,fs};
fn main()->Result<(),Box<dyn Error>>{
    let args:Vec<_>=env::args().skip(1).collect();
    if args.len()!=2{return Err("usage: agent20-09-cdsp emit FILE | verify REFERENCE".into());}
    match args[0].as_str(){
        "emit"=>fs::write(&args[1],cfgbase::build_joined()?)?,
        "verify"=>guard::verify_reference(&fs::read(&args[1])?)?,
        "emit-worker-guard"=>fs::write(&args[1],guard::build_worker_guard()?)?,
        "emit-worker-thread"=>fs::write(&args[1],thread::build_worker_thread_start()?)?,
        "emit-worker-idle"=>fs::write(&args[1],worker_idle::build_worker_idle()?)?,
        "emit-worker-trap"=>fs::write(&args[1],worker_trap::build_worker_fatal_path()?)?,
        "emit-worker-state"=>fs::write(&args[1],worker_state::build_worker_state_prefix()?)?,
        "emit-worker-runtime"=>fs::write(&args[1],runtime::build_worker_runtime_prefix()?)?,
        "emit-worker-bootstrap"=>fs::write(&args[1],worker::build_worker_bootstrap()?)?,
        "emit-memory-dispatch"=>fs::write(&args[1],dispatcher::build_full_memory_dispatch()?)?,
        "emit-maintenance"=>fs::write(&args[1],maintenance::build_maintenance_helpers()?)?,
        "emit-phys-dump"=>fs::write(&args[1],dump::build_physical_dump()?)?,
        "emit-cache-publish"=>fs::write(&args[1],dump::build_cache_publish()?)?,
        "emit-l2-sync"=>fs::write(&args[1],l2_sync::build_l2_sync()?)?,
        "emit-exception-entry"=>fs::write(&args[1],exception_entry::build_exception_entry()?)?,
        "emit-context"=>fs::write(&args[1],context::build_context_save()?)?,
        "emit-controller-setup"=>fs::write(&args[1],setup::build_controller_setup()?)?,
        "emit-supervisor"=>fs::write(&args[1],worker::build_supervisor_with_worker()?)?,
        "emit-snapshot"=>fs::write(&args[1],snapshot::build_context_snapshot()?)?,
        "emit-finalize"=>fs::write(&args[1],finalize::build_exception_finalize()?)?,
        "emit-wait"=>fs::write(&args[1],wait::build_exception_wait()?)?,
        "emit-transition"=>fs::write(&args[1],transition::build_exception_transition()?)?,
        "emit-vectors or emit-exception-entry"=>fs::write(&args[1],vectors::build_supervisor_vectors()?)?,
        "emit-interrupt-debug"=>fs::write(&args[1],interrupt_debug::build_interrupt_debug()?)?,
        "emit-system-start"=>fs::write(&args[1],system_start::build_system_start()?)?,
        _=>return Err("unsupported action: use emit, verify, emit-interrupt-debug, emit-supervisor or emit-vectors".into()),
    }
    println!("CDSP typed functions: {:#x}..{:#x} and {:#x}..{:#x}, {}+{}={} generated executable bytes (240 separate linker-alignment bytes)",tlb::FILE_OFFSET,cfgbase::REGION_END,system_start::FILE_OFFSET,guard::REGION_END,cfgbase::REGION_END-tlb::FILE_OFFSET,guard::REGION_END-system_start::FILE_OFFSET,(cfgbase::REGION_END-tlb::FILE_OFFSET)+(guard::REGION_END-system_start::FILE_OFFSET));
    Ok(())
}

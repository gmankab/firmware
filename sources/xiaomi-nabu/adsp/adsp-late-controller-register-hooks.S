// ADSP controller platform-registration call sequence at file
// [0x3f468,0x3f4b0). Links subsystem callbacks, configures fixed command
// parameters and performs the shared tail-handoff after releasing its frame.
.text
.globl adsp_late_controller_register_hooks
adsp_late_controller_register_hooks:
{ call register_qurt_state; allocframe(#0) }
{ call register_irq_state; immext(#4028152256); r1:0 = combine(#5,##4028152304) }
{ call attach_platform_streams }
{ call set_service_callbacks }
{ call configure_registry_events }
{ call enable_service_fields }
{ call program_controller_mode }
{ call inspect_runtime_settings }
{ call initialize_platform_buffers }
{ call initialize_service_queues }
{ call register_controller_runtime }
{ call publish_platform_state }
{ r0 = #1; immext(#4294965056); jump runtime_complete; deallocframe }
.set register_qurt_state, adsp_late_controller_register_hooks+0x2e4
.set register_irq_state, adsp_late_controller_register_hooks+0x304
.set attach_platform_streams, adsp_late_controller_register_hooks-0x6020
.set set_service_callbacks, adsp_late_controller_register_hooks+0x3ed8
.set configure_registry_events, adsp_late_controller_register_hooks+0x48f8
.set enable_service_fields, adsp_late_controller_register_hooks+0x49a8
.set program_controller_mode, adsp_late_controller_register_hooks+0x1d8
.set inspect_runtime_settings, adsp_late_controller_register_hooks-0x3928
.set initialize_platform_buffers, adsp_late_controller_register_hooks+0x1c18
.set initialize_service_queues, adsp_late_controller_register_hooks+0x70a8
.set register_controller_runtime, adsp_late_controller_register_hooks-0xcc8
.set publish_platform_state, adsp_late_controller_register_hooks+0xede68
.set runtime_complete, adsp_late_controller_register_hooks-0x858

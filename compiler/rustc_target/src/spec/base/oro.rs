use crate::spec::{Cc, LinkerFlavor, Lld, RelocModel, StackProbeType, TargetOptions, LinkSelfContainedDefault, PanicStrategy, CodeModel};

pub(crate) fn opts() -> TargetOptions {
    TargetOptions {
        os: "oro".into(),
        linker: Some("rust-lld".into()),
        linker_flavor: LinkerFlavor::Gnu(Cc::No, Lld::Yes),
        relocation_model: RelocModel::Static,
        link_self_contained: LinkSelfContainedDefault::True,
        dynamic_linking: false,
        dll_tls_export: false,
        executables: true,
        exe_suffix: ".oro".into(),
        has_rpath: false,
        position_independent_executables: true,
        static_position_independent_executables: true,
        allow_asm: true,
        archive_format: "gnu".into(),
        main_needs_argc_argv: false,
        has_thread_local: false, // NOTE(qix-): Temporary
        panic_strategy: PanicStrategy::Abort, // NOTE(qix-): Temporary
        crt_static_allows_dylibs: false,
        crt_static_default: false,
        crt_static_respected: false,
        stack_probes: StackProbeType::Inline,
        trap_unreachable: true,
        requires_lto: true,
        no_builtins: false,
        supports_xray: false, // NOTE(qix-): Temporary
        code_model: Some(CodeModel::Medium),
        ..Default::default()
    }
}

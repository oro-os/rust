use crate::spec::{Cc, LinkerFlavor, Lld, RelocModel, StackProbeType, TargetOptions};

pub(crate) fn opts() -> TargetOptions {
    TargetOptions {
        os: "oro".into(),
        linker: Some("rust-lld".into()),
        linker_flavor: LinkerFlavor::Gnu(Cc::No, Lld::Yes),
        stack_probes: StackProbeType::Inline,
        relocation_model: RelocModel::Static,
        ..Default::default()
    }
}

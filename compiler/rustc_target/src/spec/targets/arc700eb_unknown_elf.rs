use rustc_abi::Endian;

use crate::spec::{Arch, PanicStrategy, RelocModel, Target, TargetMetadata, TargetOptions};

pub(crate) fn target() -> Target {
    Target {
        llvm_target: "arceb-unknown-unknown-elf".into(),
        metadata: TargetMetadata {
            description: Some("ARC700 big-endian bare-metal (ARCompact)".into()),
            tier: Some(3),
            host_tools: Some(false),
            std: Some(false),
        },
        pointer_width: 32,
        data_layout: "E-m:e-p:32:32-i1:8:32-i8:8:32-i16:16:32-i32:32:32-f32:32:32-i64:32-f64:32-a:0:32-n32".into(),
        arch: Arch::Arc,

        options: TargetOptions {
            c_int_width: 32,
            cpu: "arc700".into(),
            features: "+arcompact,+bigendian,+norm".into(),
            linker: Some("arceb-elf32-gcc".into()),
            max_atomic_width: Some(0),
            atomic_cas: false,
            singlethread: true,
            panic_strategy: PanicStrategy::Abort,
            relocation_model: RelocModel::Static,
            disable_redzone: true,
            endian: Endian::Big,
            eh_frame_header: false,
            executables: true,
            ..Default::default()
        },
    }
}

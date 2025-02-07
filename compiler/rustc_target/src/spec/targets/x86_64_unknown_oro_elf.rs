use crate::spec::{base, PanicStrategy, Target, TargetMetadata};

pub(crate) fn target() -> Target {
    let mut base = base::oro::opts();
    base.cpu = "x86-64".into();
    base.disable_redzone = true;
    base.panic_strategy = PanicStrategy::Abort;
    base.features = "-mmx,-sse,+soft-float".into();
   base.link_script = Some(LINK_SCRIPT.into());

    Target {
        llvm_target: "x86_64-unknown-none".into(),
        pointer_width: 64,
        data_layout:
            "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-i128:128-f80:128-n8:16:32:64-S128".into(),
        arch: "x86_64".into(),
        options: base,
        metadata: TargetMetadata {
            description: Some("64-bit Oro (ELF Module)".into()),
            tier: Some(3),
            host_tools: Some(false),
            std: Some(true),
        },
    }
}

static LINK_SCRIPT: &str = r#"
OUTPUT_FORMAT(elf64-x86-64)
OUTPUT_ARCH(i386:x86-64)

ENTRY(_start)

PHDRS {
	text     PT_LOAD    FLAGS((1 << 0) | (1 << 2));  /* rx */
	rodata   PT_LOAD    FLAGS((1 << 2)           );  /* r  */
	data     PT_LOAD    FLAGS((1 << 1) | (1 << 2));  /* rw */
    lrodata  PT_LOAD    FLAGS((1 << 2)           );  /* r  */
    ldata    PT_LOAD    FLAGS((1 << 1) | (1 << 2));  /* rw */
}

SECTIONS {
	. = 0x2000000;

	.text : {
		*(.text .text.*)
	} :text

	. = ALIGN(4096);

	.rodata : {
		KEEP(*(.oro .oro.*))
		*(.rodata .rodata.*)
		*(.got .got.*)
	} :rodata

	. = ALIGN(4096);

	.data : {
		*(.data .data.*)
	} :data

	. = ALIGN(4096);

	.bss : {
		*(COMMON)
		*(.bss .bss.*) /* MUST be last allocated to :data */
	} :data

    . = ALIGN(4096);

    /* https://web.archive.org/web/20250207153752/https://lld.llvm.org/ELF/large_sections.html */

    .lrodata : {
        *(.lrodata .lrodata.*)
    } :lrodata

    . = ALIGN(4096);

    .ldata : {
        *(.ldata .ldata.*)
    } :ldata

    . = ALIGN(4096);

    .lbss : {
        *(.lbss .lbss.*) /* MUST be last allocated to :ldata */
    } :ldata

	/DISCARD/ : {
		*(.eh_frame)
		*(.note .note.*)
	}
}
"#;

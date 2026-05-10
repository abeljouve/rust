use std::fmt;

use rustc_span::Symbol;

use super::{InlineAsmArch, InlineAsmType, ModifierInfo};

def_reg_class! {
    Arc ArcInlineAsmRegClass {
        reg,
    }
}

impl ArcInlineAsmRegClass {
    pub fn valid_modifiers(self, _arch: super::InlineAsmArch) -> &'static [char] {
        &[]
    }

    pub fn suggest_class(self, _arch: InlineAsmArch, _ty: InlineAsmType) -> Option<Self> {
        None
    }

    pub fn suggest_modifier(
        self,
        _arch: InlineAsmArch,
        _ty: InlineAsmType,
    ) -> Option<ModifierInfo> {
        None
    }

    pub fn default_modifier(self, _arch: InlineAsmArch) -> Option<ModifierInfo> {
        None
    }

    pub fn supported_types(
        self,
        _arch: InlineAsmArch,
    ) -> &'static [(InlineAsmType, Option<Symbol>)] {
        match self {
            Self::reg => types! { _: I8, I16, I32; },
        }
    }
}

// ARC700 / ARCompact general-purpose register file:
//   r0..r7   — argument and return (caller-saved)
//   r8..r12  — temporaries (caller-saved)
//   r13..r25 — callee-saved
//   r26 (gp) — global pointer, reserved by ABI
//   r27 (fp) — frame pointer
//   r28 (sp) — stack pointer
//   r29 (ilink1), r30 (ilink2) — interrupt link registers
//   r31 (blink) — branch link register, clobbered by BL
def_regs! {
    Arc ArcInlineAsmReg ArcInlineAsmRegClass {
        r0: reg = ["r0"],
        r1: reg = ["r1"],
        r2: reg = ["r2"],
        r3: reg = ["r3"],
        r4: reg = ["r4"],
        r5: reg = ["r5"],
        r6: reg = ["r6"],
        r7: reg = ["r7"],
        r8: reg = ["r8"],
        r9: reg = ["r9"],
        r10: reg = ["r10"],
        r11: reg = ["r11"],
        r12: reg = ["r12"],
        r13: reg = ["r13"],
        r14: reg = ["r14"],
        r15: reg = ["r15"],
        r16: reg = ["r16"],
        r17: reg = ["r17"],
        r18: reg = ["r18"],
        r19: reg = ["r19"],
        r20: reg = ["r20"],
        r21: reg = ["r21"],
        r22: reg = ["r22"],
        r23: reg = ["r23"],
        r24: reg = ["r24"],
        r25: reg = ["r25"],

        #error = ["r26", "gp"] =>
            "the global pointer cannot be used as an operand for inline asm",
        #error = ["r27", "fp"] =>
            "the frame pointer cannot be used as an operand for inline asm",
        #error = ["r28", "sp"] =>
            "the stack pointer cannot be used as an operand for inline asm",
        #error = ["r29", "ilink1"] =>
            "the interrupt link register cannot be used as an operand for inline asm",
        #error = ["r30", "ilink2"] =>
            "the interrupt link register cannot be used as an operand for inline asm",
        #error = ["r31", "blink"] =>
            "the branch link register cannot be used as an operand for inline asm",
        #error = ["r63", "pcl"] =>
            "the program counter cannot be used as an operand for inline asm",
    }
}

impl ArcInlineAsmReg {
    pub fn emit(
        self,
        out: &mut dyn fmt::Write,
        _arch: InlineAsmArch,
        _modifier: Option<char>,
    ) -> fmt::Result {
        out.write_str(self.name())
    }
}

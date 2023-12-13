use clap::ValueEnum;

#[derive(ValueEnum, Copy, Clone, Debug)]
#[warn(non_camel_case_types)]
pub enum TargetObjectType { X86_64, Macos }

impl TargetObjectType {
    pub fn object_format(&self) -> &'static str {
        match self {
            Self::X86_64 => "elf64",
            Self::Macos => "macho64"
        }
    }
}

pub enum SyscallTable {
    Exit
}

impl SyscallTable {
    pub fn code(&self, arch: TargetObjectType) -> u32 {
        match self {
            Self::Exit => match arch {
                TargetObjectType::X86_64 => 60,
                TargetObjectType::Macos => 0x2000001
            }
        }
    }
}
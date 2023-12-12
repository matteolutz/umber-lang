use clap::ValueEnum;

#[derive(ValueEnum, Copy, Clone, Debug)]
pub enum ArchType {
    #[warn(non_camel_case_types)]
    X86_64,
    Macos
}

impl ArchType {
    pub fn nasm_format(&self) -> &'static str {
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
    pub fn code(&self, arch: ArchType) -> u32 {
        match self {
            Self::Exit => match arch {
                ArchType::X86_64 => 60,
                ArchType::Macos => 0x2000001
            }
        }
    }
}
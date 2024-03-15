use clap::ValueEnum;

pub enum CallingConvention {
    Unix,
    Win,
}

#[derive(ValueEnum, Copy, Clone, Debug)]
#[warn(non_camel_case_types)]
pub enum TargetObjectType {
    X86_64,
    Win64,
    Macos,
}

impl TargetObjectType {
    pub fn object_format(&self) -> &'static str {
        match self {
            Self::X86_64 => "elf64",
            Self::Win64 => "win64",
            Self::Macos => "macho64",
        }
    }

    pub fn object_file_extension(&self) -> &'static str {
        match self {
            Self::X86_64 => "o",
            Self::Win64 => "obj",
            Self::Macos => "o",
        }
    }

    pub fn calling_convention(&self) -> CallingConvention {
        match self {
            Self::X86_64 => CallingConvention::Unix,
            Self::Win64 => CallingConvention::Win,
            Self::Macos => CallingConvention::Unix,
        }
    }
}

pub enum SyscallTable {
    Exit,
}

impl SyscallTable {
    pub fn code(&self, arch: TargetObjectType) -> u32 {
        match self {
            Self::Exit => match arch {
                TargetObjectType::X86_64 => 60,
                TargetObjectType::Win64 => 1,
                TargetObjectType::Macos => 0x2000001,
            },
        }
    }
}

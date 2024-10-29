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
    Macho64,
}

impl TargetObjectType {
    pub fn object_format(&self) -> &'static str {
        match self {
            Self::X86_64 => "elf64",
            Self::Win64 => "win64",
            Self::Macho64 => "macho64",
        }
    }

    pub fn object_file_extension(&self) -> &'static str {
        match self {
            Self::X86_64 => "o",
            Self::Win64 => "obj",
            Self::Macho64 => "o",
        }
    }

    pub fn calling_convention(&self) -> CallingConvention {
        match self {
            Self::X86_64 => CallingConvention::Unix,
            Self::Win64 => CallingConvention::Win,
            Self::Macho64 => CallingConvention::Unix,
        }
    }

    pub fn should_use_rel(&self) -> bool {
        match self {
            Self::X86_64 => false,
            Self::Win64 => false,
            Self::Macho64 => true,
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
                TargetObjectType::Macho64 => 0x2000001,
            },
        }
    }
}

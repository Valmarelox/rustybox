use std::fs::Metadata;
use std::convert::TryFrom;
use num_enum::TryFromPrimitiveError;
use num_enum::TryFromPrimitive;
use std::os::unix::fs::MetadataExt;

#[derive(PartialEq, Clone, Copy, Display, Debug,TryFromPrimitive)]
#[repr(u32)]
pub enum FileType {
    #[strum(serialize="s")]
    Socket          = 0o140000,
    #[strum(serialize="l")]
    SymbolicLink    = 0o120000,
    #[strum(serialize=".")]
    RegularFile     = 0o100000,
    #[strum(serialize="b")]
    BlockDevice     = 0o060000,
    #[strum(serialize="d")]
    Directory       = 0o040000,
    #[strum(serialize="c")]
    CharDevice      = 0o020000 ,
    #[strum(serialize="f")]
    Fifo            = 0o010000,
}

impl TryFrom<Metadata> for FileType {
    type Error = TryFromPrimitiveError<Self>;

    fn try_from(f: Metadata) -> Result<Self, Self::Error> {
        FileType::try_from(f.mode() & 0o170000)
    }
}

#[cfg(test)]
mod tests {
    use super::FileType;
    use std::convert::TryFrom;
    use crate::librb::file::filemeta::FileMetadata;

    #[test]
    fn test_file_type_try_from() {
        for x in [FileType::Socket, FileType::RegularFile, FileType::Directory, FileType::BlockDevice, FileType::CharDevice, FileType::Fifo, FileType::SymbolicLink].iter() {
            let x = *x;
            let y = x as u32;
            let f = FileType::try_from(y).ok().unwrap(); // | 0xffff0fff
            assert_eq!(x, f);
        }
    }
}
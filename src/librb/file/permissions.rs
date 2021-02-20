use core::fmt;
use core::result::Result::Ok;

bitflags! {
    pub struct FilePermissions: u32 {
        const PF_R = 0o4;
        const PF_W = 0o2;
        const PF_X = 0o1;
    }
}

bitflags! {
    pub struct PermissionsMask: u32 {
        const S_IRUSR = 0o0400;
        const S_IWUSR = 0o0200;
        const S_IXUSR = 0o0100;
        const S_IRGRP = 0o0040;
        const S_IWGRP = 0o0020;
        const S_IXGRP = 0o0010;
        const S_IROTH = 0o0004;
        const S_IWOTH = 0o0002;
        const S_IXOTH = 0o0001;
    }

}

impl PermissionsMask {
    pub fn user(&self) -> FilePermissions {
        FilePermissions{bits: (self.bits >> 6) & 0o7}
    }
    pub fn group(&self) -> FilePermissions {
        FilePermissions{bits: (self.bits >> 3) & 0o7}
    }
    pub fn other(&self) -> FilePermissions {
        FilePermissions{bits: (self.bits >> 0) & 0o7}
    }

    pub fn build(v: u32) -> Self {
        Self { bits: v & 0o777 }
    }
}

impl fmt::Display for FilePermissions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.contains(FilePermissions::PF_R) {
            write!(f, "r")?;
        } else {
            write!(f, "-")?;
        }
        if self.contains(FilePermissions::PF_W) {
            write!(f, "w")?;
        } else {
            write!(f, "-")?;
        }
        if self.contains(FilePermissions::PF_X) {
            write!(f, "x")?;
        } else {
            write!(f, "-")?;
        }
    Ok(())
    }
}

impl fmt::Display for PermissionsMask{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.user())?;
        write!(f, "{}", self.group())?;
        write!(f, "{}", self.other())
    }
}

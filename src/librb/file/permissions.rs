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
    fn _get_bits(&self, shift: u8) -> FilePermissions {
        FilePermissions{bits: (self.bits >> shift) & 0o7}
    }
    pub fn user(&self) -> FilePermissions { self._get_bits(6) }
    pub fn group(&self) -> FilePermissions { self._get_bits(3) }
    pub fn other(&self) -> FilePermissions { self._get_bits(0) }

    pub fn build(v: u32) -> Self {
        Self { bits: v & 0o777 }
    }
}

impl fmt::Display for FilePermissions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let perms = [(FilePermissions::PF_R, "r"), (FilePermissions::PF_W, "w"), (FilePermissions::PF_X, "x")];
        let formatted = perms.iter().map(|(p, s)| if self.contains(*p) {s} else {"-"}).collect::<Vec<&str>>().join("");
        write!(f, "{}", formatted)
    }
}

impl fmt::Display for PermissionsMask{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.user())?;
        write!(f, "{}", self.group())?;
        write!(f, "{}", self.other())
    }
}

#[cfg(test)]
mod tests {
    use super::FilePermissions;

    #[test]
    fn test_file_permissions_values() {
        assert_eq!(FilePermissions::PF_R.bits, 4);
        assert_eq!(FilePermissions::PF_X.bits, 1);
        assert_eq!(FilePermissions::PF_W.bits, 2);
        assert_eq!((FilePermissions::PF_W | FilePermissions::PF_X | FilePermissions::PF_R).bits, 7);
    }
    #[test]
    fn test_file_permissions_format() {
        assert_eq!(format!("{}", (FilePermissions::PF_R)), "r--");
        assert_eq!(format!("{}", (FilePermissions::PF_W)), "-w-");
        assert_eq!(format!("{}", (FilePermissions::PF_R | FilePermissions::PF_W)), "rw-");
        assert_eq!(format!("{}", (FilePermissions::PF_X)), "--x");
        assert_eq!(format!("{}", (FilePermissions::PF_R | FilePermissions::PF_X)), "r-x");
        assert_eq!(format!("{}", (FilePermissions::PF_W | FilePermissions::PF_X)), "-wx");
        assert_eq!(format!("{}", (FilePermissions::PF_R | FilePermissions::PF_W | FilePermissions::PF_X)), "rwx");

    }
}
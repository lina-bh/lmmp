use libc::c_int;
use std::collections::HashMap;
use std::convert::From;
use std::path::Path;
use std::{error, ffi, fmt};
use vorbisfile_sys::{ov_clear, ov_comment, ov_fopen, OggVorbis_File};

#[derive(Debug)]
pub enum Error {
    Nul(ffi::NulError),
    Vorbis(c_int),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Nul(e) => e.fmt(f),
            Error::Vorbis(c) => write!(
                f,
                "{}",
                match *c {
                    vorbis_sys::OV_EREAD => "A read from media returned an error",
                    vorbis_sys::OV_ENOTVORBIS => "Bitstream does not contain any Vorbis data",
                    vorbis_sys::OV_EVERSION => "Vorbis version mismatch",
                    vorbis_sys::OV_EBADHEADER => "Invalid Vorbis bitstream header",
                    vorbis_sys::OV_EFAULT => "Internal logic fault",
                    c => return write!(f, "Undescribed error {}", c),
                },
            ),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Error::Nul(e) => Some(e),
            _ => None,
        }
    }
}

impl From<ffi::NulError> for Error {
    fn from(e: ffi::NulError) -> Self {
        Self::Nul(e)
    }
}

pub struct VorbisFile {
    vf: OggVorbis_File,
}

impl VorbisFile {
    pub fn open(path: impl AsRef<Path>) -> Result<VorbisFile, Error> {
        let vf = unsafe { open_vf(path.as_ref())? };

        Ok(VorbisFile { vf })
    }

    pub fn comments(&mut self) -> HashMap<String, String> {
        let mut map = HashMap::<String, String>::new();
        let list = match unsafe { self.comments_list() } {
            Some(v) => v,
            None => return map,
        };
        for c in list {
            let it = c.split('=').take(2).collect::<Vec<&str>>();
            if let Some((k, v)) = it.get(0).zip(it.get(1)) {
                map.insert((*k).to_owned(), (*v).to_owned());
            }
        }
        map
    }

    unsafe fn comments_list(&mut self) -> Option<Vec<String>> {
        use std::slice;

        let com = ov_comment(&mut self.vf, -1);
        if com.is_null() {
            return None;
        }

        let len = (*com).comments as usize;
        let comments = slice::from_raw_parts((*com).user_comments as *mut *mut u8, len);
        let lengths = slice::from_raw_parts((*com).comment_lengths, len);

        Some(
            comments
                .iter()
                .zip(lengths)
                .map(|(s, n)| {
                    let sl = slice::from_raw_parts(*s, *n as usize);
                    String::from_utf8_lossy(sl).into_owned()
                })
                .collect::<Vec<String>>(),
        )
    }
}

impl Drop for VorbisFile {
    fn drop(&mut self) {
        unsafe {
            if ov_clear(&mut self.vf) != 0 {
                panic!("ov_clear failed");
            }
        }
    }
}

unsafe fn open_vf(p: &Path) -> Result<OggVorbis_File, Error> {
    use std::ffi::CString;
    use std::mem::MaybeUninit;

    let cs;
    #[cfg(unix)]
    {
        use std::os::unix::ffi::OsStrExt;

        cs = CString::new(p.as_os_str().as_bytes())?;
    }

    let mut vf = MaybeUninit::<OggVorbis_File>::zeroed();
    let ret = ov_fopen(cs.as_ptr(), vf.as_mut_ptr());
    match ret {
        0 => Ok(vf.assume_init()),
        c => Err(Error::Vorbis(c)),
    }
}

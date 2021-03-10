use super::Track;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use std::collections::HashMap;
use std::convert::From;
use std::error::Error;
use std::ffi::{self, CString};
use std::fmt::{self, Display};
use std::mem::MaybeUninit;
use std::path::{Path, PathBuf};
use std::slice;
use vorbisfile_sys::{ov_clear, ov_comment, ov_fopen, OggVorbis_File};

#[derive(Debug, FromPrimitive, Copy, Clone)]
#[allow(non_camel_case_types)]
#[repr(i32)]
enum OvCode {
    OV_EREAD = -128,
    OV_EFAULT = -129,
    OV_EIMPL = -130,
    OV_EINVAL = -131,
    OV_ENOTVORBIS = -132,
    OV_EBADHEADER = -133,
    OV_EVERSION = -134,
    OV_ENOTAUDIO = -135,
    OV_EBADPACKET = -136,
    OV_EBADLINK = -137,
    OV_ENOSEEK = -138,
}

#[derive(Debug)]
enum FopenError {
    Nul(ffi::NulError),
    Fopen(OvCode),
}

impl Display for FopenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        use FopenError::*;
        match self {
            Nul(nul) => nul.fmt(f),
            Fopen(code) => {
                use OvCode::*;
                let msg = match *code {
                    OV_EREAD => "A read from media returned an error",
                    OV_ENOTVORBIS => "Bitstream does not contain any Vorbis data",
                    OV_EVERSION => "Vorbis version mismatch",
                    OV_EBADHEADER => "Invalid Vorbis bitstream header",
                    OV_EFAULT => "Internal logic fault",
                    _ => "Undescribed error",
                };
                write!(f, "{}", msg)
            }
        }
    }
}

impl Error for FopenError {}

impl From<ffi::NulError> for FopenError {
    fn from(e: ffi::NulError) -> FopenError {
        FopenError::Nul(e)
    }
}

struct VorbisFile {
    vf: OggVorbis_File,
}

impl VorbisFile {
    pub fn new(path: impl AsRef<Path>) -> Result<VorbisFile, Box<dyn Error>> {
        let vf = unsafe { Self::open_vf(path.as_ref())? };

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

    #[cfg(unix)]
    unsafe fn open_vf(p: &Path) -> Result<OggVorbis_File, FopenError> {
        use std::os::unix::ffi::OsStrExt;

        let cs = CString::new(p.as_os_str().as_bytes())?;
        let mut vf = MaybeUninit::<OggVorbis_File>::zeroed();
        let ret = ov_fopen(cs.as_ptr(), vf.as_mut_ptr());
        match ret {
            0 => Ok(vf.assume_init()),
            c => Err(FopenError::Fopen(
                OvCode::from_i32(c).expect("unknown error code"),
            )),
        }
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

pub unsafe fn parse(path: impl AsRef<Path>) -> Result<Option<Track>, Box<dyn Error>> {
    let vf = VorbisFile::new(path)?;

    unimplemented!()
}

pub fn _test() {
    let ogg = "/home/lina/Music/Gnod with White Hills - Gnod Drop Out With White Hills II/bits.ogg";
    let mut vf = VorbisFile::new(&ogg).unwrap();
    eprintln!("successfully opened");
    let tags = vf.comments();
    eprintln!("{:?}", tags.is_empty());
    for (k, v) in tags.iter() {
        eprintln!("{}: {}", k, v);
    }
    ::std::mem::drop(vf);
    eprintln!("successfully closed");
}

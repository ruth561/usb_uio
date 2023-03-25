use crate::helper::DynError;
use std::{
    io::{BufReader, Read}, 
    fs::{
        self,
        File,
        OpenOptions,
    },
    os::fd::AsRawFd,
    vec,
    ffi::c_void,
};
use nix::sys::mman::{MapFlags, ProtFlags};
use libc;
use std::num::NonZeroUsize;

pub struct UioPciDevice {
    uio_num: usize,
    _dev_file: File, // /dev/uioX形式のファイル
    pci_config_file: File, // pci configuration spaceのファイル
}

impl UioPciDevice {
    pub fn new(uio_num: usize) -> Result<UioPciDevice, DynError> {
        Ok(Self { 
            uio_num, 
            _dev_file: File::open(format!("/dev/uio{}", uio_num))?,
            pci_config_file: File::open(format!("/sys/class/uio/uio{}/device/config", uio_num))?,
        })
    }

    pub fn read_pci(&mut self) -> Result<Vec<u8>, DynError>
    {
        let mut buf = vec![];
        let mut reader = BufReader::new(&mut self.pci_config_file);
        reader.read_to_end(&mut buf)?;
        dump(&buf);
        Ok(buf)
    }

    pub fn get_name(&self) -> Result<String, DynError>
    {
        let path = format!("/sys/class/uio/uio{}/name", self.uio_num);
        let mut f = File::open(path)?;
        let mut buf = String::new();
        f.read_to_string(&mut buf)?;
        Ok(buf)
    }

    /// Pci Config SpaceのBARが指す先のMMIO領域は、
    /// /sys/class/uio/uioX/device/resourceYというファイルをマップすることで
    /// アクセスすることができる。
    /// XはUIO番号、YはBARの番号である。
    pub fn map_mmio(&self, bar_num: usize) -> Result<*mut c_void, DynError> {
        let path = format!("/sys/class/uio/uio{}/device/resource{}", self.uio_num, bar_num);
        let mmio = OpenOptions::new()
            .read(true)
            .write(true)
            .open(&path)?;
        let metadata = fs::metadata(&path)?;
        let fd = mmio.as_raw_fd();

        let res = unsafe {
            nix::sys::mman::mmap(
                None,
                NonZeroUsize::new(metadata.len() as usize).unwrap(),
                ProtFlags::PROT_READ | ProtFlags::PROT_WRITE,
                MapFlags::MAP_SHARED,
                fd,
                0 as libc::off_t,
            )
        };
        match res {
            Ok(m) => Ok(m),
            Err(_) => Err("Failed to map mmio.".into()),
        }
    }
}

/// bytes型をダンプするmisc関数
fn dump(bytes: &[u8])
{
    for (i, b) in bytes.iter().enumerate() {
        if i % 16 == 0 { print!("\n{:#04x}: ", i); }
        print!(" {:02x}", b);
    }
    println!();
}
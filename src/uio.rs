use crate::helper::DynError;
use std::{
    io::{BufReader, Read}, 
    fs::File,
    vec,
};

pub struct UioPciDevice {
    uio_num: usize,
    dev_file: File, // /dev/uioX形式のファイル
    pci_config_file: File, // pci configuration spaceのファイル
}

impl UioPciDevice {
    pub fn new(uio_num: usize) -> Result<UioPciDevice, DynError> {
        Ok(Self { 
            uio_num, 
            dev_file: File::open(format!("/dev/uio{}", uio_num))?,
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
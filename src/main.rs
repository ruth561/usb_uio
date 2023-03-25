pub mod helper;
pub mod uio;
use uio::*;


pub fn main() {



    let uio_num = 0; // /dev/uio0
    let mut dev = UioPciDevice::new(uio_num).unwrap();
    dev.read_pci().unwrap();
}
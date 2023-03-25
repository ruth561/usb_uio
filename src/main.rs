pub mod helper;
pub mod uio;
pub mod pci;
use uio::*;


pub fn main() {
    let uio_num = 0; // /dev/uio0
    let mut dev = UioPciDevice::new(uio_num).unwrap();
    let pci_buf = dev.read_pci().unwrap();
    pci::dump(&pci_buf);
}
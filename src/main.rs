pub mod helper;
pub mod uio;
pub mod pci;
use uio::*;


pub fn main() {
    let uio_num = 0; // /dev/uio0
    let mut dev = UioPciDevice::new(uio_num).unwrap();
    println!("name: {}", dev.get_name().unwrap());
    let pci_buf = dev.read_pci().unwrap();
    pci::pci_dump(&pci_buf);
    let p = dev.map_mmio(0).unwrap() as *mut u8;
    println!("mmio_addr: {:p}", p);
    dump(p, 0x40);
}

fn dump(p: *mut u8, n: usize)
{
    for i in 0..n {
        if i % 0x10 == 0 { print!("\n{:#04x}: ", i); }
        print!(" {:02x}", unsafe { *p.add(i) });
    }
    println!();
}
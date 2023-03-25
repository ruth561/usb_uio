pub mod helper;
pub mod uio;
pub mod pci;
use uio::*;
use accessor::mapper::Mapper;
use std::num::NonZeroUsize;

/// uioでは、ユーザー空間で実行するため、
/// 物理アドレスと仮想アドレスの変換はいらない？
#[derive(Clone)]
struct MemoryMapper;
impl Mapper for MemoryMapper {
    unsafe fn map(&mut self, phys_start: usize, _bytes: usize) -> NonZeroUsize {
        NonZeroUsize::new(phys_start).unwrap()
    }

    fn unmap(&mut self, _virt_start: usize, _bytes: usize) {}
}

pub fn main() {
    let uio_num = 0; // /dev/uio0
    let mut dev = UioPciDevice::new(uio_num).unwrap();
    println!("name: {}", dev.get_name().unwrap());
    let pci_buf = dev.read_pci().unwrap();
    pci::pci_dump(&pci_buf);
    let p = dev.map_mmio(0).unwrap() as *mut u8;
    println!("mmio_addr: {:p}", p);
    dump(p, 0x40);

    let mapper = MemoryMapper;
    let xhci_regs = unsafe { xhci::Registers::new(p as usize, mapper) };
    println!("{:?}", xhci_regs.capability.caplength.read_volatile());
    println!("{:?}", xhci_regs.capability.hccparams1.read_volatile());
    println!("{:?}", xhci_regs.capability.dboff.read_volatile());
    let usbsts = xhci_regs.operational.usbsts.read_volatile();
    println!("usbsts: {:?}", usbsts);
}

fn dump(p: *mut u8, n: usize)
{
    for i in 0..n {
        if i % 0x10 == 0 { print!("\n{:#04x}: ", i); }
        print!(" {:02x}", unsafe { *p.add(i) });
    }
    println!();
}
pub mod helper;
pub mod uio;
pub mod pci;
use uio::*;
use accessor::mapper::Mapper;
use std::num::NonZeroUsize;
use log::info;

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
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

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
    let mut opt = xhci_regs.operational;
    let cap = xhci_regs.capability;

    // ここからxhciの初期化を始めていく
    println!("[*] start initializing xhc");
    if !opt.usbsts.read_volatile().hc_halted() { 
        eprintln!("xHC is not halted.");
        // リセットの開始
        opt.usbcmd.update_volatile(|regs| { regs.clear_run_stop(); });
    }
    // 停止するまで待つ
    while !opt.usbsts.read_volatile().hc_halted() {}

    // リセット処理を開始
    println!("usbcmd.reset: {}", opt.usbcmd.read_volatile().host_controller_reset());
    opt.usbcmd.update_volatile(|regs| { regs.set_host_controller_reset(); });
    while opt.usbcmd.read_volatile().host_controller_reset() {
        println!("xHC is resetting.");
    }
    println!("xHC is reset.");
    // レジスタへの書き込みが許可されるまで待つ
    while opt.usbsts.read_volatile().controller_not_ready() {
        println!("xHC is not writable yet.");
    }
    info!("max ports: {}", cap.hcsparams1.read_volatile().number_of_ports());
    info!("max device slots: {}", cap.hcsparams1.read_volatile().number_of_device_slots());
    info!("max interrupters: {}", cap.hcsparams1.read_volatile().number_of_interrupts());

}

fn dump(p: *mut u8, n: usize)
{
    for i in 0..n {
        if i % 0x10 == 0 { print!("\n{:#04x}: ", i); }
        print!(" {:02x}", unsafe { *p.add(i) });
    }
    println!();
}
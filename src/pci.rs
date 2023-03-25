
#[repr(C)]
pub struct PciConfigSpace {
    pub vendor_id: u16, // 0x00
    pub device_id: u16,
    pub command: u16, // 0x04
    pub status: u16, 
    pub revision_id: u8, // 0x08
    pub class_code: (u8, u8, u8),
    _0: u16, // 0x0c
    pub header_type: u8,
    _1: u8,
    bar: u64, // 0x10
}

/// bytes列で渡されたPci Config Spaceをダンプする関数
pub fn dump(buf: &[u8])
{
    let config = unsafe { (buf.as_ptr() as *const PciConfigSpace).as_ref().unwrap() };
    println!("vendor_id: {:#04x}", config.vendor_id);
    println!("device_id: {:#04x}", config.device_id);
    println!("command: {:#04x}", config.command);
    println!("status: {:#04x}", config.status);
    println!("revision_id: {:#02x}", config.revision_id);
    println!("class_code: ({:#02x}, {:#02x}, {:#02x})"
        , config.class_code.0, config.class_code.1, config.class_code.2);
    println!("header_type: {:#02x}", config.header_type);
    println!("bar: {:#016x}", config.bar);
}


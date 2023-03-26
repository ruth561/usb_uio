
#[repr(C, align(4096))]
pub struct DeviceContextBaseAddressArray {
    scratchpad_buffers: u64,
    context: [u64; 255],
}

impl Default for DeviceContextBaseAddressArray {
    fn default() -> Self { Self::new() }
}

impl DeviceContextBaseAddressArray {
    pub fn new() -> Self {
        Self { scratchpad_buffers: 0, context: [0; 255] }
    }

    pub fn raw_ptr(&self) -> u64 {
        // もっときれいな実装がある〜？
        (&self.scratchpad_buffers as *const u64) as u64 
    } 
}
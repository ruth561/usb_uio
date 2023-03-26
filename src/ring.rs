
use log::debug;
use xhci::ring::trb::{
    Link,
    command::Allowed
};

const RING_SIZE: usize = 5;

#[repr(C, align(0x10000))]
pub struct CommandRing {
    trbs: [[u32; 4]; RING_SIZE],
    enq_idx: usize, 
    deq_idx: usize, 
    cycle_state: bool, 
}

impl Default for CommandRing {
    fn default() -> Self { Self::new() }
}

impl CommandRing {
    pub fn new() -> Self {
        let mut ring = Self { 
            trbs: [[0; 4]; RING_SIZE], 
            enq_idx: 0, 
            deq_idx: 0,
            cycle_state: true, 
        };
        // Ringの最後にLinkedTrbを設置
        let mut link_trb = Link::new();
        debug!("command ring pointer: {:#016x}", ring.raw_ptr());
        link_trb.set_ring_segment_pointer(ring.raw_ptr());
        link_trb.set_toggle_cycle();
        ring.trbs[RING_SIZE - 1] = link_trb.into_raw();
        ring.print_debug_ring();
        ring
    }

    pub fn push(&mut self, mut trb: Allowed) {
        // TODO: dequeueポインタの位置を把握し、プッシュしていいか判断

        if self.cycle_state { trb.set_cycle_bit(); }
        else { trb.clear_cycle_bit(); }
        self.trbs[self.enq_idx] = trb.into_raw();
        self.enq_idx += 1;

        // リングバッファは1つしか使用していないので、
        // Link TRBに到達したら、enqueueはバッファの先頭へ、
        // cycle_stateは反転させる。
        if let Ok(mut link) = Link::try_from(self.trbs[self.enq_idx]) {
            if self.cycle_state { link.set_cycle_bit(); }
            else { link.clear_cycle_bit(); }
            self.trbs[self.enq_idx] = link.into_raw();
            self.cycle_state = !self.cycle_state;
            self.enq_idx = 0;
        }

        self.print_debug_ring();
    }

    pub fn raw_ptr(&self) -> u64 {
        self as *const CommandRing as u64
    }

    fn print_debug_ring(&self) {
        // debug
        debug!("[*] command ring is initialized");
        for i in 0..RING_SIZE {
            debug!("{:02}: {:08x} {:08x} {:08x} {:08x}   ({:p}) {}", i, 
                self.trbs[i][0], self.trbs[i][1], self.trbs[i][2], self.trbs[i][3], 
                &self.trbs[i] as *const _,
                if self.enq_idx == i { "<- enqueue" } else { "" }
            );
        }
    }
}



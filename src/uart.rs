pub type SendFn = Box<dyn Fn(u8)>;
pub type RecvFn = Box<dyn Fn() -> Option<u8>>;

const FCTL_FIFOEN: u8 = 0x1;

pub struct Uart {
    send_fn: SendFn,
    recv_fn: RecvFn,
    rx_buf: Option<u8>,

    /* number of clock cycles before a byte can be sent again */
    transmit_cooldown: i32,

    // interrupt enable register
    pub ier: u8,
    // FIFO control register
    fctl: u8,
    // Line control register
    pub lctl: u8, // lctl & 0x80 enables access to baud rate generator
                  // registers where rbr/thr and ier are normally accessed
    pub brg_div: u16,
    // scratch pad register
    pub spr: u8,

    tx_fifo: Vec<u8>,
}

impl Uart {
    pub fn new(send_fn: SendFn, recv_fn: RecvFn) -> Self {
        Uart {
            send_fn, recv_fn,
            transmit_cooldown: 0,
            tx_fifo: vec![],
            ier: 0, fctl: 0, lctl: 0, brg_div: 2, spr: 0, rx_buf: None
        }
    }

    pub fn apply_ticks(&mut self, cycles: i32) {
        self.transmit_cooldown = i32::max(0, self.transmit_cooldown - cycles);
        if self.transmit_cooldown == 0 {
            if !self.tx_fifo.is_empty() {
                let val = self.tx_fifo.remove(0);
                // actually send
                (*self.send_fn)(val);
                self.transmit_cooldown += self.brg_div as i32 * 16 * 9; /* XXX 9 = 8bits data, 1 bit parity */
            }
        }
    }

    pub fn send_byte(&mut self, value: u8) {
        if (self.tx_fifo.len() < 16 && self.fctl & FCTL_FIFOEN != 0) ||
           (self.tx_fifo.is_empty() && self.fctl & FCTL_FIFOEN == 0) {
            self.tx_fifo.push(value);
        } else {
            // drop the data. the ez80 was pushing data too fast
        }
    }

    pub fn maybe_fill_rx_buf(&mut self) -> Option<u8> {
        if self.rx_buf == None {
            self.rx_buf = (*self.recv_fn)();
        }
        self.rx_buf
    }

    pub fn receive_byte(&mut self) -> u8 {
        // uart0 receive
        self.maybe_fill_rx_buf();

        let maybe_data = self.rx_buf;
        self.rx_buf = None;

        match maybe_data {
            Some(data) => data,
            None => 0
        }
    }

    /** line status register */
    pub fn read_lsr(&mut self) -> u8 {
        // 0x01 = DR (data ready: ie can receive)
        (if self.maybe_fill_rx_buf().is_some() { 1 } else { 0 }) |
        // 0x20 = TRHE (fifo / transmit  holding register empty)
        (if self.tx_fifo.is_empty() { 0x20 } else { 0 }) |
        // 0x40 = TEMT (fifo / transmit holding register empty & transmitter idle)
        (if self.tx_fifo.is_empty() && self.transmit_cooldown == 0 { 0x40 } else { 0 })
    }

    pub fn write_fctl(&mut self, val: u8) {
        self.fctl = val;
    }

    /*
    pub fn get_baud_rate(&self) -> u32 {
        18_432_000 / (self.brg_div as u32 * 16)
    }
    */

    pub fn is_access_brg_registers(&self) -> bool {
        self.lctl & 0x80 != 0
    }

    pub fn is_rx_interrupt_enabled(&self) -> bool {
        self.ier & 1 != 0
    }
}

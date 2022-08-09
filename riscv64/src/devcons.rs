// Racy to start.

use crate::uart16550::Uart16550;
use port::devcons::Console;

// use 0x10000000 as default
static mut UART: Uart16550 = Uart16550::new(0x10000000);

pub fn init(bse_address: usize) {
    unsafe {
        UART.init(bse_address, 115_200);
    }
    Console::new(unsafe { &mut UART });
}

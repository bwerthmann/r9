// Racy to start.

use crate::sbiuart::sbiuart;
use port::devcons::Console;

static mut UART: sbiuart = sbiuart::new();

pub fn init() {
    Console::new(unsafe { &mut UART });
}

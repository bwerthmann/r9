use core::fmt::Error;
use core::fmt::Write;

use port::devcons::Uart;

pub struct sbiuart {}

impl Write for sbiuart {
    fn write_str(&mut self, out: &str) -> Result<(), Error> {
        for c in out.bytes() {
            self.putb(c);
        }
        Ok(())
    }
}

impl Uart for sbiuart {
    fn putb(&self, b: u8) {
        crate::sbi::consputb(b);
    }
}

impl sbiuart {
    pub const fn new() -> Self {
        sbiuart {}
    }
}

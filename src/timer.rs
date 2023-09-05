#[macro_export]
macro_rules! timer {
    ($(
        $TIMERX:ident: $PACTIMERX:ty,
    )+) => {
        $(
            #[derive(Debug)]
            pub struct $TIMERX {
                registers: $PACTIMERX,
                pub sys_clk: u32,
            }

            impl $TIMERX {
                pub fn new(registers: $PACTIMERX, sys_clk: u32) -> Self {
                    Self { registers, sys_clk }
                }

                pub fn free(self) -> $PACTIMERX {
                    self.registers
                }

                pub fn uptime(&self) -> u64 {
                    self.registers.uptime_latch.write(unsafe { |w| w.uptime_latch().bit(true) } );
                    let cycles0: u32 = self.registers.uptime_cycles0.read().bits();
                    let cycles1: u32 = self.registers.uptime_cycles1.read().bits();
                    ((cycles1 as u64) << 32) | (cycles0 as u64)
                }
            }

            impl<UXX: core::convert::Into<u32>> $crate::hal::blocking::delay::DelayMs<UXX> for $TIMERX {
                fn delay_ms(&mut self, ms: UXX) -> () {
                    let value: u32 = self.sys_clk / 1_000 * ms.into();
                    unsafe {
                        self.registers.en.write(|w| w.bits(0));
                        self.registers.reload.write(|w| w.bits(0));
                        self.registers.load.write(|w| w.bits(value));
                        self.registers.en.write(|w| w.bits(1));
                        self.registers.update_value.write(|w| w.bits(1));
                        while self.registers.value.read().bits() > 0 {
                            self.registers.update_value.write(|w| w.bits(1));
                        }
                    }
                }
            }
        )+
    }
}

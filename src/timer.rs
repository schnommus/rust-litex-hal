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
                    riscv::interrupt::free(|| {
                        self.registers.uptime_latch().write(unsafe { |w| w.uptime_latch().bit(true) } );
                        let cycles0: u32 = self.registers.uptime_cycles0().read().bits();
                        let cycles1: u32 = self.registers.uptime_cycles1().read().bits();
                        ((cycles1 as u64) << 32) | (cycles0 as u64)
                    })
                }

                pub fn set_periodic_event(&self, period_ms: u32) {
                    let value: u32 = (self.sys_clk / 1_000) * period_ms;
                    unsafe {
                        self.registers.en().write(|w| w.bits(0));
                        self.registers.load().write(|w| w.load().bits(value));
                        self.registers.reload().write(|w| w.reload().bits(value));
                        self.registers.ev_enable().write(|w| w.bits(1));
                        self.registers.en().write(|w| w.bits(1));
                    }
                }
            }

            impl<UXX: core::convert::Into<u32>> $crate::hal::blocking::delay::DelayMs<UXX> for $TIMERX {
                fn delay_ms(&mut self, ms: UXX) -> () {
                    let start: u64 = self.uptime();
                    let value: u32 = self.sys_clk / 1_000 * ms.into();
                    while (self.uptime() - start) < value as u64 {
                        // Blocking wait for the timer
                    }
                }
            }
        )+
    }
}

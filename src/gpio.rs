#[macro_export]
macro_rules! gpio {
    ($(
        $GPIOX:ident: $PACGPIOX:ty,
    )+) => {
        $(
            #[derive(Debug)]
            pub struct $GPIOX {
                pub index: usize,
            }

            impl $GPIOX {
                pub fn new(index: usize) -> Self {
                    Self { index }
                }
            }

            impl $crate::hal::digital::v2::OutputPin for $GPIOX {
                type Error = core::convert::Infallible;

                fn set_low(&mut self) -> Result<(), Self::Error> {
                    let reg = unsafe { &*<$PACGPIOX>::ptr() };
                    let mask: u32 = !(1 << self.index);
                    riscv::interrupt::free(|| {
                        let val: u32 = reg.out().read().bits() & mask;
                        unsafe {
                            reg.out().write(|w| w.bits(val));
                        }
                    });
                    Ok(())
                }
                fn set_high(&mut self) -> Result<(), Self::Error> {
                    let reg = unsafe { &*<$PACGPIOX>::ptr() };
                    let mask: u32 = 1 << self.index;
                    riscv::interrupt::free(|| {
                        let val: u32 = reg.out().read().bits() | mask;
                        unsafe {
                            reg.out().write(|w| w.bits(val));
                        }
                    });
                    Ok(())
                }
            }

            impl $crate::hal::digital::v2::StatefulOutputPin for $GPIOX {
                fn is_set_low(&self) -> Result<bool, Self::Error> {
                    let reg = unsafe { &*<$PACGPIOX>::ptr() };
                    let mask: u32 = 1 << self.index;
                    let val: u32 = reg.out().read().bits() & mask;
                    Ok(val == 0)
                }
                fn is_set_high(&self) -> Result<bool, Self::Error> {
                    let reg = unsafe { &*<$PACGPIOX>::ptr() };
                    let mask: u32 = 1 << self.index;
                    let val: u32 = reg.out().read().bits() & mask;
                    Ok(val != 0)
                }
            }

            /// Opt-in to the software implementation.
            impl $crate::hal::digital::v2::toggleable::Default for $GPIOX {}
        )+
    }
}

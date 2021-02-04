use crate::dmg::busconnection::BusConnection;

pub struct Timer {
    div: u8,  // 0xFF04
    tima: u8, // 0xFF05
    tma: u8,  // 0xFF06
    tac: u8,  // 0xFF07
              // TODO: May want to make tac it's own struct but it would still need to return the unimplemented bits since
              // they're still there
              // -> bit 2 = timer enabled
              // -> bit 0..1 = Input clock select
              //    ->  00: CPU Clock / 1024 (DMG, SGB2, CGB Single Speed Mode:   4096 Hz, SGB1:   ~4194 Hz, CGB Double Speed Mode:   8192 Hz)
              //        01: CPU Clock / 16   (DMG, SGB2, CGB Single Speed Mode: 262144 Hz, SGB1: ~268400 Hz, CGB Double Speed Mode: 524288 Hz)
              //        10: CPU Clock / 64   (DMG, SGB2, CGB Single Speed Mode:  65536 Hz, SGB1:  ~67110 Hz, CGB Double Speed Mode: 131072 Hz)
              //        11: CPU Clock / 256  (DMG, SGB2, CGB Single Speed Mode:  16384 Hz, SGB1:  ~16780 Hz, CGB Double Speed Mode:  32768 Hz)
}

impl Default for Timer {
    fn default() -> Timer {
        Timer {
            div: 0,
            tima: 0,
            tma: 0,
            tac: 0,
        }
    }
}

impl BusConnection for Timer {
    fn read_byte(&self, address: u16) -> u8 {
        match address {
            0xFF04 => self.div,
            0xFF05 => self.tima,
            0xFF06 => self.tma,
            0xFF07 => self.tac,
            _ => panic!("This should never happen"),
        }
    }

    fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            0xFF04 => self.div = 0,
            0xFF05 => self.tima = value,
            0xFF06 => self.tma = value,
            0xFF07 => self.tac = value,
            _ => panic!("This should never happen"),
        }
    }
}

impl Timer {
    pub fn step(&mut self, cycles: u8) {
        // TODO: Implement the step function
        // self.div += self.div.wrapping_add(1);
        // let mut timer_clocksum: u32 = 0;
        // if self.timer_enabled() {
        //     timer_clocksum += cycles as u32;

        //     let freq = match self.tac & 3 {
        //         0 => 4096,
        //         1 => 262144,
        //         2 => 65536,
        //         3 => 16384,
        //         _ => panic!("We've defied a law of mathematics!!")
        //     };

        //     while timer_clocksum >= (4194304 / freq) {
        //         self.tima = self.tima.wrapping_add(1);
        //         if self.tima == 0 {
        //             // TODO: Write to interrupt flags that we've overflowed at 0xFF0F
        //         }
        //     }
        // }
    }

    fn timer_enabled(&self) -> bool {
        // The second bit of the tac tells is of the timer is enabled
        ((self.tac >> 2) & 0x1) == 1
    }

    #[cfg(test)]
    pub fn div(&self) -> u8 {
        self.div
    }

    #[cfg(test)]
    pub fn tima(&self) -> u8 {
        self.tima
    }
}
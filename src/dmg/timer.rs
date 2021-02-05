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
    timer_counter: u16,
}

impl Default for Timer {
    fn default() -> Timer {
        Timer {
            div: 0,
            tima: 0,
            tma: 0,
            tac: 0,
            timer_counter: 1024, // TAC starts at zero so the timer_counter should begin accordingly
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
            0xFF07 => {
                let curr_freq = self.cpu_cycles_per_tick();
                self.tac = value;
                let new_freq = self.cpu_cycles_per_tick();
                if curr_freq != new_freq {
                    self.timer_counter = self.cpu_cycles_per_tick();
                }
            },
            _ => panic!("This should never happen"),
        }
    }
}

impl Timer {
    pub fn step(&mut self, cycles: u8) {
        let (new_div, did_overflow) = self.div.overflowing_add(cycles);
        if did_overflow {
            self.div = 0;
        } else {
            self.div = new_div;
        }

        if self.timer_enabled() {
            let (new_tc, tc_overflow) = self.timer_counter.overflowing_sub(cycles as u16);
            if tc_overflow {

                // Reset the timer
                self.timer_counter = self.cpu_cycles_per_tick();

                // timer about to overflow
                if self.tima == 255 {
                    self.tima = self.tma;
                    // TODO: Request an interrupt
                } else {
                    self.tima += 1;
                }
            } else {
                self.timer_counter = new_tc;
            }
        }
    }

    fn timer_enabled(&self) -> bool {
        // The second bit of the tac tells is of the timer is enabled
        ((self.tac >> 2) & 0x1) == 1
    }

    // Gets the frequency for the timer from tac and returns the precalculated amount of cylces per cpu tick
    fn cpu_cycles_per_tick(&self) -> u16 {
        match self.tac & 3 {
            0 => 1024, // 00: 4096 Hz, 4194304 cycles per second
            1 => 16,   // 01: 262144 Hz, 4194304 cycles per second
            2 => 64,   // 10: 65536 Hz, 4194304 cycles per second
            3 => 256,  // 11: 16384 Hz, 4194304 cycles per second
            _ => panic!("We've defied a law of mathematics!!")
        }
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

#[test]
fn timer_write_to_div() {
    let mut t = Timer::default();
    t.write_byte(0xFF04, 10);
    assert_eq!(t.div, 0, "The div should always be set to zero if we write to it");
}

#[test]
fn timer_counter_test_initial() {
    let t = Timer::default();
    assert_eq!(t.cpu_cycles_per_tick(), 1024, "Timer cpu cylces should start at 1024");
    assert_eq!(t.timer_counter, 1024, "The timer counter should begin at 1024");
}

#[test]
fn timer_counter_partial_step() {
    let mut t = Timer::default();
    t.tac = 5;
    t.timer_counter = t.cpu_cycles_per_tick();
    assert_eq!(t.timer_enabled(), true);
    assert_eq!(t.cpu_cycles_per_tick(), 16, "Timer cpu cylces should start at 16");
    t.step(10);
    assert_eq!(t.timer_counter, 6, "We didn't make a full clock tick");
}

#[test]
fn timer_should_inc_once() {
    let mut t = Timer::default();
    t.tac = 5;
    t.timer_counter = t.cpu_cycles_per_tick();
    assert_eq!(t.timer_enabled(), true);
    assert_eq!(t.cpu_cycles_per_tick(), 16, "Timer cpu cylces should start at 16");
    t.step(20);
    assert_eq!(t.timer_counter, 16, "Timer should have reset to 16 according to the TAC");
}

#[test]
fn timer_disabled_doesnt_move() {
    let mut t = Timer::default();
    assert_eq!(t.timer_enabled(), false, "Timer begins as disables");
    assert_eq!(t.timer_counter, 1024, "Timer cpu cylces should start at 16");
    t.step(10);
    assert_eq!(t.timer_counter, 1024, "Timer should stil be at the starting point");
}

#[test]
fn changing_frequency_changes_timer() {
    let mut t = Timer::default();
    assert_eq!(t.timer_counter, 1024);
    t.write_byte(0xFF07, 0b100);
    assert_eq!(t.timer_enabled(), true, "TAC at one should enable our timer");
    assert_eq!(t.timer_counter, 1024, "TAC has changed but hasn't change the freqency, so this should still be 1024");
    t.step(10);
    assert_eq!(t.timer_counter, 1014, "A partial step should result in the timer counter moving");
    t.write_byte(0xFF07, 0b101);
    assert_eq!(t.timer_counter, 16, "The timer should be reset to a new frequency");
}

// TODO: Test for triggering interrupt
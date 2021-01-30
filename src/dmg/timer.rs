pub struct Timer {
    div: u8,  // 0xFF04
    tima: u8, // 0xFF05
    tma: u8,  // 0xFF06
    tac: u8,  // 0xFF07
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

impl Timer {
    fn step() {
        // TODO: Will need a step function that may
        // need to take cyles
        unimplemented!();
    }

    pub fn read(&self, address: u16) -> u8 {
        match address {
            0xFF04 => self.div,
            0xFF05 => self.tima,
            0xFF06 => self.tma,
            0xFF07 => self.tac,
            _ => panic!("This should never happen"),
        }
    }

    pub fn write(&mut self, address: u16, value: u8) {
        match address {
            0xFF04 => self.div = 0,
            0xFF05 => self.tima = value,
            0xFF06 => self.tma = value,
            0xFF07 => self.tac = value,
            _ => panic!("This should never happen"),
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

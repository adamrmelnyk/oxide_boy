pub trait BusConnection {
    /// Returns a byte at a given address
    fn read_byte(&self, address: u16) -> u8;

    /// Writes a byte to a given address
    fn write_byte(&mut self, address:u16, value: u8);

    /// Reads the word from the address
    /// Note that we are using little-endian
    fn read_word(&self, address: u16) -> u16 {
        let l_byte = self.read_byte(address);
        let h_byte = self.read_byte(address + 1);
        ((h_byte as u16) << 8) | l_byte as u16
    }

    /// Writes the word from the address
    /// This uses little endian
    fn write_word(&mut self, address: u16, value: u16) {
        let h_byte = (value >> 8) as u8;
        let l_byte = value as u8;
        self.write_byte(address, l_byte);
        self.write_byte(address + 1, h_byte);
    }
}
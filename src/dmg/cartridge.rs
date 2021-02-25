use crate::dmg::busconnection::BusConnection;
use crate::dmg::rom_only::RomOnly;
use crate::dmg::mbc1::MBC1;

use std::fs::File;
use std::io::Read;

const DEFAULT_ROM: &str = "src/dmg/rom/DEFAULT_ROM.bin";

pub struct Cartridge {
    cart: Box<dyn BusConnection>,
}

// complete list of cartridges taken from here: https://gbdev.io/pandocs/#_0147-cartridge-type
#[derive(Debug, PartialEq)]
enum Type {
    RomOnly,
    MBC1,
    Mbc1Ram,
    Mbc1RamBattery,
    MBC2,
    Mbc2Battery,
    RomRam,
    RomRamBattery,
    MMM01,
    Mmm01Ram,
    Mmm01RamBattery,
    Mbc3TimerBattery,
    Mbc3TimerRamBattery,
    MBC3,
    Mbc3Ram,
    Mbc3RamBattery,
    MBC5,
    Mbc5Ram,
    Mbc5RamBattery,
    Mbc5Rumble,
    Mbc5RumbleRam,
    Mbc5RumbleRamBattery,
    MBC6,
    Mbc7SensorRumbleRamBattery,
    PocketCamera,
    BandaiTama5,
    HuC3,
    HuC1RamBattery,
}

impl std::convert::From<u8> for Type {
    fn from(byte: u8) -> Type {
        match byte {
            0x00 => Type::RomOnly,
            0x01 => Type::MBC1,
            0x02 => Type::Mbc1Ram,
            0x03 => Type::Mbc1RamBattery,
            0x05 => Type::MBC2,
            0x06 => Type::Mbc2Battery,
            0x08 => Type::RomRam,
            0x09 => Type::RomRamBattery,
            0x0B => Type::MMM01,
            0x0C => Type::Mmm01Ram,
            0x0D => Type::Mmm01RamBattery,
            0x0F => Type::Mbc3TimerBattery,
            0x10 => Type::Mbc3TimerRamBattery,
            0x11 => Type::MBC3,
            0x12 => Type::Mbc3Ram,
            0x13 => Type::Mbc3RamBattery,
            0x19 => Type::MBC5,
            0x1A => Type::Mbc5Ram,
            0x1B => Type::Mbc5RamBattery,
            0x1C => Type::Mbc5Rumble,
            0x1D => Type::Mbc5RumbleRam,
            0x1E => Type::Mbc5RumbleRamBattery,
            0x20 => Type::MBC6,
            0x22 => Type::Mbc7SensorRumbleRamBattery,
            0xFC => Type::PocketCamera,
            0xFD => Type::BandaiTama5,
            0xFE => Type::HuC3,
            0xFF => Type::HuC1RamBattery,
            _ => panic!("{:#02x} is not a value type of cartridge", byte)
        }
    }
}

impl Default for Cartridge {
    fn default() -> Cartridge {
        Cartridge::new(DEFAULT_ROM)
    }
}

impl Cartridge {
    pub fn new(file_name: &str) -> Cartridge {
        let mut data = vec!();
        match File::open(file_name) {
            Ok(mut file) => match file.read_to_end(&mut data) {
                Ok(_size) => {},
                Err(err) => eprintln!("Error reading file: {}", err),
            },
            Err(err) => {
                eprintln!("Error opening file: {}, defaulting to empty RomOnly Cartridge", err);
                return Cartridge {
                    cart: Box::new(RomOnly::new(vec![0u8; 0xC000])), // This is mainly so that tests may run without a cartridge
                }
            }
        }
        // TODO: We could potentially throw an error if the cart is too small
        let cart_type = Type::from(data[0x147]);
        let cart = cart(&cart_type, data);
        Cartridge {
            cart,
        }
    }
}

// Limiting the cartridge types that are implemented
fn cart(cart_type: &Type, data: Vec<u8>) -> Box<dyn BusConnection> {
    match cart_type {
        Type::RomOnly => Box::new(RomOnly::new(data)),
        Type::MBC1 => Box::new(MBC1::new(data)),
        _ => panic!("The type: {:?}, is not implemented", cart_type),
    }
}

impl BusConnection for Cartridge {
    fn read_byte(&self, address: u16) -> u8 {
        self.cart.read_byte(address)
    }

    fn write_byte(&mut self, address: u16, value: u8) {
        self.cart.write_byte(address, value);
    }
}

#[test]
fn convert_byte_to_type() {
    assert_eq!(Type::from(0x00), Type::RomOnly);
    assert_eq!(Type::from(0x01), Type::MBC1);
}
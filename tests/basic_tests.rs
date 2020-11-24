use gb_emulator::CPU;

pub fn setup() -> CPU {
    CPU::default()
}

#[test]
fn basic_test() {
    let cpu = setup();
}

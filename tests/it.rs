use bog::{Cpu, Status};
use nes::{Cartridge, NesBus};

const NESTEST_LOG: &str = include_str!("../roms/nestest.log");
const NESTEST_ROM: &[u8] = include_bytes!("../roms/nestest.nes");

struct CpuState {
    pc: u16,
    a: u8,
    x: u8,
    y: u8,
    p: u8,
    s: u8,
    cycles: u64,
}

impl CpuState {
    fn from_log_line(line: &str) -> CpuState {
        // TODO: Store the expected values in table so that we don't have to
        // parse them every time.
        let pc = u16::from_str_radix(&line[0..4], 16).unwrap();
        let mut registers = line[48..].split_ascii_whitespace();
        let a =
            u8::from_str_radix(&registers.next().unwrap()[2..], 16).unwrap();
        let x =
            u8::from_str_radix(&registers.next().unwrap()[2..], 16).unwrap();
        let y =
            u8::from_str_radix(&registers.next().unwrap()[2..], 16).unwrap();
        let p =
            u8::from_str_radix(&registers.next().unwrap()[2..], 16).unwrap();
        let s =
            u8::from_str_radix(&registers.next().unwrap()[3..], 16).unwrap();
        let cycles = registers.next().unwrap()[4..].parse::<u64>().unwrap();

        CpuState {
            pc,
            a,
            x,
            y,
            p,
            s,
            cycles,
        }
    }
}

#[test]
fn nestest() {
    let cartridge = Cartridge::new(NESTEST_ROM);
    let bus = NesBus::new(cartridge);
    let mut cpu = Cpu::new(bus);

    cpu.pc = 0xc000;
    // The nestest log has a different initial value for the status register.
    cpu.p = Status::from_bits(0x24).unwrap();
    // Pretend that the CPU already went through the reset sequence.
    cpu.cycles = 7;

    for line in NESTEST_LOG.lines() {
        let expected = CpuState::from_log_line(line);
        assert_eq!(cpu.pc, expected.pc);
        assert_eq!(cpu.a, expected.a);
        assert_eq!(cpu.x, expected.x);
        assert_eq!(cpu.y, expected.y);
        assert_eq!(cpu.p.bits(), expected.p);
        assert_eq!(cpu.s, expected.s);
        assert_eq!(cpu.cycles, expected.cycles);

        cpu.step();
    }
}

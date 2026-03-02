use super::*;

mod lda {
    use super::*;

    #[test]
    fn immediate_load_data() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x05, 0x00]);
        assert_eq!(cpu.register_a, 0x05);
        assert!(cpu.status & 0b0000_0010 == 0b00);
        assert!(cpu.status & 0b1000_0000 == 0);
    }

    #[test]
    fn zero_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x00, 0x00]);
        assert!(cpu.status & 0b0000_0010 == 0b10);
    }
}

mod tax {
    use super::*;

    #[test]
    fn move_a_to_x() {
        let mut cpu = CPU::new();
        cpu.load(vec![0xaa, 0x00]);
        cpu.reset();
        cpu.register_a = 10;
        cpu.run();
        assert_eq!(cpu.register_x, 10);
    }
}

mod inx {
    use super::*;

    #[test]
    fn overflow() {
        let mut cpu = CPU::new();
        cpu.load(vec![0xe8, 0xe8, 0x00]);
        cpu.reset();
        cpu.register_x = 0xff;
        cpu.run();
        assert_eq!(cpu.register_x, 1);
    }
}

mod sta {
    use super::*;

    #[test]
    fn zero_page() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x42, 0x85, 0x10, 0x00]);
        assert_eq!(cpu.mem_read(0x10), 0x42);
    }
}

mod integration {
    use super::*;

    #[test]
    fn five_ops_working_together() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00]);
        assert_eq!(cpu.register_x, 0xc1);
    }
}

mod adc {
    use super::*;

    #[test]
    fn basic() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 10, 0x69, 5, 0x00]);
        assert_eq!(cpu.register_a, 15);
    }

    #[test]
    fn carry_set() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 200, 0x69, 100, 0x00]);
        assert_eq!(cpu.register_a, 44);
        assert!(cpu.status & 0b0000_0001 != 0);
    }

    #[test]
    fn zero_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0, 0x69, 0, 0x00]);
        assert!(cpu.status & 0b0000_0010 != 0);
    }

    #[test]
    fn negative_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 200, 0x69, 10, 0x00]);
        assert!(cpu.status & 0b1000_0000 != 0);
    }

    #[test]
    fn overflow_positive() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 127, 0x69, 1, 0x00]);
        assert!(cpu.status & 0b0100_0000 != 0);
    }

    #[test]
    fn no_overflow() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 50, 0x69, 30, 0x00]);
        assert!(cpu.status & 0b0100_0000 == 0);
    }

    #[test]
    fn with_carry() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 200, 0x69, 100, 0xa9, 10, 0x69, 5, 0x00]);
        assert_eq!(cpu.register_a, 16);
    }

    #[test]
    fn overflow_negative() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0xB0, 0x69, 0xA6, 0x00]);
        assert!(cpu.status & 0b0100_0000 != 0);
    }

    #[test]
    fn carry_clears() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 200, 0x69, 100, 0xa9, 10, 0x69, 5, 0x00]);
        assert!(cpu.status & 0b0000_0001 == 0);
    }

    #[test]
    fn zero_page() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 42, 0x85, 0x10, 0xa9, 10, 0x65, 0x10, 0x00]);
        assert_eq!(cpu.register_a, 52);
    }
}
    mod sbc {
    use super::*;

    // 10 - 5 = 5 (carry ustawione przez poprzednie 0xFF + 0x01)
    #[test]
    fn basic() {
        let mut cpu = CPU::new();
        // 0xFF + 0x01 → carry = 1, potem LDA 10, SBC 5
        cpu.load_and_run(vec![0xa9, 0xFF, 0x69, 0x01, 0xa9, 10, 0xE9, 5, 0x00]);
        assert_eq!(cpu.register_a, 5);
    }

    // 3 - 10 = -7 (carry = 1 przed SBC)
    #[test]
    fn negative_result() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0xFF, 0x69, 0x01, 0xa9, 3, 0xE9, 10, 0x00]);
        assert!(cpu.status & 0b1000_0000 != 0); // negative = 1
    }

    // 5 - 5 = 0
    #[test]
    fn zero_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0xFF, 0x69, 0x01, 0xa9, 5, 0xE9, 5, 0x00]);
        assert!(cpu.status & 0b0000_0010 != 0); // zero = 1
    }

    // 3 - 10 → underflow → carry = 0
    #[test]
    fn carry_clear_on_underflow() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0xFF, 0x69, 0x01, 0xa9, 3, 0xE9, 10, 0x00]);
        assert!(cpu.status & 0b0000_0001 == 0); // carry = 0
    }

    // 10 - 5 → no underflow → carry = 1
    #[test]
    fn carry_set_no_underflow() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0xFF, 0x69, 0x01, 0xa9, 10, 0xE9, 5, 0x00]);
        assert!(cpu.status & 0b0000_0001 != 0); // carry = 1
    }

    // overflow: 127 - (-1) = 128 → plus - minus = minus → overflow
    #[test]
    fn overflow() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0xFF, 0x69, 0x01, 0xa9, 127, 0xE9, 0xFF, 0x00]);
        assert!(cpu.status & 0b0100_0000 != 0); // overflow = 1
    }

    // brak overflow: 50 - 30 = 20
    #[test]
    fn no_overflow() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0xFF, 0x69, 0x01, 0xa9, 50, 0xE9, 30, 0x00]);
        assert!(cpu.status & 0b0100_0000 == 0); // overflow = 0
    }

    // SBC bez carry (carry = 0): 10 - 5 - 1 = 4
    #[test]
    fn without_carry() {
        let mut cpu = CPU::new();
        // nie ustawiamy carry, więc SBC odejmie dodatkowe 1
        cpu.load_and_run(vec![0xa9, 10, 0xE9, 5, 0x00]);
        assert_eq!(cpu.register_a, 4);
    }

    // SBC ZeroPage
    #[test]
    fn zero_page() {
        let mut cpu = CPU::new();
        // ustaw carry, zapisz 42 pod 0x10, LDA 50, SBC z 0x10
        cpu.load_and_run(vec![
            0xa9, 0xFF, 0x69, 0x01,  // carry = 1
            0xa9, 42, 0x85, 0x10,     // STA 0x10
            0xa9, 50, 0xE5, 0x10,     // LDA 50, SBC ZeroPage 0x10
            0x00
        ]);
        assert_eq!(cpu.register_a, 8); // 50 - 42 = 8
    }
}

    mod asl {
    use super::*;

    #[test]
    fn accumulator_shift() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0b0000_0011, 0x0A, 0x00]); // LDA #3, ASL A
        assert_eq!(cpu.register_a, 0b0000_0110);
    }

    #[test]
    fn accumulator_carry_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0b1000_0001, 0x0A, 0x00]); // LDA #129, ASL A
        assert_eq!(cpu.register_a, 0b0000_0010);
        assert!(cpu.status & 0b0000_0001 == 0b0000_0001); // carry ustawiony
    }

    #[test]
    fn accumulator_zero_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0b1000_0000, 0x0A, 0x00]); // LDA #128, ASL A
        assert_eq!(cpu.register_a, 0x00);
        assert!(cpu.status & 0b0000_0010 == 0b0000_0010); // zero ustawiony
        assert!(cpu.status & 0b0000_0001 == 0b0000_0001); // carry ustawiony
    }

    #[test]
    fn accumulator_negative_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0b0100_0000, 0x0A, 0x00]); // LDA #64, ASL A
        assert_eq!(cpu.register_a, 0b1000_0000);
        assert!(cpu.status & 0b1000_0000 == 0b1000_0000); // negative ustawiony
    }

    #[test]
    fn zero_page_shift() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x10, 0b0000_0011);
        cpu.load_and_run(vec![0x06, 0x10, 0x00]); // ASL $10
        assert_eq!(cpu.mem_read(0x10), 0b0000_0110);
    }

    #[test]
    fn zero_page_carry_flag() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x10, 0b1000_0001);
        cpu.load_and_run(vec![0x06, 0x10, 0x00]); // ASL $10
        assert_eq!(cpu.mem_read(0x10), 0b0000_0010);
        assert!(cpu.status & 0b0000_0001 == 0b0000_0001); // carry ustawiony
    }
}
mod lsr {
    use super::*;

    #[test]
    fn accumulator_shift() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0b0000_1100, 0x4A, 0x00]); // LDA #12, LSR A
        assert_eq!(cpu.register_a, 0b0000_0110);
    }

    #[test]
    fn accumulator_carry_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0b0000_0011, 0x4A, 0x00]); // LDA #3, LSR A
        assert_eq!(cpu.register_a, 0b0000_0001);
        assert!(cpu.status & 0b0000_0001 == 0b0000_0001); // carry ustawiony
    }

    #[test]
    fn accumulator_no_carry_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0b0000_0100, 0x4A, 0x00]); // LDA #4, LSR A
        assert_eq!(cpu.register_a, 0b0000_0010);
        assert!(cpu.status & 0b0000_0001 == 0); // carry wyczyszczony
    }

    #[test]
    fn accumulator_zero_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0b0000_0001, 0x4A, 0x00]); // LDA #1, LSR A
        assert_eq!(cpu.register_a, 0x00);
        assert!(cpu.status & 0b0000_0010 == 0b0000_0010); // zero ustawiony
        assert!(cpu.status & 0b0000_0001 == 0b0000_0001); // carry ustawiony
    }

    #[test]
    fn accumulator_negative_flag_never_set() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0b1111_1110, 0x4A, 0x00]); // LDA #254, LSR A
        assert_eq!(cpu.register_a, 0b0111_1111);
        assert!(cpu.status & 0b1000_0000 == 0); // negative nigdy nie ustawiony po LSR
    }

    #[test]
    fn zero_page_shift() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x10, 0b0000_1100);
        cpu.load_and_run(vec![0x46, 0x10, 0x00]); // LSR $10
        assert_eq!(cpu.mem_read(0x10), 0b0000_0110);
    }

    #[test]
    fn zero_page_carry_flag() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x10, 0b0000_0011);
        cpu.load_and_run(vec![0x46, 0x10, 0x00]); // LSR $10
        assert_eq!(cpu.mem_read(0x10), 0b0000_0001);
        assert!(cpu.status & 0b0000_0001 == 0b0000_0001); // carry ustawiony
    }
}


mod rol {
    use super::*;

    #[test]
    fn accumulator_shift() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0b0000_0011, 0x2A, 0x00]); // LDA #3, ROL A
        assert_eq!(cpu.register_a, 0b0000_0110);
    }

    #[test]
    fn accumulator_carry_in() {
        let mut cpu = CPU::new();
        // ustawiamy carry ręcznie przed ROL
        cpu.load(vec![0xa9, 0b0000_0011, 0x2A, 0x00]);
        cpu.reset();
        cpu.status |= 0b0000_0001; // carry ustawiony
        cpu.run();
        assert_eq!(cpu.register_a, 0b0000_0111); // bit 0 ustawiony przez carry
    }

    #[test]
    fn accumulator_carry_out() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0b1000_0001, 0x2A, 0x00]); // LDA #129, ROL A
        assert_eq!(cpu.register_a, 0b0000_0010);
        assert!(cpu.status & 0b0000_0001 == 0b0000_0001); // carry ustawiony z bitu 7
    }

    #[test]
    fn accumulator_carry_in_and_out() {
        let mut cpu = CPU::new();
        cpu.load(vec![0xa9, 0b1000_0001, 0x2A, 0x00]);
        cpu.reset();
        cpu.status |= 0b0000_0001; // carry ustawiony
        cpu.run();
        assert_eq!(cpu.register_a, 0b0000_0011); // bit 0 z carry, bit 7 wypadł do carry
        assert!(cpu.status & 0b0000_0001 == 0b0000_0001);
    }

    #[test]
    fn accumulator_zero_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0b0000_0000, 0x2A, 0x00]);
        assert_eq!(cpu.register_a, 0x00);
        assert!(cpu.status & 0b0000_0010 == 0b0000_0010); // zero ustawiony
    }

    #[test]
    fn zero_page_shift() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x10, 0b0000_0011);
        cpu.load_and_run(vec![0x26, 0x10, 0x00]); // ROL $10
        assert_eq!(cpu.mem_read(0x10), 0b0000_0110);
    }
}

mod ror {
    use super::*;

    #[test]
    fn accumulator_shift() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0b0000_1100, 0x6A, 0x00]); // LDA #12, ROR A
        assert_eq!(cpu.register_a, 0b0000_0110);
    }

    #[test]
    fn accumulator_carry_in() {
        let mut cpu = CPU::new();
        cpu.load(vec![0xa9, 0b0000_0110, 0x6A, 0x00]);
        cpu.reset();
        cpu.status |= 0b0000_0001; // carry ustawiony
        cpu.run();
        assert_eq!(cpu.register_a, 0b1000_0011); // bit 7 ustawiony przez carry
    }

    #[test]
    fn accumulator_carry_out() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0b0000_0011, 0x6A, 0x00]); // LDA #3, ROR A
        assert_eq!(cpu.register_a, 0b0000_0001);
        assert!(cpu.status & 0b0000_0001 == 0b0000_0001); // carry ustawiony z bitu 0
    }

    #[test]
    fn accumulator_carry_in_and_out() {
        let mut cpu = CPU::new();
        cpu.load(vec![0xa9, 0b0000_0011, 0x6A, 0x00]);
        cpu.reset();
        cpu.status |= 0b0000_0001; // carry ustawiony
        cpu.run();
        assert_eq!(cpu.register_a, 0b1000_0001); // bit 7 z carry, bit 0 wypadł do carry
        assert!(cpu.status & 0b0000_0001 == 0b0000_0001);
    }

    #[test]
    fn accumulator_negative_flag() {
        let mut cpu = CPU::new();
        cpu.load(vec![0xa9, 0b0000_0000, 0x6A, 0x00]);
        cpu.reset();
        cpu.status |= 0b0000_0001; // carry ustawiony
        cpu.run();
        assert_eq!(cpu.register_a, 0b1000_0000);
        assert!(cpu.status & 0b1000_0000 == 0b1000_0000); // negative ustawiony
    }

    #[test]
    fn zero_page_shift() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x10, 0b0000_1100);
        cpu.load_and_run(vec![0x66, 0x10, 0x00]); // ROR $10
        assert_eq!(cpu.mem_read(0x10), 0b0000_0110);
    }
}



mod and {
    use super::*;

    #[test]
    fn immediate_and() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0b1111_1111, 0x29, 0b0000_1111, 0x00]);
        assert_eq!(cpu.register_a, 0b0000_1111);
    }

    #[test]
    fn immediate_zero_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0b1111_0000, 0x29, 0b0000_1111, 0x00]);
        assert_eq!(cpu.register_a, 0x00);
        assert!(cpu.status & 0b0000_0010 == 0b0000_0010);
    }

    #[test]
    fn immediate_negative_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0b1111_1111, 0x29, 0b1000_0000, 0x00]);
        assert_eq!(cpu.register_a, 0b1000_0000);
        assert!(cpu.status & 0b1000_0000 == 0b1000_0000);
    }

    #[test]
    fn zero_page_and() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x10, 0b0000_1111);
        cpu.load_and_run(vec![0xa9, 0b1111_1111, 0x25, 0x10, 0x00]);
        assert_eq!(cpu.register_a, 0b0000_1111);
    }
}


mod ora {
    use super::*;

    #[test]
    fn immediate_ora() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0b1111_0000, 0x09, 0b0000_1111, 0x00]);
        assert_eq!(cpu.register_a, 0b1111_1111);
    }

    #[test]
    fn immediate_zero_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0b0000_0000, 0x09, 0b0000_0000, 0x00]);
        assert_eq!(cpu.register_a, 0x00);
        assert!(cpu.status & 0b0000_0010 == 0b0000_0010);
    }

    #[test]
    fn immediate_negative_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0b0000_0000, 0x09, 0b1000_0000, 0x00]);
        assert_eq!(cpu.register_a, 0b1000_0000);
        assert!(cpu.status & 0b1000_0000 == 0b1000_0000);
    }

    #[test]
    fn zero_page_ora() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x10, 0b0000_1111);
        cpu.load_and_run(vec![0xa9, 0b1111_0000, 0x05, 0x10, 0x00]);
        assert_eq!(cpu.register_a, 0b1111_1111);
    }
}


mod eor {
    use super::*;

    #[test]
    fn immediate_eor() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0b1111_1111, 0x49, 0b0000_1111, 0x00]);
        assert_eq!(cpu.register_a, 0b1111_0000);
    }

    #[test]
    fn immediate_zero_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0b1111_1111, 0x49, 0b1111_1111, 0x00]);
        assert_eq!(cpu.register_a, 0x00);
        assert!(cpu.status & 0b0000_0010 == 0b0000_0010);
    }

    #[test]
    fn immediate_negative_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0b0000_0000, 0x49, 0b1000_0000, 0x00]);
        assert_eq!(cpu.register_a, 0b1000_0000);
        assert!(cpu.status & 0b1000_0000 == 0b1000_0000);
    }

    #[test]
    fn immediate_toggle_bits() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0b1010_1010, 0x49, 0b1010_1010, 0x00]);
        assert_eq!(cpu.register_a, 0x00);
    }

    #[test]
    fn zero_page_eor() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x10, 0b0000_1111);
        cpu.load_and_run(vec![0xa9, 0b1111_1111, 0x45, 0x10, 0x00]);
        assert_eq!(cpu.register_a, 0b1111_0000);
    }
}


mod bit {
    use super::*;

    #[test]
    fn zero_flag_set() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x10, 0b0000_1111);
        cpu.load_and_run(vec![0xa9, 0b1111_0000, 0x24, 0x10, 0x00]);
        assert!(cpu.status & 0b0000_0010 == 0b0000_0010); // zero ustawiony
    }

    #[test]
    fn zero_flag_clear() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x10, 0b0000_1111);
        cpu.load_and_run(vec![0xa9, 0b0000_1111, 0x24, 0x10, 0x00]);
        assert!(cpu.status & 0b0000_0010 == 0); // zero wyczyszczony
    }

    #[test]
    fn negative_flag_from_memory() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x10, 0b1000_0000);
        cpu.load_and_run(vec![0xa9, 0b0000_0000, 0x24, 0x10, 0x00]);
        assert!(cpu.status & 0b1000_0000 == 0b1000_0000); // negative z bitu 7 pamięci
    }

    #[test]
    fn negative_flag_not_from_result() {
        let mut cpu = CPU::new();
        // A AND wartość = 0b1000_0000, ale bit 7 pamięci = 0
        cpu.mem_write(0x10, 0b0000_0001);
        cpu.load_and_run(vec![0xa9, 0b1111_1111, 0x24, 0x10, 0x00]);
        assert!(cpu.status & 0b1000_0000 == 0); // negative NIE ustawiony
    }

    #[test]
    fn overflow_flag_from_memory() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x10, 0b0100_0000);
        cpu.load_and_run(vec![0xa9, 0b0000_0000, 0x24, 0x10, 0x00]);
        assert!(cpu.status & 0b0100_0000 == 0b0100_0000); // overflow z bitu 6 pamięci
    }

    #[test]
    fn overflow_flag_clear() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x10, 0b0000_0001);
        cpu.load_and_run(vec![0xa9, 0b0000_0000, 0x24, 0x10, 0x00]);
        assert!(cpu.status & 0b0100_0000 == 0); // overflow wyczyszczony
    }
}

mod cmp {
    use super::*;

    #[test]
    fn negative_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 120, 0xC9, 130, 0x00]);
        assert!(cpu.status & 0b1000_0000 == 0b1000_0000); // negative
        assert!(cpu.status & 0b0000_0001 == 0); // carry wyczyszczony
        assert!(cpu.status & 0b0000_0010 == 0); // zero wyczyszczony
    }

    #[test]
    fn negative_and_carry() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 120, 0xC9, 160, 0x00]);
        assert!(cpu.status & 0b1000_0000 == 0b1000_0000); // negative
        assert!(cpu.status & 0b0000_0001 == 0); // carry wyczyszczony
    }

    #[test]
    fn zero_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 120, 0xC9, 120, 0x00]);
        assert!(cpu.status & 0b0000_0010 == 0b0000_0010); // zero
        assert!(cpu.status & 0b0000_0001 == 0b0000_0001); // carry ustawiony gdy A >= value
    }
}

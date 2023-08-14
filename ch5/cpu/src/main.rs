struct CPU {
    current_operation: u16,
    registers: [u8; 2],
}

fn main() {
    let mut cpu = CPU {
        current_operation: 0,
        registers: [0; 2],
    };

    cpu.current_operation = 0x8014; // 8: Operation, 0: R0, 1: R1, 4: ADD
    cpu.registers[0] = 5;
    cpu.registers[1] = 10;
}

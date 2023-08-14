fn mock_rand(n: u8) -> f32 {
    let base: u32 = 0b0_01111110_00000000000000000000000;
    let large_n = (n as u32) << 15;
    let f32_bits = base | large_n;
    let m = f32::from_bits(f32_bits); // u32 -> f32로 변환
    2.0 * (m - 0.5)
}

fn main() {
    println!("max of input range: {:08b} -> {:?}", )
}

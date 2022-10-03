const BIAS: i32 = 127;
const RADIX: f32 = 2.0;

fn main() {
    let n: f32 = 42.42;

    let (sign, exp, frac) = to_parts(n);
    let (sign_, exp_, mant) = decode(sign, exp, frac);
    let n_ = from_parts(sign_, exp_, mant);

    println!("{} -> {}", n, n_);
    println!("field    | as bits | as real number");
    println!("sign     | {:01b}  | {}", sign, sign_);
    println!("exponent | {:08b}  | {}", exp, exp_);
    println!("mantissa | {:023b} | {}", frac, mant);
}

fn to_parts(n: f32) -> (u32, u32, u32) {
    let bits = n.to_bits();

    let sign = (bits >> 31) & 1; // 부호 비트만 남기기
    let exponent = (bits >> 23) & 0xFF; // 최하위 비트 23개만 남기기(지수부)
    let fraction = bits & 0x7FFFFF; // 가수부(Decoding 전)

    (sign, exponent, fraction)
}

fn decode(sign: u32, exp: u32, frac: u32) -> (f32, f32, f32) {
    let sign_ = (-1.0_f32).powf(sign as f32); // 메서드 호출(powf 호출임)이 단항 마이너스 연산자보다 우선순위가 높아 괄호로 감싸야 한다
    let exp_ = (exp as i32) - BIAS; // 음수가 나올 수 있으므로 i32로 변환
    let exp_ = RADIX.powf(exp_ as f32);
    let mut mant: f32 = 1.0;

    // 지수부 Decoding
    for i in 0..23 {
        let mask = 1 << i;
        let one_at_bit_i = frac & mask;
        if one_at_bit_i != 0 {
            let i_ = i as f32;
            let weight = 2_f32.powf(i_ - 23.0);
            mant += weight;
        }
    }

    (sign_, exp_, mant)
}

fn from_parts(sign: f32, exp: f32, mant: f32) -> f32 {
    sign * exp * mant
}

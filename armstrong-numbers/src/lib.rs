pub fn is_armstrong_number(num: u32) -> bool {
    let mut digits: Vec<u32> = Vec::new();

    let mut next = num;
    while next / 10 > 0 {
        digits.push(next % 10);
        next /= 10;
    }

    digits.push(next);

    let power = digits.len() as u32;
    let mut value: u32 = 0;
    for digit in digits {
        value += u32::pow(digit, power);
    }

    return value == num;
}

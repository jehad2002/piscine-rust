#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RomanDigit {
    Nulla,
    I,
    V,
    X,
    L,
    C,
    D,
    M,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RomanNumber(pub Vec<RomanDigit>);

impl From<u32> for RomanDigit {
    fn from(num: u32) -> Self {
        match num {
            0 => RomanDigit::Nulla,
            1 => RomanDigit::I,
            5 => RomanDigit::V,
            10 => RomanDigit::X,
            50 => RomanDigit::L,
            100 => RomanDigit::C,
            500 => RomanDigit::D,
            1000 => RomanDigit::M,
            _ => RomanDigit::Nulla, // نرجع Nulla كحالة افتراضية
        }
    }
}

impl From<u32> for RomanNumber {
    fn from(mut num: u32) -> Self {
        let mut result = RomanNumber(Vec::new());
        if num == 0 {
            result.0.push(RomanDigit::Nulla);
            return result;
        }
        while num > 0 {
            if num >= 1000 {
                result.0.push(RomanDigit::M);
                num -= 1000;
            } else if num >= 900 {
                result.0.push(RomanDigit::C);
                result.0.push(RomanDigit::M);
                num -= 900;
            } else if num >= 500 {
                result.0.push(RomanDigit::D);
                num -= 500;
            } else if num >= 400 {
                result.0.push(RomanDigit::C);
                result.0.push(RomanDigit::D);
                num -= 400;
            } else if num >= 100 {
                result.0.push(RomanDigit::C);
                num -= 100;
            } else if num >= 90 {
                result.0.push(RomanDigit::X);
                result.0.push(RomanDigit::C);
                num -= 90;
            } else if num >= 50 {
                result.0.push(RomanDigit::L);
                num -= 50;
            } else if num >= 40 {
                result.0.push(RomanDigit::X);
                result.0.push(RomanDigit::L);
                num -= 40;
            } else if num >= 10 {
                result.0.push(RomanDigit::X);
                num -= 10;
            } else if num >= 9 {
                result.0.push(RomanDigit::I);
                result.0.push(RomanDigit::X);
                num -= 9;
            } else if num >= 5 {
                result.0.push(RomanDigit::V);
                num -= 5;
            } else if num >= 4 {
                result.0.push(RomanDigit::I);
                result.0.push(RomanDigit::V);
                num -= 4;
            } else {
                result.0.push(RomanDigit::I);
                num -= 1;
            }
        }
        result
    }
}

fn digit_value(d: RomanDigit) -> u32 {
    match d {
        RomanDigit::Nulla => 0,
        RomanDigit::I => 1,
        RomanDigit::V => 5,
        RomanDigit::X => 10,
        RomanDigit::L => 50,
        RomanDigit::C => 100,
        RomanDigit::D => 500,
        RomanDigit::M => 1000,
    }
}

impl From<RomanNumber> for u32 {
    fn from(num: RomanNumber) -> Self {
        let mut result = 0;
        let mut prev_value = 0;
        for digit in num.0.iter().rev() {
            let value = digit_value(*digit);
            if value < prev_value {
                result -= value;
            } else {
                result += value;
            }
            prev_value = value;
        }
        result
    }
}

impl Iterator for RomanNumber {
    type Item = RomanNumber;

    fn next(&mut self) -> Option<Self::Item> {
        let value: u32 = self.clone().into();
        if value == 0 {
            return None;
        }
        let next_value = value + 1;
        Some(RomanNumber::from(next_value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roman_digit_from_u32() {
        assert_eq!(RomanDigit::from(1), RomanDigit::I);
        assert_eq!(RomanDigit::from(5), RomanDigit::V);
        assert_eq!(RomanDigit::from(100), RomanDigit::C);
        assert_eq!(RomanDigit::from(9999), RomanDigit::Nulla);
    }

    #[test]
    fn test_from_u32_to_roman_number() {
        let num = RomanNumber::from(1987);
        let expected = RomanNumber(vec![
            RomanDigit::M,
            RomanDigit::C,
            RomanDigit::M,
            RomanDigit::X,
            RomanDigit::L,
            RomanDigit::X,
            RomanDigit::V,
            RomanDigit::I,
            RomanDigit::I,
        ]);
        assert_eq!(num, expected);
    }

    #[test]
    fn test_from_roman_number_to_u32() {
        let rn = RomanNumber(vec![RomanDigit::M, RomanDigit::D, RomanDigit::C, RomanDigit::C, RomanDigit::X, RomanDigit::L, RomanDigit::I, RomanDigit::V]);
        assert_eq!(u32::from(rn), 1444);
    }

    #[test]
    fn test_iterator() {
        let mut rn = RomanNumber::from(1);
        assert_eq!(rn.next(), Some(RomanNumber::from(2)));
        assert_eq!(rn.next(), Some(RomanNumber::from(3)));
    }
}

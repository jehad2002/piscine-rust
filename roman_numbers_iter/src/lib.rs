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
            _ => RomanDigit::Nulla,
        }
    }
}

impl From<u32> for RomanNumber {
    fn from(mut num: u32) -> Self {
        let mut result = RomanNumber(Vec::new());
        if num == 0 {
            result.0.push(RomanDigit::from(0));
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
            } else if num >= 1 {
                result.0.push(RomanDigit::I);
                num -= 1;
            }
        }
        result
    }
}

impl From<RomanNumber> for u32 {
    fn from(num: RomanNumber) -> Self {
        let mut result = 0;
        let mut prev_value = 0;
        for digit in num.0 {
            let value = match digit {
                RomanDigit::Nulla => 0,
                RomanDigit::I => 1,
                RomanDigit::V => 5,
                RomanDigit::X => 10,
                RomanDigit::L => 50,
                RomanDigit::C => 100,
                RomanDigit::D => 500,
                RomanDigit::M => 1000,
            };
            if value > prev_value {
                result += value - 2 * prev_value; // Subtraction case (IV, IX, etc.)
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
        let next_value = u32::from(self.clone()) + 1;
        if next_value > 3999 {
            None
        } else {
            Some(RomanNumber::from(next_value))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_u32_to_roman() {
        assert_eq!(RomanNumber::from(1), RomanNumber(vec![RomanDigit::I]));
        assert_eq!(RomanNumber::from(4), RomanNumber(vec![RomanDigit::I, RomanDigit::V]));
        assert_eq!(RomanNumber::from(9), RomanNumber(vec![RomanDigit::I, RomanDigit::X]));
        assert_eq!(RomanNumber::from(58), RomanNumber(vec![RomanDigit::L, RomanDigit::V, RomanDigit::I, RomanDigit::I, RomanDigit::I]));
        assert_eq!(RomanNumber::from(1994), RomanNumber(vec![RomanDigit::M, RomanDigit::C, RomanDigit::M, RomanDigit::X, RomanDigit::C, RomanDigit::I, RomanDigit::V]));
    }

    #[test]
    fn test_roman_to_u32() {
        assert_eq!(u32::from(RomanNumber(vec![RomanDigit::I])), 1);
        assert_eq!(u32::from(RomanNumber(vec![RomanDigit::I, RomanDigit::V])), 4);
        assert_eq!(u32::from(RomanNumber(vec![RomanDigit::I, RomanDigit::X])), 9);
        assert_eq!(u32::from(RomanNumber(vec![RomanDigit::L, RomanDigit::V, RomanDigit::I, RomanDigit::I, RomanDigit::I])), 58);
        assert_eq!(u32::from(RomanNumber(vec![RomanDigit::M, RomanDigit::C, RomanDigit::M, RomanDigit::X, RomanDigit::C, RomanDigit::I, RomanDigit::V])), 1994);
    }

    #[test]
    fn test_iterator_next() {
        let mut roman = RomanNumber::from(1);
        let next1 = roman.next().unwrap();
        let next2 = roman.next().unwrap();
        let next3 = roman.next().unwrap();
        
        assert_eq!(u32::from(next1.clone()), 2);
        assert_eq!(u32::from(next2.clone()), 3);
        assert_eq!(u32::from(next3.clone()), 4);
        
        assert_eq!(next1, RomanNumber::from(2));
        assert_eq!(next2, RomanNumber::from(3));
        assert_eq!(next3, RomanNumber::from(4));
    }
}

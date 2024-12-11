use std::{
    cell::RefCell,
    collections::{HashMap, VecDeque},
};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Stones {
    stones: Vec<Stone>,
    memo: RefCell<HashMap<(Stone, u32), u64>>,
}

impl Stones {
    pub fn new(v: &[u64]) -> Self {
        let mut memo = HashMap::with_capacity(1000);

        Self {
            stones: v.iter().copied().map(Stone::new).collect(),
            memo: RefCell::new(memo),
        }
    }

    pub fn blink(&self, blinks: u32) -> u64 {
        self.stones
            .iter()
            .map(|s| s.blink(blinks, &mut self.memo.borrow_mut()))
            .sum()
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct Stone(u64);

impl Stone {
    pub fn new(n: u64) -> Self {
        Self(n)
    }

    fn blink(self, blinks: u32, memo: &mut HashMap<(Stone, u32), u64>) -> u64 {
        if blinks == 0 {
            1
        } else if let Some(&stones) = memo.get(&(self, blinks)) {
            stones
        } else {
            let result = if self.0 == 0 {
                Stone(1).blink(blinks - 1, memo)
            } else if let Some((a, b)) = self.split() {
                a.blink(blinks - 1, memo) + b.blink(blinks - 1, memo)
            } else {
                Stone(self.0 * 2024).blink(blinks - 1, memo)
            };

            memo.insert((self, blinks), result);

            result
        }
    }

    fn blink_once(&self) -> BlinkResult {
        if self.0 == 0 {
            BlinkResult::OneStone(Stone(1))
        } else if let Some((a, b)) = self.split() {
            BlinkResult::TwoStones(a, b)
        } else {
            BlinkResult::OneStone(Stone(self.0 * 2024))
        }
    }

    fn split(&self) -> Option<(Self, Self)> {
        split_number(self.0).map(|(a, b)| (Stone::new(a), Stone::new(b)))
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum BlinkResult {
    OneStone(Stone),
    TwoStones(Stone, Stone),
}

fn split_number(n: u64) -> Option<(u64, u64)> {
    let digits = number_of_digits(n);
    if digits % 2 != 0 {
        None
    } else {
        let half_digits = digits / 2;

        let tens = 10_u64.pow(half_digits);

        let first_half = n / tens;
        let last_half = n % tens;

        Some((first_half, last_half))
    }
}

fn number_of_digits(n: u64) -> u32 {
    ((n as f64).log10().floor() as u32 + 1)
}

fn has_even_number_of_digits(n: u64) -> bool {
    number_of_digits(n) % 2 == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn number_of_digits_test() {
        for n in 0..10 {
            assert_eq!(number_of_digits(n), 1, "{}", n);
        }

        for n in 10..100 {
            assert_eq!(number_of_digits(n), 2, "{}", n);
        }

        for n in 100..1000 {
            assert_eq!(number_of_digits(n), 3, "{}", n);
        }

        for n in 1000..10000 {
            assert_eq!(number_of_digits(n), 4, "{}", n);
        }

        for n in 10000..100000 {
            assert_eq!(number_of_digits(n), 5, "{}", n);
        }
    }

    #[test]
    fn split_number_test() {
        assert_eq!(split_number(10), Some((1, 0)));
        assert_eq!(split_number(1234), Some((12, 34)));
        assert_eq!(split_number(12345), None);
    }

    #[test]
    fn cycle_test() {
        let start = Stone(0);
        let mut current = start;

        for _ in 0..100 {
            println!("{}", current.0);

            let next = current.blink_once();
            match next {
                BlinkResult::OneStone(stone) => current = stone,
                BlinkResult::TwoStones(a, _b) => current = a,
            }
            if current == start {
                break;
            }
        }
    }
}

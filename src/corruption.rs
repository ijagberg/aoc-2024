use regex::Regex;
use std::error::Error;

pub fn uncorrupt(corrupted: &str) -> Result<Vec<Multiplication>, Box<dyn Error>> {
    let re = Regex::new(r"mul\((?<x>[0-9]+),(?<y>[0-9]+)\)|don't\(\)|do\(\)").unwrap();

    let mut results = Vec::new();

    let mut toggle = true;
    for capture in re.captures_iter(corrupted) {
        match (capture.name("x"), capture.name("y")) {
            (Some(x), Some(y)) => {
                results.push(Multiplication::new(
                    toggle,
                    x.as_str().parse()?,
                    y.as_str().parse()?,
                ));
            }
            _ => match capture.get(0).unwrap().as_str() {
                "do()" => {
                    toggle = true;
                }
                "don't()" => {
                    toggle = false;
                }
                _ => return Err("regex error".into()),
            },
        }
    }

    Ok(results)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Multiplication {
    on: bool,
    x: i64,
    y: i64,
}

impl Multiplication {
    pub fn new(on: bool, x: i64, y: i64) -> Self {
        Self { on, x, y }
    }

    pub fn product(&self) -> i64 {
        self.x * self.y
    }

    pub fn product_checked(&self) -> Option<i64> {
        if self.on {
            Some(self.x * self.y)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_test() {
        let input = r"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let muls = uncorrupt(&input).unwrap();

        assert_eq!(
            muls,
            vec![
                Multiplication::new(true, 2, 4),
                Multiplication::new(true, 5, 5),
                Multiplication::new(true, 11, 8),
                Multiplication::new(true, 8, 5)
            ]
        );
    }
}

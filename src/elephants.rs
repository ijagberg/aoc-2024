use std::iter;

pub struct Equation {
    numbers: Vec<i64>,
}

impl Equation {
    pub fn new(numbers: Vec<i64>) -> Self {
        Self { numbers }
    }

    pub fn all_results(&self, operations: &[Operation], max_result: i64) -> Vec<i64> {
        let mut results = Vec::with_capacity(2_usize.pow(self.numbers.len() as u32 - 1));
        Self::all_results_rec(0, &self.numbers, &mut results, operations, max_result);

        results
    }

    pub fn can_result_in(&self, operations: &[Operation], target: i64) -> bool {
        Self::find_result_rec(0, &self.numbers, operations, target)
    }

    fn find_result_rec(
        result_so_far: i64,
        numbers: &[i64],
        operations: &[Operation],
        find_result: i64,
    ) -> bool {
        if result_so_far > find_result {
            return false;
        }
        if numbers.is_empty() {
            return result_so_far == find_result;
        }
        for operation in operations {
            match operation {
                Operation::Addition => {
                    if Self::find_result_rec(
                        result_so_far + numbers[0],
                        &numbers[1..],
                        operations,
                        find_result,
                    ) {
                        return true;
                    }
                }
                Operation::Multiplication => {
                    if Self::find_result_rec(
                        result_so_far * numbers[0],
                        &numbers[1..],
                        operations,
                        find_result,
                    ) {
                        return true;
                    }
                }
                Operation::Concatenation => {
                    if Self::find_result_rec(
                        Self::concatenate(result_so_far, numbers[0]),
                        &numbers[1..],
                        operations,
                        find_result,
                    ) {
                        return true;
                    }
                }
            }
        }

        return false;
    }

    fn all_results_rec(
        result_so_far: i64,
        numbers: &[i64],
        results: &mut Vec<i64>,
        operations: &[Operation],
        max_result: i64,
    ) {
        if result_so_far > max_result {
            return;
        }
        if numbers.is_empty() {
            results.push(result_so_far);
            return;
        }
        for operation in operations {
            match operation {
                Operation::Addition => Self::all_results_rec(
                    result_so_far + numbers[0],
                    &numbers[1..],
                    results,
                    operations,
                    max_result,
                ),
                Operation::Multiplication => Self::all_results_rec(
                    result_so_far * numbers[0],
                    &numbers[1..],
                    results,
                    operations,
                    max_result,
                ),
                Operation::Concatenation => Self::all_results_rec(
                    Self::concatenate(result_so_far, numbers[0]),
                    &numbers[1..],
                    results,
                    operations,
                    max_result,
                ),
            }
        }
    }

    fn concatenate(a: i64, b: i64) -> i64 {
        format!("{}{}", a, b).parse().unwrap()
    }
}

pub enum Operation {
    Addition,
    Multiplication,
    Concatenation,
}

impl Operation {
    pub fn iter() -> impl Iterator<Item = Self> {
        iter::once(Self::Addition)
            .chain(iter::once(Self::Multiplication))
            .chain(iter::once(Self::Concatenation))
    }
}

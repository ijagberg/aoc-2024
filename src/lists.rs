use std::{collections::HashMap, ops::AddAssign};

pub fn sync_lists(list_a: &mut Vec<i64>, list_b: &mut Vec<i64>) -> Result<u64, &'static str> {
    if list_a.len() != list_b.len() {
        return Err("lists must be the same length");
    }

    list_a.sort();
    list_b.sort();

    Ok(list_a
        .as_slice()
        .iter()
        .zip(list_b.as_slice().iter())
        .map(|(a, b)| a.abs_diff(*b))
        .sum())
}

pub fn similarity_score(list_a: &[i64], list_b: &[i64]) -> Result<i64, &'static str> {
    let mut counts = HashMap::with_capacity(list_b.len());
    for b in list_b {
        counts.entry(b).or_insert(0).add_assign(1);
    }

    let mut score = 0;
    for a in list_a {
        score += (a * counts.get(&a).unwrap_or(&0));
    }

    Ok(score)
}

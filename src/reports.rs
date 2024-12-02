pub fn safe_reports(
    reports: &[Vec<i64>],
    bounds: (u64, u64),
    dampener: bool,
) -> impl Iterator<Item = &Vec<i64>> {
    reports
        .iter()
        .filter(move |report| is_safe(report, bounds, dampener))
}

fn is_safe(report: &Vec<i64>, bounds: (u64, u64), dampener: bool) -> bool {
    if dampener {
        for remove in 0..report.len() {
            let dampened = report
                .iter()
                .copied()
                .enumerate()
                .filter(|(i, v)| *i != remove)
                .map(|(_, v)| v)
                .collect();
            if is_increasing_or_decreasing(&dampened)
                && is_adjacent_within_bounds(&dampened, bounds)
            {
                return true;
            }
        }
        return false;
    } else {
        is_increasing_or_decreasing(&report) && is_adjacent_within_bounds(&report, bounds)
    }
}

fn is_increasing_or_decreasing(report: &Vec<i64>) -> bool {
    let mut increasing = true;
    let mut decreasing = true;
    for w in report.windows(2) {
        if w[0] < w[1] {
            decreasing = false;
        } else if w[0] > w[1] {
            increasing = false;
        } else {
            increasing = false;
            decreasing = false;
        }
    }

    increasing || decreasing
}

fn is_adjacent_within_bounds(report: &Vec<i64>, (lower, upper): (u64, u64)) -> bool {
    for w in report.windows(2) {
        let diff = w[0].abs_diff(w[1]);
        if !(diff >= lower && diff <= upper) {
            return false;
        }
    }

    true
}

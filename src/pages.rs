use std::collections::{HashMap, HashSet};

pub struct PageRules {
    rules: HashMap<u32, HashSet<u32>>,
}

impl PageRules {
    pub fn new(rules: HashMap<u32, HashSet<u32>>) -> Self {
        Self { rules }
    }

    pub fn is_valid(&self, pages: &[u32]) -> bool {
        self.find_error(pages).is_none()
    }

    fn find_error(&self, pages: &[u32]) -> Option<(usize, usize)> {
        for i in 0..pages.len() {
            let page = pages[i];
            if let Some(should_be_after_i) = self.rules.get(&page) {
                for page_before_idx in 0..i {
                    let page_before = &pages[page_before_idx];
                    if should_be_after_i.contains(page_before) {
                        return Some((i, page_before_idx));
                    }
                }
            }
        }

        None
    }

    pub fn reorder(&self, mut pages: Vec<u32>) -> Vec<u32> {
        while let Some((a, b)) = self.find_error(&pages) {
            pages.swap(a, b);
        }

        pages
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_rules() -> PageRules {
        PageRules::new(
            vec![(2, [3]), (1, [2])]
                .into_iter()
                .map(|(i, after)| (i, after.into_iter().collect::<HashSet<_>>()))
                .collect(),
        )
    }

    #[test]
    fn validate_test() {
        let page_rules = test_rules();

        assert!(page_rules.is_valid(&[1, 2, 3]));
        assert!(page_rules.is_valid(&[2, 3]));
        assert!(page_rules.is_valid(&[2]));
        assert!(page_rules.is_valid(&[3]));
        assert!(!page_rules.is_valid(&[2, 1, 3]));
    }

    #[test]
    fn reorder_test() {
        let page_rules = test_rules();

        assert_eq!(page_rules.reorder(vec![3, 2, 1]), vec![1, 2, 3]);
    }
}

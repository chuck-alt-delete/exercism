use std::collections::HashSet;

fn test_sets() {
    let chars1: HashSet<_> = "abc".chars().collect();
    let chars2: HashSet<_> = "cba".chars().collect();

    assert_eq!(chars1, chars2);
}

fn main() {
    test_sets();
}
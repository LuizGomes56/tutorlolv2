use wiki::lock::compare;

fn entries(list: &[&str]) -> Vec<String> {
    list.iter().map(|s| s.to_string()).collect()
}

#[test]
fn reports_shifted_indexes_and_formula_changes() {
    let old = entries(&["0: A (1)", "1: B (2)", "2: C (3)"]);
    let new = entries(&["0: A (9)", "1: C (3)"]);

    let (lines, formulas) = compare(&old, &new);

    assert_eq!(
        lines,
        ["    - 1: B", "    - 2: C", "    + 1: C"].map(String::from)
    );
    assert_eq!(formulas, 1);
}

#[test]
fn identical_lists_report_nothing() {
    let list = entries(&["0: A (1)", "1: B (2)"]);
    assert_eq!(compare(&list, &list), (Vec::new(), 0));
}

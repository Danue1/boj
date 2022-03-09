use problem_11816::solve;

#[test]
fn example1() {
    let source = include_str!("fixtures/example1/in");

    assert_eq!(
        solve(source),
        Ok(include_str!("fixtures/example1/out").to_string())
    );
}

#[test]
fn example2() {
    let source = include_str!("fixtures/example2/in");

    assert_eq!(
        solve(source),
        Ok(include_str!("fixtures/example2/out").to_string())
    );
}

#[test]
fn example3() {
    let source = include_str!("fixtures/example3/in");

    assert_eq!(
        solve(source),
        Ok(include_str!("fixtures/example3/out").to_string())
    );
}

#[test]
fn example4() {
    let source = include_str!("fixtures/example4/in");

    assert_eq!(
        solve(source),
        Ok(include_str!("fixtures/example4/out").to_string())
    );
}

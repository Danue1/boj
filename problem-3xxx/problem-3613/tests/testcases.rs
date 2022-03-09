use problem_3613::solve;

#[test]
fn example() {
    let source = include_str!("fixtures/example/in");

    assert_eq!(
        solve(source),
        Ok(include_str!("fixtures/example/out").to_string())
    );
}

#[test]
fn official() {
    let source = include_str!("fixtures/official/in");

    assert_eq!(
        solve(source),
        Ok(include_str!("fixtures/official/out").to_string())
    );
}

#[test]
fn camelcase_to_snakecase() {
    let source = include_str!("fixtures/camelcase-to-snakecase/in");

    assert_eq!(
        solve(source),
        Ok(include_str!("fixtures/camelcase-to-snakecase/out").to_string())
    );
}

#[test]
fn snakecase_to_camelcase() {
    let source = include_str!("fixtures/snakecase-to-camelcase/in");

    assert_eq!(
        solve(source),
        Ok(include_str!("fixtures/snakecase-to-camelcase/out").to_string())
    );
}

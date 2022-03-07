use problem_4828::solve;

#[test]
fn example() {
    let source = include_str!("fixtures/example/xml.in").trim();

    assert_eq!(
        solve(source),
        Ok(include_str!("fixtures/example/xml.out").trim().to_string())
    );
}

#[test]
fn test1() {
    let source = include_str!("fixtures/test1/xml.in").trim();

    assert_eq!(
        solve(source),
        Ok(include_str!("fixtures/test1/xml.out").trim().to_string())
    );
}

#[test]
fn test2() {
    let source = include_str!("fixtures/test2/xml.in").trim();

    assert_eq!(
        solve(source),
        Ok(include_str!("fixtures/test2/xml.out").trim().to_string())
    );
}

#[test]
fn test3() {
    let source = include_str!("fixtures/test3/xml.in").trim();

    assert_eq!(
        solve(source),
        Ok(include_str!("fixtures/test3/xml.out").trim().to_string())
    );
}

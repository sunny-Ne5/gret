#[test]
fn test_check_match_positive() {
    let mut result = Vec::new();
    let line: &String = &"Lets see if it works".to_string();
    let pattern: &String = &"works".to_string();
    gret::check_match(&line, &pattern, &mut result);
    assert_eq!(result, line.as_bytes())
}

fn test_check_match_negative() {
    let mut result = Vec::new();
    let line: &String = &"Lets see if it works".to_string();
    let pattern: &String = &"not there".to_string();
    gret::check_match(&line, &pattern, &mut result);
    assert_eq!(result, Vec::new())
}

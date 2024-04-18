use scramble;

#[test]
fn full_word() {
    let words: Vec<&str> = vec!["red", "dee", "cede", "reed", "creed", "decree"];
    assert_eq!(scramble::scramble(words, "red"), ["red"]);
}

#[test]
fn all_stars() {
    let words: Vec<&str> = vec!["red", "dee", "cede", "reed", "creed", "decree"];
    assert_eq!(scramble::scramble(words, "****"), ["cede", "reed"]);
}

#[test]
fn some_chars() {
    let words: Vec<&str> = vec!["red", "dee", "cede", "reed", "creed", "decree"];
    assert_eq!(scramble::scramble(words, "*r*e*"), ["creed"]);
}

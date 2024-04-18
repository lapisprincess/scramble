use scramble;

#[test]
fn full_word() {
    let words: Vec<&str> = vec!["red", "dee", "cede", "reed", "creed", "decree"];
    let mask: &str = "red";
    assert_eq!(scramble::scramble(words, mask), ["red"]);
}

#[test]
fn all_stars() {
    let words: Vec<&str> = vec!["red", "dee", "cede", "reed", "creed", "decree"];
    let mask: &str = "****";
    assert_eq!(scramble::scramble(words, mask), ["cede", "reed"]);
}

#[test]
fn some_chars() {
    let words: Vec<&str> = vec!["red", "dee", "cede", "reed", "creed", "decree"];
    let mask: &str = "*r*e*";
    assert_eq!(scramble::scramble(words, mask), ["creed"]);
}

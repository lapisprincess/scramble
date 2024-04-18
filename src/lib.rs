use regex::Regex;

pub fn scramble(words: Vec<&str>, mask: &str) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    let expr = format!(r"\b({})\b", mask.replace("*", r"\w{1}"));
    let re = Regex::new(&expr).unwrap();
    for &word in words.iter().filter(|word| re.is_match(word)) {
        out.push(String::from(word));
    }
    out
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn full_word() {
        let words: Vec<&str> = vec!["red", "dee", "cede", "reed", "creed", "decree"];
        let mask: &str = "red";
        assert_eq!(scramble(words, mask), ["red"]);
    }

    #[test]
    fn all_stars() {
        let words: Vec<&str> = vec!["red", "dee", "cede", "reed", "creed", "decree"];
        let mask: &str = "****";
        assert_eq!(scramble(words, mask), ["cede", "reed"]);
    }

    #[test]
    fn some_chars() {
        let words: Vec<&str> = vec!["red", "dee", "cede", "reed", "creed", "decree"];
        let mask: &str = "*r*e*";
        assert_eq!(scramble(words, mask), ["creed"]);
    }
}

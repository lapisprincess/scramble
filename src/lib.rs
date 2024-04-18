use regex::Regex;

pub fn scramble(words: Vec<&str>, mask: &str) -> Vec<String> {
    let expr = format!(r"\b({})\b", mask.replace("*", r"\w"));
    let re = Regex::new(&expr).unwrap();
    words.iter()
        .filter(|word| re.is_match(word))
        .map(|&word| String::from(word))
        .collect()
}

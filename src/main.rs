use regex::Regex;

fn main() {
    let words: Vec<&str> = vec![ "red", "dee", "cede", "reed", "creed", "decree" ];
    let mask: &str = "*e*d";
    let result = scramble(words, mask);
    for word in result { print!("{}\n", word); }
}

fn scramble(words: Vec<&str>, mask: &str) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    let expr = format!(r"\b({})\b", mask.replace("*", r"\w{1}"));
    let re = Regex::new(&expr).unwrap();
    for &word in words.iter().filter(|word| re.is_match(word)) {
        out.push(String::from(word));
    }
    out
}

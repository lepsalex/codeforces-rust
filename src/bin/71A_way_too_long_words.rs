use input::Scanner;

fn main() {
    let mut input = Scanner::new();
    let len: usize = input.next();

    for _ in 0..len {
        let word: String = input.next();
        let solution = solve(word);
        println!("{}", solution)
    }
}

fn solve(word: String) -> String {
    if word.len() <= 10 {
        return word;
    }

    format!("{}{}{}", &word[0..1], &word.len() - 2, &word[word.len() - 1..])
}

#[cfg(test)]
mod test {
    use crate::solve;

    #[test]
    fn case_1() {
        assert_eq!(solve("word".to_string()), "word");
    }

    #[test]
    fn case_2() {
        assert_eq!(solve("localization".to_string()), "l10n");
    }

    #[test]
    fn case_3() {
        assert_eq!(solve("internationalization".to_string()), "i18n");
    }

    #[test]
    fn case_4() {
        assert_eq!(solve("pneumonoultramicroscopicsilicovolcanoconiosis".to_string()), "p43s");
    }
}
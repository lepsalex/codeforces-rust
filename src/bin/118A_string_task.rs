use input::Scanner;

fn main() {
    let mut input = Scanner::new();
    let solution = solve(input.next());
    println!("{}", solution)
}

fn solve(s: String) -> String {
    let vowels = vec!['a', 'o', 'y', 'e', 'u', 'i'];
    s.to_lowercase().chars().filter(|c| !vowels.contains(c)).map(|c| format!(".{}", c)).collect()
}

#[cfg(test)]
mod test {
    use crate::solve;

    #[test]
    fn case_1() {
        assert_eq!(solve("tour".to_string()), ".t.r");
    }

    #[test]
    fn case_2() {
        assert_eq!(solve("Codeforces".to_string()), ".c.d.f.r.c.s");
    }

    #[test]
    fn case_3() {
        assert_eq!(solve("aBAcAba".to_string()), ".b.c.b");
    }
}
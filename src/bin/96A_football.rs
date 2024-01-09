use input::Scanner;

fn main() {
    let mut input = Scanner::new();
    println!("{}", solve(&input.next::<String>()));
}

fn solve(s: &str) -> String {
    // base case
    if s.len() < 7 {
        return "NO".to_string();
    }

    let mut chars = s.chars();
    let mut last_char = chars.next().unwrap();
    let mut count = 1;
    let mut max = 1;

    while let Some(curr) = chars.next() {
        if curr == last_char {
            count += 1;

            if count > max {
                max = count;

                if max >= 7 {
                    return "YES".to_string();
                }
            }
        } else {
            last_char = curr;
            count = 1;
        }
    }

    return "NO".to_string();
}

#[cfg(test)]
mod test {
    use crate::solve;

    #[test]
    fn case_1() {
        assert_eq!(solve("001001"), "NO")
    }

    #[test]
    fn case_2() {
        assert_eq!(solve("1000000001"), "YES")
    }

    #[test]
    fn case_3() {
        assert_eq!(solve("00100110111111101"), "YES")
    }

    #[test]
    fn case_41() {
        assert_eq!(solve("10000000"), "YES")
    }
}